use super::{
    Capabilities, DatabaseInfo, DeletionResult, GraphName, GraphStore, GraphStoreError,
    GraphStoreResult, GraphViewError, GraphViewResult, GraphViewSpec, InducedSubgraphResult,
    ProjectedPropertiesResult,
};
use crate::collections::backends::arrow::{ArrowDoubleArray, ArrowLongArray};
use crate::collections::backends::factory::{
    create_double_backend_from_config, create_float_backend_from_config,
    create_int_backend_from_config, create_long_backend_from_config, DoubleCollection,
    LongCollection,
};
use crate::collections::backends::vec::{
    VecDouble, VecDoubleArray, VecFloat, VecInt, VecLong, VecLongArray,
};
use crate::config::GraphStoreConfig;
use crate::projection::Orientation;
use crate::projection::{NodeLabel, RelationshipType};
use crate::types::graph::id_map::{MappedNodeId, OriginalNodeId, RelationshipIndex};
use crate::types::graph::{
    id_map::{IdMap, SimpleIdMap},
    DefaultGraph, Graph, GraphCharacteristics, GraphCharacteristicsBuilder, RelationshipTopology,
};
use crate::types::properties::graph::{
    DefaultDoubleGraphPropertyValues, DefaultGraphPropertyStore, DefaultLongGraphPropertyValues,
    GraphProperty, GraphPropertyStore, GraphPropertyValues,
};
use crate::types::properties::node::{
    DefaultDoubleArrayNodePropertyValues, DefaultLongArrayNodePropertyValues,
};
use crate::types::properties::node::{
    DefaultDoubleNodePropertyValues, DefaultFloatNodePropertyValues, DefaultIntNodePropertyValues,
    DefaultLongNodePropertyValues,
};
use crate::types::properties::node::{
    DefaultNodePropertyStore, NodeProperty, NodePropertyStore, NodePropertyValues,
};
use crate::types::properties::relationship::default_relationship_property_store::DefaultRelationshipPropertyStore;
use crate::types::properties::relationship::relationship_property::RelationshipProperty;
use crate::types::properties::relationship::RelationshipPropertyValues;
use crate::types::properties::relationship::{
    DefaultDoubleRelationshipPropertyValues, DefaultIntRelationshipPropertyValues,
    DefaultLongRelationshipPropertyValues,
};
use crate::types::properties::relationship::{
    RelationshipPropertyStore, RelationshipPropertyStoreBuilder,
};
use crate::types::properties::PropertyStore;
use crate::types::properties::PropertyValues;
use crate::types::properties::PropertyValuesError;
use crate::types::properties::PropertyValuesResult;
use crate::types::schema::{
    Aggregation, Direction, GraphSchema, MutableGraphSchema, PropertySchemaTrait,
    RelationshipPropertySchema, RelationshipSchema, RelationshipSchemaEntry,
};
use crate::types::DefaultValue;
use crate::types::PropertyState;
use crate::types::ValueType;
use chrono::{DateTime, Utc};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::algo::algorithms::scaling::{MinMaxScaler, Scaler};

#[derive(Debug, Clone)]
struct ReindexedRelationshipPropertyValues {
    values: Arc<dyn RelationshipPropertyValues>,
    old_indices: Arc<Vec<RelationshipIndex>>,
}

impl ReindexedRelationshipPropertyValues {
    fn new(
        values: Arc<dyn RelationshipPropertyValues>,
        old_indices: Arc<Vec<RelationshipIndex>>,
    ) -> Self {
        Self {
            values,
            old_indices,
        }
    }

    fn old_index(
        &self,
        relationship_index: RelationshipIndex,
    ) -> PropertyValuesResult<RelationshipIndex> {
        let physical_index = relationship_index
            .to_usize()
            .ok_or(PropertyValuesError::ValueNotFound(relationship_index.get()))?;
        self.old_indices
            .get(physical_index)
            .copied()
            .ok_or(PropertyValuesError::ValueNotFound(relationship_index.get()))
    }
}

impl PropertyValues for ReindexedRelationshipPropertyValues {
    fn value_type(&self) -> ValueType {
        self.values.value_type()
    }

    fn element_count(&self) -> usize {
        self.old_indices.len()
    }
}

impl RelationshipPropertyValues for ReindexedRelationshipPropertyValues {
    fn double_value(&self, rel_index: RelationshipIndex) -> PropertyValuesResult<f64> {
        self.values.double_value(self.old_index(rel_index)?)
    }

    fn long_value(&self, rel_index: RelationshipIndex) -> PropertyValuesResult<i64> {
        self.values.long_value(self.old_index(rel_index)?)
    }

    fn get_object(
        &self,
        rel_index: RelationshipIndex,
    ) -> PropertyValuesResult<Box<dyn std::any::Any>> {
        self.values.get_object(self.old_index(rel_index)?)
    }

    fn default_value(&self) -> f64 {
        self.values.default_value()
    }

    fn has_value(&self, rel_index: RelationshipIndex) -> bool {
        rel_index
            .to_usize()
            .and_then(|index| self.old_indices.get(index))
            .is_some_and(|old_index| self.values.has_value(*old_index))
    }
}

/// RAM-only Bootstrap [`GraphStore`] for deterministic algorithm development and tests.
///
/// Cloning creates a shallow snapshot fork for `Arc`-backed ID maps, topologies, and
/// property values, while metadata maps, sets, and vectors are copied. This type is
/// neither persistent storage nor the future production CoreGraphStore/HugeGraphStore.
#[derive(Debug, Clone)]
pub struct DefaultGraphStore {
    config: Arc<GraphStoreConfig>,
    graph_name: GraphName,
    database_info: DatabaseInfo,
    schema: Arc<GraphSchema>,
    capabilities: Capabilities,
    creation_time: DateTime<Utc>,
    modification_time: DateTime<Utc>,
    id_map: Arc<SimpleIdMap>,
    relationship_topologies: HashMap<RelationshipType, Arc<RelationshipTopology>>,
    ordered_relationship_types: Vec<RelationshipType>,
    inverse_indexed_relationship_types: HashSet<RelationshipType>,
    relationship_count: usize,
    has_parallel_relationships: bool,
    graph_characteristics: GraphCharacteristics,
    graph_properties: DefaultGraphPropertyStore,
    node_properties: DefaultNodePropertyStore,
    node_properties_by_label: HashMap<String, HashSet<String>>,
    relationship_property_stores: HashMap<RelationshipType, DefaultRelationshipPropertyStore>,
    has_relationship_properties: bool,
}

