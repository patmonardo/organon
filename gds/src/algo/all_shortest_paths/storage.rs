//! All Shortest Paths Storage Runtime
//!
//! This module implements the **Gross pole** of the Functor machinery for All Shortest Paths.
//! It represents persistent data structures (Graph view and graph topology).
//!
//! **Translation Source**: `org.neo4j.gds.allshortestpaths.MSBFSAllShortestPaths` and `WeightedAllShortestPaths`
//! **Key Features**: Multi-source parallelization, weighted/unweighted support, streaming results

use crate::algo::msbfs::{AggregatedNeighborProcessingMsBfs, OMEGA};
use crate::concurrency::{install_with_concurrency, Concurrency, TerminationFlag};
use crate::core::utils::progress::ProgressTracker;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::graph::Graph;
use crate::types::graph::NodeId;
use parking_lot::Mutex;
use rayon::prelude::*;
use std::sync::mpsc;
use std::sync::Arc;

use super::AllShortestPathsComputationRuntime;

/// Algorithm type for All Shortest Paths
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AlgorithmType {
    /// Unweighted Multi-Source BFS (MSBFS)
    Unweighted,
    /// Weighted Multi-Source Dijkstra
    Weighted,
}

/// Storage Runtime for All Shortest Paths
///
/// This is the **Gross pole** - persistent data structures.
/// It knows how to access the graph structure and compute shortest paths.
///
/// ## The Pole's Role
///
/// In the Functor machinery:
/// - **Storage Runtime** (Gross) = persistent GraphStore and graph topology
/// - **Computation Runtime** (Subtle) = ephemeral shortest path results
/// - **Functor** = the mapping between them via shortest path computation
pub struct AllShortestPathsStorageRuntime<'a> {
    /// Graph view to traverse
    graph: &'a dyn Graph,
    /// Algorithm type (weighted vs unweighted)
    algorithm_type: AlgorithmType,
    /// Number of parallel workers
    concurrency: usize,
}

impl<'a> AllShortestPathsStorageRuntime<'a> {
    /// Create with specific settings
    pub fn with_settings(
        graph: &'a dyn Graph,
        algorithm_type: AlgorithmType,
        concurrency: usize,
    ) -> Self {
        Self {
            graph,
            algorithm_type,
            concurrency,
        }
    }

    /// Compute shortest paths from a source node
    ///
    /// This projects from Graph (Gross - persistent topology)
    /// to shortest path results (Subtle - path distances).
    ///
    /// **This is where the Functor machinery actually works**:
    /// Graph (Gross) → ShortestPathResults (Subtle)
    ///
    /// **Translation of Java logic**:
    /// - Unweighted: Multi-Source BFS using MSBFS
    /// - Weighted: Multi-Source Dijkstra with priority queue
    pub fn compute_shortest_paths(
        &self,
        source_node: NodeId,
        direction: u8,
    ) -> Result<Vec<ShortestPathResult>, AlgorithmError> {
        match self.algorithm_type {
            AlgorithmType::Unweighted => {
                self.compute_unweighted_shortest_paths(source_node, direction)
            }
            AlgorithmType::Weighted => self.compute_weighted_shortest_paths(source_node, direction),
        }
    }

    /// Compute unweighted shortest paths using BFS
    fn compute_unweighted_shortest_paths(
        &self,
        source_node: NodeId,
        direction: u8,
    ) -> Result<Vec<ShortestPathResult>, AlgorithmError> {
        let node_count = self.graph.node_count();
        let source_index = usize::try_from(source_node).map_err(|_| {
            AlgorithmError::InvalidGraph(format!("Invalid source node id: {source_node}"))
        })?;
        if source_index >= node_count {
            return Err(AlgorithmError::InvalidGraph(format!(
                "Source node id out of range: {source_node} (node_count={node_count})"
            )));
        }
        let mut distances = vec![f64::INFINITY; node_count];
        let mut queue = std::collections::VecDeque::new();

        // Initialize BFS
        distances[source_index] = 0.0;
        queue.push_back(source_node);

        // BFS traversal
        while let Some(current_node) = queue.pop_front() {
            let current_index = usize::try_from(current_node).map_err(|_| {
                AlgorithmError::InvalidGraph(format!("Invalid node id: {current_node}"))
            })?;
            let current_distance = distances[current_index];

            for neighbor in self.get_neighbors(current_node, direction) {
                let neighbor_index = usize::try_from(neighbor).map_err(|_| {
                    AlgorithmError::InvalidGraph(format!("Invalid neighbor id: {neighbor}"))
                })?;
                if neighbor_index >= node_count {
                    continue;
                }
                if distances[neighbor_index] == f64::INFINITY {
                    distances[neighbor_index] = current_distance + 1.0;
                    queue.push_back(neighbor);
                }
            }
        }

        // Convert to results
        let results = distances
            .into_iter()
            .enumerate()
            .filter(|(target, distance)| *target != source_index && *distance != f64::INFINITY)
            .map(|(target, distance)| ShortestPathResult {
                source: source_node,
                target: target as NodeId,
                distance,
            })
            .collect();

        Ok(results)
    }

