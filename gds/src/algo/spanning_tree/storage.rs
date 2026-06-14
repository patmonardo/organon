//! Spanning Tree Storage Runtime - Graph Data Access and Algorithm Orchestration
//!
//! **Translation Source**: `org.neo4j.gds.spanningtree.Prim` (algorithm orchestration)
//!
//! This module implements the "Gross pole" for spanning tree algorithms,
//! handling persistent data access and orchestrating the Prim's algorithm execution.

use super::{SpanningTree, SpanningTreeComputationRuntime};
use crate::task::concurrency::TerminationFlag;
use crate::task::progress::ProgressTracker;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::graph::Graph;
use crate::types::graph::NodeId;

/// Spanning Tree Storage Runtime
///
/// **Translation Source**: `org.neo4j.gds.spanningtree.Prim`
///
/// Implements the "Gross pole" for spanning tree algorithms, handling:
/// - Graph data access and neighbor iteration
/// - Algorithm orchestration and execution
/// - Result construction and validation
pub struct SpanningTreeStorageRuntime {
    /// Start node for the spanning tree
    pub start_node_id: u32,

    /// Whether to compute minimum (true) or maximum (false) spanning tree
    pub compute_minimum: bool,

    /// Concurrency level
    pub concurrency: usize,
}

impl SpanningTreeStorageRuntime {
    /// Creates a new SpanningTreeStorageRuntime.
    ///
    /// # Arguments
    ///
    /// * `start_node_id` - Starting node for the spanning tree
    /// * `compute_minimum` - Whether to compute minimum (true) or maximum (false) spanning tree
    /// * `concurrency` - Concurrency level
    ///
    /// # Returns
    ///
    /// A new `SpanningTreeStorageRuntime` instance.
    pub fn new(start_node_id: u32, compute_minimum: bool, concurrency: usize) -> Self {
        Self {
            start_node_id,
            compute_minimum,
            concurrency,
        }
    }

