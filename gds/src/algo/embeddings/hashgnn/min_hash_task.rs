use crate::collections::{BitSet, HugeObjectArray};
use crate::core::utils::paged::HugeAtomicBitSet;
use crate::core::utils::partition::DegreePartition;
use crate::task::concurrency::virtual_threads::Executor;
use crate::task::concurrency::{Concurrency, TerminatedException, TerminationFlag};
use crate::types::graph::Graph;
use std::sync::Arc;

use super::hash_gnn::MinAndArgmin;
use super::hash_gnn_companion::HashGNNCompanion;
use super::hash_task::Hashes;

pub struct MinHashTask;

impl MinHashTask {
    #[allow(clippy::too_many_arguments)]
    pub fn compute(
        degree_partitions: &[DegreePartition],
        graphs: &[Arc<dyn Graph>],
        concurrency: Concurrency,
        embedding_density: usize,
        embedding_dimension: usize,
        current_embeddings: &HugeObjectArray<Option<Arc<HugeAtomicBitSet>>>,
        previous_embeddings: &HugeObjectArray<Option<Arc<HugeAtomicBitSet>>>,
        hashes: &[Hashes],
        termination_flag: &TerminationFlag,
    ) -> Result<u64, TerminatedException> {
        let feature_counts = Executor::new(concurrency).parallel_map(
            0,
            degree_partitions.len(),
            termination_flag,
            |partition_index| {
                let partition = &degree_partitions[partition_index];
                let mut partition_feature_count = 0u64;
                for hashes_for_k in hashes.iter().take(embedding_density) {
                    if !termination_flag.running() {
                        break;
                    }
                    let mut neighbors_vector = BitSet::new(embedding_dimension);
                    let mut self_min_and_arg_min = MinAndArgmin::default();
                    let mut neighbors_min_and_arg_min = MinAndArgmin::default();
                    let mut temp = MinAndArgmin::default();

                    let mut terminated = false;
                    partition.as_partition().consume(|node_id| {
                        if terminated || !termination_flag.running() {
                            terminated = true;
                            return;
                        }
                        let current_embedding = current_embeddings
                            .get(node_id)
                            .as_ref()
                            .expect("current embedding not initialized");

                        HashGNNCompanion::hash_argmin_atomic(
                            previous_embeddings
                                .get(node_id)
                                .as_ref()
                                .expect("previous embedding not initialized"),
                            &hashes_for_k.self_aggregation_hashes,
                            &mut self_min_and_arg_min,
                            &mut temp,
                        );

                        neighbors_vector.clear_all();

                        for (i, graph) in graphs.iter().enumerate() {
                            let pre_hashes = &hashes_for_k.pre_aggregation_hashes[i];

                            for cursor in graph.stream_relationships(node_id as i64, 1.0) {
                                let trg = cursor.target_id() as usize;
                                let prev_target_embedding = previous_embeddings
                                    .get(trg)
                                    .as_ref()
                                    .expect("previous embedding not initialized");

                                HashGNNCompanion::hash_argmin_atomic(
                                    prev_target_embedding,
                                    pre_hashes,
                                    &mut neighbors_min_and_arg_min,
                                    &mut temp,
                                );

                                let arg_min = neighbors_min_and_arg_min.arg_min;
                                if arg_min != -1 {
                                    neighbors_vector.set(arg_min as usize);
                                }
                            }
                        }

                        HashGNNCompanion::hash_argmin(
                            &neighbors_vector,
                            &hashes_for_k.neighbors_aggregation_hashes,
                            &mut neighbors_min_and_arg_min,
                        );

                        let arg_min = if neighbors_min_and_arg_min.min < self_min_and_arg_min.min {
                            neighbors_min_and_arg_min.arg_min
                        } else {
                            self_min_and_arg_min.arg_min
                        };

                        if arg_min != -1 && !current_embedding.get_and_set(arg_min as usize) {
                            partition_feature_count += 1;
                        }
                    });
                    if terminated {
                        break;
                    }
                }
                partition_feature_count
            },
        )?;

        Ok(feature_counts.into_iter().sum())
    }
}
