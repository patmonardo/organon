use crate::types::graph::id_map::MappedNodeId;
use crate::types::graph::id_map::RelationshipIndex;
use crate::types::properties::relationship::{ModifiableRelationshipCursor, RelationshipCursor};

/// Immutable relationship cursor mirroring the TypeScript primitive
/// implementation. Stores the source node id, target node id and an
/// associated property value.
#[derive(Debug, Clone, Copy, Default)]
pub struct DefaultRelationshipCursor {
    relationship_index: RelationshipIndex,
    source_id: MappedNodeId,
    target_id: MappedNodeId,
    property: f64,
}

impl DefaultRelationshipCursor {
    /// Construct a new immutable cursor instance.
    pub fn new(
        relationship_index: RelationshipIndex,
        source_id: MappedNodeId,
        target_id: MappedNodeId,
        property: f64,
    ) -> Self {
        Self {
            relationship_index,
            source_id,
            target_id,
            property,
        }
    }

    /// Create a modifiable cursor seeded with the same values.
    pub fn to_modifiable(self) -> DefaultModifiableRelationshipCursor {
        DefaultModifiableRelationshipCursor::new(
            self.relationship_index,
            self.source_id,
            self.target_id,
            self.property,
        )
    }
}

impl RelationshipCursor for DefaultRelationshipCursor {
    fn relationship_index(&self) -> RelationshipIndex {
        self.relationship_index
    }

    fn source_id(&self) -> MappedNodeId {
        self.source_id
    }

    fn target_id(&self) -> MappedNodeId {
        self.target_id
    }

    fn property(&self) -> f64 {
        self.property
    }
}

/// Mutable cursor implementation used by iterators that reuse a single
/// cursor instance while traversing relationships.
#[derive(Debug, Clone, Copy, Default)]
pub struct DefaultModifiableRelationshipCursor {
    relationship_index: RelationshipIndex,
    source_id: MappedNodeId,
    target_id: MappedNodeId,
    property: f64,
}

impl DefaultModifiableRelationshipCursor {
    /// Creates a new modifiable cursor with the provided initial values.
    pub fn new(
        relationship_index: RelationshipIndex,
        source_id: MappedNodeId,
        target_id: MappedNodeId,
        property: f64,
    ) -> Self {
        Self {
            relationship_index,
            source_id,
            target_id,
            property,
        }
    }

    /// Consume the modifiable cursor and return an immutable view.
    pub fn freeze(self) -> DefaultRelationshipCursor {
        DefaultRelationshipCursor::new(
            self.relationship_index,
            self.source_id,
            self.target_id,
            self.property,
        )
    }
}

impl RelationshipCursor for DefaultModifiableRelationshipCursor {
    fn relationship_index(&self) -> RelationshipIndex {
        self.relationship_index
    }

    fn source_id(&self) -> MappedNodeId {
        self.source_id
    }

    fn target_id(&self) -> MappedNodeId {
        self.target_id
    }

    fn property(&self) -> f64 {
        self.property
    }
}

impl ModifiableRelationshipCursor for DefaultModifiableRelationshipCursor {
    fn set_relationship_index(&mut self, relationship_index: RelationshipIndex) {
        self.relationship_index = relationship_index;
    }

    fn set_source_id(&mut self, source_id: MappedNodeId) {
        self.source_id = source_id;
    }

    fn set_target_id(&mut self, target_id: MappedNodeId) {
        self.target_id = target_id;
    }

    fn set_property(&mut self, property: f64) {
        self.property = property;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn immutable_cursor_exposes_values() {
        let cursor = DefaultRelationshipCursor::new(
            RelationshipIndex::new(4),
            MappedNodeId::new(1),
            MappedNodeId::new(2),
            3.5,
        );
        assert_eq!(cursor.relationship_index(), RelationshipIndex::new(4));
        assert_eq!(cursor.source_id(), MappedNodeId::new(1));
        assert_eq!(cursor.target_id(), MappedNodeId::new(2));
        assert_eq!(cursor.property(), 3.5);
    }

    #[test]
    fn modifiable_cursor_updates_in_place() {
        let mut cursor = DefaultModifiableRelationshipCursor::new(
            RelationshipIndex::ZERO,
            MappedNodeId::ZERO,
            MappedNodeId::ZERO,
            0.0,
        );
        cursor.set_relationship_index(RelationshipIndex::new(7));
        cursor.set_source_id(MappedNodeId::new(10));
        cursor.set_target_id(MappedNodeId::new(20));
        cursor.set_property(2.5);

        assert_eq!(cursor.relationship_index(), RelationshipIndex::new(7));
        assert_eq!(cursor.source_id(), MappedNodeId::new(10));
        assert_eq!(cursor.target_id(), MappedNodeId::new(20));
        assert_eq!(cursor.property(), 2.5);
    }

    #[test]
    fn freeze_returns_immutable_snapshot() {
        let cursor = DefaultModifiableRelationshipCursor::new(
            RelationshipIndex::new(2),
            MappedNodeId::new(3),
            MappedNodeId::new(4),
            5.5,
        )
        .freeze();
        assert_eq!(cursor.relationship_index(), RelationshipIndex::new(2));
        assert_eq!(cursor.source_id(), MappedNodeId::new(3));
        assert_eq!(cursor.target_id(), MappedNodeId::new(4));
        assert_eq!(cursor.property(), 5.5);
    }

    #[test]
    fn to_modifiable_round_trips() {
        let original = DefaultRelationshipCursor::new(
            RelationshipIndex::new(6),
            MappedNodeId::new(7),
            MappedNodeId::new(8),
            9.0,
        );
        let mut modifiable = original.to_modifiable();
        modifiable.set_property(1.5);
        assert_eq!(modifiable.relationship_index(), RelationshipIndex::new(6));
        assert_eq!(modifiable.source_id(), MappedNodeId::new(7));
        assert_eq!(modifiable.target_id(), MappedNodeId::new(8));
        assert_eq!(modifiable.property(), 1.5);

        let frozen = modifiable.freeze();
        assert_eq!(frozen.relationship_index(), RelationshipIndex::new(6));
        assert_eq!(frozen.source_id(), MappedNodeId::new(7));
        assert_eq!(frozen.target_id(), MappedNodeId::new(8));
        assert_eq!(frozen.property(), 1.5);
    }
}
