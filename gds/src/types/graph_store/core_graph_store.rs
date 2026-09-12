//! Immutable, componentized enterprise graph snapshot.
//!
//! This first CoreGraphStore slice establishes snapshot identity, a dense node
//! domain, relationship-type partitions, and genuine compressed sparse row
//! topology. Property backplanes and persistence are deliberately subsequent
//! components.

use std::collections::{BTreeMap, HashSet};
use std::fmt;
use std::sync::Arc;

use chrono::{DateTime, Utc};

use crate::config::GraphStoreConfig;
use crate::projection::{NodeLabel, RelationshipType};
use crate::types::graph::id_map::{
    IdMap, MappedNodeId, OriginalNodeId, PartialIdMap, RelationshipIndex, SimpleIdMap,
    SimpleIdMapError,
};
use crate::types::graph::Graph;
use crate::types::properties::graph::GraphPropertyValues;
use crate::types::properties::node::NodePropertyValues;
use crate::types::properties::relationship::RelationshipPropertyValues;
use crate::types::schema::GraphSchema;
use crate::types::ValueType;

use super::{
    Capabilities, CoreGraph, DatabaseInfo, GraphName, GraphStoreError, GraphStoreRead,
    GraphStoreResult, GraphViewError, GraphViewResult, GraphViewSpec,
};

/// Immutable identity of one graph snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphSnapshotIdentity {
    graph_name: GraphName,
    snapshot_id: String,
    version: u64,
    created_at: DateTime<Utc>,
}

impl GraphSnapshotIdentity {
    pub fn new(
        graph_name: impl Into<GraphName>,
        snapshot_id: impl Into<String>,
        version: u64,
    ) -> Self {
        Self::at(graph_name, snapshot_id, version, Utc::now())
    }

    pub fn at(
        graph_name: impl Into<GraphName>,
        snapshot_id: impl Into<String>,
        version: u64,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            graph_name: graph_name.into(),
            snapshot_id: snapshot_id.into(),
            version,
            created_at,
        }
    }

    pub fn graph_name(&self) -> &GraphName {
        &self.graph_name
    }

    pub fn snapshot_id(&self) -> &str {
        &self.snapshot_id
    }

    pub const fn version(&self) -> u64 {
        self.version
    }

    pub const fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

/// Immutable dense mapping between database node IDs and `[0, node_count)`.
#[derive(Debug, Clone)]
pub struct DenseNodeMap {
    inner: Arc<SimpleIdMap>,
}

impl DenseNodeMap {
    pub fn try_from_original_ids<I, T>(ids: I) -> Result<Self, CoreGraphStoreError>
    where
        I: IntoIterator<Item = T>,
        T: Into<OriginalNodeId>,
    {
        Ok(Self {
            inner: Arc::new(SimpleIdMap::try_from_original_ids(ids)?),
        })
    }

    pub fn node_count(&self) -> usize {
        self.inner.node_count()
    }

    pub fn to_mapped(&self, original: OriginalNodeId) -> Option<MappedNodeId> {
        self.inner.to_mapped_node_id(original)
    }

    pub fn to_original(&self, mapped: MappedNodeId) -> Option<OriginalNodeId> {
        self.inner.to_original_node_id(mapped)
    }

    pub(crate) fn simple_arc(&self) -> Arc<SimpleIdMap> {
        Arc::clone(&self.inner)
    }
}

/// One mapped relationship before physical partition construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MappedRelationship {
    pub source: MappedNodeId,
    pub target: MappedNodeId,
}

impl MappedRelationship {
    pub const fn new(source: MappedNodeId, target: MappedNodeId) -> Self {
        Self { source, target }
    }
}

/// One typed relationship expressed with external node identities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedRelationshipInput {
    pub relationship_type: RelationshipType,
    pub source: OriginalNodeId,
    pub target: OriginalNodeId,
}

impl TypedRelationshipInput {
    pub fn new(
        relationship_type: impl Into<String>,
        source: impl Into<OriginalNodeId>,
        target: impl Into<OriginalNodeId>,
    ) -> Self {
        Self {
            relationship_type: RelationshipType::of(relationship_type),
            source: source.into(),
            target: target.into(),
        }
    }
}

