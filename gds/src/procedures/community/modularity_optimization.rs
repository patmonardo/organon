//! Modularity Optimization Facade
//!
//! Produces community assignments by locally optimizing modularity.

use crate::algo::algorithms::Result;
use crate::algo::algorithms::{ConfigValidator, WriteResult};
use crate::algo::modularity_optimization::{
    modularity_optimization_progress_task, ModularityOptimizationComputationRuntime,
    ModularityOptimizationConfig, ModularityOptimizationMutateResult,
    ModularityOptimizationMutationSummary, ModularityOptimizationResult,
    ModularityOptimizationResultBuilder, ModularityOptimizationStats,
    ModularityOptimizationStorageRuntime,
};
use crate::collections::backends::vec::VecLong;
use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::TaskRegistry;
use crate::mem::{Estimate, MemoryRange};
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use crate::types::properties::node::DefaultLongNodePropertyValues;
use crate::types::properties::node::NodePropertyValues;
use crate::types::schema::NodeLabel;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct ModularityOptimizationRow {
    pub node_id: u64,
    pub community_id: u64,
}

#[derive(Clone)]
pub struct ModularityOptimizationFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: ModularityOptimizationConfig,
    task_registry: Option<TaskRegistry>,
}

impl ModularityOptimizationFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: ModularityOptimizationConfig::default(),
            task_registry: None,
        }
    }

    pub fn from_spec_config(
        graph_store: Arc<DefaultGraphStore>,
        config: ModularityOptimizationConfig,
    ) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;

        Ok(Self {
            graph_store,
            config,
            task_registry: None,
        })
    }

    pub fn from_spec_json(
        graph_store: Arc<DefaultGraphStore>,
        raw_config: &serde_json::Value,
    ) -> Result<Self> {
        let parsed: ModularityOptimizationConfig = serde_json::from_value(raw_config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        Self::from_spec_config(graph_store, parsed)
    }

    pub fn with_spec_config(mut self, config: ModularityOptimizationConfig) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;
        self.config = config;
        Ok(self)
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.config.concurrency = concurrency;
        self
    }

    pub fn max_iterations(mut self, max_iterations: usize) -> Self {
        self.config.max_iterations = max_iterations;
        self
    }

    pub fn tolerance(mut self, tolerance: f64) -> Self {
        self.config.tolerance = tolerance;
        self
    }

    pub fn gamma(mut self, gamma: f64) -> Self {
        self.config.gamma = gamma;
        self
    }

    pub fn min_batch_size(mut self, min_batch_size: usize) -> Self {
        self.config.min_batch_size = min_batch_size;
        self
    }

    pub fn relationship_weight_property(mut self, property_name: &str) -> Self {
        self.config.relationship_weight_property = Some(property_name.to_string());
        self
    }

    pub fn task_registry(mut self, task_registry: TaskRegistry) -> Self {
        self.task_registry = Some(task_registry);
        self
    }

    fn validate(&self) -> Result<()> {
        self.config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))
    }

    fn compute(&self) -> Result<ModularityOptimizationResult> {
        self.validate()?;
        let start = Instant::now();

        let storage =
            ModularityOptimizationStorageRuntime::new(self.graph_store.as_ref(), &self.config)?;
        let mut computation = ModularityOptimizationComputationRuntime::new();
        let termination_flag = TerminationFlag::default();

        let base_task = modularity_optimization_progress_task(
            storage.node_count(),
            storage.relationship_count(),
            self.config.max_iterations,
        );
        let mut progress_tracker = super::progress_tracker_for_task(
            base_task,
            self.config.concurrency,
            self.task_registry.as_ref(),
        );

        let result = storage.compute_modularity_optimization(
            &mut computation,
            &self.config,
            &mut progress_tracker,
            &termination_flag,
        )?;

        Ok(ModularityOptimizationResult {
            node_count: storage.node_count(),
            execution_time: start.elapsed(),
            ..result
        })
    }

    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = ModularityOptimizationRow>>> {
        let result = self.compute()?;
        let iter = result
            .communities
            .into_iter()
            .enumerate()
            .map(|(node_id, community_id)| ModularityOptimizationRow {
                node_id: node_id as u64,
                community_id,
            });
        Ok(Box::new(iter))
    }

    pub fn stats(&self) -> Result<ModularityOptimizationStats> {
        let result = self.compute()?;
        Ok(ModularityOptimizationResultBuilder::new(result).stats())
    }

    pub fn run(&self) -> Result<ModularityOptimizationResult> {
        self.compute()
    }

    pub fn mutate(self, property_name: &str) -> Result<ModularityOptimizationMutateResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        let result = self.compute()?;
        let node_count = self.graph_store.node_count();
        let nodes_updated = node_count as u64;

        let longs: Vec<i64> = result.communities.into_iter().map(|c| c as i64).collect();
        let backend = VecLong::from(longs);
        let values = DefaultLongNodePropertyValues::from_collection(backend, node_count);
        let values: Arc<dyn NodePropertyValues> = Arc::new(values);

        let mut new_store = self.graph_store.as_ref().clone();
        let labels: HashSet<NodeLabel> = new_store.node_labels();
        new_store
            .add_node_property(labels, property_name.to_string(), values)
            .map_err(|e| {
                AlgorithmError::Execution(format!(
                    "ModularityOptimization mutate failed to add property: {e}"
                ))
            })?;

        let summary = ModularityOptimizationMutationSummary {
            nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: result.execution_time.as_millis() as u64,
        };

        Ok(ModularityOptimizationMutateResult {
            summary,
            updated_store: Arc::new(new_store),
        })
    }

    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        let res = self.mutate(property_name)?;
        Ok(WriteResult::new(
            res.summary.nodes_updated,
            property_name.to_string(),
            std::time::Duration::from_millis(res.summary.execution_time_ms),
        ))
    }

    pub fn estimate_memory(&self) -> Result<MemoryRange> {
        let node_count = self.graph_store.node_count();
        let relationship_count = self.graph_store.relationship_count();
        let concurrency = self.config.concurrency.max(1);

        let current_communities = Estimate::size_of_long_array(node_count);
        let next_communities = Estimate::size_of_long_array(node_count);
        let cumulative_node_weights = Estimate::size_of_double_array(node_count);
        let community_weights = Estimate::size_of_double_array(node_count);
        let colors_used = Estimate::size_of_bitset(node_count);
        let colors = Estimate::size_of_long_array(node_count);
        let community_weight_updates = Estimate::size_of_double_array(node_count);
        let adjacency = relationship_count
            .saturating_mul(std::mem::size_of::<(usize, f64)>())
            .saturating_add(node_count.saturating_mul(Estimate::BYTES_OBJECT_HEADER));
        let min_influences = Estimate::size_of_long_double_hash_map(50).saturating_mul(concurrency);
        let max_influences =
            Estimate::size_of_long_double_hash_map(node_count.max(50)).saturating_mul(concurrency);

        let fixed = Estimate::BYTES_OBJECT_HEADER
            .saturating_add(current_communities)
            .saturating_add(next_communities)
            .saturating_add(cumulative_node_weights)
            .saturating_add(community_weights)
            .saturating_add(colors_used)
            .saturating_add(colors)
            .saturating_add(community_weight_updates)
            .saturating_add(adjacency);

        Ok(MemoryRange::of_range(
            fixed.saturating_add(min_influences),
            fixed.saturating_add(max_influences),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GraphStoreConfig;
    use crate::projection::RelationshipType;
    use crate::types::graph::{RelationshipTopology, SimpleIdMap};
    use crate::types::graph_store::{
        Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, GraphName,
    };
    use crate::types::schema::{Direction, MutableGraphSchema};
    use std::collections::HashMap;

    fn store_from_edges(node_count: usize, edges: &[(usize, usize)]) -> DefaultGraphStore {
        let mut outgoing: Vec<Vec<i64>> = vec![Vec::new(); node_count];
        let mut incoming: Vec<Vec<i64>> = vec![Vec::new(); node_count];

        for &(source, target) in edges {
            outgoing[source].push(target as i64);
            incoming[target].push(source as i64);
        }

        let rel_type = RelationshipType::of("REL");
        let mut schema_builder = MutableGraphSchema::empty();
        schema_builder
            .relationship_schema_mut()
            .add_relationship_type(rel_type.clone(), Direction::Directed);
        let schema = schema_builder.build();

        let mut relationship_topologies = HashMap::new();
        relationship_topologies.insert(
            rel_type,
            RelationshipTopology::new(outgoing, Some(incoming)),
        );

        let original_ids: Vec<i64> = (0..node_count as i64).collect();
        let id_map = SimpleIdMap::from_original_ids(original_ids);

        DefaultGraphStore::new(
            GraphStoreConfig::default(),
            GraphName::new("g"),
            DatabaseInfo::new(
                DatabaseId::new("db"),
                DatabaseLocation::remote("localhost", 7687, None, None),
            ),
            schema,
            Capabilities::default(),
            id_map,
            relationship_topologies,
        )
    }

    #[test]
    fn builder_api() {
        assert_eq!(
            std::mem::size_of::<ModularityOptimizationFacade>(),
            std::mem::size_of::<ModularityOptimizationFacade>()
        );
    }

    #[test]
    fn facade_modes_on_small_graph() {
        let store = Arc::new(store_from_edges(3, &[(0, 1), (1, 0)]));
        let facade = ModularityOptimizationFacade::new(Arc::clone(&store))
            .concurrency(1)
            .max_iterations(5);

        let rows: Vec<_> = facade.stream().expect("stream").collect();
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].community_id, rows[1].community_id);
        assert_ne!(rows[0].community_id, rows[2].community_id);

        let stats = facade.stats().expect("stats");
        assert_eq!(stats.community_count, 2);
        assert!(stats.ran_iterations <= 5);

        let memory = facade.estimate_memory().expect("estimate memory");
        assert!(memory.max() >= memory.min());

        let mutated = facade.mutate("community").expect("mutate");
        assert_eq!(mutated.summary.nodes_updated, 3);
        let values = mutated
            .updated_store
            .node_property_values("community")
            .expect("community property");
        assert_eq!(values.long_value(0).unwrap(), values.long_value(1).unwrap());
        assert_ne!(values.long_value(0).unwrap(), values.long_value(2).unwrap());
    }
}
