use super::{AdjacencyList, Graph, GraphCharacteristics, RelationshipTopology};
use crate::config::GraphStoreConfig;
use crate::projection::RelationshipType;
use crate::task::concurrency::Concurrency;
use crate::types::graph::characteristics::GraphCharacteristicsBuilder;
use crate::types::graph::degrees::Degrees;
use crate::types::graph::id_map::NodeLabelConsumer;
use crate::types::graph::id_map::{
    BatchNodeIterable, FilteredIdMap, IdMap, MappedNodeId, NodeConsumer, NodeIdBatch,
    NodeIdIterator, NodeIterator, OriginalNodeId, PartialIdMap, RelationshipIndex, SimpleIdMap,
};
use crate::types::graph::Neighbor;
use crate::types::graph::NeighborCursor;
use crate::types::graph::TraversalDirection;
use crate::types::properties::node::{NodePropertyContainer, NodePropertyValues};
use crate::types::properties::relationship::{
    relationship_properties::RelationshipProperties,
    relationship_property_values::RelationshipPropertyValues, DefaultRelationshipCursor,
    DefaultRelationshipPropertyStore, RelationshipCursorBox, RelationshipIterator,
    RelationshipPredicate, RelationshipStream, WeightedRelationshipCursor,
    WeightedRelationshipCursorBox, WeightedRelationshipStream,
};
use crate::types::properties::PropertyStore;
use crate::types::schema::{GraphSchema, NodeLabel};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// Default in-memory graph implementation backed by [`SimpleIdMap`] and [`RelationshipTopology`].
#[derive(Debug, Clone)]
pub struct DefaultGraph {
    config: Arc<GraphStoreConfig>,
    schema: Arc<GraphSchema>,
    id_map: Arc<SimpleIdMap>,
    characteristics: GraphCharacteristics,
    topologies: HashMap<RelationshipType, Arc<RelationshipTopology>>,
    ordered_types: Vec<RelationshipType>,
    inverse_indexed_types: HashSet<RelationshipType>,
    relationship_count: usize,
    has_parallel_edges: bool,
    node_properties: HashMap<String, Arc<dyn NodePropertyValues>>,
    relationship_properties: HashMap<RelationshipType, DefaultRelationshipPropertyStore>,
    selected_relationship_properties: HashMap<RelationshipType, SelectedRelationshipProperty>,
    relationship_property_selectors: HashMap<RelationshipType, String>,
    has_relationship_properties: bool,
}

// === Phase 2C: WeightedRelationshipCursor Implementation ===

#[derive(Debug)]
struct WeightedCursor {
    relationship_index: RelationshipIndex,
    source: MappedNodeId,
    target: MappedNodeId,
    weight: f64,
}

impl WeightedRelationshipCursor for WeightedCursor {
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

struct DefaultNeighborStream<'a> {
    graph: &'a DefaultGraph,
    node_id: MappedNodeId,
    direction: TraversalDirection,
    fallback_value: f64,
    next_type_index: usize,
    active_type_index: usize,
    cursor: Option<Box<dyn NeighborCursor>>,
}

impl<'a> DefaultNeighborStream<'a> {
    fn new(
        graph: &'a DefaultGraph,
        node_id: MappedNodeId,
        direction: TraversalDirection,
        fallback_value: f64,
    ) -> Self {
        Self {
            graph,
            node_id,
            direction,
            fallback_value,
            next_type_index: 0,
            active_type_index: 0,
            cursor: None,
        }
    }
}

impl Iterator for DefaultNeighborStream<'_> {
    type Item = (Neighbor, f64);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(cursor) = self.cursor.as_mut() {
                if let Some(neighbor) = cursor.next_neighbor() {
                    let relationship_type = &self.graph.ordered_types[self.active_type_index];
                    let property = self.graph.relationship_property_value_for(
                        relationship_type,
                        neighbor.relationship_index,
                        self.fallback_value,
                    );
                    return Some((neighbor, property));
                }
                self.cursor = None;
            }

            let relationship_type = self.graph.ordered_types.get(self.next_type_index)?;
            self.active_type_index = self.next_type_index;
            self.next_type_index += 1;
            let Some(topology) = self.graph.topology_for(relationship_type) else {
                continue;
            };

            let mut cursor = Arc::clone(topology).new_cursor();
            if cursor.reset(self.node_id, self.direction).is_err() {
                continue;
            }
            self.cursor = Some(cursor);
        }
    }
}

