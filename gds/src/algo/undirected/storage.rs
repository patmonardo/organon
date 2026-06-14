//! ToUndirected storage runtime.
//!
//! Translation source: `org.neo4j.gds.undirected.ToUndirectedAlgorithmFactory`.
//!
//! Builds a graph view for the requested relationship type and invokes the
//! computation runtime to produce an undirected projection.

use super::spec::{ToUndirectedConfig, ToUndirectedResult};
use super::ToUndirectedComputationRuntime;
use crate::task::concurrency::TerminationFlag;
use crate::core::utils::progress::ProgressTracker;
use crate::core::Aggregation as CoreAggregation;
use crate::projection::{Orientation, RelationshipType};
use crate::types::graph::MappedNodeId;
use crate::types::graph_store::{GraphName, GraphStore};
use crate::types::prelude::DefaultGraphStore;
use crate::types::properties::relationship::{
    DefaultRelationshipPropertyValues, RelationshipPropertyValues,
};
use crate::types::schema::{
    Aggregation as SchemaAggregation, Direction, PropertySchemaTrait, RelationshipPropertySchema,
};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub struct ToUndirectedStorageRuntime {
    concurrency: usize,
}

impl ToUndirectedStorageRuntime {
    pub fn new(concurrency: usize) -> Self {
        Self { concurrency }
    }

    pub fn compute(
        &self,
        graph_store: &DefaultGraphStore,
        config: &ToUndirectedConfig,
        computation: &mut ToUndirectedComputationRuntime,
    ) -> Result<ToUndirectedResult, String> {
        let termination_flag = TerminationFlag::running_true();
        let mut progress_tracker = crate::core::utils::progress::NoopProgressTracker;
        self.compute_with_controls(
            graph_store,
            config,
            computation,
            &termination_flag,
            &mut progress_tracker,
        )
    }

