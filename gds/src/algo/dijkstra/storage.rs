//! Dijkstra Storage Runtime
//!
//! **Translation Source**: `org.neo4j.gds.paths.dijkstra.Dijkstra`
//!
//! This module implements the "Gross pole" of the Dijkstra algorithm,
//! handling persistent data access and the main algorithm orchestration.
//! This is the core of the Algorithmic Virtual Machine.

use super::spec::{DijkstraPathResult, DijkstraResult};
use super::targets::Targets;
use super::DijkstraComputationRuntime;
use crate::task::progress::{ProgressTracker, UNKNOWN_VOLUME};
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::graph::Graph;
use crate::types::graph::NodeId;
use std::time::Instant;

type HeuristicFunction = Box<dyn Fn(NodeId) -> f64 + Send + Sync>;

/// Dijkstra Storage Runtime
///
/// Translation of: Main `Dijkstra` class (lines 45-309)
/// Handles the persistent data access and algorithm orchestration
pub struct DijkstraStorageRuntime {
    /// Source node for shortest path computation
    pub source_node: NodeId,

    /// Whether to track relationship IDs
    pub track_relationships: bool,

    /// Concurrency level for parallel processing
    pub concurrency: usize,

    /// Whether to use heuristic function (for A* behavior)
    pub use_heuristic: bool,

    /// Relationship filter (Java: `RelationshipFilter`) applied during relaxation.
    ///
    /// Signature: `(source_node, target_node, relationship_id) -> bool`.
    ///
    /// Note: `relationship_id` follows Java GDS semantics here: index within the
    /// enumerated adjacency stream for the current source node.
    relationship_filter: Box<dyn Fn(NodeId, NodeId, NodeId) -> bool + Send + Sync>,

    /// Optional A* style heuristic used only to order the priority queue.
    heuristic_function: Option<HeuristicFunction>,
}

impl DijkstraStorageRuntime {
    /// Create a new Dijkstra storage runtime
    ///
    /// Translation of: Constructor (lines 118-140)
    pub fn new(
        source_node: NodeId,
        track_relationships: bool,
        concurrency: usize,
        use_heuristic: bool,
    ) -> Self {
        Self {
            source_node,
            track_relationships,
            concurrency,
            use_heuristic,
            relationship_filter: Box::new(|_, _, _| true),
            heuristic_function: None,
        }
    }

    /// Compose an additional relationship filter.
    ///
    /// Mirrors Java `Dijkstra.withRelationshipFilter(...)` which AND-combines filters.
    pub fn with_relationship_filter<F>(mut self, filter: F) -> Self
    where
        F: Fn(NodeId, NodeId, NodeId) -> bool + Send + Sync + 'static,
    {
        let previous = self.relationship_filter;
        self.relationship_filter = Box::new(move |source, target, relationship_id| {
            previous(source, target, relationship_id) && filter(source, target, relationship_id)
        });
        self
    }

    /// Set an optional heuristic function for Java A* parity.
    ///
    /// The function affects queue priority as `cost + heuristic(node)`; stored
    /// path costs remain the true Dijkstra distance from the source.
    pub fn with_heuristic_function<F>(mut self, heuristic_function: F) -> Self
    where
        F: Fn(NodeId) -> f64 + Send + Sync + 'static,
    {
        self.use_heuristic = true;
        self.heuristic_function = Some(Box::new(heuristic_function));
        self
    }

