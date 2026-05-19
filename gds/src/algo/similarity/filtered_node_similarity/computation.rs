use super::storage::FilteredNodeSimilarityStorageRuntime;
use crate::algo::similarity::node_similarity::{NodeSimilarityConfig, NodeSimilarityResult};
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
pub fn compute_filtered_node_similarity(
    graph: &dyn Graph,
    config: &NodeSimilarityConfig,
    sources: Option<&HashSet<MappedNodeId>>,
    targets: Option<&HashSet<MappedNodeId>>,
    termination: &TerminationFlag,
    on_sources_done: Arc<dyn Fn(usize) + Send + Sync>,
) -> Result<FilteredNodeSimilarityComputationReport, TerminatedException> {
    let storage = FilteredNodeSimilarityStorageRuntime::new(config.concurrency);
    storage.compute_with_filters_report(
        graph,
        config,
        sources,
        targets,
        termination,
        on_sources_done,
    )
}