    /// Compute weighted shortest paths using Dijkstra
    fn compute_weighted_shortest_paths(
        &self,
        source_node: NodeId,
        direction: u8,
    ) -> Result<Vec<ShortestPathResult>, AlgorithmError> {
        if !self.graph.has_relationship_property() {
            return Err(AlgorithmError::Execution(
                "WeightedAllShortestPaths is not supported on graphs without a weight property"
                    .to_string(),
            ));
        }

        use std::cmp::Ordering;
        use std::collections::BinaryHeap;

        #[derive(Debug, Clone, Copy)]
        struct State {
            cost: f64,
            node: NodeId,
        }

        impl PartialEq for State {
            fn eq(&self, other: &Self) -> bool {
                self.cost == other.cost && self.node == other.node
            }
        }

        impl Eq for State {}

        impl PartialOrd for State {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for State {
            fn cmp(&self, other: &Self) -> Ordering {
                // Reverse for min-heap behavior.
                match other.cost.partial_cmp(&self.cost) {
                    Some(ord) => ord,
                    None => other.node.cmp(&self.node),
                }
            }
        }

        let node_count = self.graph.node_count();
        let source_index = usize::try_from(source_node).map_err(|_| {
            AlgorithmError::InvalidGraph(format!("Invalid source node id: {source_node}"))
        })?;
        if source_index >= node_count {
            return Err(AlgorithmError::InvalidGraph(format!(
                "Source node id out of range: {source_node} (node_count={node_count})"
            )));
        }

        let mut distances = vec![f64::INFINITY; node_count];
        distances[source_index] = 0.0;

        let mut heap = BinaryHeap::new();
        heap.push(State {
            cost: 0.0,
            node: source_node,
        });

        while let Some(State { cost, node }) = heap.pop() {
            let node_index = match usize::try_from(node) {
                Ok(idx) => idx,
                Err(_) => continue,
            };
            if node_index >= node_count {
                continue;
            }

            // Stale queue entry.
            if cost > distances[node_index] {
                continue;
            }

            for (neighbor, weight) in self.get_neighbors_with_weights(node, direction) {
                let neighbor_index = match usize::try_from(neighbor) {
                    Ok(idx) => idx,
                    Err(_) => continue,
                };
                if neighbor_index >= node_count {
                    continue;
                }

                let next_cost = cost + weight;
                if next_cost < distances[neighbor_index] {
                    distances[neighbor_index] = next_cost;
                    heap.push(State {
                        cost: next_cost,
                        node: neighbor,
                    });
                }
            }
        }

        // Convert to results
        let results = distances
            .into_iter()
            .enumerate()
            .filter(|(_, distance)| *distance != f64::INFINITY)
            .map(|(target, distance)| ShortestPathResult {
                source: source_node,
                target: target as NodeId,
                distance,
            })
            .collect();

        Ok(results)
    }

    fn get_neighbors(&self, node_id: NodeId, direction: u8) -> Vec<NodeId> {
        let fallback: f64 = 1.0;
        match direction {
            1 => self
                .graph
                .stream_inverse_relationships(node_id, fallback)
                .map(|cursor| cursor.target_id())
                .collect(),
            2 => {
                let mut out: Vec<NodeId> = self
                    .graph
                    .stream_relationships(node_id, fallback)
                    .map(|cursor| cursor.target_id())
                    .collect();
                out.extend(
                    self.graph
                        .stream_inverse_relationships(node_id, fallback)
                        .map(|cursor| cursor.target_id()),
                );
                out
            }
            _ => self
                .graph
                .stream_relationships(node_id, fallback)
                .map(|cursor| cursor.target_id())
                .collect(),
        }
    }

    fn get_neighbors_with_weights(&self, node_id: NodeId, direction: u8) -> Vec<(NodeId, f64)> {
        let fallback: f64 = 1.0;
        match direction {
            1 => self
                .graph
                .stream_inverse_relationships_weighted(node_id, fallback)
                .map(|cursor| (cursor.target_id(), cursor.weight()))
                .collect(),
            2 => {
                let mut out: Vec<(NodeId, f64)> = self
                    .graph
                    .stream_relationships_weighted(node_id, fallback)
                    .map(|cursor| (cursor.target_id(), cursor.weight()))
                    .collect();
                out.extend(
                    self.graph
                        .stream_inverse_relationships_weighted(node_id, fallback)
                        .map(|cursor| (cursor.target_id(), cursor.weight())),
                );
                out
            }
            _ => self
                .graph
                .stream_relationships_weighted(node_id, fallback)
                .map(|cursor| (cursor.target_id(), cursor.weight()))
                .collect(),
        }
    }

    /// Compute all shortest paths in parallel
    ///
    /// This implements the multi-source parallelization from Java GDS.
    /// Results are streamed to avoid O(V²) memory usage.
    ///
    /// Note: This is a simplified version that doesn't use threading
    /// to avoid lifetime issues. In a real implementation, we would
    /// need to handle the GraphStore lifetime properly.
    pub fn compute_all_shortest_paths_streaming(
        &self,
        computation: &mut AllShortestPathsComputationRuntime,
        direction: u8,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<mpsc::Receiver<ShortestPathResult>, AlgorithmError> {
        let node_count = self.graph.node_count();
        progress_tracker.begin_subtask_with_volume(node_count);

        let result = (|| {
            let (sender, receiver) = mpsc::channel::<ShortestPathResult>();

            match self.algorithm_type {
                AlgorithmType::Unweighted => {
                    self.compute_unweighted_all_shortest_paths_streaming(
                        computation,
                        direction,
                        progress_tracker,
                        sender,
                    )?;
                }
                AlgorithmType::Weighted => {
                    self.compute_weighted_all_shortest_paths_streaming(
                        computation,
                        direction,
                        progress_tracker,
                        sender,
                    )?;
                }
            }

            Ok(receiver)
        })();

        match result {
            Ok(receiver) => {
                progress_tracker.end_subtask();
                Ok(receiver)
            }
            Err(e) => {
                progress_tracker.end_subtask_with_failure();
                Err(e)
            }
        }
    }

    fn compute_unweighted_all_shortest_paths_streaming(
        &self,
        computation: &mut AllShortestPathsComputationRuntime,
        direction: u8,
        progress_tracker: &mut dyn ProgressTracker,
        sender: mpsc::Sender<ShortestPathResult>,
    ) -> Result<(), AlgorithmError> {
        let node_count = self.graph.node_count();
        let mut msbfs = AggregatedNeighborProcessingMsBfs::new(node_count);
        let mut receiver_dropped = false;

        for source_offset in (0..node_count).step_by(OMEGA) {
            if receiver_dropped {
                break;
            }

            let source_len = (source_offset + OMEGA).min(node_count) - source_offset;
            msbfs.run(
                source_offset,
                source_len,
                false,
                |node| {
                    self.get_neighbors(node as NodeId, direction)
                        .into_iter()
                        .filter_map(|neighbor| usize::try_from(neighbor).ok())
                        .collect()
                },
                |target, distance, source_mask| {
                    if receiver_dropped {
                        return;
                    }

                    for source in
                        AggregatedNeighborProcessingMsBfs::iter_sources(source_mask, source_offset)
                    {
                        let result = ShortestPathResult {
                            source: source as NodeId,
                            target: target as NodeId,
                            distance: f64::from(distance),
                        };

                        computation.add_result(result.clone());
                        if sender.send(result).is_err() {
                            receiver_dropped = true;
                            break;
                        }
                    }
                },
            );

            for _ in 0..source_len {
                computation.record_source_processed();
            }
            progress_tracker.log_progress(source_len);
        }

        Ok(())
    }

    fn compute_weighted_all_shortest_paths_streaming(
        &self,
        computation: &mut AllShortestPathsComputationRuntime,
        direction: u8,
        progress_tracker: &mut dyn ProgressTracker,
        sender: mpsc::Sender<ShortestPathResult>,
    ) -> Result<(), AlgorithmError> {
        if !self.graph.has_relationship_property() {
            return Err(AlgorithmError::Execution(
                "WeightedAllShortestPaths is not supported on graphs without a weight property"
                    .to_string(),
            ));
        }

        let node_count = self.graph.node_count();
        let concurrency = self.concurrency.max(1);

        // AtomicUsize counter: mirrors Java's AtomicInteger in WeightedAllShortestPaths.
        // Each worker increments to claim the next source node.
        let counter = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let termination = TerminationFlag::running_true();

        // Shared sender so all workers push into the same channel.
        // Mutex is cheap here — contention is only on send, not on Dijkstra itself.
        let shared_sender = Arc::new(Mutex::new(sender));

        // Shared accumulated computation state — merged after all workers finish.
        // Each worker holds its own local AllShortestPathsComputationRuntime so there
        // is zero lock contention on statistics during the compute phase.
        let local_computations: Arc<Mutex<Vec<AllShortestPathsComputationRuntime>>> =
            Arc::new(Mutex::new(Vec::with_capacity(concurrency)));

        // Shared progress accumulator — progress_tracker is &mut dyn so cannot cross threads;
        // we collect raw counts and flush after the parallel section.
        let progress_counts = Arc::new(std::sync::atomic::AtomicUsize::new(0));

        // Spawn `concurrency` worker tasks, each competing for source nodes via the counter.
        // This mirrors Java's `for (int i = 0; i < concurrency.value(); i++) executorService.submit(new ShortestPathTask())`
        let worker_result: Result<(), AlgorithmError> =
            install_with_concurrency(Concurrency::of(concurrency), || {
                (0..concurrency)
                    .into_par_iter()
                    .map(|_| -> Result<(), AlgorithmError> {
                        // Each worker gets its own thread-local graph copy (mirrors Java's `graph.concurrentCopy()`).
                        let local_graph =
                            crate::types::graph::graph::Graph::concurrent_copy(self.graph);

                        // Worker-local Dijkstra state — reused across sources (reset via fill).
                        let mut distances = vec![f64::INFINITY; node_count];
                        let mut local_computation = AllShortestPathsComputationRuntime::new();

                        loop {
                            if !termination.running() {
                                break;
                            }

                            // Claim the next source node atomically.
                            let source_idx =
                                counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            if source_idx >= node_count {
                                break;
                            }

                            let source_node = source_idx as NodeId;

                            // Reset distances for this source.
                            distances.fill(f64::INFINITY);
                            distances[source_idx] = 0.0;

                            // Min-heap Dijkstra — identical logic to compute_weighted_shortest_paths
                            // but using the worker-local graph copy.
                            {
                                use std::cmp::Ordering;
                                use std::collections::BinaryHeap;

                                #[derive(Debug, Clone, Copy)]
                                struct State {
                                    cost: f64,
                                    node: NodeId,
                                }
                                impl PartialEq for State {
                                    fn eq(&self, other: &Self) -> bool {
                                        self.cost == other.cost && self.node == other.node
                                    }
                                }
                                impl Eq for State {}
                                impl PartialOrd for State {
                                    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                                        Some(self.cmp(other))
                                    }
                                }
                                impl Ord for State {
                                    fn cmp(&self, other: &Self) -> Ordering {
                                        match other.cost.partial_cmp(&self.cost) {
                                            Some(ord) => ord,
                                            None => other.node.cmp(&self.node),
                                        }
                                    }
                                }

                                let mut heap = BinaryHeap::new();
                                heap.push(State {
                                    cost: 0.0,
                                    node: source_node,
                                });

                                let fallback = 1.0f64;
                                while let Some(State { cost, node }) = heap.pop() {
                                    let node_idx = node as usize;
                                    if node_idx >= node_count {
                                        continue;
                                    }
                                    if cost > distances[node_idx] {
                                        continue;
                                    }

                                    let neighbors: Vec<(NodeId, f64)> = match direction {
                                        1 => local_graph
                                            .stream_inverse_relationships_weighted(node, fallback)
                                            .map(|c| (c.target_id(), c.weight()))
                                            .collect(),
                                        2 => {
                                            let mut v: Vec<(NodeId, f64)> = local_graph
                                                .stream_relationships_weighted(node, fallback)
                                                .map(|c| (c.target_id(), c.weight()))
                                                .collect();
                                            v.extend(
                                                local_graph
                                                    .stream_inverse_relationships_weighted(
                                                        node, fallback,
                                                    )
                                                    .map(|c| (c.target_id(), c.weight())),
                                            );
                                            v
                                        }
                                        _ => local_graph
                                            .stream_relationships_weighted(node, fallback)
                                            .map(|c| (c.target_id(), c.weight()))
                                            .collect(),
                                    };

                                    for (neighbor, weight) in neighbors {
                                        let nb_idx = neighbor as usize;
                                        if nb_idx >= node_count {
                                            continue;
                                        }
                                        let next_cost = cost + weight;
                                        if next_cost < distances[nb_idx] {
                                            distances[nb_idx] = next_cost;
                                            heap.push(State {
                                                cost: next_cost,
                                                node: neighbor,
                                            });
                                        }
                                    }
                                }
                            }

                            // Emit results — filter Infinity (Java parity: stream filters POSITIVE_INFINITY).
                            let sender_guard = shared_sender.lock();
                            for (target_idx, &dist) in distances.iter().enumerate() {
                                if dist == f64::INFINITY {
                                    continue;
                                }
                                let result = ShortestPathResult {
                                    source: source_node,
                                    target: target_idx as NodeId,
                                    distance: dist,
                                };
                                local_computation.add_result(result.clone());
                                // Channel closed means downstream consumer stopped — not an error.
                                let _ = sender_guard.send(result);
                            }
                            drop(sender_guard);

                            local_computation.record_source_processed();
                            progress_counts.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        }

                        local_computations.lock().push(local_computation);
                        Ok(())
                    })
                    .reduce(|| Ok(()), |acc, res| acc.and(res))
            });

        worker_result?;

        // Merge worker-local computation state back into the caller's mutable runtime.
        let computations = Arc::try_unwrap(local_computations)
            .map(|mutex| mutex.into_inner())
            .unwrap_or_else(|arc| std::mem::take(&mut *arc.lock()));
        for local in computations {
            computation.merge_from(local);
        }

        // Flush accumulated progress to the tracker now that we're back on the single-threaded path.
        let total = progress_counts.load(std::sync::atomic::Ordering::Relaxed);
        if total > 0 {
            progress_tracker.log_progress(total);
        }

        Ok(())
    }

    /// Get total number of nodes
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    /// Get algorithm type
    pub fn algorithm_type(&self) -> AlgorithmType {
        self.algorithm_type
    }

    /// Get concurrency setting
    pub fn concurrency(&self) -> usize {
        self.concurrency
    }
}

/// Result of a shortest path computation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShortestPathResult {
    /// Source node ID
    pub source: NodeId,
    /// Target node ID
    pub target: NodeId,
    /// Shortest path distance
    pub distance: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GraphStoreConfig;
    use crate::core::utils::progress::{TaskProgressTracker, Tasks};
    use crate::projection::RelationshipType;
    use crate::types::graph::{
        DefaultGraph, GraphCharacteristicsBuilder, RelationshipTopology, SimpleIdMap,
    };
    use crate::types::schema::GraphSchema;
    use std::collections::{HashMap, HashSet};
    use std::sync::Arc;

