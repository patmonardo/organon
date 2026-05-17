//! Betweenness Centrality Facade
//!
//! **What is it?**: Fraction of shortest paths that pass through each node
//! **Why care?**: Identifies "bridge" nodes that connect different network regions
//! **Complexity**: O(V*(V+E)) using Brandes' algorithm - more expensive!
//! **Best for**: Finding bottlenecks and critical connectors in networks
//!
//! ## What Betweenness Means
//!
//! For each node N:
//! - For every pair of other nodes (S, T), find shortest path from S to T
//! - Count how many of those shortest paths pass through N
//! - Betweenness = (# paths through N) / (# shortest paths total)
//!
//! High betweenness = critical for network flow/communication
//!
//! ## Example
//!
//! ```rust,no_run
//! # use gds::Graph;
//! # let graph = Graph::default();
//! let results = graph
//!     .betweenness()
//!     .stream()?
//!     .collect::<Vec<_>>();
//!
//! let stats = graph.betweenness().stats()?;
//! println!("Max betweenness: {} (bottleneck identified)", stats.max);
//! ```

use crate::algo::algorithms::{CentralityScore, Result};
use crate::algo::algorithms::{ConfigValidator, WriteResult};
use crate::algo::betweenness::storage::BetweennessCentralityStorageRuntime;
use crate::algo::betweenness::{
    betweenness_progress_task, parse_betweenness_orientation,
    BetweennessCentralityComputationRuntime, BetweennessCentralityConfig,
    BetweennessCentralityMutateResult, BetweennessCentralityMutationSummary,
    BetweennessCentralityResult, BetweennessCentralityResultBuilder, BetweennessCentralityStats,
};
use crate::collections::backends::vec::VecDouble;
use crate::concurrency::{Concurrency, TerminationFlag};
use crate::config::config_trait::ValidatedConfig;
use crate::core::utils::progress::ProgressTracker;
use crate::core::utils::progress::{
    EmptyTaskRegistryFactory, JobId, TaskProgressTracker, TaskRegistryFactory,
};
use crate::mem::MemoryRange;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::NodeLabel;
use crate::projection::Orientation;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use crate::types::properties::node::DefaultDoubleNodePropertyValues;
use crate::types::properties::node::NodePropertyValues;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

/// Betweenness centrality facade/builder bound to a live graph store.
#[derive(Clone)]
pub struct BetweennessCentralityFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: BetweennessCentralityConfig,
    task_registry: Arc<dyn TaskRegistryFactory>,
}

