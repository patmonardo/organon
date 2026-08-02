//! TopologicalSort Storage
//!
//! Stores in-degrees, sorted nodes, and optional longest path distances.

use super::computation::TopologicalSortComputationRuntime;
use super::spec::TopologicalSortResult;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::task::concurrency::TerminationFlag;
use crate::task::progress::ProgressTracker;
use crate::types::graph::{Graph, MappedNodeId};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

/// Storage for topological sort computation
pub struct TopologicalSortStorageRuntime {
    /// In-degree for each node (updated during traversal)
    pub in_degrees: Vec<AtomicUsize>,
    /// Sorted nodes in topological order
    pub sorted_nodes: Vec<AtomicU64>,
    /// Current position in sorted_nodes array
    pub add_index: AtomicUsize,
    /// Optional longest path distances
    pub max_source_distances: Option<Vec<AtomicU64>>,
}

impl TopologicalSortStorageRuntime {
    pub fn new(node_count: usize, compute_max_distance: bool) -> Self {
        Self {
            in_degrees: (0..node_count).map(|_| AtomicUsize::new(0)).collect(),
            sorted_nodes: (0..node_count).map(|_| AtomicU64::new(0)).collect(),
            add_index: AtomicUsize::new(0),
            max_source_distances: if compute_max_distance {
                Some((0..node_count).map(|_| AtomicU64::new(0)).collect())
            } else {
                None
            },
        }
    }

    pub fn add_node(&self, node_id: MappedNodeId) {
        let index = self.add_index.fetch_add(1, Ordering::SeqCst);
        self.sorted_nodes[index].store(node_id.get(), Ordering::SeqCst);
    }

    pub fn size(&self) -> usize {
        self.add_index.load(Ordering::SeqCst)
    }

    pub fn compute_topological_sort(
        &self,
        computation: &mut TopologicalSortComputationRuntime,
        graph: &dyn Graph,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<TopologicalSortResult, AlgorithmError> {
        self.compute_topological_sort_with_concurrency(
            computation,
            graph,
            progress_tracker,
            4,
            &TerminationFlag::running_true(),
        )
    }

    pub fn compute_topological_sort_with_concurrency(
        &self,
        computation: &mut TopologicalSortComputationRuntime,
        graph: &dyn Graph,
        progress_tracker: &mut dyn ProgressTracker,
        concurrency: usize,
        termination: &TerminationFlag,
    ) -> Result<TopologicalSortResult, AlgorithmError> {
        let node_count = graph.node_count();
        progress_tracker.begin_subtask_with_volume(node_count);

        let result = self.compute_topological_sort_with_context(
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

    pub fn compute_topological_sort_with_context(
        &self,
        computation: &mut TopologicalSortComputationRuntime,
        graph: &dyn Graph,
        progress_tracker: &mut dyn ProgressTracker,
        concurrency: usize,
        termination: &TerminationFlag,
    ) -> Result<TopologicalSortResult, AlgorithmError> {
        Self::ensure_running(termination)?;
        let node_count = graph.node_count();
        let fallback = graph.default_property_value();
        let mut edge_list: Vec<Vec<(MappedNodeId, f64)>> = vec![Vec::new(); node_count];

        for node_index in 0..node_count {
            Self::ensure_running(termination)?;
            let node_id = MappedNodeId::try_from(node_index)
                .expect("graph node count must fit mapped ID space");
            for cursor in graph.stream_relationships(node_id, fallback) {
                Self::ensure_running(termination)?;
                edge_list[node_index].push((cursor.target_id(), cursor.property()));
            }
        }

        let edge_list = std::sync::Arc::new(edge_list);
        let get_neighbors = move |node_idx: MappedNodeId| -> Vec<(MappedNodeId, f64)> {
            let node_index = node_idx
                .to_usize()
                .expect("mapped graph node must fit physical index space");
            edge_list[node_index].clone()
        };

        let result = computation.compute_with_concurrency(
            node_count,
            concurrency,
            termination,
            get_neighbors,
        )?;
        progress_tracker.log_progress(node_count);
        Ok(result)
    }

    fn ensure_running(termination: &TerminationFlag) -> Result<(), AlgorithmError> {
        if !termination.running() {
            return Err(AlgorithmError::Execution(
                "Topological sort computation terminated".to_string(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::Orientation;
    use crate::task::progress::NoopProgressTracker;
    use crate::types::graph_store::{DefaultGraphStore, GraphStore};
    use crate::types::random::{RandomGraphConfig, RandomRelationshipConfig};
    use std::collections::HashSet;

    #[test]
    fn context_computation_honors_pre_terminated_request() {
        let config = RandomGraphConfig {
            seed: Some(31),
            node_count: 4,
            relationships: vec![RandomRelationshipConfig::new("REL", 0.5)],
            ..RandomGraphConfig::default()
        };
        let store = DefaultGraphStore::random(&config).unwrap();
        let graph = store
            .get_graph_with_types_and_orientation(&HashSet::new(), Orientation::Natural)
            .unwrap();
        let storage = TopologicalSortStorageRuntime::new(graph.node_count(), false);
        let mut computation = TopologicalSortComputationRuntime::new(graph.node_count(), false);
        let mut progress_tracker = NoopProgressTracker;
        let termination = TerminationFlag::stop_running();

        let result = storage.compute_topological_sort_with_context(
            &mut computation,
            graph.as_ref(),
            &mut progress_tracker,
            2,
            &termination,
        );

        assert!(
            matches!(result, Err(AlgorithmError::Execution(message)) if message.contains("terminated"))
        );
    }

    #[test]
    fn initializes_result_state_without_distance_storage() {
        let storage = TopologicalSortStorageRuntime::new(3, false);

        assert_eq!(storage.size(), 0);
        assert_eq!(storage.in_degrees.len(), 3);
        assert!(storage
            .in_degrees
            .iter()
            .all(|degree| degree.load(Ordering::SeqCst) == 0));
        assert!(storage
            .sorted_nodes
            .iter()
            .all(|node| node.load(Ordering::SeqCst) == 0));
        assert!(storage.max_source_distances.is_none());
    }

    #[test]
    fn initializes_optional_distance_storage_to_zero() {
        let storage = TopologicalSortStorageRuntime::new(3, true);
        let distances = storage
            .max_source_distances
            .as_ref()
            .expect("distance storage must be initialized when requested");

        assert_eq!(storage.size(), 0);
        assert_eq!(distances.len(), 3);
        assert!(distances
            .iter()
            .all(|distance| f64::from_bits(distance.load(Ordering::SeqCst)) == 0.0));
    }
}
