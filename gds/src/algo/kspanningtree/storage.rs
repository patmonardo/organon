//! KSpanningTree Storage Runtime

use super::computation::KSpanningTreeComputationRuntime;
use super::spec::KSpanningTreeResult;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::task::concurrency::TerminationFlag;
use crate::task::progress::ProgressTracker;
use crate::types::graph::Graph;
use crate::types::graph::MappedNodeId;

/// KSpanningTree Storage Runtime - handles persistent data access and algorithm orchestration
pub struct KSpanningTreeStorageRuntime {
    /// Source node for spanning tree
    pub source_node: MappedNodeId,
    /// Number of spanning trees to create (k)
    pub k: u64,
    /// Objective: "min" or "max"
    pub objective: String,
}

impl KSpanningTreeStorageRuntime {
    /// Create new KSpanningTree storage runtime
    pub fn new(source_node: MappedNodeId, k: u64, objective: String) -> Self {
        Self {
            source_node,
            k,
            objective,
        }
    }

    /// Compute k-spanning tree using controller pattern
    ///
    /// **Translation Source**: `org.neo4j.gds.kspanningtree.KSpanningTree.compute()`
    ///
    /// This method orchestrates the k-spanning tree algorithm:
    /// 1. Compute MST using Prim's algorithm
    /// 2. Apply k-limiting logic to create exactly k spanning trees
    ///
    /// # Arguments
    ///
    /// * `computation` - Mutable reference to computation runtime for state management
    /// * `graph` - Optional graph interface for neighbor access
    /// * `progress_tracker` - Progress tracking interface
    ///
    /// # Returns
    ///
    /// A `Result` containing the `KSpanningTreeResult` or an error.
    pub fn compute_kspanningtree(
        &self,
        computation: &mut KSpanningTreeComputationRuntime,
        graph: Option<&dyn Graph>,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<KSpanningTreeResult, AlgorithmError> {
        self.compute_kspanningtree_with_termination(
            computation,
            graph,
            progress_tracker,
            &TerminationFlag::running_true(),
        )
    }

    /// Compute k-spanning tree using controller pattern with request termination support.
    pub fn compute_kspanningtree_with_termination(
        &self,
        computation: &mut KSpanningTreeComputationRuntime,
        graph: Option<&dyn Graph>,
        progress_tracker: &mut dyn ProgressTracker,
        termination: &TerminationFlag,
    ) -> Result<KSpanningTreeResult, AlgorithmError> {
        let progress_volume = graph.map(|graph| graph.relationship_count()).unwrap_or(0);
        progress_tracker.begin_subtask_with_volume(progress_volume);
        let result = self.compute_kspanningtree_with_context(
            computation,
            graph,
            progress_tracker,
            termination,
        );

        match result {
            Ok(value) => {
                progress_tracker.end_subtask();
                Ok(value)
            }
            Err(error) => {
                progress_tracker.end_subtask_with_failure();
                Err(error)
            }
        }
    }

    /// Compute k-spanning tree within a caller-owned progress lifecycle.
    pub fn compute_kspanningtree_with_context(
        &self,
        computation: &mut KSpanningTreeComputationRuntime,
        graph: Option<&dyn Graph>,
        progress_tracker: &mut dyn ProgressTracker,
        termination: &TerminationFlag,
    ) -> Result<KSpanningTreeResult, AlgorithmError> {
        Self::ensure_running(termination)?;
        let graph = graph.ok_or_else(|| {
            AlgorithmError::Execution("Graph interface required for k-spanning tree".to_string())
        })?;

        let node_count = graph.node_count();

        // Step 1: Compute MST using Prim's algorithm via spanning tree storage
        let is_min = self.objective == "min";
        let mst_result =
            self.compute_mst_using_prim(graph, is_min, progress_tracker, termination)?;

        // Step 2: Initialize computation runtime with MST data
        computation.initialize_from_mst(
            &mst_result
                .parent
                .iter()
                .map(|&x| x as i64)
                .collect::<Vec<_>>(),
            &mst_result.cost_to_parent,
            mst_result.total_weight,
            self.source_node.to_usize().ok_or_else(|| {
                AlgorithmError::InvalidGraph(format!(
                    "Source mapped node ID does not fit array indexing: {}",
                    self.source_node
                ))
            })?,
            node_count,
        );

        // Step 3: If k >= number of nodes, return MST as-is
        if self.k as usize >= node_count {
            return Ok(KSpanningTreeResult {
                parent: mst_result.parent.iter().map(|&x| x as i64).collect(),
                cost_to_parent: mst_result.cost_to_parent,
                total_cost: mst_result.total_weight,
                root: self.source_node.get(),
                node_count,
            });
        }

        // Step 4: Apply k-limiting logic
        computation.apply_k_limiting(
            self.k as usize,
            is_min,
            |node_id| self.get_neighbors_from_graph(graph, node_id),
            termination,
        )?;

        Ok(KSpanningTreeResult {
            parent: computation.get_parent().iter().map(|&x| x as i64).collect(),
            cost_to_parent: computation.get_cost_to_parent().clone(),
            total_cost: computation.get_total_cost(),
            root: self.source_node.get(),
            node_count,
        })
    }

    /// Compute MST using Prim's algorithm via spanning tree storage
    fn compute_mst_using_prim(
        &self,
        graph: &dyn Graph,
        is_min: bool,
        progress_tracker: &mut dyn ProgressTracker,
        termination: &TerminationFlag,
    ) -> Result<super::super::spanning_tree::SpanningTree, AlgorithmError> {
        use crate::algo::spanning_tree::{
            SpanningTreeComputationRuntime, SpanningTreeStorageRuntime,
        };

        // Create spanning tree runtimes
        let source_node = u32::try_from(self.source_node.get()).map_err(|_| {
            AlgorithmError::InvalidGraph(format!(
                "Source mapped node ID does not fit spanning-tree runtime: {}",
                self.source_node
            ))
        })?;
        let spanning_tree_storage = SpanningTreeStorageRuntime::new(
            source_node,
            is_min,
            1, // concurrency
        );
        let mut spanning_tree_computation = SpanningTreeComputationRuntime::new(
            source_node,
            is_min,
            graph.node_count() as u32,
            1, // concurrency
        );

        // Compute MST
        spanning_tree_storage.compute_spanning_tree_with_context(
            &mut spanning_tree_computation,
            Some(graph),
            2, // undirected
            progress_tracker,
            termination,
        )
    }

    fn ensure_running(termination: &TerminationFlag) -> Result<(), AlgorithmError> {
        if !termination.running() {
            return Err(AlgorithmError::Execution(
                "K-spanning tree computation terminated".to_string(),
            ));
        }
        Ok(())
    }

    /// Get neighbors from graph interface
    fn get_neighbors_from_graph(&self, graph: &dyn Graph, node_id: usize) -> Vec<(usize, f64)> {
        let fallback = 1.0;
        let Ok(mapped_node_id) = MappedNodeId::try_from(node_id) else {
            return Vec::new();
        };
        graph
            .stream_relationships_weighted(mapped_node_id, fallback)
            .filter_map(|cursor| {
                cursor
                    .target_id()
                    .to_usize()
                    .map(|target| (target, cursor.weight()))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::progress::NoopProgressTracker;

    #[test]
    fn context_computation_honors_pre_terminated_request() {
        let storage = KSpanningTreeStorageRuntime::new(MappedNodeId::new(0), 1, "min".to_string());
        let mut computation = KSpanningTreeComputationRuntime::new(0);
        let mut progress_tracker = NoopProgressTracker;
        let termination = TerminationFlag::stop_running();

        let error = storage
            .compute_kspanningtree_with_context(
                &mut computation,
                None,
                &mut progress_tracker,
                &termination,
            )
            .unwrap_err();

        assert!(error.to_string().contains("terminated"));
    }
}