impl DefaultGraphStore {
    /// Creates a new store from the provided components.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        config: GraphStoreConfig,
        graph_name: GraphName,
        database_info: DatabaseInfo,
        schema: GraphSchema,
        capabilities: Capabilities,
        id_map: SimpleIdMap,
        relationship_topologies: HashMap<RelationshipType, RelationshipTopology>,
    ) -> Self {
        Self::try_new(
            config,
            graph_name,
            database_info,
            schema,
            capabilities,
            id_map,
            relationship_topologies,
        )
        .expect("default graph store components must be valid")
    }

    /// Creates a store after validating topology capacity against the mapped node domain.
    #[allow(clippy::too_many_arguments)]
    pub fn try_new(
        config: GraphStoreConfig,
        graph_name: GraphName,
        database_info: DatabaseInfo,
        schema: GraphSchema,
        capabilities: Capabilities,
        id_map: SimpleIdMap,
        relationship_topologies: HashMap<RelationshipType, RelationshipTopology>,
    ) -> GraphStoreResult<Self> {
        let node_count = id_map.node_count();
        for (relationship_type, topology) in &relationship_topologies {
            if topology.node_capacity() != node_count {
                return Err(GraphStoreError::InvalidOperation(format!(
                    "relationship topology '{relationship_type}' has capacity {} but IdMap contains {node_count} nodes",
                    topology.node_capacity()
                )));
            }
        }

        let now = Utc::now();
        let config = Arc::new(config);
        let schema = Arc::new(schema);
        let id_map = Arc::new(id_map);
        let relationship_topologies = relationship_topologies
            .into_iter()
            .map(|(rel_type, topology)| (rel_type, Arc::new(topology)))
            .collect();

        let mut store = Self {
            config: Arc::clone(&config),
            graph_name,
            database_info,
            schema,
            capabilities,
            creation_time: now,
            modification_time: now,
            id_map,
            relationship_topologies,
            ordered_relationship_types: Vec::new(),
            inverse_indexed_relationship_types: HashSet::new(),
            relationship_count: 0,
            has_parallel_relationships: false,
            graph_characteristics: GraphCharacteristicsBuilder::new().build(),
            graph_properties: DefaultGraphPropertyStore::empty(),
            node_properties: DefaultNodePropertyStore::empty(),
            node_properties_by_label: HashMap::new(),
            relationship_property_stores: HashMap::new(),
            has_relationship_properties: false,
        };

        store.rebuild_relationship_metadata();
        store.refresh_relationship_property_state();
        Ok(store)
    }

    /// Builds a [`DefaultGraph`] view over the current store contents.
    /// Returns the concrete DefaultGraph type for backwards compatibility.
    pub fn graph(&self) -> Arc<DefaultGraph> {
        self.graph_with_view(HashMap::new(), Orientation::Natural)
    }

    fn graph_with_view(
        &self,
        relationship_property_selectors: HashMap<RelationshipType, String>,
        orientation: Orientation,
    ) -> Arc<DefaultGraph> {
        let (topologies, relationship_properties) = self.oriented_relationship_data(orientation);
        let schema = if orientation == Orientation::Undirected {
            Arc::new(oriented_schema(&self.schema, orientation))
        } else {
            Arc::clone(&self.schema)
        };
        let relationship_count = topologies
            .values()
            .map(|topology| topology.relationship_count())
            .sum();
        let has_parallel_relationships = topologies
            .values()
            .any(|topology| topology.has_parallel_edges());
        let inverse_indexed_relationship_types = topologies
            .iter()
            .filter(|(_, topology)| topology.is_inverse_indexed())
            .map(|(rel_type, _)| rel_type.clone())
            .collect::<HashSet<_>>();
        let all_inverse_indexed = !self.ordered_relationship_types.is_empty()
            && self
                .ordered_relationship_types
                .iter()
                .all(|rel_type| inverse_indexed_relationship_types.contains(rel_type));
        let mut characteristics = GraphCharacteristicsBuilder::new();
        characteristics = if orientation == Orientation::Undirected || schema.is_undirected() {
            characteristics.undirected()
        } else {
            characteristics.directed()
        };
        if all_inverse_indexed {
            characteristics = characteristics.inverse_indexed();
        }

        Arc::new(DefaultGraph::new(
            Arc::clone(&self.config),
            schema,
            Arc::clone(&self.id_map),
            characteristics.build(),
            topologies,
            self.ordered_relationship_types.clone(),
            inverse_indexed_relationship_types,
            relationship_count,
            has_parallel_relationships,
            self.materialized_node_property_values(),
            relationship_properties,
            relationship_property_selectors,
        ))
    }

    fn oriented_relationship_data(
        &self,
        orientation: Orientation,
    ) -> (
        HashMap<RelationshipType, Arc<RelationshipTopology>>,
        HashMap<RelationshipType, DefaultRelationshipPropertyStore>,
    ) {
        if orientation == Orientation::Natural {
            return (
                self.relationship_topologies.clone(),
                self.relationship_property_stores.clone(),
            );
        }

        let mut topologies = HashMap::new();
        let mut property_stores = HashMap::new();

        for rel_type in &self.ordered_relationship_types {
            let Some(topology) = self.relationship_topologies.get(rel_type) else {
                continue;
            };
            let already_undirected = orientation == Orientation::Undirected
                && self
                    .schema
                    .relationship_schema()
                    .get(rel_type)
                    .is_some_and(RelationshipSchemaEntry::is_undirected);
            if already_undirected {
                topologies.insert(rel_type.clone(), Arc::clone(topology));
                if let Some(store) = self.relationship_property_stores.get(rel_type) {
                    property_stores.insert(rel_type.clone(), store.clone());
                }
                continue;
            }
            let (oriented_topology, old_indices) = orient_topology(topology, orientation);
            topologies.insert(rel_type.clone(), Arc::new(oriented_topology));

            if let Some(store) = self.relationship_property_stores.get(rel_type) {
                property_stores.insert(
                    rel_type.clone(),
                    reindex_relationship_property_store(store, Arc::new(old_indices)),
                );
            }
        }

        (topologies, property_stores)
    }

    fn materialized_node_property_values(&self) -> HashMap<String, Arc<dyn NodePropertyValues>> {
        self.node_properties
            .columns()
            .map(|property| (property.key().to_string(), property.values_arc()))
            .collect()
    }

    /// Creates an undirected version of this graph store.
    ///
    /// Semantics:
    /// - For each relationship type, creates symmetric adjacency: if (u→v) exists, then (v→u) exists.
    /// - Neighbor sets are deduplicated per source node.
    /// - Relationship properties are not carried over yet.
    pub fn to_undirected(&self, graph_name: GraphName) -> GraphStoreResult<DefaultGraphStore> {
        let node_count = self.node_count();

        let mut new_relationship_topologies: HashMap<RelationshipType, RelationshipTopology> =
            HashMap::new();

        for (rel_type, topology) in &self.relationship_topologies {
            let mut outgoing: Vec<Vec<MappedNodeId>> = vec![Vec::new(); node_count];

            for source in 0..node_count {
                let source_id = MappedNodeId::try_from(source).map_err(|_| {
                    GraphStoreError::InvalidOperation(
                        "node count exceeds mapped ID space while building undirected topology"
                            .to_string(),
                    )
                })?;
                let Some(neighbors) = topology.outgoing(source_id) else {
                    continue;
                };

                for &target_id in neighbors.iter() {
                    let target = target_id.to_usize().ok_or_else(|| {
                        GraphStoreError::InvalidOperation(format!(
                            "relationship target {target_id} exceeds physical index space"
                        ))
                    })?;
                    outgoing[source].push(target_id);
                    outgoing[target].push(source_id);
                }
            }

            for adj in outgoing.iter_mut() {
                adj.sort_unstable();
                adj.dedup();
            }

            let undirected = RelationshipTopology::try_new(outgoing, None).map_err(|error| {
                GraphStoreError::InvalidOperation(format!(
                    "invalid undirected topology for relationship type '{rel_type}': {error}"
                ))
            })?;
            new_relationship_topologies.insert(rel_type.clone(), undirected);
        }

        // Update schema: mark all relationship types as undirected, preserving properties.
        let old_schema = self.schema.as_ref();
        let mut rel_entries: HashMap<RelationshipType, RelationshipSchemaEntry> = HashMap::new();
        for entry in old_schema.relationship_schema().entries() {
            rel_entries.insert(
                entry.identifier().clone(),
                RelationshipSchemaEntry::new(
                    entry.identifier().clone(),
                    Direction::Undirected,
                    entry.properties().clone(),
                ),
            );
        }
        let relationship_schema = RelationshipSchema::new(rel_entries);
        let schema = GraphSchema::new(
            old_schema.node_schema().clone(),
            relationship_schema,
            old_schema.graph_properties().clone(),
        );

        let mut store = self.clone();
        store.graph_name = graph_name;
        store.schema = Arc::new(schema);
        store.relationship_topologies = new_relationship_topologies
            .into_iter()
            .map(|(t, topo)| (t, Arc::new(topo)))
            .collect();
        store.relationship_property_stores.clear();
        store.has_relationship_properties = false;

        store.rebuild_relationship_metadata();
        store.refresh_relationship_property_state();

        Ok(store)
    }

    /// Creates a version of this store where all relationship types have inverse indices.
    ///
    /// This enables `graph.stream_inverse_relationships(...)` and `degree_inverse(...)`.
    pub fn with_inverse_indices(
        &self,
        graph_name: GraphName,
    ) -> GraphStoreResult<DefaultGraphStore> {
        self.with_inverse_indices_filtered(graph_name, None)
    }

    /// Creates a version of this store where a subset of relationship types have inverse indices.
    ///
    /// When `relationship_types` is `None`, all types are indexed (same as `with_inverse_indices`).
    /// When provided, only those types are re-built with incoming adjacency; other types are kept
    /// unchanged.
    pub fn with_inverse_indices_filtered(
        &self,
        graph_name: GraphName,
        relationship_types: Option<&HashSet<RelationshipType>>,
    ) -> GraphStoreResult<DefaultGraphStore> {
        let node_count = self.node_count();

        let mut new_relationship_topologies: HashMap<RelationshipType, Arc<RelationshipTopology>> =
            HashMap::new();
        let selected = relationship_types.cloned();

        for (rel_type, topology) in &self.relationship_topologies {
            let should_index = selected
                .as_ref()
                .map(|set| set.contains(rel_type))
                .unwrap_or(true);

            if !should_index {
                new_relationship_topologies.insert(rel_type.clone(), Arc::clone(topology));
                continue;
            }

            let outgoing = topology.outgoing_lists().to_vec();
            let mut incoming: Vec<Vec<MappedNodeId>> = vec![Vec::new(); node_count];

            for source in 0..node_count {
                let source_id = MappedNodeId::try_from(source).map_err(|_| {
                    GraphStoreError::InvalidOperation(
                        "node count exceeds mapped ID space while building inverse topology"
                            .to_string(),
                    )
                })?;
                let Some(neighbors) = topology.outgoing(source_id) else {
                    continue;
                };
                for &target_id in neighbors.iter() {
                    let target = target_id.to_usize().ok_or_else(|| {
                        GraphStoreError::InvalidOperation(format!(
                            "relationship target {target_id} exceeds physical index space"
                        ))
                    })?;
                    incoming[target].push(source_id);
                }
            }

            for adj in incoming.iter_mut() {
                adj.sort_unstable();
            }

            let inverse_topology = RelationshipTopology::try_new(outgoing, Some(incoming))
                .map_err(|error| {
                    GraphStoreError::InvalidOperation(format!(
                        "invalid inverse topology for relationship type '{rel_type}': {error}"
                    ))
                })?;
            new_relationship_topologies.insert(rel_type.clone(), Arc::new(inverse_topology));
        }

        let mut store = self.clone();
        store.graph_name = graph_name;
        store.relationship_topologies = new_relationship_topologies;

        store.rebuild_relationship_metadata();
        store.refresh_relationship_property_state();
        Ok(store)
    }

    pub(crate) fn with_added_relationship_type(
        &self,
        graph_name: GraphName,
        rel_type: RelationshipType,
        outgoing: Vec<Vec<MappedNodeId>>,
        direction: Direction,
    ) -> GraphStoreResult<DefaultGraphStore> {
        self.with_added_relationship_type_and_properties(
            graph_name,
            rel_type,
            outgoing,
            direction,
            Vec::new(),
        )
    }

    pub(crate) fn with_added_relationship_type_and_properties(
        &self,
        graph_name: GraphName,
        rel_type: RelationshipType,
        outgoing: Vec<Vec<MappedNodeId>>,
        direction: Direction,
        property_schemas: Vec<RelationshipPropertySchema>,
    ) -> GraphStoreResult<DefaultGraphStore> {
        let node_count = self.node_count();
        if outgoing.len() != node_count {
            return Err(GraphStoreError::InvalidOperation(format!(
                "outgoing adjacency length {} does not match node_count {node_count}",
                outgoing.len()
            )));
        }

        let topology = RelationshipTopology::try_new(outgoing, None).map_err(|error| {
            GraphStoreError::InvalidOperation(format!(
                "invalid topology for relationship type '{rel_type}': {error}"
            ))
        })?;
        let mut relationship_topologies = self.relationship_topologies.clone();
        relationship_topologies.insert(rel_type.clone(), Arc::new(topology));
        self.validate_retained_relationship_property_cardinalities(&relationship_topologies)?;

        let mut schema = MutableGraphSchema::from_schema(&self.schema);
        let entry = schema
            .relationship_schema_mut()
            .get_or_create_type(rel_type.clone(), direction);
        for property_schema in property_schemas {
            entry.add_property_schema(property_schema);
        }
        let schema = Arc::new(schema.build());

        let mut store = self.clone();
        store.graph_name = graph_name;
        store.schema = schema;
        store.relationship_topologies = relationship_topologies;
        store.rebuild_relationship_metadata();
        store.refresh_relationship_property_state();
        Ok(store)
    }

    pub(crate) fn with_added_relationship_type_preserve_name(
        &self,
        rel_type: RelationshipType,
        outgoing: Vec<Vec<MappedNodeId>>,
        direction: Direction,
    ) -> GraphStoreResult<DefaultGraphStore> {
        self.with_added_relationship_type(self.graph_name.clone(), rel_type, outgoing, direction)
    }

    pub(crate) fn with_rebuilt_relationship_topologies(
        &self,
        graph_name: GraphName,
        relationship_topologies: HashMap<RelationshipType, RelationshipTopology>,
    ) -> GraphStoreResult<DefaultGraphStore> {
        let node_count = self.node_count();
        for topology in relationship_topologies.values() {
            if topology.node_capacity() != node_count {
                return Err(GraphStoreError::InvalidOperation(format!(
                    "relationship topology capacity {} does not match node_count {node_count}",
                    topology.node_capacity()
                )));
            }
        }

        let relationship_topologies = relationship_topologies
            .into_iter()
            .map(|(relationship_type, topology)| (relationship_type, Arc::new(topology)))
            .collect();
        self.validate_retained_relationship_property_cardinalities(&relationship_topologies)?;

        let mut store = self.clone();
        store.graph_name = graph_name;
        store.relationship_topologies = relationship_topologies;
        store.rebuild_relationship_metadata();
        store.refresh_relationship_property_state();
        Ok(store)
    }

    /// Creates a new store with a scaled numeric node property added.
    ///
    /// Currently uses MinMax scaling and writes the result as a Double property.
    pub fn with_scaled_node_property_minmax(
        &self,
        graph_name: GraphName,
        source_property: &str,
        target_property: &str,
        concurrency: usize,
    ) -> GraphStoreResult<DefaultGraphStore> {
        if source_property.is_empty() {
            return Err(GraphStoreError::InvalidOperation(
                "source_property must be non-empty".to_string(),
            ));
        }
        if target_property.is_empty() {
            return Err(GraphStoreError::InvalidOperation(
                "target_property must be non-empty".to_string(),
            ));
        }
        if concurrency == 0 {
            return Err(GraphStoreError::InvalidOperation(
                "concurrency must be > 0".to_string(),
            ));
        }

        let pv = self.node_property_values(source_property)?;
        let node_count = self.node_count() as u64;

        let property_fn: Box<dyn Fn(u64) -> f64 + Send + Sync> = match pv.value_type() {
            ValueType::Long => {
                Box::new(move |node_id: u64| pv.long_value(node_id).unwrap_or(0) as f64)
            }
            ValueType::Double => {
                Box::new(move |node_id: u64| pv.double_value(node_id).unwrap_or(0.0))
            }
            other => {
                return Err(GraphStoreError::InvalidOperation(format!(
                    "scaleProperties expects Long/Double node property (got {other:?})"
                )))
            }
        };

        let scaler: Box<dyn Scaler> = MinMaxScaler::create(node_count, &property_fn, concurrency);
        let mut scaled: Vec<f64> = Vec::with_capacity(node_count as usize);
        for node_id in 0..node_count {
            scaled.push(scaler.scale_property(node_id, property_fn.as_ref()));
        }

        let pv_out: Arc<dyn NodePropertyValues> = Arc::new(DefaultDoubleNodePropertyValues::<
            VecDouble,
        >::from_collection(
            VecDouble::from(scaled),
            node_count as usize,
        ));

        let mut store = self.clone();
        store.graph_name = graph_name;
        store.add_node_property(
            HashSet::from([NodeLabel::all_nodes()]),
            target_property.to_string(),
            pv_out,
        )?;

        Ok(store)
    }

    /// Collapses linear paths by removing degree-2 intermediate nodes.
    ///
    /// Semantics (directed, per relationship type):
    /// - Detect chains where intermediate nodes have exactly 1 incoming and 1 outgoing edge.
    /// - Replace each chain `s -> ... -> t` with a single edge `s -> t`.
    /// - Does not modify the node set.
    /// - Leaves pure directed cycles unchanged (no natural chain start).
    /// - Relationship properties are not carried over.
    pub fn collapse_paths_degree2(
        &self,
        graph_name: GraphName,
        max_hops: Option<usize>,
    ) -> GraphStoreResult<DefaultGraphStore> {
        let node_count = self.node_count();

        let mut new_relationship_topologies: HashMap<RelationshipType, Arc<RelationshipTopology>> =
            HashMap::new();

        for (rel_type, topology) in &self.relationship_topologies {
            let outgoing_lists = topology.outgoing_lists();

            // Compute in/out degrees and unique successor (when out_degree==1).
            let mut out_degree: Vec<usize> = vec![0; node_count];
            let mut in_degree: Vec<usize> = vec![0; node_count];
            let mut succ: Vec<Option<MappedNodeId>> = vec![None; node_count];

            for (u, neighbors) in outgoing_lists.iter().enumerate() {
                out_degree[u] = neighbors.len();
                if neighbors.len() == 1 {
                    succ[u] = Some(neighbors[0]);
                }
                for &v in neighbors {
                    let target_index = v.to_usize().ok_or_else(|| {
                        GraphStoreError::InvalidOperation(format!(
                            "relationship target {v} exceeds physical index space"
                        ))
                    })?;
                    *in_degree.get_mut(target_index).ok_or_else(|| {
                        GraphStoreError::InvalidOperation(format!(
                            "relationship target {v} exceeds graph node count {node_count}"
                        ))
                    })? += 1;
                }
            }

            // Mark nodes whose unique outgoing edge is part of a collapsed chain.
            let mut remove_unique_outgoing: Vec<bool> = vec![false; node_count];
            let mut collapsed_edges: Vec<(MappedNodeId, MappedNodeId)> = Vec::new();

            for s in 0..node_count {
                if out_degree[s] != 1 {
                    continue;
                }
                // Start nodes are where the chain can be entered from "outside".
                if in_degree[s] == 1 {
                    continue;
                }

                let mut next = match succ[s] {
                    Some(v) => v,
                    None => continue,
                };

                let mut hops = 1usize;
                remove_unique_outgoing[s] = true;

                loop {
                    if max_hops.is_some_and(|m| hops >= m) {
                        break;
                    }

                    let next_usize = next.to_usize().ok_or_else(|| {
                        GraphStoreError::InvalidOperation(format!(
                            "relationship target {next} exceeds physical index space"
                        ))
                    })?;
                    if next_usize >= node_count {
                        return Err(GraphStoreError::InvalidOperation(format!(
                            "relationship target {next} exceeds graph node count {node_count}"
                        )));
                    }

                    // Stop if next isn't a strict intermediate.
                    if in_degree[next_usize] != 1 || out_degree[next_usize] != 1 {
                        break;
                    }

                    // Advance through the intermediate.
                    remove_unique_outgoing[next_usize] = true;
                    next = match succ[next_usize] {
                        Some(v) => v,
                        None => break,
                    };
                    hops += 1;
                }

                // Create the collapsed edge from the original start to the terminal.
                let source = MappedNodeId::try_from(s).map_err(|_| {
                    GraphStoreError::InvalidOperation(
                        "graph node count exceeds mapped ID space".to_string(),
                    )
                })?;
                collapsed_edges.push((source, next));
            }

            // Rebuild outgoing adjacency, skipping removed unique edges and adding collapsed edges.
            let mut new_outgoing: Vec<Vec<MappedNodeId>> = vec![Vec::new(); node_count];
            for (u, neighbors) in outgoing_lists.iter().enumerate() {
                if out_degree[u] == 1 && remove_unique_outgoing[u] {
                    continue;
                }
                for &v in neighbors {
                    new_outgoing[u].push(v);
                }
            }

            for (s, t) in collapsed_edges {
                let source_index = s.to_usize().ok_or_else(|| {
                    GraphStoreError::InvalidOperation(format!(
                        "collapsed source {s} exceeds physical index space"
                    ))
                })?;
                new_outgoing
                    .get_mut(source_index)
                    .expect("collapsed source must belong to the graph mapped domain")
                    .push(t);
            }

            for adj in new_outgoing.iter_mut() {
                adj.sort_unstable();
            }

            let rebuilt_topology =
                RelationshipTopology::try_new(new_outgoing, None).map_err(|error| {
                    GraphStoreError::InvalidOperation(format!(
                        "invalid collapsed topology for relationship type '{rel_type}': {error}"
                    ))
                })?;
            new_relationship_topologies.insert(rel_type.clone(), Arc::new(rebuilt_topology));
        }

        let mut store = self.clone();
        store.graph_name = graph_name;
        store.relationship_topologies = new_relationship_topologies;
        store.relationship_property_stores.clear();
        store.has_relationship_properties = false;

        store.rebuild_relationship_metadata();
        store.refresh_relationship_property_state();
        Ok(store)
    }

    /// Projects an induced subgraph into a new [`DefaultGraphStore`].
    ///
    /// - `selected_original_node_ids` are original (external) node ids.
    /// - The returned store reuses this store's config/schema/capabilities.
    /// - Properties are not copied; this is a topology-only projection.
    ///
    /// Returns the new store, an `old_mapped_id -> new_mapped_id` map, and per-type relationship counts kept.
    pub fn commit_induced_subgraph_by_original_node_ids(
        &self,
        graph_name: GraphName,
        selected_original_node_ids: &[OriginalNodeId],
    ) -> GraphStoreResult<InducedSubgraphResult<DefaultGraphStore>> {
        use std::collections::HashSet;

        if selected_original_node_ids.is_empty() {
            return Err(GraphStoreError::InvalidOperation(
                "Selection must be non-empty".to_string(),
            ));
        }

        // Validate selection (no unknown ids, no duplicates), and build old->new mapping.
        let mut seen_original = HashSet::new();
        let mut selected_ordered_old_mapped: Vec<MappedNodeId> =
            Vec::with_capacity(selected_original_node_ids.len());
        let mut old_mapped_to_new: HashMap<MappedNodeId, MappedNodeId> = HashMap::new();

        for (index, &original_id) in selected_original_node_ids.iter().enumerate() {
            if !seen_original.insert(original_id) {
                return Err(GraphStoreError::InvalidOperation(format!(
                    "Duplicate node id in selection: {original_id}"
                )));
            }
            let old_mapped = self
                .id_map
                .safe_to_mapped_node_id(original_id)
                .ok_or_else(|| {
                    GraphStoreError::InvalidOperation(format!(
                        "Unknown node id in selection: {original_id}"
                    ))
                })?;
            let new_mapped = MappedNodeId::try_from(index).map_err(|_| {
                GraphStoreError::InvalidOperation(
                    "induced subgraph node count exceeds mapped ID space".to_string(),
                )
            })?;
            selected_ordered_old_mapped.push(old_mapped);
            old_mapped_to_new.insert(old_mapped, new_mapped);
        }

        let projected_properties = self.project_node_properties(&selected_ordered_old_mapped)?;
        let node_properties = projected_properties.node_properties;
        let node_properties_by_label = projected_properties.property_keys;

        // Build new IdMap, preserving labels.
        let mut new_id_map =
            SimpleIdMap::try_from_original_ids(selected_original_node_ids.iter().copied())
                .map_err(|error| GraphStoreError::InvalidOperation(error.to_string()))?;
        for (new_mapped_index, &original_id) in selected_original_node_ids.iter().enumerate() {
            let new_mapped = MappedNodeId::try_from(new_mapped_index).map_err(|_| {
                GraphStoreError::InvalidOperation(
                    "induced subgraph node count exceeds mapped ID space".to_string(),
                )
            })?;
            let old_mapped = self
                .id_map
                .safe_to_mapped_node_id(original_id)
                .ok_or_else(|| {
                    GraphStoreError::InvalidOperation(format!(
                        "Unknown node id in selection: {original_id}"
                    ))
                })?;
            for label in self.id_map.node_labels(old_mapped) {
                new_id_map.add_node_label(label.clone());
                new_id_map.add_node_id_to_label(new_mapped, label);
            }
        }

        // Induce relationship topologies by type.
        let n = selected_original_node_ids.len();
        let mut relationship_topologies: HashMap<RelationshipType, RelationshipTopology> =
            HashMap::new();
        let mut kept_by_type: HashMap<RelationshipType, usize> = HashMap::new();

        for (rel_type, topology) in &self.relationship_topologies {
            let mut outgoing: Vec<Vec<MappedNodeId>> = vec![Vec::new(); n];

            for (new_source_index, &old_source) in selected_ordered_old_mapped.iter().enumerate() {
                let neighbors = match topology.outgoing(old_source) {
                    Some(neighbors) => neighbors,
                    None => continue,
                };
                for &old_target in neighbors {
                    if let Some(&new_target) = old_mapped_to_new.get(&old_target) {
                        outgoing[new_source_index].push(new_target);
                    }
                }
            }

            let incoming = if topology.is_inverse_indexed() {
                let mut incoming: Vec<Vec<MappedNodeId>> = vec![Vec::new(); n];
                for (source_index, neighbors) in outgoing.iter().enumerate() {
                    let source_id = MappedNodeId::try_from(source_index).map_err(|_| {
                        GraphStoreError::InvalidOperation(
                            "induced subgraph node count exceeds mapped ID space".to_string(),
                        )
                    })?;
                    for &target in neighbors {
                        let target_index = target.to_usize().ok_or_else(|| {
                            GraphStoreError::InvalidOperation(format!(
                                "induced relationship target {target} exceeds physical index space"
                            ))
                        })?;
                        incoming
                            .get_mut(target_index)
                            .expect("induced target must belong to selected mapped domain")
                            .push(source_id);
                    }
                }
                Some(incoming)
            } else {
                None
            };

            let induced = RelationshipTopology::try_new(outgoing, incoming).map_err(|error| {
                GraphStoreError::InvalidOperation(format!(
                    "invalid induced topology for relationship type '{rel_type}': {error}"
                ))
            })?;
            let kept = induced.relationship_count();
            if kept > 0 {
                kept_by_type.insert(rel_type.clone(), kept);
                relationship_topologies.insert(rel_type.clone(), induced);
            }
        }

        let relationship_property_stores = self.project_relationship_properties(
            &selected_ordered_old_mapped,
            &old_mapped_to_new,
            &relationship_topologies,
        )?;

        let store = DefaultGraphStore::try_new(
            self.config.as_ref().clone(),
            graph_name,
            self.database_info.clone(),
            self.schema.as_ref().clone(),
            self.capabilities.clone(),
            new_id_map,
            relationship_topologies,
        )?;

        let mut store = store;
        let mut projected_node_properties = DefaultNodePropertyStore::empty();
        for (key, values) in node_properties {
            let source = self
                .node_properties
                .get(&key)
                .ok_or_else(|| GraphStoreError::PropertyNotFound(key.clone()))?;
            projected_node_properties
                .add_column(NodeProperty::with_schema(
                    source.property_schema().clone(),
                    values,
                ))
                .map_err(|error| GraphStoreError::InvalidOperation(error.to_string()))?;
        }
        store.node_properties = projected_node_properties;
        store.node_properties_by_label = node_properties_by_label;
        store.graph_properties = self.graph_properties.clone();
        store.relationship_property_stores = relationship_property_stores;
        store.refresh_relationship_property_state();
        store.set_modified();

        Ok(InducedSubgraphResult {
            store,
            old_to_new_mapping: old_mapped_to_new,
            relationships_kept_by_type: kept_by_type,
        })
    }

    fn project_node_properties(
        &self,
        selected_ordered_old_mapped: &[MappedNodeId],
    ) -> GraphStoreResult<ProjectedPropertiesResult> {
        let node_count = selected_ordered_old_mapped.len();
        let mut projected: HashMap<String, Arc<dyn NodePropertyValues>> = HashMap::new();

        for property in self.node_properties.columns() {
            let key = property.key();
            let values = property.values();
            match values.value_type() {
                ValueType::Double => {
                    let mut data = Vec::with_capacity(node_count);
                    for &old_id in selected_ordered_old_mapped {
                        let v = values
                            .double_value(old_id.get())
                            .map_err(|err| GraphStoreError::InvalidOperation(format!("{err}")))?;
                        data.push(v);
                    }
                    let cfg = self.config.node_collections_config::<f64>(node_count);
                    let backend = create_double_backend_from_config(&cfg, data);
                    let pv = build_node_double_property_values(backend, node_count);
                    projected.insert(key.to_string(), pv);
                }
                ValueType::Long => {
                    let mut data = Vec::with_capacity(node_count);
                    for &old_id in selected_ordered_old_mapped {
                        let v = values
                            .long_value(old_id.get())
                            .map_err(|err| GraphStoreError::InvalidOperation(format!("{err}")))?;
                        data.push(v);
                    }
                    let cfg = self.config.node_collections_config::<i64>(node_count);
                    let backend = create_long_backend_from_config(&cfg, data);
                    let pv = build_node_long_property_values(backend, node_count);
                    projected.insert(key.to_string(), pv);
                }
                ValueType::Float => {
                    let mut data = Vec::with_capacity(node_count);
                    for &old_id in selected_ordered_old_mapped {
                        let v = values
                            .double_value(old_id.get())
                            .map_err(|err| GraphStoreError::InvalidOperation(format!("{err}")))?;
                        data.push(v as f32);
                    }
                    let cfg = self.config.node_collections_config::<f32>(node_count);
                    let backend = create_float_backend_from_config(&cfg, data);
                    let pv = build_node_float_property_values(backend, node_count);
                    projected.insert(key.to_string(), pv);
                }
                ValueType::Int => {
                    let mut data = Vec::with_capacity(node_count);
                    for &old_id in selected_ordered_old_mapped {
                        let v = values
                            .long_value(old_id.get())
                            .map_err(|err| GraphStoreError::InvalidOperation(format!("{err}")))?;
                        data.push(v as i32);
                    }
                    let cfg = self.config.node_collections_config::<i32>(node_count);
                    let backend = create_int_backend_from_config(&cfg, data);
                    let pv = build_node_int_property_values(backend, node_count);
                    projected.insert(key.to_string(), pv);
                }
                ValueType::DoubleArray => {
                    let mut data: Vec<Option<Vec<f64>>> = Vec::with_capacity(node_count);
                    for &old_id in selected_ordered_old_mapped {
                        let v = values
                            .double_array_value(old_id.get())
                            .ok()
                            .map(Some)
                            .unwrap_or(None);
                        data.push(v);
                    }
                    let backend = VecDoubleArray::from(data);
                    let pv = build_node_double_array_property_values(backend, node_count);
                    projected.insert(key.to_string(), pv);
                }
                ValueType::LongArray => {
                    let mut data: Vec<Option<Vec<i64>>> = Vec::with_capacity(node_count);
                    for &old_id in selected_ordered_old_mapped {
                        let v = values
                            .long_array_value(old_id.get())
                            .ok()
                            .map(Some)
                            .unwrap_or(None);
                        data.push(v);
                    }
                    let backend = VecLongArray::from(data);
                    let pv = build_node_long_array_property_values(backend, node_count);
                    projected.insert(key.to_string(), pv);
                }
                _ => {
                    // Skip unsupported projection types for now.
                    continue;
                }
            }
        }

        let mut projected_by_label: HashMap<String, HashSet<String>> = HashMap::new();
        for (label_key, keys) in &self.node_properties_by_label {
            for key in keys {
                if projected.contains_key(key) {
                    projected_by_label
                        .entry(label_key.clone())
                        .or_default()
                        .insert(key.clone());
                }
            }
        }

        Ok(ProjectedPropertiesResult {
            node_properties: projected,
            property_keys: projected_by_label,
        })
    }

    fn project_relationship_properties(
        &self,
        selected_ordered_old_mapped: &[MappedNodeId],
        old_mapped_to_new: &HashMap<MappedNodeId, MappedNodeId>,
        new_relationship_topologies: &HashMap<RelationshipType, RelationshipTopology>,
    ) -> GraphStoreResult<HashMap<RelationshipType, DefaultRelationshipPropertyStore>> {
        let mut projected = HashMap::new();

        for (rel_type, new_topology) in new_relationship_topologies {
            let old_topology = match self.relationship_topologies.get(rel_type) {
                Some(t) => t,
                None => continue,
            };
            let new_count = new_topology.relationship_count();
            if new_count == 0 {
                continue;
            }

            let old_store = match self.relationship_property_stores.get(rel_type) {
                Some(store) if !store.is_empty() => store,
                _ => continue,
            };

            let mut builder = DefaultRelationshipPropertyStore::builder();

            for property in old_store.columns() {
                let values = property.values();
                match values.value_type() {
                    ValueType::Double => {
                        let mut data = Vec::with_capacity(new_count);
                        for &old_source in selected_ordered_old_mapped {
                            if let Some(old_neighbors) = old_topology.outgoing(old_source) {
                                for (neighbor_idx, &old_target) in old_neighbors.iter().enumerate()
                                {
                                    if old_mapped_to_new.contains_key(&old_target) {
                                        let old_index = old_topology
                                            .relationship_index(old_source, neighbor_idx)
                                            .ok_or_else(|| {
                                                GraphStoreError::InvalidOperation(format!(
                                                    "missing canonical relationship index for {old_source} at offset {neighbor_idx}"
                                                ))
                                            })?;
                                        let v = values.double_value(old_index).map_err(|err| {
                                            GraphStoreError::InvalidOperation(format!("{err}"))
                                        })?;
                                        data.push(v);
                                    }
                                }
                            }
                        }

                        if data.is_empty() {
                            continue;
                        }

                        let cfg = self
                            .config
                            .relationship_collections_config::<f64>(data.len());
                        let backend = create_double_backend_from_config(&cfg, data);
                        let pv = build_relationship_double_property_values(backend, new_count);
                        let projected_property = RelationshipProperty::with_schema(
                            property.property_schema().clone(),
                            pv,
                        );
                        builder = builder.put(projected_property);
                    }
                    ValueType::Long => {
                        let mut data = Vec::with_capacity(new_count);
                        for &old_source in selected_ordered_old_mapped {
                            if let Some(old_neighbors) = old_topology.outgoing(old_source) {
                                for (neighbor_idx, &old_target) in old_neighbors.iter().enumerate()
                                {
                                    if old_mapped_to_new.contains_key(&old_target) {
                                        let old_index = old_topology
                                            .relationship_index(old_source, neighbor_idx)
                                            .ok_or_else(|| {
                                                GraphStoreError::InvalidOperation(format!(
                                                    "missing canonical relationship index for {old_source} at offset {neighbor_idx}"
                                                ))
                                            })?;
                                        let v = values.long_value(old_index).map_err(|err| {
                                            GraphStoreError::InvalidOperation(format!("{err}"))
                                        })?;
                                        data.push(v);
                                    }
                                }
                            }
                        }

                        if data.is_empty() {
                            continue;
                        }

                        let cfg = self
                            .config
                            .relationship_collections_config::<i64>(data.len());
                        let backend = create_long_backend_from_config(&cfg, data);
                        let pv = build_relationship_long_property_values(backend, new_count);
                        let projected_property = RelationshipProperty::with_schema(
                            property.property_schema().clone(),
                            pv,
                        );
                        builder = builder.put(projected_property);
                    }
                    ValueType::Int => {
                        let mut data = Vec::with_capacity(new_count);
                        for &old_source in selected_ordered_old_mapped {
                            if let Some(old_neighbors) = old_topology.outgoing(old_source) {
                                for (neighbor_idx, &old_target) in old_neighbors.iter().enumerate()
                                {
                                    if old_mapped_to_new.contains_key(&old_target) {
                                        let old_index = old_topology
                                            .relationship_index(old_source, neighbor_idx)
                                            .ok_or_else(|| {
                                                GraphStoreError::InvalidOperation(format!(
                                                    "missing canonical relationship index for {old_source} at offset {neighbor_idx}"
                                                ))
                                            })?;
                                        let v = values.long_value(old_index).map_err(|err| {
                                            GraphStoreError::InvalidOperation(format!("{err}"))
                                        })?;
                                        data.push(v as i32);
                                    }
                                }
                            }
                        }

                        if data.is_empty() {
                            continue;
                        }

                        let cfg = self
                            .config
                            .relationship_collections_config::<i32>(data.len());
                        let backend = create_int_backend_from_config(&cfg, data);
                        let pv = build_relationship_int_property_values(backend, new_count);
                        let projected_property = RelationshipProperty::with_schema(
                            property.property_schema().clone(),
                            pv,
                        );
                        builder = builder.put(projected_property);
                    }
                    _ => continue,
                }
            }

            let projected_store = builder.build();
            if !projected_store.is_empty() {
                projected.insert(rel_type.clone(), projected_store);
            }
        }

        Ok(projected)
    }

    fn set_modified(&mut self) {
        self.modification_time = Utc::now();
    }

    fn schema_labels(&self) -> HashSet<NodeLabel> {
        self.id_map
            .available_node_labels()
            .into_iter()
            .map(|label| NodeLabel::of(label.name()))
            .collect()
    }

    fn node_schema_labels_for_mutation(&self) -> HashSet<NodeLabel> {
        let labels = self.schema.node_schema().available_labels();
        if labels.is_empty() {
            HashSet::from([NodeLabel::all_nodes()])
        } else {
            labels
        }
    }

    // === Property Management with Config ===

    /// Add a node property with i64 values using the store's config for backend selection.
    pub fn add_node_property_i64(
        &mut self,
        key: String,
        values: Vec<i64>,
    ) -> Result<(), GraphStoreError> {
        let node_count = self.id_map.node_count();

        // Use config to create CollectionsConfig
        let collections_config = self.config.node_collections_config::<i64>(node_count);

        // Create property using config
        let backend = create_long_backend_from_config(&collections_config, values);
        let pv = build_node_long_property_values(backend, node_count);

        let labels = self.node_schema_labels_for_mutation();
        self.add_node_property(labels, key, pv)
    }

    /// Add a node property with f64 values using the store's config for backend selection.
    pub fn add_node_property_f64(
        &mut self,
        key: String,
        values: Vec<f64>,
    ) -> Result<(), GraphStoreError> {
        let node_count = self.id_map.node_count();

        // Use config to create CollectionsConfig
        let collections_config = self.config.node_collections_config::<f64>(node_count);

        // Create property using config
        let backend = create_double_backend_from_config(&collections_config, values);
        let pv = build_node_double_property_values(backend, node_count);

        let labels = self.node_schema_labels_for_mutation();
        self.add_node_property(labels, key, pv)
    }

    /// Add a graph property with i64 values using the store's config for backend selection.
    pub fn add_graph_property_i64(
        &mut self,
        key: String,
        values: Vec<i64>,
    ) -> Result<(), GraphStoreError> {
        // Use config to create CollectionsConfig (graph properties don't scale with node count)
        let collections_config = self.config.graph_collections_config::<i64>(values.len());

        // Create property using config
        let backend = create_long_backend_from_config(&collections_config, values);
        let pv = build_graph_long_property_values(backend);

        self.add_graph_property(key, pv)
    }

    /// Add a graph property with f64 values using the store's config for backend selection.
    pub fn add_graph_property_f64(
        &mut self,
        key: String,
        values: Vec<f64>,
    ) -> Result<(), GraphStoreError> {
        // Use config to create CollectionsConfig
        let collections_config = self.config.graph_collections_config::<f64>(values.len());

        // Create property using config
        let backend = create_double_backend_from_config(&collections_config, values);
        let pv = build_graph_double_property_values(backend);

        self.add_graph_property(key, pv)
    }

    fn to_schema_label(label: &NodeLabel) -> NodeLabel {
        NodeLabel::of(label.name())
    }

    fn label_key(label: &NodeLabel) -> String {
        label.name().to_string()
    }

    fn rebuild_relationship_metadata(&mut self) {
        let mut ordered: Vec<RelationshipType> =
            self.relationship_topologies.keys().cloned().collect();
        ordered.sort_by(|left, right| left.name().cmp(right.name()));

        let mut inverse_indexed = HashSet::new();
        let mut relationship_count = 0usize;
        let mut has_parallel = false;

        for rel_type in &ordered {
            if let Some(topology) = self.relationship_topologies.get(rel_type) {
                if topology.is_inverse_indexed() {
                    inverse_indexed.insert(rel_type.clone());
                }
                if topology.has_parallel_edges() {
                    has_parallel = true;
                }
                relationship_count += topology.relationship_count();
            }
        }

        let all_inverse_indexed = !ordered.is_empty()
            && ordered
                .iter()
                .all(|rel_type| inverse_indexed.contains(rel_type));

        let mut characteristics_builder = GraphCharacteristicsBuilder::new();
        for entry in self.schema.relationship_schema().entries() {
            match entry.direction() {
                Direction::Directed => {
                    characteristics_builder = characteristics_builder.directed();
                }
                Direction::Undirected => {
                    characteristics_builder = characteristics_builder.undirected();
                }
            }
        }

        if self.schema.relationship_schema().entries().is_empty() && !ordered.is_empty() {
            // Bootstrap stores may still be assembled before schema compilation.
            // Keep topology usable without claiming an undirected schema fact.
            characteristics_builder = characteristics_builder.directed();
        }

        if all_inverse_indexed {
            characteristics_builder = characteristics_builder.inverse_indexed();
        }

        self.ordered_relationship_types = ordered;
        self.inverse_indexed_relationship_types = inverse_indexed;
        self.relationship_count = relationship_count;
        self.has_parallel_relationships = has_parallel;
        self.graph_characteristics = characteristics_builder.build();
    }

    fn refresh_relationship_property_state(&mut self) {
        self.has_relationship_properties = self
            .relationship_property_stores
            .values()
            .any(|store| !store.is_empty());
    }

    fn validate_retained_relationship_property_cardinalities(
        &self,
        relationship_topologies: &HashMap<RelationshipType, Arc<RelationshipTopology>>,
    ) -> GraphStoreResult<()> {
        for (relationship_type, property_store) in &self.relationship_property_stores {
            if property_store.is_empty() {
                continue;
            }
            let topology = relationship_topologies.get(relationship_type).ok_or_else(|| {
                GraphStoreError::InvalidOperation(format!(
                    "cannot retain properties for relationship type '{relationship_type}' without its topology"
                ))
            })?;
            let expected_count = topology.relationship_count();
            for property in property_store.columns() {
                let materialized_count = property.values().element_count();
                if materialized_count != expected_count {
                    return Err(GraphStoreError::InvalidOperation(format!(
                        "cannot retain relationship property '{}' for type '{relationship_type}': column cardinality {materialized_count} does not match rebuilt topology cardinality {expected_count}",
                        property.key()
                    )));
                }
            }
        }
        Ok(())
    }

    fn validate_graph_view_spec(&self, spec: &GraphViewSpec) -> GraphViewResult<()> {
        let selected_types = if spec.relationship_types().is_empty() {
            self.relationship_topologies.keys().cloned().collect()
        } else {
            let mut requested_types = spec.relationship_types().iter().collect::<Vec<_>>();
            requested_types.sort_by(|left, right| left.name().cmp(right.name()));
            for relationship_type in requested_types {
                if !self.relationship_topologies.contains_key(relationship_type) {
                    return Err(GraphViewError::RelationshipTypeNotMaterialized(
                        relationship_type.name().to_string(),
                    ));
                }
            }
            spec.relationship_types().clone()
        };

        let mut selectors = spec
            .relationship_property_selectors()
            .iter()
            .collect::<Vec<_>>();
        selectors.sort_by(|(left, _), (right, _)| left.name().cmp(right.name()));

        for (relationship_type, property_key) in selectors {
            if !selected_types.contains(relationship_type) {
                return Err(GraphViewError::SelectorForUnselectedType(
                    relationship_type.name().to_string(),
                ));
            }

            let property_is_materialized = self
                .relationship_property_stores
                .get(relationship_type)
                .is_some_and(|store| store.contains_key(property_key));
            if !property_is_materialized {
                return Err(GraphViewError::RelationshipPropertyNotMaterialized {
                    relationship_type: relationship_type.name().to_string(),
                    property_key: property_key.clone(),
                });
            }
        }

        Ok(())
    }
}