    /// Compute the spanning tree using Prim's algorithm.
    ///
    /// **Translation Source**: `org.neo4j.gds.spanningtree.Prim.compute()`
    ///
    /// This method orchestrates the Prim's algorithm, handling graph access and progress tracking,
    /// while delegating all state management to the computation runtime.
    ///
    /// # Arguments
    ///
    /// * `computation` - Mutable reference to the computation runtime for state management
    /// * `graph` - Optional graph interface for neighbor access
    /// * `direction` - Traversal direction (0=outgoing, 1=incoming, 2=undirected)
    /// * `progress_tracker` - Progress tracking interface
    ///
    /// # Returns
    ///
    /// A `Result` containing the `SpanningTree` or an error.
    pub fn compute_spanning_tree(
        &self,
        computation: &mut SpanningTreeComputationRuntime,
        graph: Option<&dyn Graph>,
        direction: u8,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<SpanningTree, AlgorithmError> {
        self.compute_spanning_tree_with_termination(
            computation,
            graph,
            direction,
            progress_tracker,
            &TerminationFlag::running_true(),
        )
    }

    /// Compute the spanning tree using Prim's algorithm with request termination support.
    pub fn compute_spanning_tree_with_termination(
        &self,
        computation: &mut SpanningTreeComputationRuntime,
        graph: Option<&dyn Graph>,
        direction: u8,
        progress_tracker: &mut dyn ProgressTracker,
        termination: &TerminationFlag,
    ) -> Result<SpanningTree, AlgorithmError> {
        let node_count = graph.map(|g| g.node_count() as u32).unwrap_or(0);
        let progress_volume = graph
            .map(|g| g.relationship_count())
            .unwrap_or(node_count as usize);

        progress_tracker.begin_subtask_with_volume(progress_volume);

        let result = (|| {
            // Handle empty graph upfront
            if node_count == 0 {
                return Ok(computation.build_result(0));
            }

            self.validate(node_count)?;

            // Initialize computation runtime
            computation.initialize(self.start_node_id);

            // Main Prim's algorithm loop
            while !computation.is_queue_empty() {
                if !termination.running() {
                    return Err(AlgorithmError::Execution(
                        "Spanning tree computation terminated".to_string(),
                    ));
                }

                // Get next node from priority queue
                let (current_node, current_cost) = match computation.pop_from_queue() {
                    Some((node, cost)) => (node, cost),
                    None => break,
                };

                // Skip if already visited
                if computation.is_visited(current_node) {
                    continue;
                }

                // Mark as visited and update progress
                computation.mark_visited(current_node, current_cost);

                // Process neighbors via graph interface
                if let Some(graph) = graph {
                    let neighbors =
                        self.get_neighbors_from_graph(graph, current_node, direction)?;
                    progress_tracker.log_progress(neighbors.len());
                    for (neighbor, weight) in neighbors {
                        Self::validate_node_in_graph(neighbor, node_count, "neighbor")?;
                        Self::validate_edge_weight(current_node, neighbor, weight)?;

                        // Skip if neighbor already visited
                        if computation.is_visited(neighbor) {
                            continue;
                        }

                        // Transform weight for min/max spanning tree
                        let transformed_weight = computation.transform_weight(weight);

                        // Check if neighbor is already in queue
                        let current_parent = computation.parent(neighbor);
                        let current_cost_to_parent = computation.cost_to_parent(neighbor);

                        if current_parent == -1 {
                            // Neighbor not in queue, add it
                            computation.add_to_queue(neighbor, transformed_weight, current_node);
                        } else if transformed_weight < current_cost_to_parent {
                            // Better path found, update
                            computation.update_cost(neighbor, transformed_weight, current_node);
                        }
                    }
                }
            }

            // Build and return result
            Ok(computation.build_result(node_count))
        })();

        match result {
            Ok(result) => {
                progress_tracker.end_subtask();
                Ok(result)
            }
            Err(e) => {
                progress_tracker.end_subtask_with_failure();
                Err(e)
            }
        }
    }

    /// Compute the spanning tree using a bound Graph (neighbor streaming via relationship cursors).
    pub fn compute_spanning_tree_with_graph(
        &self,
        graph: &dyn Graph,
        direction: u8,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<SpanningTree, AlgorithmError> {
        self.compute_spanning_tree_with_graph_and_termination(
            graph,
            direction,
            progress_tracker,
            &TerminationFlag::running_true(),
        )
    }

    /// Compute the spanning tree using a bound Graph and request termination support.
    pub fn compute_spanning_tree_with_graph_and_termination(
        &self,
        graph: &dyn Graph,
        direction: u8,
        progress_tracker: &mut dyn ProgressTracker,
        termination: &TerminationFlag,
    ) -> Result<SpanningTree, AlgorithmError> {
        let node_count = graph.node_count() as u32;

        // Create computation runtime for this operation
        let mut computation = SpanningTreeComputationRuntime::new(
            self.start_node_id,
            self.compute_minimum,
            node_count,
            self.concurrency,
        );

        self.compute_spanning_tree_with_termination(
            &mut computation,
            Some(graph),
            direction,
            progress_tracker,
            termination,
        )
    }

    /// Neighbor retrieval backed by Graph::stream_relationships (outgoing edges), with numeric fallback.
    fn get_neighbors_from_graph(
        &self,
        graph: &dyn Graph,
        node_id: u32,
        direction: u8,
    ) -> Result<Vec<(u32, f64)>, AlgorithmError> {
        let fallback: f64 = 1.0;
        let mut out = Vec::new();

        let mut push_cursor = |target_id: NodeId, weight: f64| -> Result<(), AlgorithmError> {
            let target = u32::try_from(target_id).map_err(|_| {
                AlgorithmError::InvalidGraph(format!("Invalid neighbor node id: {target_id}"))
            })?;
            out.push((target, weight));
            Ok(())
        };

        match direction {
            1 => {
                for cursor in graph.stream_inverse_relationships_weighted(node_id as i64, fallback)
                {
                    push_cursor(cursor.target_id(), cursor.weight())?;
                }
            }
            2 => {
                for cursor in graph.stream_relationships_weighted(node_id as i64, fallback) {
                    push_cursor(cursor.target_id(), cursor.weight())?;
                }
                for cursor in graph.stream_inverse_relationships_weighted(node_id as i64, fallback)
                {
                    push_cursor(cursor.target_id(), cursor.weight())?;
                }
            }
            _ => {
                for cursor in graph.stream_relationships_weighted(node_id as i64, fallback) {
                    push_cursor(cursor.target_id(), cursor.weight())?;
                }
            }
        }

        Ok(out)
    }

    /// Get neighbors of a node (mock implementation for testing).
    ///
    /// # Arguments
    ///
    /// * `node_id` - Node ID to get neighbors for
    ///
    /// # Returns
    ///
    /// A vector of (neighbor_id, weight) pairs.
    pub fn get_neighbors(&self, node_id: u32) -> Vec<(u32, f64)> {
        // Mock implementation for testing
        // In a real implementation, this would access the graph store
        match node_id {
            0 => vec![(1, 1.0), (2, 2.0), (3, 1.5)],
            1 => vec![(0, 1.0), (2, 1.5), (3, 2.5)],
            2 => vec![(0, 2.0), (1, 1.5), (3, 1.0)],
            3 => vec![(0, 1.5), (1, 2.5), (2, 1.0)],
            _ => vec![],
        }
    }

    /// Compute spanning tree with mock graph data.
    ///
    /// # Arguments
    ///
    /// * `node_count` - Total number of nodes in the graph
    ///
    /// # Returns
    ///
    /// A `Result` containing the `SpanningTree` or an error.
    /// Compute spanning tree with mock graph data (for testing).
    ///
    /// # Arguments
    ///
    /// * `node_count` - Total number of nodes in the graph
    ///
    /// # Returns
    ///
    /// A `Result` containing the `SpanningTree` or an error.
    pub fn compute_spanning_tree_mock(
        &self,
        node_count: u32,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<SpanningTree, AlgorithmError> {
        progress_tracker.begin_subtask_with_volume(node_count as usize);

        let result = (|| {
            if node_count == 0 {
                return Ok(SpanningTree::new(
                    self.start_node_id,
                    0,
                    0,
                    Vec::new(),
                    Vec::new(),
                    0.0,
                ));
            }

            self.validate(node_count)?;

            let mut computation = SpanningTreeComputationRuntime::new(
                self.start_node_id,
                self.compute_minimum,
                node_count,
                self.concurrency,
            );
            computation.initialize(self.start_node_id);

            while !computation.is_queue_empty() {
                let (current_node, current_cost) = match computation.pop_from_queue() {
                    Some(entry) => entry,
                    None => break,
                };

                if computation.is_visited(current_node) {
                    continue;
                }

                computation.mark_visited(current_node, current_cost);
                progress_tracker.log_progress(1);

                for (neighbor, weight) in self.get_neighbors(current_node) {
                    if neighbor >= node_count {
                        continue;
                    }
                    Self::validate_node_in_graph(neighbor, node_count, "neighbor")?;
                    Self::validate_edge_weight(current_node, neighbor, weight)?;

                    if computation.is_visited(neighbor) {
                        continue;
                    }

                    let transformed_weight = computation.transform_weight(weight);
                    let current_parent = computation.parent(neighbor);
                    let current_cost_to_parent = computation.cost_to_parent(neighbor);

                    if current_parent == -1 {
                        computation.add_to_queue(neighbor, transformed_weight, current_node);
                    } else if transformed_weight < current_cost_to_parent {
                        computation.update_cost(neighbor, transformed_weight, current_node);
                    }
                }
            }

            Ok(computation.build_result(node_count))
        })();

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

    fn validate(&self, node_count: u32) -> Result<(), AlgorithmError> {
        if self.concurrency == 0 {
            return Err(AlgorithmError::InvalidGraph(
                "Spanning tree requires concurrency > 0".to_string(),
            ));
        }
        Self::validate_node_in_graph(self.start_node_id, node_count, "start")
    }

    fn validate_node_in_graph(
        node_id: u32,
        node_count: u32,
        role: &str,
    ) -> Result<(), AlgorithmError> {
        if node_id >= node_count {
            return Err(AlgorithmError::InvalidGraph(format!(
                "{role} node id out of range: {node_id} (node_count={node_count})"
            )));
        }
        Ok(())
    }

    fn validate_edge_weight(
        source_node: u32,
        target_node: u32,
        weight: f64,
    ) -> Result<(), AlgorithmError> {
        if !weight.is_finite() {
            return Err(AlgorithmError::InvalidGraph(format!(
                "Spanning tree requires finite edge weights; edge {source_node}->{target_node} has weight {weight}"
            )));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::progress::{TaskProgressTracker, Tasks};

    #[test]
    fn test_storage_runtime_creation() {
        let runtime = SpanningTreeStorageRuntime::new(0, true, 1);

        assert_eq!(runtime.start_node_id, 0);
        assert!(runtime.compute_minimum);
        assert_eq!(runtime.concurrency, 1);
    }

    #[test]
    fn test_storage_runtime_minimum_spanning_tree() {
        let runtime = SpanningTreeStorageRuntime::new(0, true, 1);
        let mut progress_tracker =
            TaskProgressTracker::new(Tasks::leaf_with_volume("spanning_tree".to_string(), 0));
        let result = runtime
            .compute_spanning_tree_mock(4, &mut progress_tracker)
            .unwrap();

        // Verify basic properties
        assert_eq!(result.head(0), 0);
        assert_eq!(result.effective_node_count(), 4);
        assert_eq!(result.total_weight(), 3.5);

        // Verify tree structure (all nodes should be connected)
        assert_eq!(result.parent(0), -1); // Root has no parent
        assert!(result.parent(1) != -1); // Other nodes have parents
        assert!(result.parent(2) != -1);
        assert!(result.parent(3) != -1);
    }

    #[test]
    fn test_storage_runtime_maximum_spanning_tree() {
        let runtime = SpanningTreeStorageRuntime::new(0, false, 1);
        let mut progress_tracker =
            TaskProgressTracker::new(Tasks::leaf_with_volume("spanning_tree".to_string(), 0));
        let result = runtime
            .compute_spanning_tree_mock(4, &mut progress_tracker)
            .unwrap();

        // Verify basic properties
        assert_eq!(result.head(0), 0);
        assert_eq!(result.effective_node_count(), 4);
        assert_eq!(result.total_weight(), 6.0);

        // Verify tree structure
        assert_eq!(result.parent(0), -1); // Root has no parent
        assert!(result.parent(1) != -1); // Other nodes have parents
        assert!(result.parent(2) != -1);
        assert!(result.parent(3) != -1);
    }

    #[test]
    fn test_storage_runtime_different_start_nodes() {
        let runtime1 = SpanningTreeStorageRuntime::new(0, true, 1);
        let runtime2 = SpanningTreeStorageRuntime::new(1, true, 1);

        let mut progress_tracker1 =
            TaskProgressTracker::new(Tasks::leaf_with_volume("spanning_tree".to_string(), 0));
        let mut progress_tracker2 =
            TaskProgressTracker::new(Tasks::leaf_with_volume("spanning_tree".to_string(), 0));
        let result1 = runtime1
            .compute_spanning_tree_mock(4, &mut progress_tracker1)
            .unwrap();
        let result2 = runtime2
            .compute_spanning_tree_mock(4, &mut progress_tracker2)
            .unwrap();

        // Both should produce valid spanning trees
        assert_eq!(result1.effective_node_count(), 4);
        assert_eq!(result2.effective_node_count(), 4);

        // Different start nodes should produce different trees
        assert_eq!(result1.head(0), 0);
        assert_eq!(result2.head(1), 1);
    }

    #[test]
    fn test_storage_runtime_edge_iteration() {
        let runtime = SpanningTreeStorageRuntime::new(0, true, 1);
        let mut progress_tracker =
            TaskProgressTracker::new(Tasks::leaf_with_volume("spanning_tree".to_string(), 0));
        let result = runtime
            .compute_spanning_tree_mock(4, &mut progress_tracker)
            .unwrap();

        let mut edges = Vec::new();
        result.for_each_edge(|source, target, cost| {
            edges.push((source, target, cost));
            true
        });

        // Should have exactly 3 edges for a 4-node spanning tree
        assert_eq!(edges.len(), 3);

        // All edges should have positive costs
        for (_, _, cost) in &edges {
            assert!(*cost > 0.0);
        }
    }

    #[test]
    fn test_storage_runtime_empty_graph() {
        let runtime = SpanningTreeStorageRuntime::new(0, true, 1);

        // Mock empty graph
        let mut progress_tracker =
            TaskProgressTracker::new(Tasks::leaf_with_volume("spanning_tree".to_string(), 0));
        let result = runtime
            .compute_spanning_tree_mock(0, &mut progress_tracker)
            .unwrap();

        assert_eq!(result.effective_node_count(), 0);
        assert_eq!(result.total_weight(), 0.0);
    }

    #[test]
    fn test_storage_runtime_single_node() {
        let runtime = SpanningTreeStorageRuntime::new(0, true, 1);

        // Mock single node graph
        let mut progress_tracker =
            TaskProgressTracker::new(Tasks::leaf_with_volume("spanning_tree".to_string(), 0));
        let result = runtime
            .compute_spanning_tree_mock(1, &mut progress_tracker)
            .unwrap();

        assert_eq!(result.effective_node_count(), 1);
        assert_eq!(result.total_weight(), 0.0);
        assert_eq!(result.parent(0), -1); // Root has no parent
    }

    #[test]
    fn test_storage_rejects_invalid_contracts() {
        assert!(matches!(
            SpanningTreeStorageRuntime::new(4, true, 1).validate(4),
            Err(AlgorithmError::InvalidGraph(_))
        ));
        assert!(matches!(
            SpanningTreeStorageRuntime::new(0, true, 0).validate(4),
            Err(AlgorithmError::InvalidGraph(_))
        ));
        assert!(SpanningTreeStorageRuntime::validate_edge_weight(0, 1, 1.0).is_ok());
        assert!(SpanningTreeStorageRuntime::validate_edge_weight(0, 1, -1.0).is_ok());
        assert!(SpanningTreeStorageRuntime::validate_edge_weight(0, 1, f64::NAN).is_err());
        assert!(SpanningTreeStorageRuntime::validate_edge_weight(0, 1, f64::INFINITY).is_err());
    }
}