    /// Compute Dijkstra shortest paths
    ///
    /// Translation of: `compute()` method (lines 170-183)
    pub fn compute_dijkstra(
        &mut self,
        computation: &mut DijkstraComputationRuntime,
        mut targets: Box<dyn Targets>,
        graph: Option<&dyn Graph>,
        direction: u8,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<DijkstraResult, AlgorithmError> {
        let volume = graph
            .map(|g| g.relationship_count())
            .unwrap_or(UNKNOWN_VOLUME);
        if volume == UNKNOWN_VOLUME {
            progress_tracker.begin_subtask_unknown();
        } else {
            progress_tracker.begin_subtask_with_volume(volume);
        }

        let start_time = Instant::now();

        let result = (|| {
            // Initialize computation runtime
            // Bind to actual node count from a Graph view when available
            let node_count = graph.map(|g| g.node_count()).unwrap_or(100);
            Self::validate_node_in_graph(self.source_node, node_count, "source")?;
            computation.initialize(
                self.source_node,
                self.track_relationships,
                self.use_heuristic,
                node_count,
            );

            // Initialize priority queue with source node
            self.add_to_queue(computation, self.source_node, 0.0)?;
            let mut nodes_expanded = 0u64;
            let mut edges_considered = 0u64;
            let mut max_queue_size = computation.queue_size() as u64;

            let mut paths = Vec::new();
            let mut path_index = 0u64;

            // Main Dijkstra loop
            while !computation.is_queue_empty() {
                // Get node with minimum cost
                let (current_node, current_cost) = computation.pop_from_queue();

                if computation.is_visited(current_node)
                    || computation.is_stale_queue_entry(current_node, current_cost)
                {
                    continue;
                }

                computation.mark_visited(current_node);
                nodes_expanded += 1;

                // Relax all outgoing edges using graph-backed neighbor streaming when available
                edges_considered += self.relax_edges(
                    computation,
                    current_node,
                    current_cost,
                    graph,
                    direction,
                    node_count,
                    progress_tracker,
                )? as u64;
                max_queue_size = max_queue_size.max(computation.queue_size() as u64);

                // Java GDS applies target state after relaxing the current node.
                let traversal_state = targets.apply(current_node);

                if traversal_state.should_emit() {
                    let path = self.reconstruct_path(computation, current_node, path_index)?;
                    paths.push(path);
                    path_index += 1;

                    if traversal_state.should_stop() {
                        break;
                    }
                }
            }

            let computation_time_ms = start_time.elapsed().as_millis() as u64;

            // Create path finding result
            let path_finding_result = super::path_finding_result::PathFindingResult::new(paths);

            Ok(DijkstraResult {
                path_finding_result,
                computation_time_ms,
                nodes_expanded,
                edges_considered,
                max_queue_size,
            })
        })();

        match result {
            Ok(v) => {
                progress_tracker.end_subtask();
                Ok(v)
            }
            Err(e) => {
                progress_tracker.end_subtask_with_failure();
                Err(e)
            }
        }
    }

    /// Relax all outgoing edges from a node
    ///
    /// Translation of: `updateCost()` method (lines 220-241)
    fn relax_edges(
        &self,
        computation: &mut DijkstraComputationRuntime,
        source_node: NodeId,
        source_cost: f64,
        graph: Option<&dyn Graph>,
        direction: u8,
        node_count: usize,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<usize, AlgorithmError> {
        // Get neighbors with weights for the source node
        let neighbors = self.get_neighbors_with_weights(graph, source_node, direction)?;
        let neighbor_count = neighbors.len();

        // Work unit: edges scanned during relaxation.
        progress_tracker.log_progress(neighbor_count);

        for (target_node, weight, relationship_id) in neighbors {
            Self::validate_node_in_graph(target_node, node_count, "target")?;
            Self::validate_edge_weight(source_node, target_node, weight)?;

            // Skip if target is already visited
            if computation.is_visited(target_node) {
                continue;
            }

            // Java GDS: RelationshipFilter is applied during relaxation.
            if !(self.relationship_filter)(source_node, target_node, relationship_id) {
                continue;
            }

            let new_cost = source_cost + weight;
            if !new_cost.is_finite() {
                return Err(AlgorithmError::InvalidGraph(format!(
                    "Dijkstra path cost overflowed for edge {source_node}->{target_node}"
                )));
            }

            if !computation.is_in_queue(target_node) {
                // First time seeing this node
                self.add_to_queue(computation, target_node, new_cost)?;
                computation.set_predecessor(target_node, Some(source_node));
                if self.track_relationships {
                    computation.set_relationship_id(target_node, Some(relationship_id));
                }
            } else if new_cost < computation.get_cost(target_node) {
                // Found a shorter path
                self.update_queue_cost(computation, target_node, new_cost)?;
                computation.set_predecessor(target_node, Some(source_node));
                if self.track_relationships {
                    computation.set_relationship_id(target_node, Some(relationship_id));
                }
            }
        }

        Ok(neighbor_count)
    }

    fn add_to_queue(
        &self,
        computation: &mut DijkstraComputationRuntime,
        node_id: NodeId,
        cost: f64,
    ) -> Result<(), AlgorithmError> {
        let priority = self.queue_priority(node_id, cost)?;
        computation.add_to_queue_with_priority(node_id, cost, priority);
        Ok(())
    }

    fn update_queue_cost(
        &self,
        computation: &mut DijkstraComputationRuntime,
        node_id: NodeId,
        cost: f64,
    ) -> Result<(), AlgorithmError> {
        let priority = self.queue_priority(node_id, cost)?;
        computation.update_queue_cost_with_priority(node_id, cost, priority);
        Ok(())
    }

    fn queue_priority(&self, node_id: NodeId, cost: f64) -> Result<f64, AlgorithmError> {
        let heuristic = self
            .heuristic_function
            .as_ref()
            .map(|heuristic| heuristic(node_id))
            .unwrap_or(0.0);
        let priority = cost + heuristic;
        if !priority.is_finite() {
            return Err(AlgorithmError::InvalidGraph(format!(
                "Dijkstra queue priority overflowed for node {node_id}"
            )));
        }
        Ok(priority)
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

    fn validate_edge_weight(
        source_node: NodeId,
        target_node: NodeId,
        weight: f64,
    ) -> Result<(), AlgorithmError> {
        if !weight.is_finite() || weight < 0.0 {
            return Err(AlgorithmError::InvalidGraph(format!(
                "Dijkstra requires non-negative finite edge weights; edge {source_node}->{target_node} has weight {weight}"
            )));
        }
        Ok(())
    }

    /// Reconstruct a path from source to target
    ///
    /// Translation of: `pathResult()` method (lines 245-284)
    fn reconstruct_path(
        &self,
        computation: &DijkstraComputationRuntime,
        target_node: NodeId,
        path_index: u64,
    ) -> Result<DijkstraPathResult, AlgorithmError> {
        let mut node_ids = Vec::new();
        let mut relationship_ids = Vec::new();
        let mut costs = Vec::new();

        let mut current_node = target_node;

        // Backtrack from target to source
        while current_node != self.source_node {
            node_ids.push(current_node);
            costs.push(computation.get_cost(current_node));

            if self.track_relationships {
                relationship_ids.push(computation.get_relationship_id(current_node).unwrap_or(0));
            }

            current_node = computation
                .get_predecessor(current_node)
                .ok_or_else(|| AlgorithmError::InvalidGraph("Missing predecessor".to_string()))?;
        }

        // Add source node
        node_ids.push(self.source_node);
        costs.push(0.0);

        // Reverse to get correct order
        node_ids.reverse();
        costs.reverse();
        if self.track_relationships {
            relationship_ids.reverse();
        }

        Ok(DijkstraPathResult {
            index: path_index,
            source_node: self.source_node,
            target_node,
            node_ids,
            relationship_ids,
            costs,
        })
    }

    /// Get neighbors with weights for a given node
    ///
    /// This currently uses the `Graph` streaming APIs; GraphStore wiring is deferred.
    fn get_neighbors_with_weights(
        &self,
        graph: Option<&dyn Graph>,
        node_id: NodeId,
        direction: u8,
    ) -> Result<Vec<(NodeId, f64, NodeId)>, AlgorithmError> {
        if let Some(g) = graph {
            let fallback = g.default_property_value();
            let mapped: NodeId = node_id;
            let iter = if direction == 1 {
                // 1 = incoming
                g.stream_inverse_relationships_weighted(mapped, fallback)
            } else {
                g.stream_relationships_weighted(mapped, fallback)
            };

            let mut out = Vec::new();
            for (idx, cursor) in iter.enumerate() {
                let relationship_id = idx as NodeId;
                let target_node = cursor.target_id();
                out.push((target_node, cursor.weight(), relationship_id));
            }
            return Ok(out);
        }

        // Deterministic mock when no Graph is provided
        let neighbors: Vec<(NodeId, f64)> = match node_id {
            0 => vec![(1, 1.0), (2, 4.0)],
            1 => vec![(2, 2.0), (3, 5.0)],
            2 => vec![(3, 1.0), (4, 3.0)],
            3 => vec![(4, 2.0)],
            _ => vec![],
        };

        Ok(neighbors
            .into_iter()
            .enumerate()
            .map(|(idx, (target, weight))| (target, weight, idx as NodeId))
            .collect())
    }

    /// Best-effort node count hint from a bound GraphStore once integrated.
    #[allow(dead_code)]
    fn graph_node_count_hint(&self) -> Option<usize> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::super::targets::{AllTargets, ManyTargets, SingleTarget};
    use super::super::DijkstraComputationRuntime;
    use super::*;
    use crate::task::progress::{TaskProgressTracker, Tasks};

    #[test]
    fn test_dijkstra_storage_runtime_creation() {
        let storage = DijkstraStorageRuntime::new(0, true, 4, false);
        assert_eq!(storage.source_node, 0);
        assert!(storage.track_relationships);
        assert_eq!(storage.concurrency, 4);
        assert!(!storage.use_heuristic);
    }

    #[test]
    fn test_dijkstra_path_computation_single_target() {
        let mut storage = DijkstraStorageRuntime::new(0, false, 4, false);
        let mut computation = DijkstraComputationRuntime::new(0, false, 4, false);
        let targets = Box::new(SingleTarget::new(3));
        let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("dijkstra".to_string()));

        // Test basic path computation
        let result =
            storage.compute_dijkstra(&mut computation, targets, None, 0, &mut progress_tracker);
        assert!(result.is_ok());

        let _ = result.unwrap();
    }

    #[test]
    fn test_dijkstra_path_computation_many_targets() {
        let mut storage = DijkstraStorageRuntime::new(0, false, 4, false);
        let mut computation = DijkstraComputationRuntime::new(0, false, 4, false);
        let targets = Box::new(ManyTargets::new(vec![3, 5]));
        let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("dijkstra".to_string()));

        // Test with multiple targets
        let result =
            storage.compute_dijkstra(&mut computation, targets, None, 0, &mut progress_tracker);
        assert!(result.is_ok());

        let _ = result.unwrap();
    }

    #[test]
    fn test_dijkstra_path_computation_all_targets() {
        let mut storage = DijkstraStorageRuntime::new(0, false, 4, false);
        let mut computation = DijkstraComputationRuntime::new(0, false, 4, false);
        let targets = Box::new(AllTargets::new());
        let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("dijkstra".to_string()));

        // Test with all targets
        let result =
            storage.compute_dijkstra(&mut computation, targets, None, 0, &mut progress_tracker);
        assert!(result.is_ok());

        let _ = result.unwrap();
    }

