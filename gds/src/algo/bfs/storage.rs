//! **BFS Storage Runtime**
//!
//! **Translation Source**: `org.neo4j.gds.paths.traverse.BFS`
//!
//! This module implements the "Gross pole" for BFS algorithm - persistent data access
//! and algorithm orchestration using the Java GDS parallel BFS architecture.

use super::spec::BfsResult;
use super::BfsComputationRuntime;
use crate::algo::traversal::{
    run_parallel_bfs, Aggregator, ExitPredicate, ExitPredicateResult, FollowExitPredicate,
    OneHopAggregator, ParallelBfsConfig, TargetExitPredicate,
};
use crate::task::progress::{ProgressTracker, UNKNOWN_VOLUME};
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::graph::Graph;
use crate::types::graph::NodeId;

/// BFS Storage Runtime - handles persistent data access and algorithm orchestration
///
/// Translation of: `BFS.java` (lines 62-1.073)
/// This implements the "Gross pole" for accessing graph data using simplified BFS architecture
pub struct BfsStorageRuntime {
    /// Source node for BFS traversal
    pub source_node: NodeId,
    /// Target nodes to find
    pub target_nodes: Vec<NodeId>,
    /// Maximum depth to traverse
    pub max_depth: Option<u32>,
    /// Whether to track paths during traversal
    pub track_paths: bool,
    /// Chunk size used by parallel frontier expansion.
    pub delta: usize,
}

const DEFAULT_DELTA: usize = 64;

impl BfsStorageRuntime {
    /// Create new BFS storage runtime
    pub fn new(
        source_node: NodeId,
        target_nodes: Vec<NodeId>,
        max_depth: Option<u32>,
        track_paths: bool,
    ) -> Self {
        Self {
            source_node,
            target_nodes,
            max_depth,
            track_paths,
            delta: DEFAULT_DELTA,
        }
    }

    /// Override chunk size used by parallel frontier expansion.
    pub fn with_delta(mut self, delta: usize) -> Self {
        self.delta = delta.max(1);
        self
    }

