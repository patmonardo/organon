//! DagLongestPath Storage
//!
//! Stores tentative distances and predecessors for longest path computation.

use super::computation::DagLongestPathComputationRuntime;
use super::spec::DagLongestPathResult;
use crate::task::concurrency::TerminatedException;
use crate::task::progress::ProgressTracker;
use crate::types::graph::{Graph, MappedNodeId};
use std::sync::atomic::{AtomicI64, AtomicUsize, Ordering};
use std::sync::Mutex;

/// Storage for dag longest path computation
pub struct DagLongestPathStorageRuntime {
    /// In-degree for each node (updated during traversal)
    pub in_degrees: Vec<AtomicUsize>,
    /// Best distances found to each node (stored as bits for atomic f64)
    pub distances: Vec<AtomicI64>,
    /// Predecessor for each node in the longest path
    pub predecessors: Vec<Mutex<Option<MappedNodeId>>>,
}

impl DagLongestPathStorageRuntime {
    pub fn new(node_count: usize) -> Self {
        // Initialize distances to -infinity (worst possible for maximization)
        let neg_infinity_bits = f64::NEG_INFINITY.to_bits() as i64;

        Self {
            in_degrees: (0..node_count).map(|_| AtomicUsize::new(0)).collect(),
            distances: (0..node_count)
                .map(|_| AtomicI64::new(neg_infinity_bits))
                .collect(),
            predecessors: (0..node_count)
                .map(|_| Mutex::new(None))
                .collect(),
        }
    }

    pub fn get_distance(&self, node: MappedNodeId) -> f64 {
        let bits = self.distances[physical_node_index(node)].load(Ordering::SeqCst);
        f64::from_bits(bits as u64)
    }

    pub fn set_distance(&self, node: MappedNodeId, distance: f64) {
        self.set_distance_tag(node, distance, "set_distance");
    }

    pub fn set_distance_tag(&self, node: MappedNodeId, distance: f64, _tag: &'static str) {
        self.distances[physical_node_index(node)]
            .store(distance.to_bits() as i64, Ordering::SeqCst);
    }

    pub fn compare_and_update_distance(
        &self,
        node: MappedNodeId,
        new_distance: f64,
        predecessor: MappedNodeId,
    ) -> bool {
        let node_index = physical_node_index(node);
        loop {
            let current_bits = self.distances[node_index].load(Ordering::SeqCst);
            let current = f64::from_bits(current_bits as u64);

            if new_distance > current {
                let new_bits = new_distance.to_bits() as i64;
                match self.distances[node_index].compare_exchange(
                    current_bits,
                    new_bits,
                    Ordering::SeqCst,
                    Ordering::SeqCst,
                ) {
                    Ok(_) => {
                        // Successfully updated distance, also set predecessor
                        *self.predecessors[node_index].lock().unwrap() = Some(predecessor);
                        return true;
                    }
                    Err(_) => continue,
                }
            } else {
                return false;
            }
        }
    }

    pub fn get_predecessor(&self, node: MappedNodeId) -> Option<MappedNodeId> {
        *self.predecessors[physical_node_index(node)].lock().unwrap()
    }

    pub fn set_predecessor(&self, node: MappedNodeId, predecessor: MappedNodeId) {
        self.set_predecessor_tag(node, predecessor, "set_predecessor");
    }

    pub fn set_predecessor_tag(
        &self,
        node: MappedNodeId,
        predecessor: MappedNodeId,
        _tag: &'static str,
    ) {
        *self.predecessors[physical_node_index(node)].lock().unwrap() = Some(predecessor);
    }

    pub fn compute_dag_longest_path(
        &self,
        computation: &mut DagLongestPathComputationRuntime,
        graph: &dyn Graph,
        progress_tracker: &mut dyn ProgressTracker,
        concurrency: usize,
        termination: &crate::task::concurrency::TerminationFlag,
    ) -> Result<DagLongestPathResult, TerminatedException> {
        let node_count = graph.node_count();
        progress_tracker.begin_subtask_with_volume(node_count);

        let result = (|| {
            let fallback = graph.default_property_value();
            let mut adjacency: Vec<Vec<(MappedNodeId, f64)>> = Vec::with_capacity(node_count);

            for node_index in 0..node_count {
                let node_id = mapped_node_id(node_index);
                let neighbors = graph
                    .stream_relationships(node_id, fallback)
                    .map(|cursor| (cursor.target_id(), cursor.property()))
                    .collect();
                adjacency.push(neighbors);
            }

            let result = computation.compute_with_concurrency(
                node_count,
                concurrency,
                termination,
                move |node_id| {
                    node_id
                        .to_usize()
                        .and_then(|idx| adjacency.get(idx).cloned())
                        .unwrap_or_default()
                },
            )?;

            progress_tracker.log_progress(node_count);
            Ok(result)
        })();

        match result {
            Ok(result) => {
                progress_tracker.end_subtask();
                Ok(result)
            }
            Err(err) => {
                progress_tracker.end_subtask_with_failure();
                Err(err)
            }
        }
    }
}

fn mapped_node_id(index: usize) -> MappedNodeId {
    MappedNodeId::try_from(index).expect("graph node count must fit mapped ID space")
}

fn physical_node_index(node_id: MappedNodeId) -> usize {
    node_id
        .to_usize()
        .expect("mapped graph node must fit DAG longest-path storage")
}