impl DefaultGraph {
    /// Creates a new graph instance from the provided components.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        config: Arc<GraphStoreConfig>,
        schema: Arc<GraphSchema>,
        id_map: Arc<SimpleIdMap>,
        characteristics: GraphCharacteristics,
        topologies: HashMap<RelationshipType, Arc<RelationshipTopology>>,
        ordered_types: Vec<RelationshipType>,
        inverse_indexed_types: HashSet<RelationshipType>,
        relationship_count: usize,
        has_parallel_edges: bool,
        node_properties: HashMap<String, Arc<dyn NodePropertyValues>>,
        relationship_properties: HashMap<RelationshipType, DefaultRelationshipPropertyStore>,
        relationship_property_selectors: HashMap<RelationshipType, String>,
    ) -> Self {
        let (selected_relationship_properties, effective_selectors) =
            build_selected_relationship_properties(
                &ordered_types,
                &relationship_properties,
                &relationship_property_selectors,
            );
        let has_relationship_properties = !selected_relationship_properties.is_empty();

        Self {
            config,
            schema,
            id_map,
            characteristics,
            topologies,
            ordered_types,
            inverse_indexed_types,
            relationship_count,
            has_parallel_edges,
            node_properties,
            relationship_properties,
            selected_relationship_properties,
            relationship_property_selectors: effective_selectors,
            has_relationship_properties,
        }
    }

    /// Returns the topology associated with the provided relationship type, if present.
    fn topology_for(
        &self,
        relationship_type: &RelationshipType,
    ) -> Option<&Arc<RelationshipTopology>> {
        self.topologies.get(relationship_type)
    }

    /// Returns the set of relationship types present in this graph.
    pub fn relationship_types(&self) -> HashSet<RelationshipType> {
        self.ordered_types.iter().cloned().collect()
    }

    fn filtered_characteristics(&self, has_inverse_indices: bool) -> GraphCharacteristics {
        let mut builder = GraphCharacteristicsBuilder::new();
        if self.characteristics.is_directed() {
            builder = builder.directed();
        }
        if self.characteristics.is_undirected() {
            builder = builder.undirected();
        }
        if has_inverse_indices {
            builder = builder.inverse_indexed();
        }
        builder.build()
    }

    fn selected_property(
        &self,
        relationship_type: &RelationshipType,
    ) -> Option<&SelectedRelationshipProperty> {
        self.selected_relationship_properties.get(relationship_type)
    }

    fn relationship_property_value_for(
        &self,
        relationship_type: &RelationshipType,
        relationship_index: RelationshipIndex,
        fallback_value: f64,
    ) -> f64 {
        let selected = match self.selected_property(relationship_type) {
            Some(selected) => selected,
            None => return fallback_value,
        };

        selected.value_at_or(relationship_index, fallback_value)
    }

    pub(crate) fn filtered_by_relationship_types(
        &self,
        relationship_types: &HashSet<RelationshipType>,
    ) -> Arc<dyn Graph> {
        if relationship_types.is_empty() {
            return Arc::new(self.clone());
        }

        let mut filtered_topologies: HashMap<RelationshipType, Arc<RelationshipTopology>> =
            HashMap::new();
        let mut ordered_types = Vec::new();
        let mut inverse_indexed_types = HashSet::new();
        let mut relationship_count = 0usize;
        let mut has_parallel_edges = false;

        for relationship_type in &self.ordered_types {
            if !relationship_types.contains(relationship_type) {
                continue;
            }

            if let Some(topology) = self.topology_for(relationship_type) {
                ordered_types.push(relationship_type.clone());
                filtered_topologies.insert(relationship_type.clone(), Arc::clone(topology));
                relationship_count += topology.relationship_count();
                if topology.is_inverse_indexed() {
                    inverse_indexed_types.insert(relationship_type.clone());
                }
                if topology.has_parallel_edges() {
                    has_parallel_edges = true;
                }
            }
        }

        let has_inverse_indices = !ordered_types.is_empty()
            && ordered_types
                .iter()
                .all(|rel_type| inverse_indexed_types.contains(rel_type));

        let filtered_characteristics = self.filtered_characteristics(has_inverse_indices);
        let schema_types = relationship_types
            .iter()
            .map(|rel_type| RelationshipType::of(rel_type.name()))
            .collect();
        let filtered_schema = Arc::new(self.schema.filter_relationship_types(&schema_types));

        let filtered_relationship_properties = ordered_types
            .iter()
            .filter_map(|rel_type| {
                self.relationship_properties
                    .get(rel_type)
                    .map(|store| (rel_type.clone(), store.clone()))
            })
            .collect();
        let filtered_selectors = ordered_types
            .iter()
            .filter_map(|rel_type| {
                self.relationship_property_selectors
                    .get(rel_type)
                    .map(|key| (rel_type.clone(), key.clone()))
            })
            .collect();

        Arc::new(DefaultGraph::new(
            Arc::clone(&self.config),
            filtered_schema,
            Arc::clone(&self.id_map),
            filtered_characteristics,
            filtered_topologies,
            ordered_types,
            inverse_indexed_types,
            relationship_count,
            has_parallel_edges,
            self.node_properties.clone(),
            filtered_relationship_properties,
            filtered_selectors,
        ))
    }
}

