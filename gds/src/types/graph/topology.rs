use std::collections::HashMap;
use std::collections::VecDeque;
use std::error::Error;
use std::fmt;
use std::sync::Arc;

use crate::types::graph::id_map::MappedNodeId;
use crate::types::graph::id_map::RelationshipIndex;
use crate::types::graph::TopologyNeighborCursor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationshipTopologyError {
    RelationshipCountOverflow,
    TargetOutOfRange {
        source: MappedNodeId,
        target: MappedNodeId,
        node_count: usize,
    },
    InverseNodeCountMismatch {
        outgoing: usize,
        incoming: usize,
    },
    InverseRelationshipMismatch {
        source: MappedNodeId,
        target: MappedNodeId,
    },
}

impl fmt::Display for RelationshipTopologyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RelationshipCountOverflow => {
                formatter.write_str("relationship count exceeds relationship index space")
            }
            Self::TargetOutOfRange {
                source,
                target,
                node_count,
            } => write!(
                formatter,
                "relationship {source} -> {target} exceeds node count {node_count}"
            ),
            Self::InverseNodeCountMismatch { outgoing, incoming } => write!(
                formatter,
                "inverse node count {incoming} does not match outgoing node count {outgoing}"
            ),
            Self::InverseRelationshipMismatch { source, target } => write!(
                formatter,
                "inverse relationship {source} -> {target} has no matching outgoing relationship"
            ),
        }
    }
}

impl Error for RelationshipTopologyError {}

/// In-memory adjacency representation used by the default graph implementation.
#[derive(Debug, Clone)]
pub struct RelationshipTopology {
    outgoing: Vec<Vec<MappedNodeId>>,
    incoming: Option<Vec<Vec<MappedNodeId>>>,
    incoming_relationship_indices: Option<Vec<Vec<RelationshipIndex>>>,
    offsets: Vec<usize>,
    relationship_count: usize,
    has_parallel_edges: bool,
}

impl RelationshipTopology {
    /// Creates a new topology from outgoing adjacency lists.
    pub fn try_new(
        outgoing: Vec<Vec<MappedNodeId>>,
        incoming: Option<Vec<Vec<MappedNodeId>>>,
    ) -> Result<Self, RelationshipTopologyError> {
        let node_count = outgoing.len();
        let mut offsets: Vec<usize> = Vec::with_capacity(node_count + 1);
        offsets.push(0);
        let mut relationship_indices =
            HashMap::<(MappedNodeId, MappedNodeId), VecDeque<RelationshipIndex>>::new();

        for (source_index, targets) in outgoing.iter().enumerate() {
            let source = MappedNodeId::try_from(source_index)
                .map_err(|_| RelationshipTopologyError::RelationshipCountOverflow)?;
            let row_start = *offsets.last().expect("offset zero is present");
            let row_end = row_start
                .checked_add(targets.len())
                .ok_or(RelationshipTopologyError::RelationshipCountOverflow)?;
            offsets.push(row_end);

            for (neighbor_offset, &target) in targets.iter().enumerate() {
                if target.to_usize().is_none_or(|index| index >= node_count) {
                    return Err(RelationshipTopologyError::TargetOutOfRange {
                        source,
                        target,
                        node_count,
                    });
                }
                let physical_index = row_start
                    .checked_add(neighbor_offset)
                    .ok_or(RelationshipTopologyError::RelationshipCountOverflow)?;
                let relationship_index = RelationshipIndex::try_from(physical_index)
                    .map_err(|_| RelationshipTopologyError::RelationshipCountOverflow)?;
                relationship_indices
                    .entry((source, target))
                    .or_default()
                    .push_back(relationship_index);
            }
        }

        let incoming_relationship_indices = if let Some(incoming_lists) = incoming.as_ref() {
            if incoming_lists.len() != node_count {
                return Err(RelationshipTopologyError::InverseNodeCountMismatch {
                    outgoing: node_count,
                    incoming: incoming_lists.len(),
                });
            }

            let mut inverse_indices = Vec::with_capacity(node_count);
            for (target_index, sources) in incoming_lists.iter().enumerate() {
                let target = MappedNodeId::try_from(target_index)
                    .map_err(|_| RelationshipTopologyError::RelationshipCountOverflow)?;
                let mut row_indices = Vec::with_capacity(sources.len());
                for &source in sources {
                    let relationship_index = relationship_indices
                        .get_mut(&(source, target))
                        .and_then(VecDeque::pop_front)
                        .ok_or(RelationshipTopologyError::InverseRelationshipMismatch {
                            source,
                            target,
                        })?;
                    row_indices.push(relationship_index);
                }
                inverse_indices.push(row_indices);
            }

            if let Some((&(source, target), _)) = relationship_indices
                .iter()
                .find(|(_, indices)| !indices.is_empty())
            {
                return Err(RelationshipTopologyError::InverseRelationshipMismatch {
                    source,
                    target,
                });
            }
            Some(inverse_indices)
        } else {
            None
        };

        let relationship_count = *offsets.last().expect("offset zero is present");
        let has_parallel_edges = outgoing.iter().any(|adj| {
            let mut sorted = adj.clone();
            sorted.sort_unstable();
            sorted.windows(2).any(|window| window[0] == window[1])
        });

        Ok(Self {
            outgoing,
            incoming,
            incoming_relationship_indices,
            offsets,
            relationship_count,
            has_parallel_edges,
        })
    }