    #[test]
    fn test_neighbors_with_weights() {
        let storage = DijkstraStorageRuntime::new(0, false, 4, false);

        let neighbors = storage.get_neighbors_with_weights(None, 0, 0).unwrap();
        assert_eq!(neighbors.len(), 2);
        assert_eq!(neighbors[0], (1, 1.0, 0));
        assert_eq!(neighbors[1], (2, 4.0, 1));

        let neighbors_empty = storage.get_neighbors_with_weights(None, 99, 0).unwrap();
        assert!(neighbors_empty.is_empty());
    }

    #[test]
    fn relationship_filter_blocks_by_relationship_id_and_tracking_records_ids() {
        // Block the first adjacency entry for source=0 (relationship_id=0), forcing
        // the path 0->2->3 instead of 0->1->2->3 on the deterministic mock graph.
        let mut storage = DijkstraStorageRuntime::new(0, true, 4, false).with_relationship_filter(
            |source, _target, relationship_id| !(source == 0 && relationship_id == 0),
        );

        let mut computation = DijkstraComputationRuntime::new(0, true, 4, false);
        let targets = Box::new(SingleTarget::new(3));
        let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("dijkstra".to_string()));

        let result = storage
            .compute_dijkstra(&mut computation, targets, None, 0, &mut progress_tracker)
            .unwrap();
        let mut paths = result.path_finding_result.clone();
        let first = paths.find_first().expect("expected at least one path");

