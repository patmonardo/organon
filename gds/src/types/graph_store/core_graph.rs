//! Native immutable graph view over CoreGraphStore CSR partitions.

use std::collections::HashSet;
use std::sync::Arc;

use crate::projection::{NodeLabel, Orientation, RelationshipType};
use crate::task::concurrency::Concurrency;
use crate::types::graph::id_map::{
    BatchNodeIterable, FilteredIdMap, IdMap, MappedNodeId, NodeConsumer, NodeIdBatch,
    NodeIdIterator, NodeIterator, NodeLabelConsumer, OriginalNodeId, PartialIdMap,
    RelationshipIndex, SimpleIdMap,
};
use crate::types::graph::{Degrees, Graph, GraphCharacteristics, GraphCharacteristicsBuilder};
use crate::types::properties::node::{NodePropertyContainer, NodePropertyValues};
use crate::types::properties::relationship::{
    DefaultRelationshipCursor, RelationshipCursorBox, RelationshipIterator, RelationshipPredicate,
    RelationshipProperties, RelationshipStream, WeightedRelationshipCursor,
    WeightedRelationshipCursorBox, WeightedRelationshipStream,
};
use crate::types::schema::{Direction, GraphSchema, RelationshipSchema, RelationshipSchemaEntry};

use super::{CsrTopology, RelationshipPartition};

#[derive(Debug, Clone)]
struct CoreGraphPartition {
    relationship_type: RelationshipType,
    outgoing: CsrTopology,
    incoming: Option<CsrTopology>,
}

/// Immutable algorithm-facing graph view backed directly by CSR arrays.
#[derive(Debug, Clone)]
pub struct CoreGraph {
    schema: Arc<GraphSchema>,
    id_map: Arc<SimpleIdMap>,
    partitions: Arc<[CoreGraphPartition]>,
    characteristics: GraphCharacteristics,
    relationship_count: usize,
    has_parallel_edges: bool,
}

impl CoreGraph {
    pub(crate) fn from_partitions(
        schema: Arc<GraphSchema>,
        id_map: Arc<SimpleIdMap>,
        partitions: Vec<Arc<RelationshipPartition>>,
        orientation: Orientation,
    ) -> Self {
        let mut views = Vec::with_capacity(partitions.len());
        for partition in partitions {
            let (outgoing, incoming) = match orientation {
                Orientation::Natural => (partition.forward().clone(), partition.inverse().cloned()),
                Orientation::Reverse => (
                    partition
                        .inverse()
                        .cloned()
                        .unwrap_or_else(|| partition.forward().transpose()),
                    Some(partition.forward().clone()),
                ),
                Orientation::Undirected => {
                    let topology = partition.forward().undirected();
                    (topology.clone(), Some(topology))
                }
            };
            views.push(CoreGraphPartition {
                relationship_type: partition.relationship_type().clone(),
                outgoing,
                incoming,
            });
        }

        views.sort_by(|left, right| left.relationship_type.cmp(&right.relationship_type));
        let relationship_count = views
            .iter()
            .map(|partition| partition.outgoing.relationship_count())
            .sum();
        let has_parallel_edges = views
            .iter()
            .any(|partition| partition.outgoing.has_parallel_edges());
        let all_inverse_indexed =
            !views.is_empty() && views.iter().all(|partition| partition.incoming.is_some());
        let mut characteristics = GraphCharacteristicsBuilder::new();
        characteristics = match orientation {
            Orientation::Undirected => characteristics.undirected(),
            Orientation::Natural | Orientation::Reverse => characteristics.directed(),
        };
        if all_inverse_indexed {
            characteristics = characteristics.inverse_indexed();
        }

        Self {
            schema: Arc::new(oriented_schema(&schema, orientation)),
            id_map,
            partitions: views.into(),
            characteristics: characteristics.build(),
            relationship_count,
            has_parallel_edges,
        }
    }

    pub fn relationship_types(&self) -> HashSet<RelationshipType> {
        self.partitions
            .iter()
            .map(|partition| partition.relationship_type.clone())
            .collect()
    }

    fn stream<'a>(
        &'a self,
        node: MappedNodeId,
        incoming: bool,
        fallback: f64,
    ) -> CoreRelationshipStream<'a> {
        CoreRelationshipStream::new(self, node, incoming, fallback)
    }
}

struct CoreRelationshipStream<'a> {
    graph: &'a CoreGraph,
    node: MappedNodeId,
    incoming: bool,
    fallback: f64,
    partition_index: usize,
    row_offset: usize,
}

impl<'a> CoreRelationshipStream<'a> {
    fn new(graph: &'a CoreGraph, node: MappedNodeId, incoming: bool, fallback: f64) -> Self {
        Self {
            graph,
            node,
            incoming,
            fallback,
            partition_index: 0,
            row_offset: 0,
        }
    }
}

