//! **DFS Computation Runtime**
//!
//! **Translation Source**: `org.neo4j.gds.traversal.DFS`
//!
//! This module implements the "Subtle pole" for DFS algorithm - ephemeral computation state.

use crate::types::graph::MappedNodeId;

/// DFS Computation Runtime - handles ephemeral computation state
///
/// Translation of: `DFSComputation.java` (lines 32-75)
/// This implements the "Subtle pole" for accumulating traversal state
pub struct DfsComputationRuntime {
    /// Source node for traversal
    pub source_node: MappedNodeId,
    /// Whether to track paths
    pub track_paths: bool,
    /// Concurrency level
    pub concurrency: usize,
    /// Visited nodes (BitSet equivalent)
    visited: Vec<bool>,
    /// Maximum depth constraint
    max_depth: Option<u32>,
}

impl DfsComputationRuntime {
    /// Create new DFS computation runtime
    pub fn new(
        source_node: MappedNodeId,
        track_paths: bool,
        concurrency: usize,
        node_count: usize,
    ) -> Self {
        Self {
            source_node,
            track_paths,
            concurrency,
            visited: vec![false; node_count],
            max_depth: None,
        }
    }

    /// Initialize computation state
    ///
    /// Translation of: `DFSComputation.initialize()` (lines 76-100)
    /// This resets the internal state for a new traversal
    pub fn initialize(
        &mut self,
        source_node: MappedNodeId,
        max_depth: Option<u32>,
        node_count: usize,
    ) {
        self.source_node = source_node;
        self.max_depth = max_depth;
        self.visited = vec![false; node_count];
        // Add source node
        self.set_visited(source_node);
    }

    /// Check if a node has been visited
    ///
    /// Translation of: `DFSComputation.isVisited()` (lines 126-140)
    /// This checks the visited state of a node
    pub fn is_visited(&self, node: MappedNodeId) -> bool {
        usize::try_from(node)
            .ok()
            .and_then(|node_index| self.visited.get(node_index))
            .copied()
            .unwrap_or(false)
    }

    /// Set a node as visited
    pub fn set_visited(&mut self, node: MappedNodeId) {
        if let Ok(node_index) = usize::try_from(node) {
            if let Some(visited) = self.visited.get_mut(node_index) {
                *visited = true;
            }
        }
    }

    /// Get total number of visited nodes
    ///
    /// Translation of: `DFSComputation.getVisitedCount()` (lines 156-170)
    /// This returns the count of visited nodes
    pub fn visited_count(&self) -> usize {
        self.visited.iter().filter(|&&v| v).count()
    }

    /// Check if max depth constraint is satisfied
    ///
    /// Translation of: `DFSComputation.checkMaxDepth()` (lines 186-200)
    /// This validates depth constraints during traversal
    pub fn check_max_depth(&self, current_depth: f64) -> bool {
        match self.max_depth {
            Some(max_depth) => current_depth < max_depth as f64,
            None => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mapped(node_id: u64) -> MappedNodeId {
        MappedNodeId::new(node_id)
    }

    #[test]
    fn test_dfs_computation_runtime_creation() {
        let runtime = DfsComputationRuntime::new(mapped(0), true, 4, 10);
        assert_eq!(runtime.source_node, mapped(0));
        assert!(runtime.track_paths);
        assert_eq!(runtime.concurrency, 4);
        assert_eq!(runtime.visited_count(), 0);
    }

    #[test]
    fn test_dfs_computation_runtime_initialization() {
        let mut runtime = DfsComputationRuntime::new(mapped(0), true, 1, 10);
        runtime.initialize(mapped(5), Some(10), 10);

        assert_eq!(runtime.source_node, mapped(5));
        assert_eq!(runtime.max_depth, Some(10));
        assert_eq!(runtime.visited_count(), 1);
        assert!(runtime.is_visited(mapped(5)));
    }

    #[test]
    fn test_dfs_computation_runtime_visited_operations() {
        let mut runtime = DfsComputationRuntime::new(mapped(0), false, 1, 10);
        runtime.initialize(mapped(0), None, 10);

        assert!(!runtime.is_visited(mapped(1)));

        runtime.set_visited(mapped(1));
        assert!(runtime.is_visited(mapped(1)));
        assert_eq!(runtime.visited_count(), 2);
    }

    #[test]
    fn test_dfs_computation_runtime_max_depth_check() {
        let mut runtime = DfsComputationRuntime::new(mapped(0), false, 1, 10);
        runtime.initialize(mapped(0), Some(3), 10);

        assert!(runtime.check_max_depth(0.0));
        assert!(runtime.check_max_depth(1.0));
        assert!(runtime.check_max_depth(2.9));
        assert!(!runtime.check_max_depth(3.0));

        runtime.initialize(mapped(0), None, 10);
        assert!(runtime.check_max_depth(100.0)); // No limit
    }
}
