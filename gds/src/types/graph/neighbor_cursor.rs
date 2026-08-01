use std::error::Error;
use std::fmt;
use std::sync::Arc;

use crate::types::graph::MappedNodeId;
use crate::types::graph::RelationshipIndex;
use crate::types::graph::RelationshipTopology;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraversalDirection {
    Outgoing,
    Incoming,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Neighbor {
    pub relationship_index: RelationshipIndex,
    pub source: MappedNodeId,
    pub target: MappedNodeId,
}

impl Neighbor {
    pub fn adjacent_node(self, direction: TraversalDirection) -> MappedNodeId {
        match direction {
            TraversalDirection::Outgoing => self.target,
            TraversalDirection::Incoming => self.source,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NeighborCursorError {
    NodeOutOfRange(MappedNodeId),
    InverseIndexUnavailable,
}

impl fmt::Display for NeighborCursorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NodeOutOfRange(node) => write!(formatter, "node {node} is outside mapped space"),
            Self::InverseIndexUnavailable => formatter.write_str("inverse index is unavailable"),
        }
    }
}

impl Error for NeighborCursorError {}

/// Reusable traversal state bound to one relationship-type topology partition.
pub trait NeighborCursor: Send + fmt::Debug {
    fn reset(
        &mut self,
        node: MappedNodeId,
        direction: TraversalDirection,
    ) -> Result<(), NeighborCursorError>;
    fn size(&self) -> usize;
    fn remaining(&self) -> usize;
    fn next_neighbor(&mut self) -> Option<Neighbor>;
    fn peek_neighbor(&self) -> Option<Neighbor>;

    /// Advances to the first adjacent node greater than or equal to `node`.
    ///
    /// The current row must be ordered by adjacent mapped node ID.
    fn advance(&mut self, node: MappedNodeId) -> Option<Neighbor> {
        while let Some(neighbor) = self.peek_neighbor() {
            if neighbor.adjacent_node(self.direction()) >= node {
                return self.next_neighbor();
            }
            self.next_neighbor()?;
        }
        None
    }

    /// Advances to the first adjacent node strictly greater than `node`.
    ///
    /// The current row must be ordered by adjacent mapped node ID.
    fn skip_until(&mut self, node: MappedNodeId) -> Option<Neighbor> {
        while let Some(neighbor) = self.peek_neighbor() {
            if neighbor.adjacent_node(self.direction()) > node {
                return self.next_neighbor();
            }
            self.next_neighbor()?;
        }
        None
    }

    /// Returns the neighbor at the zero-based offset from the current position.
    fn advance_by(&mut self, offset: usize) -> Option<Neighbor> {
        for _ in 0..offset {
            self.next_neighbor()?;
        }
        self.next_neighbor()
    }

    fn direction(&self) -> TraversalDirection;
}

#[derive(Debug)]
pub struct TopologyNeighborCursor {
    topology: Arc<RelationshipTopology>,
    node: MappedNodeId,
    direction: TraversalDirection,
    position: usize,
    size: usize,
}

impl TopologyNeighborCursor {
    pub(crate) fn new(topology: Arc<RelationshipTopology>) -> Self {
        Self {
            topology,
            node: MappedNodeId::ZERO,
            direction: TraversalDirection::Outgoing,
            position: 0,
            size: 0,
        }
    }

    fn neighbor_at(&self, position: usize) -> Option<Neighbor> {
        match self.direction {
            TraversalDirection::Outgoing => {
                let target = *self.topology.outgoing(self.node)?.get(position)?;
                let relationship_index = self.topology.relationship_index(self.node, position)?;
                Some(Neighbor {
                    relationship_index,
                    source: self.node,
                    target,
                })
            }
            TraversalDirection::Incoming => {
                let source = *self.topology.incoming(self.node)?.get(position)?;
                let relationship_index = *self
                    .topology
                    .incoming_relationship_indices(self.node)?
                    .get(position)?;
                Some(Neighbor {
                    relationship_index,
                    source,
                    target: self.node,
                })
            }
        }
    }
}

impl NeighborCursor for TopologyNeighborCursor {
    fn reset(
        &mut self,
        node: MappedNodeId,
        direction: TraversalDirection,
    ) -> Result<(), NeighborCursorError> {
        if node.to_usize().is_none_or(|index| index >= self.topology.node_capacity()) {
            return Err(NeighborCursorError::NodeOutOfRange(node));
        }
        if direction == TraversalDirection::Incoming && !self.topology.is_inverse_indexed() {
            return Err(NeighborCursorError::InverseIndexUnavailable);
        }
        self.node = node;
        self.direction = direction;
        self.position = 0;
        self.size = match direction {
            TraversalDirection::Outgoing => self.topology.outgoing(node).map_or(0, <[_]>::len),
            TraversalDirection::Incoming => self.topology.incoming(node).map_or(0, <[_]>::len),
        };
        Ok(())
    }

    fn size(&self) -> usize {
        self.size
    }

    fn remaining(&self) -> usize {
        self.size.saturating_sub(self.position)
    }

    fn next_neighbor(&mut self) -> Option<Neighbor> {
        let neighbor = self.neighbor_at(self.position)?;
        self.position += 1;
        Some(neighbor)
    }

    fn peek_neighbor(&self) -> Option<Neighbor> {
        self.neighbor_at(self.position)
    }

    fn direction(&self) -> TraversalDirection {
        self.direction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn topology() -> Arc<RelationshipTopology> {
        Arc::new(RelationshipTopology::try_new(
            vec![
                vec![MappedNodeId::new(1), MappedNodeId::new(1)],
                vec![],
            ],
            Some(vec![
                vec![],
                vec![MappedNodeId::new(0), MappedNodeId::new(0)],
            ]),
        )
        .unwrap())
    }

    #[test]
    fn outgoing_and_inverse_share_canonical_indices() {
        let topology = topology();
        let mut cursor = TopologyNeighborCursor::new(topology);
        cursor
            .reset(MappedNodeId::new(0), TraversalDirection::Outgoing)
            .unwrap();
        let outgoing = std::iter::from_fn(|| cursor.next_neighbor()).collect::<Vec<_>>();

        cursor
            .reset(MappedNodeId::new(1), TraversalDirection::Incoming)
            .unwrap();
        let incoming = std::iter::from_fn(|| cursor.next_neighbor()).collect::<Vec<_>>();

        assert_eq!(outgoing, incoming);
        assert_eq!(outgoing[0].relationship_index, RelationshipIndex::new(0));
        assert_eq!(outgoing[1].relationship_index, RelationshipIndex::new(1));
    }

    #[test]
    fn cursor_can_be_reused_for_empty_rows() {
        let topology = topology();
        let mut cursor = TopologyNeighborCursor::new(topology);
        cursor
            .reset(MappedNodeId::new(1), TraversalDirection::Outgoing)
            .unwrap();
        assert_eq!(cursor.size(), 0);
        assert_eq!(cursor.next_neighbor(), None);
    }

    #[test]
    fn advance_by_uses_zero_based_offsets() {
        let topology = topology();
        let mut cursor = TopologyNeighborCursor::new(topology);
        cursor
            .reset(MappedNodeId::ZERO, TraversalDirection::Outgoing)
            .unwrap();

        assert_eq!(
            cursor.advance_by(0).map(|neighbor| neighbor.relationship_index),
            Some(RelationshipIndex::ZERO)
        );
        assert_eq!(
            cursor.advance_by(0).map(|neighbor| neighbor.relationship_index),
            Some(RelationshipIndex::new(1))
        );
        assert_eq!(cursor.advance_by(0), None);
    }

    #[test]
    fn target_navigation_uses_the_adjacent_endpoint() {
        let topology = Arc::new(RelationshipTopology::try_new(
            vec![
                vec![MappedNodeId::new(1), MappedNodeId::new(3)],
                vec![MappedNodeId::new(3)],
                vec![],
                vec![],
            ],
            Some(vec![
                vec![],
                vec![MappedNodeId::ZERO],
                vec![],
                vec![MappedNodeId::ZERO, MappedNodeId::new(1)],
            ]),
        )
        .unwrap());
        let mut cursor = TopologyNeighborCursor::new(topology);

        cursor
            .reset(MappedNodeId::ZERO, TraversalDirection::Outgoing)
            .unwrap();
        assert_eq!(
            cursor.advance(MappedNodeId::new(2)).map(|neighbor| neighbor.target),
            Some(MappedNodeId::new(3))
        );

        cursor
            .reset(MappedNodeId::new(3), TraversalDirection::Incoming)
            .unwrap();
        assert_eq!(
            cursor.skip_until(MappedNodeId::ZERO).map(|neighbor| neighbor.source),
            Some(MappedNodeId::new(1))
        );
    }
}
