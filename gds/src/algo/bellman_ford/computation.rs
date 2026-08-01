//! Bellman-Ford Computation Runtime
//!
//! **Translation Source**: `org.neo4j.gds.paths.bellmanford.DistanceTracker`
//!
//! This module implements the "Subtle pole" of the Bellman-Ford algorithm,
//! handling ephemeral computation state and distance tracking.

use crate::types::graph::MappedNodeId;
use std::collections::HashMap;

/// Bellman-Ford Computation Runtime
///
/// Translation of: `DistanceTracker` class (lines 30-155)
/// Handles ephemeral computation state for the Bellman-Ford algorithm
pub struct BellmanFordComputationRuntime {
    /// Distances from source to each node
    distances: HashMap<MappedNodeId, f64>,

    /// Predecessor for each node in shortest path
    predecessors: HashMap<MappedNodeId, Option<MappedNodeId>>,

    /// Path length for each node
    lengths: HashMap<MappedNodeId, u32>,

    /// Nodes involved in negative cycles
    negative_cycle_nodes: Vec<MappedNodeId>,

    /// Whether any negative cycle was detected, independent of path tracking.
    contains_negative_cycle: bool,

    /// Source node
    source_node: MappedNodeId,

    /// Whether to track negative cycles
    track_negative_cycles: bool,

    /// Whether to track shortest paths
    track_paths: bool,

    /// Concurrency level
    #[allow(dead_code)]
    concurrency: usize,
}

impl BellmanFordComputationRuntime {
    /// Create a new Bellman-Ford computation runtime
    ///
    /// Translation of: `DistanceTracker.create()` method (lines 36-54)
    pub fn new(
        source_node: MappedNodeId,
        track_negative_cycles: bool,
        track_paths: bool,
        concurrency: usize,
    ) -> Self {
        Self {
            distances: HashMap::new(),
            predecessors: HashMap::new(),
            lengths: HashMap::new(),
            negative_cycle_nodes: Vec::new(),
            contains_negative_cycle: false,
            source_node,
            track_negative_cycles,
            track_paths,
            concurrency,
        }
    }

    /// Initialize the computation runtime
    ///
    /// Translation of: Initialization in `compute()` method (lines 100-103)
    pub fn initialize(
        &mut self,
        source_node: MappedNodeId,
        track_negative_cycles: bool,
        track_paths: bool,
        node_count: usize,
    ) {
        self.source_node = source_node;
        self.track_negative_cycles = track_negative_cycles;
        self.track_paths = track_paths;

        // Clear previous state
        self.distances.clear();
        self.predecessors.clear();
        self.lengths.clear();
        self.negative_cycle_nodes.clear();
        self.contains_negative_cycle = false;

        // Initialize with infinite distances
        for node_id in 0..node_count {
            let node_id = MappedNodeId::try_from(node_id)
                .expect("Bellman-Ford node count must fit mapped node ID space");
            self.distances.insert(node_id, f64::INFINITY);
            self.predecessors.insert(node_id, None);
            self.lengths.insert(node_id, u32::MAX);
        }
    }

    /// Get distance to a node
    ///
    /// Translation of: `distance()` method (lines 83-85)
    pub fn distance(&self, node_id: MappedNodeId) -> f64 {
        self.distances
            .get(&node_id)
            .copied()
            .unwrap_or(f64::INFINITY)
    }

    /// Set distance to a node
    ///
    /// Translation of: `set()` method (lines 101-105)
    pub fn set_distance(&mut self, node_id: MappedNodeId, distance: f64) {
        self.distances.insert(node_id, distance);
    }

    /// Get predecessor of a node
    ///
    /// Translation of: `predecessor()` method (lines 87-89)
    pub fn predecessor(&self, node_id: MappedNodeId) -> Option<MappedNodeId> {
        self.predecessors.get(&node_id).copied().flatten()
    }

    /// Set predecessor of a node
    ///
    /// Translation of: `set()` method (lines 101-105)
    pub fn set_predecessor(&mut self, node_id: MappedNodeId, predecessor: Option<MappedNodeId>) {
        self.predecessors.insert(node_id, predecessor);
    }

    /// Get path length to a node
    ///
    /// Translation of: `length()` method (lines 91-91)
    pub fn length(&self, node_id: MappedNodeId) -> u32 {
        self.lengths.get(&node_id).copied().unwrap_or(u32::MAX)
    }

    /// Set path length to a node
    ///
    /// Translation of: `set()` method (lines 101-105)
    pub fn set_length(&mut self, node_id: MappedNodeId, length: u32) {
        self.lengths.insert(node_id, length);
    }

