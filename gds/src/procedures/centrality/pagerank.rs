//! PageRank Facade
//!
//! **What is it?**: Iterative algorithm computing node importance via link structure
//! **Why care?**: Models "random surfer" - nodes that many nodes link to are important
//! **Complexity**: O(k*(V + E)) where k is iterations
//! **Best for**: Finding important/authoritative nodes in networks
//!
//! ## The Random Surfer Model
//!
//! PageRank imagines a random person surfing the web:
//! - With probability `damping_factor`, they follow a random outgoing link
//! - With probability `1 - damping_factor`, they jump to a random page
//!
//! Nodes with more incoming links (and links from important nodes) get higher scores.
//!
//! ## Example
//!
//! ```rust,no_run
//! # use gds::Graph;
//! # let graph = Graph::default();
//! let results = graph
//!     .pagerank()
//!     .iterations(20)
//!     .damping_factor(0.85)
//!     .tolerance(1e-4)
//!     .stream()?
//!     .collect::<Vec<_>>();
//! ```

use crate::algo::algorithms::{CentralityScore, Result};
use crate::algo::algorithms::{ConfigValidator, WriteResult};
use crate::algo::pagerank::{
    computation::{PageRankComputationRuntime, PageRankMemoryEstimation},
    pagerank_progress_task, parse_pagerank_orientation,
    storage::PageRankStorageRuntime,
    PageRankConfig, PageRankMutateResult, PageRankMutationSummary, PageRankResult,
    PageRankResultBuilder, PageRankStats, PageRankVariant,
};
use crate::collections::backends::vec::VecDouble;
use crate::task::concurrency::Concurrency;
use crate::core::graph_dimensions::ConcreteGraphDimensions;
use crate::core::utils::progress::ProgressTracker;
use crate::core::utils::progress::{
    EmptyTaskRegistryFactory, JobId, TaskProgressTracker, TaskRegistryFactory,
};
use crate::task::memory::{MemoryEstimation, MemoryRange};
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::Orientation;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use crate::types::properties::node::DefaultDoubleNodePropertyValues;
use crate::types::properties::node::NodePropertyValues;
use crate::types::schema::NodeLabel;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

// ============================================================================
// Builder Type
// ============================================================================

/// PageRank algorithm facade - fluent configuration
///
/// Use this to configure and run PageRank with custom parameters.
/// Supports multiple execution modes via method chaining.
///
/// ## Default Configuration
/// - iterations: 20
/// - damping_factor: 0.85 (traditional value from Google)
/// - tolerance: 1e-4
///
/// ## Example
/// ```rust,no_run
/// # use std::sync::Arc;
/// # use gds::types::prelude::DefaultGraphStore;
/// # let graph = Arc::new(DefaultGraphStore::empty());
/// # use gds::procedures::centrality::PageRankFacade;
/// let facade = PageRankFacade::new(graph)
///     .iterations(30)
///     .damping_factor(0.85)
///     .tolerance(1e-5);
/// ```
#[derive(Clone)]
pub struct PageRankFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: PageRankConfig,
    /// Task registry for progress tracking
    task_registry: Arc<dyn TaskRegistryFactory>,
}

