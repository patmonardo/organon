//! ToUndirected specification and configuration.
//!
//! Translation source: `org.neo4j.gds.undirected.ToUndirectedConfig`.

use crate::core::Aggregation;
use crate::types::prelude::DefaultGraphStore;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Aggregation overrides for creating an undirected projection.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToUndirectedAggregations {
    /// Default aggregation to apply to all relationship properties unless overridden.
    pub global: Option<Aggregation>,
    /// Per-property aggregation overrides keyed by relationship property name.
    pub properties: HashMap<String, Aggregation>,
}

impl ToUndirectedAggregations {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_global(mut self, aggregation: Aggregation) -> Self {
        self.global = Some(aggregation);
        self
    }

    pub fn with_property(
        mut self,
        property_key: impl Into<String>,
        aggregation: Aggregation,
    ) -> Self {
        self.properties.insert(property_key.into(), aggregation);
        self
    }

    pub fn global_aggregation(&self) -> Option<Aggregation> {
        self.global
    }

    pub fn local_aggregation(&self, property_key: &str) -> Option<Aggregation> {
        self.properties.get(property_key).copied().or(self.global)
    }
}

/// Configuration for creating an undirected projection from a single relationship type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToUndirectedConfig {
    /// The relationship type to symmetrize.
    pub relationship_type: String,
    /// Graph name to assign to the mutated graph store.
    pub mutate_graph_name: String,
    /// Relationship type name to use for the undirected output.
    pub mutate_relationship_type: String,
    /// Concurrency hint (currently unused by the sequential runtime).
    pub concurrency: usize,
    /// Optional global and per-property aggregation overrides.
    pub aggregation: Option<ToUndirectedAggregations>,
}

impl Default for ToUndirectedConfig {
    fn default() -> Self {
        Self {
            relationship_type: "".to_string(),
            mutate_graph_name: "to_undirected".to_string(),
            mutate_relationship_type: "undirected".to_string(),
            concurrency: 4,
            aggregation: None,
        }
    }
}

/// Result of the undirected projection.
#[derive(Debug, Clone)]
pub struct ToUndirectedResult {
    /// Graph name assigned to the mutated store.
    pub graph_name: String,
    /// Relationship type assigned to the undirected edges.
    pub mutate_relationship_type: String,
    /// Node count of the resulting graph store.
    pub node_count: u64,
    /// Relationship count of the resulting graph store.
    pub relationship_count: u64,
    /// Unique directed edges (source, target) representing the undirected view.
    pub edges: Vec<(u64, u64)>,
    /// Materialized graph store with the undirected relationship type added.
    pub graph_store: DefaultGraphStore,
}

/// Statistics for ToUndirected computation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToUndirectedStats {
    pub graph_name: String,
    pub mutate_relationship_type: String,
    pub node_count: u64,
    pub relationship_count: u64,
}

/// Marker used by the executor catalog.
pub struct ToUndirectedAlgorithmSpec {
    graph_name: String,
}

impl ToUndirectedAlgorithmSpec {
    pub fn new(graph_name: String) -> Self {
        Self { graph_name }
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }
}
