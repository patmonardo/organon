//! NodeCentricContext - Base context for node-centric operations
//!
//! Provides the foundation for InitContext and ComputeContext with common
//! node-centric operations like setting node values, accessing neighbors, etc.

use crate::task::pregel::{NodeValue, PregelRuntimeConfig};
use crate::types::graph::Graph;
use crate::types::graph::MappedNodeId;
use crate::types::graph::OriginalNodeId;
use parking_lot::RwLock;
use std::sync::Arc;

/// Base context class providing node-centric access to the graph.
///
/// This serves as the foundation for `InitContext` and `ComputeContext`,
/// providing common operations that work on a per-node basis.
///
/// # Node-Centric Operations
///
/// - **Node ID Management**: Track which node is currently being processed
/// - **Node Values**: Set properties for the current node
/// - **Graph Queries**: Degree, neighbors, node existence checks
/// - **ID Translation**: Convert between internal and original node IDs
///
/// # Translation from Java/TS
///
/// Follows Java `NodeCentricContext` constructor:
///
/// Rust ownership model: contexts borrow Graph and NodeValue to allow concurrent access.
pub struct NodeCentricContext<C: PregelRuntimeConfig> {
    node_id: MappedNodeId,
    config: C,
    graph: Arc<dyn Graph>,
    node_value: Arc<RwLock<NodeValue>>,
}

impl<C: PregelRuntimeConfig> NodeCentricContext<C> {
    /// Create a new node-centric context.
    ///
    /// # Arguments
    ///
    /// * `graph` - The graph topology to query
    /// * `config` - The Pregel configuration
    /// * `node_value` - The node property storage (wrapped in RwLock for interior mutability)
    ///
    /// # Translation from Java
    ///
    /// Maps directly to Java constructor:
    pub fn new(graph: Arc<dyn Graph>, config: C, node_value: Arc<RwLock<NodeValue>>) -> Self {
        Self {
            node_id: MappedNodeId::ZERO,
            config,
            graph,
            node_value,
        }
    }

    /// Set the node ID for this context.
    ///
    /// Called by the framework before each init() or compute() invocation
    /// to indicate which node is being processed.
    ///
    /// # Java equivalent
    ///
    pub fn set_node_id(&mut self, node_id: u64) {
        self.node_id = MappedNodeId::new(node_id);
    }

    /// Get the node ID currently being processed.
    ///
    /// # Java equivalent
    ///
    pub fn node_id(&self) -> u64 {
        self.node_id.get()
    }

    /// Get the configuration.
    ///
    /// # Java equivalent
    ///
    pub fn config(&self) -> &C {
        &self.config
    }

    /// Get a clone of the underlying graph Arc for advanced queries.
    pub fn graph_arc(&self) -> Arc<dyn Graph> {
        self.graph.clone()
    }

    /// Check if the graph is a multi-graph (allows multiple edges between nodes).
    ///
    /// # Java equivalent
    ///
    pub fn is_multi_graph(&self) -> bool {
        self.graph.is_multi_graph()
    }

    /// Get the number of nodes in the graph.
    ///
    /// # Java equivalent
    ///
    pub fn node_count(&self) -> u64 {
        u64::try_from(self.graph.node_count()).expect("graph node count must fit Pregel ID space")
    }

    /// Get the number of relationships in the graph.
    ///
    /// # Java equivalent
    ///
    pub fn relationship_count(&self) -> u64 {
        u64::try_from(self.graph.relationship_count())
            .expect("relationship count must fit Pregel count space")
    }

    /// Set a double node value for the given property key.
    ///
    /// # Parameters
    ///
    /// - `key`: The property key from the PregelSchema
    /// - `value`: The double value to set
    ///
    /// # Java equivalent
    ///
    pub fn set_node_value(&mut self, key: &str, value: f64) {
        self.node_value.write().set(key, self.node_index(), value);
    }

    /// Set a long node value for the given property key.
    ///
    /// # Java equivalent
    ///
    pub fn set_node_value_long(&mut self, key: &str, value: i64) {
        self.node_value
            .write()
            .set_long(key, self.node_index(), value);
    }