    pub fn new(outgoing: Vec<Vec<MappedNodeId>>, incoming: Option<Vec<Vec<MappedNodeId>>>) -> Self {
        Self::try_new(outgoing, incoming).expect("relationship topology must be valid")
    }

    /// Returns the number of relationships encoded in this topology.
    pub fn relationship_count(&self) -> usize {
        self.relationship_count
    }

    /// Returns true when this topology may contain parallel edges.
    pub fn has_parallel_edges(&self) -> bool {
        self.has_parallel_edges
    }

    /// Returns the outgoing adjacency for the given node, if available.
    pub fn outgoing(&self, node: MappedNodeId) -> Option<&[MappedNodeId]> {
        self.outgoing
            .get(node.to_usize()?)
            .map(|neighbors| neighbors.as_slice())
    }

    pub fn relationship_index(
        &self,
        source: MappedNodeId,
        neighbor_offset: usize,
    ) -> Option<RelationshipIndex> {
        let source_index = source.to_usize()?;
        let row_start = *self.offsets.get(source_index)?;
        let row_end = *self.offsets.get(source_index.checked_add(1)?)?;
        let physical_index = row_start.checked_add(neighbor_offset)?;
        (physical_index < row_end)
            .then(|| RelationshipIndex::try_from(physical_index).ok())
            .flatten()
    }

    /// Returns all outgoing adjacency lists.
    pub fn outgoing_lists(&self) -> &[Vec<MappedNodeId>] {
        &self.outgoing
    }

    /// Returns the incoming adjacency for the given node when an inverse index exists.
    pub fn incoming(&self, node: MappedNodeId) -> Option<&[MappedNodeId]> {
        self.incoming
            .as_ref()
            .and_then(|lists| lists.get(node.to_usize()?))
            .map(|neighbors| neighbors.as_slice())
    }

    pub fn incoming_relationship_indices(
        &self,
        node: MappedNodeId,
    ) -> Option<&[RelationshipIndex]> {
        self.incoming_relationship_indices
            .as_ref()
            .and_then(|indices| indices.get(node.to_usize()?))
            .map(Vec::as_slice)
    }

    /// Returns true when an inverse index is available.
    pub fn is_inverse_indexed(&self) -> bool {
        self.incoming.is_some()
    }

    /// Returns the total number of nodes tracked by this topology.
    pub fn node_capacity(&self) -> usize {
        self.outgoing.len()
    }

    pub fn neighbor_cursor(self: &Arc<Self>) -> TopologyNeighborCursor {
        TopologyNeighborCursor::new(Arc::clone(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_parallel_edges() {
        let one = MappedNodeId::new(1);
        let topology = RelationshipTopology::new(vec![vec![one, one], vec![]], None);
        assert!(topology.has_parallel_edges());
    }

    #[test]
    fn counts_relationships() {
        let topology = RelationshipTopology::new(
            vec![
                vec![MappedNodeId::new(1), MappedNodeId::new(2)],
                vec![MappedNodeId::new(0)],
                vec![],
            ],
            None,
        );
        assert_eq!(topology.relationship_count(), 3);
    }

    #[test]
    fn rejects_targets_outside_mapped_space() {
        assert!(matches!(
            RelationshipTopology::try_new(
                vec![vec![MappedNodeId::new(1)], vec![MappedNodeId::new(2)]],
                None,
            ),
            Err(RelationshipTopologyError::TargetOutOfRange { .. })
        ));
    }

    #[test]
    fn inverse_rows_preserve_canonical_parallel_relationship_indices() {
        let zero = MappedNodeId::new(0);
        let one = MappedNodeId::new(1);
        let topology = RelationshipTopology::try_new(
            vec![vec![one, one], vec![]],
            Some(vec![vec![], vec![zero, zero]]),
        )
        .unwrap();

        assert_eq!(
            topology.incoming_relationship_indices(one),
            Some([RelationshipIndex::new(0), RelationshipIndex::new(1)].as_slice())
        );
    }
}