    fn graph_from_outgoing(outgoing: Vec<Vec<NodeId>>) -> DefaultGraph {
        let schema = Arc::new(GraphSchema::empty());
        let node_count = outgoing.len();
        let id_map = Arc::new(SimpleIdMap::from_original_ids(
            (0..node_count as NodeId).collect::<Vec<_>>(),
        ));

        let topology = RelationshipTopology::new(outgoing, None);
        let relationship_count = topology.relationship_count();
        let has_parallel_edges = topology.has_parallel_edges();

        let rel_type = RelationshipType::of("REL");
        let mut topologies = HashMap::new();
        topologies.insert(rel_type.clone(), Arc::new(topology));

        DefaultGraph::new(
            Arc::new(GraphStoreConfig::default()),
            schema,
            id_map,
            GraphCharacteristicsBuilder::new().directed().build(),
            topologies,
            vec![rel_type],
            HashSet::new(),
            relationship_count,
            has_parallel_edges,
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
        )
    }

    #[test]
    fn unweighted_streaming_uses_reachable_non_self_pairs() {
        let graph = graph_from_outgoing(vec![vec![1], vec![2], vec![], vec![]]);
        let storage =
            AllShortestPathsStorageRuntime::with_settings(&graph, AlgorithmType::Unweighted, 1);
        let mut computation = AllShortestPathsComputationRuntime::new();
        let mut progress_tracker =
            TaskProgressTracker::new(Tasks::leaf("all_shortest_paths".to_string()));

        let receiver = storage
            .compute_all_shortest_paths_streaming(&mut computation, 0, &mut progress_tracker)
            .unwrap();
        let mut rows: Vec<_> = receiver.into_iter().collect();
        rows.sort_by(|a, b| (a.source, a.target).cmp(&(b.source, b.target)));

        assert_eq!(computation.node_count(), 4);
        assert_eq!(computation.infinite_distances(), 0);
        assert!(rows.iter().all(|row| row.source != row.target));
        assert_eq!(
            rows.iter()
                .map(|row| (row.source, row.target, row.distance))
                .collect::<Vec<_>>(),
            vec![(0, 1, 1.0), (0, 2, 2.0), (1, 2, 1.0)]
        );
    }

