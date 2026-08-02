//! RandomWalk Storage
//!
//! Stores generated walk paths during computation.

use super::computation::RandomWalkComputationRuntime;
use super::spec::RandomWalkResult;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::task::concurrency::{TerminatedException, TerminationFlag};
use crate::task::progress::ProgressTracker;
use crate::types::graph::id_map::MappedNodeId;
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
            let node_id = MappedNodeId::try_from(node_idx)
                .expect("dense random-walk node index must fit a mapped node ID");
            graph
                .stream_relationships(node_id, fallback)
                .filter_map(|cursor| cursor.target_id().to_usize())
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

        let result = self.compute_random_walk_with_context(
            computation,
            graph,
            progress_tracker,
            concurrency,
            termination,
        );

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

    pub fn compute_random_walk_with_context(
        &self,
        computation: &RandomWalkComputationRuntime,
        graph: &dyn Graph,
        progress_tracker: &mut dyn ProgressTracker,
        concurrency: usize,
        termination: &TerminationFlag,
    ) -> Result<RandomWalkResult, TerminatedException> {
        Self::ensure_running(termination)?;
        let node_count = graph.node_count();

        let fallback = graph.default_property_value();
        let mut adjacency: Vec<Vec<usize>> = vec![Vec::new(); node_count];

        for node_idx in 0..node_count {
            Self::ensure_running(termination)?;
            let node_id = MappedNodeId::try_from(node_idx)
                .expect("dense random-walk node index must fit a mapped node ID");
            for cursor in graph.stream_relationships(node_id, fallback) {
                Self::ensure_running(termination)?;
                if let Some(target) = cursor.target_id().to_usize() {
                    adjacency[node_idx].push(target);
                }
            }
        }

        let adjacency = std::sync::Arc::new(adjacency);
        let get_neighbors = move |node_idx: usize| -> Vec<usize> { adjacency[node_idx].clone() };

        let result = computation.compute_with_concurrency(
            node_count,
            concurrency,
            termination,
            get_neighbors,
        )?;

        Self::ensure_running(termination)?;
        progress_tracker.log_progress(node_count);
        Ok(result)
    }

    fn ensure_running(termination: &TerminationFlag) -> Result<(), TerminatedException> {
        if !termination.running() {
            return Err(TerminatedException);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::Orientation;
    use crate::task::progress::NoopProgressTracker;
    use crate::types::graph_store::DefaultGraphStore;
    use crate::types::graph_store::GraphStore;
    use crate::types::random::RandomGraphConfig;
    use crate::types::random::RandomRelationshipConfig;
    use std::collections::HashSet;

    #[test]
    fn context_computation_honors_pre_terminated_request() {
        let config = RandomGraphConfig {
            seed: Some(23),
            node_count: 4,
            relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
            ..RandomGraphConfig::default()
        };
        let store = DefaultGraphStore::random(&config).unwrap();
        let graph = store
            .get_graph_with_types_and_orientation(&HashSet::new(), Orientation::Natural)
            .unwrap();
        let storage = RandomWalkStorageRuntime::new();
        let computation = RandomWalkComputationRuntime::new(1, 4, 1.0, 1.0, vec![], 7);
        let mut progress_tracker = NoopProgressTracker;
        let termination = TerminationFlag::stop_running();

        let result = storage.compute_random_walk_with_context(
            &computation,
            graph.as_ref(),
            &mut progress_tracker,
            1,
            &termination,
        );

        assert!(result.is_err());
    }
}
