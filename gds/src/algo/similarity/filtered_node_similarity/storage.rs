use super::computation::FilteredNodeSimilarityComputationReport;
use crate::algo::similarity::node_similarity::{
    NodeSimilarityComputationRuntime, NodeSimilarityConfig, NodeSimilarityResult,
    NodeSimilarityStorageRuntime,
};
use crate::concurrency::{TerminatedException, TerminationFlag};
use crate::types::graph::graph::Graph;
use crate::types::graph::MappedNodeId;
use std::collections::HashSet;
use std::sync::Arc;

pub struct FilteredNodeSimilarityStorageRuntime {
    inner: NodeSimilarityStorageRuntime,
}

impl FilteredNodeSimilarityStorageRuntime {
    pub fn new(concurrency: usize) -> Self {
        Self {
            inner: NodeSimilarityStorageRuntime::new(concurrency),
        }
    }

    pub fn compute_with_filters_report(
        &self,
        graph: &dyn Graph,
        config: &NodeSimilarityConfig,
        sources: Option<&HashSet<MappedNodeId>>,
        targets: Option<&HashSet<MappedNodeId>>,
        termination: &TerminationFlag,
        on_sources_done: Arc<dyn Fn(usize) + Send + Sync>,
    ) -> Result<FilteredNodeSimilarityComputationReport, TerminatedException> {
        let node_count = graph.node_count();
        let computation = NodeSimilarityComputationRuntime::new();

        let sources_vec: Vec<u64> = if let Some(source_nodes) = sources {
            source_nodes.iter().map(|id| *id as u64).collect()
        } else {
            (0..node_count as u64).collect()
        };

        let target_mask = targets.map(|target_nodes| {
            let mut mask = vec![false; node_count];
            for id in target_nodes {
                let idx = *id as usize;
                if idx < node_count {
                    mask[idx] = true;
                }
            }
            mask
        });

        let report = self.inner.compute_with_filters_report(
            &computation,
            graph,
            config,
            sources_vec,
            target_mask,
            true,
            termination,
            on_sources_done,
        )?;

        Ok(FilteredNodeSimilarityComputationReport {
            results: report
                .results
                .into_iter()
                .map(NodeSimilarityResult::from)
                .collect(),
            compared_nodes: report.compared_nodes,
            completed_sources: report.completed_sources,
        })
    }
}
