//! **Yen's Storage Runtime**
//!
//! **Translation Source**: `org.neo4j.gds.paths.yens.Yens`
//!
//! This module implements the "Gross pole" for Yen's algorithm - persistent data access
//! and algorithm orchestration.

use super::candidate_queue::CandidatePathsPriorityQueue;
use super::mutable_path_result::MutablePathResult;
use super::spec::{YensPathResult, YensResult};
use super::YensComputationRuntime;
use crate::algo::dijkstra::targets::create_targets;
use crate::algo::dijkstra::{DijkstraComputationRuntime, DijkstraStorageRuntime};
use crate::core::utils::progress::{ProgressTracker, TaskProgressTracker, Tasks, UNKNOWN_VOLUME};
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::graph::Graph;
use crate::types::graph::NodeId;
use std::collections::HashSet;

/// Yen's Storage Runtime - handles persistent data access and algorithm orchestration
///
/// Translation of: `Yens.java` (lines 40-182)
/// This implements the "Gross pole" for accessing graph data
pub struct YensStorageRuntime {
    /// Source node for path finding
    pub source_node: NodeId,
    /// Target node for path finding
    pub target_node: NodeId,
    /// Number of shortest paths to find (K)
    pub k: usize,
    /// Whether to track relationships
    pub track_relationships: bool,
    /// Concurrency level
    pub concurrency: usize,
}

impl YensStorageRuntime {
    /// Create new Yen's storage runtime
    pub fn new(
        source_node: NodeId,
        target_node: NodeId,
        k: usize,
        track_relationships: bool,
        concurrency: usize,
    ) -> Self {
        Self {
            source_node,
            target_node,
            k,
            track_relationships,
            concurrency,
        }
    }

