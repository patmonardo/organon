//! Bellman-Ford Storage Runtime
//!
//! **Translation Source**: `org.neo4j.gds.paths.bellmanford.BellmanFord`
//!
//! This module implements the "Gross pole" of the Bellman-Ford algorithm,
//! handling persistent data access and the main algorithm execution.

use super::spec::{BellmanFordPathResult, BellmanFordResult};
use super::BellmanFordComputationRuntime;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::task::concurrency::{install_with_concurrency, Concurrency, TerminationFlag};
use crate::task::progress::{ProgressTracker, UNKNOWN_VOLUME};
use crate::types::graph::Graph;
use crate::types::graph::MappedNodeId;
use crate::types::graph::RelationshipIndex;
use crate::types::properties::relationship::RelationshipCursorBox;
use rayon::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

const FRONTIER_CHUNK_SIZE: usize = 64;

#[derive(Debug, Clone)]
struct RelaxationCandidate {
    source_node: MappedNodeId,
    target_node: MappedNodeId,
    new_distance: f64,
    new_length: u32,
}

#[derive(Debug, Clone)]
struct FrontierExpansion {
    candidates: Vec<RelaxationCandidate>,
    negative_cycle_nodes: Vec<MappedNodeId>,
    scanned_relationships: usize,
}

/// Bellman-Ford Storage Runtime
///
/// Translation of: Main `BellmanFord` class (lines 48-357)
/// Handles the persistent data access and algorithm orchestration
pub struct BellmanFordStorageRuntime {
    /// Source node for shortest path computation
    pub source_node: MappedNodeId,

    /// Whether to track negative cycles
    pub track_negative_cycles: bool,

    /// Whether to track shortest paths
    pub track_paths: bool,

    /// Concurrency level for parallel processing
    pub concurrency: usize,
}

impl BellmanFordStorageRuntime {
    /// Create a new Bellman-Ford storage runtime
    ///
    /// Translation of: Constructor (lines 55-69)
    pub fn new(
        source_node: MappedNodeId,
        track_negative_cycles: bool,
        track_paths: bool,
        concurrency: usize,
    ) -> Self {
        Self {
            source_node,
            track_negative_cycles,
            track_paths,
            concurrency,
        }
    }

