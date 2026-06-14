//! CELF Facade
//!
//! Live wiring for Cost-Effective Lazy Forward influence maximization.

use crate::algo::algorithms::{AlgorithmRunner, Result};
use crate::algo::algorithms::{ConfigValidator, WriteResult};
use crate::algo::celf::storage::CELFStorageRuntime;
use crate::algo::celf::{
    celf_progress_task, CELFComputationRuntime, CELFConfig, CELFMutateResult, CELFMutationSummary,
    CELFResult, CELFResultBuilder, CELFRow, CELFStats,
};
use crate::collections::backends::vec::VecDouble;
use crate::task::concurrency::{Concurrency, TerminationFlag};
use crate::core::utils::progress::{
    EmptyTaskRegistryFactory, JobId, ProgressTracker, TaskProgressTracker, TaskRegistryFactory,
};
use crate::task::memory::MemoryRange;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::graph_store::GraphStore;
use crate::types::prelude::DefaultGraphStore;
use crate::types::properties::node::DefaultDoubleNodePropertyValues;
use crate::types::properties::node::NodePropertyValues;
use crate::types::schema::NodeLabel;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Instant;

/// CELF facade bound to a live graph store
#[derive(Clone)]
pub struct CELFFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: CELFConfig,
    task_registry: Arc<dyn TaskRegistryFactory>,
}

