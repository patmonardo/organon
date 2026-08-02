use crate::projection::RelationshipType;
use crate::types::graph::MappedNodeId;
use crate::types::graph::RelationshipTopology;
use crate::types::schema::Direction;
use crate::types::schema::RelationshipPropertySchema;
use std::collections::HashMap;

use super::DefaultGraphStore;
use super::GraphName;
use super::GraphStore;
use super::GraphStoreResult;

/// Shell-control capability seam for graph-store snapshot operations.
///
/// This trait captures the immutable-to-successor operations that current
/// miscellaneous control procedures need when materializing derived graph
/// stores. `DefaultGraphStore` is the initial implementation; additional store
/// backends can implement this trait to participate in the same control flow.
pub trait ShellStoreControl: GraphStore + Sized {
    fn with_added_relationship_type(
        &self,
        graph_name: GraphName,
        rel_type: RelationshipType,
        outgoing: Vec<Vec<MappedNodeId>>,
        direction: Direction,
    ) -> GraphStoreResult<Self>;

    fn with_added_relationship_type_and_properties(
        &self,
        graph_name: GraphName,
        rel_type: RelationshipType,
        outgoing: Vec<Vec<MappedNodeId>>,
        direction: Direction,
        property_schemas: Vec<RelationshipPropertySchema>,
    ) -> GraphStoreResult<Self>;

    fn with_rebuilt_relationship_topologies(
        &self,
        graph_name: GraphName,
        relationship_topologies: HashMap<RelationshipType, RelationshipTopology>,
    ) -> GraphStoreResult<Self>;
}

impl ShellStoreControl for DefaultGraphStore {
    fn with_added_relationship_type(
        &self,
        graph_name: GraphName,
        rel_type: RelationshipType,
        outgoing: Vec<Vec<MappedNodeId>>,
        direction: Direction,
    ) -> GraphStoreResult<Self> {
        DefaultGraphStore::with_added_relationship_type(
            self, graph_name, rel_type, outgoing, direction,
        )
    }

    fn with_added_relationship_type_and_properties(
        &self,
        graph_name: GraphName,
        rel_type: RelationshipType,
        outgoing: Vec<Vec<MappedNodeId>>,
        direction: Direction,
        property_schemas: Vec<RelationshipPropertySchema>,
    ) -> GraphStoreResult<Self> {
        DefaultGraphStore::with_added_relationship_type_and_properties(
            self,
            graph_name,
            rel_type,
            outgoing,
            direction,
            property_schemas,
        )
    }

    fn with_rebuilt_relationship_topologies(
        &self,
        graph_name: GraphName,
        relationship_topologies: HashMap<RelationshipType, RelationshipTopology>,
    ) -> GraphStoreResult<Self> {
        DefaultGraphStore::with_rebuilt_relationship_topologies(
            self,
            graph_name,
            relationship_topologies,
        )
    }
}
