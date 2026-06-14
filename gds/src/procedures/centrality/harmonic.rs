//! Harmonic Centrality Facade
//!
//! **What is it?**: A closeness variant that sums reciprocal distances.
//! **Why care?**: Highlights nodes that are, on average, close to many others,
//! including in disconnected graphs (unreachable pairs contribute 0).
//! **Complexity**: O(V*(V+E)) in the worst case (all-pairs BFS).
//!
//! This implementation follows the Neo4j GDS behavior:
//! - Uses MSBFS-style aggregated neighbor processing
//! - Accumulates into the *reached node* per depth
//! - Normalizes by `(nodeCount - 1)`

use crate::algo::algorithms::{CentralityScore, Result};
use crate::algo::algorithms::{ConfigValidator, WriteResult};
use crate::algo::harmonic::{
    harmonic_orientation, harmonic_progress_task, parse_harmonic_direction,
    HarmonicCentralityMutateResult, HarmonicCentralityMutationSummary, HarmonicCentralityStats,
    HarmonicComputationRuntime, HarmonicConfig, HarmonicDirection, HarmonicResult,
    HarmonicResultBuilder, HarmonicStorageRuntime,
};
use crate::collections::backends::vec::VecDouble;
use crate::task::concurrency::{Concurrency, TerminationFlag};
use crate::task::progress::ProgressTracker;
use crate::task::progress::{
    EmptyTaskRegistryFactory, JobId, TaskProgressTracker, TaskRegistryFactory,
};
use crate::task::memory::MemoryRange;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::NodeLabel;
use crate::projection::Orientation;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use crate::types::properties::node::DefaultDoubleNodePropertyValues;
use crate::types::properties::node::NodePropertyValues;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

/// Harmonic centrality facade/builder bound to a live graph store.
#[derive(Clone)]
pub struct HarmonicCentralityFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: HarmonicConfig,
    task_registry: Arc<dyn TaskRegistryFactory>,
}