impl GraphStore for DefaultGraphStore {
    fn database_info(&self) -> &DatabaseInfo {
        &self.database_info
    }

    fn schema(&self) -> &GraphSchema {
        &self.schema
    }

    fn creation_time(&self) -> DateTime<Utc> {
        self.creation_time
    }

    fn modification_time(&self) -> DateTime<Utc> {
        self.modification_time
    }

    fn capabilities(&self) -> &Capabilities {
        &self.capabilities
    }

    fn nodes(&self) -> Arc<dyn IdMap> {
        Arc::clone(&self.id_map) as Arc<dyn IdMap>
    }

    fn graph_property_keys(&self) -> HashSet<String> {
        self.graph_properties
            .key_set()
            .into_iter()
            .map(str::to_string)
            .collect()
    }

    fn has_graph_property(&self, property_key: &str) -> bool {
        self.graph_properties.contains_key(property_key)
    }

    fn graph_property_type(&self, property_key: &str) -> GraphStoreResult<ValueType> {
        self.graph_properties
            .get(property_key)
            .map(|property| property.property_schema().value_type())
            .ok_or_else(|| GraphStoreError::PropertyNotFound(property_key.to_string()))
    }

    fn graph_property_values(
        &self,
        property_key: &str,
    ) -> GraphStoreResult<Arc<dyn GraphPropertyValues>> {
        self.graph_properties
            .get(property_key)
            .map(|property| property.values_arc())
            .ok_or_else(|| GraphStoreError::PropertyNotFound(property_key.to_string()))
    }

