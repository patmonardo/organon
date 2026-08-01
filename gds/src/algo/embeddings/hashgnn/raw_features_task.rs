//! Java: `RawFeaturesTask`.
//!
//! Depends on the `FeatureExtraction` subsystem (property extractors) which is
//! not fully wired in Rust GDS yet.

use crate::collections::HugeObjectArray;
use crate::core::utils::paged::HugeAtomicBitSet;
use crate::core::utils::partition::Partition;
use crate::ml::core::features::{self, FeatureConsumer};
use crate::task::concurrency::{TerminatedException, TerminationFlag};
use crate::types::graph::Graph;
use std::sync::Arc;

pub struct RawFeaturesTask;

impl RawFeaturesTask {
    pub fn compute(
        graph: Arc<dyn Graph>,
        partitions: Vec<Partition>,
        feature_properties: Vec<String>,
        termination_flag: &TerminationFlag,
    ) -> Result<(HugeObjectArray<Option<Arc<HugeAtomicBitSet>>>, u64), TerminatedException> {
        let extractors = features::property_extractors(graph.as_ref(), &feature_properties);
        let input_dimension = features::feature_count(&extractors);

        let mut out: HugeObjectArray<Option<Arc<HugeAtomicBitSet>>> =
            HugeObjectArray::new(graph.node_count());

        let mut total_feature_count: u64 = 0;

        struct Consumer<'a> {
            bitset: &'a HugeAtomicBitSet,
        }

        impl FeatureConsumer for Consumer<'_> {
            fn accept_scalar(&mut self, node_offset: u64, offset: usize, value: f64) {
                if value == 1.0 {
                    self.bitset.set(offset);
                } else if value != 0.0 {
                    panic!(
                        "Feature properties may only contain values 0 and 1 unless `binarizeFeatures` is used. Node {} has a feature property containing value {}",
                        node_offset, value
                    );
                }
            }

            fn accept_array(&mut self, node_offset: u64, offset: usize, values: &[f64]) {
                for (i, &value) in values.iter().enumerate() {
                    if value == 1.0 {
                        self.bitset.set(offset + i);
                    } else if value != 0.0 {
                        panic!(
                            "Feature properties may only contain values 0 and 1 unless `binarizeFeatures` is used. Node {} has a feature property containing value {}",
                            node_offset, value
                        );
                    }
                }
            }
        }

        for partition in partitions {
            if !termination_flag.running() {
                return Err(TerminatedException);
            }
            let mut terminated = false;
            partition.consume(|node_id| {
                if terminated || !termination_flag.running() {
                    terminated = true;
                    return;
                }
                let bitset = Arc::new(HugeAtomicBitSet::new(input_dimension));

                let mut consumer = Consumer {
                    bitset: bitset.as_ref(),
                };

                let mapped_node_id = u64::try_from(node_id)
                    .expect("HashGNN feature index must fit a mapped node ID");
                features::extract(mapped_node_id, mapped_node_id, &extractors, &mut consumer);

                total_feature_count += bitset.cardinality() as u64;
                out.set(node_id, Some(bitset));
            });
            if terminated {
                return Err(TerminatedException);
            }
        }

        Ok((out, total_feature_count))
    }
}
