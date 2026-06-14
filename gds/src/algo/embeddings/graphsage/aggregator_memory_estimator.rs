//! Java: `AggregatorMemoryEstimator`.

use crate::task::memory::MemoryRange;

pub trait AggregatorMemoryEstimator {
    fn estimate(
        &self,
        min_node_count: u64,
        max_node_count: u64,
        min_previous_node_count: u64,
        max_previous_node_count: u64,
        input_dimension: usize,
        embedding_dimension: usize,
    ) -> MemoryRange;
}
