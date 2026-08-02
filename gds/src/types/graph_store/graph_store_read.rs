use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

use crate::projection::NodeLabel;
use crate::projection::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::IdMap;
use crate::types::graph::Graph;
use crate::types::properties::graph::GraphPropertyValues;
use crate::types::properties::node::NodePropertyValues;
use crate::types::properties::relationship::RelationshipPropertyValues;
use crate::types::schema::GraphSchema;
use crate::types::ValueType;

use super::Capabilities;
use super::DatabaseInfo;
use super::GraphStore;
use super::GraphStoreResult;
use super::GraphViewResult;
use super::GraphViewSpec;

/// Object-safe observation and graph-view capability for graph stores.
///
/// Algorithms and procedure facades should depend on this contract when they
/// do not mutate or derive a store.
pub trait GraphStoreRead: Send + Sync {
    fn database_info(&self) -> &DatabaseInfo;
    fn schema(&self) -> &GraphSchema;
    fn creation_time(&self) -> chrono::DateTime<chrono::Utc>;
    fn modification_time(&self) -> chrono::DateTime<chrono::Utc>;
    fn capabilities(&self) -> &Capabilities;
    fn nodes(&self) -> Arc<dyn IdMap>;

    fn relationships(&self) -> HashSet<RelationshipType> {
        self.relationship_types()
    }

    fn graph_property_keys(&self) -> HashSet<String>;
    fn has_graph_property(&self, property_key: &str) -> bool;
    fn graph_property_type(&self, property_key: &str) -> GraphStoreResult<ValueType>;
    fn graph_property_values(
        &self,
        property_key: &str,
    ) -> GraphStoreResult<Arc<dyn GraphPropertyValues>>;

    fn node_count(&self) -> usize;
    fn node_count_for_label(&self, label: &NodeLabel) -> usize;
    fn node_labels(&self) -> HashSet<NodeLabel>;
    fn has_node_label(&self, label: &NodeLabel) -> bool;
    fn node_property_keys(&self) -> HashSet<String>;
    fn node_property_keys_for_label(&self, label: &NodeLabel) -> HashSet<String>;
    fn node_property_keys_for_labels(&self, labels: &HashSet<NodeLabel>) -> HashSet<String>;
    fn has_node_property(&self, property_key: &str) -> bool;
    fn has_node_property_for_label(&self, label: &NodeLabel, property_key: &str) -> bool;
    fn node_property_type(&self, property_key: &str) -> GraphStoreResult<ValueType>;
    fn node_property_values(
        &self,
        property_key: &str,
    ) -> GraphStoreResult<Arc<dyn NodePropertyValues>>;

    fn relationship_count(&self) -> usize;
    fn relationship_count_for_type(&self, relationship_type: &RelationshipType) -> usize;
    fn relationship_types(&self) -> HashSet<RelationshipType>;
    fn has_relationship_type(&self, relationship_type: &RelationshipType) -> bool;
    fn inverse_indexed_relationship_types(&self) -> HashSet<RelationshipType>;
    fn relationship_property_keys(&self) -> HashSet<String>;
    fn relationship_property_keys_for_type(&self, rel_type: &RelationshipType) -> HashSet<String>;
    fn relationship_property_keys_for_types(
        &self,
        rel_types: &HashSet<RelationshipType>,
    ) -> HashSet<String>;
    fn has_relationship_property(&self, rel_type: &RelationshipType, property_key: &str) -> bool;
    fn relationship_property_type(
        &self,
        relationship_type: &RelationshipType,
        property_key: &str,
    ) -> GraphStoreResult<ValueType>;
    fn relationship_property_values(
        &self,
        relationship_type: &RelationshipType,
        property_key: &str,
    ) -> GraphStoreResult<Arc<dyn RelationshipPropertyValues>>;

    fn get_graph(&self) -> Arc<dyn Graph>;
    fn get_graph_view(&self, spec: &GraphViewSpec) -> GraphViewResult<Arc<dyn Graph>>;

    fn get_graph_with_types(
        &self,
        relationship_types: &HashSet<RelationshipType>,
    ) -> GraphViewResult<Arc<dyn Graph>> {
        self.get_graph_view(
            &GraphViewSpec::new().with_relationship_types(relationship_types.clone()),
        )
    }

    fn get_graph_with_types_and_selectors(
        &self,
        relationship_types: &HashSet<RelationshipType>,
        relationship_property_selectors: &HashMap<RelationshipType, String>,
    ) -> GraphViewResult<Arc<dyn Graph>> {
        self.get_graph_view(
            &GraphViewSpec::new()
                .with_relationship_types(relationship_types.clone())
                .with_relationship_property_selectors(relationship_property_selectors.clone()),
        )
    }

    fn get_graph_with_types_and_orientation(
        &self,
        relationship_types: &HashSet<RelationshipType>,
        orientation: Orientation,
    ) -> GraphViewResult<Arc<dyn Graph>> {
        self.get_graph_view(
            &GraphViewSpec::new()
                .with_relationship_types(relationship_types.clone())
                .with_orientation(orientation),
        )
    }

    fn get_graph_with_types_selectors_and_orientation(
        &self,
        relationship_types: &HashSet<RelationshipType>,
        relationship_property_selectors: &HashMap<RelationshipType, String>,
        orientation: Orientation,
    ) -> GraphViewResult<Arc<dyn Graph>> {
        self.get_graph_view(
            &GraphViewSpec::new()
                .with_relationship_types(relationship_types.clone())
                .with_relationship_property_selectors(relationship_property_selectors.clone())
                .with_orientation(orientation),
        )
    }
}