    fn add_graph_property(
        &mut self,
        property_key: impl Into<String>,
        property_values: Arc<dyn GraphPropertyValues>,
    ) -> GraphStoreResult<()> {
        self.add_graph_property_column(GraphProperty::with_state(
            property_key,
            PropertyState::Persistent,
            property_values,
        ))
    }

    fn add_graph_property_column(&mut self, property: GraphProperty) -> GraphStoreResult<()> {
        let key = property.key().to_string();
        if property.property_schema().value_type() != property.values().value_type() {
            return Err(GraphStoreError::SchemaError(format!(
                "graph property '{key}' declares {:?} but values are {:?}",
                property.property_schema().value_type(),
                property.values().value_type()
            )));
        }
        let mut schema = MutableGraphSchema::from_schema(&self.schema);
        schema.put_graph_property(key, property.property_schema().clone());
        self.graph_properties
            .add_column(property)
            .map_err(|error| GraphStoreError::InvalidOperation(error.to_string()))?;
        self.schema = Arc::new(schema.build());
        self.set_modified();
        Ok(())
    }

    fn replace_graph_property_column(
        &mut self,
        property: GraphProperty,
    ) -> GraphStoreResult<GraphProperty> {
        let key = property.key().to_string();
        if property.property_schema().value_type() != property.values().value_type() {
            return Err(GraphStoreError::SchemaError(format!(
                "graph property '{key}' declares {:?} but values are {:?}",
                property.property_schema().value_type(),
                property.values().value_type()
            )));
        }
        let mut schema = MutableGraphSchema::from_schema(&self.schema);
        schema.put_graph_property(key, property.property_schema().clone());
        let replaced = self
            .graph_properties
            .replace_column(property)
            .map_err(|error| GraphStoreError::InvalidOperation(error.to_string()))?;
        self.schema = Arc::new(schema.build());
        self.set_modified();
        Ok(replaced)
    }

    fn remove_graph_property(&mut self, property_key: &str) -> GraphStoreResult<()> {
        self.graph_properties
            .remove_column(property_key)
            .map_err(|error| GraphStoreError::InvalidOperation(error.to_string()))?;
        let mut schema = MutableGraphSchema::from_schema(&self.schema);
        schema.remove_graph_property(property_key);
        self.schema = Arc::new(schema.build());
        self.set_modified();
        Ok(())
    }

    fn node_count(&self) -> usize {
        self.id_map.node_count()
    }

    fn node_count_for_label(&self, label: &NodeLabel) -> usize {
        let schema_label = Self::to_schema_label(label);
        self.id_map.node_count_for_label(&schema_label)
    }

    fn node_labels(&self) -> HashSet<NodeLabel> {
        self.schema_labels()
    }

    fn has_node_label(&self, label: &NodeLabel) -> bool {
        let schema_label = Self::to_schema_label(label);
        self.schema_labels().contains(&schema_label)
    }

    fn add_node_label(&mut self, node_label: NodeLabel) -> GraphStoreResult<()> {
        let schema_label = Self::to_schema_label(&node_label);
        Arc::make_mut(&mut self.id_map).add_node_label(schema_label.clone());
        let mut schema = MutableGraphSchema::from_schema(&self.schema);
        schema.node_schema_mut().add_label(schema_label);
        self.schema = Arc::new(schema.build());
        self.set_modified();
        Ok(())
    }

    fn node_property_keys(&self) -> HashSet<String> {
        self.node_properties
            .key_set()
            .into_iter()
            .map(str::to_string)
            .collect()
    }

    fn node_property_keys_for_label(&self, label: &NodeLabel) -> HashSet<String> {
        self.node_properties_by_label
            .get(&Self::label_key(label))
            .cloned()
            .unwrap_or_default()
    }

    fn node_property_keys_for_labels(&self, labels: &HashSet<NodeLabel>) -> HashSet<String> {
        if labels.is_empty() {
            return self.node_property_keys();
        }

        let mut iter = labels.iter();
        let first = iter.next().unwrap();
        let mut intersection = self.node_property_keys_for_label(first);

        for label in iter {
            let keys = self.node_property_keys_for_label(label);
            intersection = intersection
                .intersection(&keys)
                .cloned()
                .collect::<HashSet<_>>();
        }

        intersection
    }