    /// Add a node to negative cycles
    ///
    /// Translation of: `processNegativeCycle()` method (lines 152-162)
    pub fn add_negative_cycle_node(&mut self, node_id: MappedNodeId) {
        self.contains_negative_cycle = true;
        if self.track_negative_cycles && !self.negative_cycle_nodes.contains(&node_id) {
            self.negative_cycle_nodes.push(node_id);
        }
    }

    /// Check if there are negative cycles
    ///
    /// Translation of: `containsNegativeCycle` check (line 122)
    pub fn has_negative_cycles(&self) -> bool {
        self.contains_negative_cycle
    }

    /// Get all negative cycle nodes
    ///
    /// Translation of: `negativeCyclesVertices` usage (lines 83, 122)
    pub fn get_negative_cycle_nodes(&self) -> &[MappedNodeId] {
        &self.negative_cycle_nodes
    }

    /// Compare and exchange distance (atomic operation)
    ///
    /// Translation of: `compareAndExchange()` method (lines 107-154)
    /// Simplified version without atomic operations for now
    pub fn compare_and_exchange(
        &mut self,
        node_id: MappedNodeId,
        expected_distance: f64,
        new_distance: f64,
        predecessor: MappedNodeId,
        length: u32,
    ) -> f64 {
        let current_distance = self.distance(node_id);

        if current_distance > new_distance {
            self.set_distance(node_id, new_distance);
            self.set_predecessor(node_id, Some(predecessor));
            self.set_length(node_id, length);
            expected_distance
        } else {
            // Signal unsuccessful update
            if expected_distance == 0.0 {
                -1.0
            } else {
                -expected_distance
            }
        }
    }

    /// Get total number of nodes processed
    pub fn node_count(&self) -> usize {
        self.distances.len()
    }

    /// Get source node
    pub fn source_node(&self) -> MappedNodeId {
        self.source_node
    }

    /// Check if tracking negative cycles
    pub fn track_negative_cycles(&self) -> bool {
        self.track_negative_cycles
    }

    /// Check if tracking paths
    pub fn track_paths(&self) -> bool {
        self.track_paths
    }

