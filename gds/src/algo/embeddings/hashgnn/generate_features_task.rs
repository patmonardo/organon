use crate::collections::HugeObjectArray;
use crate::core::utils::paged::HugeAtomicBitSet;
use crate::core::utils::partition::Partition;
use crate::types::graph::Graph;
use crate::types::graph::MappedNodeId;
use crate::types::graph::OriginalNodeId;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::sync::Arc;

use super::hash_gnn_parameters::GenerateFeaturesConfig;

pub struct GenerateFeaturesTask {
    partition: Partition,
    graph: Arc<dyn Graph>,
    random_seed: u64,
    config: GenerateFeaturesConfig,
    output: Arc<parking_lot::Mutex<HugeObjectArray<Option<Arc<HugeAtomicBitSet>>>>>,
    total_feature_count: u64,
}

impl GenerateFeaturesTask {
    pub fn new(
        partition: Partition,
        graph: Arc<dyn Graph>,
        random_seed: u64,
        config: GenerateFeaturesConfig,
        output: Arc<parking_lot::Mutex<HugeObjectArray<Option<Arc<HugeAtomicBitSet>>>>>,
    ) -> Self {
        Self {
            partition,
            graph,
            random_seed,
            config,
            output,
            total_feature_count: 0,
        }
    }

    pub fn compute(
        config: GenerateFeaturesConfig,
        graph: Arc<dyn Graph>,
        partitions: Vec<Partition>,
        random_seed: u64,
    ) -> (HugeObjectArray<Option<Arc<HugeAtomicBitSet>>>, u64) {
        let output: HugeObjectArray<Option<Arc<HugeAtomicBitSet>>> =
            HugeObjectArray::new(graph.node_count());
        let output = Arc::new(parking_lot::Mutex::new(output));

        // Sequential for now (close-to-Java decomposition, but safe).
        let mut total = 0u64;
        for p in partitions {
            let mut task = GenerateFeaturesTask::new(
                p,
                Arc::clone(&graph),
                random_seed,
                config.clone(),
                Arc::clone(&output),
            );
            task.run();
            total += task.total_feature_count;
        }

        let output = match Arc::try_unwrap(output) {
            Ok(mutex) => mutex.into_inner(),
            Err(_) => panic!("output still referenced"),
        };
        (output, total)
    }

    pub fn run(&mut self) {
        let dimension = self.config.dimension;
        let density_level = self.config.density_level;

        self.partition.consume(|node_id| {
            let bitset = Arc::new(HugeAtomicBitSet::new(dimension));

            let mapped_node_id = MappedNodeId::try_from(node_id)
                .expect("HashGNN partition node index must fit a mapped node ID");
            let original = self
                .graph
                .to_original_node_id(mapped_node_id)
                .unwrap_or_else(|| {
                    OriginalNodeId::new(
                        i64::try_from(node_id)
                            .expect("HashGNN fallback original node ID must fit i64"),
                    )
                });
            let original_seed_bits = u64::from_ne_bytes(original.get().to_ne_bytes());
            let seed = self.random_seed ^ original_seed_bits;
            let mut rng = ChaCha8Rng::seed_from_u64(seed);

            for _ in 0..density_level {
                if dimension == 0 {
                    break;
                }
                let idx = rng.gen_range(0..dimension);
                bitset.set(idx);
            }

            self.total_feature_count += bitset.cardinality() as u64;

            self.output.lock().set(node_id, Some(bitset));
        });
    }

    #[allow(dead_code)]
    pub fn total_feature_count(&self) -> u64 {
        self.total_feature_count
    }
}