impl BetweennessCentralityFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: BetweennessCentralityConfig::default(),
            task_registry: Arc::new(EmptyTaskRegistryFactory),
        }
    }

    /// Create a facade using the spec.rs config model.
    pub fn from_spec_config(
        graph_store: Arc<DefaultGraphStore>,
        config: BetweennessCentralityConfig,
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
        let parsed: BetweennessCentralityConfig = serde_json::from_value(raw_config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        Self::from_spec_config(graph_store, parsed)
    }

    /// Apply a spec.rs config onto an existing facade.
    pub fn with_spec_config(mut self, config: BetweennessCentralityConfig) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;
        self.config = config;
        Ok(self)
    }

    /// Direction of traversal: "outgoing", "incoming", or "both".
    pub fn direction(mut self, direction: &str) -> Self {
        self.config.direction = direction.to_string();
        self
    }

    /// Set concurrency level for parallel computation.
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.config.concurrency = concurrency;
        self
    }

    /// Optional relationship weight property.
    ///
    /// When set, betweenness runs the weighted Brandes variant (Dijkstra forward phase).
    pub fn relationship_weight_property(mut self, property: Option<String>) -> Self {
        self.config.relationship_weight_property = property.filter(|p| !p.trim().is_empty());
        self
    }

    /// Sampling strategy for selecting source nodes.
    ///
    /// Supported values:
    /// - "all" (default): use all nodes as sources
    /// - "random_degree": sample sources weighted by node degree
    pub fn sampling_strategy(mut self, strategy: &str) -> Self {
        self.config.sampling_strategy = strategy.to_string();
        self
    }

    /// Optional sampling size (number of source nodes to process).
    ///
    /// If not set, all nodes are used.
    pub fn sampling_size(mut self, size: Option<usize>) -> Self {
        self.config.sampling_size = size;
        self
    }

    /// Seed for sampling RNG.
    pub fn random_seed(mut self, seed: u64) -> Self {
        self.config.random_seed = seed;
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
        parse_betweenness_orientation(&self.config.direction)
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

    fn compute(&self) -> Result<BetweennessCentralityResult> {
        self.validate()?;

        let start = Instant::now();
        let orientation = self.orientation()?;

        let storage = BetweennessCentralityStorageRuntime::new(
            &*self.graph_store,
            orientation,
            self.config.relationship_weight_property.as_deref(),
        )?;
        let node_count = storage.node_count();
        if node_count == 0 {
            return Ok(BetweennessCentralityResult {
                centralities: Vec::new(),
                node_count,
                execution_time: start.elapsed(),
            });
        }

        let mut computation = BetweennessCentralityComputationRuntime::new(node_count);

        let sources: Vec<usize> = {
            let requested = self
                .config
                .sampling_size
                .unwrap_or(node_count)
                .min(node_count);
            storage.select_sources(
                &self.config.sampling_strategy,
                Some(requested),
                self.config.random_seed,
            )?
        };

        let mut progress_tracker = TaskProgressTracker::with_registry(
            betweenness_progress_task(sources.len()).base().clone(),
            Concurrency::of(self.config.concurrency.max(1)),
            JobId::new(),
            self.task_registry.as_ref(),
        );
        progress_tracker.begin_subtask_with_volume(sources.len());

        let divisor = if orientation == Orientation::Undirected {
            2.0
        } else {
            1.0
        };
        let concurrency = Concurrency::of(self.config.concurrency.max(1));

        let progress_handle = progress_tracker.clone();
        let on_source_done = Arc::new(move || {
            let mut tracker = progress_handle.clone();
            tracker.log_progress(1);
        });

        let termination = TerminationFlag::running_true();

        // Call storage.compute_betweenness - Applications talk only to procedures
        let result = storage
            .compute_betweenness(
                &mut computation,
                &sources,
                divisor,
                concurrency,
                &termination,
                on_source_done,
            )
            .map_err(|e| AlgorithmError::Execution(format!("Betweenness terminated: {e}")))?;

        progress_tracker.end_subtask();

        Ok(BetweennessCentralityResult {
            centralities: result.centralities,
            node_count,
            execution_time: start.elapsed(),
        })
    }

    /// Stream mode: Get betweenness score for each node
    ///
    /// Returns an iterator over (node_id, score) tuples.
    ///
    /// **Warning**: This algorithm is O(V*E), so streaming on large graphs
    /// may take a while. Consider computing stats instead for overview.
    ///
    /// ## Example
    /// ```rust,no_run
    /// # use std::sync::Arc;
    /// # use gds::types::prelude::DefaultGraphStore;
    /// # let graph = Arc::new(DefaultGraphStore::empty());
    /// # use gds::procedures::centrality::BetweennessCentralityFacade;
    /// let facade = BetweennessCentralityFacade::new(graph);
    /// for score in facade.stream()? {
    ///     if score.score > 0.1 {
    ///         println!("Bridge node: {} (betweenness: {:.4})", score.node_id, score.score);
    ///     }
    /// }
    /// ```
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

    /// Stats mode: Get aggregated statistics
    ///
    /// This is the recommended way to analyze betweenness on large graphs.
    /// Returns min, max, mean, stddev, percentiles, and identifies "bridge" nodes.
    ///
    /// ## Example
    /// ```rust,no_run
    /// # use std::sync::Arc;
    /// # use gds::types::prelude::DefaultGraphStore;
    /// # let graph = Arc::new(DefaultGraphStore::empty());
    /// # use gds::procedures::centrality::BetweennessCentralityFacade;
    /// let facade = BetweennessCentralityFacade::new(graph);
    /// let stats = facade.stats()?;
    /// println!("Found {} bridge nodes", stats.bridge_nodes);
    /// println!("Execution took {}ms", stats.execution_time_ms);
    /// ```
    pub fn stats(&self) -> Result<BetweennessCentralityStats> {
        self.validate()?;
        let result = self.compute()?;
        Ok(BetweennessCentralityResultBuilder::new(result).stats())
    }

    /// Mutate mode: Compute and store as node property
    ///
    /// Stores betweenness scores as a node property.
    /// Useful for follow-up analysis like identifying connectors.
    ///
    /// ## Example
    /// ```rust,no_run
    /// # use std::sync::Arc;
    /// # use gds::types::prelude::DefaultGraphStore;
    /// # let graph = Arc::new(DefaultGraphStore::empty());
    /// # use gds::procedures::centrality::BetweennessCentralityFacade;
    /// let facade = BetweennessCentralityFacade::new(graph);
    /// let result = facade.mutate("betweenness")?;
    /// println!("Computed and stored for {} nodes", result.nodes_updated);
    /// ```
    pub fn mutate(self, property_name: &str) -> Result<BetweennessCentralityMutateResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;
        let result = self.compute()?;
        let scores = result.centralities.clone();
        let nodes_updated = scores.len() as u64;
        let builder = BetweennessCentralityResultBuilder::new(result);

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
                AlgorithmError::Execution(format!("Betweenness mutate failed to add property: {e}"))
            })?;

        let summary = BetweennessCentralityMutationSummary {
            nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: builder.execution_time_ms(),
        };

        Ok(BetweennessCentralityMutateResult {
            summary,
            updated_store: Arc::new(new_store),
        })
    }

    /// Write mode is not implemented yet for betweenness.
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

    /// Estimate memory requirements for betweenness centrality computation.
    ///
    /// # Returns
    /// Memory range estimate (min/max bytes)
    ///
    /// # Example
    /// ```ignore
    /// # use std::sync::Arc;
    /// # use gds::types::prelude::DefaultGraphStore;
    /// # let graph = Arc::new(DefaultGraphStore::empty());
    /// # use gds::procedures::centrality::BetweennessCentralityFacade;
    /// let facade = BetweennessCentralityFacade::new(graph);
    /// let memory = facade.estimate_memory();
    /// println!("Will use between {} and {} bytes", memory.min(), memory.max());
    /// ```
    pub fn estimate_memory(&self) -> MemoryRange {
        let node_count = self.graph_store.node_count();
        let relationship_count = self.graph_store.relationship_count();
        let workers = self.config.concurrency.max(1);
        let bitset_bytes = (node_count + 7) / 8;

        let centrality_scores = node_count.saturating_mul(std::mem::size_of::<f64>());

        let average_degree = if node_count == 0 {
            0
        } else {
            relationship_count.div_ceil(node_count).max(1)
        };
        let predecessors = node_count
            .saturating_mul(average_degree)
            .saturating_mul(std::mem::size_of::<usize>());
        let backward_nodes = node_count.saturating_mul(std::mem::size_of::<usize>());
        let deltas = node_count.saturating_mul(std::mem::size_of::<f64>());
        let sigmas = node_count.saturating_mul(std::mem::size_of::<u64>());

        let forward_traverser = if self.config.relationship_weight_property.is_some() {
            node_count
                .saturating_mul(2)
                .saturating_mul(std::mem::size_of::<usize>())
                .saturating_add(bitset_bytes)
        } else {
            node_count
                .saturating_mul(std::mem::size_of::<i32>())
                .saturating_add(node_count.saturating_mul(std::mem::size_of::<usize>()))
        };

        let per_worker = predecessors
            .saturating_add(backward_nodes)
            .saturating_add(deltas)
            .saturating_add(sigmas)
            .saturating_add(forward_traverser);

        let min = centrality_scores.saturating_add(workers.saturating_mul(per_worker));

        let overhead = min
            .saturating_div(5)
            .saturating_add(relationship_count.saturating_mul(std::mem::size_of::<usize>()))
            .saturating_add(1024 * 1024);
        let max = min.saturating_add(overhead);

        MemoryRange::of_range(min, max)
    }
}

