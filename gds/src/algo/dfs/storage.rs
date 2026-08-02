//! **DFS Storage Runtime**
//!
//! **Translation Source**: `org.neo4j.gds.traversal.DFS`
//!
//! This module implements the "Gross pole" for DFS algorithm - persistent data access
//! and algorithm orchestration.

use super::spec::DfsResult;
use super::DfsComputationRuntime;
use crate::algo::traversal::{
    run_sequential_dfs, Aggregator, ExitPredicate, FollowExitPredicate, OneHopAggregator,
    SequentialDfsConfig, TargetExitPredicate,
};
use crate::projection::eval::algorithm::AlgorithmError;
use crate::task::concurrency::TerminationFlag;
use crate::task::progress::{ProgressTracker, UNKNOWN_VOLUME};
use crate::types::graph::Graph;
use crate::types::graph::MappedNodeId;

/// DFS Storage Runtime - handles persistent data access and algorithm orchestration
///
/// Translation of: `DFS.java` (lines 76-1.050)
/// This implements the "Gross pole" for accessing graph data
pub struct DfsStorageRuntime {
    /// Source node for DFS traversal
    pub source_node: MappedNodeId,
    /// Target nodes to find
    pub target_nodes: Vec<MappedNodeId>,
    /// Maximum depth to traverse
    pub max_depth: Option<u32>,
    /// Whether to track paths during traversal
    pub track_paths: bool,
    /// Concurrency level
    pub concurrency: usize,
}

impl DfsStorageRuntime {
    /// Create new DFS storage runtime
    pub fn new(
        source_node: MappedNodeId,
        target_nodes: Vec<MappedNodeId>,
        max_depth: Option<u32>,
        track_paths: bool,
        concurrency: usize,
    ) -> Self {
        Self {
            source_node,
            target_nodes,
            max_depth,
            track_paths,
            concurrency,
        }
    }

