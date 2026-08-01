//! **Relationship Filterer**
//!
//! **Translation Source**: `org.neo4j.gds.paths.yens.RelationshipFilterer`
//!
//! This module implements relationship filtering for Yen's algorithm to avoid cycles and duplicates.

use super::mutable_path_result::MutablePathResult;

use crate::types::graph::MappedNodeId;
use crate::types::graph::RelationshipIndex;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum BlockedIdentity {
    Node(MappedNodeId),
    Relationship(RelationshipIndex),
}

/// Relationship filterer for Yen's algorithm
///
/// Translation of: `RelationshipFilterer.java` (lines 25-83)
/// Filters relationships to avoid cycles and duplicate paths
#[derive(Clone)]
pub struct RelationshipFilterer {
    /// Neighbors to avoid
    neighbors: Vec<BlockedIdentity>,
    /// Current filtering spur node
    filtering_spur_node: Option<MappedNodeId>,
    /// Whether to track relationships
    track_relationships: bool,
}

impl RelationshipFilterer {
    /// Create new relationship filterer
    ///
    /// Translation of: Constructor (lines 34-46)
    pub fn new(k: usize, track_relationships: bool) -> Self {
        Self {
            neighbors: Vec::with_capacity(k),
            filtering_spur_node: None,
            track_relationships,
        }
    }

    /// Add a blocking neighbor to avoid
    ///
    /// Translation of: `addBlockingNeighbor()` method (lines 47-50)
    pub fn add_blocking_neighbor(&mut self, path: &MutablePathResult, index_id: usize) {
        let avoid_id = if self.track_relationships {
            path.relationship(index_id)
                .map(BlockedIdentity::Relationship)
        } else {
            path.node(index_id + 1).map(BlockedIdentity::Node)
        };

        if let Some(avoid_id) = avoid_id {
            self.neighbors.push(avoid_id);
        }
    }

    /// Set the filtering spur node
    ///
    /// Translation of: `setFilter()` method (lines 52-56)
    pub fn set_filter(&mut self, filtering_spur_node: MappedNodeId) {
        self.filtering_spur_node = Some(filtering_spur_node);
        self.neighbors.clear();
    }

    /// Prepare the filter by sorting neighbors
    ///
    /// Translation of: `prepare()` method (lines 57-59)
    pub fn prepare(&mut self) {
        self.neighbors.sort_unstable();
    }

    /// Check if a relationship is valid (not blocked)
    ///
    /// Translation of: `validRelationship()` method (lines 60-80)
    pub fn valid_relationship(
        &self,
        source: MappedNodeId,
        target: MappedNodeId,
        relationship_id: RelationshipIndex,
    ) -> bool {
        if self.filtering_spur_node == Some(source) {
            let forbidden = if self.track_relationships {
                BlockedIdentity::Relationship(relationship_id)
            } else {
                BlockedIdentity::Node(target)
            };

            return self.neighbors.binary_search(&forbidden).is_err();
        }

        true
    }

    /// Reset the filter state
    pub fn reset(&mut self) {
        self.filtering_spur_node = None;
        self.neighbors.clear();
    }

    /// Get the number of blocked neighbors
    pub fn blocked_count(&self) -> usize {
        self.neighbors.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(value: u64) -> MappedNodeId {
        MappedNodeId::new(value)
    }

    fn relationship(value: u64) -> RelationshipIndex {
        RelationshipIndex::new(value)
    }

    #[test]
    fn test_relationship_filterer_creation() {
        let filterer = RelationshipFilterer::new(10, true);
        assert_eq!(filterer.blocked_count(), 0);
    }

    #[test]
    fn test_relationship_filterer_add_blocking_neighbor() {
        let mut filterer = RelationshipFilterer::new(5, true);

        let path = MutablePathResult::new(
            0,
            node(0),
            node(3),
            vec![node(0), node(1), node(2), node(3)],
            vec![relationship(10), relationship(11), relationship(12)],
            vec![0.0, 1.0, 2.0, 3.0],
        );

        filterer.add_blocking_neighbor(&path, 0);
        filterer.add_blocking_neighbor(&path, 1);

        assert_eq!(filterer.blocked_count(), 2);
    }

    #[test]
    fn test_relationship_filterer_set_filter() {
        let mut filterer = RelationshipFilterer::new(5, false);

        filterer.set_filter(node(5));
        assert_eq!(filterer.filtering_spur_node, Some(node(5)));
        assert_eq!(filterer.blocked_count(), 0);
    }

    #[test]
    fn test_relationship_filterer_valid_relationship() {
        let mut filterer = RelationshipFilterer::new(5, false);

        // Set up filter for node 1
        filterer.set_filter(node(1));

        // Add some blocked neighbors
        let path = MutablePathResult::new(
            0,
            node(0),
            node(3),
            vec![node(0), node(1), node(2), node(3)],
            vec![relationship(10), relationship(11), relationship(12)],
            vec![0.0, 1.0, 2.0, 3.0],
        );
        filterer.add_blocking_neighbor(&path, 1); // blocks target node 2
        filterer.prepare();

        // Test valid relationship (different source)
        assert!(filterer.valid_relationship(node(0), node(2), relationship(10)));

        // Test invalid relationship (blocked target)
        assert!(!filterer.valid_relationship(node(1), node(2), relationship(10)));

        // Test valid relationship (different target)
        assert!(filterer.valid_relationship(node(1), node(3), relationship(11)));
    }

    #[test]
    fn test_relationship_filterer_with_relationships() {
        let mut filterer = RelationshipFilterer::new(5, true);

        filterer.set_filter(node(1));

        let path = MutablePathResult::new(
            0,
            node(0),
            node(3),
            vec![node(0), node(1), node(2), node(3)],
            vec![relationship(10), relationship(11), relationship(12)],
            vec![0.0, 1.0, 2.0, 3.0],
        );
        filterer.add_blocking_neighbor(&path, 0); // blocks relationship 10
        filterer.prepare();

        // Test invalid relationship (blocked relationship)
        assert!(!filterer.valid_relationship(node(1), node(2), relationship(10)));

        // Test valid relationship (different relationship)
        assert!(filterer.valid_relationship(node(1), node(2), relationship(15)));
    }

    #[test]
    fn test_relationship_filterer_is_order_independent() {
        let mut filterer = RelationshipFilterer::new(5, false);
        filterer.set_filter(node(1));

        let path = MutablePathResult::new(
            0,
            node(0),
            node(4),
            vec![node(0), node(1), node(2), node(3), node(4)],
            vec![
                relationship(10),
                relationship(11),
                relationship(12),
                relationship(13),
            ],
            vec![0.0, 1.0, 2.0, 3.0, 4.0],
        );
        filterer.add_blocking_neighbor(&path, 1);
        filterer.add_blocking_neighbor(&path, 2);
        filterer.prepare();

        assert!(filterer.valid_relationship(node(1), node(4), relationship(20)));
        assert!(!filterer.valid_relationship(node(1), node(2), relationship(10)));
        assert!(!filterer.valid_relationship(node(1), node(3), relationship(11)));
    }
}
