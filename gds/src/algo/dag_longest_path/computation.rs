//! DagLongestPath Computation
//!
//! **Translation Source**: `org.neo4j.gds.dag.longestPath.DagLongestPath`
//!
//! Finds longest paths in a DAG using parallel topological traversal with Executor-based work-stealing.

use super::spec::{DagLongestPathResult, PathRow};
use super::DagLongestPathStorageRuntime;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::task::concurrency::{
    virtual_threads::Executor, Concurrency, TerminatedException, TerminationFlag,
};
use crate::types::graph::MappedNodeId;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

pub struct DagLongestPathComputationRuntime {
    storage: Arc<DagLongestPathStorageRuntime>,
}

impl DagLongestPathComputationRuntime {
    pub fn new(node_count: usize) -> Self {
        Self {
            storage: Arc::new(DagLongestPathStorageRuntime::new(node_count)),
        }
    }

    /// Compute longest paths in DAG using parallel topological traversal with Executor
    pub fn compute(
        &mut self,
        node_count: usize,
        get_neighbors: impl Fn(MappedNodeId) -> Vec<(MappedNodeId, f64)> + Send + Sync + 'static,
    ) -> Result<DagLongestPathResult, TerminatedException> {
        self.compute_with_concurrency(
            node_count,
            4,
            &TerminationFlag::running_true(),
            get_neighbors,
        )
    }

    pub fn compute_with_concurrency(
        &mut self,
        node_count: usize,
        concurrency: usize,
        termination: &TerminationFlag,
        get_neighbors: impl Fn(MappedNodeId) -> Vec<(MappedNodeId, f64)> + Send + Sync + 'static,
    ) -> Result<DagLongestPathResult, TerminatedException> {
        self.storage = Arc::new(DagLongestPathStorageRuntime::new(node_count));
        let get_neighbors = Arc::new(get_neighbors);

        // Phase 1: Initialize in-degrees
        for node_index in 0..node_count {
            ensure_running(termination)?;
            let node_id = mapped_node_id(node_index);
            for (target, weight) in get_neighbors(node_id) {
                ensure_running(termination)?;
                let target_index = validate_neighbor(node_count, node_id, target)
                    .map_err(|_| TerminatedException)?;
                validate_weight(node_id, target, weight).map_err(|_| TerminatedException)?;
                self.storage.in_degrees[target_index].fetch_add(1, Ordering::SeqCst);
            }
        }

        // Phase 2: Collect initial ready nodes (in-degree 0)
        let ready_nodes = Arc::new(Mutex::new(Vec::new()));
        for node_index in 0..node_count {
            ensure_running(termination)?;
            let node_id = mapped_node_id(node_index);
            if self.storage.in_degrees[node_index].load(Ordering::SeqCst) == 0 {
                ready_nodes.lock().unwrap().push(node_id);
                // Initialize source node distance
                self.storage.set_distance(node_id, 0.0);
                self.storage.set_predecessor(node_id, node_id);
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

                    if let Some(node_id) = node_id {
                        counter.fetch_add(1, Ordering::SeqCst);

                        // Process all neighbors
                        for (target, weight) in get_neighbors(node_id) {
                            if !termination.running() {
                                return;
                            }
                            let target_index = physical_node_index(target);
                            let source_distance = self.storage.get_distance(node_id);
                            let potential_distance = source_distance + weight;
                            self.storage.compare_and_update_distance(
                                target,
                                potential_distance,
                                node_id,
                            );

                            // Decrement in-degree
                            let prev_degree = self.storage.in_degrees[target_index]
                                .fetch_sub(1, Ordering::SeqCst);

                            // If in-degree reaches 0, add to ready queue
                            if prev_degree == 1 {
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

        // Phase 4: Build path results
        self.build_paths(node_count, termination)
    }

    fn build_paths(
        &self,
        node_count: usize,
        termination: &TerminationFlag,
    ) -> Result<DagLongestPathResult, TerminatedException> {
        let mut paths = Vec::new();
        let mut path_index = 0u64;

        for target_index in 0..node_count {
            ensure_running(termination)?;
            let target_node = mapped_node_id(target_index);
            let distance = self.storage.get_distance(target_node);

            // Skip unreachable nodes (still have -infinity)
            if distance.is_infinite() && distance.is_sign_negative() {
                continue;
            }

            // Backtrack to build path
            let mut node_ids = Vec::new();
            let mut costs = Vec::new();
            let mut current = target_node;

            // Walk back through predecessors until we reach a source node
            loop {
                ensure_running(termination)?;
                node_ids.push(current);
                costs.push(self.storage.get_distance(current));

                match self.storage.get_predecessor(current) {
                    Some(pred) if pred != current => current = pred,
                    _ => break, // Reached a source node (predecessor == self)
                }
            }

            // Reverse to get path from source to target
            node_ids.reverse();
            costs.reverse();

            let source_node = node_ids[0];

            paths.push(PathRow {
                index: path_index,
                source_node,
                target_node,
                total_cost: distance,
                node_ids,
                costs,
            });

            path_index += 1;
        }

        Ok(DagLongestPathResult { paths })
    }
}

fn ensure_running(termination: &TerminationFlag) -> Result<(), TerminatedException> {
    if !termination.running() {
        return Err(TerminatedException);
    }
    Ok(())
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
