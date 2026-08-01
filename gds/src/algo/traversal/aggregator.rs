use crate::types::graph::MappedNodeId;

/// Aggregator function for computing weights during traversal
///
/// Translation of: `Aggregator.java` (lines 22-35)
/// Aggregates weight between source and current node
pub trait Aggregator: Send + Sync {
    /// Apply aggregation function
    ///
    /// # Arguments
    /// * `source_node` - Source node
    /// * `current_node` - Current node
    /// * `weight_at_source` - Weight that has been aggregated for current node so far
    ///
    /// # Returns
    /// New weight (e.g., weight_at_source + 1.0)
    fn apply(&self, source_node: MappedNodeId, current_node: MappedNodeId, weight_at_source: f64) -> f64;
}

/// No aggregation aggregator
///
/// Translation of: `Aggregator.NO_AGGREGATION` (line 24)
pub struct NoAggregator;

impl Aggregator for NoAggregator {
    fn apply(&self, _source_node: MappedNodeId, _current_node: MappedNodeId, _weight_at_source: f64) -> f64 {
        0.0
    }
}

/// One-hop aggregator that increments weight by 1
///
/// Translation of: `OneHopAggregator.java` (lines 22-27)
pub struct OneHopAggregator;

impl Aggregator for OneHopAggregator {
    fn apply(&self, _source_node: MappedNodeId, _current_node: MappedNodeId, weight_at_source: f64) -> f64 {
        weight_at_source + 1.0
    }
}

/// Weight-based aggregator that adds edge weights
pub struct WeightAggregator;

impl Aggregator for WeightAggregator {
    fn apply(&self, _source_node: MappedNodeId, _current_node: MappedNodeId, weight_at_source: f64) -> f64 {
        // Note: edge weight lookup is deferred.
        // For now, just increment by 1.0
        weight_at_source + 1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(value: u64) -> MappedNodeId {
        MappedNodeId::new(value)
    }

    #[test]
    fn test_no_aggregator() {
        let aggregator = NoAggregator;
        assert_eq!(aggregator.apply(node(0), node(1), 5.0), 0.0);
    }

    #[test]
    fn test_one_hop_aggregator() {
        let aggregator = OneHopAggregator;
        assert_eq!(aggregator.apply(node(0), node(1), 0.0), 1.0);
        assert_eq!(aggregator.apply(node(1), node(2), 1.0), 2.0);
    }

    #[test]
    fn test_weight_aggregator() {
        let aggregator = WeightAggregator;
        assert_eq!(aggregator.apply(node(0), node(1), 0.0), 1.0);
        assert_eq!(aggregator.apply(node(1), node(2), 1.0), 2.0);
    }
}