    #[test]
    fn weighted_parallel_matches_sequential() {
        use crate::types::graph_store::GraphStore as _;
        use crate::types::graph_store::{
            Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName,
        };
        use crate::types::properties::relationship::{
            DefaultRelationshipPropertyValues, RelationshipPropertyValues,
        };
        use crate::types::schema::Direction;

        // 4-node chain: 0→1 (w=1.0), 1→2 (w=2.0), 2→3 (w=3.0), plus 0→3 (w=10.0)
        let outgoing: Vec<Vec<NodeId>> = vec![vec![1, 3], vec![2], vec![3], vec![]];
        let incoming: Vec<Vec<NodeId>> = vec![vec![], vec![0], vec![1], vec![2, 0]];

        let rel_type = RelationshipType::of("REL");
        let mut schema_builder = crate::types::schema::MutableGraphSchema::empty();
        schema_builder
            .relationship_schema_mut()
            .add_relationship_type(rel_type.clone(), Direction::Directed);
        let schema = schema_builder.build();

        let mut topologies = HashMap::new();
        topologies.insert(
            rel_type.clone(),
            RelationshipTopology::new(outgoing, Some(incoming)),
        );

        let original_ids: Vec<NodeId> = (0..4i64).collect();
        let id_map = SimpleIdMap::from_original_ids(original_ids);

        let mut store = DefaultGraphStore::new(
            GraphStoreConfig::default(),
            GraphName::new("g"),
            DatabaseInfo::new(
                DatabaseId::new("db"),
                DatabaseLocation::remote("localhost", 7687, None, None),
            ),
            schema,
            Capabilities::default(),
            id_map,
            topologies,
        );

        // weights order: edges in topology order — 0→1, 0→3, 1→2, 2→3
        let weights = vec![1.0f64, 10.0, 2.0, 3.0];
        let values: Arc<dyn RelationshipPropertyValues> = Arc::new(
            DefaultRelationshipPropertyValues::with_values(weights, 0.0, 4),
        );
        store
            .add_relationship_property(rel_type.clone(), "weight", values)
            .unwrap();

        let store = Arc::new(store);

        let run = |concurrency: usize| -> Vec<(NodeId, NodeId, f64)> {
            let rel_types: HashSet<RelationshipType> = store.relationship_types();
            let selectors: HashMap<RelationshipType, String> = rel_types
                .iter()
                .map(|t| (t.clone(), "weight".to_string()))
                .collect();
            let graph_view = store
                .get_graph_with_types_selectors_and_orientation(
                    &rel_types,
                    &selectors,
                    crate::projection::Orientation::Natural,
                )
                .unwrap();

            let storage = AllShortestPathsStorageRuntime::with_settings(
                graph_view.as_ref(),
                AlgorithmType::Weighted,
                concurrency,
            );
            let mut computation = AllShortestPathsComputationRuntime::new();
            let mut tracker = TaskProgressTracker::new(Tasks::leaf("asp".to_string()));

            let receiver = storage
                .compute_all_shortest_paths_streaming(&mut computation, 0, &mut tracker)
                .unwrap();

            let mut rows: Vec<_> = receiver
                .into_iter()
                .map(|r| (r.source, r.target, r.distance))
                .collect();
            rows.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            rows
        };

        let sequential = run(1);
        let parallel = run(4);

        assert!(
            !sequential.is_empty(),
            "weighted ASP should produce results"
        );
        assert_eq!(
            sequential, parallel,
            "parallel result must match sequential"
        );
    }

    #[test]
    fn weighted_streaming_requires_relationship_property() {
        let graph = graph_from_outgoing(vec![vec![1], vec![]]);
        let storage =
            AllShortestPathsStorageRuntime::with_settings(&graph, AlgorithmType::Weighted, 1);
        let mut computation = AllShortestPathsComputationRuntime::new();
        let mut progress_tracker =
            TaskProgressTracker::new(Tasks::leaf("all_shortest_paths".to_string()));

        let result = storage.compute_all_shortest_paths_streaming(
            &mut computation,
            0,
            &mut progress_tracker,
        );

        assert!(matches!(result, Err(AlgorithmError::Execution(_))));
    }
}