    fn has_node_property(&self, property_key: &str) -> bool {
        self.node_properties.contains_key(property_key)
    }

    fn has_node_property_for_label(&self, label: &NodeLabel, property_key: &str) -> bool {
        self.node_properties_by_label
            .get(&Self::label_key(label))
            .map(|keys| keys.contains(property_key))
            .unwrap_or(false)
    }

    fn node_property_type(&self, property_key: &str) -> GraphStoreResult<ValueType> {
        self.node_properties
            .get(property_key)
            .map(|property| property.property_schema().value_type())
            .ok_or_else(|| GraphStoreError::PropertyNotFound(property_key.to_string()))
    }

    fn node_property_values(
        &self,
        property_key: &str,
    ) -> GraphStoreResult<Arc<dyn NodePropertyValues>> {
        self.node_properties
            .get(property_key)
            .map(|property| property.values_arc())
            .ok_or_else(|| GraphStoreError::PropertyNotFound(property_key.to_string()))
    }

    fn add_node_property(
        &mut self,
        node_labels: HashSet<NodeLabel>,
        property_key: impl Into<String>,
        property_values: Arc<dyn NodePropertyValues>,
    ) -> GraphStoreResult<()> {
        let property =
            NodeProperty::with_state(property_key, PropertyState::Persistent, property_values);
        if self.node_properties.contains_key(property.key()) {
            self.replace_node_property_column(node_labels, property)
                .map(|_| ())
        } else {
            self.add_node_property_column(node_labels, property)
        }
    }

    fn add_node_property_column(
        &mut self,
        node_labels: HashSet<NodeLabel>,
        property: NodeProperty,
    ) -> GraphStoreResult<()> {
        let key = property.key().to_string();
        let node_labels = if node_labels.is_empty() {
            HashSet::from([NodeLabel::all_nodes()])
        } else {
            node_labels
        };
        if property.property_schema().value_type() != property.values().value_type() {
            return Err(GraphStoreError::SchemaError(format!(
                "node property '{key}' declares {:?} but values are {:?}",
                property.property_schema().value_type(),
                property.values().value_type()
            )));
        }
        let property_values = property.values_arc();
        if property_values.element_count() != self.node_count() {
            return Err(GraphStoreError::InvalidOperation(format!(
                "node property '{key}' has {} values but the node domain requires {}",
                property_values.element_count(),
                self.node_count()
            )));
        }
        for label in &node_labels {
            if !label.is_all_nodes() && !self.has_node_label(label) {
                return Err(GraphStoreError::NodeLabelNotFound(label.name().to_string()));
            }
        }

        let mut schema = MutableGraphSchema::from_schema(&self.schema);
        for label in &node_labels {
            schema
                .node_schema_mut()
                .get_or_create_label(label.clone())
                .add_property_schema(property.property_schema().clone());
        }
        self.node_properties
            .add_column(property)
            .map_err(|error| GraphStoreError::InvalidOperation(error.to_string()))?;

        for label in node_labels {
            let label_key = Self::label_key(&label);
            self.node_properties_by_label
                .entry(label_key)
                .or_default()
                .insert(key.clone());
        }

        self.schema = Arc::new(schema.build());
        self.set_modified();
        Ok(())
    }

    fn replace_node_property_column(
        &mut self,
        node_labels: HashSet<NodeLabel>,
        property: NodeProperty,
    ) -> GraphStoreResult<NodeProperty> {
        let key = property.key().to_string();
        let node_labels = if node_labels.is_empty() {
            HashSet::from([NodeLabel::all_nodes()])
        } else {
            node_labels
        };
        if property.property_schema().value_type() != property.values().value_type() {
            return Err(GraphStoreError::SchemaError(format!(
                "node property '{key}' declares {:?} but values are {:?}",
                property.property_schema().value_type(),
                property.values().value_type()
            )));
        }
        if property.values().element_count() != self.node_count() {
            return Err(GraphStoreError::InvalidOperation(format!(
                "node property '{key}' has {} values but the node domain requires {}",
                property.values().element_count(),
                self.node_count()
            )));
        }
        for label in &node_labels {
            if !label.is_all_nodes() && !self.has_node_label(label) {
                return Err(GraphStoreError::NodeLabelNotFound(label.name().to_string()));
            }
        }

        let mut schema = MutableGraphSchema::from_schema(&self.schema);
        for label in self.schema.node_schema().available_labels() {
            schema
                .node_schema_mut()
                .get_or_create_label(label)
                .remove_property(&key);
        }
        for label in &node_labels {
            schema
                .node_schema_mut()
                .get_or_create_label(label.clone())
                .add_property_schema(property.property_schema().clone());
        }
        let replaced = self
            .node_properties
            .replace_column(property)
            .map_err(|error| GraphStoreError::InvalidOperation(error.to_string()))?;
        for keys in self.node_properties_by_label.values_mut() {
            keys.remove(&key);
        }
        for label in node_labels {
            self.node_properties_by_label
                .entry(Self::label_key(&label))
                .or_default()
                .insert(key.clone());
        }
        self.schema = Arc::new(schema.build());
        self.set_modified();
        Ok(replaced)
    }

    fn remove_node_property(&mut self, property_key: &str) -> GraphStoreResult<()> {
        self.node_properties
            .remove_column(property_key)
            .map_err(|error| GraphStoreError::InvalidOperation(error.to_string()))?;
        for keys in self.node_properties_by_label.values_mut() {
            keys.remove(property_key);
        }
        let mut schema = MutableGraphSchema::from_schema(&self.schema);
        for label in self.schema.node_schema().available_labels() {
            schema
                .node_schema_mut()
                .get_or_create_label(label)
                .remove_property(property_key);
        }
        self.schema = Arc::new(schema.build());
        self.set_modified();
        Ok(())
    }

    fn relationship_count(&self) -> usize {
        self.relationship_count
    }

    fn relationship_count_for_type(&self, relationship_type: &RelationshipType) -> usize {
        self.relationship_topologies
            .get(relationship_type)
            .map(|topology| topology.relationship_count())
            .unwrap_or(0)
    }

    fn relationship_types(&self) -> HashSet<RelationshipType> {
        let mut types: HashSet<RelationshipType> =
            self.relationship_topologies.keys().cloned().collect();

        for schema_type in self.schema.relationship_schema().available_types() {
            types.insert(RelationshipType::of(schema_type.name()));
        }

        types
    }

    fn has_relationship_type(&self, relationship_type: &RelationshipType) -> bool {
        self.relationship_types().contains(relationship_type)
    }

    fn inverse_indexed_relationship_types(&self) -> HashSet<RelationshipType> {
        self.inverse_indexed_relationship_types.clone()
    }

    fn relationship_property_keys(&self) -> HashSet<String> {
        self.relationship_property_stores
            .values()
            .flat_map(|store| store.key_set().into_iter().map(str::to_string))
            .collect()
    }

    fn relationship_property_keys_for_type(&self, rel_type: &RelationshipType) -> HashSet<String> {
        self.relationship_property_stores
            .get(rel_type)
            .map(|store| store.key_set().into_iter().map(str::to_string).collect())
            .unwrap_or_default()
    }

    fn relationship_property_keys_for_types(
        &self,
        rel_types: &HashSet<RelationshipType>,
    ) -> HashSet<String> {
        if rel_types.is_empty() {
            return self.relationship_property_keys();
        }

        let mut rel_types = rel_types.iter();
        let Some(first) = rel_types.next() else {
            return HashSet::new();
        };
        let mut intersection = self.relationship_property_keys_for_type(first);

        for rel_type in rel_types {
            let keys = self.relationship_property_keys_for_type(rel_type);
            intersection.retain(|key| keys.contains(key));
        }

        intersection
    }

    fn has_relationship_property(&self, rel_type: &RelationshipType, property_key: &str) -> bool {
        self.relationship_property_stores
            .get(rel_type)
            .map(|store| store.contains_key(property_key))
            .unwrap_or(false)
    }

    fn relationship_property_type(
        &self,
        relationship_type: &RelationshipType,
        property_key: &str,
    ) -> GraphStoreResult<ValueType> {
        self.relationship_property_stores
            .get(relationship_type)
            .and_then(|store| store.get(property_key))
            .map(|property| property.property_schema().value_type())
            .ok_or_else(|| GraphStoreError::PropertyNotFound(property_key.to_string()))
    }

    fn relationship_property_values(
        &self,
        relationship_type: &RelationshipType,
        property_key: &str,
    ) -> GraphStoreResult<Arc<dyn RelationshipPropertyValues>> {
        self.relationship_property_stores
            .get(relationship_type)
            .and_then(|store| store.get(property_key))
            .map(|property| property.values_arc())
            .ok_or_else(|| GraphStoreError::PropertyNotFound(property_key.to_string()))
    }

    fn add_relationship_property(
        &mut self,
        relationship_type: RelationshipType,
        property_key: impl Into<String>,
        property_values: Arc<dyn RelationshipPropertyValues>,
    ) -> GraphStoreResult<()> {
        let key = property_key.into();
        let schema_entry = self
            .schema
            .relationship_schema()
            .get(&relationship_type)
            .ok_or_else(|| {
                GraphStoreError::SchemaError(format!(
                    "relationship type '{relationship_type}' is materialized but absent from the schema"
                ))
            })?;
        let column_schema = if let Some(schema) = schema_entry.properties().get(&key) {
            schema.clone()
        } else {
            RelationshipPropertySchema::with_aggregation(
                key,
                property_values.value_type(),
                DefaultValue::of(property_values.value_type()),
                PropertyState::Persistent,
                Aggregation::None,
            )
        };
        let property = RelationshipProperty::try_with_schema(column_schema, property_values)
            .map_err(|error| GraphStoreError::SchemaError(error.to_string()))?;
        self.add_relationship_property_column(relationship_type, property)
    }

    fn add_relationship_property_column(
        &mut self,
        relationship_type: RelationshipType,
        property: RelationshipProperty,
    ) -> GraphStoreResult<()> {
        let key = property.key().to_string();
        let property_values = property.values_arc();
        let column_schema = property.property_schema().clone();
        if column_schema.value_type() != property_values.value_type() {
            return Err(GraphStoreError::SchemaError(format!(
                "relationship property '{key}' declares {:?} but values are {:?}",
                column_schema.value_type(),
                property_values.value_type()
            )));
        }
        let expected_count = self
            .relationship_topologies
            .get(&relationship_type)
            .ok_or_else(|| {
                GraphStoreError::RelationshipTypeNotFound(relationship_type.name().to_string())
            })?
            .relationship_count();
        if property_values.element_count() != expected_count {
            return Err(GraphStoreError::InvalidOperation(format!(
                "relationship property '{key}' for type '{relationship_type}' has {} values but the topology requires {expected_count}",
                property_values.element_count()
            )));
        }

        let schema_entry = self
            .schema
            .relationship_schema()
            .get(&relationship_type)
            .ok_or_else(|| {
                GraphStoreError::SchemaError(format!(
                    "relationship type '{relationship_type}' is materialized but absent from the schema"
                ))
            })?;
        let direction = schema_entry.direction();

        let mut schema = MutableGraphSchema::from_schema(&self.schema);
        schema.relationship_schema_mut().add_property_schema(
            relationship_type.clone(),
            direction,
            column_schema,
        );
        self.relationship_property_stores
            .entry(relationship_type)
            .or_insert_with(DefaultRelationshipPropertyStore::empty)
            .add_column(property)
            .map_err(|error| GraphStoreError::InvalidOperation(error.to_string()))?;
        self.schema = Arc::new(schema.build());

        self.refresh_relationship_property_state();
        self.set_modified();
        Ok(())
    }

    fn replace_relationship_property_column(
        &mut self,
        relationship_type: RelationshipType,
        property: RelationshipProperty,
    ) -> GraphStoreResult<RelationshipProperty> {
        let key = property.key().to_string();
        if property.property_schema().value_type() != property.values().value_type() {
            return Err(GraphStoreError::SchemaError(format!(
                "relationship property '{key}' declares {:?} but values are {:?}",
                property.property_schema().value_type(),
                property.values().value_type()
            )));
        }
        let expected_count = self
            .relationship_topologies
            .get(&relationship_type)
            .ok_or_else(|| {
                GraphStoreError::RelationshipTypeNotFound(relationship_type.name().to_string())
            })?
            .relationship_count();
        if property.values().element_count() != expected_count {
            return Err(GraphStoreError::InvalidOperation(format!(
                "relationship property '{key}' for type '{relationship_type}' has {} values but the topology requires {expected_count}",
                property.values().element_count()
            )));
        }
        let direction = self
            .schema
            .relationship_schema()
            .get(&relationship_type)
            .ok_or_else(|| {
                GraphStoreError::SchemaError(format!(
                    "relationship type '{relationship_type}' is materialized but absent from the schema"
                ))
            })?
            .direction();
        let mut schema = MutableGraphSchema::from_schema(&self.schema);
        schema.relationship_schema_mut().add_property_schema(
            relationship_type.clone(),
            direction,
            property.property_schema().clone(),
        );
        let replaced = self
            .relationship_property_stores
            .get_mut(&relationship_type)
            .ok_or_else(|| GraphStoreError::PropertyNotFound(key.clone()))?
            .replace_column(property)
            .map_err(|error| GraphStoreError::InvalidOperation(error.to_string()))?;
        self.schema = Arc::new(schema.build());
        self.refresh_relationship_property_state();
        self.set_modified();
        Ok(replaced)
    }

    fn remove_relationship_property(
        &mut self,
        relationship_type: &RelationshipType,
        property_key: &str,
    ) -> GraphStoreResult<()> {
        let remove_empty_store = {
            let store = self
                .relationship_property_stores
                .get_mut(relationship_type)
                .ok_or_else(|| GraphStoreError::PropertyNotFound(property_key.to_string()))?;
            store
                .remove_column(property_key)
                .map_err(|error| GraphStoreError::InvalidOperation(error.to_string()))?;
            store.is_empty()
        };
        if remove_empty_store {
            self.relationship_property_stores.remove(relationship_type);
        }
        let mut schema = MutableGraphSchema::from_schema(&self.schema);
        schema
            .relationship_schema_mut()
            .remove_property(relationship_type, property_key);
        self.schema = Arc::new(schema.build());

        self.refresh_relationship_property_state();
        self.set_modified();
        Ok(())
    }