/// Compressed sparse row adjacency with stable logical relationship indices.
#[derive(Debug, Clone)]
pub struct CsrTopology {
    node_count: usize,
    offsets: Arc<[usize]>,
    neighbors: Arc<[MappedNodeId]>,
    relationship_indices: Arc<[RelationshipIndex]>,
}

impl CsrTopology {
    fn try_from_indexed_edges(
        node_count: usize,
        edges: &[(MappedNodeId, MappedNodeId, RelationshipIndex)],
    ) -> Result<Self, CoreGraphStoreError> {
        let mut counts = vec![0usize; node_count];
        for &(source, target, _) in edges {
            let source_index = checked_node(source, node_count)?;
            checked_node(target, node_count)?;
            counts[source_index] = counts[source_index]
                .checked_add(1)
                .ok_or(CoreGraphStoreError::RelationshipCountOverflow)?;
        }

        let mut offsets = Vec::with_capacity(node_count + 1);
        offsets.push(0usize);
        for count in counts {
            let next = offsets
                .last()
                .copied()
                .expect("CSR starts with offset zero")
                .checked_add(count)
                .ok_or(CoreGraphStoreError::RelationshipCountOverflow)?;
            offsets.push(next);
        }

        let mut next = offsets[..node_count].to_vec();
        let mut neighbors = vec![MappedNodeId::ZERO; edges.len()];
        let mut relationship_indices = vec![RelationshipIndex::ZERO; edges.len()];
        for &(source, target, relationship_index) in edges {
            let source_index = checked_node(source, node_count)?;
            let position = next[source_index];
            neighbors[position] = target;
            relationship_indices[position] = relationship_index;
            next[source_index] += 1;
        }

        Ok(Self {
            node_count,
            offsets: offsets.into(),
            neighbors: neighbors.into(),
            relationship_indices: relationship_indices.into(),
        })
    }

    pub const fn node_count(&self) -> usize {
        self.node_count
    }

    pub fn relationship_count(&self) -> usize {
        self.neighbors.len()
    }

    pub fn offsets(&self) -> &[usize] {
        &self.offsets
    }

    pub fn neighbors(&self, node: MappedNodeId) -> Option<&[MappedNodeId]> {
        let index = node.to_usize()?;
        let start = *self.offsets.get(index)?;
        let end = *self.offsets.get(index.checked_add(1)?)?;
        self.neighbors.get(start..end)
    }

    pub fn relationship_indices(&self, node: MappedNodeId) -> Option<&[RelationshipIndex]> {
        let index = node.to_usize()?;
        let start = *self.offsets.get(index)?;
        let end = *self.offsets.get(index.checked_add(1)?)?;
        self.relationship_indices.get(start..end)
    }

    pub fn has_parallel_edges(&self) -> bool {
        (0..self.node_count).any(|node| {
            let mut neighbors = self
                .neighbors(MappedNodeId::new(node as u64))
                .unwrap_or_default()
                .to_vec();
            neighbors.sort_unstable();
            neighbors.windows(2).any(|window| window[0] == window[1])
        })
    }

    pub(crate) fn transpose(&self) -> Self {
        Self::try_from_indexed_edges(self.node_count, &self.indexed_edges(true))
            .expect("transposing validated CSR preserves its node domain")
    }

    pub(crate) fn undirected(&self) -> Self {
        let mut edges = self.indexed_edges(false);
        edges.extend(
            self.indexed_edges(true)
                .into_iter()
                .filter(|(source, target, _)| source != target),
        );
        Self::try_from_indexed_edges(self.node_count, &edges)
            .expect("symmetrizing validated CSR preserves its node domain")
    }

    fn indexed_edges(
        &self,
        transpose: bool,
    ) -> Vec<(MappedNodeId, MappedNodeId, RelationshipIndex)> {
        let mut edges = Vec::with_capacity(self.relationship_count());
        for node in 0..self.node_count {
            let source = MappedNodeId::new(node as u64);
            let neighbors = self.neighbors(source).unwrap_or_default();
            let indices = self.relationship_indices(source).unwrap_or_default();
            for (&target, &index) in neighbors.iter().zip(indices) {
                edges.push(if transpose {
                    (target, source, index)
                } else {
                    (source, target, index)
                });
            }
        }
        edges
    }
}

/// All topology for one relationship type.
#[derive(Debug, Clone)]
pub struct RelationshipPartition {
    relationship_type: RelationshipType,
    forward: CsrTopology,
    inverse: Option<CsrTopology>,
}