        assert_eq!(first.node_ids, vec![0, 2, 3]);
        // relationship_id semantics here: index within enumerated adjacency per source node.
        // - for 0->2 on node 0, that's index 1
        // - for 2->3 on node 2, that's index 0
        assert_eq!(first.relationship_ids, vec![1, 0]);
    }

    #[test]
    fn stale_queue_entries_do_not_emit_or_lock_in_old_paths() {
        let mut storage = DijkstraStorageRuntime::new(0, false, 4, false);
        let mut computation = DijkstraComputationRuntime::new(0, false, 4, false);
        let targets = Box::new(SingleTarget::new(3));
        let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("dijkstra".to_string()));

        let result = storage
            .compute_dijkstra(&mut computation, targets, None, 0, &mut progress_tracker)
            .unwrap();
        let mut paths = result.path_finding_result.clone();
        let first = paths.find_first().expect("expected target path");

        assert_eq!(first.node_ids, vec![0, 1, 2, 3]);
        assert_eq!(first.costs, vec![0.0, 1.0, 3.0, 4.0]);
    }

    #[test]
    fn heuristic_function_orders_queue_without_changing_path_costs() {
        let mut storage = DijkstraStorageRuntime::new(0, false, 4, true)
            .with_heuristic_function(|node| if node == 1 { 100.0 } else { 0.0 });
        let mut computation = DijkstraComputationRuntime::new(0, false, 4, true);
        let targets = Box::new(AllTargets::new());
        let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("dijkstra".to_string()));

        let result = storage
            .compute_dijkstra(&mut computation, targets, None, 0, &mut progress_tracker)
            .unwrap();
        let paths: Vec<_> = result.path_finding_result.paths().cloned().collect();

        assert_eq!(paths[0].target_node, 0);
        assert_eq!(paths[1].target_node, 2);
        assert_eq!(paths[1].costs.last().copied(), Some(4.0));
    }

    #[test]
    fn rejects_source_node_outside_graph_hint() {
        let mut storage = DijkstraStorageRuntime::new(101, false, 4, false);
        let mut computation = DijkstraComputationRuntime::new(101, false, 4, false);
        let targets = Box::new(AllTargets::new());
        let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("dijkstra".to_string()));

        let result =
            storage.compute_dijkstra(&mut computation, targets, None, 0, &mut progress_tracker);

        assert!(matches!(result, Err(AlgorithmError::InvalidGraph(_))));
    }

    #[test]
    fn rejects_negative_or_non_finite_weights() {
        assert!(DijkstraStorageRuntime::validate_edge_weight(0, 1, -1.0).is_err());
        assert!(DijkstraStorageRuntime::validate_edge_weight(0, 1, f64::NAN).is_err());
        assert!(DijkstraStorageRuntime::validate_edge_weight(0, 1, f64::INFINITY).is_err());
        assert!(DijkstraStorageRuntime::validate_edge_weight(0, 1, 0.0).is_ok());
    }
}
