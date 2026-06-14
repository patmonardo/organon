//! Degree Centrality Facade
//!
//! **What is it?**: Counts the number of connections (edges) each node has
//! **Why care?**: Identifies highly-connected nodes (hubs) in the network
//! **Complexity**: O(V + E) - linear in graph size
//! **Best for**: Quick identification of important nodes by connectivity
//!
//! ## Example
//!
//! ```rust,no_run
//! # use gds::Graph;
//! # use std::sync::Arc;
//! # use gds::types::prelude::DefaultGraphStore;
//! # let graph = Graph::new(Arc::new(DefaultGraphStore::empty()));
//! // Get degree scores for all nodes
//! let results = graph.degree_centrality()
//!     .stream()?
//!     .collect::<Vec<_>>();
//!
//! // Get statistics
//! let stats = graph.degree_centrality().stats()?;
//! println!("Average degree: {}", stats.mean);
//!
//! // Store as node property for use in other algorithms
//! graph.degree_centrality().mutate("degree")?;
//! ```

use crate::algo::algorithms::{CentralityScore, Result};
use crate::algo::algorithms::{ConfigValidator, WriteResult};
pub use crate::algo::degree_centrality::storage::Orientation;
use crate::algo::degree_centrality::{
    degree_centrality_progress_task, parse_degree_orientation, DegreeCentralityComputationRuntime,
    DegreeCentralityConfig, DegreeCentralityMutateResult, DegreeCentralityMutationSummary,
    DegreeCentralityResult, DegreeCentralityResultBuilder, DegreeCentralityStats,
    DegreeCentralityStorageRuntime,
};
use crate::collections::backends::vec::VecDouble;
use crate::task::concurrency::{Concurrency, TerminationFlag};
use crate::task::progress::{
    EmptyTaskRegistryFactory, JobId, ProgressTracker, TaskProgressTracker, TaskRegistryFactory,
};
use crate::task::memory::MemoryRange;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::NodeLabel;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use crate::types::properties::node::DefaultDoubleNodePropertyValues;
use crate::types::properties::node::NodePropertyValues;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

// ============================================================================
// Facade Type
// ============================================================================

/// DegreeCentrality algorithm facade/builder bound to a live graph store.
#[derive(Clone)]
pub struct DegreeCentralityFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: DegreeCentralityConfig,
    task_registry: Arc<dyn TaskRegistryFactory>,
}