// Sampling logic no longer needs a standalone item wrapper.

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::types::random::{RandomGraphConfig, RandomRelationshipConfig};

    fn store() -> Arc<DefaultGraphStore> {
        let config = RandomGraphConfig {
            seed: Some(19),
            node_count: 8,
            relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
            ..RandomGraphConfig::default()
        };
        Arc::new(DefaultGraphStore::random(&config).unwrap())
    }

    #[test]
    fn test_stream_returns_node_count_rows() {
        let facade = BetweennessCentralityFacade::new(store());
        let rows: Vec<_> = facade.stream().unwrap().collect();
        assert_eq!(rows.len(), 8);
    }

    #[test]
    fn test_stats_shape() {
        let facade = BetweennessCentralityFacade::new(store());
        let stats = facade.stats().unwrap();
        assert!(stats.max >= stats.min);
    }

    #[test]
    fn test_mutate_validates_property_name() {
        let facade = BetweennessCentralityFacade::new(store());
        assert!(facade.clone().mutate("").is_err());
        let result = facade.mutate("betweenness");
        assert!(result.is_ok());
        let mutation_result = result.unwrap();
        assert_eq!(mutation_result.summary.property_name, "betweenness");
        assert!(mutation_result
            .updated_store
            .has_node_property("betweenness"));
    }

    #[test]
    fn test_invalid_direction_fails_fast() {
        let facade = BetweennessCentralityFacade::new(store()).direction("sideways");
        assert!(facade.stream().is_err());
    }

    #[test]
    fn test_invalid_sampling_strategy_fails_fast() {
        let facade = BetweennessCentralityFacade::new(store()).sampling_strategy("largest_first");
        assert!(facade.stream().is_err());
    }

    #[test]
    fn test_memory_estimate_has_range() {
        let facade = BetweennessCentralityFacade::new(store()).concurrency(2);
        let memory = facade.estimate_memory();
        assert!(memory.max() >= memory.min());
    }
}
