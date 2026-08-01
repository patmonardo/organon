//! Neighborhood sampler for subgraphs in GDS.
//!
//! Translated from Java GDS ml-core NeighborhoodSampler.java.
//! This is a literal 1:1 translation following repository translation policy.

use crate::ml::core::relationship_weights::DEFAULT_VALUE;
use crate::ml::core::samplers::{UniformSampler, WeightedUniformSampler};
use crate::types::graph::id_map::MappedNodeId;
use crate::types::graph::Graph;
use crate::types::properties::relationship::{
    RelationshipCursorBox, WeightedRelationshipCursorBox,
};

/// Samples neighborhoods for graph neural network batch processing.
///
pub struct NeighborhoodSampler {
    random_seed: u64,
}

impl NeighborhoodSampler {
    /// Create a new neighborhood sampler with the given random seed.
    pub fn new(random_seed: u64) -> Self {
        Self { random_seed }
    }

    /// Get the random seed (for future implementation).
    #[allow(dead_code)]
    pub fn random_seed(&self) -> u64 {
        self.random_seed
    }

    /// Sample up to `sample_size` neighbors of `node_id` uniformly without replacement.
    ///
    pub fn sample(&self, graph: &dyn Graph, node_id: u64, sample_size: usize) -> Vec<u64> {
        let mapped_node_id = MappedNodeId::new(node_id);
        let degree = graph.degree(mapped_node_id);
        if degree == 0 || sample_size == 0 {
            return Vec::new();
        }

        let concurrent_graph = Graph::concurrent_view(graph);

        // Every neighbor needs to be sampled
        if degree <= sample_size {
            return concurrent_graph
                .stream_relationships(mapped_node_id, DEFAULT_VALUE)
                .map(|cursor: RelationshipCursorBox| u64::from(cursor.target_id()))
                .collect();
        }

        if graph.has_relationship_property() {
            let mut sampler = WeightedUniformSampler::new(self.random_seed + node_id);
            let input = concurrent_graph
                .stream_relationships_weighted(mapped_node_id, DEFAULT_VALUE)
                .map(|cursor: WeightedRelationshipCursorBox| {
                    (u64::from(cursor.target_id()), cursor.weight())
                });
            sampler.sample(input, degree, sample_size)
        } else {
            let mut sampler = UniformSampler::new(self.random_seed + node_id);
            let input = concurrent_graph
                .stream_relationships(mapped_node_id, DEFAULT_VALUE)
                .map(|cursor: RelationshipCursorBox| u64::from(cursor.target_id()));
            let input_length = u64::try_from(degree).expect("degree must fit in u64");
            sampler.sample(input, input_length, sample_size)
        }
    }
}