    pub fn compute_with_controls(
        &self,
        graph_store: &DefaultGraphStore,
        config: &ToUndirectedConfig,
        computation: &mut ToUndirectedComputationRuntime,
        termination_flag: &TerminationFlag,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<ToUndirectedResult, String> {
        if self.concurrency == 0 || config.concurrency == 0 {
            return Err("concurrency must be greater than zero".to_string());
        }
        if config.relationship_type.is_empty() {
            return Err("relationship_type must be provided".to_string());
        }
        if config.mutate_relationship_type.is_empty() {
            return Err("mutate_relationship_type must be provided".to_string());
        }
        if config.mutate_graph_name.is_empty() {
            return Err("mutate_graph_name must be provided".to_string());
        }

        let mut rels = HashSet::new();
        let source_relationship_type = RelationshipType::of(&config.relationship_type);
        let mutate_relationship_type = RelationshipType::of(&config.mutate_relationship_type);
        rels.insert(source_relationship_type.clone());

        let property_schemas =
            relationship_property_schemas(graph_store, &source_relationship_type)?;
        let target_property_schemas = target_property_schemas(&property_schemas, config);

        let progress_volume = graph_store
            .node_count()
            .saturating_mul(property_schemas.len().saturating_add(1));
        progress_tracker.begin_subtask_with_volume(progress_volume);

        let result = self.compute_inner(
            graph_store,
            config,
            computation,
            termination_flag,
            progress_tracker,
            rels,
            source_relationship_type,
            mutate_relationship_type,
            property_schemas,
            target_property_schemas,
        );

        match &result {
            Ok(_) => progress_tracker.end_subtask(),
            Err(_) => progress_tracker.end_subtask_with_failure(),
        }

        result
    }

    #[allow(clippy::too_many_arguments)]
    fn compute_inner(
        &self,
        graph_store: &DefaultGraphStore,
        config: &ToUndirectedConfig,
        computation: &mut ToUndirectedComputationRuntime,
        termination_flag: &TerminationFlag,
        progress_tracker: &mut dyn ProgressTracker,
        rels: HashSet<RelationshipType>,
        source_relationship_type: RelationshipType,
        mutate_relationship_type: RelationshipType,
        property_schemas: Vec<RelationshipPropertySchema>,
        target_property_schemas: Vec<RelationshipPropertySchema>,
    ) -> Result<ToUndirectedResult, String> {
        if !termination_flag.running() {
            return Err("ToUndirected terminated".to_string());
        }

        let graph = graph_store
            .get_graph_with_types_and_orientation(&rels, Orientation::Natural)
            .map_err(|e| {
                format!(
                    "failed to build graph for relationship type '{}': {e}",
                    config.relationship_type
                )
            })?;

        let edges = computation.compute_with_controls(
            graph.as_ref(),
            &config.mutate_relationship_type,
            termination_flag,
            |done| progress_tracker.log_progress(done),
        )?;
        let outgoing = build_outgoing(graph_store.node_count(), edges)?;
        let edges_for_result: Vec<(u64, u64)> = outgoing
            .iter()
            .enumerate()
            .flat_map(|(src, targets)| targets.iter().map(move |t| (src as u64, *t as u64)))
            .collect();

        let mut store = graph_store
            .with_added_relationship_type_and_properties(
                GraphName::new(&config.mutate_graph_name),
                mutate_relationship_type.clone(),
                outgoing,
                Direction::Undirected,
                target_property_schemas,
            )
            .map_err(|e| e.to_string())?;

        for property_schema in &property_schemas {
            let property_key = property_schema.key().to_string();
            let aggregation = effective_core_aggregation(config, property_schema);
            let property_values = aggregate_property_values(
                graph_store,
                computation,
                &rels,
                &source_relationship_type,
                property_schema,
                aggregation,
                &edges_for_result,
                termination_flag,
                progress_tracker,
            )?;

            if property_values.is_empty() {
                continue;
            }

            let element_count = property_values.len();
            let default_value = property_schema
                .default_value()
                .double_value()
                .unwrap_or(f64::NAN);
            let values: Arc<dyn RelationshipPropertyValues> =
                Arc::new(DefaultRelationshipPropertyValues::with_values(
                    property_values,
                    default_value,
                    element_count,
                ));

            store
                .add_relationship_property(mutate_relationship_type.clone(), property_key, values)
                .map_err(|e| e.to_string())?;
        }

        Ok(ToUndirectedResult {
            graph_name: config.mutate_graph_name.clone(),
            mutate_relationship_type: config.mutate_relationship_type.clone(),
            node_count: store.node_count() as u64,
            relationship_count: store.relationship_count() as u64,
            edges: edges_for_result,
            graph_store: store,
        })
    }

    pub fn concurrency(&self) -> usize {
        self.concurrency
    }
}

fn build_outgoing(
    node_count: usize,
    mut edges: Vec<(u64, u64)>,
) -> Result<Vec<Vec<MappedNodeId>>, String> {
    let mut outgoing: Vec<Vec<MappedNodeId>> = vec![Vec::new(); node_count];
    for (src, tgt) in edges.drain(..) {
        let src_usize = src as usize;
        let tgt_usize = tgt as usize;
        if src_usize >= node_count || tgt_usize >= node_count {
            return Err(format!(
                "edge ({src},{tgt}) is out of bounds for node_count={node_count}"
            ));
        }
        outgoing[src_usize].push(tgt as MappedNodeId);
    }

    for adj in outgoing.iter_mut() {
        adj.sort_unstable();
        adj.dedup();
    }

    Ok(outgoing)
}

fn relationship_property_schemas(
    graph_store: &DefaultGraphStore,
    rel_type: &RelationshipType,
) -> Result<Vec<RelationshipPropertySchema>, String> {
    let mut schemas: Vec<_> = graph_store
        .schema()
        .relationship_schema()
        .get(rel_type)
        .map(|entry| entry.properties().values().cloned().collect())
        .unwrap_or_default();
    schemas.sort_by(|left: &RelationshipPropertySchema, right| left.key().cmp(right.key()));
    Ok(schemas)
}

fn target_property_schemas(
    property_schemas: &[RelationshipPropertySchema],
    config: &ToUndirectedConfig,
) -> Vec<RelationshipPropertySchema> {
    property_schemas
        .iter()
        .map(|schema| {
            let aggregation = config
                .aggregation
                .as_ref()
                .and_then(|aggregations| aggregations.local_aggregation(schema.key()))
                .map(core_to_schema_aggregation)
                .unwrap_or_else(|| schema.aggregation());

            RelationshipPropertySchema::with_aggregation(
                schema.key().to_string(),
                schema.value_type(),
                schema.default_value().clone(),
                schema.state(),
                aggregation,
            )
        })
        .collect()
}

fn aggregate_property_values(
    graph_store: &DefaultGraphStore,
    computation: &ToUndirectedComputationRuntime,
    rels: &HashSet<RelationshipType>,
    source_relationship_type: &RelationshipType,
    property_schema: &RelationshipPropertySchema,
    aggregation: CoreAggregation,
    edges_for_result: &[(u64, u64)],
    termination_flag: &TerminationFlag,
    progress_tracker: &mut dyn ProgressTracker,
) -> Result<Vec<f64>, String> {
    let property_key = property_schema.key().to_string();
    let mut selectors = HashMap::new();
    selectors.insert(source_relationship_type.clone(), property_key.clone());

    let property_graph = graph_store
        .get_graph_with_types_selectors_and_orientation(rels, &selectors, Orientation::Natural)
        .map_err(|e| {
            format!(
                "failed to build graph for relationship property '{}': {e}",
                property_key
            )
        })?;

    let aggregated = computation.aggregate_property_with_controls(
        property_graph.as_ref(),
        aggregation,
        termination_flag,
        |done| progress_tracker.log_progress(done),
    )?;
    let default_value = property_schema
        .default_value()
        .double_value()
        .unwrap_or(f64::NAN);
    let mut values = Vec::with_capacity(edges_for_result.len());

    for &(source, target) in edges_for_result {
        let key = if source <= target {
            (source, target)
        } else {
            (target, source)
        };
        let value = aggregated.get(&key).copied().unwrap_or(default_value);
        values.push(value);
    }

    Ok(values)
}

fn effective_core_aggregation(
    config: &ToUndirectedConfig,
    property_schema: &RelationshipPropertySchema,
) -> CoreAggregation {
    config
        .aggregation
        .as_ref()
        .and_then(|aggregations| aggregations.local_aggregation(property_schema.key()))
        .unwrap_or_else(|| schema_to_core_aggregation(property_schema.aggregation()))
}

fn schema_to_core_aggregation(aggregation: SchemaAggregation) -> CoreAggregation {
    match aggregation {
        SchemaAggregation::None => CoreAggregation::None,
        SchemaAggregation::Min => CoreAggregation::Min,
        SchemaAggregation::Max => CoreAggregation::Max,
        SchemaAggregation::Sum => CoreAggregation::Sum,
        SchemaAggregation::Count => CoreAggregation::Count,
        SchemaAggregation::Single => CoreAggregation::Single,
        SchemaAggregation::Default => CoreAggregation::Default,
    }
}

fn core_to_schema_aggregation(aggregation: CoreAggregation) -> SchemaAggregation {
    match aggregation {
        CoreAggregation::Default => SchemaAggregation::Default,
        CoreAggregation::None => SchemaAggregation::None,
        CoreAggregation::Single => SchemaAggregation::Single,
        CoreAggregation::Sum => SchemaAggregation::Sum,
        CoreAggregation::Min => SchemaAggregation::Min,
        CoreAggregation::Max => SchemaAggregation::Max,
        CoreAggregation::Count => SchemaAggregation::Count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GraphStoreConfig;
    use crate::types::graph::{RelationshipTopology, SimpleIdMap};
    use crate::types::graph_store::{Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation};
    use crate::types::properties::relationship::DefaultRelationshipPropertyValues;
    use crate::types::schema::{MutableGraphSchema, RelationshipPropertySchema};
    use crate::types::{DefaultValue, PropertyState, ValueType};

    fn store_with_weighted_reciprocal_edges(
        aggregation: SchemaAggregation,
        values: Vec<f64>,
    ) -> DefaultGraphStore {
        let rel_type = RelationshipType::of("REL");
        let mut schema_builder = MutableGraphSchema::empty();
        schema_builder
            .relationship_schema_mut()
            .get_or_create_type(rel_type.clone(), Direction::Directed)
            .add_property_schema(RelationshipPropertySchema::with_aggregation(
                "weight",
                ValueType::Double,
                DefaultValue::system_default(None),
                PropertyState::Persistent,
                aggregation,
            ));

        let mut topologies = HashMap::new();
        topologies.insert(
            rel_type.clone(),
            RelationshipTopology::new(vec![vec![1], vec![0]], None),
        );

        let id_map = SimpleIdMap::from_original_ids(vec![0, 1]);
        let mut store = DefaultGraphStore::new(
            GraphStoreConfig::default(),
            GraphName::new("g"),
            DatabaseInfo::new(
                DatabaseId::new("db"),
                DatabaseLocation::remote("localhost", 7687, None, None),
            ),
            schema_builder.build(),
            Capabilities::default(),
            id_map,
            topologies,
        );

        let element_count = values.len();
        let property_values: Arc<dyn RelationshipPropertyValues> = Arc::new(
            DefaultRelationshipPropertyValues::with_values(values, f64::NAN, element_count),
        );
        store
            .add_relationship_property(rel_type, "weight", property_values)
            .expect("add weight property");
        store
    }

    fn undirected_config() -> ToUndirectedConfig {
        ToUndirectedConfig {
            relationship_type: "REL".to_string(),
            mutate_graph_name: "mutated".to_string(),
            mutate_relationship_type: "UNDIRECTED".to_string(),
            concurrency: 1,
            aggregation: None,
        }
    }

    #[test]
    fn sums_reciprocal_relationship_property_values_from_schema() {
        let store = store_with_weighted_reciprocal_edges(SchemaAggregation::Sum, vec![2.0, 3.0]);
        let mut computation = ToUndirectedComputationRuntime::new();
        let storage = ToUndirectedStorageRuntime::new(1);

        let result = storage
            .compute(&store, &undirected_config(), &mut computation)
            .expect("to undirected with sum aggregation");

        assert_eq!(result.edges, vec![(0, 1), (1, 0)]);
        let rel_type = RelationshipType::of("UNDIRECTED");
        let values = result
            .graph_store
            .relationship_property_values(&rel_type, "weight")
            .expect("mutated weight values");
        assert_eq!(values.double_value(0).unwrap(), 5.0);
        assert_eq!(values.double_value(1).unwrap(), 5.0);

        let schema_aggregation = result
            .graph_store
            .schema()
            .relationship_schema()
            .get(&rel_type)
            .and_then(|entry| entry.properties().get("weight"))
            .map(|schema| schema.aggregation());
        assert_eq!(schema_aggregation, Some(SchemaAggregation::Sum));
    }

    #[test]
    fn per_property_override_replaces_schema_aggregation() {
        let store = store_with_weighted_reciprocal_edges(SchemaAggregation::None, vec![2.0, 3.0]);
        let mut config = undirected_config();
        config.aggregation = Some(
            super::super::spec::ToUndirectedAggregations::new()
                .with_property("weight", CoreAggregation::Max),
        );

        let mut computation = ToUndirectedComputationRuntime::new();
        let storage = ToUndirectedStorageRuntime::new(1);
        let result = storage
            .compute(&store, &config, &mut computation)
            .expect("to undirected with max override");

        let rel_type = RelationshipType::of("UNDIRECTED");
        let values = result
            .graph_store
            .relationship_property_values(&rel_type, "weight")
            .expect("mutated weight values");
        assert_eq!(values.double_value(0).unwrap(), 3.0);
        assert_eq!(values.double_value(1).unwrap(), 3.0);

        let schema_aggregation = result
            .graph_store
            .schema()
            .relationship_schema()
            .get(&rel_type)
            .and_then(|entry| entry.properties().get("weight"))
            .map(|schema| schema.aggregation());
        assert_eq!(schema_aggregation, Some(SchemaAggregation::Max));
    }

    #[test]
    fn none_aggregation_rejects_duplicate_unordered_pairs() {
        let store = store_with_weighted_reciprocal_edges(SchemaAggregation::None, vec![2.0, 3.0]);
        let mut computation = ToUndirectedComputationRuntime::new();
        let storage = ToUndirectedStorageRuntime::new(1);

        let error = storage
            .compute(&store, &undirected_config(), &mut computation)
            .expect_err("NONE aggregation should reject reciprocal duplicates");

        assert!(error.contains("NONE"));
    }
}