impl RelationshipPartition {
    pub fn try_new(
        relationship_type: RelationshipType,
        node_count: usize,
        mut relationships: Vec<MappedRelationship>,
        build_inverse: bool,
    ) -> Result<Self, CoreGraphStoreError> {
        // Stable source ordering makes RelationshipIndex equal the physical
        // forward-CSR position while preserving import order within each row.
        relationships.sort_by_key(|relationship| relationship.source);
        let indexed = relationships
            .iter()
            .enumerate()
            .map(|(index, relationship)| {
                let index = RelationshipIndex::try_from(index)
                    .map_err(|_| CoreGraphStoreError::RelationshipCountOverflow)?;
                Ok((relationship.source, relationship.target, index))
            })
            .collect::<Result<Vec<_>, CoreGraphStoreError>>()?;
        let forward = CsrTopology::try_from_indexed_edges(node_count, &indexed)?;
        let inverse = build_inverse
            .then(|| {
                let reversed = indexed
                    .iter()
                    .map(|&(source, target, index)| (target, source, index))
                    .collect::<Vec<_>>();
                CsrTopology::try_from_indexed_edges(node_count, &reversed)
            })
            .transpose()?;

        Ok(Self {
            relationship_type,
            forward,
            inverse,
        })
    }

    pub fn relationship_type(&self) -> &RelationshipType {
        &self.relationship_type
    }

    pub fn forward(&self) -> &CsrTopology {
        &self.forward
    }

    pub fn inverse(&self) -> Option<&CsrTopology> {
        self.inverse.as_ref()
    }

    pub fn relationship_count(&self) -> usize {
        self.forward.relationship_count()
    }
}

#[derive(Debug)]
pub enum CoreGraphStoreError {
    EmptyGraphName,
    EmptySnapshotId,
    DuplicateOriginalId(OriginalNodeId),
    NodeCountOverflow,
    RelationshipCountOverflow,
    NodeOutOfRange {
        node: MappedNodeId,
        node_count: usize,
    },
    UnknownOriginalNode(OriginalNodeId),
    DuplicateRelationshipPartition(RelationshipType),
    PartitionNodeCountMismatch {
        relationship_type: RelationshipType,
        expected: usize,
        actual: usize,
    },
}

impl fmt::Display for CoreGraphStoreError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyGraphName => formatter.write_str("graph snapshot requires a graph name"),
            Self::EmptySnapshotId => formatter.write_str("graph snapshot requires a snapshot ID"),
            Self::DuplicateOriginalId(id) => write!(formatter, "duplicate original node ID {id}"),
            Self::NodeCountOverflow => formatter.write_str("node count exceeds mapped ID space"),
            Self::RelationshipCountOverflow => {
                formatter.write_str("relationship count exceeds relationship index space")
            }
            Self::NodeOutOfRange { node, node_count } => {
                write!(
                    formatter,
                    "mapped node {node} is outside node count {node_count}"
                )
            }
            Self::UnknownOriginalNode(node) => write!(formatter, "unknown original node ID {node}"),
            Self::DuplicateRelationshipPartition(relationship_type) => write!(
                formatter,
                "duplicate relationship partition {relationship_type}"
            ),
            Self::PartitionNodeCountMismatch {
                relationship_type,
                expected,
                actual,
            } => write!(
                formatter,
                "relationship partition {relationship_type} has {actual} nodes; expected {expected}"
            ),
        }
    }
}

impl std::error::Error for CoreGraphStoreError {}

impl From<SimpleIdMapError> for CoreGraphStoreError {
    fn from(value: SimpleIdMapError) -> Self {
        match value {
            SimpleIdMapError::DuplicateOriginalId(id) => Self::DuplicateOriginalId(id),
            SimpleIdMapError::NodeCountOverflow => Self::NodeCountOverflow,
        }
    }
}

/// Immutable enterprise graph product assembled from independently validated parts.
#[derive(Debug, Clone)]
pub struct CoreGraphStore {
    config: Arc<GraphStoreConfig>,
    identity: GraphSnapshotIdentity,
    database_info: DatabaseInfo,
    schema: Arc<GraphSchema>,
    capabilities: Capabilities,
    nodes: DenseNodeMap,
    partitions: BTreeMap<RelationshipType, Arc<RelationshipPartition>>,
}

