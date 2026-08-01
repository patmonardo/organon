//! Delta Stepping Computation Runtime
//!
//! **Translation Source**: `org.neo4j.gds.paths.delta.TentativeDistances`
//!
//! This module implements the "Subtle pole" of the Delta Stepping algorithm,
//! handling ephemeral computation state and the sophisticated binning strategy
//! for efficient frontier management.

use crate::types::graph::MappedNodeId;
use std::collections::{HashMap, VecDeque};

/// Delta Stepping Computation Runtime
///
/// Translation of: `TentativeDistances` interface and implementations (lines 32-218)
/// Handles ephemeral computation state for the Delta Stepping algorithm
pub struct DeltaSteppingComputationRuntime {
    /// Distances from source to each node
    distances: HashMap<MappedNodeId, f64>,

    /// Predecessor for each node in shortest path (if storing predecessors)
    predecessors: HashMap<MappedNodeId, Option<MappedNodeId>>,

    /// Bins for organizing nodes by distance ranges
    bins: Vec<VecDeque<MappedNodeId>>,

    /// Source node
    source_node: MappedNodeId,

    /// Delta parameter for binning strategy
    delta: f64,

    /// Whether to store predecessors for path reconstruction
    store_predecessors: bool,

    /// Concurrency level
    #[allow(dead_code)]
    concurrency: usize,
}

impl DeltaSteppingComputationRuntime {
    /// Create a new Delta Stepping computation runtime
    ///
    /// Translation of: `TentativeDistances.distanceAndPredecessors()` (lines 76-100)
    pub fn new(
        source_node: MappedNodeId,
        delta: f64,
        concurrency: usize,
        store_predecessors: bool,
    ) -> Self {
        Self {
            distances: HashMap::new(),
            predecessors: HashMap::new(),
            bins: Vec::new(),
            source_node,
            delta,
            store_predecessors,
            concurrency,
        }
    }

    /// Initialize the computation runtime
    ///
    /// Translation of: Initialization in `compute()` method (lines 124-125)
    pub fn initialize(
        &mut self,
        source_node: MappedNodeId,
        delta: f64,
        store_predecessors: bool,
        node_count: usize,
    ) {
        self.source_node = source_node;
        self.delta = delta;
        self.store_predecessors = store_predecessors;

        // Clear previous state
        self.distances.clear();
        self.predecessors.clear();
        self.bins.clear();

        // Initialize with infinite distances
        for node_index in 0..node_count {
            let node_id = MappedNodeId::try_from(node_index)
                .expect("graph node count must fit mapped node ID space");
            self.distances.insert(node_id, f64::INFINITY);
            if self.store_predecessors {
                self.predecessors.insert(node_id, None);
            }
        }
    }

    /// Get distance to a node
    ///
    /// Translation of: `distance()` method (lines 40, 109, 154)
    pub fn distance(&self, node_id: MappedNodeId) -> f64 {
        self.distances
            .get(&node_id)
            .copied()
            .unwrap_or(f64::INFINITY)
    }

    /// Set distance to a node
    ///
    /// Translation of: `set()` method (lines 50, 119, 173)
    pub fn set_distance(&mut self, node_id: MappedNodeId, distance: f64) {
        self.distances.insert(node_id, distance);
    }

    /// Get predecessor of a node
    ///
    /// Translation of: `predecessor()` method (lines 45, 114, 158)
    pub fn predecessor(&self, node_id: MappedNodeId) -> Option<MappedNodeId> {
        if self.store_predecessors {
            self.predecessors.get(&node_id).copied().flatten()
        } else {
            None
        }
    }

    /// Set predecessor of a node
    ///
    /// Translation of: `set()` method (lines 50, 119, 173)
    pub fn set_predecessor(&mut self, node_id: MappedNodeId, predecessor: Option<MappedNodeId>) {
        if self.store_predecessors {
            self.predecessors.insert(node_id, predecessor);
        }
    }

    /// Add a node to a specific bin
    ///
    /// Translation of: Bin management in `DeltaSteppingTask.relaxNode()` (lines 270-279)
    pub fn add_to_bin(&mut self, node_id: MappedNodeId, bin_index: usize) {
        // Ensure we have enough bins
        while self.bins.len() <= bin_index {
            self.bins.push(VecDeque::new());
        }

        self.bins[bin_index].push_back(node_id);
    }