    /// Compute DFS traversal
    ///
    /// Translation of: `DFS.compute()` (lines 1.051.0-200)
    /// This orchestrates the main DFS algorithm loop using stacks
    pub fn compute_dfs(
        &self,
        computation: &mut DfsComputationRuntime,
        graph: Option<&dyn Graph>,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<DfsResult, AlgorithmError> {
        let termination_flag = TerminationFlag::running_true();
        self.compute_dfs_with_local_lifecycle(
            computation,
            graph,
            progress_tracker,
            &termination_flag,
        )
    }

    pub fn compute_dfs_with_context(
        &self,
        computation: &mut DfsComputationRuntime,
        graph: Option<&dyn Graph>,
        progress_tracker: &mut dyn ProgressTracker,
        termination_flag: &TerminationFlag,
    ) -> Result<DfsResult, AlgorithmError> {
        let aggregator = OneHopAggregator;
        if self.target_nodes.is_empty() {
            let exit_predicate = FollowExitPredicate;
            self.compute_dfs_with_traversal_context(
                computation,
                graph,
                progress_tracker,
                &aggregator,
                &exit_predicate,
                termination_flag,
            )
        } else {
            let exit_predicate = TargetExitPredicate::new(self.target_nodes.clone());
            self.compute_dfs_with_traversal_context(
                computation,
                graph,
                progress_tracker,
                &aggregator,
                &exit_predicate,
                termination_flag,
            )
        }
    }

    fn compute_dfs_with_local_lifecycle(
        &self,
        computation: &mut DfsComputationRuntime,
        graph: Option<&dyn Graph>,
        progress_tracker: &mut dyn ProgressTracker,
        termination_flag: &TerminationFlag,
    ) -> Result<DfsResult, AlgorithmError> {
        let volume = graph
            .map(|g| g.relationship_count())
            .unwrap_or(UNKNOWN_VOLUME);
        if volume == UNKNOWN_VOLUME {
            progress_tracker.begin_subtask_unknown();
        } else {
            progress_tracker.begin_subtask_with_volume(volume);
        }

        let result =
            self.compute_dfs_with_context(computation, graph, progress_tracker, termination_flag);
        match result {
            Ok(value) => {
                progress_tracker.end_subtask();
                Ok(value)
            }
            Err(error) => {
                progress_tracker.end_subtask_with_failure();
                Err(error)
            }
        }
    }

    /// Compute DFS traversal with explicit Java GDS traversal hooks.
    pub fn compute_dfs_with_traversal(
        &self,
        computation: &mut DfsComputationRuntime,
        graph: Option<&dyn Graph>,
        progress_tracker: &mut dyn ProgressTracker,
        aggregator: &dyn Aggregator,
        exit_predicate: &dyn ExitPredicate,
    ) -> Result<DfsResult, AlgorithmError> {
        let termination_flag = TerminationFlag::running_true();
        let volume = graph
            .map(|g| g.relationship_count())
            .unwrap_or(UNKNOWN_VOLUME);
        if volume == UNKNOWN_VOLUME {
            progress_tracker.begin_subtask_unknown();
        } else {
            progress_tracker.begin_subtask_with_volume(volume);
        }

        let result = self.compute_dfs_with_traversal_context(
            computation,
            graph,
            progress_tracker,
            aggregator,
            exit_predicate,
            &termination_flag,
        );
        match result {
            Ok(value) => {
                progress_tracker.end_subtask();
                Ok(value)
            }
            Err(error) => {
                progress_tracker.end_subtask_with_failure();
                Err(error)
            }
        }
    }

    pub fn compute_dfs_with_traversal_context(
        &self,
        computation: &mut DfsComputationRuntime,
        graph: Option<&dyn Graph>,
        progress_tracker: &mut dyn ProgressTracker,
        aggregator: &dyn Aggregator,
        exit_predicate: &dyn ExitPredicate,
        termination_flag: &TerminationFlag,
    ) -> Result<DfsResult, AlgorithmError> {
        let start_time = std::time::Instant::now();

        (|| {
            Self::ensure_running(termination_flag)?;
            let node_count = match graph {
                Some(graph) => usize::try_from(graph.node_count()).map_err(|_| {
                    AlgorithmError::InvalidGraph(format!(
                        "node count exceeds physical index capacity: {}",
                        graph.node_count()
                    ))
                })?,
                None => 1000,
            };
            Self::validate_node_in_graph(self.source_node, node_count, "source")?;
            for target_node in &self.target_nodes {
                Self::validate_node_in_graph(*target_node, node_count, "target")?;
            }

            computation.initialize(self.source_node, self.max_depth, node_count);
            let result = run_sequential_dfs(
                SequentialDfsConfig {
                    source_node: self.source_node,
                    node_count,
                    max_depth: self.max_depth,
                },
                aggregator,
                exit_predicate,
                termination_flag,
                |node| self.get_neighbors(graph, node),
            )?;

            progress_tracker.log_progress(result.relationships_examined);

            let computation_time = start_time.elapsed().as_millis() as u64;

            Ok(DfsResult {
                visited_nodes: result.visited_nodes,
                visited_depths: result.visited_depths,
                computation_time_ms: computation_time,
            })
        })()
    }

    fn ensure_running(termination_flag: &TerminationFlag) -> Result<(), AlgorithmError> {
        if termination_flag.running() {
            Ok(())
        } else {
            Err(AlgorithmError::Execution(
                "DFS computation terminated".to_string(),
            ))
        }
    }

    /// Get neighbors of a node (graph-backed when available; mock fallback)
    fn get_neighbors(&self, graph: Option<&dyn Graph>, node: MappedNodeId) -> Vec<MappedNodeId> {
        if let Some(g) = graph {
            let fallback: f64 = 1.0;
            let stream = g.stream_relationships(node, fallback);
            stream.into_iter().map(|c| c.target_id()).collect()
        } else {
            const ONE: MappedNodeId = MappedNodeId::new(1);
            const TWO: MappedNodeId = MappedNodeId::new(2);
            const THREE: MappedNodeId = MappedNodeId::new(3);

            match node {
                MappedNodeId::ZERO => vec![ONE, TWO],
                ONE => vec![MappedNodeId::ZERO, THREE],
                TWO => vec![MappedNodeId::ZERO, THREE],
                THREE => vec![ONE, TWO],
                _ => vec![],
            }
        }
    }

    fn validate_node_in_graph(
        node_id: MappedNodeId,
        node_count: usize,
        role: &str,
    ) -> Result<(), AlgorithmError> {
        let node_index = usize::try_from(node_id).map_err(|_| {
            AlgorithmError::InvalidGraph(format!("Invalid {role} node id: {node_id}"))
        })?;
        if node_index >= node_count {
            return Err(AlgorithmError::InvalidGraph(format!(
                "{role} node id out of range: {node_id} (node_count={node_count})"
            )));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algo::traversal::{ExitPredicateResult, FollowExitPredicate};
    use crate::task::progress::{NoopProgressTracker, TaskProgressTracker, Tasks};

    fn mapped(node_id: u64) -> MappedNodeId {
        MappedNodeId::new(node_id)
    }

    #[test]
    fn context_computation_honors_pre_terminated_request() {
        let storage = DfsStorageRuntime::new(mapped(0), Vec::new(), None, false, 1);
        let mut computation = DfsComputationRuntime::new(mapped(0), false, 1, 4);
        let mut progress_tracker = NoopProgressTracker;
        let termination_flag = TerminationFlag::stop_running();

        let result = storage.compute_dfs_with_context(
            &mut computation,
            None,
            &mut progress_tracker,
            &termination_flag,
        );

        assert!(
            matches!(result, Err(AlgorithmError::Execution(message)) if message.contains("terminated"))
        );
    }

    struct ContinueOnTwo;

    impl ExitPredicate for ContinueOnTwo {
        fn test(
            &self,
            _source_node: MappedNodeId,
            current_node: MappedNodeId,
            _weight_at_source: f64,
        ) -> ExitPredicateResult {
            if current_node == mapped(2) {
                ExitPredicateResult::Continue
            } else {
                ExitPredicateResult::Follow
            }
        }
    }

    struct DoubleHopAggregator;

    impl Aggregator for DoubleHopAggregator {
        fn apply(
            &self,
            _source_node: MappedNodeId,
            _current_node: MappedNodeId,
            weight_at_source: f64,
        ) -> f64 {
            weight_at_source + 2.0
        }
    }

    #[test]
    fn test_dfs_storage_runtime_creation() {
        let storage = DfsStorageRuntime::new(mapped(0), vec![mapped(3)], Some(5), true, 4);
        assert_eq!(storage.source_node, mapped(0));
        assert_eq!(storage.target_nodes, vec![mapped(3)]);
        assert_eq!(storage.max_depth, Some(5));
        assert!(storage.track_paths);
        assert_eq!(storage.concurrency, 4);
    }

    #[test]
    fn test_dfs_path_computation() {
        let storage = DfsStorageRuntime::new(mapped(0), vec![mapped(3)], None, true, 1);
        let mut computation = DfsComputationRuntime::new(mapped(0), true, 1, 10);

        let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("DFS".to_string()));

        let result = storage
            .compute_dfs(&mut computation, None, &mut progress_tracker)
            .unwrap();

        assert!(!result.visited_nodes.is_empty());
        assert!(result.visited_nodes.contains(&mapped(0)));
    }

    #[test]
    fn test_dfs_mock_traversal_follows_edges() {
        let storage = DfsStorageRuntime::new(mapped(0), vec![], None, false, 1);
        let mut computation = DfsComputationRuntime::new(mapped(0), false, 1, 10);

        let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("DFS".to_string()));

        let result = storage
            .compute_dfs(&mut computation, None, &mut progress_tracker)
            .unwrap();

        assert!(result.visited_nodes.len() > 1);
        assert_eq!(result.visited_nodes[0], mapped(0));
    }

    #[test]
    fn test_dfs_stops_when_target_reached() {
        let storage = DfsStorageRuntime::new(mapped(0), vec![mapped(2)], None, true, 1);
        let mut computation = DfsComputationRuntime::new(mapped(0), true, 1, 10);

        let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("DFS".to_string()));

        let result = storage
            .compute_dfs(&mut computation, None, &mut progress_tracker)
            .unwrap();

        assert_eq!(result.visited_nodes, vec![mapped(0), mapped(2)]);
    }

