use crate::types::graph_store::DefaultGraphStore;
use std::collections::HashMap;

use crate::types::graph::Degrees as _;
use crate::types::graph::IdMap as _;
use crate::types::graph::MappedNodeId;

/// DegreeDistributionApplier
///
/// Small utility that computes a simple degree histogram for a `DefaultGraphStore`.
pub struct DegreeDistributionApplier;

impl DegreeDistributionApplier {
    pub fn new() -> Self {
        Self
    }

    /// Compute a degree histogram: map degree -> count
    pub fn compute_histogram(&self, store: &DefaultGraphStore) -> HashMap<u32, u64> {
        let graph = store.graph();
        let node_count = graph.node_count();
        let mut hist: HashMap<u32, u64> = HashMap::new();

        for mapped in 0..node_count {
            let mapped_node_id = MappedNodeId::try_from(mapped)
                .expect("graph node count must fit mapped node IDs");
            let deg = graph.degree(mapped_node_id);
            let key = u32::try_from(deg).expect("node degree must fit the histogram key");
            *hist.entry(key).or_insert(0) += 1;
        }

        hist
    }
}

impl Default for DegreeDistributionApplier {
    fn default() -> Self {
        Self::new()
    }
}
