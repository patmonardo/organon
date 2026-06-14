//! DagLongestPath Storage
//!
//! Stores tentative distances and predecessors for longest path computation.

use super::computation::DagLongestPathComputationRuntime;
use super::spec::DagLongestPathResult;
use crate::task::concurrency::TerminatedException;
use crate::task::progress::ProgressTracker;
use crate::types::graph::{Graph, NodeId};
use std::sync::atomic::{AtomicI64, AtomicUsize, Ordering};

/// Storage for dag longest path computation
pub struct DagLongestPathStorageRuntime {
    /// In-degree for each node (updated during traversal)
    pub in_degrees: Vec<AtomicUsize>,
    /// Best distances found to each node (stored as bits for atomic f64)
    pub distances: Vec<AtomicI64>,
    /// Predecessor for each node in the longest path
    pub predecessors: Vec<AtomicI64>,
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
                .map(|_| AtomicI64::new(-1)) // Use -1 as sentinel instead of usize::MAX
                .collect(),
        }
    }

    pub fn get_distance(&self, node: usize) -> f64 {
        let bits = self.distances[node].load(Ordering::SeqCst);
        f64::from_bits(bits as u64)
    }

    pub fn set_distance(&self, node: usize, distance: f64) {
        self.set_distance_tag(node, distance, "set_distance");
    }

    pub fn set_distance_tag(&self, node: usize, distance: f64, _tag: &'static str) {
        self.distances[node].store(distance.to_bits() as i64, Ordering::SeqCst);
    }

    pub fn compare_and_update_distance(
        &self,
        node: usize,
        new_distance: f64,
        predecessor: usize,
    ) -> bool {
        loop {
            let current_bits = self.distances[node].load(Ordering::SeqCst);
            let current = f64::from_bits(current_bits as u64);

            if new_distance > current {
                let new_bits = new_distance.to_bits() as i64;
                match self.distances[node].compare_exchange(
                    current_bits,
                    new_bits,
                    Ordering::SeqCst,
                    Ordering::SeqCst,
                ) {
                    Ok(_) => {
                        // Successfully updated distance, also set predecessor
                        self.predecessors[node].store(predecessor as i64, Ordering::SeqCst);
                        return true;
                    }
                    Err(_) => continue,
                }
            } else {
                return false;
            }
        }
    }

    pub fn get_predecessor(&self, node: usize) -> Option<usize> {
        let pred = self.predecessors[node].load(Ordering::SeqCst);
        if pred == -1 {
            None
        } else {
            Some(pred as usize)
        }
    }

    pub fn set_predecessor(&self, node: usize, predecessor: usize) {
        self.set_predecessor_tag(node, predecessor, "set_predecessor");
    }

    pub fn set_predecessor_tag(&self, node: usize, predecessor: usize, _tag: &'static str) {
        self.predecessors[node].store(predecessor as i64, Ordering::SeqCst);
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
            let mut adjacency: Vec<Vec<(NodeId, f64)>> = Vec::with_capacity(node_count);

            for node_id in 0..node_count {
                let neighbors = graph
                    .stream_relationships(node_id as NodeId, fallback)
                    .filter_map(|cursor| {
                        let target = cursor.target_id();
                        if target < 0 {
                            return None;
                        }
                        Some((target, cursor.property()))
                    })
                    .collect();
                adjacency.push(neighbors);
            }

            let result = computation.compute_with_concurrency(
                node_count,
                concurrency,
                termination,
                move |node_id| {
                    usize::try_from(node_id)
                        .ok()
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