impl Iterator for CoreRelationshipStream<'_> {
    type Item = (RelationshipIndex, MappedNodeId, MappedNodeId, f64);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let partition = self.graph.partitions.get(self.partition_index)?;
            let topology = if self.incoming {
                match partition.incoming.as_ref() {
                    Some(topology) => topology,
                    None => {
                        self.partition_index += 1;
                        self.row_offset = 0;
                        continue;
                    }
                }
            } else {
                &partition.outgoing
            };
            let neighbors = topology.neighbors(self.node).unwrap_or_default();
            let indices = topology.relationship_indices(self.node).unwrap_or_default();
            if let (Some(&neighbor), Some(&relationship_index)) =
                (neighbors.get(self.row_offset), indices.get(self.row_offset))
            {
                self.row_offset += 1;
                let (source, target) = if self.incoming {
                    (neighbor, self.node)
                } else {
                    (self.node, neighbor)
                };
                return Some((relationship_index, source, target, self.fallback));
            }
            self.partition_index += 1;
            self.row_offset = 0;
        }
    }
}

#[derive(Debug)]
struct CoreWeightedCursor {
    relationship_index: RelationshipIndex,
    source: MappedNodeId,
    target: MappedNodeId,
    weight: f64,
}

impl WeightedRelationshipCursor for CoreWeightedCursor {
    fn relationship_index(&self) -> RelationshipIndex {
        self.relationship_index
    }

    fn source_id(&self) -> MappedNodeId {
        self.source
    }

    fn target_id(&self) -> MappedNodeId {
        self.target
    }

    fn weight(&self) -> f64 {
        self.weight
    }
}

impl Graph for CoreGraph {
    fn schema(&self) -> &GraphSchema {
        &self.schema
    }

    fn characteristics(&self) -> GraphCharacteristics {
        self.characteristics
    }

    fn relationship_count(&self) -> usize {
        self.relationship_count
    }

    fn is_multi_graph(&self) -> bool {
        self.has_parallel_edges
    }

    fn has_relationship_property(&self) -> bool {
        false
    }

    fn concurrent_view(&self) -> Arc<dyn Graph> {
        Arc::new(self.clone())
    }

    fn as_node_filtered_graph(&self) -> Option<Arc<dyn FilteredIdMap>> {
        None
    }
}

impl PartialIdMap for CoreGraph {
    fn to_mapped_node_id(&self, original_node_id: OriginalNodeId) -> Option<MappedNodeId> {
        self.id_map.to_mapped_node_id(original_node_id)
    }

    fn root_node_count(&self) -> Option<usize> {
        self.id_map.root_node_count()
    }
}

impl NodeIterator for CoreGraph {
    fn for_each_node(&self, consumer: &mut dyn NodeConsumer) {
        self.id_map.for_each_node(consumer)
    }

    fn iter(&self) -> NodeIdIterator<'_> {
        self.id_map.iter()
    }

    fn iter_with_labels<'a>(&'a self, labels: &'a HashSet<NodeLabel>) -> NodeIdIterator<'a> {
        self.id_map.iter_with_labels(labels)
    }
}

impl BatchNodeIterable for CoreGraph {
    fn batch_iterables(&self, batch_size: usize) -> Vec<NodeIdBatch> {
        self.id_map.batch_iterables(batch_size)
    }
}

impl IdMap for CoreGraph {
    fn type_id(&self) -> &str {
        "core-dense"
    }

    fn safe_to_mapped_node_id(&self, original_node_id: OriginalNodeId) -> Option<MappedNodeId> {
        self.id_map.safe_to_mapped_node_id(original_node_id)
    }

    fn to_original_node_id(&self, mapped_node_id: MappedNodeId) -> Option<OriginalNodeId> {
        self.id_map.to_original_node_id(mapped_node_id)
    }

    fn to_root_node_id(&self, mapped_node_id: MappedNodeId) -> Option<MappedNodeId> {
        self.id_map.to_root_node_id(mapped_node_id)
    }

    fn node_count(&self) -> usize {
        self.id_map.node_count()
    }

    fn node_count_for_label(&self, node_label: &NodeLabel) -> usize {
        self.id_map.node_count_for_label(node_label)
    }

    fn highest_original_id(&self) -> Option<OriginalNodeId> {
        self.id_map.highest_original_id()
    }

    fn node_labels(&self, mapped_node_id: MappedNodeId) -> HashSet<NodeLabel> {
        self.id_map.node_labels(mapped_node_id)
    }

    fn for_each_node_label(
        &self,
        mapped_node_id: MappedNodeId,
        consumer: &mut dyn NodeLabelConsumer,
    ) {
        self.id_map.for_each_node_label(mapped_node_id, consumer)
    }

    fn available_node_labels(&self) -> HashSet<NodeLabel> {
        self.id_map.available_node_labels()
    }

    fn has_label(&self, mapped_node_id: MappedNodeId, label: &NodeLabel) -> bool {
        self.id_map.has_label(mapped_node_id, label)
    }

    fn add_node_label(&mut self, node_label: NodeLabel) {
        Arc::make_mut(&mut self.id_map).add_node_label(node_label)
    }