impl<Store> GraphStoreRead for Store
where
    Store: GraphStore + ?Sized,
{
    fn database_info(&self) -> &DatabaseInfo {
        GraphStore::database_info(self)
    }

    fn schema(&self) -> &GraphSchema {
        GraphStore::schema(self)
    }

    fn creation_time(&self) -> chrono::DateTime<chrono::Utc> {
        GraphStore::creation_time(self)
    }

    fn modification_time(&self) -> chrono::DateTime<chrono::Utc> {
        GraphStore::modification_time(self)
    }

    fn capabilities(&self) -> &Capabilities {
        GraphStore::capabilities(self)
    }

    fn nodes(&self) -> Arc<dyn IdMap> {
        GraphStore::nodes(self)
    }

    fn graph_property_keys(&self) -> HashSet<String> {
        GraphStore::graph_property_keys(self)
    }

    fn has_graph_property(&self, property_key: &str) -> bool {
        GraphStore::has_graph_property(self, property_key)
    }

    fn graph_property_type(&self, property_key: &str) -> GraphStoreResult<ValueType> {
        GraphStore::graph_property_type(self, property_key)
    }

    fn graph_property_values(
        &self,
        property_key: &str,
    ) -> GraphStoreResult<Arc<dyn GraphPropertyValues>> {
        GraphStore::graph_property_values(self, property_key)
    }

    fn node_count(&self) -> usize {
        GraphStore::node_count(self)
    }

    fn node_count_for_label(&self, label: &NodeLabel) -> usize {
        GraphStore::node_count_for_label(self, label)
    }

    fn node_labels(&self) -> HashSet<NodeLabel> {
        GraphStore::node_labels(self)
    }

    fn has_node_label(&self, label: &NodeLabel) -> bool {
        GraphStore::has_node_label(self, label)
    }

    fn node_property_keys(&self) -> HashSet<String> {
        GraphStore::node_property_keys(self)
    }

    fn node_property_keys_for_label(&self, label: &NodeLabel) -> HashSet<String> {
        GraphStore::node_property_keys_for_label(self, label)
    }

    fn node_property_keys_for_labels(&self, labels: &HashSet<NodeLabel>) -> HashSet<String> {
        GraphStore::node_property_keys_for_labels(self, labels)
    }

    fn has_node_property(&self, property_key: &str) -> bool {
        GraphStore::has_node_property(self, property_key)
    }

    fn has_node_property_for_label(&self, label: &NodeLabel, property_key: &str) -> bool {
        GraphStore::has_node_property_for_label(self, label, property_key)
    }

    fn node_property_type(&self, property_key: &str) -> GraphStoreResult<ValueType> {
        GraphStore::node_property_type(self, property_key)
    }

    fn node_property_values(
        &self,
        property_key: &str,
    ) -> GraphStoreResult<Arc<dyn NodePropertyValues>> {
        GraphStore::node_property_values(self, property_key)
    }

    fn relationship_count(&self) -> usize {
        GraphStore::relationship_count(self)
    }

    fn relationship_count_for_type(&self, relationship_type: &RelationshipType) -> usize {
        GraphStore::relationship_count_for_type(self, relationship_type)
    }

    fn relationship_types(&self) -> HashSet<RelationshipType> {
        GraphStore::relationship_types(self)
    }

    fn has_relationship_type(&self, relationship_type: &RelationshipType) -> bool {
        GraphStore::has_relationship_type(self, relationship_type)
    }

    fn inverse_indexed_relationship_types(&self) -> HashSet<RelationshipType> {
        GraphStore::inverse_indexed_relationship_types(self)
    }

    fn relationship_property_keys(&self) -> HashSet<String> {
        GraphStore::relationship_property_keys(self)
    }

    fn relationship_property_keys_for_type(&self, rel_type: &RelationshipType) -> HashSet<String> {
        GraphStore::relationship_property_keys_for_type(self, rel_type)
    }

    fn relationship_property_keys_for_types(
        &self,
        rel_types: &HashSet<RelationshipType>,
    ) -> HashSet<String> {
        GraphStore::relationship_property_keys_for_types(self, rel_types)
    }

    fn has_relationship_property(&self, rel_type: &RelationshipType, property_key: &str) -> bool {
        GraphStore::has_relationship_property(self, rel_type, property_key)
    }

    fn relationship_property_type(
        &self,
        relationship_type: &RelationshipType,
        property_key: &str,
    ) -> GraphStoreResult<ValueType> {
        GraphStore::relationship_property_type(self, relationship_type, property_key)
    }

    fn relationship_property_values(
        &self,
        relationship_type: &RelationshipType,
        property_key: &str,
    ) -> GraphStoreResult<Arc<dyn RelationshipPropertyValues>> {
        GraphStore::relationship_property_values(self, relationship_type, property_key)
    }

    fn get_graph(&self) -> Arc<dyn Graph> {
        GraphStore::get_graph(self)
    }

    fn get_graph_view(&self, spec: &GraphViewSpec) -> GraphViewResult<Arc<dyn Graph>> {
        GraphStore::get_graph_view(self, spec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::graph_store::DefaultGraphStore;
    use crate::types::random::RandomGraphConfig;

    #[test]
    fn default_store_supports_object_safe_read_access() {
        let concrete = Arc::new(
            DefaultGraphStore::random(&RandomGraphConfig::default()).expect("random graph store"),
        );
        let expected_node_count = GraphStore::node_count(concrete.as_ref());
        let store: Arc<dyn GraphStoreRead> = concrete;

        assert_eq!(store.node_count(), expected_node_count);
        assert_eq!(store.get_graph().node_count(), expected_node_count);
    }
}