    fn delete_relationships(
        &mut self,
        relationship_type: &RelationshipType,
    ) -> GraphStoreResult<DeletionResult> {
        if let Some(topology) = self.relationship_topologies.remove(relationship_type) {
            let removed_count = topology.relationship_count();
            self.relationship_property_stores.remove(relationship_type);
            let mut schema = MutableGraphSchema::from_schema(&self.schema);
            schema
                .relationship_schema_mut()
                .remove_type(relationship_type);
            self.schema = Arc::new(schema.build());
            self.rebuild_relationship_metadata();
            self.refresh_relationship_property_state();
            self.set_modified();
            Ok(DeletionResult::with_counts(
                self.graph_name.clone(),
                0,
                removed_count,
            ))
        } else {
            Err(GraphStoreError::RelationshipTypeNotFound(
                relationship_type.name().to_string(),
            ))
        }
    }

    fn get_graph(&self) -> Arc<dyn Graph> {
        let topologies = self
            .relationship_topologies
            .iter()
            .map(|(rel_type, topology)| (rel_type.clone(), Arc::clone(topology)))
            .collect::<HashMap<_, _>>();

        Arc::new(DefaultGraph::new(
            Arc::clone(&self.config),
            Arc::clone(&self.schema),
            Arc::clone(&self.id_map),
            self.graph_characteristics,
            topologies,
            self.ordered_relationship_types.clone(),
            self.inverse_indexed_relationship_types.clone(),
            self.relationship_count,
            self.has_parallel_relationships,
            self.materialized_node_property_values(),
            self.relationship_property_stores.clone(),
            HashMap::new(),
        ))
    }

    fn get_graph_view(&self, spec: &GraphViewSpec) -> GraphViewResult<Arc<dyn Graph>> {
        self.validate_graph_view_spec(spec)?;
        Ok(self
            .graph_with_view(
                spec.relationship_property_selectors().clone(),
                spec.orientation(),
            )
            .filtered_by_relationship_types(spec.relationship_types()))
    }
}

fn orient_topology(
    topology: &RelationshipTopology,
    orientation: Orientation,
) -> (RelationshipTopology, Vec<RelationshipIndex>) {
    let node_count = topology.node_capacity();
    let mut buckets = vec![Vec::<(MappedNodeId, RelationshipIndex)>::new(); node_count];

    for (source_index, targets) in topology.outgoing_lists().iter().enumerate() {
        let source = MappedNodeId::try_from(source_index)
            .expect("validated topology node capacity must fit mapped ID space");
        for (neighbor_offset, &target) in targets.iter().enumerate() {
            let old_index = topology
                .relationship_index(source, neighbor_offset)
                .expect("validated topology row must have a canonical relationship index");
            match orientation {
                Orientation::Natural => buckets[source_index].push((target, old_index)),
                Orientation::Reverse => {
                    let target_index = target
                        .to_usize()
                        .expect("validated mapped target must fit physical index space");
                    buckets
                        .get_mut(target_index)
                        .expect("validated mapped target must belong to topology")
                        .push((source, old_index));
                }
                Orientation::Undirected => {
                    buckets[source_index].push((target, old_index));
                    if target != source {
                        let target_index = target
                            .to_usize()
                            .expect("validated mapped target must fit physical index space");
                        buckets
                            .get_mut(target_index)
                            .expect("validated mapped target must belong to topology")
                            .push((source, old_index));
                    }
                }
            }
        }
    }

    let mut outgoing = Vec::with_capacity(node_count);
    let mut old_indices = Vec::new();
    for bucket in buckets {
        let mut targets = Vec::with_capacity(bucket.len());
        for (target, old_index) in bucket {
            targets.push(target);
            old_indices.push(old_index);
        }
        outgoing.push(targets);
    }

    let incoming = if topology.is_inverse_indexed() {
        Some(build_incoming(&outgoing))
    } else {
        None
    };

    (RelationshipTopology::new(outgoing, incoming), old_indices)
}

fn build_incoming(outgoing: &[Vec<MappedNodeId>]) -> Vec<Vec<MappedNodeId>> {
    let mut incoming = vec![Vec::new(); outgoing.len()];
    for (source_index, targets) in outgoing.iter().enumerate() {
        let source = MappedNodeId::try_from(source_index)
            .expect("validated topology node capacity must fit mapped ID space");
        for &target in targets {
            let target_incoming = target
                .to_usize()
                .and_then(|index| incoming.get_mut(index))
                .expect("validated mapped target must belong to topology");
            target_incoming.push(source);
        }
    }
    incoming
}

fn reindex_relationship_property_store(
    store: &DefaultRelationshipPropertyStore,
    old_indices: Arc<Vec<RelationshipIndex>>,
) -> DefaultRelationshipPropertyStore {
    let mut builder = DefaultRelationshipPropertyStore::builder();
    for property in store.columns() {
        let values: Arc<dyn RelationshipPropertyValues> =
            Arc::new(ReindexedRelationshipPropertyValues::new(
                property.values_arc(),
                Arc::clone(&old_indices),
            ));
        builder = builder.put(RelationshipProperty::with_schema(
            property.property_schema().clone(),
            values,
        ));
    }
    builder.build()
}

fn oriented_schema(schema: &GraphSchema, orientation: Orientation) -> GraphSchema {
    if orientation != Orientation::Undirected {
        return schema.clone();
    }

    let entries = schema
        .relationship_schema()
        .entries()
        .into_iter()
        .map(|entry| {
            (
                entry.identifier().clone(),
                RelationshipSchemaEntry::new(
                    entry.identifier().clone(),
                    Direction::Undirected,
                    entry.properties().clone(),
                ),
            )
        })
        .collect();
    GraphSchema::new(
        schema.node_schema().clone(),
        RelationshipSchema::new(entries),
        schema.graph_properties().clone(),
    )
}

fn build_node_long_property_values(
    backend: LongCollection,
    node_count: usize,
) -> Arc<dyn NodePropertyValues> {
    match backend {
        LongCollection::Vec(collection) => Arc::new(
            DefaultLongNodePropertyValues::<VecLong>::from_collection(collection, node_count),
        ),
        LongCollection::Huge(collection) => {
            let vec_backend = VecLong::from(collection.to_vec());
            Arc::new(DefaultLongNodePropertyValues::<VecLong>::from_collection(
                vec_backend,
                node_count,
            ))
        }
        LongCollection::Arrow(collection) => Arc::new(DefaultLongNodePropertyValues::<
            ArrowLongArray,
        >::from_collection(
            collection, node_count
        )),
    }
}

fn build_node_double_property_values(
    backend: DoubleCollection,
    node_count: usize,
) -> Arc<dyn NodePropertyValues> {
    match backend {
        DoubleCollection::Vec(collection) => Arc::new(
            DefaultDoubleNodePropertyValues::<VecDouble>::from_collection(collection, node_count),
        ),
        DoubleCollection::Huge(collection) => {
            let vec_backend = VecDouble::from(collection.to_vec());
            Arc::new(
                DefaultDoubleNodePropertyValues::<VecDouble>::from_collection(
                    vec_backend,
                    node_count,
                ),
            )
        }
        DoubleCollection::Arrow(collection) => Arc::new(DefaultDoubleNodePropertyValues::<
            ArrowDoubleArray,
        >::from_collection(
            collection, node_count
        )),
    }
}

fn build_node_float_property_values(
    backend: VecFloat,
    node_count: usize,
) -> Arc<dyn NodePropertyValues> {
    Arc::new(DefaultFloatNodePropertyValues::<VecFloat>::from_collection(
        backend, node_count,
    ))
}

fn build_node_int_property_values(
    backend: VecInt,
    node_count: usize,
) -> Arc<dyn NodePropertyValues> {
    Arc::new(DefaultIntNodePropertyValues::<VecInt>::from_collection(
        backend, node_count,
    ))
}

fn build_node_double_array_property_values(
    backend: VecDoubleArray,
    node_count: usize,
) -> Arc<dyn NodePropertyValues> {
    Arc::new(
        DefaultDoubleArrayNodePropertyValues::<VecDoubleArray>::from_collection(
            backend, node_count,
        ),
    )
}

fn build_node_long_array_property_values(
    backend: VecLongArray,
    node_count: usize,
) -> Arc<dyn NodePropertyValues> {
    Arc::new(
        DefaultLongArrayNodePropertyValues::<VecLongArray>::from_collection(backend, node_count),
    )
}

fn build_graph_long_property_values(backend: LongCollection) -> Arc<dyn GraphPropertyValues> {
    match backend {
        LongCollection::Vec(collection) => Arc::new(
            DefaultLongGraphPropertyValues::<VecLong>::from_collection(collection),
        ),
        LongCollection::Huge(collection) => {
            let vec_backend = VecLong::from(collection.to_vec());
            Arc::new(DefaultLongGraphPropertyValues::<VecLong>::from_collection(
                vec_backend,
            ))
        }
        LongCollection::Arrow(collection) => {
            Arc::new(DefaultLongGraphPropertyValues::<ArrowLongArray>::from_collection(collection))
        }
    }
}

fn build_graph_double_property_values(backend: DoubleCollection) -> Arc<dyn GraphPropertyValues> {
    match backend {
        DoubleCollection::Vec(collection) => {
            Arc::new(DefaultDoubleGraphPropertyValues::<VecDouble>::from_collection(collection))
        }
        DoubleCollection::Huge(collection) => {
            let vec_backend = VecDouble::from(collection.to_vec());
            Arc::new(DefaultDoubleGraphPropertyValues::<VecDouble>::from_collection(vec_backend))
        }
        DoubleCollection::Arrow(collection) => Arc::new(DefaultDoubleGraphPropertyValues::<
            ArrowDoubleArray,
        >::from_collection(collection)),
    }
}

fn build_relationship_long_property_values(
    backend: LongCollection,
    relationship_count: usize,
) -> Arc<dyn RelationshipPropertyValues> {
    match backend {
        LongCollection::Vec(collection) => Arc::new(
            DefaultLongRelationshipPropertyValues::<VecLong>::from_collection(
                collection,
                relationship_count,
            ),
        ),
        LongCollection::Huge(collection) => {
            let vec_backend = VecLong::from(collection.to_vec());
            Arc::new(
                DefaultLongRelationshipPropertyValues::<VecLong>::from_collection(
                    vec_backend,
                    relationship_count,
                ),
            )
        }
        LongCollection::Arrow(collection) => Arc::new(DefaultLongRelationshipPropertyValues::<
            ArrowLongArray,
        >::from_collection(
            collection, relationship_count
        )),
    }
}

fn build_relationship_double_property_values(
    backend: DoubleCollection,
    relationship_count: usize,
) -> Arc<dyn RelationshipPropertyValues> {
    match backend {
        DoubleCollection::Vec(collection) => Arc::new(DefaultDoubleRelationshipPropertyValues::<
            VecDouble,
        >::from_collection(
            collection, relationship_count
        )),
        DoubleCollection::Huge(collection) => {
            let vec_backend = VecDouble::from(collection.to_vec());
            Arc::new(
                DefaultDoubleRelationshipPropertyValues::<VecDouble>::from_collection(
                    vec_backend,
                    relationship_count,
                ),
            )
        }
        DoubleCollection::Arrow(collection) => Arc::new(DefaultDoubleRelationshipPropertyValues::<
            ArrowDoubleArray,
        >::from_collection(
            collection, relationship_count
        )),
    }
}

