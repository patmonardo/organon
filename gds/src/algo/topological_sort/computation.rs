//! TopologicalSort Computation
//!
//! **Translation Source**: `org.neo4j.gds.dag.topologicalsort.TopologicalSort`
//!
//! Implements Kahn's algorithm for topological sorting with optional longest path calculation.

use super::spec::TopologicalSortResult;
use super::TopologicalSortStorageRuntime;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::task::concurrency::{Concurrency, Executor, TerminatedException, TerminationFlag};
use crate::types::graph::Graph;
use crate::types::graph::MappedNodeId;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

pub struct TopologicalSortComputationRuntime {
    storage: TopologicalSortStorageRuntime,
    compute_max_distance: bool,
}

impl TopologicalSortComputationRuntime {
    pub fn new(node_count: usize, compute_max_distance: bool) -> Self {
        Self {
            storage: TopologicalSortStorageRuntime::new(node_count, compute_max_distance),
            compute_max_distance,
        }
    }

    /// Compute topological sort (sequential)
    pub fn compute(
        &mut self,
        node_count: usize,
        get_neighbors: impl Fn(MappedNodeId) -> Vec<(MappedNodeId, f64)>,
    ) -> std::result::Result<TopologicalSortResult, AlgorithmError> {
        self.storage = TopologicalSortStorageRuntime::new(node_count, self.compute_max_distance);

        // Phase 1: Initialize in-degrees (with validation)
        for node_index in 0..node_count {
            let node_id = mapped_node_id(node_index);
            for (target, weight) in get_neighbors(node_id) {
                let target_index = validate_neighbor(node_count, node_id, target)?;
                if self.compute_max_distance {
                    validate_weight(node_id, target, weight)?;
                }
                self.storage.in_degrees[target_index].fetch_add(1, Ordering::SeqCst);
            }
        }

        // Phase 2: Initialize queue with nodes having in-degree 0
        let mut ready_queue = Vec::new();
        for node_index in 0..node_count {
            let node_id = mapped_node_id(node_index);
            if self.storage.in_degrees[node_index].load(Ordering::SeqCst) == 0 {
                ready_queue.push(node_id);
                // Initialize distance for source nodes
                if let Some(ref distances) = self.storage.max_source_distances {
                    distances[node_index].store(0.0_f64.to_bits(), Ordering::SeqCst);
                }
            }
        }

        // Phase 3: Process nodes sequentially
        while let Some(source) = ready_queue.pop() {
            self.storage.add_node(source);
            let source_index = physical_node_index(source);

            let source_distance = if let Some(ref distances) = self.storage.max_source_distances {
                f64::from_bits(distances[source_index].load(Ordering::SeqCst))
            } else {
                0.0
            };

            for (target, weight) in get_neighbors(source) {
                let target_index = validate_neighbor(node_count, source, target)?;
                // Update longest path distance if computing
                if let Some(ref distances) = self.storage.max_source_distances {
                    loop {
                        let current_bits = distances[target_index].load(Ordering::SeqCst);
                        let current = f64::from_bits(current_bits);
                        let new_distance = source_distance + weight;

                        if new_distance > current {
                            let new_bits = new_distance.to_bits();
                            if distances[target_index]
                                .compare_exchange(
                                    current_bits,
                                    new_bits,
                                    Ordering::SeqCst,
                                    Ordering::SeqCst,
                                )
                                .is_ok()
                            {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                }

                // Decrement in-degree
                let prev_in_degree =
                    self.storage.in_degrees[target_index].fetch_sub(1, Ordering::SeqCst);

                // If in-degree becomes 0, add to queue
                if prev_in_degree == 1 {
                    ready_queue.push(target);
                }
            }
        }

        // Phase 4: Build result
        let size = self.storage.size();
        let mut sorted_nodes = Vec::with_capacity(size);

        for i in 0..size {
            let node = self.storage.sorted_nodes[i].load(Ordering::SeqCst);
            sorted_nodes.push(MappedNodeId::new(node));
        }

        let max_source_distances = self.storage.max_source_distances.as_ref().map(|distances| {
            (0..node_count)
                .map(|i| f64::from_bits(distances[i].load(Ordering::SeqCst)))
                .collect()
        });

        Ok(TopologicalSortResult {
            sorted_nodes,
            max_source_distances,
        })
    }

    pub fn compute_with_concurrency(
        &mut self,
        node_count: usize,
        concurrency: usize,
        termination: &TerminationFlag,
        get_neighbors: impl Fn(MappedNodeId) -> Vec<(MappedNodeId, f64)> + Send + Sync + 'static,
    ) -> std::result::Result<TopologicalSortResult, AlgorithmError> {
        self.storage = TopologicalSortStorageRuntime::new(node_count, self.compute_max_distance);
        let get_neighbors = Arc::new(get_neighbors);

        // Phase 1: Initialize in-degrees (with validation)
        for node_index in 0..node_count {
            let node_id = mapped_node_id(node_index);
            for (target, weight) in get_neighbors(node_id) {
                let target_index = validate_neighbor(node_count, node_id, target).map_err(|e| {
                    AlgorithmError::InvalidGraph(format!("Validation failed: {}", e))
                })?;
                if self.compute_max_distance {
                    validate_weight(node_id, target, weight).map_err(|e| {
                        AlgorithmError::InvalidGraph(format!("Validation failed: {}", e))
                    })?;
                }
                self.storage.in_degrees[target_index].fetch_add(1, Ordering::SeqCst);
            }
        }

        // Phase 2: Run parallel computation (can only fail with TerminatedException now)
        self.execute_parallel(node_count, concurrency, termination, get_neighbors)
            .map_err(|_| {
                AlgorithmError::Execution("Topological sort computation terminated".to_string())
            })
    }

    fn execute_parallel(
        &mut self,
        node_count: usize,
        concurrency: usize,
        termination: &TerminationFlag,
        get_neighbors: Arc<dyn Fn(MappedNodeId) -> Vec<(MappedNodeId, f64)> + Send + Sync>,
    ) -> std::result::Result<TopologicalSortResult, TerminatedException> {
        // Phase 2: Initialize queue with nodes having in-degree 0
        let ready_nodes = Arc::new(Mutex::new(Vec::new()));
        for node_index in 0..node_count {
            let node_id = mapped_node_id(node_index);
            if self.storage.in_degrees[node_index].load(Ordering::SeqCst) == 0 {
                ready_nodes.lock().unwrap().push(node_id);
                // Initialize distance for source nodes
                if let Some(ref distances) = self.storage.max_source_distances {
                    distances[node_index].store(0.0_f64.to_bits(), Ordering::SeqCst);
                }
            }
        }

        // Phase 3: Process nodes in parallel using work-stealing pattern
        let concurrency = concurrency.max(1);
        let executor = Executor::new(Concurrency::of(concurrency));
        let counter = Arc::new(AtomicUsize::new(0));
        let ready_nodes_copy = Arc::clone(&ready_nodes);

        executor.scope(termination, |scope| {
            scope.spawn_many(concurrency, |_worker_id| {
                loop {
                    if !termination.running() {
                        return;
                    }

                    let node_id = {
                        let mut ready = ready_nodes_copy.lock().unwrap();
                        if ready.is_empty() {
                            // Check if there are any pending nodes being processed
                            let processed = counter.load(Ordering::SeqCst);
                            if processed == 0 {
                                None
                            } else {
                                // Other workers might be adding nodes; spin briefly
                                drop(ready);
                                std::thread::yield_now();
                                None
                            }
                        } else {
                            Some(ready.pop().unwrap())
                        }
                    };

                    if let Some(source) = node_id {
                        counter.fetch_add(1, Ordering::SeqCst);
                        self.storage.add_node(source);
                        let source_index = physical_node_index(source);

                        let source_distance =
                            if let Some(ref distances) = self.storage.max_source_distances {
                                f64::from_bits(distances[source_index].load(Ordering::SeqCst))
                            } else {
                                0.0
                            };

                        for (target, weight) in get_neighbors(source) {
                            let target_index = physical_node_index(target);
                            // Update longest path distance if computing
                            if let Some(ref distances) = self.storage.max_source_distances {
                                loop {
                                    let current_bits =
                                        distances[target_index].load(Ordering::SeqCst);
                                    let current = f64::from_bits(current_bits);
                                    let new_distance = source_distance + weight;

                                    if new_distance > current {
                                        let new_bits = new_distance.to_bits();
                                        if distances[target_index]
                                            .compare_exchange(
                                                current_bits,
                                                new_bits,
                                                Ordering::SeqCst,
                                                Ordering::SeqCst,
                                            )
                                            .is_ok()
                                        {
                                            break;
                                        }
                                    } else {
                                        break;
                                    }
                                }
                            }

                            // Decrement in-degree
                            let prev_in_degree = self.storage.in_degrees[target_index]
                                .fetch_sub(1, Ordering::SeqCst);

                            // If in-degree becomes 0, add to queue
                            if prev_in_degree == 1 {
                                ready_nodes_copy.lock().unwrap().push(target);
                            }
                        }

                        counter.fetch_sub(1, Ordering::SeqCst);
                    } else if counter.load(Ordering::SeqCst) == 0 {
                        // No nodes ready and no nodes being processed; we're done
                        break;
                    }
                }
            });
        })?;

        // Phase 4: Build result
        let size = self.storage.size();
        let mut sorted_nodes = Vec::with_capacity(size);

        for i in 0..size {
            let node = self.storage.sorted_nodes[i].load(Ordering::SeqCst);
            sorted_nodes.push(MappedNodeId::new(node));
        }

        let max_source_distances = self.storage.max_source_distances.as_ref().map(|distances| {
            (0..node_count)
                .map(|i| f64::from_bits(distances[i].load(Ordering::SeqCst)))
                .collect()
        });

        Ok(TopologicalSortResult {
            sorted_nodes,
            max_source_distances,
        })
    }

    /// Compute topological sort using a graph for neighbor access
    pub fn compute_with_graph(
        &mut self,
        graph: &dyn Graph,
    ) -> std::result::Result<TopologicalSortResult, AlgorithmError> {
        let node_count = graph.node_count();
        let fallback = graph.default_property_value();

        let get_neighbors = |node_idx: MappedNodeId| -> Vec<(MappedNodeId, f64)> {
            graph
                .stream_relationships(node_idx, fallback)
                .map(|cursor| (cursor.target_id(), cursor.property()))
                .collect()
        };

        self.compute(node_count, get_neighbors)
    }
}

fn validate_neighbor(
    node_count: usize,
    source: MappedNodeId,
    target: MappedNodeId,
) -> std::result::Result<usize, AlgorithmError> {
    let target_index = target.to_usize().ok_or_else(|| {
        AlgorithmError::InvalidGraph(format!(
            "edge from node {source} points to target {target} outside physical index space"
        ))
    })?;
    if target_index >= node_count {
        return Err(AlgorithmError::InvalidGraph(format!(
            "edge from node {source} points to out-of-range target {target} for graph with {node_count} nodes"
        )));
    }

    Ok(target_index)
}

fn mapped_node_id(index: usize) -> MappedNodeId {
    MappedNodeId::try_from(index).expect("graph node count must fit mapped ID space")
}

fn physical_node_index(node_id: MappedNodeId) -> usize {
    node_id
        .to_usize()
        .expect("mapped graph node must fit physical index space")
}

fn validate_weight(
    source: MappedNodeId,
    target: MappedNodeId,
    weight: f64,
) -> std::result::Result<(), AlgorithmError> {
    if !weight.is_finite() {
        return Err(AlgorithmError::InvalidGraph(format!(
            "edge from node {source} to {target} has non-finite weight {weight}"
        )));
    }

    Ok(())
}