    /// Compute Yen's K-shortest paths
    ///
    /// Translation of: `Yens.compute()` (lines 82-129)
    /// This orchestrates the main Yen's algorithm loop
    pub fn compute_yens(
        &self,
        computation: &mut YensComputationRuntime,
        graph: Option<&dyn Graph>,
        direction: u8,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<YensResult, AlgorithmError> {
        let start_time = std::time::Instant::now();

        progress_tracker.begin_subtask_with_volume(self.k);

        let result = (|| {
            let node_count = graph.map(|g| g.node_count()).unwrap_or(100);
            self.validate(node_count)?;

            // Initialize computation runtime
            computation.initialize(
                self.source_node,
                self.target_node,
                self.k,
                self.track_relationships,
            );

            // Find first shortest path using Dijkstra
            let first_path = self.find_first_path(graph, direction)?;
            if first_path.is_none() {
                return Ok(YensResult {
                    paths: Vec::new(),
                    path_count: 0,
                    computation_time_ms: start_time.elapsed().as_millis() as u64,
                });
            }

            let mut k_shortest_paths = vec![first_path.unwrap()];
            progress_tracker.log_progress(1);

            let candidate_queue = CandidatePathsPriorityQueue::new();

            // Main Yen's algorithm loop
            for solution_index in 1..self.k {
                if let Some(prev_path) = k_shortest_paths.get(solution_index - 1) {
                    // Generate candidate paths from previous path
                    let candidates =
                        self.generate_candidates(prev_path, &k_shortest_paths, graph, direction)?;

                    for candidate in candidates {
                        candidate_queue.add_path(candidate);
                    }
                }

                if candidate_queue.is_empty() {
                    break;
                }

                // Add best candidate to results
                if let Some(best_candidate) = candidate_queue.pop() {
                    k_shortest_paths.push(best_candidate);
                    // 1 unit of progress per path found.
                    progress_tracker.log_progress(1);
                }
            }

            let computation_time = start_time.elapsed().as_millis() as u64;

            // Convert to result format
            let paths: Vec<YensPathResult> = k_shortest_paths
                .into_iter()
                .enumerate()
                .map(|(index, path)| {
                    let total_cost = path.total_cost();
                    YensPathResult {
                        index: index as u32,
                        source_node: path.source_node,
                        target_node: path.target_node,
                        node_ids: path.node_ids,
                        relationship_ids: path.relationship_ids,
                        costs: path.costs,
                        total_cost,
                    }
                })
                .collect();

            Ok(YensResult {
                path_count: paths.len(),
                paths,
                computation_time_ms: computation_time,
            })
        })();

        match result {
            Ok(value) => {
                progress_tracker.end_subtask();
                Ok(value)
            }
            Err(e) => {
                progress_tracker.end_subtask_with_failure();
                Err(e)
            }
        }
    }

    fn validate(&self, node_count: usize) -> Result<(), AlgorithmError> {
        if self.k == 0 {
            return Err(AlgorithmError::InvalidGraph(
                "Yen's algorithm requires k > 0".to_string(),
            ));
        }
        if self.concurrency == 0 {
            return Err(AlgorithmError::InvalidGraph(
                "Yen's algorithm requires concurrency > 0".to_string(),
            ));
        }
        if self.source_node == self.target_node {
            return Err(AlgorithmError::InvalidGraph(
                "Yen's algorithm requires distinct source and target nodes".to_string(),
            ));
        }
        Self::validate_node_in_graph(self.source_node, node_count, "source")?;
        Self::validate_node_in_graph(self.target_node, node_count, "target")?;
        Ok(())
    }

    fn validate_node_in_graph(
        node_id: NodeId,
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

    /// Find the first shortest path using Dijkstra
    fn find_first_path(
        &self,
        graph: Option<&dyn Graph>,
        direction: u8,
    ) -> Result<Option<MutablePathResult>, AlgorithmError> {
        self.shortest_path(self.source_node, self.target_node, graph, direction, None)
    }

    #[allow(clippy::type_complexity)]
    fn shortest_path(
        &self,
        source: NodeId,
        target: NodeId,
        graph: Option<&dyn Graph>,
        direction: u8,
        extra_filter: Option<Box<dyn Fn(NodeId, NodeId, NodeId) -> bool + Send + Sync>>,
    ) -> Result<Option<MutablePathResult>, AlgorithmError> {
        let targets = create_targets(vec![target]);

        let mut storage =
            DijkstraStorageRuntime::new(source, self.track_relationships, self.concurrency, false);

        if let Some(filter) = extra_filter {
            storage = storage.with_relationship_filter(filter);
        }

        let mut computation = DijkstraComputationRuntime::new(
            source,
            self.track_relationships,
            self.concurrency,
            false,
        );

        let volume = graph
            .map(|g| g.relationship_count())
            .unwrap_or(UNKNOWN_VOLUME);
        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("dijkstra".to_string(), volume),
            self.concurrency,
        );

        let result = storage.compute_dijkstra(
            &mut computation,
            targets,
            graph,
            direction,
            &mut progress_tracker,
        )?;
        let first = result.path_finding_result.paths().next().cloned();

        let Some(path) = first else {
            return Ok(None);
        };

        let relationship_ids = if self.track_relationships {
            path.relationship_ids
        } else {
            Vec::new()
        };

        Ok(Some(MutablePathResult::new(
            0,
            path.source_node,
            path.target_node,
            path.node_ids,
            relationship_ids,
            path.costs,
        )))
    }

    /// Generate candidate paths from a previous path
    fn generate_candidates(
        &self,
        prev_path: &MutablePathResult,
        k_shortest_paths: &[MutablePathResult],
        graph: Option<&dyn Graph>,
        direction: u8,
    ) -> Result<Vec<MutablePathResult>, AlgorithmError> {
        let mut candidates = Vec::new();

        if prev_path.node_ids.len() < 2 {
            return Ok(candidates);
        }

        // Standard Yen plus Lawler's modification: selected candidate paths carry
        // the spur index where their next expansion may begin.
        let start_spur_index = prev_path.index as usize;
        for spur_index in start_spur_index..(prev_path.node_ids.len() - 1) {
            let spur_node = prev_path.node_ids[spur_index];

            // Root path = prefix up to and including the spur node.
            let mut root_path = prev_path.sub_path(spur_index + 1);

            // Remove nodes in the root path (except spur node) to prevent loops.
            let removed_nodes: HashSet<NodeId> = root_path
                .node_ids
                .iter()
                .copied()
                .take(root_path.node_ids.len().saturating_sub(1))
                .collect();

            // Remove edges that would recreate previously-found paths with the same root.
            let mut blocked_identifiers: HashSet<NodeId> = HashSet::new();
            for p in k_shortest_paths {
                if p.matches_exactly(prev_path, spur_index + 1) {
                    let blocked = if self.track_relationships {
                        p.relationship_ids.get(spur_index).copied()
                    } else {
                        p.node_ids.get(spur_index + 1).copied()
                    };
                    if let Some(blocked) = blocked {
                        blocked_identifiers.insert(blocked);
                    }
                }
            }

            let filter_spur = spur_node;
            let filter_removed_nodes = removed_nodes.clone();
            let filter_blocked = blocked_identifiers.clone();
            let track_relationships = self.track_relationships;

            let filter: Box<dyn Fn(NodeId, NodeId, NodeId) -> bool + Send + Sync> =
                Box::new(move |source, target, relationship_id| {
                    if filter_removed_nodes.contains(&source)
                        || filter_removed_nodes.contains(&target)
                    {
                        return false;
                    }

                    let blocked_identifier = if track_relationships {
                        relationship_id
                    } else {
                        target
                    };
                    if source == filter_spur && filter_blocked.contains(&blocked_identifier) {
                        return false;
                    }

                    true
                });

            let spur_path =
                self.shortest_path(spur_node, self.target_node, graph, direction, Some(filter))?;
            let Some(spur_path) = spur_path else {
                continue;
            };

            if self.track_relationships {
                root_path.append(&spur_path);
            } else {
                root_path.append_without_relationship_ids(&spur_path);
            }

            root_path.index = spur_index as u32;

            // Ensure the path is still source->target.
            root_path.source_node = self.source_node;
            root_path.target_node = self.target_node;

            candidates.push(root_path);
        }

        Ok(candidates)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::utils::progress::TaskProgressTracker;
    use crate::types::prelude::DefaultGraphStore;
    use crate::types::random::{RandomGraphConfig, RandomRelationshipConfig};
    use std::sync::Arc;

    #[test]
    fn test_yens_storage_runtime_creation() {
        let storage = YensStorageRuntime::new(0, 3, 5, true, 4);
        assert_eq!(storage.source_node, 0);
        assert_eq!(storage.target_node, 3);
        assert_eq!(storage.k, 5);
        assert!(storage.track_relationships);
        assert_eq!(storage.concurrency, 4);
    }

    #[test]
    fn test_yens_path_computation() {
        let config = RandomGraphConfig {
            seed: Some(42),
            node_count: 12,
            relationships: vec![RandomRelationshipConfig::new("REL", 0.8)],
            ..RandomGraphConfig::default()
        };

        let store = Arc::new(DefaultGraphStore::random(&config).unwrap());
        let graph = store.graph();

        let storage = YensStorageRuntime::new(0, 5, 3, true, 1);
        let mut computation = YensComputationRuntime::new(0, 5, 3, true, 1);

        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("yens".to_string(), 3),
            1,
        );

        let result = storage
            .compute_yens(
                &mut computation,
                Some(graph.as_ref()),
                0,
                &mut progress_tracker,
            )
            .unwrap();

        assert!(result.path_count <= 3);

        // Paths should be unique.
        let unique: std::collections::HashSet<Vec<NodeId>> =
            result.paths.iter().map(|p| p.node_ids.clone()).collect();
        assert_eq!(unique.len(), result.paths.len());
    }

    #[test]
    fn computes_ordered_mock_k_shortest_paths() {
        let storage = YensStorageRuntime::new(0, 3, 3, false, 1);
        let mut computation = YensComputationRuntime::new(0, 3, 3, false, 1);

        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("yens".to_string(), 3),
            1,
        );

        let result = storage
            .compute_yens(&mut computation, None, 0, &mut progress_tracker)
            .unwrap();

        assert_eq!(result.path_count, 3);
        assert_eq!(result.paths[0].node_ids, vec![0, 1, 2, 3]);
        assert_eq!(result.paths[0].total_cost, 4.0);
        assert_eq!(result.paths[1].node_ids, vec![0, 2, 3]);
        assert_eq!(result.paths[1].total_cost, 5.0);
        assert_eq!(result.paths[2].node_ids, vec![0, 1, 3]);
        assert_eq!(result.paths[2].total_cost, 6.0);
    }

