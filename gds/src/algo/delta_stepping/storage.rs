//! Delta Stepping Storage Runtime
//!
//! **Translation Source**: `org.neo4j.gds.paths.delta.DeltaStepping`
//!
//! This module implements the "Gross pole" of the Delta Stepping algorithm,
//! handling persistent data access and the main algorithm orchestration.
//! Delta Stepping uses a sophisticated binning strategy for efficient frontier management.

use super::spec::{DeltaSteppingPathResult, DeltaSteppingResult};
use super::DeltaSteppingComputationRuntime;
use crate::concurrency::{install_with_concurrency, Concurrency};
use crate::core::utils::progress::{ProgressTracker, UNKNOWN_VOLUME};
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::graph::Graph;
use crate::types::graph::NodeId;
use crate::types::properties::relationship::RelationshipCursorBox;
use rayon::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Instant;

const FRONTIER_BATCH_SIZE: usize = 64;
const NO_BIN: usize = usize::MAX;

#[derive(Debug, Clone)]
struct RelaxationCandidate {
    source_node: NodeId,
    target_node: NodeId,
    new_distance: f64,
    dest_bin: usize,
}

#[derive(Debug, Clone)]
struct BinExpansion {
    candidates: Vec<RelaxationCandidate>,
    scanned_relationships: usize,
}

/// Delta Stepping Storage Runtime
///
/// Translation of: Main `DeltaStepping` class (lines 52-380)
/// Handles the persistent data access and algorithm orchestration
pub struct DeltaSteppingStorageRuntime {
    /// Source node for shortest path computation
    pub source_node: NodeId,

    /// Delta parameter for binning strategy
    pub delta: f64,

    /// Concurrency level for parallel processing
    pub concurrency: usize,

    /// Whether to store predecessors for path reconstruction
    pub store_predecessors: bool,
}

impl DeltaSteppingStorageRuntime {
    /// Create a new Delta Stepping storage runtime
    ///
    /// Translation of: Constructor (lines 86-114)
    pub fn new(
        source_node: NodeId,
        delta: f64,
        concurrency: usize,
        store_predecessors: bool,
    ) -> Self {
        Self {
            source_node,
            delta,
            concurrency,
            store_predecessors,
        }
    }