    /// Find the next non-empty bin starting from the given index
    ///
    /// Translation of: `minNonEmptyBin()` method (lines 227-234)
    pub fn find_next_non_empty_bin(&self, start_index: usize) -> Option<usize> {
        for i in start_index..self.bins.len() {
            if !self.bins[i].is_empty() {
                return Some(i);
            }
        }
        None
    }

    /// Get all nodes in a specific bin
    ///
    /// Translation of: Bin access in `DeltaSteppingTask.updateFrontier()` (lines 291-303)
    pub fn get_bin_nodes(&mut self, bin_index: usize) -> Vec<MappedNodeId> {
        if bin_index < self.bins.len() {
            self.bins[bin_index].drain(..).collect()
        } else {
            vec![]
        }
    }

    /// Compare and exchange distance (atomic operation)
    ///
    /// Translation of: `compareAndExchange()` method (lines 59, 124, 179)
    /// Simplified version without atomic operations for now
    pub fn compare_and_exchange(
        &mut self,
        node_id: MappedNodeId,
        expected_distance: f64,
        new_distance: f64,
        predecessor: MappedNodeId,
    ) -> f64 {
        let current_distance = self.distance(node_id);

        if current_distance > new_distance {
            self.set_distance(node_id, new_distance);
            if self.store_predecessors {
                self.set_predecessor(node_id, Some(predecessor));
            }
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

    /// Get all visited nodes (nodes with finite distance)
    pub fn get_visited_nodes(&self) -> Vec<MappedNodeId> {
        self.distances
            .iter()
            .filter(|(_, &distance)| distance < f64::INFINITY)
            .map(|(&node_id, _)| node_id)
            .collect()
    }

    /// Get delta parameter
    pub fn delta(&self) -> f64 {
        self.delta
    }

    /// Check if storing predecessors
    pub fn store_predecessors(&self) -> bool {
        self.store_predecessors
    }

    /// Get number of bins
    pub fn bin_count(&self) -> usize {
        self.bins.len()
    }

    /// Get nodes in a specific bin (without removing them)
    pub fn peek_bin_nodes(&self, bin_index: usize) -> &VecDeque<MappedNodeId> {
        if bin_index < self.bins.len() {
            &self.bins[bin_index]
        } else {
            static EMPTY: VecDeque<MappedNodeId> = VecDeque::new();
            &EMPTY
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mapped_node_id(value: u64) -> MappedNodeId {
        MappedNodeId::new(value)
    }

    #[test]
    fn test_delta_stepping_computation_runtime_initialization() {
        let source = mapped_node_id(0);
        let mut runtime = DeltaSteppingComputationRuntime::new(source, 1.0, 4, true);
        runtime.initialize(source, 1.0, true, 100);

        assert_eq!(runtime.source_node(), source);
        assert_eq!(runtime.delta(), 1.0);
        assert!(runtime.store_predecessors());
        assert_eq!(runtime.distance(source), f64::INFINITY);
        assert_eq!(runtime.predecessor(source), None);
    }

    #[test]
    fn test_delta_stepping_computation_runtime_empty_bins() {
        let source = mapped_node_id(0);
        let mut runtime = DeltaSteppingComputationRuntime::new(source, 1.0, 4, true);
        runtime.initialize(source, 1.0, true, 100);

        assert_eq!(runtime.bin_count(), 0);
        assert_eq!(runtime.find_next_non_empty_bin(0), None);
    }

    #[test]
    fn test_delta_stepping_computation_runtime_nodes_explored() {
        let source = mapped_node_id(0);
        let first = mapped_node_id(1);
        let second = mapped_node_id(2);
        let unreachable = mapped_node_id(3);
        let mut runtime = DeltaSteppingComputationRuntime::new(source, 1.0, 4, true);
        runtime.initialize(source, 1.0, true, 100);

        // Set some distances
        runtime.set_distance(first, 5.0);
        runtime.set_distance(second, 10.0);

        assert_eq!(runtime.distance(first), 5.0);
        assert_eq!(runtime.distance(second), 10.0);
        assert_eq!(runtime.distance(unreachable), f64::INFINITY);
    }

    #[test]
    fn test_delta_stepping_computation_runtime_total_cost() {
        let source = mapped_node_id(0);
        let mut runtime = DeltaSteppingComputationRuntime::new(source, 1.0, 4, true);
        runtime.initialize(source, 1.0, true, 100);

        // Set source distance
        runtime.set_distance(source, 0.0);
        runtime.set_predecessor(source, None);

        assert_eq!(runtime.distance(source), 0.0);
        assert_eq!(runtime.predecessor(source), None);
    }

    #[test]
    fn test_delta_stepping_computation_runtime_operations() {
        let source = mapped_node_id(0);
        let target = mapped_node_id(1);
        let mut runtime = DeltaSteppingComputationRuntime::new(source, 1.0, 4, true);
        runtime.initialize(source, 1.0, true, 100);

        // Test distance operations
        runtime.set_distance(target, 5.0);
        assert_eq!(runtime.distance(target), 5.0);

        // Test predecessor operations
        runtime.set_predecessor(target, Some(source));
        assert_eq!(runtime.predecessor(target), Some(source));
    }

    #[test]
    fn test_delta_stepping_computation_runtime_path_reconstruction() {
        let source = mapped_node_id(0);
        let middle = mapped_node_id(1);
        let target = mapped_node_id(2);
        let mut runtime = DeltaSteppingComputationRuntime::new(source, 1.0, 4, true);
        runtime.initialize(source, 1.0, true, 100);

        // Set up a simple path: 0 -> 1 -> 2
        runtime.set_distance(source, 0.0);
        runtime.set_predecessor(source, None);

        runtime.set_distance(middle, 5.0);
        runtime.set_predecessor(middle, Some(source));

        runtime.set_distance(target, 10.0);
        runtime.set_predecessor(target, Some(middle));

        // Test path reconstruction
        assert_eq!(runtime.predecessor(target), Some(middle));
        assert_eq!(runtime.predecessor(middle), Some(source));
        assert_eq!(runtime.predecessor(source), None);
    }

    #[test]
    fn test_delta_stepping_computation_runtime_lowest_f_cost() {
        let source = mapped_node_id(0);
        let mut runtime = DeltaSteppingComputationRuntime::new(source, 1.0, 4, true);
        runtime.initialize(source, 1.0, true, 100);

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
    fn test_delta_stepping_computation_runtime_binning() {
        let source = mapped_node_id(0);
        let first = mapped_node_id(1);
        let second = mapped_node_id(2);
        let third = mapped_node_id(3);
        let mut runtime = DeltaSteppingComputationRuntime::new(source, 1.0, 4, true);
        runtime.initialize(source, 1.0, true, 100);

        // Add nodes to different bins
        runtime.add_to_bin(first, 0);
        runtime.add_to_bin(second, 1);
        runtime.add_to_bin(third, 0);

        assert_eq!(runtime.bin_count(), 2);
        assert_eq!(runtime.find_next_non_empty_bin(0), Some(0));
        assert_eq!(runtime.find_next_non_empty_bin(1), Some(1));
        assert_eq!(runtime.find_next_non_empty_bin(2), None);

        // Test getting bin nodes
        let bin_0_nodes = runtime.get_bin_nodes(0);
        assert_eq!(bin_0_nodes.len(), 2);
        assert!(bin_0_nodes.contains(&first));
        assert!(bin_0_nodes.contains(&third));

        let bin_1_nodes = runtime.get_bin_nodes(1);
        assert_eq!(bin_1_nodes.len(), 1);
        assert_eq!(bin_1_nodes[0], second);
    }

    #[test]
    fn test_delta_stepping_computation_runtime_compare_and_exchange() {
        let source = mapped_node_id(0);
        let target = mapped_node_id(1);
        let mut runtime = DeltaSteppingComputationRuntime::new(source, 1.0, 4, true);
        runtime.initialize(source, 1.0, true, 100);

        // Set initial distance
        runtime.set_distance(target, 10.0);

        // Try to update with better distance
        let result = runtime.compare_and_exchange(target, 10.0, 5.0, source);
        assert_eq!(result, 10.0); // Should return expected distance on success
        assert_eq!(runtime.distance(target), 5.0);
        assert_eq!(runtime.predecessor(target), Some(source));

        // Try to update with worse distance
        runtime.compare_and_exchange(target, 5.0, 8.0, source);
        assert_eq!(runtime.distance(target), 5.0);
        assert_eq!(runtime.predecessor(target), Some(source));
    }
}