    /// Set a long array node value for the given property key.
    ///
    /// # Java equivalent
    ///
    pub fn set_node_value_long_array(&mut self, key: &str, value: Vec<i64>) {
        self.node_value
            .write()
            .set_long_array(key, self.node_index(), value);
    }

    /// Set a double array node value for the given property key.
    ///
    /// # Java equivalent
    ///
    pub fn set_node_value_double_array(&mut self, key: &str, value: Vec<f64>) {
        self.node_value
            .write()
            .set_double_array(key, self.node_index(), value);
    }

    /// Read a double node value for the given property key.
    pub(crate) fn double_node_value(&self, key: &str) -> f64 {
        self.node_value.read().double_value(key, self.node_index())
    }

    /// Read a long node value for the given property key.
    pub(crate) fn long_node_value(&self, key: &str) -> i64 {
        self.node_value.read().long_value(key, self.node_index())
    }

    /// Returns the degree (number of relationships) of the currently processed node.
    ///
    /// # Java equivalent
    ///
    pub fn degree(&self) -> usize {
        self.graph.degree(self.node_id)
    }

    /// Returns the incoming degree (number of incoming relationships) of the currently processed node.
    ///
    /// # Java equivalent
    ///
    pub fn incoming_degree(&self) -> usize {
        // For now, return the same as degree
        // Proper implementation would use topology.incoming(node_id).len()
        // but that requires access to the topology which isn't exposed through Graph trait
        self.graph.degree_inverse(self.node_id).unwrap_or(0)
    }

    /// Returns the corresponding node id in the original graph for the current node id.
    ///
    /// # Java equivalent
    ///
    pub fn to_original_id(&self) -> i64 {
        self.graph
            .to_original_node_id(self.node_id)
            .expect("node should exist in graph")
            .get()
    }

    /// Returns the corresponding node id in the original graph for the given internal node id.
    ///
    /// # Java equivalent
    ///
    pub fn to_original_id_of(&self, internal_node_id: u64) -> i64 {
        self.graph
            .to_original_node_id(MappedNodeId::new(internal_node_id))
            .expect("node should exist in graph")
            .get()
    }

    /// Returns the corresponding node id in the internal graph for the given original node id.
    ///
    /// # Java equivalent
    ///
    pub fn to_internal_id(&self, original_node_id: i64) -> u64 {
        self.graph
            .to_mapped_node_id(OriginalNodeId::new(original_node_id))
            .expect("node should exist in graph")
            .get()
    }

    /// Calls the consumer for each neighbor of the currently processed node (outgoing edges).
    ///
    /// # Java equivalent
    ///
    pub fn for_each_neighbor<F>(&self, mut consumer: F)
    where
        F: FnMut(u64),
    {
        let stream = self.graph.stream_relationships(self.node_id, 0.0);
        for cursor in stream {
            consumer(cursor.target_id().get());
        }
    }

    /// Calls the consumer for each incoming neighbor of the currently processed node (reverse edges).
    ///
    /// This is used by bidirectional algorithms like HITS that need to iterate over
    /// incoming edges.
    ///
    /// # Java equivalent
    ///
    pub fn for_each_incoming_neighbor<F>(&self, mut consumer: F)
    where
        F: FnMut(u64),
    {
        let stream = self.graph.stream_inverse_relationships(self.node_id, 0.0);
        for cursor in stream {
            consumer(cursor.source_id().get());
        }
    }

    /// Calls the consumer for each neighbor of the given node.
    ///
    /// # Java equivalent
    ///
    pub fn for_each_neighbor_of<F>(&self, node_id: u64, mut consumer: F)
    where
        F: FnMut(u64),
    {
        let stream = self
            .graph
            .stream_relationships(MappedNodeId::new(node_id), 0.0);
        for cursor in stream {
            consumer(cursor.target_id().get());
        }
    }

    fn node_index(&self) -> usize {
        self.node_id
            .to_usize()
            .expect("Pregel mapped node must fit node-value storage index space")
    }
}

// Tests will be added in integration tests once context wiring is complete