    /// Compute Delta Stepping shortest paths
    ///
    /// Translation of: `compute()` method (lines 117-164)
    pub fn compute_delta_stepping(
        &mut self,
        computation: &mut DeltaSteppingComputationRuntime,
        graph: Option<&dyn Graph>,
        direction: u8,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<DeltaSteppingResult, AlgorithmError> {
        let volume = graph
            .map(|g| g.relationship_count())
            .unwrap_or(UNKNOWN_VOLUME);
        if volume == UNKNOWN_VOLUME {
            progress_tracker.begin_subtask_unknown();
        } else {
            progress_tracker.begin_subtask_with_volume(volume);
        }

        let mut scanned_relationships: usize = 0;

        let start_time = Instant::now();

        let result = (|| {
            // Initialize computation runtime
            let node_count = graph.map(|g| g.node_count()).unwrap_or(100);
            Self::validate_node_in_graph(self.source_node, node_count, "source")?;
            computation.initialize(
                self.source_node,
                self.delta,
                self.store_predecessors,
                node_count,
            );

            // Initialize distances
            computation.set_distance(self.source_node, 0.0);
            if self.store_predecessors {
                computation.set_predecessor(self.source_node, None);
            }

            if let Some(graph) = graph {
                scanned_relationships = self.compute_parallel_bins(
                    computation,
                    graph,
                    direction,
                    node_count,
                    progress_tracker,
                )?;
            } else {
                scanned_relationships = self.compute_sequential_bins(
                    computation,
                    graph,
                    direction,
                    node_count,
                    progress_tracker,
                )?;
            }

            if scanned_relationships > 0 {
                progress_tracker.log_progress(scanned_relationships);
            }

            // Generate results
            let shortest_paths = if self.store_predecessors {
                self.generate_shortest_paths(computation)?
            } else {
                vec![]
            };

            let computation_time_ms = start_time.elapsed().as_millis() as u64;

            Ok(DeltaSteppingResult {
                shortest_paths,
                computation_time_ms,
            })
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

    /// Generate shortest paths from the distance tracker
    ///
    /// Translation of: `pathResults()` method (lines 306-346)
    fn generate_shortest_paths(
        &self,
        computation: &DeltaSteppingComputationRuntime,
    ) -> Result<Vec<DeltaSteppingPathResult>, AlgorithmError> {
        let mut paths = Vec::new();
        let node_count = computation.node_count();

        for target_node in 0..node_count {
            let target_node = target_node as NodeId;
            if computation.predecessor(target_node).is_some() {
                let path = self.reconstruct_path(computation, self.source_node, target_node)?;
                paths.push(path);
            }
        }

        Ok(paths)
    }

    fn compute_sequential_bins(
        &self,
        computation: &mut DeltaSteppingComputationRuntime,
        graph: Option<&dyn Graph>,
        direction: u8,
        node_count: usize,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<usize, AlgorithmError> {
        let mut scanned_relationships = 0usize;
        const LOG_BATCH: usize = 256;

        let mut current_bin = 0usize;
        let mut frontier = vec![self.source_node];

        while current_bin != NO_BIN {
            loop {
                let mut next_local_frontier = Vec::new();

                for node_id in frontier.drain(..) {
                    if self.bin_index(computation.distance(node_id)) != current_bin {
                        continue;
                    }

                    let neighbors = self.get_neighbors_with_weights(graph, node_id, direction)?;
                    scanned_relationships = scanned_relationships.saturating_add(neighbors.len());
                    if scanned_relationships >= LOG_BATCH {
                        progress_tracker.log_progress(scanned_relationships);
                        scanned_relationships = 0;
                    }

                    for (neighbor, weight) in neighbors {
                        Self::validate_node_in_graph(neighbor, node_count, "target")?;
                        Self::validate_edge_weight(node_id, neighbor, weight)?;

                        let new_distance = computation.distance(node_id) + weight;
                        Self::validate_path_cost(node_id, neighbor, new_distance)?;
                        if new_distance < computation.distance(neighbor) {
                            computation.set_distance(neighbor, new_distance);
                            if self.store_predecessors {
                                computation.set_predecessor(neighbor, Some(node_id));
                            }

                            let dest_bin = self.bin_index(new_distance);
                            if dest_bin == current_bin {
                                next_local_frontier.push(neighbor);
                            } else {
                                computation.add_to_bin(neighbor, dest_bin);
                            }
                        }
                    }
                }

                if next_local_frontier.is_empty() {
                    break;
                }
                frontier = next_local_frontier;
            }

            current_bin = computation.find_next_non_empty_bin(current_bin + 1);
            if current_bin == NO_BIN {
                break;
            }
            frontier = computation.get_bin_nodes(current_bin);
        }

        Ok(scanned_relationships)
    }

    fn compute_parallel_bins(
        &self,
        computation: &mut DeltaSteppingComputationRuntime,
        graph: &dyn Graph,
        direction: u8,
        node_count: usize,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<usize, AlgorithmError> {
        let concurrency = Concurrency::from_usize(self.concurrency.max(1));
        let mut scanned_relationships = 0usize;
        let mut current_bin = 0usize;
        let mut frontier = vec![self.source_node];

        while current_bin != NO_BIN {
            loop {
                let expansions = self.expand_frontier_parallel(
                    graph,
                    computation,
                    &frontier,
                    current_bin,
                    direction,
                    node_count,
                    concurrency,
                )?;

                let mut candidates_by_target: HashMap<NodeId, RelaxationCandidate> = HashMap::new();
                for expansion in expansions {
                    scanned_relationships =
                        scanned_relationships.saturating_add(expansion.scanned_relationships);
                    for candidate in expansion.candidates {
                        candidates_by_target
                            .entry(candidate.target_node)
                            .and_modify(|existing| {
                                if candidate.new_distance < existing.new_distance {
                                    *existing = candidate.clone();
                                }
                            })
                            .or_insert(candidate);
                    }
                }

                if scanned_relationships > 0 {
                    progress_tracker.log_progress(scanned_relationships);
                    scanned_relationships = 0;
                }

                let mut next_local_frontier = Vec::new();
                let mut queued_local = HashSet::new();
                let mut candidates: Vec<_> = candidates_by_target.into_values().collect();
                candidates.sort_by_key(|candidate| candidate.target_node);

                for candidate in candidates {
                    if candidate.new_distance < computation.distance(candidate.target_node) {
                        computation.set_distance(candidate.target_node, candidate.new_distance);
                        if self.store_predecessors {
                            computation.set_predecessor(
                                candidate.target_node,
                                Some(candidate.source_node),
                            );
                        }

                        if candidate.dest_bin == current_bin {
                            if queued_local.insert(candidate.target_node) {
                                next_local_frontier.push(candidate.target_node);
                            }
                        } else {
                            computation.add_to_bin(candidate.target_node, candidate.dest_bin);
                        }
                    }
                }

                if next_local_frontier.is_empty() {
                    break;
                }
                frontier = next_local_frontier;
            }

            current_bin = computation.find_next_non_empty_bin(current_bin + 1);
            if current_bin == NO_BIN {
                break;
            }
            frontier = computation.get_bin_nodes(current_bin);
        }

        Ok(scanned_relationships)
    }

    fn expand_frontier_parallel(
        &self,
        graph: &dyn Graph,
        computation: &DeltaSteppingComputationRuntime,
        frontier: &[NodeId],
        current_bin: usize,
        direction: u8,
        node_count: usize,
        concurrency: Concurrency,
    ) -> Result<Vec<BinExpansion>, AlgorithmError> {
        let chunk_count = frontier.len().div_ceil(FRONTIER_BATCH_SIZE);
        install_with_concurrency(concurrency, || {
            (0..chunk_count)
                .into_par_iter()
                .map(|chunk_idx| {
                    let start = chunk_idx * FRONTIER_BATCH_SIZE;
                    let end = (start + FRONTIER_BATCH_SIZE).min(frontier.len());
                    self.expand_frontier_chunk(
                        graph,
                        computation,
                        &frontier[start..end],
                        current_bin,
                        direction,
                        node_count,
                    )
                })
                .collect::<Result<Vec<_>, _>>()
        })
    }

    fn expand_frontier_chunk(
        &self,
        graph: &dyn Graph,
        computation: &DeltaSteppingComputationRuntime,
        frontier: &[NodeId],
        current_bin: usize,
        direction: u8,
        node_count: usize,
    ) -> Result<BinExpansion, AlgorithmError> {
        let worker_graph = Graph::concurrent_copy(graph);
        let fallback = worker_graph.default_property_value();
        let mut candidates = Vec::new();
        let mut scanned_relationships = 0usize;

        for node_id in frontier {
            if self.bin_index(computation.distance(*node_id)) != current_bin {
                continue;
            }

            let stream = if direction == 1 {
                worker_graph.stream_inverse_relationships(*node_id, fallback)
            } else {
                worker_graph.stream_relationships(*node_id, fallback)
            };

            for cursor in stream {
                scanned_relationships += 1;
                let neighbor = cursor.target_id();
                let weight = cursor.property();
                Self::validate_node_in_graph(neighbor, node_count, "target")?;
                Self::validate_edge_weight(*node_id, neighbor, weight)?;

                let new_distance = computation.distance(*node_id) + weight;
                Self::validate_path_cost(*node_id, neighbor, new_distance)?;
                if new_distance < computation.distance(neighbor) {
                    candidates.push(RelaxationCandidate {
                        source_node: *node_id,
                        target_node: neighbor,
                        new_distance,
                        dest_bin: self.bin_index(new_distance),
                    });
                }
            }
        }

        Ok(BinExpansion {
            candidates,
            scanned_relationships,
        })
    }

    /// Reconstruct a path from source to target
    ///
    /// Translation of: `pathResult()` method (lines 348-380)
    fn reconstruct_path(
        &self,
        computation: &DeltaSteppingComputationRuntime,
        source_node: NodeId,
        target_node: NodeId,
    ) -> Result<DeltaSteppingPathResult, AlgorithmError> {
        let mut node_ids = Vec::new();
        let mut costs = Vec::new();

        let mut current_node = target_node;

        // Backtrack from target to source
        while current_node != source_node {
            node_ids.push(current_node);
            costs.push(computation.distance(current_node));

            current_node = computation
                .predecessor(current_node)
                .ok_or_else(|| AlgorithmError::InvalidGraph("Missing predecessor".to_string()))?;
        }

        // Add source node
        node_ids.push(source_node);
        costs.push(computation.distance(source_node));

        // Reverse to get correct order
        node_ids.reverse();
        costs.reverse();

        Ok(DeltaSteppingPathResult {
            index: 0, // Indexing is not yet exposed/used in the Rust surface.
            source_node,
            target_node,
            node_ids,
            costs,
        })
    }

    /// Get neighbors with weights for a given node
    ///
    /// Uses Graph::stream_relationships to iterate outgoing edges with weights
    fn get_neighbors_with_weights(
        &self,
        graph: Option<&dyn Graph>,
        node_id: NodeId,
        direction: u8,
    ) -> Result<Vec<(NodeId, f64)>, AlgorithmError> {
        if let Some(g) = graph {
            let fallback: f64 = 1.0;
            let iter: Box<dyn Iterator<Item = RelationshipCursorBox> + Send> = if direction == 1 {
                g.stream_inverse_relationships(node_id, fallback)
            } else {
                g.stream_relationships(node_id, fallback)
            };
            Ok(iter
                .map(|cursor| (cursor.target_id(), cursor.property()))
                .collect())
        } else {
            // Mock implementation for tests
            Ok(match node_id {
                0 => vec![(1, 1.0), (2, 4.0)],
                1 => vec![(2, 2.0), (3, 5.0)],
                2 => vec![(3, 1.0), (4, 3.0)],
                3 => vec![(4, 2.0)],
                _ => vec![],
            })
        }
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
                "Delta Stepping requires non-negative finite edge weights; edge {source_node}->{target_node} has weight {weight}"
            )));
        }
        Ok(())
    }

    fn validate_path_cost(
        source_node: NodeId,
        target_node: NodeId,
        cost: f64,
    ) -> Result<(), AlgorithmError> {
        if !cost.is_finite() {
            return Err(AlgorithmError::InvalidGraph(format!(
                "Delta Stepping path cost overflowed for edge {source_node}->{target_node}"
            )));
        }
        Ok(())
    }

    fn bin_index(&self, distance: f64) -> usize {
        (distance / self.delta) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::utils::progress::{TaskProgressTracker, Tasks};
    use crate::projection::Orientation;
    use crate::types::graph_store::{DefaultGraphStore, GraphStore};
    use crate::types::random::{RandomGraphConfig, RandomRelationshipConfig};
    use std::collections::HashSet;

    #[test]
    fn test_delta_stepping_storage_runtime_creation() {
        let storage = DeltaSteppingStorageRuntime::new(0, 1.0, 4, true);
        assert_eq!(storage.source_node, 0);
        assert_eq!(storage.delta, 1.0);
        assert_eq!(storage.concurrency, 4);
        assert!(storage.store_predecessors);
    }

    #[test]
    fn test_delta_stepping_path_computation() {
        let mut storage = DeltaSteppingStorageRuntime::new(0, 1.0, 4, true);
        let mut computation = DeltaSteppingComputationRuntime::new(0, 1.0, 4, true);
        let mut progress_tracker =
            TaskProgressTracker::new(Tasks::leaf("delta_stepping".to_string()));

        // Test basic path computation
        let result =
            storage.compute_delta_stepping(&mut computation, None, 0, &mut progress_tracker);
        assert!(result.is_ok());

        let _ = result.unwrap();
    }

    #[test]
    fn test_delta_stepping_path_same_source_target() {
        let mut storage = DeltaSteppingStorageRuntime::new(0, 1.0, 4, true);
        let mut computation = DeltaSteppingComputationRuntime::new(0, 1.0, 4, true);
        let mut progress_tracker =
            TaskProgressTracker::new(Tasks::leaf("delta_stepping".to_string()));

        // Test with same source and target
        let result =
            storage.compute_delta_stepping(&mut computation, None, 0, &mut progress_tracker);
        assert!(result.is_ok());

        let _ = result.unwrap();
    }

    #[test]
    fn test_neighbors_with_weights() {
        let storage = DeltaSteppingStorageRuntime::new(0, 1.0, 4, true);

        let neighbors = storage.get_neighbors_with_weights(None, 0, 0).unwrap();
        assert_eq!(neighbors.len(), 2);
        assert_eq!(neighbors[0], (1, 1.0));
        assert_eq!(neighbors[1], (2, 4.0));

        let neighbors_empty = storage.get_neighbors_with_weights(None, 99, 0).unwrap();
        assert!(neighbors_empty.is_empty());
    }

    #[test]
    fn computes_expected_mock_shortest_paths() {
        let mut storage = DeltaSteppingStorageRuntime::new(0, 1.0, 4, true);
        let mut computation = DeltaSteppingComputationRuntime::new(0, 1.0, 4, true);
        let mut progress_tracker =
            TaskProgressTracker::new(Tasks::leaf("delta_stepping".to_string()));

        let result = storage
            .compute_delta_stepping(&mut computation, None, 0, &mut progress_tracker)
            .unwrap();

        let path_to_three = result
            .shortest_paths
            .iter()
            .find(|path| path.target_node == 3)
            .expect("expected path to node 3");
        assert_eq!(path_to_three.node_ids, vec![0, 1, 2, 3]);
        assert_eq!(path_to_three.costs, vec![0.0, 1.0, 3.0, 4.0]);
    }

    #[test]
    fn graph_backed_parallel_matches_single_threaded_costs() {
        let config = RandomGraphConfig {
            seed: Some(99),
            node_count: 12,
            relationships: vec![RandomRelationshipConfig::new("REL", 0.6)],
            ..RandomGraphConfig::default()
        };
        let store = DefaultGraphStore::random(&config).unwrap();
        let rel_types = HashSet::new();
        let graph = store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Natural)
            .unwrap();

        let mut single_storage = DeltaSteppingStorageRuntime::new(0, 1.0, 1, true);
        let mut single_computation = DeltaSteppingComputationRuntime::new(0, 1.0, 1, true);
        let mut single_progress =
            TaskProgressTracker::new(Tasks::leaf("delta_stepping".to_string()));
        let single = single_storage
            .compute_delta_stepping(
                &mut single_computation,
                Some(graph.as_ref()),
                0,
                &mut single_progress,
            )
            .unwrap();

        let mut parallel_storage = DeltaSteppingStorageRuntime::new(0, 1.0, 4, true);
        let mut parallel_computation = DeltaSteppingComputationRuntime::new(0, 1.0, 4, true);
        let mut parallel_progress =
            TaskProgressTracker::new(Tasks::leaf("delta_stepping".to_string()));
        let parallel = parallel_storage
            .compute_delta_stepping(
                &mut parallel_computation,
                Some(graph.as_ref()),
                0,
                &mut parallel_progress,
            )
            .unwrap();

        let mut single_costs: Vec<_> = single
            .shortest_paths
            .iter()
            .map(|path| (path.target_node, path.costs.last().copied().unwrap_or(0.0)))
            .collect();
        let mut parallel_costs: Vec<_> = parallel
            .shortest_paths
            .iter()
            .map(|path| (path.target_node, path.costs.last().copied().unwrap_or(0.0)))
            .collect();
        single_costs.sort_by_key(|(target_node, _)| *target_node);
        parallel_costs.sort_by_key(|(target_node, _)| *target_node);

        assert_eq!(parallel_costs, single_costs);
    }

    #[test]
    fn rejects_source_node_outside_graph_hint() {
        let mut storage = DeltaSteppingStorageRuntime::new(101, 1.0, 4, true);
        let mut computation = DeltaSteppingComputationRuntime::new(101, 1.0, 4, true);
        let mut progress_tracker =
            TaskProgressTracker::new(Tasks::leaf("delta_stepping".to_string()));

        let result =
            storage.compute_delta_stepping(&mut computation, None, 0, &mut progress_tracker);

        assert!(matches!(result, Err(AlgorithmError::InvalidGraph(_))));
    }

    #[test]
    fn rejects_negative_or_non_finite_weights() {
        assert!(DeltaSteppingStorageRuntime::validate_edge_weight(0, 1, -1.0).is_err());
        assert!(DeltaSteppingStorageRuntime::validate_edge_weight(0, 1, f64::NAN).is_err());
        assert!(DeltaSteppingStorageRuntime::validate_edge_weight(0, 1, f64::INFINITY).is_err());
        assert!(DeltaSteppingStorageRuntime::validate_edge_weight(0, 1, 0.0).is_ok());
    }
}
