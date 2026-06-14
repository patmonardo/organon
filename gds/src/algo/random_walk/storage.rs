//! RandomWalk Storage
//!
//! Stores generated walk paths during computation.

use super::computation::RandomWalkComputationRuntime;
use super::spec::RandomWalkResult;
use crate::task::concurrency::{TerminatedException, TerminationFlag};
use crate::task::progress::ProgressTracker;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::graph::Graph;
use std::sync::Mutex;

/// Storage for random walk computation
pub struct RandomWalkStorageRuntime {
    /// Generated walks (each walk is a sequence of node IDs)
    pub walks: Mutex<Vec<Vec<u64>>>,
}

impl Default for RandomWalkStorageRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl RandomWalkStorageRuntime {
    pub fn new() -> Self {
        Self {
            walks: Mutex::new(Vec::new()),
        }
    }

    pub fn add_walk(&self, walk: Vec<u64>) {
        self.walks.lock().unwrap().push(walk);
    }

    pub fn take_walks(&self) -> Vec<Vec<u64>> {
        let mut walks = self.walks.lock().unwrap();
        std::mem::take(&mut *walks)
    }

    pub fn compute_random_walk(
        &self,
        computation: &RandomWalkComputationRuntime,
        graph: &dyn Graph,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<RandomWalkResult, AlgorithmError> {
        let node_count = graph.node_count();
        progress_tracker.begin_subtask_with_volume(node_count);

        let fallback = graph.default_property_value();
        let get_neighbors = |node_idx: usize| -> Vec<usize> {
            graph
                .stream_relationships(node_idx as i64, fallback)
                .filter_map(|cursor| {
                    let target = cursor.target_id();
                    if target >= 0 {
                        Some(target as usize)
                    } else {
                        None
                    }
                })
                .collect()
        };

        let result = computation.compute(node_count, get_neighbors);

        progress_tracker.log_progress(node_count);
        progress_tracker.end_subtask();

        Ok(result)
    }

    pub fn compute_random_walk_with_concurrency(
        &self,
        computation: &RandomWalkComputationRuntime,
        graph: &dyn Graph,
        progress_tracker: &mut dyn ProgressTracker,
        concurrency: usize,
        termination: &TerminationFlag,
    ) -> Result<RandomWalkResult, TerminatedException> {
        let node_count = graph.node_count();
        progress_tracker.begin_subtask_with_volume(node_count);

        let fallback = graph.default_property_value();
        let mut adjacency: Vec<Vec<usize>> = vec![Vec::new(); node_count];

        for node_idx in 0..node_count {
            adjacency[node_idx] = graph
                .stream_relationships(node_idx as i64, fallback)
                .filter_map(|cursor| {
                    let target = cursor.target_id();
                    if target >= 0 {
                        Some(target as usize)
                    } else {
                        None
                    }
                })
                .collect();
        }

        let adjacency = std::sync::Arc::new(adjacency);
        let get_neighbors = move |node_idx: usize| -> Vec<usize> { adjacency[node_idx].clone() };

        let result = computation.compute_with_concurrency(
            node_count,
            concurrency,
            termination,
            get_neighbors,
        );

        match result {
            Ok(result) => {
                progress_tracker.log_progress(node_count);
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