impl HarmonicCentralityFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: HarmonicConfig::default(),
            task_registry: Arc::new(EmptyTaskRegistryFactory),
        }
    }

    /// Create a facade using the spec.rs config model.
    pub fn from_spec_config(
        graph_store: Arc<DefaultGraphStore>,
        config: HarmonicConfig,
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
        let parsed: HarmonicConfig = serde_json::from_value(raw_config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        Self::from_spec_config(graph_store, parsed)
    }

    /// Apply a spec.rs config onto an existing facade.
    pub fn with_spec_config(mut self, config: HarmonicConfig) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;
        self.config = config;
        Ok(self)
    }

    /// Direction of traversal: "outgoing", "incoming", or "both".
    pub fn direction(mut self, direction: &str) -> Self {
        self.config.direction =
            parse_harmonic_direction(direction).unwrap_or(HarmonicDirection::Invalid);
        self
    }

    /// Set concurrency level for parallel computation.
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.config.concurrency = concurrency;
        self
    }

    /// Set the task registry factory for progress tracking and concurrency control.
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

    fn orientation(&self) -> Result<Orientation> {
        harmonic_orientation(self.config.direction)
    }

    /// Validate the facade configuration.
    ///
    /// # Returns
    /// Ok(()) if configuration is valid, Err otherwise
    ///
    /// # Errors
    /// Returns an error if concurrency is not positive
    pub fn validate(&self) -> Result<()> {
        self.config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))
    }

    fn compute(&self) -> Result<HarmonicResult> {
        self.validate()?;

        let start = Instant::now();

        let storage = HarmonicStorageRuntime::with_orientation(
            self.graph_store.as_ref(),
            self.orientation()?,
        )?;

        let node_count = storage.node_count();
        if node_count == 0 {
            return Ok(HarmonicResult {
                centralities: Vec::new(),
                node_count,
                execution_time: start.elapsed(),
            });
        }

        let concurrency = Concurrency::of(self.config.concurrency.max(1));

        let mut progress_tracker = TaskProgressTracker::with_registry(
            harmonic_progress_task(node_count).base().clone(),
            concurrency,
            JobId::new(),
            self.task_registry.as_ref(),
        );
        progress_tracker.begin_subtask_with_volume(node_count);

        let computation = HarmonicComputationRuntime::new(node_count);
        let termination = TerminationFlag::running_true();

        let progress_handle = progress_tracker.clone();
        let on_sources_done = Arc::new(move |n: usize| {
            let mut tracker = progress_handle.clone();
            tracker.log_progress(n);
        });

        let scores = match storage.compute_parallel(
            &computation,
            concurrency,
            &termination,
            on_sources_done,
        ) {
            Ok(scores) => scores,
            Err(e) => {
                progress_tracker.end_subtask_with_failure();
                return Err(AlgorithmError::Execution(format!(
                    "Harmonic terminated: {e}"
                )));
            }
        };

        progress_tracker.end_subtask();

        Ok(HarmonicResult {
            centralities: scores,
            node_count,
            execution_time: start.elapsed(),
        })
    }

    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = CentralityScore>>> {
        self.validate()?;
        let result = self.compute()?;
        let iter = result
            .centralities
            .into_iter()
            .enumerate()
            .map(|(node_id, score)| CentralityScore {
                node_id: node_id as u64,
                score,
            });
        Ok(Box::new(iter))
    }

    pub fn stats(&self) -> Result<HarmonicCentralityStats> {
        self.validate()?;
        let result = self.compute()?;
        Ok(HarmonicResultBuilder::new(result).stats())
    }

    /// Mutate mode: compute scores, write them to a new graph store, and return it.
    pub fn mutate(self, property_name: &str) -> Result<HarmonicCentralityMutateResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;
        let result = self.compute()?;
        let scores = result.centralities.clone();
        let nodes_updated = scores.len() as u64;
        let builder = HarmonicResultBuilder::new(result);

        // Build property values
        let node_count = scores.len();
        let backend = VecDouble::from(scores);
        let values = DefaultDoubleNodePropertyValues::from_collection(backend, node_count);
        let values: Arc<dyn NodePropertyValues> = Arc::new(values);

        // Clone store, add property, and return updated store
        let mut new_store = self.graph_store.as_ref().clone();
        let labels: HashSet<NodeLabel> = new_store.node_labels();
        new_store
            .add_node_property(labels, property_name.to_string(), values)
            .map_err(|e| {
                AlgorithmError::Execution(format!("Harmonic mutate failed to add property: {e}"))
            })?;

        let summary = HarmonicCentralityMutationSummary {
            nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: builder.execution_time_ms(),
        };

        Ok(HarmonicCentralityMutateResult {
            summary,
            updated_store: Arc::new(new_store),
        })
    }

    /// Write mode is not implemented yet for harmonic.
    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;
        let result = self.compute()?;
        let nodes_written = result.centralities.len() as u64;
        let execution_time = result.execution_time;

        Ok(WriteResult::new(
            nodes_written,
            property_name.to_string(),
            execution_time,
        ))
    }

    /// Estimate memory requirements for harmonic centrality computation.
    ///
    /// # Returns
    /// Memory range estimate (min/max bytes)
    ///
    /// # Example
    /// ```ignore
    /// # use std::sync::Arc;
    /// # use gds::types::prelude::DefaultGraphStore;
    /// # let graph = Arc::new(DefaultGraphStore::empty());
    /// # use gds::procedures::centrality::HarmonicCentralityFacade;
    /// let facade = HarmonicCentralityFacade::new(graph);
    /// let memory = facade.estimate_memory();
    /// println!("Will use between {} and {} bytes", memory.min(), memory.max());
    /// ```
    pub fn estimate_memory(&self) -> MemoryRange {
        let node_count = self.graph_store.node_count();
        let concurrency = self.config.concurrency.max(1);

        let inverse_farness_memory = node_count.saturating_mul(std::mem::size_of::<f64>());
        let scores_memory = node_count.saturating_mul(std::mem::size_of::<f64>());
        let msbfs_worker_memory = node_count
            .saturating_mul(std::mem::size_of::<u64>())
            .saturating_mul(concurrency);
        let executor_overhead = concurrency.saturating_mul(64 * 1024);

        let total_memory = inverse_farness_memory
            .saturating_add(scores_memory)
            .saturating_add(msbfs_worker_memory)
            .saturating_add(executor_overhead);
        let total_with_overhead = total_memory.saturating_add(total_memory / 5);

        MemoryRange::of_range(total_memory, total_with_overhead)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::random::{RandomGraphConfig, RandomRelationshipConfig};

    fn store() -> Arc<DefaultGraphStore> {
        let config = RandomGraphConfig {
            seed: Some(11),
            node_count: 8,
            relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
            ..RandomGraphConfig::default()
        };
        Arc::new(DefaultGraphStore::random(&config).unwrap())
    }

    #[test]
    fn test_stream_returns_node_count_rows() {
        let facade = HarmonicCentralityFacade::new(store());
        let rows: Vec<_> = facade.stream().unwrap().collect();
        assert_eq!(rows.len(), 8);
    }

    #[test]
    fn test_stats_shape() {
        let facade = HarmonicCentralityFacade::new(store());
        let stats = facade.stats().unwrap();
        assert!(stats.max >= stats.min);
        assert_eq!(stats.node_count, 8);
    }

    #[test]
    fn test_mutate_validates_property_name() {
        let facade = HarmonicCentralityFacade::new(store());
        assert!(facade.clone().mutate("").is_err());
        let result = facade.mutate("harmonic");
        assert!(result.is_ok());
        let mutation_result = result.unwrap();
        assert_eq!(mutation_result.summary.property_name, "harmonic");
        assert!(mutation_result.updated_store.has_node_property("harmonic"));
    }

    #[test]
    fn test_invalid_direction_fails_fast() {
        let facade = HarmonicCentralityFacade::new(store()).direction("sideways");
        assert!(facade.stream().is_err());
    }

    #[test]
    fn test_memory_estimate_has_range() {
        let facade = HarmonicCentralityFacade::new(store()).concurrency(2);
        let memory = facade.estimate_memory();
        assert!(memory.max() >= memory.min());
    }
}
