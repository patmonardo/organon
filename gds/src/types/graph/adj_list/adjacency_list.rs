use std::sync::Arc;

use crate::types::graph::MappedNodeId;
use crate::types::graph::NeighborCursor;
use crate::types::graph::NeighborCursorError;
use crate::types::graph::RelationshipTopology;
use crate::types::graph::TopologyNeighborCursor;
use crate::types::graph::TraversalDirection;

/// Immutable topology for one relationship-type partition.
///
/// Implementations own their physical representation and create worker-local semantic cursors.
/// CSR offsets, decoder pages, and compression state are deliberately absent from this contract.
pub trait AdjacencyList: Send + Sync + std::fmt::Debug {
    fn degree(
        &self,
        node: MappedNodeId,
        direction: TraversalDirection,
    ) -> Result<usize, NeighborCursorError>;

    fn relationship_count(&self) -> usize;

    fn node_capacity(&self) -> usize;

    fn is_inverse_indexed(&self) -> bool;

    fn new_cursor(self: Arc<Self>) -> Box<dyn NeighborCursor>;
}

impl AdjacencyList for RelationshipTopology {
    fn degree(
        &self,
        node: MappedNodeId,
        direction: TraversalDirection,
    ) -> Result<usize, NeighborCursorError> {
        if node.to_usize().is_none_or(|index| index >= self.node_capacity()) {
            return Err(NeighborCursorError::NodeOutOfRange(node));
        }

        match direction {
            TraversalDirection::Outgoing => Ok(self.outgoing(node).map_or(0, <[_]>::len)),
            TraversalDirection::Incoming if !self.is_inverse_indexed() => {
                Err(NeighborCursorError::InverseIndexUnavailable)
            }
            TraversalDirection::Incoming => Ok(self.incoming(node).map_or(0, <[_]>::len)),
        }
    }

    fn relationship_count(&self) -> usize {
        RelationshipTopology::relationship_count(self)
    }

    fn node_capacity(&self) -> usize {
        RelationshipTopology::node_capacity(self)
    }

    fn is_inverse_indexed(&self) -> bool {
        RelationshipTopology::is_inverse_indexed(self)
    }

    fn new_cursor(self: Arc<Self>) -> Box<dyn NeighborCursor> {
        Box::new(TopologyNeighborCursor::new(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn topology_partition_creates_reusable_semantic_cursors() {
        let topology: Arc<dyn AdjacencyList> = Arc::new(RelationshipTopology::try_new(
            vec![vec![MappedNodeId::new(1)], vec![]],
            Some(vec![vec![], vec![MappedNodeId::ZERO]]),
        )
        .unwrap());

        assert_eq!(
            topology.degree(MappedNodeId::ZERO, TraversalDirection::Outgoing),
            Ok(1)
        );
        let mut cursor = Arc::clone(&topology).new_cursor();
        cursor
            .reset(MappedNodeId::ZERO, TraversalDirection::Outgoing)
            .unwrap();
        assert_eq!(
            cursor.next_neighbor().map(|neighbor| neighbor.target),
            Some(MappedNodeId::new(1))
        );

        cursor
            .reset(MappedNodeId::new(1), TraversalDirection::Incoming)
            .unwrap();
        assert_eq!(
            cursor.next_neighbor().map(|neighbor| neighbor.source),
            Some(MappedNodeId::ZERO)
        );
    }
}