    #[test]
    fn test_dfs_honors_continue_exit_predicate() {
        let storage = DfsStorageRuntime::new(mapped(0), vec![], None, false, 1);
        let mut computation = DfsComputationRuntime::new(mapped(0), false, 1, 10);
        let aggregator = OneHopAggregator;
        let exit_predicate = ContinueOnTwo;

        let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("DFS".to_string()));

        let result = storage
            .compute_dfs_with_traversal(
                &mut computation,
                None,
                &mut progress_tracker,
                &aggregator,
                &exit_predicate,
            )
            .unwrap();

        assert!(!result.visited_nodes.contains(&mapped(2)));
        assert_eq!(result.visited_nodes, vec![mapped(0), mapped(1), mapped(3)]);
    }

    #[test]
    fn test_dfs_honors_custom_aggregator_for_depth() {
        let storage = DfsStorageRuntime::new(mapped(0), vec![], Some(2), false, 1);
        let mut computation = DfsComputationRuntime::new(mapped(0), false, 1, 10);
        let aggregator = DoubleHopAggregator;
        let exit_predicate = FollowExitPredicate;

        let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("DFS".to_string()));

        let result = storage
            .compute_dfs_with_traversal(
                &mut computation,
                None,
                &mut progress_tracker,
                &aggregator,
                &exit_predicate,
            )
            .unwrap();

        assert_eq!(result.visited_nodes, vec![mapped(0), mapped(2), mapped(1)]);
    }

    #[test]
    fn test_dfs_path_same_source_target() {
        let storage = DfsStorageRuntime::new(mapped(0), vec![mapped(0)], None, true, 1);
        let mut computation = DfsComputationRuntime::new(mapped(0), true, 1, 10);

        let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("DFS".to_string()));

        let result = storage
            .compute_dfs(&mut computation, None, &mut progress_tracker)
            .unwrap();

        assert_eq!(result.visited_nodes, vec![mapped(0)]);
    }

    #[test]
    fn test_dfs_max_depth_constraint() {
        let storage = DfsStorageRuntime::new(mapped(0), vec![], Some(1), false, 1);
        let mut computation = DfsComputationRuntime::new(mapped(0), false, 1, 10);

        let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("DFS".to_string()));

        let result = storage
            .compute_dfs(&mut computation, None, &mut progress_tracker)
            .unwrap();

        // With max_depth=1, should visit source and its neighbors
        assert!(!result.visited_nodes.is_empty());
        assert!(result.visited_nodes.len() <= 3); // Source + immediate neighbors
    }

    #[test]
    fn test_dfs_rejects_out_of_range_source() {
        let storage = DfsStorageRuntime::new(mapped(1000), vec![], None, false, 1);
        let mut computation = DfsComputationRuntime::new(mapped(0), false, 1, 10);

        let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("DFS".to_string()));

        let result = storage.compute_dfs(&mut computation, None, &mut progress_tracker);

        assert!(matches!(result, Err(AlgorithmError::InvalidGraph(_))));
    }

    #[test]
    fn test_dfs_rejects_out_of_range_target() {
        let storage = DfsStorageRuntime::new(mapped(0), vec![mapped(1000)], None, false, 1);
        let mut computation = DfsComputationRuntime::new(mapped(0), false, 1, 10);

        let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("DFS".to_string()));

        let result = storage.compute_dfs(&mut computation, None, &mut progress_tracker);

        assert!(matches!(result, Err(AlgorithmError::InvalidGraph(_))));
    }
}