#[derive(Debug, Clone)]
struct SelectedRelationshipProperty {
    values: Arc<dyn RelationshipPropertyValues>,
    fallback: f64,
}

impl SelectedRelationshipProperty {
    fn new(values: Arc<dyn RelationshipPropertyValues>, fallback: f64) -> Self {
        Self { values, fallback }
    }

    fn value_at_or(&self, index: RelationshipIndex, fallback: f64) -> f64 {
        self.values.double_value(index).unwrap_or(fallback)
    }
}

fn build_selected_relationship_properties(
    ordered_types: &[RelationshipType],
    stores: &HashMap<RelationshipType, DefaultRelationshipPropertyStore>,
    selectors: &HashMap<RelationshipType, String>,
) -> (
    HashMap<RelationshipType, SelectedRelationshipProperty>,
    HashMap<RelationshipType, String>,
) {
    let mut selected = HashMap::new();
    let mut effective = HashMap::new();

    for rel_type in ordered_types {
        let store = match stores.get(rel_type) {
            Some(store) if !store.is_empty() => store,
            _ => continue,
        };

        let chosen_key = selectors
            .get(rel_type)
            .cloned()
            .or_else(|| auto_select_property_key(store));

        if let Some(key) = chosen_key {
            if let Some(property) = store.get(&key) {
                let rel_values = property.values_arc();
                let selection = SelectedRelationshipProperty::new(
                    rel_values.clone(),
                    rel_values.default_value(),
                );
                selected.insert(rel_type.clone(), selection);
                effective.insert(rel_type.clone(), key);
            }
        }
    }

    (selected, effective)
}

fn auto_select_property_key(store: &DefaultRelationshipPropertyStore) -> Option<String> {
    if store.len() == 1 {
        store
            .columns()
            .next()
            .map(|property| property.key().to_string())
    } else {
        None
    }
}

impl Graph for DefaultGraph {
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
        self.has_relationship_properties
    }

    fn concurrent_view(&self) -> Arc<dyn Graph> {
        Arc::new(self.clone())
    }

    fn as_node_filtered_graph(&self) -> Option<Arc<dyn FilteredIdMap>> {
        None
    }

    fn nth_target(&self, source_id: MappedNodeId, offset: usize) -> Option<MappedNodeId> {
        let mut partition_offset = offset;

        for relationship_type in &self.ordered_types {
            let topology = self.topology_for(relationship_type)?;
            let degree =
                AdjacencyList::degree(topology.as_ref(), source_id, TraversalDirection::Outgoing)
                    .ok()?;
            if partition_offset >= degree {
                partition_offset -= degree;
                continue;
            }

            let mut cursor = Arc::clone(topology).new_cursor();
            cursor.reset(source_id, TraversalDirection::Outgoing).ok()?;
            return cursor
                .advance_by(partition_offset)
                .map(|neighbor| neighbor.target);
        }

        None
    }
}

impl PartialIdMap for DefaultGraph {
    fn to_mapped_node_id(&self, original_node_id: OriginalNodeId) -> Option<MappedNodeId> {
        self.id_map.to_mapped_node_id(original_node_id)
    }

    fn root_node_count(&self) -> Option<usize> {
        self.id_map.root_node_count()
    }
}

impl NodeIterator for DefaultGraph {
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

impl BatchNodeIterable for DefaultGraph {
    fn batch_iterables(&self, batch_size: usize) -> Vec<NodeIdBatch> {
        self.id_map.batch_iterables(batch_size)
    }
}

impl IdMap for DefaultGraph {
    fn type_id(&self) -> &str {
        self.id_map.type_id()
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
        Arc::make_mut(&mut self.id_map).add_node_label(node_label);
    }

