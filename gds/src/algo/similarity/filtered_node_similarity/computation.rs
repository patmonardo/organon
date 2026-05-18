use crate::algo::similarity::node_similarity::{
    NodeSimilarityComputationRuntime, NodeSimilarityConfig, NodeSimilarityResult,
    NodeSimilarityStorageRuntime,
};
use crate::concurrency::{TerminatedException, TerminationFlag};
use crate::types::graph::graph::Graph;
use crate::types::graph::MappedNodeId;
use std::collections::HashSet;
use std::sync::Arc;

pub struct FilteredNodeSimilarityComputationReport {
    pub results: Vec<NodeSimilarityResult>,
    pub compared_nodes: u64,
    pub completed_sources: usize,
}

/// Compute node similarity restricted by optional source/target node sets.
/// This is a thin wrapper that constructs the computation + storage runtimes and
/// delegates to `NodeSimilarityStorageRuntime::compute_with_filters`.
pub fn compute_filtered_node_similarity(
    graph: &dyn Graph,
    config: &NodeSimilarityConfig,
    sources: Option<&HashSet<MappedNodeId>>,
    targets: Option<&HashSet<MappedNodeId>>,
    termination: &TerminationFlag,
    on_sources_done: Arc<dyn Fn(usize) + Send + Sync>,
) -> Result<FilteredNodeSimilarityComputationReport, TerminatedException> {
    let computation = NodeSimilarityComputationRuntime::new();
    let storage = NodeSimilarityStorageRuntime::new(config.concurrency);

    let node_count = graph.node_count();

    // Build sources vector
    let sources_vec: Vec<u64> = if let Some(srcs) = sources {
        srcs.iter().map(|id| *id as u64).collect()
    } else {
        (0..node_count as u64).collect()
    };

    // Build target mask if provided
    let target_mask: Option<Vec<bool>> = if let Some(tgts) = targets {
        let mut mask = vec![false; node_count];
        for id in tgts {
            let idx = *id as usize;
            if idx < node_count {
                mask[idx] = true;
            }
        }
        Some(mask)
    } else {
        None
    };

    let report = storage.compute_with_filters_report(
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