impl DegreeCentralityFacade {
    /// Create a new DegreeCentrality facade bound to a graph store.
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: DegreeCentralityConfig::default(),
            task_registry: Arc::new(EmptyTaskRegistryFactory),
        }
    }

    /// Create a facade using the spec.rs config model.
    pub fn from_spec_config(
        graph_store: Arc<DefaultGraphStore>,
        config: DegreeCentralityConfig,
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
        let parsed: DegreeCentralityConfig = serde_json::from_value(raw_config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        Self::from_spec_config(graph_store, parsed)
    }

    /// Apply a spec.rs config onto an existing facade.
    pub fn with_spec_config(mut self, config: DegreeCentralityConfig) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;
        self.config = config;
        Ok(self)
    }

    /// Normalize scores by max degree.
    pub fn normalize(mut self, normalize: bool) -> Self {
        self.config.normalize = normalize;
        self
    }

    /// Set orientation for degree computation.
    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.config.orientation = match orientation {
            Orientation::Natural => "natural".to_string(),
            Orientation::Reverse => "reverse".to_string(),
            Orientation::Undirected => "undirected".to_string(),
        };
        self
    }

    /// Use relationship weights when computing degree (sum of weights).
    pub fn weighted(mut self, weighted: bool) -> Self {
        self.config.weighted = weighted;
        self
    }

    /// Select a relationship weight property and compute degree as sum of positive weights.
    pub fn relationship_weight_property(mut self, property_name: impl Into<String>) -> Self {
        self.config.relationship_weight_property = Some(property_name.into());
        self.config.weighted = true;
        self
    }

    /// Set concurrency level
    ///
    /// Number of parallel threads to use.
    /// Degree centrality benefits from parallelism in large graphs.
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.config.concurrency = concurrency;
        self
    }

    /// Compatibility alias for older builder call sites; prefer `task_registry`.
    pub fn task_registry_factory(mut self, factory: Box<dyn TaskRegistryFactory>) -> Self {
        self.task_registry = factory.into();
        self
    }

    /// Set task registry factory for progress tracking.
    pub fn task_registry(mut self, task_registry: Arc<dyn TaskRegistryFactory>) -> Self {
        self.task_registry = task_registry;
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

    fn compute(&self) -> Result<DegreeCentralityResult> {
        self.validate()?;

        let start = Instant::now();

        let orientation = parse_degree_orientation(&self.config.orientation)?;

        let storage = DegreeCentralityStorageRuntime::with_relationship_weight_property(
            self.graph_store.as_ref(),
            orientation,
            self.config
                .relationship_weight_property
                .as_deref()
                .or(self.config.weighted.then_some("weight")),
        )?;

        let node_count = storage.node_count();
        if node_count == 0 {
            return Ok(DegreeCentralityResult {
                centralities: Vec::new(),
                node_count,
                execution_time: start.elapsed(),
            });
        }

        let concurrency = Concurrency::of(self.config.concurrency.max(1));
        let computation = DegreeCentralityComputationRuntime::new();

        let mut progress_tracker = TaskProgressTracker::with_registry(
            degree_centrality_progress_task(node_count).base().clone(),
            concurrency,
            JobId::new(),
            self.task_registry.as_ref(),
        );
        progress_tracker.begin_subtask_with_volume(node_count);

        let progress_handle = progress_tracker.clone();
        let on_nodes_done = Arc::new(move |n: usize| {
            let mut tracker = progress_handle.clone();
            tracker.log_progress(n);
        });

        let termination = TerminationFlag::running_true();

        let mut scores = match storage.compute_parallel(
            &computation,
            concurrency,
            &termination,
            on_nodes_done,
        ) {
            Ok(scores) => scores,
            Err(e) => {
                progress_tracker.end_subtask_with_failure();
                return Err(AlgorithmError::Execution(format!(
                    "Degree centrality terminated: {e}"
                )));
            }
        };

        if self.config.normalize {
            computation.normalize_scores(&mut scores);
        }

        progress_tracker.end_subtask();

        Ok(DegreeCentralityResult {
            centralities: scores,
            node_count,
            execution_time: start.elapsed(),
        })
    }

    /// Stream mode: Get degree for each node
    ///
    /// Returns an iterator over (node_id, degree) tuples.
    /// Degrees are raw counts by default (not normalized).
    ///
    /// ## Example
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # use std::sync::Arc;
    /// # use gds::types::prelude::DefaultGraphStore;
    /// # let graph = Arc::new(DefaultGraphStore::empty());
    /// # use gds::procedures::centrality::DegreeCentralityFacade;
    /// let facade = DegreeCentralityFacade::new(graph);
    /// for (node_id, degree) in facade.stream()? {
    ///     if degree > 100.0 {
    ///         println!("Hub: node {} has {} connections", node_id, degree);
    ///     }
    /// }
    /// ```
    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = CentralityScore>>> {
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

    /// Stats mode: Get aggregated statistics about degree distribution
    ///
    /// Returns min, max, mean, stddev, and percentiles of the degree distribution.
    /// This is useful for understanding the overall graph structure without
    /// needing to process individual scores.
    ///
    /// ## Example
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # use std::sync::Arc;
    /// # use gds::types::prelude::DefaultGraphStore;
    /// # let graph = Arc::new(DefaultGraphStore::empty());
    /// # use gds::procedures::centrality::DegreeCentralityFacade;
    /// let facade = DegreeCentralityFacade::new(graph);
    /// let stats = facade.stats()?;
    /// println!("Graph has average degree: {}", stats.mean);
    /// println!("Max degree (highest hub): {}", stats.max);
    /// println!("Isolated nodes: {}", stats.isolated_nodes);
    /// ```
    pub fn stats(&self) -> Result<DegreeCentralityStats> {
        self.validate()?;
        let result = self.compute()?;
        Ok(DegreeCentralityResultBuilder::new(result).stats())
    }

    /// Mutate mode: Compute and store degree as a node property
    ///
    /// Stores the degree of each node as a property in the graph.
    /// This allows other algorithms to use the degree as input.
    ///
    /// ## Example
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # use std::sync::Arc;
    /// # use gds::types::prelude::DefaultGraphStore;
    /// # let graph = Arc::new(DefaultGraphStore::empty());
    /// # use gds::procedures::centrality::DegreeCentralityFacade;
    /// let facade = DegreeCentralityFacade::new(graph);
    /// let result = facade.mutate("degree")?;
    /// println!("Updated {} nodes", result.nodes_updated);
    /// ```
    pub fn mutate(self, property_name: &str) -> Result<DegreeCentralityMutateResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;
        let result = self.compute()?;
        let scores = result.centralities.clone();
        let nodes_updated = scores.len() as u64;
        let builder = DegreeCentralityResultBuilder::new(result);

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
                AlgorithmError::Execution(format!(
                    "Degree centrality mutate failed to add property: {e}"
                ))
            })?;

        let summary = DegreeCentralityMutationSummary {
            nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: builder.execution_time_ms(),
        };

        Ok(DegreeCentralityMutateResult {
            summary,
            updated_store: Arc::new(new_store),
        })
    }

    /// Write mode: Compute and write results to external storage
    ///
    /// Writes the degree centrality scores to an external data store.
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
    /// # let graph = Graph::default();
    /// # use std::sync::Arc;
    /// # use gds::types::prelude::DefaultGraphStore;
    /// # let graph = Arc::new(DefaultGraphStore::empty());
    /// # use gds::procedures::centrality::DegreeCentralityFacade;
    /// let facade = DegreeCentralityFacade::new(graph);
    /// let result = facade.write("degree_centrality")?;
    /// println!("Wrote {} records", result.records_written);
    /// ```
    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;
        let result = self.compute()?;

        // For now, use placeholder write - just count the nodes that would be written
        // TODO: Implement actual persistence to external storage
        let nodes_written = result.centralities.len() as u64;

        let execution_time = result.execution_time;
        Ok(WriteResult::new(
            nodes_written,
            property_name.to_string(),
            execution_time,
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
    /// # let graph = Graph::default();
    /// # use std::sync::Arc;
    /// # use gds::types::prelude::DefaultGraphStore;
    /// # let graph = Arc::new(DefaultGraphStore::empty());
    /// # use gds::procedures::centrality::DegreeCentralityFacade;
    /// let facade = DegreeCentralityFacade::new(graph);
    /// let memory = facade.estimate_memory();
    /// println!("Will use between {} and {} bytes", memory.min_bytes, memory.max_bytes);
    /// ```
    pub fn estimate_memory(&self) -> MemoryRange {
        let node_count = self.graph_store.node_count();
        let concurrency = self.config.concurrency.max(1);

        // One atomic score buffer during computation plus the materialized result vector.
        let scores_memory = node_count * std::mem::size_of::<f64>() * 2;

        // Executor and per-worker batching overhead.
        let executor_memory = concurrency * 64 * 1024;

        // Additional overhead for graph cursors and temporary structures.
        let computation_overhead = 1024 * 1024;

        let total_memory = scores_memory + executor_memory + computation_overhead;
        let total_with_overhead = total_memory + (total_memory / 5);

        MemoryRange::of_range(total_memory, total_with_overhead)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GraphStoreConfig;
    use crate::projection::RelationshipType;
    use crate::types::graph::{RelationshipTopology, SimpleIdMap};
    use crate::types::graph_store::{
        Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName,
    };
    use crate::types::properties::relationship::{
        DefaultRelationshipPropertyValues, RelationshipPropertyValues,
    };
    use crate::types::random::{RandomGraphConfig, RandomRelationshipConfig};
    use crate::types::schema::{Direction, MutableGraphSchema};
    use std::collections::HashMap;

    fn store() -> Arc<DefaultGraphStore> {
        let config = RandomGraphConfig {
            seed: Some(3),
            node_count: 8,
            relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
            ..RandomGraphConfig::default()
        };
        Arc::new(DefaultGraphStore::random(&config).unwrap())
    }

    fn weighted_store_from_outgoing(
        outgoing: Vec<Vec<i64>>,
        weights: Vec<f64>,
    ) -> Arc<DefaultGraphStore> {
        weighted_store_from_outgoing_with_property(outgoing, weights, "weight")
    }

    fn weighted_store_from_outgoing_with_property(
        outgoing: Vec<Vec<i64>>,
        weights: Vec<f64>,
        property_name: &str,
    ) -> Arc<DefaultGraphStore> {
        let node_count = outgoing.len();
        let mut incoming: Vec<Vec<i64>> = vec![Vec::new(); node_count];
        for (source, targets) in outgoing.iter().enumerate() {
            for &target in targets {
                if target >= 0 && (target as usize) < node_count {
                    incoming[target as usize].push(source as i64);
                }
            }
        }

        let rel_type = RelationshipType::of("REL");

        let mut schema_builder = MutableGraphSchema::empty();
        schema_builder
            .relationship_schema_mut()
            .add_relationship_type(rel_type.clone(), Direction::Directed);
        let schema = schema_builder.build();

        let mut relationship_topologies = HashMap::new();
        relationship_topologies.insert(
            rel_type.clone(),
            RelationshipTopology::new(outgoing, Some(incoming)),
        );

        let original_ids: Vec<i64> = (0..node_count as i64).collect();
        let id_map = SimpleIdMap::from_original_ids(original_ids);

        let mut store = DefaultGraphStore::new(
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
        );

        let element_count = weights.len();
        let values: Arc<dyn RelationshipPropertyValues> = Arc::new(
            DefaultRelationshipPropertyValues::with_values(weights, 0.0, element_count),
        );
        store
            .add_relationship_property(rel_type, property_name, values)
            .unwrap();

        Arc::new(store)
    }

    #[test]
    fn test_facade_creation() {
        let _facade = DegreeCentralityFacade::new(store());
        // Smoke test - just verify it creates without panic
    }

    #[test]
    fn test_stream_returns_iterator() {
        let facade = DegreeCentralityFacade::new(store());
        let result = facade.stream();
        assert!(result.is_ok());
    }

    #[test]
    fn test_memory_estimate_has_range() {
        let facade = DegreeCentralityFacade::new(store()).concurrency(2);
        let memory = facade.estimate_memory();
        assert!(memory.max() >= memory.min());
    }

    #[test]
    fn test_stats_returns_valid_structure() {
        let facade = DegreeCentralityFacade::new(store());
        let stats = facade.stats().unwrap();
        assert!(stats.max >= stats.min);
        assert_eq!(stats.node_count, 8);
    }

    #[test]
    fn test_mutate_validates_property_name() {
        let facade = DegreeCentralityFacade::new(store());
        let result = facade.mutate("");
        assert!(result.is_err()); // Empty property name should fail
    }

    #[test]
    fn test_mutate_accepts_valid_property_name() {
        let facade = DegreeCentralityFacade::new(store());
        let result = facade.mutate("degree");
        assert!(result.is_ok()); // Should succeed with valid property name
        let mutation_result = result.unwrap();
        assert_eq!(mutation_result.summary.property_name, "degree");
        assert!(mutation_result.updated_store.has_node_property("degree"));
    }

    #[test]
    fn test_invalid_orientation_fails_fast() {
        let facade =
            DegreeCentralityFacade::new(store()).with_spec_config(DegreeCentralityConfig {
                orientation: "sideways".to_string(),
                ..DegreeCentralityConfig::default()
            });

        assert!(facade.is_err());
    }

    #[test]
    fn weighted_degree_ignores_non_positive_weights() {
        let store =
            weighted_store_from_outgoing(vec![vec![1, 2], vec![2], vec![]], vec![2.0, -3.0, 0.0]);

        let scores: Vec<_> = DegreeCentralityFacade::new(store)
            .weighted(true)
            .stream()
            .unwrap()
            .map(|row| row.score)
            .collect();

        assert_eq!(scores, vec![2.0, 0.0, 0.0]);
    }

    #[test]
    fn degree_orientation_matches_java_natural_reverse_undirected() {
        let store =
            weighted_store_from_outgoing(vec![vec![1, 2], vec![2], vec![]], vec![2.0, 3.0, 5.0]);

        let natural: Vec<_> = DegreeCentralityFacade::new(Arc::clone(&store))
            .orientation(Orientation::Natural)
            .stream()
            .unwrap()
            .map(|row| row.score)
            .collect();
        let reverse: Vec<_> = DegreeCentralityFacade::new(Arc::clone(&store))
            .orientation(Orientation::Reverse)
            .stream()
            .unwrap()
            .map(|row| row.score)
            .collect();
        let undirected: Vec<_> = DegreeCentralityFacade::new(store)
            .orientation(Orientation::Undirected)
            .stream()
            .unwrap()
            .map(|row| row.score)
            .collect();

        assert_eq!(natural, vec![2.0, 1.0, 0.0]);
        assert_eq!(reverse, vec![0.0, 1.0, 2.0]);
        assert_eq!(undirected, vec![2.0, 2.0, 2.0]);
    }

    #[test]
    fn weighted_degree_orientation_matches_java_weight_sums() {
        let store =
            weighted_store_from_outgoing(vec![vec![1, 2], vec![2], vec![]], vec![2.0, 3.0, 5.0]);

        let natural: Vec<_> = DegreeCentralityFacade::new(Arc::clone(&store))
            .weighted(true)
            .orientation(Orientation::Natural)
            .stream()
            .unwrap()
            .map(|row| row.score)
            .collect();
        let reverse: Vec<_> = DegreeCentralityFacade::new(Arc::clone(&store))
            .weighted(true)
            .orientation(Orientation::Reverse)
            .stream()
            .unwrap()
            .map(|row| row.score)
            .collect();
        let undirected: Vec<_> = DegreeCentralityFacade::new(store)
            .weighted(true)
            .orientation(Orientation::Undirected)
            .stream()
            .unwrap()
            .map(|row| row.score)
            .collect();

        assert_eq!(natural, vec![5.0, 5.0, 0.0]);
        assert_eq!(reverse, vec![0.0, 2.0, 8.0]);
        assert_eq!(undirected, vec![5.0, 7.0, 8.0]);
    }

    #[test]
    fn relationship_weight_property_selects_named_property() {
        let store = weighted_store_from_outgoing_with_property(
            vec![vec![1, 2], vec![2], vec![]],
            vec![7.0, 11.0, 13.0],
            "cost",
        );

        let scores: Vec<_> = DegreeCentralityFacade::new(store)
            .relationship_weight_property("cost")
            .stream()
            .unwrap()
            .map(|row| row.score)
            .collect();

        assert_eq!(scores, vec![18.0, 13.0, 0.0]);
    }
}