impl CoreGraphStore {
    #[allow(clippy::too_many_arguments)]
    pub fn try_new(
        config: GraphStoreConfig,
        identity: GraphSnapshotIdentity,
        database_info: DatabaseInfo,
        schema: GraphSchema,
        capabilities: Capabilities,
        nodes: DenseNodeMap,
        partitions: Vec<RelationshipPartition>,
    ) -> Result<Self, CoreGraphStoreError> {
        validate_identity(&identity)?;
        let node_count = nodes.node_count();
        let mut partition_map = BTreeMap::new();
        for partition in partitions {
            let relationship_type = partition.relationship_type.clone();
            if partition.forward.node_count() != node_count {
                return Err(CoreGraphStoreError::PartitionNodeCountMismatch {
                    relationship_type,
                    expected: node_count,
                    actual: partition.forward.node_count(),
                });
            }
            if partition_map.contains_key(&relationship_type) {
                return Err(CoreGraphStoreError::DuplicateRelationshipPartition(
                    relationship_type,
                ));
            }
            partition_map.insert(relationship_type, Arc::new(partition));
        }

        Ok(Self {
            config: Arc::new(config),
            identity,
            database_info,
            schema: Arc::new(schema),
            capabilities,
            nodes,
            partitions: partition_map,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn try_from_inputs<I, T>(
        config: GraphStoreConfig,
        identity: GraphSnapshotIdentity,
        database_info: DatabaseInfo,
        schema: GraphSchema,
        capabilities: Capabilities,
        original_node_ids: I,
        relationships: impl IntoIterator<Item = TypedRelationshipInput>,
        inverse_indexed_types: &HashSet<RelationshipType>,
    ) -> Result<Self, CoreGraphStoreError>
    where
        I: IntoIterator<Item = T>,
        T: Into<OriginalNodeId>,
    {
        let nodes = DenseNodeMap::try_from_original_ids(original_node_ids)?;
        let mut grouped = BTreeMap::<RelationshipType, Vec<MappedRelationship>>::new();
        for relationship in relationships {
            let source = nodes.to_mapped(relationship.source).ok_or(
                CoreGraphStoreError::UnknownOriginalNode(relationship.source),
            )?;
            let target = nodes.to_mapped(relationship.target).ok_or(
                CoreGraphStoreError::UnknownOriginalNode(relationship.target),
            )?;
            grouped
                .entry(relationship.relationship_type)
                .or_default()
                .push(MappedRelationship::new(source, target));
        }
        let partitions = grouped
            .into_iter()
            .map(|(relationship_type, edges)| {
                let inverse = inverse_indexed_types.contains(&relationship_type);
                RelationshipPartition::try_new(
                    relationship_type,
                    nodes.node_count(),
                    edges,
                    inverse,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;

        Self::try_new(
            config,
            identity,
            database_info,
            schema,
            capabilities,
            nodes,
            partitions,
        )
    }

    pub fn identity(&self) -> &GraphSnapshotIdentity {
        &self.identity
    }

    pub fn config(&self) -> &GraphStoreConfig {
        &self.config
    }

    pub fn node_domain(&self) -> &DenseNodeMap {
        &self.nodes
    }

    pub fn partitions(&self) -> &BTreeMap<RelationshipType, Arc<RelationshipPartition>> {
        &self.partitions
    }

    pub fn partition(
        &self,
        relationship_type: &RelationshipType,
    ) -> Option<&RelationshipPartition> {
        self.partitions.get(relationship_type).map(Arc::as_ref)
    }

    fn core_graph(&self, spec: &GraphViewSpec) -> GraphViewResult<Arc<dyn Graph>> {
        let selected_types = if spec.relationship_types().is_empty() {
            self.partitions.keys().cloned().collect::<HashSet<_>>()
        } else {
            for relationship_type in spec.relationship_types() {
                if !self.partitions.contains_key(relationship_type) {
                    return Err(GraphViewError::RelationshipTypeNotMaterialized(
                        relationship_type.name().to_string(),
                    ));
                }
            }
            spec.relationship_types().clone()
        };

        for (relationship_type, property_key) in spec.relationship_property_selectors() {
            if !selected_types.contains(relationship_type) {
                return Err(GraphViewError::SelectorForUnselectedType(
                    relationship_type.name().to_string(),
                ));
            }
            return Err(GraphViewError::RelationshipPropertyNotMaterialized {
                relationship_type: relationship_type.name().to_string(),
                property_key: property_key.clone(),
            });
        }

        let partitions = self
            .partitions
            .iter()
            .filter(|(relationship_type, _)| selected_types.contains(*relationship_type))
            .map(|(_, partition)| Arc::clone(partition))
            .collect::<Vec<_>>();
        let schema_types = selected_types
            .iter()
            .map(|kind| RelationshipType::of(kind.name()))
            .collect();
        let schema = Arc::new(self.schema.filter_relationship_types(&schema_types));

        Ok(Arc::new(CoreGraph::from_partitions(
            schema,
            self.nodes.simple_arc(),
            partitions,
            spec.orientation(),
        )))
    }
}

fn validate_identity(identity: &GraphSnapshotIdentity) -> Result<(), CoreGraphStoreError> {
    if identity.graph_name.value().is_empty() {
        return Err(CoreGraphStoreError::EmptyGraphName);
    }
    if identity.snapshot_id.is_empty() {
        return Err(CoreGraphStoreError::EmptySnapshotId);
    }
    Ok(())
}

fn checked_node(node: MappedNodeId, node_count: usize) -> Result<usize, CoreGraphStoreError> {
    let index = node
        .to_usize()
        .filter(|&index| index < node_count)
        .ok_or(CoreGraphStoreError::NodeOutOfRange { node, node_count })?;
    Ok(index)
}

impl GraphStoreRead for CoreGraphStore {
    fn database_info(&self) -> &DatabaseInfo {
        &self.database_info
    }

    fn schema(&self) -> &GraphSchema {
        &self.schema
    }

    fn creation_time(&self) -> DateTime<Utc> {
        self.identity.created_at
    }

    fn modification_time(&self) -> DateTime<Utc> {
        self.identity.created_at
    }

    fn capabilities(&self) -> &Capabilities {
        &self.capabilities
    }

    fn nodes(&self) -> Arc<dyn IdMap> {
        self.nodes.simple_arc()
    }

    fn graph_property_keys(&self) -> HashSet<String> {
        HashSet::new()
    }

    fn has_graph_property(&self, _property_key: &str) -> bool {
        false
    }

    fn graph_property_type(&self, property_key: &str) -> GraphStoreResult<ValueType> {
        Err(GraphStoreError::PropertyNotFound(property_key.to_string()))
    }

    fn graph_property_values(
        &self,
        property_key: &str,
    ) -> GraphStoreResult<Arc<dyn GraphPropertyValues>> {
        Err(GraphStoreError::PropertyNotFound(property_key.to_string()))
    }

    fn node_count(&self) -> usize {
        self.nodes.node_count()
    }

    fn node_count_for_label(&self, label: &NodeLabel) -> usize {
        self.nodes.inner.node_count_for_label(label)
    }

    fn node_labels(&self) -> HashSet<NodeLabel> {
        self.nodes.inner.available_node_labels()
    }

    fn has_node_label(&self, label: &NodeLabel) -> bool {
        self.node_labels().contains(label)
    }

    fn node_property_keys(&self) -> HashSet<String> {
        HashSet::new()
    }

    fn node_property_keys_for_label(&self, _label: &NodeLabel) -> HashSet<String> {
        HashSet::new()
    }

    fn node_property_keys_for_labels(&self, _labels: &HashSet<NodeLabel>) -> HashSet<String> {
        HashSet::new()
    }

    fn has_node_property(&self, _property_key: &str) -> bool {
        false
    }

    fn has_node_property_for_label(&self, _label: &NodeLabel, _property_key: &str) -> bool {
        false
    }

    fn node_property_type(&self, property_key: &str) -> GraphStoreResult<ValueType> {
        Err(GraphStoreError::PropertyNotFound(property_key.to_string()))
    }

    fn node_property_values(
        &self,
        property_key: &str,
    ) -> GraphStoreResult<Arc<dyn NodePropertyValues>> {
        Err(GraphStoreError::PropertyNotFound(property_key.to_string()))
    }

    fn relationship_count(&self) -> usize {
        self.partitions
            .values()
            .map(|partition| partition.relationship_count())
            .sum()
    }

    fn relationship_count_for_type(&self, relationship_type: &RelationshipType) -> usize {
        self.partition(relationship_type)
            .map_or(0, RelationshipPartition::relationship_count)
    }

    fn relationship_types(&self) -> HashSet<RelationshipType> {
        self.partitions.keys().cloned().collect()
    }

    fn has_relationship_type(&self, relationship_type: &RelationshipType) -> bool {
        self.partitions.contains_key(relationship_type)
    }

    fn inverse_indexed_relationship_types(&self) -> HashSet<RelationshipType> {
        self.partitions
            .iter()
            .filter(|(_, partition)| partition.inverse().is_some())
            .map(|(relationship_type, _)| relationship_type.clone())
            .collect()
    }

    fn relationship_property_keys(&self) -> HashSet<String> {
        HashSet::new()
    }

    fn relationship_property_keys_for_type(&self, _rel_type: &RelationshipType) -> HashSet<String> {
        HashSet::new()
    }

    fn relationship_property_keys_for_types(
        &self,
        _rel_types: &HashSet<RelationshipType>,
    ) -> HashSet<String> {
        HashSet::new()
    }

    fn has_relationship_property(&self, _rel_type: &RelationshipType, _property_key: &str) -> bool {
        false
    }

    fn relationship_property_type(
        &self,
        relationship_type: &RelationshipType,
        property_key: &str,
    ) -> GraphStoreResult<ValueType> {
        Err(GraphStoreError::PropertyNotFound(format!(
            "{relationship_type}.{property_key}"
        )))
    }

    fn relationship_property_values(
        &self,
        relationship_type: &RelationshipType,
        property_key: &str,
    ) -> GraphStoreResult<Arc<dyn RelationshipPropertyValues>> {
        Err(GraphStoreError::PropertyNotFound(format!(
            "{relationship_type}.{property_key}"
        )))
    }

    fn get_graph(&self) -> Arc<dyn Graph> {
        self.core_graph(&GraphViewSpec::new())
            .expect("an unfiltered CoreGraphStore view is always valid")
    }

    fn get_graph_view(&self, spec: &GraphViewSpec) -> GraphViewResult<Arc<dyn Graph>> {
        self.core_graph(spec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::graphframe::GraphFrame;
    use crate::types::graph_store::{DatabaseId, DatabaseLocation};

    fn database_info() -> DatabaseInfo {
        DatabaseInfo::new(
            DatabaseId::new("enterprise-test"),
            DatabaseLocation::remote("localhost", 7687, None, None),
        )
    }

    fn store() -> CoreGraphStore {
        let cites = RelationshipType::of("CITES");
        CoreGraphStore::try_from_inputs(
            GraphStoreConfig::default(),
            GraphSnapshotIdentity::new("knowledge", "snapshot-7", 7),
            database_info(),
            GraphSchema::empty(),
            Capabilities::new(),
            [100_i64, 200, 300, 400],
            [
                TypedRelationshipInput::new("CITES", 300_i64, 200_i64),
                TypedRelationshipInput::new("CITES", 100_i64, 200_i64),
                TypedRelationshipInput::new("CITES", 100_i64, 300_i64),
            ],
            &HashSet::from([cites]),
        )
        .expect("valid enterprise graph snapshot")
    }

    #[test]
    fn snapshot_has_stable_identity_and_dense_node_domain() {
        let store = store();
        assert_eq!(store.identity().graph_name().value(), "knowledge");
        assert_eq!(store.identity().snapshot_id(), "snapshot-7");
        assert_eq!(store.identity().version(), 7);
        assert_eq!(store.node_domain().node_count(), 4);
        assert_eq!(
            store.node_domain().to_mapped(OriginalNodeId::new(300)),
            Some(MappedNodeId::new(2))
        );
        assert_eq!(
            store.node_domain().to_original(MappedNodeId::new(3)),
            Some(OriginalNodeId::new(400))
        );
    }

    #[test]
    fn typed_partition_is_forward_and_inverse_csr() {
        let store = store();
        let partition = store.partition(&RelationshipType::of("CITES")).unwrap();

        assert_eq!(partition.forward().offsets(), &[0, 2, 2, 3, 3]);
        assert_eq!(
            partition.forward().neighbors(MappedNodeId::new(0)),
            Some(&[MappedNodeId::new(1), MappedNodeId::new(2)][..])
        );
        let inverse = partition.inverse().expect("CITES is inverse indexed");
        assert_eq!(
            inverse.neighbors(MappedNodeId::new(1)),
            Some(&[MappedNodeId::new(0), MappedNodeId::new(2)][..])
        );
        assert_eq!(
            inverse.relationship_indices(MappedNodeId::new(1)),
            Some(&[RelationshipIndex::new(0), RelationshipIndex::new(2)][..])
        );
    }

    #[test]
    fn graphframe_consumes_core_graph_store_through_read_contract() {
        let store = Arc::new(store());
        let frame = GraphFrame::from_store(store).expect("CoreGraphStore GraphFrame");
        assert_eq!(frame.node_count().unwrap(), 4);
        assert_eq!(frame.relationship_count().unwrap(), 3);
        assert_eq!(frame.graph().unwrap().type_id(), "core-dense");
    }

    #[test]
    fn native_core_graph_honors_reverse_and_undirected_views() {
        let store = store();
        let reverse = store
            .get_graph_view(
                &GraphViewSpec::new().with_orientation(crate::projection::Orientation::Reverse),
            )
            .unwrap();
        let reverse_targets = reverse
            .stream_relationships(MappedNodeId::new(1), 0.0)
            .map(|cursor| cursor.target_id())
            .collect::<Vec<_>>();
        assert_eq!(
            reverse_targets,
            vec![MappedNodeId::new(0), MappedNodeId::new(2)]
        );

        let undirected = store
            .get_graph_view(
                &GraphViewSpec::new().with_orientation(crate::projection::Orientation::Undirected),
            )
            .unwrap();
        assert_eq!(undirected.relationship_count(), 6);
        assert_eq!(undirected.degree(MappedNodeId::new(1)), 2);
        assert!(undirected.characteristics().is_undirected());
        assert!(undirected.characteristics().is_inverse_indexed());
    }

    #[test]
    fn native_core_graph_filters_typed_partitions() {
        let store = CoreGraphStore::try_from_inputs(
            GraphStoreConfig::default(),
            GraphSnapshotIdentity::new("knowledge", "typed-snapshot", 1),
            database_info(),
            GraphSchema::empty(),
            Capabilities::new(),
            [10_i64, 20, 30],
            [
                TypedRelationshipInput::new("CITES", 10_i64, 20_i64),
                TypedRelationshipInput::new("KNOWS", 20_i64, 30_i64),
            ],
            &HashSet::new(),
        )
        .unwrap();
        let graph = store
            .get_graph_view(
                &GraphViewSpec::new()
                    .with_relationship_types(HashSet::from([RelationshipType::of("KNOWS")])),
            )
            .unwrap();

        assert_eq!(graph.relationship_count(), 1);
        assert!(!graph.exists(MappedNodeId::new(0), MappedNodeId::new(1)));
        assert!(graph.exists(MappedNodeId::new(1), MappedNodeId::new(2)));
    }

    #[test]
    fn native_core_graph_reports_parallel_relationships() {
        let store = CoreGraphStore::try_from_inputs(
            GraphStoreConfig::default(),
            GraphSnapshotIdentity::new("knowledge", "parallel-snapshot", 1),
            database_info(),
            GraphSchema::empty(),
            Capabilities::new(),
            [10_i64, 20],
            [
                TypedRelationshipInput::new("KNOWS", 10_i64, 20_i64),
                TypedRelationshipInput::new("KNOWS", 10_i64, 20_i64),
            ],
            &HashSet::new(),
        )
        .unwrap();

        let graph = store.get_graph();
        assert!(graph.is_multi_graph());
        assert_eq!(graph.degree(MappedNodeId::ZERO), 2);
    }

    #[test]
    fn construction_rejects_unknown_relationship_endpoint() {
        let result = CoreGraphStore::try_from_inputs(
            GraphStoreConfig::default(),
            GraphSnapshotIdentity::new("knowledge", "bad-snapshot", 1),
            database_info(),
            GraphSchema::empty(),
            Capabilities::new(),
            [100_i64],
            [TypedRelationshipInput::new("CITES", 100_i64, 999_i64)],
            &HashSet::new(),
        );
        assert!(matches!(
            result,
            Err(CoreGraphStoreError::UnknownOriginalNode(id)) if id == OriginalNodeId::new(999)
        ));
    }
}