    fn add_node_id_to_label(&mut self, node_id: MappedNodeId, node_label: NodeLabel) {
        Arc::make_mut(&mut self.id_map).add_node_id_to_label(node_id, node_label);
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

impl Degrees for DefaultGraph {
    fn degree(&self, node_id: MappedNodeId) -> usize {
        self.ordered_types
            .iter()
            .filter_map(|rel_type| self.topology_for(rel_type))
            .filter_map(|topology| {
                AdjacencyList::degree(topology.as_ref(), node_id, TraversalDirection::Outgoing).ok()
            })
            .sum()
    }

    fn degree_inverse(&self, node_id: MappedNodeId) -> Option<usize> {
        if self.inverse_indexed_types.is_empty() {
            return None;
        }

        let total: usize = self
            .ordered_types
            .iter()
            .filter(|rel_type| self.inverse_indexed_types.contains(*rel_type))
            .filter_map(|rel_type| self.topology_for(rel_type))
            .filter_map(|topology| {
                AdjacencyList::degree(topology.as_ref(), node_id, TraversalDirection::Incoming).ok()
            })
            .sum();
        Some(total)
    }

    fn degree_without_parallel_relationships(&self, node_id: MappedNodeId) -> usize {
        let mut unique = HashSet::new();
        for rel_type in &self.ordered_types {
            if let Some(topology) = self.topology_for(rel_type) {
                if let Some(neighbors) = topology.outgoing(node_id) {
                    unique.extend(neighbors.iter().copied());
                }
            }
        }
        unique.len()
    }
}

impl RelationshipPredicate for DefaultGraph {
    fn exists(&self, source_id: MappedNodeId, target_id: MappedNodeId) -> bool {
        self.ordered_types.iter().any(|rel_type| {
            let Some(topology) = self.topology_for(rel_type) else {
                return false;
            };
            let mut cursor = Arc::clone(topology).new_cursor();
            if cursor
                .reset(source_id, TraversalDirection::Outgoing)
                .is_err()
            {
                return false;
            }

            std::iter::from_fn(|| cursor.next_neighbor())
                .any(|neighbor| neighbor.target == target_id)
        })
    }
}

impl RelationshipIterator for DefaultGraph {
    fn stream_relationships<'a>(
        &'a self,
        node_id: MappedNodeId,
        fallback_value: f64,
    ) -> RelationshipStream<'a> {
        Box::new(
            DefaultNeighborStream::new(self, node_id, TraversalDirection::Outgoing, fallback_value)
                .map(|(neighbor, property)| {
                    Box::new(DefaultRelationshipCursor::new(
                        neighbor.relationship_index,
                        neighbor.source,
                        neighbor.target,
                        property,
                    )) as RelationshipCursorBox
                }),
        )
    }

    fn stream_inverse_relationships<'a>(
        &'a self,
        node_id: MappedNodeId,
        fallback_value: f64,
    ) -> RelationshipStream<'a> {
        Box::new(
            DefaultNeighborStream::new(self, node_id, TraversalDirection::Incoming, fallback_value)
                .map(|(neighbor, property)| {
                    Box::new(DefaultRelationshipCursor::new(
                        neighbor.relationship_index,
                        neighbor.source,
                        neighbor.target,
                        property,
                    )) as RelationshipCursorBox
                }),
        )
    }

    fn concurrent_copy(&self) -> Box<dyn RelationshipIterator> {
        Box::new(self.clone())
    }

    // === Phase 2C: Weighted Stream Implementations ===

    fn stream_relationships_weighted<'a>(
        &'a self,
        node_id: MappedNodeId,
        fallback_value: f64,
    ) -> WeightedRelationshipStream<'a> {
        Box::new(
            DefaultNeighborStream::new(self, node_id, TraversalDirection::Outgoing, fallback_value)
                .map(|(neighbor, weight)| {
                    Box::new(WeightedCursor {
                        relationship_index: neighbor.relationship_index,
                        source: neighbor.source,
                        target: neighbor.target,
                        weight,
                    }) as WeightedRelationshipCursorBox
                }),
        )
    }

    fn stream_inverse_relationships_weighted<'a>(
        &'a self,
        node_id: MappedNodeId,
        fallback_value: f64,
    ) -> WeightedRelationshipStream<'a> {
        Box::new(
            DefaultNeighborStream::new(self, node_id, TraversalDirection::Incoming, fallback_value)
                .map(|(neighbor, weight)| {
                    Box::new(WeightedCursor {
                        relationship_index: neighbor.relationship_index,
                        source: neighbor.source,
                        target: neighbor.target,
                        weight,
                    }) as WeightedRelationshipCursorBox
                }),
        )
    }
}