    /// Get all visited nodes (nodes with finite distances)
    pub fn get_visited_nodes(&self) -> Vec<MappedNodeId> {
        self.distances.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mapped_node_id(value: u64) -> MappedNodeId {
        MappedNodeId::new(value)
    }

    #[test]
    fn test_bellman_ford_computation_runtime_initialization() {
        let source = mapped_node_id(0);
        let mut runtime = BellmanFordComputationRuntime::new(source, true, true, 4);
        runtime.initialize(source, true, true, 100);

        assert_eq!(runtime.source_node(), source);
        assert!(runtime.track_negative_cycles());
        assert!(runtime.track_paths());
        assert_eq!(runtime.distance(source), f64::INFINITY);
        assert_eq!(runtime.predecessor(source), None);
        assert_eq!(runtime.length(source), u32::MAX);
    }

    #[test]
    fn test_bellman_ford_computation_runtime_empty_negative_cycles() {
        let source = mapped_node_id(0);
        let mut runtime = BellmanFordComputationRuntime::new(source, true, true, 4);
        runtime.initialize(source, true, true, 100);

        assert!(!runtime.has_negative_cycles());
        assert!(runtime.get_negative_cycle_nodes().is_empty());
    }

    #[test]
    fn negative_cycle_detection_is_independent_of_cycle_path_tracking() {
        let source = mapped_node_id(0);
        let mut runtime = BellmanFordComputationRuntime::new(source, false, true, 4);
        runtime.initialize(source, false, true, 100);

        runtime.add_negative_cycle_node(mapped_node_id(3));

        assert!(runtime.has_negative_cycles());
        assert!(runtime.get_negative_cycle_nodes().is_empty());
    }

    #[test]
    fn test_bellman_ford_computation_runtime_nodes_explored() {
        let source = mapped_node_id(0);
        let first = mapped_node_id(1);
        let second = mapped_node_id(2);
        let unreachable = mapped_node_id(3);
        let mut runtime = BellmanFordComputationRuntime::new(source, true, true, 4);
        runtime.initialize(source, true, true, 100);

        // Set some distances
        runtime.set_distance(first, 5.0);
        runtime.set_distance(second, 10.0);

        assert_eq!(runtime.distance(first), 5.0);
        assert_eq!(runtime.distance(second), 10.0);
        assert_eq!(runtime.distance(unreachable), f64::INFINITY);
    }

    #[test]
    fn test_bellman_ford_computation_runtime_total_cost() {
        let source = mapped_node_id(0);
        let mut runtime = BellmanFordComputationRuntime::new(source, true, true, 4);
        runtime.initialize(source, true, true, 100);

        // Set source distance
        runtime.set_distance(source, 0.0);
        runtime.set_predecessor(source, None);
        runtime.set_length(source, 0);

        assert_eq!(runtime.distance(source), 0.0);
        assert_eq!(runtime.predecessor(source), None);
        assert_eq!(runtime.length(source), 0);
    }

    #[test]
    fn test_bellman_ford_computation_runtime_operations() {
        let source = mapped_node_id(0);
        let target = mapped_node_id(1);
        let mut runtime = BellmanFordComputationRuntime::new(source, true, true, 4);
        runtime.initialize(source, true, true, 100);

        // Test distance operations
        runtime.set_distance(target, 5.0);
        assert_eq!(runtime.distance(target), 5.0);

        // Test predecessor operations
        runtime.set_predecessor(target, Some(source));
        assert_eq!(runtime.predecessor(target), Some(source));

        // Test length operations
        runtime.set_length(target, 1);
        assert_eq!(runtime.length(target), 1);
    }

    #[test]
    fn test_bellman_ford_computation_runtime_path_reconstruction() {
        let source = mapped_node_id(0);
        let middle = mapped_node_id(1);
        let target = mapped_node_id(2);
        let mut runtime = BellmanFordComputationRuntime::new(source, true, true, 4);
        runtime.initialize(source, true, true, 100);

        // Set up a simple path: 0 -> 1 -> 2
        runtime.set_distance(source, 0.0);
        runtime.set_predecessor(source, None);
        runtime.set_length(source, 0);

        runtime.set_distance(middle, 5.0);
        runtime.set_predecessor(middle, Some(source));
        runtime.set_length(middle, 1);

        runtime.set_distance(target, 10.0);
        runtime.set_predecessor(target, Some(middle));
        runtime.set_length(target, 2);

        // Test path reconstruction
        assert_eq!(runtime.predecessor(target), Some(middle));
        assert_eq!(runtime.predecessor(middle), Some(source));
        assert_eq!(runtime.predecessor(source), None);
    }

    #[test]
    fn test_bellman_ford_computation_runtime_lowest_f_cost() {
        let source = mapped_node_id(0);
        let mut runtime = BellmanFordComputationRuntime::new(source, true, true, 4);
        runtime.initialize(source, true, true, 100);

        // Set different distances
        runtime.set_distance(mapped_node_id(1), 10.0);
        runtime.set_distance(mapped_node_id(2), 5.0);
        runtime.set_distance(mapped_node_id(3), 15.0);

        // Find node with minimum distance
        let mut min_node = None;
        let mut min_distance = f64::INFINITY;

        for node_id in (1..=3).map(mapped_node_id) {
            let distance = runtime.distance(node_id);
            if distance < min_distance {
                min_distance = distance;
                min_node = Some(node_id);
            }
        }

        assert_eq!(min_node, Some(mapped_node_id(2)));
        assert_eq!(min_distance, 5.0);
    }

    #[test]
    fn test_bellman_ford_computation_runtime_negative_cycles() {
        let source = mapped_node_id(0);
        let first_cycle_node = mapped_node_id(5);
        let second_cycle_node = mapped_node_id(6);
        let mut runtime = BellmanFordComputationRuntime::new(source, true, true, 4);
        runtime.initialize(source, true, true, 100);

        // Add negative cycle nodes
        runtime.add_negative_cycle_node(first_cycle_node);
        runtime.add_negative_cycle_node(second_cycle_node);

        assert!(runtime.has_negative_cycles());
        assert_eq!(runtime.get_negative_cycle_nodes().len(), 2);
        assert!(runtime
            .get_negative_cycle_nodes()
            .contains(&first_cycle_node));
        assert!(runtime
            .get_negative_cycle_nodes()
            .contains(&second_cycle_node));
    }

    #[test]
    fn test_bellman_ford_computation_runtime_compare_and_exchange() {
        let source = mapped_node_id(0);
        let target = mapped_node_id(1);
        let mut runtime = BellmanFordComputationRuntime::new(source, true, true, 4);
        runtime.initialize(source, true, true, 100);

        // Set initial distance
        runtime.set_distance(target, 10.0);

        // Try to update with better distance
        let result = runtime.compare_and_exchange(target, 10.0, 5.0, source, 1);
        assert_eq!(result, 10.0); // Should return expected distance on success
        assert_eq!(runtime.distance(target), 5.0);
        assert_eq!(runtime.predecessor(target), Some(source));

        // Try to update with worse distance
        runtime.compare_and_exchange(target, 5.0, 8.0, source, 1);
        assert_eq!(runtime.distance(target), 5.0);
        assert_eq!(runtime.predecessor(target), Some(source));
    }
}
