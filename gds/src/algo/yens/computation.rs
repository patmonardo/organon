//! **Yen's Computation Runtime**
//!
//! **Translation Source**: `org.neo4j.gds.paths.yens.YensTask`
//!
//! This module implements the "Subtle pole" for Yen's algorithm - ephemeral computation state.

use super::mutable_path_result::MutablePathResult;
use super::relationship_filterer::RelationshipFilterer;
use super::spec::YensResult;
use super::YensStorageRuntime;
use crate::core::utils::progress::TaskProgressTracker;
use crate::core::utils::progress::Tasks;
use crate::types::graph::Graph;
use crate::types::graph::NodeId;
use std::collections::HashMap;

/// Yen's Computation Runtime - handles ephemeral computation state
///
/// Translation of: `YensTask.java` (lines 35-167)
/// This implements the "Subtle pole" for managing Yen's algorithm state
pub struct YensComputationRuntime {
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
    /// Relationship filterer for avoiding cycles
    relationship_filterer: RelationshipFilterer,
    /// Visited nodes for cycle avoidance
    visited_nodes: HashMap<NodeId, bool>,
    /// Number of spur searches attempted in this computation.
    spur_searches: usize,
    /// Number of candidate paths generated in this computation.
    candidates_generated: usize,
    /// Number of accepted shortest paths found in this computation.
    paths_found: usize,
}

impl YensComputationRuntime {
    /// Create new Yen's computation runtime
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
            relationship_filterer: RelationshipFilterer::new(k, track_relationships),
            visited_nodes: HashMap::new(),
            spur_searches: 0,
            candidates_generated: 0,
            paths_found: 0,
        }
    }

    /// Initialize computation state
    ///
    /// Translation of: `YensTask` constructor and initialization
    /// This resets the internal state for a new computation
    pub fn initialize(
        &mut self,
        source_node: NodeId,
        target_node: NodeId,
        k: usize,
        track_relationships: bool,
    ) {
        self.source_node = source_node;
        self.target_node = target_node;
        self.k = k;
        self.track_relationships = track_relationships;
        self.visited_nodes.clear();
        self.relationship_filterer = RelationshipFilterer::new(k, track_relationships);
        self.spur_searches = 0;
        self.candidates_generated = 0;
        self.paths_found = 0;
    }

    /// Prepare a filter for one spur-node Dijkstra search.
    pub fn prepare_relationship_filter(
        &mut self,
        spur_node: NodeId,
        prev_path: &MutablePathResult,
        k_shortest_paths: &[MutablePathResult],
        spur_index: usize,
    ) -> RelationshipFilterer {
        self.relationship_filterer =
            RelationshipFilterer::new(k_shortest_paths.len().max(1), self.track_relationships);
        self.relationship_filterer.set_filter(spur_node);

        for path in k_shortest_paths {
            if path.matches_exactly(prev_path, spur_index + 1) {
                self.relationship_filterer
                    .add_blocking_neighbor(path, spur_index);
            }
        }

        self.relationship_filterer.prepare();
        self.relationship_filterer.clone()
    }

    pub fn record_spur_search(&mut self) {
        self.spur_searches += 1;
    }

    pub fn record_candidate_generated(&mut self) {
        self.candidates_generated += 1;
    }

    pub fn record_path_found(&mut self) {
        self.paths_found += 1;
    }

    pub fn spur_search_count(&self) -> usize {
        self.spur_searches
    }

    pub fn candidates_generated_count(&self) -> usize {
        self.candidates_generated
    }

    pub fn paths_found_count(&self) -> usize {
        self.paths_found
    }

    /// Add a visited node to avoid cycles
    ///
    /// Translation of: `YensTask.withVisited()` (lines 128-130)
    /// This marks nodes as visited to avoid cycles
    pub fn add_visited_node(&mut self, node: NodeId) {
        self.visited_nodes.insert(node, true);
    }

    /// Check if a node has been visited
    pub fn is_visited(&self, node: NodeId) -> bool {
        self.visited_nodes.get(&node).copied().unwrap_or(false)
    }

    /// Get the relationship filterer
    pub fn relationship_filterer(&mut self) -> &mut RelationshipFilterer {
        &mut self.relationship_filterer
    }

    /// Reset visited nodes
    pub fn reset_visited(&mut self) {
        self.visited_nodes.clear();
    }

    /// Get number of visited nodes
    pub fn visited_count(&self) -> usize {
        self.visited_nodes.len()
    }

    /// Compute Yen's K-shortest paths using a graph for neighbor access
    pub fn compute_with_graph(&mut self, graph: &dyn Graph) -> YensResult {
        // Create storage runtime
        let storage = YensStorageRuntime::new(
            self.source_node,
            self.target_node,
            self.k,
            self.track_relationships,
            self.concurrency,
        );

        // For now, use outgoing direction (0 = outgoing)
        // This can be parameterized later if needed
        let direction_byte = 0u8;

        // Create a dummy progress tracker
        // In a real implementation, this would be passed in
        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf("yens".to_string()),
            self.concurrency,
        );

        // Run the algorithm
        storage
            .compute_yens(self, Some(graph), direction_byte, &mut progress_tracker)
            .unwrap_or_else(|_e| {
                // For now, return empty result on error
                // In production, this should be handled properly
                YensResult {
                    paths: vec![],
                    path_count: 0,
                    computation_time_ms: 0,
                    spur_searches: 0,
                    candidates_generated: 0,
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yens_computation_runtime_creation() {
        let runtime = YensComputationRuntime::new(0, 3, 5, true, 4);
        assert_eq!(runtime.source_node, 0);
        assert_eq!(runtime.target_node, 3);
        assert_eq!(runtime.k, 5);
        assert!(runtime.track_relationships);
        assert_eq!(runtime.concurrency, 4);
        assert_eq!(runtime.visited_count(), 0);
    }

    #[test]
    fn test_yens_computation_runtime_initialization() {
        let mut runtime = YensComputationRuntime::new(0, 3, 5, true, 1);
        runtime.record_spur_search();
        runtime.record_candidate_generated();
        runtime.record_path_found();

        runtime.initialize(1, 4, 3, false);

        assert_eq!(runtime.source_node, 1);
        assert_eq!(runtime.target_node, 4);
        assert_eq!(runtime.k, 3);
        assert!(!runtime.track_relationships);
        assert_eq!(runtime.visited_count(), 0);
        assert_eq!(runtime.spur_search_count(), 0);
        assert_eq!(runtime.candidates_generated_count(), 0);
        assert_eq!(runtime.paths_found_count(), 0);
    }

    #[test]
    fn test_yens_computation_runtime_visited_operations() {
        let mut runtime = YensComputationRuntime::new(0, 3, 5, true, 1);

        assert!(!runtime.is_visited(1));
        runtime.add_visited_node(1);
        assert!(runtime.is_visited(1));
        assert_eq!(runtime.visited_count(), 1);

        runtime.reset_visited();
        assert!(!runtime.is_visited(1));
        assert_eq!(runtime.visited_count(), 0);
    }

    #[test]
    fn test_prepare_relationship_filter_blocks_same_root() {
        let mut runtime = YensComputationRuntime::new(0, 3, 3, false, 1);
        let previous =
            MutablePathResult::new(0, 0, 3, vec![0, 1, 2, 3], vec![], vec![0.0, 1.0, 3.0, 4.0]);
        let filter = runtime.prepare_relationship_filter(1, &previous, &[previous.clone()], 1);

        assert!(!filter.valid_relationship(1, 2, 10));
        assert!(filter.valid_relationship(1, 3, 11));
    }
}