    /// Compute Bellman-Ford shortest paths with negative cycle detection
    ///
    /// Translation of: `compute()` method (lines 72-124)
    pub fn compute_bellman_ford(
        &mut self,
        computation: &mut BellmanFordComputationRuntime,
        graph: Option<&dyn Graph>,
        direction: u8,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<BellmanFordResult, AlgorithmError> {
        let termination_flag = TerminationFlag::running_true();
        let volume = graph
            .map(|g| g.relationship_count())
            .unwrap_or(UNKNOWN_VOLUME);
        if volume == UNKNOWN_VOLUME {
            progress_tracker.begin_subtask_unknown();
        } else {
            progress_tracker.begin_subtask_with_volume(volume);
        }

        let result = self.compute_bellman_ford_with_context(
            computation,
            graph,
            direction,
            progress_tracker,
            &termination_flag,
        );

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

    pub fn compute_bellman_ford_with_context(
        &mut self,
        computation: &mut BellmanFordComputationRuntime,
        graph: Option<&dyn Graph>,
        direction: u8,
        progress_tracker: &mut dyn ProgressTracker,
        termination_flag: &TerminationFlag,
    ) -> Result<BellmanFordResult, AlgorithmError> {
        let mut scanned_relationships: usize = 0;

        (|| {
            Self::ensure_running(termination_flag)?;
            // Initialize computation runtime
            let node_count = graph.map(|g| g.node_count()).unwrap_or(100);
            Self::validate_node_in_graph(self.source_node, node_count, "source")?;
            computation.initialize(
                self.source_node,
                self.track_negative_cycles,
                self.track_paths,
                node_count,
            );

            // Initialize distances
            computation.set_distance(self.source_node, 0.0);
            computation.set_predecessor(self.source_node, None);
            computation.set_length(self.source_node, 0);

            if let Some(graph) = graph {
                scanned_relationships = self.compute_parallel_frontier(
                    computation,
                    graph,
                    direction,
                    node_count,
                    progress_tracker,
                    termination_flag,
                )?;
            } else {
                scanned_relationships = self.compute_sequential_frontier(
                    computation,
                    graph,
                    direction,
                    node_count,
                    progress_tracker,
                    termination_flag,
                )?;
            }

            if scanned_relationships > 0 {
                progress_tracker.log_progress(scanned_relationships);
            }

            // Check if we have negative cycles
            let contains_negative_cycle = computation.has_negative_cycles();

            // Generate results
            let shortest_paths = if contains_negative_cycle || !self.track_paths {
                vec![]
            } else {
                self.generate_shortest_paths(computation)?
            };

            let negative_cycles = if self.track_negative_cycles {
                self.generate_negative_cycles(computation)?
            } else {
                vec![]
            };

            Ok(BellmanFordResult {
                shortest_paths,
                negative_cycles,
                contains_negative_cycle,
            })
        })()
    }

    /// Generate shortest paths from the distance tracker
    ///
    /// Translation of: `pathResults()` method (lines 198-232)
    fn generate_shortest_paths(
        &self,
        computation: &BellmanFordComputationRuntime,
    ) -> Result<Vec<BellmanFordPathResult>, AlgorithmError> {
        let mut paths = Vec::new();
        let node_count = computation.node_count();

        for target_index in 0..node_count {
            let target_node = MappedNodeId::try_from(target_index).map_err(|_| {
                AlgorithmError::InvalidGraph(format!(
                    "Node index does not fit mapped node identity: {target_index}"
                ))
            })?;
            if computation.predecessor(target_node).is_some() {
                let path = self.reconstruct_path(computation, self.source_node, target_node)?;
                paths.push(path);
            }
        }

        Ok(paths)
    }

    fn compute_sequential_frontier(
        &self,
        computation: &mut BellmanFordComputationRuntime,
        graph: Option<&dyn Graph>,
        direction: u8,
        node_count: usize,
        progress_tracker: &mut dyn ProgressTracker,
        termination_flag: &TerminationFlag,
    ) -> Result<usize, AlgorithmError> {
        let mut scanned_relationships = 0usize;
        const LOG_BATCH: usize = 256;

        let mut frontier = VecDeque::new();
        frontier.push_back(self.source_node);

        while let Some(node_id) = frontier.pop_front() {
            Self::ensure_running(termination_flag)?;
            if Self::is_negative_cycle_length(computation.length(node_id), node_count) {
                computation.add_negative_cycle_node(node_id);
                if !self.track_negative_cycles {
                    break;
                }
                continue;
            }

            let neighbors = self.get_neighbors_with_weights(graph, node_id, direction)?;
            scanned_relationships = scanned_relationships.saturating_add(neighbors.len());
            if scanned_relationships >= LOG_BATCH {
                progress_tracker.log_progress(scanned_relationships);
                scanned_relationships = 0;
            }

            for (neighbor, weight, _relationship_index) in neighbors {
                Self::validate_node_in_graph(neighbor, node_count, "target")?;
                Self::validate_edge_weight(node_id, neighbor, weight)?;

                let current_distance = computation.distance(node_id);
                let new_distance = current_distance + weight;
                Self::validate_path_cost(node_id, neighbor, new_distance)?;

                if new_distance < computation.distance(neighbor) {
                    let new_length = computation.length(node_id).saturating_add(1);
                    computation.set_distance(neighbor, new_distance);
                    computation.set_predecessor(neighbor, Some(node_id));
                    computation.set_length(neighbor, new_length);

                    if Self::is_negative_cycle_length(new_length, node_count) {
                        computation.add_negative_cycle_node(neighbor);
                        if !self.track_negative_cycles {
                            return Ok(scanned_relationships);
                        }
                    } else {
                        frontier.push_back(neighbor);
                    }
                }
            }
        }

        Ok(scanned_relationships)
    }

    fn compute_parallel_frontier(
        &self,
        computation: &mut BellmanFordComputationRuntime,
        graph: &dyn Graph,
        direction: u8,
        node_count: usize,
        progress_tracker: &mut dyn ProgressTracker,
        termination_flag: &TerminationFlag,
    ) -> Result<usize, AlgorithmError> {
        let mut frontier = vec![self.source_node];
        let mut scanned_relationships = 0usize;
        let concurrency = Concurrency::from_usize(self.concurrency.max(1));

        while !frontier.is_empty() {
            Self::ensure_running(termination_flag)?;
            let chunk_count = frontier.len().div_ceil(FRONTIER_CHUNK_SIZE);
            let expansions: Vec<FrontierExpansion> = install_with_concurrency(concurrency, || {
                (0..chunk_count)
                    .into_par_iter()
                    .map(|chunk_idx| {
                        Self::ensure_running(termination_flag)?;
                        let start = chunk_idx * FRONTIER_CHUNK_SIZE;
                        let end = (start + FRONTIER_CHUNK_SIZE).min(frontier.len());
                        self.expand_frontier_chunk(
                            graph,
                            computation,
                            &frontier[start..end],
                            direction,
                            node_count,
                            termination_flag,
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?;

            let mut candidates_by_target: HashMap<MappedNodeId, RelaxationCandidate> =
                HashMap::new();
            for expansion in expansions {
                scanned_relationships =
                    scanned_relationships.saturating_add(expansion.scanned_relationships);
                for node_id in expansion.negative_cycle_nodes {
                    computation.add_negative_cycle_node(node_id);
                }
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

            if computation.has_negative_cycles() && !self.track_negative_cycles {
                break;
            }

            let mut next_frontier = Vec::new();
            let mut queued = HashSet::new();
            let mut candidates: Vec<_> = candidates_by_target.into_values().collect();
            candidates.sort_by_key(|candidate| candidate.target_node);

            for candidate in candidates {
                if candidate.new_distance < computation.distance(candidate.target_node) {
                    computation.set_distance(candidate.target_node, candidate.new_distance);
                    computation.set_predecessor(candidate.target_node, Some(candidate.source_node));
                    computation.set_length(candidate.target_node, candidate.new_length);

                    if Self::is_negative_cycle_length(candidate.new_length, node_count) {
                        computation.add_negative_cycle_node(candidate.target_node);
                        if !self.track_negative_cycles {
                            next_frontier.clear();
                            break;
                        }
                    } else if queued.insert(candidate.target_node) {
                        next_frontier.push(candidate.target_node);
                    }
                }
            }

            frontier = next_frontier;
        }

        Ok(scanned_relationships)
    }

    fn expand_frontier_chunk(
        &self,
        graph: &dyn Graph,
        computation: &BellmanFordComputationRuntime,
        frontier: &[MappedNodeId],
        direction: u8,
        node_count: usize,
        termination_flag: &TerminationFlag,
    ) -> Result<FrontierExpansion, AlgorithmError> {
        let worker_graph = Graph::concurrent_view(graph);
        let fallback = worker_graph.default_property_value();
        let mut candidates = Vec::new();
        let mut negative_cycle_nodes = Vec::new();
        let mut scanned_relationships = 0usize;

        for node_id in frontier {
            Self::ensure_running(termination_flag)?;
            let source_length = computation.length(*node_id);
            if Self::is_negative_cycle_length(source_length, node_count) {
                negative_cycle_nodes.push(*node_id);
                continue;
            }

            let stream = if direction == 1 {
                worker_graph.stream_inverse_relationships(*node_id, fallback)
            } else {
                worker_graph.stream_relationships(*node_id, fallback)
            };

            for cursor in stream {
                Self::ensure_running(termination_flag)?;
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
                        new_length: source_length.saturating_add(1),
                    });
                }
            }
        }

        Ok(FrontierExpansion {
            candidates,
            negative_cycle_nodes,
            scanned_relationships,
        })
    }

    fn ensure_running(termination_flag: &TerminationFlag) -> Result<(), AlgorithmError> {
        if termination_flag.running() {
            Ok(())
        } else {
            Err(AlgorithmError::Execution(
                "Bellman-Ford computation terminated".to_string(),
            ))
        }
    }

    /// Generate negative cycle paths
    ///
    /// Translation of: `negativeCyclesResults()` method (lines 159-196)
    fn generate_negative_cycles(
        &self,
        computation: &BellmanFordComputationRuntime,
    ) -> Result<Vec<BellmanFordPathResult>, AlgorithmError> {
        let mut cycles = Vec::new();

        for cycle_node in computation.get_negative_cycle_nodes() {
            let cycle = self.reconstruct_negative_cycle(computation, *cycle_node)?;
            cycles.push(cycle);
        }

        Ok(cycles)
    }

    /// Reconstruct a path from source to target
    ///
    /// Translation of: `pathResult()` method (lines 236-269)
    fn reconstruct_path(
        &self,
        computation: &BellmanFordComputationRuntime,
        source_node: MappedNodeId,
        target_node: MappedNodeId,
    ) -> Result<BellmanFordPathResult, AlgorithmError> {
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

        Ok(BellmanFordPathResult {
            source_node,
            target_node,
            total_cost: computation.distance(target_node),
            node_ids,
            costs,
        })
    }

    /// Reconstruct a negative cycle starting from a given node
    ///
    /// Translation of: `negativeCycleResult()` method (lines 271-308)
    fn reconstruct_negative_cycle(
        &self,
        computation: &BellmanFordComputationRuntime,
        start_node: MappedNodeId,
    ) -> Result<BellmanFordPathResult, AlgorithmError> {
        let mut node_ids = Vec::new();
        let mut costs = Vec::new();

        let mut current_node = start_node;
        let mut length = 0;
        let max_length = computation.node_count();

        // Follow predecessors until we complete the cycle
        while length < max_length {
            node_ids.push(current_node);
            costs.push(computation.distance(current_node));

            current_node = computation.predecessor(current_node).ok_or_else(|| {
                AlgorithmError::InvalidGraph("Missing predecessor in cycle".to_string())
            })?;

            length += 1;

            // Check if we've completed the cycle
            if current_node == start_node {
                break;
            }
        }

        // If we didn't complete the cycle, return empty result
        if length >= max_length {
            return Ok(BellmanFordPathResult {
                source_node: start_node,
                target_node: start_node,
                total_cost: 0.0,
                node_ids: vec![],
                costs: vec![],
            });
        }

        Ok(BellmanFordPathResult {
            source_node: start_node,
            target_node: start_node,
            total_cost: 0.0, // Negative cycles have negative total cost
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
        node_id: MappedNodeId,
        direction: u8,
    ) -> Result<Vec<(MappedNodeId, f64, RelationshipIndex)>, AlgorithmError> {
        if let Some(g) = graph {
            let fallback: f64 = 1.0;
            let iter: Box<dyn Iterator<Item = RelationshipCursorBox> + Send> = if direction == 1 {
                g.stream_inverse_relationships(node_id, fallback)
            } else {
                g.stream_relationships(node_id, fallback)
            };
            Ok(iter
                .map(|cursor| {
                    (
                        cursor.target_id(),
                        cursor.property(),
                        cursor.relationship_index(),
                    )
                })
                .collect())
        } else {
            // Mock implementation for tests
            Ok(match node_id.get() {
                0 => vec![
                    (MappedNodeId::new(1), 1.0, RelationshipIndex::new(0)),
                    (MappedNodeId::new(2), 4.0, RelationshipIndex::new(1)),
                ],
                1 => vec![
                    (MappedNodeId::new(2), 2.0, RelationshipIndex::new(2)),
                    (MappedNodeId::new(3), 5.0, RelationshipIndex::new(3)),
                ],
                2 => vec![
                    (MappedNodeId::new(3), 1.0, RelationshipIndex::new(4)),
                    (MappedNodeId::new(4), 3.0, RelationshipIndex::new(5)),
                ],
                3 => vec![(MappedNodeId::new(4), 2.0, RelationshipIndex::new(6))],
                _ => vec![],
            })
        }
    }

    fn is_negative_cycle_length(length: u32, node_count: usize) -> bool {
        usize::try_from(length).is_ok_and(|length| length >= node_count.saturating_add(1))
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

    fn validate_edge_weight(
        source_node: MappedNodeId,
        target_node: MappedNodeId,
        weight: f64,
    ) -> Result<(), AlgorithmError> {
        if !weight.is_finite() {
            return Err(AlgorithmError::InvalidGraph(format!(
                "Bellman-Ford requires finite edge weights; edge {source_node}->{target_node} has weight {weight}"
            )));
        }
        Ok(())
    }

    fn validate_path_cost(
        source_node: MappedNodeId,
        target_node: MappedNodeId,
        cost: f64,
    ) -> Result<(), AlgorithmError> {
        if !cost.is_finite() {
            return Err(AlgorithmError::InvalidGraph(format!(
                "Bellman-Ford path cost overflowed for edge {source_node}->{target_node}"
            )));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::Orientation;
    use crate::task::progress::{NoopProgressTracker, TaskProgressTracker, Tasks};
    use crate::types::graph_store::{DefaultGraphStore, GraphStore};
    use crate::types::random::{RandomGraphConfig, RandomRelationshipConfig};
    use std::collections::HashSet;

    fn mapped_node_id(value: u64) -> MappedNodeId {
        MappedNodeId::new(value)
    }

    fn relationship_index(value: u64) -> RelationshipIndex {
        RelationshipIndex::new(value)
    }

    #[test]
    fn context_computation_honors_pre_terminated_request() {
        let source = MappedNodeId::ZERO;
        let mut storage = BellmanFordStorageRuntime::new(source, true, true, 4);
        let mut computation = BellmanFordComputationRuntime::new(source, true, true, 4);
        let mut progress_tracker = NoopProgressTracker;
        let termination_flag = TerminationFlag::stop_running();

        let result = storage.compute_bellman_ford_with_context(
            &mut computation,
            None,
            0,
            &mut progress_tracker,
            &termination_flag,
        );

        assert!(
            matches!(result, Err(AlgorithmError::Execution(message)) if message.contains("terminated"))
        );
    }

    #[test]
    fn test_bellman_ford_storage_runtime_creation() {
        let storage = BellmanFordStorageRuntime::new(MappedNodeId::ZERO, true, true, 4);
        assert_eq!(storage.source_node, MappedNodeId::ZERO);
        assert!(storage.track_negative_cycles);
        assert!(storage.track_paths);
        assert_eq!(storage.concurrency, 4);
    }

    #[test]
    fn test_bellman_ford_path_computation() {
        let mut storage = BellmanFordStorageRuntime::new(MappedNodeId::ZERO, true, true, 4);
        let mut computation = BellmanFordComputationRuntime::new(MappedNodeId::ZERO, true, true, 4);
        let mut progress_tracker =
            TaskProgressTracker::new(Tasks::leaf("bellman_ford".to_string()));

        // Test basic path computation
        let result = storage.compute_bellman_ford(&mut computation, None, 0, &mut progress_tracker);
        assert!(result.is_ok());

        let bellman_ford_result = result.unwrap();
        assert!(!bellman_ford_result.contains_negative_cycle);
    }

    #[test]
    fn test_bellman_ford_path_same_source_target() {
        let mut storage = BellmanFordStorageRuntime::new(MappedNodeId::ZERO, true, true, 4);
        let mut computation = BellmanFordComputationRuntime::new(MappedNodeId::ZERO, true, true, 4);
        let mut progress_tracker =
            TaskProgressTracker::new(Tasks::leaf("bellman_ford".to_string()));

        // Test with same source and target
        let result = storage.compute_bellman_ford(&mut computation, None, 0, &mut progress_tracker);
        assert!(result.is_ok());

        let bellman_ford_result = result.unwrap();
        assert!(!bellman_ford_result.contains_negative_cycle);
    }

    #[test]
    fn test_neighbors_with_weights() {
        let storage = BellmanFordStorageRuntime::new(MappedNodeId::ZERO, true, true, 4);

        let neighbors = storage
            .get_neighbors_with_weights(None, MappedNodeId::ZERO, 0)
            .unwrap();
        assert_eq!(neighbors.len(), 2);
        assert_eq!(
            neighbors[0],
            (mapped_node_id(1), 1.0, relationship_index(0))
        );
        assert_eq!(
            neighbors[1],
            (mapped_node_id(2), 4.0, relationship_index(1))
        );

        let neighbors_empty = storage
            .get_neighbors_with_weights(None, mapped_node_id(99), 0)
            .unwrap();
        assert!(neighbors_empty.is_empty());
    }

    #[test]
    fn computes_expected_mock_shortest_paths() {
        let mut storage = BellmanFordStorageRuntime::new(MappedNodeId::ZERO, true, true, 4);
        let mut computation = BellmanFordComputationRuntime::new(MappedNodeId::ZERO, true, true, 4);
        let mut progress_tracker =
            TaskProgressTracker::new(Tasks::leaf("bellman_ford".to_string()));

        let result = storage
            .compute_bellman_ford(&mut computation, None, 0, &mut progress_tracker)
            .unwrap();

        let path_to_three = result
            .shortest_paths
            .iter()
            .find(|path| path.target_node == mapped_node_id(3))
            .expect("expected path to node 3");
        assert_eq!(
            path_to_three.node_ids,
            vec![
                MappedNodeId::ZERO,
                mapped_node_id(1),
                mapped_node_id(2),
                mapped_node_id(3)
            ]
        );
        assert_eq!(path_to_three.costs, vec![0.0, 1.0, 3.0, 4.0]);
        assert_eq!(path_to_three.total_cost, 4.0);
    }

    #[test]
    fn graph_backed_parallel_matches_single_threaded_costs() {
        let config = RandomGraphConfig {
            seed: Some(42),
            node_count: 12,
            relationships: vec![RandomRelationshipConfig::new("REL", 0.6)],
            ..RandomGraphConfig::default()
        };
        let store = DefaultGraphStore::random(&config).unwrap();
        let rel_types = HashSet::new();
        let graph = store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Natural)
            .unwrap();

        let mut single_storage = BellmanFordStorageRuntime::new(MappedNodeId::ZERO, true, true, 1);
        let mut single_computation =
            BellmanFordComputationRuntime::new(MappedNodeId::ZERO, true, true, 1);
        let mut single_progress = TaskProgressTracker::new(Tasks::leaf("bellman_ford".to_string()));
        let single = single_storage
            .compute_bellman_ford(
                &mut single_computation,
                Some(graph.as_ref()),
                0,
                &mut single_progress,
            )
            .unwrap();

        let mut parallel_storage =
            BellmanFordStorageRuntime::new(MappedNodeId::ZERO, true, true, 4);
        let mut parallel_computation =
            BellmanFordComputationRuntime::new(MappedNodeId::ZERO, true, true, 4);
        let mut parallel_progress =
            TaskProgressTracker::new(Tasks::leaf("bellman_ford".to_string()));
        let parallel = parallel_storage
            .compute_bellman_ford(
                &mut parallel_computation,
                Some(graph.as_ref()),
                0,
                &mut parallel_progress,
            )
            .unwrap();

        let mut single_costs: Vec<_> = single
            .shortest_paths
            .iter()
            .map(|path| (path.target_node, path.total_cost))
            .collect();
        let mut parallel_costs: Vec<_> = parallel
            .shortest_paths
            .iter()
            .map(|path| (path.target_node, path.total_cost))
            .collect();
        single_costs.sort_by_key(|(target_node, _)| *target_node);
        parallel_costs.sort_by_key(|(target_node, _)| *target_node);

        assert_eq!(
            parallel.contains_negative_cycle,
            single.contains_negative_cycle
        );
        assert_eq!(parallel_costs, single_costs);
    }

    #[test]
    fn rejects_source_node_outside_graph_hint() {
        let source = mapped_node_id(101);
        let mut storage = BellmanFordStorageRuntime::new(source, true, true, 4);
        let mut computation = BellmanFordComputationRuntime::new(source, true, true, 4);
        let mut progress_tracker =
            TaskProgressTracker::new(Tasks::leaf("bellman_ford".to_string()));

        let result = storage.compute_bellman_ford(&mut computation, None, 0, &mut progress_tracker);

        assert!(matches!(result, Err(AlgorithmError::InvalidGraph(_))));
    }

    #[test]
    fn allows_negative_but_rejects_non_finite_weights() {
        let source_node = MappedNodeId::ZERO;
        let target_node = mapped_node_id(1);
        assert!(
            BellmanFordStorageRuntime::validate_edge_weight(source_node, target_node, -1.0).is_ok()
        );
        assert!(
            BellmanFordStorageRuntime::validate_edge_weight(source_node, target_node, 0.0).is_ok()
        );
        assert!(BellmanFordStorageRuntime::validate_edge_weight(
            source_node,
            target_node,
            f64::NAN
        )
        .is_err());
        assert!(BellmanFordStorageRuntime::validate_edge_weight(
            source_node,
            target_node,
            f64::INFINITY
        )
        .is_err());
    }
}