impl PageRankFacade {
    /// Create a new PageRank facade bound to a live graph store.
    ///
    /// Defaults:
    /// - iterations: 20
    /// - damping_factor: 0.85
    /// - tolerance: 1e-4
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: PageRankConfig::default(),
            task_registry: Arc::new(EmptyTaskRegistryFactory),
        }
    }

    /// Create a facade using the spec.rs config model.
    pub fn from_spec_config(
        graph_store: Arc<DefaultGraphStore>,
        config: PageRankConfig,
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
        let parsed: PageRankConfig = serde_json::from_value(raw_config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        Self::from_spec_config(graph_store, parsed)
    }

    /// Apply a spec.rs config onto an existing facade.
    pub fn with_spec_config(mut self, config: PageRankConfig) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;
        self.config = config;
        Ok(self)
    }

    /// Set Pregel concurrency (Rayon worker threads).
    ///
    /// Use `1` for deterministic single-threaded debugging.
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.config.concurrency = concurrency;
        self
    }

    /// Set task registry factory for progress tracking
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

    /// Direction of traversal: "outgoing", "incoming", or "both".
    ///
    /// PageRank typically uses outgoing (natural) relationships.
    pub fn direction(mut self, direction: &str) -> Self {
        self.config.direction = direction.to_string();
        self
    }

    /// Personalize PageRank by only seeding `source_nodes` with $\alpha$.
    /// When set, all non-source nodes start at 0.
    pub fn source_nodes(mut self, source_nodes: Vec<u64>) -> Self {
        self.config.source_nodes = Some(source_nodes);
        self
    }

    /// Select the PageRank-family variant.
    pub fn variant(mut self, variant: PageRankVariant) -> Self {
        self.config.variant = variant;
        self
    }

    /// Use standard PageRank semantics.
    pub fn page_rank(self) -> Self {
        self.variant(PageRankVariant::PageRank)
    }

    /// Use ArticleRank's degree-smoothed propagation denominator.
    pub fn article_rank(self) -> Self {
        self.variant(PageRankVariant::ArticleRank)
    }

    /// Use the Eigenvector centrality variant with L2-normalized power iteration.
    pub fn eigenvector(self) -> Self {
        self.variant(PageRankVariant::Eigenvector)
    }

    /// Select a relationship weight property for weighted PageRank-family runs.
    pub fn relationship_weight_property(mut self, property_name: impl Into<String>) -> Self {
        self.config.relationship_weight_property = Some(property_name.into());
        self
    }

    /// Set maximum iterations
    ///
    /// The algorithm will stop after this many iterations or when converged,
    /// whichever comes first.
    ///
    /// Higher values = more accurate but slower.
    /// Typical: 10-50 iterations
    pub fn iterations(mut self, n: u32) -> Self {
        self.config.max_iterations = n as usize;
        self
    }

    /// Set damping factor (probability of following a link)
    ///
    /// Range: (0.0, 1.0)
    ///
    /// - 0.85 (default): Traditional Google PageRank value
    /// - Higher (0.95): Edges matter more, random nodes less
    /// - Lower (0.5): Random teleportation matters more
    pub fn damping_factor(mut self, d: f64) -> Self {
        self.config.damping_factor = d;
        self
    }

    /// Set convergence tolerance
    ///
    /// The algorithm converges when max delta between iterations < tolerance.
    ///
    /// - 1e-4 (default): Good balance
    /// - 1e-6: Very tight, slower
    /// - 1e-3: Loose, faster
    pub fn tolerance(mut self, t: f64) -> Self {
        self.config.tolerance = t;
        self
    }

    /// Validate configuration before execution
    pub fn validate(&self) -> Result<()> {
        self.config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))
    }

    fn orientation(&self) -> Result<Orientation> {
        parse_pagerank_orientation(&self.config.direction)
    }

    fn compute(&self) -> Result<PageRankResult> {
        self.validate()?;
        let start = Instant::now();

        let storage = PageRankStorageRuntime::new(
            self.graph_store.as_ref(),
            self.orientation()?,
            self.config.relationship_weight_property.as_deref(),
        )?;

        let source_set = self
            .config
            .source_nodes
            .clone()
            .map(|v| v.into_iter().collect::<std::collections::HashSet<u64>>());

        let computation = PageRankComputationRuntime::new(
            self.config.max_iterations,
            self.config.damping_factor,
            self.config.tolerance,
            source_set,
            self.config.variant,
        );
        let concurrency = Concurrency::of(self.config.concurrency.max(1));

        let mut progress_tracker = TaskProgressTracker::with_registry(
            pagerank_progress_task(self.config.max_iterations)
                .base()
                .clone(),
            concurrency,
            JobId::new(),
            self.task_registry.as_ref(),
        );
        progress_tracker.begin_subtask_with_volume(self.config.max_iterations);

        let run = storage.run(&computation, concurrency, &mut progress_tracker);

        progress_tracker.end_subtask();

        Ok(PageRankResult {
            scores: run.scores,
            ran_iterations: run.ran_iterations,
            did_converge: run.did_converge,
            node_count: self.graph_store.node_count(),
            execution_time: start.elapsed(),
        })
    }

    /// Stream mode: Get PageRank score for each node
    ///
    /// Returns an iterator over (node_id, score) tuples.
    ///
    /// Use this when you want individual results, e.g.:
    /// ```rust,no_run
    /// # use std::sync::Arc;
    /// # use gds::types::prelude::DefaultGraphStore;
    /// # let graph = Arc::new(DefaultGraphStore::empty());
    /// # use gds::procedures::centrality::PageRankFacade;
    /// let builder = PageRankFacade::new(graph);
    /// for score in builder.stream()? {
    ///     println!("Node {} has score {}", score.node_id, score.score);
    /// }
    /// ```
    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = CentralityScore>>> {
        let result = self.compute()?;
        let iter = result
            .scores
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
    /// Returns min, max, mean, stddev, percentiles, and convergence info.
    ///
    /// Use this when you want overview statistics:
    /// ```rust,no_run
    /// # use std::sync::Arc;
    /// # use gds::types::prelude::DefaultGraphStore;
    /// # let graph = Arc::new(DefaultGraphStore::empty());
    /// # use gds::procedures::centrality::PageRankFacade;
    /// let builder = PageRankFacade::new(graph);
    /// let stats = builder.stats()?;
    /// println!("Converged: {}, Iterations: {}", stats.converged, stats.iterations_ran);
    /// ```
    pub fn stats(&self) -> Result<PageRankStats> {
        let result = self.compute()?;
        Ok(PageRankResultBuilder::new(result).stats())
    }

    /// Mutate mode: Compute and store as node property
    ///
    /// Stores PageRank scores as a node property for use by other algorithms.
    ///
    /// ```rust,no_run
    /// # use std::sync::Arc;
    /// # use gds::types::prelude::DefaultGraphStore;
    /// # let graph = Arc::new(DefaultGraphStore::empty());
    /// # use gds::procedures::centrality::PageRankFacade;
    /// let facade = PageRankFacade::new(graph).damping_factor(0.85);
    /// let result = facade.mutate("pagerank")?;
    /// println!("Updated {} nodes", result.summary.nodes_updated);
    /// ```
    pub fn mutate(self, property_name: &str) -> Result<PageRankMutateResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        let result = self.compute()?;
        let scores = result.scores.clone();

        let nodes_updated = scores.len() as u64;
        let node_count = scores.len();
        let backend = VecDouble::from(scores);
        let values = DefaultDoubleNodePropertyValues::from_collection(backend, node_count);
        let values: Arc<dyn NodePropertyValues> = Arc::new(values);

        let mut new_store = self.graph_store.as_ref().clone();
        let labels: HashSet<NodeLabel> = new_store.node_labels();
        new_store
            .add_node_property(labels, property_name.to_string(), values)
            .map_err(|e| {
                AlgorithmError::Execution(format!("PageRank mutate failed to add property: {e}"))
            })?;

        let summary = PageRankMutationSummary {
            nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: result.execution_time.as_millis() as u64,
        };

        Ok(PageRankMutateResult {
            summary,
            updated_store: Arc::new(new_store),
        })
    }

    /// Write mode: Compute and write results to external storage
    ///
    /// Writes the PageRank scores to an external data store.
    /// This is useful for persisting results for later analysis.
    ///
    /// # Arguments
    /// * `property_name` - Name of the property to store the centrality scores
    ///
    /// # Returns
    /// Result containing write statistics
    ///
    /// # Example
    /// ```ignore
    /// # use std::sync::Arc;
    /// # use gds::types::prelude::DefaultGraphStore;
    /// # let graph = Arc::new(DefaultGraphStore::empty());
    /// # use gds::procedures::centrality::PageRankFacade;
    /// let facade = PageRankFacade::new(graph);
    /// let result = facade.write("pagerank")?;
    /// println!("Wrote {} records", result.records_written);
    /// ```
    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        let result = self.compute()?;
        let scores = result.scores;

        // For now, use placeholder write - just count the nodes that would be written
        // TODO: Implement actual persistence to external storage
        let nodes_written = scores.len() as u64;

        Ok(WriteResult::new(
            nodes_written,
            property_name.to_string(),
            result.execution_time,
        ))
    }

    /// Estimate memory usage for this algorithm execution
    ///
    /// Provides an estimate of the memory required to run this algorithm
    /// with the current configuration. This is useful for capacity planning
    /// and preventing out-of-memory errors.
    ///
    /// # Returns
    /// Memory range estimate (min/max bytes)
    ///
    /// # Example
    /// ```ignore
    /// # use std::sync::Arc;
    /// # use gds::types::prelude::DefaultGraphStore;
    /// # let graph = Arc::new(DefaultGraphStore::empty());
    /// # use gds::procedures::centrality::PageRankFacade;
    /// let facade = PageRankFacade::new(graph);
    /// let memory = facade.estimate_memory();
    /// println!("Will use between {} and {} bytes", memory.min(), memory.max());
    /// ```
    pub fn estimate_memory(&self) -> MemoryRange {
        let dimensions = ConcreteGraphDimensions::of(self.graph_store.node_count(), 0);
        *PageRankMemoryEstimation
            .estimate(&dimensions, self.config.concurrency)
            .memory_usage()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::properties::PropertyValues;
    use crate::types::random::{RandomGraphConfig, RandomRelationshipConfig};

    fn store() -> Arc<DefaultGraphStore> {
        let config = RandomGraphConfig {
            seed: Some(23),
            node_count: 8,
            relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
            ..RandomGraphConfig::default()
        };
        Arc::new(DefaultGraphStore::random(&config).unwrap())
    }

    #[test]
    fn test_builder_defaults() {
        let facade = PageRankFacade::new(store());
        assert_eq!(facade.config.max_iterations, 20);
        assert_eq!(facade.config.damping_factor, 0.85);
        assert_eq!(facade.config.tolerance, 1e-4);
        assert_eq!(facade.config.variant, PageRankVariant::PageRank);
    }

    #[test]
    fn test_builder_fluent_chain() {
        let facade = PageRankFacade::new(store())
            .iterations(30)
            .damping_factor(0.90)
            .tolerance(1e-5);

        assert_eq!(facade.config.max_iterations, 30);
        assert_eq!(facade.config.damping_factor, 0.90);
        assert_eq!(facade.config.tolerance, 1e-5);
    }

    #[test]
    fn test_variant_builders() {
        let facade = PageRankFacade::new(store()).article_rank();
        assert_eq!(facade.config.variant, PageRankVariant::ArticleRank);

        let facade = PageRankFacade::new(store()).eigenvector();
        assert_eq!(facade.config.variant, PageRankVariant::Eigenvector);

        let facade = PageRankFacade::new(store()).relationship_weight_property("weight");
        assert_eq!(
            facade.config.relationship_weight_property.as_deref(),
            Some("weight")
        );
    }

    #[test]
    fn test_validate_iterations() {
        let facade = PageRankFacade::new(store()).iterations(0);
        assert!(facade.validate().is_err()); // 0 is invalid

        let facade = PageRankFacade::new(store()).iterations(50);
        assert!(facade.validate().is_ok()); // 50 is valid
    }

    #[test]
    fn test_validate_damping_factor() {
        let facade = PageRankFacade::new(store()).damping_factor(0.0);
        assert!(facade.validate().is_err()); // 0.0 is invalid

        let facade = PageRankFacade::new(store()).damping_factor(1.0);
        assert!(facade.validate().is_err()); // 1.0 is invalid

        let facade = PageRankFacade::new(store()).damping_factor(0.85);
        assert!(facade.validate().is_ok()); // 0.85 is valid
    }

    #[test]
    fn test_validate_tolerance() {
        let facade = PageRankFacade::new(store()).tolerance(0.0);
        assert!(facade.validate().is_err()); // 0.0 is invalid (not positive)

        let facade = PageRankFacade::new(store()).tolerance(1e-4);
        assert!(facade.validate().is_ok()); // positive is valid
    }

    #[test]
    fn test_validate_direction() {
        let facade = PageRankFacade::new(store()).direction("sideways");
        assert!(facade.validate().is_err());

        let facade = PageRankFacade::new(store()).direction("incoming");
        assert!(facade.validate().is_ok());
    }

    #[test]
    fn test_stream_requires_validation() {
        let facade = PageRankFacade::new(store()).iterations(0); // Invalid
        assert!(facade.stream().is_err());
    }

    #[test]
    fn test_stats_requires_validation() {
        let facade = PageRankFacade::new(store()).damping_factor(0.0); // Invalid
        assert!(facade.stats().is_err());
    }

    #[test]
    fn test_mutate_requires_validation() {
        let facade = PageRankFacade::new(store()).tolerance(0.0); // Invalid
        assert!(facade.mutate("pr").is_err());
    }

    #[test]
    fn test_mutate_validates_property_name() {
        let facade = PageRankFacade::new(store()); // Valid config
        assert!(facade.mutate("").is_err()); // But empty property name
    }

    #[test]
    fn test_mutate_accepts_valid_property() {
        let facade = PageRankFacade::new(store());
        let result = facade.mutate("pagerank");
        assert!(result.is_ok()); // Should succeed with valid property
        let mutation_result = result.unwrap();
        assert_eq!(mutation_result.summary.property_name, "pagerank");
        let values = mutation_result
            .updated_store
            .node_property_values("pagerank")
            .unwrap();
        assert_eq!(values.element_count(), 8);
    }

    #[test]
    fn test_stream_returns_node_count_rows() {
        let rows: Vec<_> = PageRankFacade::new(store()).stream().unwrap().collect();
        assert_eq!(rows.len(), 8);
    }

    #[test]
    fn test_stats_shape() {
        let stats = PageRankFacade::new(store()).stats().unwrap();
        assert!(stats.max >= stats.min);
    }

    #[test]
    fn test_cycle_three_nodes_equal_scores() {
        let config = RandomGraphConfig {
            seed: Some(7),
            node_count: 3,
            // 1.0 density gives a symmetric complete digraph in the generator, which
            // should yield equal PageRank scores.
            relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
            ..RandomGraphConfig::default()
        };
        let store = Arc::new(DefaultGraphStore::random(&config).unwrap());

        let scores: Vec<_> = PageRankFacade::new(store)
            .iterations(50)
            .tolerance(1e-9)
            .stream()
            .unwrap()
            .map(|r| r.score)
            .collect();

        assert_eq!(scores.len(), 3);
        assert!((scores[0] - scores[1]).abs() < 1e-8);
        assert!((scores[1] - scores[2]).abs() < 1e-8);
    }
}
