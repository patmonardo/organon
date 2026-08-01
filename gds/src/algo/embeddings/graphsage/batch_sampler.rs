//! Java: `BatchSampler`.

use crate::core::utils::partition::{Partition, PartitionUtils};
use crate::ml::core::samplers::WeightedUniformSampler;
use crate::task::concurrency::TerminationFlag;
use crate::types::graph::Graph;
use crate::types::graph::MappedNodeId;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::collections::HashSet;
use std::sync::Arc;

pub struct BatchSampler {
    graph: Arc<dyn Graph>,
    termination_flag: TerminationFlag,
}

impl BatchSampler {
    pub const DEGREE_SMOOTHING_FACTOR: f64 = 0.75;

    pub fn new(graph: Arc<dyn Graph>, termination_flag: TerminationFlag) -> Self {
        Self {
            graph,
            termination_flag,
        }
    }

    pub fn extended_batches(
        &self,
        batch_size: usize,
        search_depth: usize,
        random_seed: u64,
    ) -> Vec<Vec<u64>> {
        PartitionUtils::range_partition_with_batch_size(
            self.graph.node_count(),
            batch_size,
            |batch| {
                self.termination_flag.assert_running();
                let batch_start = u64::try_from(batch.start_node())
                    .expect("GraphSAGE batch start must fit the seed domain");
                let node_count = u64::try_from(self.graph.node_count().max(1))
                    .expect("GraphSAGE node count must fit the seed domain");
                let local_seed = (batch_start / node_count) + random_seed;
                self.sample_neighbor_and_negative_node_per_batch_node(
                    batch,
                    search_depth,
                    local_seed,
                )
            },
        )
    }

    /// For each node in the batch we sample one neighbor node and one negative node from the graph.
    pub fn sample_neighbor_and_negative_node_per_batch_node(
        &self,
        batch: Partition,
        search_depth: usize,
        random_seed: u64,
    ) -> Vec<u64> {
        let neighbours = self.neighbor_batch(batch, random_seed, search_depth);
        let negatives = self.negative_batch(batch.node_count(), &neighbours, random_seed);

        let mut out = Vec::with_capacity(batch.node_count() * 3);
        out.extend(batch.iter().map(|node_index| {
            u64::try_from(node_index).expect("GraphSAGE batch index must fit a mapped node ID")
        }));
        out.extend(neighbours);
        out.extend(negatives);
        out
    }

    fn neighbor_batch(
        &self,
        batch: Partition,
        batch_local_seed: u64,
        search_depth: usize,
    ) -> Vec<u64> {
        let mut rng = ChaCha8Rng::seed_from_u64(batch_local_seed);
        let mut neighbors = Vec::with_capacity(batch.node_count());

        for node_id in batch.iter() {
            // random walk with at most searchDepth steps, save last node
            let mut current = MappedNodeId::try_from(node_id)
                .expect("GraphSAGE batch node index must fit a mapped node ID");
            let mut depth = rng.gen_range(1..=search_depth.max(1));

            while depth > 0 {
                let degree = self.graph.degree(current);
                if degree == 0 {
                    break;
                }
                let idx = rng.gen_range(0..degree);
                let next = self
                    .graph
                    .nth_target(current, idx)
                    .expect("nth_target within degree");
                current = next;
                depth -= 1;
            }

            neighbors.push(current.get());
        }

        neighbors
    }

    fn negative_batch(
        &self,
        batch_size: usize,
        batch_neighbors: &[u64],
        batch_local_seed: u64,
    ) -> Vec<u64> {
        let node_count = self.graph.node_count();
        let mut sampler = WeightedUniformSampler::new(batch_local_seed);

        let neighbor_set: HashSet<u64> = batch_neighbors.iter().copied().collect();

        let mapped_node_count =
            u64::try_from(node_count).expect("GraphSAGE node count must fit mapped node IDs");
        let input = (0..mapped_node_count).map(|node_id| {
            let degree = self.graph.degree(MappedNodeId::new(node_id)) as f64;
            (node_id, degree.powf(Self::DEGREE_SMOOTHING_FACTOR))
        });

        sampler.sample_filtered(input, node_count, batch_size, |sample| {
            !neighbor_set.contains(&sample)
        })
    }
}