fn build_relationship_int_property_values(
    backend: VecInt,
    relationship_count: usize,
) -> Arc<dyn RelationshipPropertyValues> {
    Arc::new(
        DefaultIntRelationshipPropertyValues::<VecInt>::from_collection(
            backend,
            relationship_count,
        ),
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GraphStoreConfig;
    use crate::types::graph::degrees::Degrees;
    use crate::types::graph::Graph;
    use crate::types::graph_store::validate_graph_store_schema;
    use crate::types::graph_store::{DatabaseId, DatabaseLocation, GraphViewError};
    use crate::types::properties::relationship::DefaultRelationshipPropertyValues;
    use crate::types::properties::relationship::RelationshipIterator;
    use std::sync::Arc;

    fn store_with_config(config: GraphStoreConfig) -> DefaultGraphStore {
        let graph_name = GraphName::new("g");
        let database_info = DatabaseInfo::new(
            DatabaseId::new("db"),
            DatabaseLocation::remote("localhost", 7687, None, None),
        );
        let mut schema = MutableGraphSchema::empty();
        schema
            .relationship_schema_mut()
            .add_relationship_type(RelationshipType::of("KNOWS"), Direction::Directed);
        let capabilities = Capabilities::default();
        let id_map = SimpleIdMap::from_original_ids([0, 1, 2]);

        let topology = RelationshipTopology::new(
            vec![
                vec![MappedNodeId::new(1), MappedNodeId::new(2)],
                vec![MappedNodeId::new(2)],
                vec![],
            ],
            None,
        );

        let mut relationship_topologies = HashMap::new();
        relationship_topologies.insert(RelationshipType::of("KNOWS"), topology);

        DefaultGraphStore::new(
            config,
            graph_name,
            database_info,
            schema.build(),
            capabilities,
            id_map,
            relationship_topologies,
        )
    }

    fn sample_store() -> DefaultGraphStore {
        store_with_config(GraphStoreConfig::default())
    }

    #[test]
    fn graph_view_reflects_store_data() {
        let store = sample_store();
        assert_eq!(store.node_count(), 3);
        assert_eq!(store.relationship_count(), 3);

        let graph = store.graph();
        assert_eq!(graph.relationship_count(), 3);
        assert!(graph.characteristics().is_directed());
        assert_eq!(graph.degree(MappedNodeId::ZERO), 2);
    }

    #[test]
    fn fallible_constructor_rejects_topology_capacity_mismatch() {
        let relationship_type = RelationshipType::of("KNOWS");
        let topology = RelationshipTopology::new(vec![vec![MappedNodeId::new(1)], vec![]], None);
        let error = DefaultGraphStore::try_new(
            GraphStoreConfig::default(),
            GraphName::new("invalid"),
            DatabaseInfo::new(
                DatabaseId::new("db"),
                DatabaseLocation::remote("localhost", 7687, None, None),
            ),
            GraphSchema::empty(),
            Capabilities::default(),
            SimpleIdMap::from_original_ids([0, 1, 2]),
            HashMap::from([(relationship_type, topology)]),
        )
        .expect_err("topology capacity must match the mapped node domain");

        assert!(matches!(error, GraphStoreError::InvalidOperation(message)
            if message.contains("KNOWS") && message.contains("capacity 2") && message.contains("3 nodes")));
    }

    #[test]
    fn adding_relationship_type_rejects_target_outside_mapped_space() {
        let store = sample_store();
        let error = store
            .with_added_relationship_type_preserve_name(
                RelationshipType::of("INVALID"),
                vec![vec![MappedNodeId::new(3)], vec![], vec![]],
                Direction::Directed,
            )
            .expect_err("relationship target must belong to the mapped node domain");

        assert!(matches!(error, GraphStoreError::InvalidOperation(message)
            if message.contains("INVALID") && message.contains("0 -> 3") && message.contains("node count 3")));
    }

    #[test]
    fn inverse_index_preserves_parallel_relationship_identity() {
        let relationship_type = RelationshipType::of("KNOWS");
        let topology = RelationshipTopology::new(
            vec![
                vec![MappedNodeId::new(1), MappedNodeId::new(1)],
                vec![],
                vec![],
            ],
            None,
        );
        let store = sample_store()
            .with_rebuilt_relationship_topologies(
                GraphName::new("parallel"),
                HashMap::from([(relationship_type, topology)]),
            )
            .unwrap()
            .with_inverse_indices(GraphName::new("parallel-inverse"))
            .unwrap();

        let inverse = store
            .graph()
            .stream_inverse_relationships(MappedNodeId::new(1), 0.0)
            .map(|cursor| cursor.relationship_index())
            .collect::<Vec<_>>();

        assert_eq!(
            inverse,
            vec![RelationshipIndex::ZERO, RelationshipIndex::new(1)]
        );
    }

    #[test]
    fn topology_rebuild_rejects_retained_property_cardinality_mismatch() {
        let relationship_type = RelationshipType::of("KNOWS");
        let mut store = sample_store();
        store
            .add_relationship_property(
                relationship_type.clone(),
                "weight",
                Arc::new(DefaultRelationshipPropertyValues::with_default(
                    vec![1.0, 2.0, 3.0],
                    3,
                )),
            )
            .unwrap();

        let rebuilt =
            RelationshipTopology::new(vec![vec![MappedNodeId::new(1)], vec![], vec![]], None);
        let error = store
            .with_rebuilt_relationship_topologies(
                GraphName::new("invalid-rebuild"),
                HashMap::from([(relationship_type.clone(), rebuilt)]),
            )
            .expect_err("mismatched retained relationship property must reject rebuild");

        assert!(matches!(error, GraphStoreError::InvalidOperation(message)
            if message.contains("weight")
                && message.contains("cardinality 3")
                && message.contains("cardinality 1")));
        assert_eq!(store.relationship_count_for_type(&relationship_type), 3);
        assert_eq!(
            store
                .relationship_property_values(&relationship_type, "weight")
                .unwrap()
                .element_count(),
            3
        );
        validate_graph_store_schema(&store).unwrap();
    }

    #[test]
    fn structural_mutations_rebuild_relationship_metadata() {
        let knows = RelationshipType::of("KNOWS");
        let likes = RelationshipType::of("LIKES");
        let store = sample_store()
            .with_added_relationship_type_preserve_name(
                likes.clone(),
                vec![vec![MappedNodeId::new(2)], vec![], vec![]],
                Direction::Directed,
            )
            .unwrap();

        assert_eq!(store.relationship_count(), 4);
        assert!(store.inverse_indexed_relationship_types().is_empty());

        let mut store = store
            .with_inverse_indices(GraphName::new("inverse-indexed"))
            .unwrap();
        assert_eq!(store.relationship_count(), 4);
        assert_eq!(
            store.inverse_indexed_relationship_types(),
            HashSet::from([knows.clone(), likes.clone()])
        );

        store.delete_relationships(&knows).unwrap();
        assert_eq!(store.relationship_count(), 1);
        assert_eq!(
            store.inverse_indexed_relationship_types(),
            HashSet::from([likes])
        );
        validate_graph_store_schema(&store).unwrap();
    }

    #[test]
    fn path_collapse_preserves_unrelated_parallel_edges() {
        let relationship_type = RelationshipType::of("KNOWS");
        let topology = RelationshipTopology::new(
            vec![
                vec![MappedNodeId::new(1)],
                vec![MappedNodeId::new(2)],
                vec![MappedNodeId::new(2), MappedNodeId::new(2)],
            ],
            None,
        );
        let store = sample_store()
            .with_rebuilt_relationship_topologies(
                GraphName::new("collapse-source"),
                HashMap::from([(relationship_type, topology)]),
            )
            .unwrap()
            .collapse_paths_degree2(GraphName::new("collapsed"), None)
            .unwrap();

        let graph = store.graph();
        let collapsed_targets = graph
            .stream_relationships(MappedNodeId::ZERO, 0.0)
            .map(|cursor| cursor.target_id())
            .collect::<Vec<_>>();
        let parallel_targets = graph
            .stream_relationships(MappedNodeId::new(2), 0.0)
            .map(|cursor| cursor.target_id())
            .collect::<Vec<_>>();

        assert_eq!(collapsed_targets, vec![MappedNodeId::new(2)]);
        assert_eq!(parallel_targets, vec![MappedNodeId::new(2); 2]);
        assert_eq!(graph.relationship_count(), 3);
    }

    #[test]
    fn reverse_orientation_preserves_parallel_relationship_properties() {
        let relationship_type = RelationshipType::of("KNOWS");
        let topology = RelationshipTopology::new(
            vec![
                vec![MappedNodeId::new(1), MappedNodeId::new(1)],
                vec![],
                vec![],
            ],
            None,
        );
        let mut store = sample_store()
            .with_rebuilt_relationship_topologies(
                GraphName::new("parallel"),
                HashMap::from([(relationship_type.clone(), topology)]),
            )
            .unwrap();
        store
            .add_relationship_property(
                relationship_type.clone(),
                "weight",
                Arc::new(DefaultRelationshipPropertyValues::with_default(
                    vec![10.0, 20.0],
                    2,
                )),
            )
            .unwrap();

        let graph = store
            .get_graph_with_types_selectors_and_orientation(
                &HashSet::new(),
                &HashMap::from([(relationship_type, "weight".to_string())]),
                Orientation::Reverse,
            )
            .unwrap();
        let relationships = graph
            .stream_relationships(MappedNodeId::new(1), -1.0)
            .map(|cursor| (cursor.relationship_index(), cursor.property()))
            .collect::<Vec<_>>();

        assert_eq!(
            relationships,
            vec![
                (RelationshipIndex::ZERO, 10.0),
                (RelationshipIndex::new(1), 20.0),
            ]
        );
    }

    #[test]
    fn test_add_node_property_with_config() {
        use crate::config::GraphStoreConfig;

        // Create config with specific backend
        let config = GraphStoreConfig::default();

        let mut store = DefaultGraphStore::new(
            config,
            GraphName::new("test"),
            DatabaseInfo::new(
                DatabaseId::new("test"),
                DatabaseLocation::remote("localhost", 7687, None, None),
            ),
            GraphSchema::empty(),
            Capabilities::default(),
            SimpleIdMap::from_original_ids([0, 1, 2]),
            HashMap::new(),
        );

        // Add property - should use Vec backend from config
        store
            .add_node_property_i64("age".to_string(), vec![1, 2, 3])
            .unwrap();

        // Verify property exists
        assert!(store.node_properties.contains_key("age"));
        assert_eq!(store.node_properties.len(), 1);
        validate_graph_store_schema(&store).unwrap();
    }

    #[test]
    fn test_add_graph_property_with_config() {
        use crate::config::GraphStoreConfig;

        let config = GraphStoreConfig::default();

        let mut store = DefaultGraphStore::new(
            config,
            GraphName::new("test"),
            DatabaseInfo::new(
                DatabaseId::new("test"),
                DatabaseLocation::remote("localhost", 7687, None, None),
            ),
            GraphSchema::empty(),
            Capabilities::default(),
            SimpleIdMap::from_original_ids([0, 1, 2]),
            HashMap::new(),
        );

        // Add graph property
        store
            .add_graph_property_f64("density".to_string(), vec![0.5])
            .unwrap();

        // Verify property exists
        assert!(store.graph_properties.contains_key("density"));
        assert_eq!(store.graph_properties.len(), 1);
        validate_graph_store_schema(&store).unwrap();
    }

    #[test]
    fn add_node_label_keeps_schema_and_id_map_synchronized() {
        let mut store = sample_store();
        let label = NodeLabel::of("Person");

        store.add_node_label(label.clone()).unwrap();

        assert!(store.has_node_label(&label));
        assert!(store.schema().node_schema().get(&label).is_some());
        validate_graph_store_schema(&store).unwrap();
    }

    #[test]
    fn replacing_node_property_replaces_its_label_domain() {
        let mut store = sample_store();
        let person = NodeLabel::of("Person");
        let company = NodeLabel::of("Company");
        store.add_node_label(person.clone()).unwrap();
        store.add_node_label(company.clone()).unwrap();

        let initial_values: Arc<dyn NodePropertyValues> = Arc::new(
            DefaultLongNodePropertyValues::from_collection(VecLong::from(vec![1, 2, 3]), 3),
        );
        store
            .add_node_property_column(
                HashSet::from([person.clone()]),
                NodeProperty::with_state("score", PropertyState::Persistent, initial_values),
            )
            .unwrap();

        let replacement_values: Arc<dyn NodePropertyValues> = Arc::new(
            DefaultLongNodePropertyValues::from_collection(VecLong::from(vec![4, 5, 6]), 3),
        );
        store
            .replace_node_property_column(
                HashSet::from([company.clone()]),
                NodeProperty::with_state("score", PropertyState::Persistent, replacement_values),
            )
            .unwrap();

        assert!(!store.has_node_property_for_label(&person, "score"));
        assert!(store.has_node_property_for_label(&company, "score"));
        assert!(store
            .schema()
            .node_schema()
            .get(&person)
            .is_some_and(|entry| !entry.properties().contains_key("score")));
        assert!(store
            .schema()
            .node_schema()
            .get(&company)
            .is_some_and(|entry| entry.properties().contains_key("score")));
        validate_graph_store_schema(&store).unwrap();
    }

    #[test]
    fn delete_relationships_removes_materialization_and_schema() {
        let mut store = sample_store();
        let relationship_type = RelationshipType::of("KNOWS");

        let result = store.delete_relationships(&relationship_type).unwrap();

        assert_eq!(result.deleted_relationship_count(), Some(3));
        assert!(!store.has_relationship_type(&relationship_type));
        assert!(store
            .schema()
            .relationship_schema()
            .get(&relationship_type)
            .is_none());
        validate_graph_store_schema(&store).unwrap();
    }

    #[test]
    fn adds_schema_bearing_graph_and_node_columns() {
        let mut store = sample_store();
        let graph_values: Arc<dyn GraphPropertyValues> =
            Arc::new(DefaultLongGraphPropertyValues::<VecLong>::singleton(3));
        let graph_property = GraphProperty::with_default(
            "iterations",
            PropertyState::Transient,
            graph_values,
            DefaultValue::long(-1),
        );
        store.add_graph_property_column(graph_property).unwrap();

        let node_values: Arc<dyn NodePropertyValues> = Arc::new(
            DefaultLongNodePropertyValues::from_collection(VecLong::from(vec![10, 20, 30]), 3),
        );
        let node_property = NodeProperty::with_default(
            "score",
            PropertyState::Transient,
            node_values,
            DefaultValue::long(-1),
        );
        store
            .add_node_property_column(HashSet::from([NodeLabel::all_nodes()]), node_property)
            .unwrap();

        assert_eq!(
            store
                .graph_properties
                .get("iterations")
                .unwrap()
                .property_schema()
                .state(),
            PropertyState::Transient
        );
        assert_eq!(
            store
                .node_properties
                .get("score")
                .unwrap()
                .property_schema()
                .default_value(),
            &DefaultValue::long(-1)
        );
        validate_graph_store_schema(&store).unwrap();
    }

    #[test]
    fn vec_only_config_keeps_node_properties_vec_backed() {
        let mut store = store_with_config(GraphStoreConfig::vec_only());
        store
            .add_node_property_i64("score".to_string(), vec![10, 20, 30])
            .expect("add node property via vec-only config");

        let values = store
            .node_property_values("score")
            .expect("retrieve node property values");
        assert_eq!(values.long_value(1).unwrap(), 20);
        assert_eq!(values.long_value(2).unwrap(), 30);
    }

    #[test]
    fn manages_relationship_properties() {
        let mut store = sample_store();
        let rel_type = RelationshipType::of("KNOWS");

        let values = Arc::new(DefaultRelationshipPropertyValues::with_default(
            vec![1.0, 2.0, 3.0],
            3,
        ));

        store
            .add_relationship_property(rel_type.clone(), "weight", values)
            .expect("add relationship property");

        assert!(store.has_relationship_property(&rel_type, "weight"));
        assert!(store.relationship_property_keys().contains("weight"));
        let retrieved = store
            .relationship_property_values(&rel_type, "weight")
            .expect("retrieve property");
        assert_eq!(
            retrieved.double_value(RelationshipIndex::new(1)).unwrap(),
            2.0
        );
        assert!(store.graph().has_relationship_property());
        validate_graph_store_schema(&store).unwrap();

        store
            .remove_relationship_property(&rel_type, "weight")
            .expect("remove relationship property");
        assert!(!store.has_relationship_property(&rel_type, "weight"));
        assert!(!store.graph().has_relationship_property());
        validate_graph_store_schema(&store).unwrap();
    }

    #[test]
    fn rejects_misaligned_relationship_column_without_mutation() {
        let mut store = sample_store();
        let schema_before = Arc::clone(&store.schema);
        let modified_before = store.modification_time;
        let values = Arc::new(DefaultRelationshipPropertyValues::with_default(
            vec![1.0, 2.0],
            2,
        ));

        assert!(matches!(
            store.add_relationship_property(RelationshipType::of("KNOWS"), "weight", values),
            Err(GraphStoreError::InvalidOperation(_))
        ));
        assert!(!store.has_relationship_property(&RelationshipType::of("KNOWS"), "weight"));
        assert_eq!(store.schema.as_ref(), schema_before.as_ref());
        assert_eq!(store.modification_time, modified_before);
    }

    #[test]
    fn materialization_preserves_declared_relationship_aggregation() {
        let mut store = sample_store();
        let rel_type = RelationshipType::of("KNOWS");
        let property_schema = RelationshipPropertySchema::with_aggregation(
            "weight",
            ValueType::Double,
            DefaultValue::double(7.5),
            PropertyState::Persistent,
            Aggregation::Max,
        );
        let mut schema = MutableGraphSchema::from_schema(&store.schema);
        schema.relationship_schema_mut().add_property_schema(
            rel_type.clone(),
            Direction::Directed,
            property_schema,
        );
        store.schema = Arc::new(schema.build());

        let values = Arc::new(DefaultRelationshipPropertyValues::with_values(
            vec![1.0, 2.0, 3.0],
            7.5,
            3,
        ));
        store
            .add_relationship_property(rel_type.clone(), "weight", values)
            .unwrap();

        let materialized = store
            .relationship_property_stores
            .get(&rel_type)
            .unwrap()
            .get("weight")
            .unwrap();
        assert_eq!(
            materialized.property_schema().aggregation(),
            Aggregation::Max
        );
        assert_eq!(
            materialized.property_schema().default_value(),
            &DefaultValue::double(7.5)
        );
        validate_graph_store_schema(&store).unwrap();
    }

    #[test]
    fn rejects_values_that_disagree_with_declared_relationship_schema() {
        let mut store = sample_store();
        let rel_type = RelationshipType::of("KNOWS");
        let mut schema = MutableGraphSchema::from_schema(&store.schema);
        schema.relationship_schema_mut().add_property_schema(
            rel_type.clone(),
            Direction::Directed,
            RelationshipPropertySchema::of("weight", ValueType::Long),
        );
        store.schema = Arc::new(schema.build());
        let schema_before = Arc::clone(&store.schema);
        let modified_before = store.modification_time;
        let values = Arc::new(DefaultRelationshipPropertyValues::with_values(
            vec![1.0, 2.0, 3.0],
            0.0,
            3,
        ));

        assert!(matches!(
            store.add_relationship_property(rel_type.clone(), "weight", values),
            Err(GraphStoreError::SchemaError(_))
        ));
        assert!(!store.has_relationship_property(&rel_type, "weight"));
        assert_eq!(store.schema.as_ref(), schema_before.as_ref());
        assert_eq!(store.modification_time, modified_before);
    }

    #[test]
    fn adds_schema_bearing_relationship_column() {
        let mut store = sample_store();
        let rel_type = RelationshipType::of("KNOWS");
        let values = Arc::new(DefaultRelationshipPropertyValues::with_values(
            vec![1.0, 2.0, 3.0],
            7.5,
            3,
        ));
        let property = RelationshipProperty::with_aggregation(
            "weight",
            PropertyState::Transient,
            values,
            DefaultValue::double(7.5),
            Aggregation::Max,
        );

        store
            .add_relationship_property_column(rel_type.clone(), property)
            .unwrap();

        let materialized = store
            .relationship_property_stores
            .get(&rel_type)
            .unwrap()
            .get("weight")
            .unwrap();
        assert_eq!(
            materialized.property_schema().aggregation(),
            Aggregation::Max
        );
        assert_eq!(
            materialized.property_schema().state(),
            PropertyState::Transient
        );
        assert_eq!(
            materialized.property_schema().default_value(),
            &DefaultValue::double(7.5)
        );
        validate_graph_store_schema(&store).unwrap();
    }

    #[test]
    fn explicitly_replaces_schema_bearing_columns() {
        let mut store = sample_store();
        let graph_values: Arc<dyn GraphPropertyValues> =
            Arc::new(DefaultLongGraphPropertyValues::<VecLong>::singleton(1));
        store
            .add_graph_property_column(GraphProperty::of("round", graph_values))
            .unwrap();
        let replacement_values: Arc<dyn GraphPropertyValues> =
            Arc::new(DefaultLongGraphPropertyValues::<VecLong>::singleton(2));
        store
            .replace_graph_property_column(GraphProperty::with_default(
                "round",
                PropertyState::Transient,
                replacement_values,
                DefaultValue::long(-1),
            ))
            .unwrap();

        let rel_type = RelationshipType::of("KNOWS");
        let relationship_values: Arc<dyn RelationshipPropertyValues> = Arc::new(
            DefaultRelationshipPropertyValues::with_values(vec![1.0, 2.0, 3.0], 0.0, 3),
        );
        store
            .add_relationship_property_column(
                rel_type.clone(),
                RelationshipProperty::of("weight", relationship_values),
            )
            .unwrap();
        let replacement_values: Arc<dyn RelationshipPropertyValues> = Arc::new(
            DefaultRelationshipPropertyValues::with_values(vec![3.0, 2.0, 1.0], 9.0, 3),
        );
        store
            .replace_relationship_property_column(
                rel_type.clone(),
                RelationshipProperty::with_aggregation(
                    "weight",
                    PropertyState::Transient,
                    replacement_values,
                    DefaultValue::double(9.0),
                    Aggregation::Min,
                ),
            )
            .unwrap();

        assert_eq!(
            store
                .graph_properties
                .get("round")
                .unwrap()
                .property_schema()
                .state(),
            PropertyState::Transient
        );
        assert_eq!(
            store
                .relationship_property_stores
                .get(&rel_type)
                .unwrap()
                .get("weight")
                .unwrap()
                .property_schema()
                .aggregation(),
            Aggregation::Min
        );
        validate_graph_store_schema(&store).unwrap();
    }

    #[test]
    fn relationship_property_keys_for_types_returns_common_keys() {
        let mut store = sample_store();
        let knows = RelationshipType::of("KNOWS");
        let likes = RelationshipType::of("LIKES");
        store = store
            .with_added_relationship_type_preserve_name(
                likes.clone(),
                vec![vec![MappedNodeId::new(2)], vec![], vec![]],
                Direction::Directed,
            )
            .expect("add LIKES relationship type");

        for (rel_type, property_key) in [
            (knows.clone(), "shared"),
            (knows.clone(), "knows_only"),
            (likes.clone(), "shared"),
            (likes.clone(), "likes_only"),
        ] {
            let relationship_count = store.relationship_count_for_type(&rel_type);
            let values = Arc::new(DefaultRelationshipPropertyValues::with_default(
                vec![1.0; relationship_count],
                relationship_count,
            ));
            store
                .add_relationship_property(rel_type, property_key, values)
                .expect("add relationship property");
        }

        let selected_types = HashSet::from([knows, likes]);
        assert_eq!(
            store.relationship_property_keys_for_types(&selected_types),
            HashSet::from(["shared".to_string()])
        );
        assert_eq!(
            store.relationship_property_keys_for_types(&HashSet::new()),
            store.relationship_property_keys()
        );
    }

    #[test]
    fn relationship_property_type_is_scoped_to_relationship_type() {
        let mut store = sample_store();
        let knows = RelationshipType::of("KNOWS");
        let likes = RelationshipType::of("LIKES");
        store = store
            .with_added_relationship_type_preserve_name(
                likes.clone(),
                vec![vec![MappedNodeId::new(2)], vec![], vec![]],
                Direction::Directed,
            )
            .expect("add LIKES relationship type");

        let knows_count = store.relationship_count_for_type(&knows);
        store
            .add_relationship_property(
                knows.clone(),
                "weight",
                Arc::new(DefaultRelationshipPropertyValues::with_default(
                    vec![1.0; knows_count],
                    knows_count,
                )),
            )
            .expect("add double KNOWS property");

        let likes_count = store.relationship_count_for_type(&likes);
        store
            .add_relationship_property(
                likes.clone(),
                "weight",
                Arc::new(
                    DefaultLongRelationshipPropertyValues::<VecLong>::from_collection(
                        VecLong::from(vec![1; likes_count]),
                        likes_count,
                    ),
                ),
            )
            .expect("add long LIKES property");

        assert_eq!(
            store.relationship_property_type(&knows, "weight").unwrap(),
            ValueType::Double
        );
        assert_eq!(
            store.relationship_property_type(&likes, "weight").unwrap(),
            ValueType::Long
        );
        validate_graph_store_schema(&store).unwrap();
    }

    #[test]
    fn empty_type_filter_preserves_graph_with_property_selectors() {
        let mut store = sample_store();
        let knows = RelationshipType::of("KNOWS");
        let values = Arc::new(DefaultRelationshipPropertyValues::with_default(
            vec![1.0, 2.0, 3.0],
            3,
        ));
        store
            .add_relationship_property(knows.clone(), "weight", values)
            .expect("add relationship property");

        let graph = store
            .get_graph_with_types_and_selectors(
                &HashSet::new(),
                &HashMap::from([(knows, "weight".to_string())]),
            )
            .expect("build selector-aware graph");

        assert_eq!(graph.relationship_count(), store.relationship_count());
        assert_eq!(graph.schema().to_map(), store.schema().to_map());
        assert_eq!(
            graph.relationship_property(MappedNodeId::ZERO, MappedNodeId::new(1), -1.0),
            1.0
        );
    }

    #[test]
    fn reverse_orientation_preserves_relationship_properties() {
        let mut store = sample_store();
        let knows = RelationshipType::of("KNOWS");
        let values = Arc::new(DefaultRelationshipPropertyValues::with_default(
            vec![1.0, 2.0, 3.0],
            3,
        ));
        store
            .add_relationship_property(knows.clone(), "weight", values)
            .expect("add relationship property");

        let graph = store
            .get_graph_with_types_selectors_and_orientation(
                &HashSet::new(),
                &HashMap::from([(knows, "weight".to_string())]),
                Orientation::Reverse,
            )
            .expect("build reverse graph");

        assert!(!graph.exists(MappedNodeId::ZERO, MappedNodeId::new(1)));
        assert!(graph.exists(MappedNodeId::new(1), MappedNodeId::ZERO));
        assert!(graph.exists(MappedNodeId::new(2), MappedNodeId::ZERO));
        assert!(graph.exists(MappedNodeId::new(2), MappedNodeId::new(1)));
        assert_eq!(
            graph.relationship_property(MappedNodeId::new(1), MappedNodeId::ZERO, -1.0),
            1.0
        );
        assert_eq!(
            graph.relationship_property(MappedNodeId::new(2), MappedNodeId::ZERO, -1.0),
            2.0
        );
        assert_eq!(
            graph.relationship_property(MappedNodeId::new(2), MappedNodeId::new(1), -1.0,),
            3.0
        );
    }

    #[test]
    fn graph_view_spec_is_the_canonical_view_request() {
        let store = sample_store();
        let graph = store
            .get_graph_view(
                &GraphViewSpec::new()
                    .with_relationship_types(HashSet::from([RelationshipType::of("KNOWS")]))
                    .with_orientation(Orientation::Reverse),
            )
            .expect("build graph from canonical view spec");

        assert!(!graph.exists(MappedNodeId::ZERO, MappedNodeId::new(1)));
        assert!(graph.exists(MappedNodeId::new(1), MappedNodeId::ZERO));
        assert_eq!(graph.relationship_count(), store.relationship_count());
    }

    #[test]
    fn graph_view_rejects_unmaterialized_relationship_type() {
        let store = sample_store();
        let error = store
            .get_graph_view(
                &GraphViewSpec::new()
                    .with_relationship_types(HashSet::from([RelationshipType::of("MISSING")])),
            )
            .expect_err("unmaterialized relationship type must be rejected");

        assert_eq!(
            error,
            GraphViewError::RelationshipTypeNotMaterialized("MISSING".to_string())
        );
    }

    #[test]
    fn graph_view_rejects_selector_for_unselected_type() {
        let store = sample_store();
        let error = store
            .get_graph_view(
                &GraphViewSpec::new()
                    .with_relationship_types(HashSet::from([RelationshipType::of("KNOWS")]))
                    .with_relationship_property_selectors(HashMap::from([(
                        RelationshipType::of("OTHER"),
                        "weight".to_string(),
                    )])),
            )
            .expect_err("selector for unselected type must be rejected");

        assert_eq!(
            error,
            GraphViewError::SelectorForUnselectedType("OTHER".to_string())
        );
    }

    #[test]
    fn graph_view_rejects_unmaterialized_relationship_property() {
        let store = sample_store();
        let error = store
            .get_graph_view(&GraphViewSpec::new().with_relationship_property_selectors(
                HashMap::from([(RelationshipType::of("KNOWS"), "weight".to_string())]),
            ))
            .expect_err("unmaterialized relationship property must be rejected");

        assert_eq!(
            error,
            GraphViewError::RelationshipPropertyNotMaterialized {
                relationship_type: "KNOWS".to_string(),
                property_key: "weight".to_string(),
            }
        );
    }

    #[test]
    fn undirected_orientation_preserves_relationship_properties() {
        let mut store = sample_store();
        let knows = RelationshipType::of("KNOWS");
        let values = Arc::new(DefaultRelationshipPropertyValues::with_default(
            vec![1.0, 2.0, 3.0],
            3,
        ));
        store
            .add_relationship_property(knows.clone(), "weight", values)
            .expect("add relationship property");

        let graph = store
            .get_graph_with_types_selectors_and_orientation(
                &HashSet::new(),
                &HashMap::from([(knows, "weight".to_string())]),
                Orientation::Undirected,
            )
            .expect("build undirected graph");

        assert!(graph.characteristics().is_undirected());
        assert!(graph.schema().is_undirected());
        assert_eq!(graph.relationship_count(), 6);
        assert_eq!(
            graph.relationship_property(MappedNodeId::ZERO, MappedNodeId::new(1), -1.0),
            1.0
        );
        assert_eq!(
            graph.relationship_property(MappedNodeId::new(1), MappedNodeId::ZERO, -1.0),
            1.0
        );
        assert_eq!(
            graph.relationship_property(MappedNodeId::new(2), MappedNodeId::ZERO, -1.0),
            2.0
        );
        assert_eq!(
            graph.relationship_property(MappedNodeId::new(2), MappedNodeId::new(1), -1.0,),
            3.0
        );
    }

    #[test]
    fn builds_node_values_from_arrow_backend() {
        let backend = LongCollection::Arrow(ArrowLongArray::from_vec(vec![5, 10]));
        let values = build_node_long_property_values(backend, 2);
        assert_eq!(values.long_value(1).unwrap(), 10);
        assert_eq!(values.node_count(), 2);
    }

    #[test]
    fn builds_graph_values_from_arrow_backend() {
        let backend = DoubleCollection::Arrow(ArrowDoubleArray::from_vec(vec![0.25, 0.75]));
        let values = build_graph_double_property_values(backend);
        let collected: Vec<f64> = values.double_values().collect();
        assert_eq!(collected, vec![0.25, 0.75]);
    }
}
