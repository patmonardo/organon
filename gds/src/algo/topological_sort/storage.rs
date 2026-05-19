//! TopologicalSort Storage
//!
//! Stores in-degrees, sorted nodes, and optional longest path distances.

use super::computation::TopologicalSortComputationRuntime;
use super::spec::TopologicalSortResult;
use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::ProgressTracker;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::graph::{Graph, NodeId};
use std::sync::atomic::{AtomicI64, Ordering};

/// Storage for topological sort computation
pub struct TopologicalSortStorageRuntime {
    /// In-degree for each node (updated during traversal)
    pub in_degrees: Vec<AtomicI64>,
    /// Sorted nodes in topological order
    pub sorted_nodes: Vec<AtomicI64>,
    /// Current position in sorted_nodes array
    pub add_index: AtomicI64,
    /// Optional longest path distances
    pub max_source_distances: Option<Vec<AtomicI64>>, // Stored as bits for atomic f64
}

impl TopologicalSortStorageRuntime {
    pub fn new(node_count: usize, compute_max_distance: bool) -> Self {
        Self {
            in_degrees: (0..node_count).map(|_| AtomicI64::new(0)).collect(),
            sorted_nodes: (0..node_count)
                .map(|_| AtomicI64::new(-1)) // Use -1 as sentinel instead of usize::MAX
                .collect(),
            add_index: AtomicI64::new(0),
            max_source_distances: if compute_max_distance {
                Some((0..node_count).map(|_| AtomicI64::new(0)).collect())
            } else {
                None
            },
        }
    }

    pub fn add_node(&self, node_id: NodeId) {
        let index = self.add_index.fetch_add(1, Ordering::SeqCst);
        self.sorted_nodes[index as usize].store(node_id, Ordering::SeqCst);
    }

    pub fn size(&self) -> usize {
        self.add_index.load(Ordering::SeqCst) as usize
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

        let result = (|| {
            // Pre-collect all edges from the graph
            let fallback = graph.default_property_value();
            let mut edge_list: Vec<Vec<(NodeId, f64)>> = vec![Vec::new(); node_count];

            for node_idx in 0..node_count as i64 {
                let neighbors: Vec<(NodeId, f64)> = graph
                    .stream_relationships(node_idx, fallback)
                    .filter_map(|cursor| {
                        let target = cursor.target_id();
                        if target < 0 {
                            return None;
                        }
                        Some((target, cursor.property()))
                    })
                    .collect();

                edge_list[node_idx as usize] = neighbors;
            }

            let edge_list = std::sync::Arc::new(edge_list);
            let get_neighbors = move |node_idx: NodeId| -> Vec<(NodeId, f64)> {
                edge_list[node_idx as usize].clone()
            };

            let result = computation.compute_with_concurrency(
                node_count,
                concurrency,
                termination,
                get_neighbors,
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
