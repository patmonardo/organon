use crate::projection::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::graph::Graph;
use crate::types::graph::MappedNodeId;

/// Abstraaction for accessing node neighbors (vectors).
pub trait VectorComputer: Send + Sync {
    /// Get the sorted neighbor IDs for a node.
    fn vector(&self, node_id: MappedNodeId) -> Vec<usize>;

    /// Get the weights corresponding to the neighbor IDs.
    /// Returns empty vector if unweighted.
    fn weights(&self, node_id: MappedNodeId) -> Vec<f64>;
}

pub struct UnweightedVectorComputer<'a> {
    graph: &'a dyn Graph,
}

impl UnweightedVectorComputer<'_> {
    pub fn new<'a>(
        graph: &'a dyn Graph,
        _relationship_type: Option<RelationshipType>,
        _orientation: Orientation,
    ) -> UnweightedVectorComputer<'a> {
        UnweightedVectorComputer { graph }
    }
}

impl<'a> VectorComputer for UnweightedVectorComputer<'a> {
    fn vector(&self, node_id: MappedNodeId) -> Vec<usize> {
        self.graph
            .stream_relationships(node_id, self.graph.default_property_value())
            .map(|cursor| {
                cursor
                    .target_id()
                    .to_usize()
                    .expect("mapped target ID must fit vector storage")
            })
            .collect()
    }

    fn weights(&self, node_id: MappedNodeId) -> Vec<f64> {
        // Optimization: use degree to pre-allocate?
        let degree = self.graph.degree(node_id);
        vec![1.0; degree]
    }
}

pub struct WeightedVectorComputer<'a> {
    graph: &'a dyn Graph,
}

impl<'a> WeightedVectorComputer<'a> {
    pub fn new(graph: &'a dyn Graph, _weight_property: String, _orientation: Orientation) -> Self {
        Self { graph }
    }
}

impl<'a> VectorComputer for WeightedVectorComputer<'a> {
    fn vector(&self, node_id: MappedNodeId) -> Vec<usize> {
        self.graph
            .stream_relationships(node_id, self.graph.default_property_value())
            .map(|cursor| {
                cursor
                    .target_id()
                    .to_usize()
                    .expect("mapped target ID must fit vector storage")
            })
            .collect()
    }

    fn weights(&self, node_id: MappedNodeId) -> Vec<f64> {
        let degree = self.graph.degree(node_id);
        vec![1.0; degree]
    }
}
