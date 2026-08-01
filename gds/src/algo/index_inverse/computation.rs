//! IndexInverse computation runtime.
//!
//! Builds per-relationship-type outgoing/incoming adjacency from a graph view.

use crate::types::graph::Graph;
use crate::types::graph::MappedNodeId;

#[derive(Default, Clone)]
pub struct IndexInverseComputationRuntime;

impl IndexInverseComputationRuntime {
    pub fn new() -> Self {
        Self
    }

    pub fn build_topology(
        &self,
        graph: &dyn Graph,
    ) -> (Vec<Vec<MappedNodeId>>, Vec<Vec<MappedNodeId>>) {
        let node_count = graph.node_count();
        let fallback = graph.default_property_value();

        let mut outgoing: Vec<Vec<MappedNodeId>> = vec![Vec::new(); node_count];
        let mut incoming: Vec<Vec<MappedNodeId>> = vec![Vec::new(); node_count];

        for src in 0..node_count {
            let Ok(source) = MappedNodeId::try_from(src) else {
                continue;
            };
            for cursor in graph.stream_relationships(source, fallback) {
                let tgt = cursor.target_id();
                outgoing[src].push(tgt);
                if let Some(incoming_row) = tgt.to_usize().and_then(|index| incoming.get_mut(index)) {
                    incoming_row.push(source);
                }
            }
        }

        for adj in outgoing.iter_mut() {
            adj.sort_unstable();
            adj.dedup();
        }
        for adj in incoming.iter_mut() {
            adj.sort_unstable();
            adj.dedup();
        }

        (outgoing, incoming)
    }
}