    #[test]
    fn rejects_invalid_storage_contracts() {
        assert!(matches!(
            YensStorageRuntime::new(0, 3, 0, false, 1).validate(100),
            Err(AlgorithmError::InvalidGraph(_))
        ));
        assert!(matches!(
            YensStorageRuntime::new(0, 3, 3, false, 0).validate(100),
            Err(AlgorithmError::InvalidGraph(_))
        ));
        assert!(matches!(
            YensStorageRuntime::new(3, 3, 3, false, 1).validate(100),
            Err(AlgorithmError::InvalidGraph(_))
        ));
        assert!(matches!(
            YensStorageRuntime::new(100, 3, 3, false, 1).validate(100),
            Err(AlgorithmError::InvalidGraph(_))
        ));
        assert!(matches!(
            YensStorageRuntime::new(0, 100, 3, false, 1).validate(100),
            Err(AlgorithmError::InvalidGraph(_))
        ));
    }

    #[test]
    fn lawler_spur_index_skips_earlier_candidates() {
        let storage = YensStorageRuntime::new(0, 3, 3, false, 1);
        let previous =
            MutablePathResult::new(1, 0, 3, vec![0, 1, 2, 3], vec![], vec![0.0, 1.0, 3.0, 4.0]);

        let candidates = storage
            .generate_candidates(&previous, &[previous.clone()], None, 0)
            .unwrap();

        let candidate_nodes: std::collections::HashSet<Vec<NodeId>> =
            candidates.into_iter().map(|path| path.node_ids).collect();

        assert!(!candidate_nodes.contains(&vec![0, 2, 3]));
        assert!(candidate_nodes.contains(&vec![0, 1, 3]));
    }
}