impl CELFFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: CELFConfig::default(),
            task_registry: Arc::new(EmptyTaskRegistryFactory),
        }
    }

    /// Create a facade using the spec.rs config model.
    pub fn from_spec_config(
        graph_store: Arc<DefaultGraphStore>,
        config: CELFConfig,
    ) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;

        Ok(Self {
            graph_store,
            config,
            task_registry: Arc::new(EmptyTaskRegistryFactory),
        })
    }

    /// Parse JSON into spec.rs config and return a configured facade.
    pub fn from_spec_json(
        graph_store: Arc<DefaultGraphStore>,
        raw_config: &serde_json::Value,
    ) -> Result<Self> {
        let parsed: CELFConfig = serde_json::from_value(raw_config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        Self::from_spec_config(graph_store, parsed)
    }

    /// Apply a spec.rs config onto an existing facade.
    pub fn with_spec_config(mut self, config: CELFConfig) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;
        self.config = config;
        Ok(self)
    }

    pub fn with_config(mut self, config: CELFConfig) -> Self {
        self.config = config;
        self
    }

    pub fn seed_set_size(mut self, size: usize) -> Self {
        self.config.seed_set_size = size;
        self
    }

    pub fn monte_carlo_simulations(mut self, simulations: usize) -> Self {
        self.config.monte_carlo_simulations = simulations;
        self
    }

    pub fn propagation_probability(mut self, prob: f64) -> Self {
        self.config.propagation_probability = prob;
        self
    }

    pub fn batch_size(mut self, size: usize) -> Self {
        self.config.batch_size = size;
        self
    }

    pub fn random_seed(mut self, seed: u64) -> Self {
        self.config.random_seed = seed;
        self
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.config.concurrency = concurrency;
        self
    }

    pub fn task_registry(mut self, task_registry: Arc<dyn TaskRegistryFactory>) -> Self {
        self.task_registry = task_registry;
        self
    }

    /// Compatibility alias for older builder call sites; prefer `task_registry`.
    pub fn task_registry_factory(mut self, factory: Box<dyn TaskRegistryFactory>) -> Self {
        self.task_registry = factory.into();
        self
    }

    /// Kept for compatibility with older pathfinding-style builders.
    pub fn user_log_registry_factory(self, _factory: Box<dyn TaskRegistryFactory>) -> Self {
        self
    }

    pub fn validate(&self) -> Result<()> {
        self.config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))
    }

    fn compute_seed_set(&self) -> Result<(CELFResult, std::time::Duration)> {
        self.validate()?;
        let start = Instant::now();

        let storage = CELFStorageRuntime::new(&*self.graph_store)?;
        let node_count = storage.node_count();
        if node_count == 0 {
            return Ok((
                CELFResult {
                    seed_set_nodes: HashMap::new(),
                    node_count,
                },
                start.elapsed(),
            ));
        }

        let seed_set_size = self.config.seed_set_size.min(node_count);
        let concurrency = Concurrency::of(self.config.concurrency.max(1));
        let mut progress_tracker = TaskProgressTracker::with_registry(
            celf_progress_task(node_count, seed_set_size).base().clone(),
            concurrency,
            JobId::new(),
            self.task_registry.as_ref(),
        );
        progress_tracker
            .begin_subtask_with_volume(node_count.saturating_add(seed_set_size.saturating_sub(1)));

        let runtime = CELFComputationRuntime::new(self.config.clone(), node_count);
        let termination = TerminationFlag::running_true();
        let greedy_progress = progress_tracker.clone();
        let on_greedy_node_done = move || {
            let mut tracker = greedy_progress.clone();
            tracker.log_progress(1);
        };
        let seed_progress = progress_tracker.clone();
        let on_seed_selected = move || {
            let mut tracker = seed_progress.clone();
            tracker.log_progress(1);
        };

        let seed_set = storage
            .compute_celf_with_progress(
                &runtime,
                &termination,
                on_greedy_node_done,
                on_seed_selected,
            )
            .map_err(|e| AlgorithmError::Execution(format!("CELF terminated: {e}")))?;

        progress_tracker.end_subtask();

        Ok((
            CELFResult {
                seed_set_nodes: seed_set,
                node_count,
            },
            start.elapsed(),
        ))
    }

    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = CELFRow>>> {
        let (result, elapsed) = self.compute_seed_set()?;
        let rows = CELFResultBuilder::new(result, elapsed).rows();
        Ok(Box::new(rows.into_iter()))
    }

    pub fn stats(&self) -> Result<CELFStats> {
        let (result, elapsed) = self.compute_seed_set()?;
        Ok(CELFResultBuilder::new(result, elapsed).stats())
    }

    pub fn mutate(self, property_name: &str) -> Result<CELFMutateResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;
        let (result, elapsed) = self.compute_seed_set()?;
        let seed_set = result.seed_set_nodes.clone();
        let builder = CELFResultBuilder::new(result, elapsed);

        let node_count = self.graph_store.node_count();
        let nodes_updated = node_count as u64;

        let mut scores = vec![0.0; node_count];
        for (node_id, spread) in seed_set {
            let idx = node_id as usize;
            if idx < scores.len() {
                scores[idx] = spread;
            }
        }

        let backend = VecDouble::from(scores);
        let values = DefaultDoubleNodePropertyValues::from_collection(backend, node_count);
        let values: Arc<dyn NodePropertyValues> = Arc::new(values);

        let mut new_store = self.graph_store.as_ref().clone();
        let labels: HashSet<NodeLabel> = new_store.node_labels();
        new_store
            .add_node_property(labels, property_name.to_string(), values)
            .map_err(|e| {
                AlgorithmError::Execution(format!("CELF mutate failed to add property: {e}"))
            })?;

        let summary = CELFMutationSummary {
            nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: builder.execution_time_ms(),
        };

        Ok(CELFMutateResult {
            summary,
            updated_store: Arc::new(new_store),
        })
    }

    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;
        let (result, execution_time) = self.compute_seed_set()?;
        let nodes_written = result.seed_set_nodes.len() as u64;
        Ok(WriteResult::new(
            nodes_written,
            property_name.to_string(),
            execution_time,
        ))
    }

    pub fn estimate_memory(&self) -> MemoryRange {
        let node_count = self.graph_store.node_count();
        let seed_set_size = self.config.seed_set_size.min(node_count).max(1);
        let batch_size = self.config.batch_size.max(1);
        let concurrency = self.config.concurrency.max(1);
        let bitset_bytes = (node_count + 7) / 8;
        let stack_bytes = node_count.saturating_mul(std::mem::size_of::<usize>());

        let seed_set = seed_set_size.saturating_mul(16);
        let first_k = batch_size.saturating_mul(std::mem::size_of::<usize>());
        let spread_priority_queue = node_count.saturating_mul(
            std::mem::size_of::<usize>()
                + std::mem::size_of::<usize>()
                + std::mem::size_of::<f64>(),
        );
        let single_spread_array = node_count.saturating_mul(std::mem::size_of::<f64>());

        let ic_init_per_thread = bitset_bytes.saturating_add(stack_bytes);
        let lazy_spread = batch_size.saturating_mul(std::mem::size_of::<f64>());
        let lazy_forward = bitset_bytes
            .saturating_mul(2)
            .saturating_add(batch_size.saturating_mul(std::mem::size_of::<f64>()))
            .saturating_add(batch_size.saturating_mul(std::mem::size_of::<usize>()))
            .saturating_add(seed_set_size.saturating_mul(std::mem::size_of::<usize>()))
            .saturating_add(stack_bytes);

        let total = seed_set
            .saturating_add(first_k)
            .saturating_add(spread_priority_queue)
            .saturating_add(single_spread_array)
            .saturating_add(concurrency.saturating_mul(ic_init_per_thread))
            .saturating_add(lazy_spread)
            .saturating_add(lazy_forward)
            .saturating_add(1024 * 1024);

        MemoryRange::of_range(total, total.saturating_add(total / 5))
    }
}