impl RelationshipProperties for DefaultGraph {
    fn default_property_value(&self) -> f64 {
        self.selected_relationship_properties
            .values()
            .next()
            .map(|selection| selection.fallback)
            .unwrap_or(0.0)
    }

    fn relationship_property(
        &self,
        source_id: MappedNodeId,
        target_id: MappedNodeId,
        fallback_value: f64,
    ) -> f64 {
        if self.selected_relationship_properties.is_empty() {
            return fallback_value;
        }

        DefaultNeighborStream::new(
            self,
            source_id,
            TraversalDirection::Outgoing,
            fallback_value,
        )
        .find(|(neighbor, _)| neighbor.target == target_id)
        .map_or(fallback_value, |(_, property)| property)
    }
}

impl NodePropertyContainer for DefaultGraph {
    fn node_properties(&self, property_key: &str) -> Option<Arc<dyn NodePropertyValues>> {
        self.node_properties.get(property_key).cloned()
    }

    fn available_node_properties(&self) -> HashSet<String> {
        self.node_properties.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::graph::degrees::Degrees;
    use crate::types::graph::Graph;

    fn build_graph() -> DefaultGraph {
        let schema = Arc::new(GraphSchema::empty());
        let id_map = Arc::new(SimpleIdMap::from_original_ids([0, 1, 2]));

        let topology = RelationshipTopology::new(
            vec![
                vec![MappedNodeId::new(1), MappedNodeId::new(2)],
                vec![MappedNodeId::new(2)],
                vec![],
            ],
            Some(vec![
                vec![],
                vec![MappedNodeId::ZERO],
                vec![MappedNodeId::ZERO, MappedNodeId::new(1)],
            ]),
        );
        let relationship_count = topology.relationship_count();
        let has_parallel_edges = topology.has_parallel_edges();

        let mut topologies = HashMap::new();
        let rel_type = RelationshipType::of("KNOWS");
        topologies.insert(rel_type.clone(), Arc::new(topology));

        DefaultGraph::new(
            Arc::new(GraphStoreConfig::default()),
            schema,
            id_map,
            GraphCharacteristicsBuilder::new().directed().build(),
            topologies,
            vec![rel_type.clone()],
            HashSet::from([rel_type]),
            relationship_count,
            has_parallel_edges,
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
        )
    }

    #[test]
    fn computes_degrees_and_relationship_counts() {
        let graph = build_graph();
        assert_eq!(graph.relationship_count(), 3);
        assert_eq!(graph.degree(MappedNodeId::ZERO), 2);
        assert_eq!(graph.degree(MappedNodeId::new(1)), 1);
        assert_eq!(
            graph.degree_without_parallel_relationships(MappedNodeId::ZERO),
            2
        );
        assert!(graph.exists(MappedNodeId::ZERO, MappedNodeId::new(2)));
        assert!(!graph.exists(MappedNodeId::new(2), MappedNodeId::ZERO));
        assert_eq!(
            graph.nth_target(MappedNodeId::ZERO, 1),
            Some(MappedNodeId::new(2))
        );

        let outgoing = graph
            .stream_relationships(MappedNodeId::ZERO, 0.0)
            .map(|cursor| (cursor.relationship_index(), cursor.target_id()))
            .collect::<Vec<_>>();
        assert_eq!(
            outgoing,
            vec![
                (RelationshipIndex::ZERO, MappedNodeId::new(1)),
                (RelationshipIndex::new(1), MappedNodeId::new(2)),
            ]
        );

        let incoming = graph
            .stream_inverse_relationships(MappedNodeId::new(2), 0.0)
            .map(|cursor| (cursor.relationship_index(), cursor.source_id()))
            .collect::<Vec<_>>();
        assert_eq!(
            incoming,
            vec![
                (RelationshipIndex::new(1), MappedNodeId::ZERO),
                (RelationshipIndex::new(2), MappedNodeId::new(1)),
            ]
        );
    }

    #[test]
    fn filters_relationship_types() {
        let graph = build_graph();
        let rel_type = RelationshipType::of("KNOWS");

        let mut filter = HashSet::new();
        filter.insert(rel_type.clone());
        let filtered = graph.filtered_by_relationship_types(&filter);
        assert_eq!(filtered.relationship_count(), 3);

        let empty_filter = HashSet::new();
        let no_filter = graph.filtered_by_relationship_types(&empty_filter);
        assert_eq!(no_filter.relationship_count(), graph.relationship_count());
    }
}