    fn add_node_id_to_label(&mut self, node_id: MappedNodeId, node_label: NodeLabel) {
        Arc::make_mut(&mut self.id_map).add_node_id_to_label(node_id, node_label)
    }

    fn root_id_map(&self) -> &dyn IdMap {
        self.id_map.root_id_map()
    }

    fn with_filtered_labels(
        &self,
        node_labels: &HashSet<NodeLabel>,
        concurrency: Concurrency,
    ) -> Option<Box<dyn FilteredIdMap>> {
        self.id_map.with_filtered_labels(node_labels, concurrency)
    }
}

impl Degrees for CoreGraph {
    fn degree(&self, node_id: MappedNodeId) -> usize {
        self.partitions
            .iter()
            .map(|partition| partition.outgoing.neighbors(node_id).map_or(0, <[_]>::len))
            .sum()
    }

    fn degree_inverse(&self, node_id: MappedNodeId) -> Option<usize> {
        if self
            .partitions
            .iter()
            .all(|partition| partition.incoming.is_none())
        {
            return None;
        }
        Some(
            self.partitions
                .iter()
                .filter_map(|partition| partition.incoming.as_ref())
                .map(|topology| topology.neighbors(node_id).map_or(0, <[_]>::len))
                .sum(),
        )
    }

    fn degree_without_parallel_relationships(&self, node_id: MappedNodeId) -> usize {
        self.stream(node_id, false, 0.0)
            .map(|(_, _, target, _)| target)
            .collect::<HashSet<_>>()
            .len()
    }
}

impl RelationshipPredicate for CoreGraph {
    fn exists(&self, source_id: MappedNodeId, target_id: MappedNodeId) -> bool {
        self.stream(source_id, false, 0.0)
            .any(|(_, _, target, _)| target == target_id)
    }
}

impl RelationshipIterator for CoreGraph {
    fn stream_relationships<'a>(
        &'a self,
        node_id: MappedNodeId,
        fallback_value: f64,
    ) -> RelationshipStream<'a> {
        Box::new(self.stream(node_id, false, fallback_value).map(
            |(index, source, target, property)| {
                Box::new(DefaultRelationshipCursor::new(
                    index, source, target, property,
                )) as RelationshipCursorBox
            },
        ))
    }

    fn stream_inverse_relationships<'a>(
        &'a self,
        node_id: MappedNodeId,
        fallback_value: f64,
    ) -> RelationshipStream<'a> {
        Box::new(self.stream(node_id, true, fallback_value).map(
            |(index, source, target, property)| {
                Box::new(DefaultRelationshipCursor::new(
                    index, source, target, property,
                )) as RelationshipCursorBox
            },
        ))
    }

    fn concurrent_copy(&self) -> Box<dyn RelationshipIterator> {
        Box::new(self.clone())
    }

    fn stream_relationships_weighted<'a>(
        &'a self,
        node_id: MappedNodeId,
        fallback_value: f64,
    ) -> WeightedRelationshipStream<'a> {
        Box::new(self.stream(node_id, false, fallback_value).map(
            |(relationship_index, source, target, weight)| {
                Box::new(CoreWeightedCursor {
                    relationship_index,
                    source,
                    target,
                    weight,
                }) as WeightedRelationshipCursorBox
            },
        ))
    }

    fn stream_inverse_relationships_weighted<'a>(
        &'a self,
        node_id: MappedNodeId,
        fallback_value: f64,
    ) -> WeightedRelationshipStream<'a> {
        Box::new(self.stream(node_id, true, fallback_value).map(
            |(relationship_index, source, target, weight)| {
                Box::new(CoreWeightedCursor {
                    relationship_index,
                    source,
                    target,
                    weight,
                }) as WeightedRelationshipCursorBox
            },
        ))
    }
}

impl RelationshipProperties for CoreGraph {
    fn default_property_value(&self) -> f64 {
        0.0
    }

    fn relationship_property(
        &self,
        _source_id: MappedNodeId,
        _target_id: MappedNodeId,
        fallback_value: f64,
    ) -> f64 {
        fallback_value
    }
}

impl NodePropertyContainer for CoreGraph {
    fn node_properties(&self, _property_key: &str) -> Option<Arc<dyn NodePropertyValues>> {
        None
    }

    fn available_node_properties(&self) -> HashSet<String> {
        HashSet::new()
    }
}

fn oriented_schema(schema: &GraphSchema, orientation: Orientation) -> GraphSchema {
    if orientation != Orientation::Undirected {
        return schema.clone();
    }
    let entries = schema
        .relationship_schema()
        .entries()
        .into_iter()
        .map(|entry| {
            (
                entry.identifier().clone(),
                RelationshipSchemaEntry::new(
                    entry.identifier().clone(),
                    Direction::Undirected,
                    entry.properties().clone(),
                ),
            )
        })
        .collect();
    GraphSchema::new(
        schema.node_schema().clone(),
        RelationshipSchema::new(entries),
        schema.graph_properties().clone(),
    )
}