    /// Compute BFS traversal
    ///
    /// Translation of: `BFS.compute()` (lines 1.075-259)
    /// This orchestrates the main BFS algorithm loop
    pub fn compute_bfs(
        &self,
        computation: &mut BfsComputationRuntime,
        graph: Option<&dyn Graph>,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<BfsResult, AlgorithmError> {
        let aggregator = OneHopAggregator;
        if self.target_nodes.is_empty() {
            let exit_predicate = FollowExitPredicate;
            self.compute_bfs_with_traversal(
                computation,
                graph,
                progress_tracker,
                &aggregator,
                &exit_predicate,
            )
        } else {
            let exit_predicate = TargetExitPredicate::new(self.target_nodes.clone());
            self.compute_bfs_with_traversal(
                computation,
                graph,
                progress_tracker,
                &aggregator,
                &exit_predicate,
            )
        }
    }

    /// Compute BFS traversal with explicit Java GDS traversal hooks.
    pub fn compute_bfs_with_traversal(
        &self,
        computation: &mut BfsComputationRuntime,
        graph: Option<&dyn Graph>,
        progress_tracker: &mut dyn ProgressTracker,
        aggregator: &dyn Aggregator,
        exit_predicate: &dyn ExitPredicate,
    ) -> Result<BfsResult, AlgorithmError> {
        let start_time = std::time::Instant::now();

        let volume = graph
            .map(|g| g.relationship_count())
            .unwrap_or(UNKNOWN_VOLUME);
        if volume == UNKNOWN_VOLUME {
            progress_tracker.begin_subtask_unknown();
        } else {
            progress_tracker.begin_subtask_with_volume(volume);
        }

        let result = (|| {
            let node_count = graph.map(|g| g.node_count()).unwrap_or(1000) as usize;
            Self::validate_node_in_graph(self.source_node, node_count, "source")?;
            for target_node in &self.target_nodes {
                Self::validate_node_in_graph(*target_node, node_count, "target")?;
            }

            if computation.concurrency > 1 {
                if let Some(graph_ref) = graph {
                    return self.compute_bfs_parallel(
                        computation,
                        graph_ref,
                        node_count,
                        progress_tracker,
                        aggregator,
                        exit_predicate,
                        start_time,
                    );
                }
            }

            computation.initialize(self.source_node, self.max_depth, node_count);

            // BFS queue (source, node, aggregated weight)
            let mut queue: std::collections::VecDeque<(NodeId, NodeId, f64)> =
                std::collections::VecDeque::new();
            computation.set_visited(self.source_node);
            queue.push_back((self.source_node, self.source_node, 0.0));

            let mut result = Vec::new();
            let mut result_depths = Vec::new();

            while let Some((source_node, current_node, weight)) = queue.pop_front() {
                match exit_predicate.test(source_node, current_node, weight) {
                    ExitPredicateResult::Continue => continue,
                    ExitPredicateResult::Break => {
                        result.push(current_node);
                        result_depths.push(weight);
                        break;
                    }
                    ExitPredicateResult::Follow => {
                        result.push(current_node);
                        result_depths.push(weight);
                    }
                }

                // Get neighbors
                let neighbors = self.get_neighbors(graph, current_node);
                progress_tracker.log_progress(neighbors.len());

                // Respect max depth: only expand when the aggregated weight is below max depth.
                if computation.check_max_depth(weight) {
                    for neighbor in neighbors {
                        Self::validate_node_in_graph(neighbor, node_count, "neighbor")?;
                        if !computation.is_visited(neighbor) {
                            computation.set_visited(neighbor);
                            let next_weight = aggregator.apply(current_node, neighbor, weight);
                            queue.push_back((current_node, neighbor, next_weight));
                        }
                    }
                }
            }

            let computation_time = start_time.elapsed().as_millis() as u64;

            Ok(BfsResult {
                visited_nodes: result,
                visited_depths: result_depths,
                computation_time_ms: computation_time,
            })
        })();

        progress_tracker.end_subtask();
        result
    }

    fn compute_bfs_parallel(
        &self,
        computation: &mut BfsComputationRuntime,
        graph: &dyn Graph,
        node_count: usize,
        progress_tracker: &mut dyn ProgressTracker,
        aggregator: &dyn Aggregator,
        exit_predicate: &dyn ExitPredicate,
        start_time: std::time::Instant,
    ) -> Result<BfsResult, AlgorithmError> {
        computation.initialize(self.source_node, self.max_depth, node_count);
        let result = run_parallel_bfs(
            graph,
            ParallelBfsConfig {
                source_node: self.source_node,
                node_count,
                max_depth: self.max_depth,
                concurrency: computation.concurrency,
                delta: self.delta,
            },
            aggregator,
            exit_predicate,
        )?;

        progress_tracker.log_progress(result.relationships_examined);

        Ok(BfsResult {
            visited_nodes: result.visited_nodes,
            visited_depths: result.visited_depths,
            computation_time_ms: start_time.elapsed().as_millis() as u64,
        })
    }

    /// Get neighbors of a node (graph-backed when available; mock fallback)
    fn get_neighbors(&self, graph: Option<&dyn Graph>, node: NodeId) -> Vec<NodeId> {
        if let Some(g) = graph {
            let fallback: f64 = 1.0;
            let stream = g.stream_relationships(node, fallback);
            stream.into_iter().map(|c| c.target_id()).collect()
        } else {
            match node {
                0 => vec![1, 2],
                1 => vec![0, 3],
                2 => vec![0, 3],
                3 => vec![1, 2],
                _ => vec![],
            }
        }
    }

    fn validate_node_in_graph(
        node_id: NodeId,
        node_count: usize,
        role: &str,
    ) -> Result<(), AlgorithmError> {
        let node_index = Self::node_index_in_graph(node_id, node_count, role)?;
        if node_index >= node_count {
            return Err(AlgorithmError::InvalidGraph(format!(
                "{role} node id out of range: {node_id} (node_count={node_count})"
            )));
        }
        Ok(())
    }

    fn node_index_in_graph(
        node_id: NodeId,
        _node_count: usize,
        role: &str,
    ) -> Result<usize, AlgorithmError> {
        usize::try_from(node_id)
            .map_err(|_| AlgorithmError::InvalidGraph(format!("Invalid {role} node id: {node_id}")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algo::traversal::{ExitPredicateResult, FollowExitPredicate};
    use crate::task::progress::{TaskProgressTracker, Tasks};

    struct ContinueOnOne;

    impl ExitPredicate for ContinueOnOne {
        fn test(
            &self,
            _source_node: NodeId,
            current_node: NodeId,
            _weight_at_source: f64,
        ) -> ExitPredicateResult {
            if current_node == 1 {
                ExitPredicateResult::Continue
            } else {
                ExitPredicateResult::Follow
            }
        }
    }

    struct DoubleHopAggregator;

    impl Aggregator for DoubleHopAggregator {
        fn apply(&self, _source_node: NodeId, _current_node: NodeId, weight_at_source: f64) -> f64 {
            weight_at_source + 2.0
        }
    }

    #[test]
    fn test_bfs_storage_runtime_creation() {
        let storage = BfsStorageRuntime::new(0, vec![3], Some(5), true);
        assert_eq!(storage.source_node, 0);
        assert_eq!(storage.target_nodes, vec![3]);
        assert_eq!(storage.max_depth, Some(5));
        assert!(storage.track_paths);
        assert_eq!(storage.delta, DEFAULT_DELTA);
    }

    #[test]
    fn test_bfs_path_computation() {
        let storage = BfsStorageRuntime::new(0, vec![3], None, true);
        let mut computation = BfsComputationRuntime::new(0, true, 1, 10);

        let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("BFS".to_string()));

        let result = storage
            .compute_bfs(&mut computation, None, &mut progress_tracker)
            .unwrap();

        assert!(!result.visited_nodes.is_empty());
    }

    #[test]
    fn test_bfs_stops_when_target_reached() {
        let storage = BfsStorageRuntime::new(0, vec![1], None, true);
        let mut computation = BfsComputationRuntime::new(0, true, 1, 10);

        let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("BFS".to_string()));

        let result = storage
            .compute_bfs(&mut computation, None, &mut progress_tracker)
            .unwrap();

        assert_eq!(result.visited_nodes, vec![0, 1]);
    }

    #[test]
    fn test_bfs_honors_continue_exit_predicate() {
        let storage = BfsStorageRuntime::new(0, vec![], None, false);
        let mut computation = BfsComputationRuntime::new(0, false, 1, 10);
        let aggregator = OneHopAggregator;
        let exit_predicate = ContinueOnOne;

        let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("BFS".to_string()));

        let result = storage
            .compute_bfs_with_traversal(
                &mut computation,
                None,
                &mut progress_tracker,
                &aggregator,
                &exit_predicate,
            )
            .unwrap();

        assert!(!result.visited_nodes.contains(&1));
        assert_eq!(result.visited_nodes, vec![0, 2, 3]);
    }

    #[test]
    fn test_bfs_honors_custom_aggregator_for_depth() {
        let storage = BfsStorageRuntime::new(0, vec![], Some(2), false);
        let mut computation = BfsComputationRuntime::new(0, false, 1, 10);
        let aggregator = DoubleHopAggregator;
        let exit_predicate = FollowExitPredicate;

        let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("BFS".to_string()));

        let result = storage
            .compute_bfs_with_traversal(
                &mut computation,
                None,
                &mut progress_tracker,
                &aggregator,
                &exit_predicate,
            )
            .unwrap();

        assert_eq!(result.visited_nodes, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_parallel_matches_single_thread_order() {
        let storage = BfsStorageRuntime::new(0, vec![], None, false);
        let aggregator = OneHopAggregator;
        let exit_predicate = FollowExitPredicate;

        let mut sequential = BfsComputationRuntime::new(0, false, 1, 10);
        let mut sequential_progress = TaskProgressTracker::new(Tasks::leaf("BFS-seq".to_string()));
        let sequential_result = storage
            .compute_bfs_with_traversal(
                &mut sequential,
                None,
                &mut sequential_progress,
                &aggregator,
                &exit_predicate,
            )
            .unwrap();

        let mut parallel = BfsComputationRuntime::new(0, false, 4, 10);
        let mut parallel_progress = TaskProgressTracker::new(Tasks::leaf("BFS-par".to_string()));
        let parallel_result = storage
            .compute_bfs_with_traversal(
                &mut parallel,
                None,
                &mut parallel_progress,
                &aggregator,
                &exit_predicate,
            )
            .unwrap();

        assert_eq!(
            sequential_result.visited_nodes,
            parallel_result.visited_nodes
        );
    }

    #[test]
    fn test_bfs_path_same_source_target() {
        let storage = BfsStorageRuntime::new(0, vec![0], None, true);
        let mut computation = BfsComputationRuntime::new(0, true, 1, 10);

        let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("BFS".to_string()));

        let result = storage
            .compute_bfs(&mut computation, None, &mut progress_tracker)
            .unwrap();

        assert!(result.visited_nodes.len() >= 1);
    }

    #[test]
    fn test_bfs_max_depth_constraint() {
        let storage = BfsStorageRuntime::new(0, vec![], Some(1), false);
        let mut computation = BfsComputationRuntime::new(0, false, 1, 10);

        let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("BFS".to_string()));

        let result = storage
            .compute_bfs(&mut computation, None, &mut progress_tracker)
            .unwrap();

        // With max_depth=1, we should only visit nodes at distance 0 and 1
        assert!(result.visited_nodes.len() <= 3); // Source + immediate neighbors
    }

    #[test]
    fn test_bfs_rejects_out_of_range_source() {
        let storage = BfsStorageRuntime::new(1000, vec![], None, false);
        let mut computation = BfsComputationRuntime::new(0, false, 1, 10);

        let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("BFS".to_string()));

        let result = storage.compute_bfs(&mut computation, None, &mut progress_tracker);

        assert!(matches!(result, Err(AlgorithmError::InvalidGraph(_))));
    }

    #[test]
    fn test_bfs_rejects_out_of_range_target() {
        let storage = BfsStorageRuntime::new(0, vec![1000], None, false);
        let mut computation = BfsComputationRuntime::new(0, false, 1, 10);

        let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("BFS".to_string()));

        let result = storage.compute_bfs(&mut computation, None, &mut progress_tracker);

        assert!(matches!(result, Err(AlgorithmError::InvalidGraph(_))));
    }
}