impl AlgorithmRunner for CELFFacade {
    fn algorithm_name(&self) -> &'static str {
        "celf"
    }

    fn description(&self) -> &'static str {
        "Cost-Effective Lazy Forward influence maximization (Independent Cascade)"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GraphStoreConfig;
    use crate::procedures::GraphFacade;
    use crate::projection::RelationshipType;
    use crate::types::graph::{RelationshipTopology, SimpleIdMap};
    use crate::types::graph_store::{
        Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName,
    };
    use crate::types::properties::PropertyValues;
    use crate::types::schema::{Direction, MutableGraphSchema};
    use std::collections::HashMap;

    fn store_from_directed_edges(node_count: usize, edges: &[(usize, usize)]) -> DefaultGraphStore {
        let mut outgoing: Vec<Vec<i64>> = vec![Vec::new(); node_count];
        let mut incoming: Vec<Vec<i64>> = vec![Vec::new(); node_count];

        for &(src, tgt) in edges {
            outgoing[src].push(tgt as i64);
            incoming[tgt].push(src as i64);
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
    fn facade_finds_seed_nodes_on_star() {
        // 0 -> 1, 0 -> 2, 0 -> 3
        // Node 0 is the hub with maximum influence
        let store = store_from_directed_edges(4, &[(0, 1), (0, 2), (0, 3)]);
        let graph = GraphFacade::new(Arc::new(store));

        let rows: Vec<_> = graph
            .celf()
            .seed_set_size(1)
            .monte_carlo_simulations(10)
            .propagation_probability(1.0)
            .stream()
            .unwrap()
            .collect();

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].node_id, 0);
    }

    #[test]
    fn facade_respects_seed_set_size() {
        let store = store_from_directed_edges(5, &[(0, 1), (1, 2), (2, 3), (3, 4)]);
        let graph = GraphFacade::new(Arc::new(store));

        let rows: Vec<_> = graph
            .celf()
            .seed_set_size(3)
            .monte_carlo_simulations(10)
            .propagation_probability(0.5)
            .stream()
            .unwrap()
            .collect();

        assert_eq!(rows.len(), 3);
    }

    #[test]
    fn mutate_adds_seed_spread_property() {
        let store = store_from_directed_edges(4, &[(0, 1), (0, 2), (0, 3)]);
        let graph = GraphFacade::new(Arc::new(store));

        let result = graph
            .celf()
            .seed_set_size(1)
            .monte_carlo_simulations(10)
            .propagation_probability(1.0)
            .random_seed(42)
            .mutate("celf_spread")
            .unwrap();

        let values = result
            .updated_store
            .node_property_values("celf_spread")
            .unwrap();

        assert_eq!(values.element_count(), 4);
        assert!(values.double_value(0).unwrap() > 0.0);
        assert_eq!(values.double_value(1).unwrap(), 0.0);
        assert_eq!(values.double_value(2).unwrap(), 0.0);
        assert_eq!(values.double_value(3).unwrap(), 0.0);
    }

    #[test]
    fn invalid_config_fails_fast() {
        let store = store_from_directed_edges(4, &[(0, 1), (0, 2), (0, 3)]);
        let graph = GraphFacade::new(Arc::new(store));

        assert!(graph.celf().seed_set_size(0).stream().is_err());
        assert!(graph.celf().propagation_probability(1.5).stream().is_err());
        assert!(graph.celf().concurrency(0).stream().is_err());
    }

    #[test]
    fn mutate_validates_property_name() {
        let store = store_from_directed_edges(4, &[(0, 1), (0, 2), (0, 3)]);
        let graph = GraphFacade::new(Arc::new(store));

        assert!(graph.celf().mutate("").is_err());
    }

    #[test]
    fn stats_include_node_count_and_seed_count() {
        let store = store_from_directed_edges(4, &[(0, 1), (0, 2), (0, 3)]);
        let graph = GraphFacade::new(Arc::new(store));

        let stats = graph
            .celf()
            .seed_set_size(1)
            .monte_carlo_simulations(10)
            .propagation_probability(1.0)
            .stats()
            .unwrap();

        assert_eq!(stats.node_count, 4);
        assert_eq!(stats.seed_count, 1);
        assert!(stats.total_spread > 0.0);
    }

    #[test]
    fn facade_uses_natural_outgoing_propagation() {
        let store = store_from_directed_edges(4, &[(1, 0), (2, 0), (3, 0)]);
        let graph = GraphFacade::new(Arc::new(store));

        let rows: Vec<_> = graph
            .celf()
            .seed_set_size(1)
            .monte_carlo_simulations(10)
            .propagation_probability(1.0)
            .random_seed(42)
            .stream()
            .unwrap()
            .collect();

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].node_id, 1);
        assert!(rows[0].spread > 1.0);
    }

    #[test]
    fn config_accepts_java_style_aliases() {
        let config: CELFConfig = serde_json::from_value(serde_json::json!({
            "seedSetSize": 2,
            "monteCarloSimulations": 5,
            "propagationProbability": 0.25,
            "batchSize": 3,
            "randomSeed": 9,
            "concurrency": 1
        }))
        .unwrap();

        assert_eq!(config.seed_set_size, 2);
        assert_eq!(config.monte_carlo_simulations, 5);
        assert_eq!(config.propagation_probability, 0.25);
        assert_eq!(config.batch_size, 3);
        assert_eq!(config.random_seed, 9);
        assert_eq!(config.concurrency, 1);
    }

    #[test]
    fn partial_config_uses_defaults() {
        let config: CELFConfig = serde_json::from_value(serde_json::json!({
            "seedSetSize": 2
        }))
        .unwrap();
        let defaults = CELFConfig::default();

        assert_eq!(config.seed_set_size, 2);
        assert_eq!(
            config.monte_carlo_simulations,
            defaults.monte_carlo_simulations
        );
        assert_eq!(
            config.propagation_probability,
            defaults.propagation_probability
        );
        assert_eq!(config.batch_size, defaults.batch_size);
        assert_eq!(config.random_seed, defaults.random_seed);
        assert_eq!(config.concurrency, defaults.concurrency);
    }

    #[test]
    fn memory_estimate_has_range() {
        let store = store_from_directed_edges(4, &[(0, 1), (0, 2), (0, 3)]);
        let graph = GraphFacade::new(Arc::new(store));
        let memory = graph.celf().estimate_memory();

        assert!(memory.max() >= memory.min());
    }
}
