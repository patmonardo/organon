//! Closeness Centrality Storage Runtime
//!
//! Storage-first controller for closeness centrality.
//!
//! Responsibilities:
//! - Build an oriented graph view from a `GraphStore`
//! - Provide neighbor access
//! - Own the top-level pipeline (farness -> closeness)

use crate::task::concurrency::{Concurrency, TerminatedException, TerminationFlag};
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::{Orientation, RelationshipType};
use crate::types::graph::Graph;
use crate::types::graph::MappedNodeId;
use crate::types::prelude::GraphStore;
use std::collections::HashSet;
use std::sync::Arc;

use super::ClosenessCentralityComputationRuntime;

pub struct ClosenessCentralityStorageRuntime<'a, G: GraphStore> {
    graph_store: &'a G,
    graph: Arc<dyn Graph>,
    orientation: Orientation,
}

impl<'a, G: GraphStore> ClosenessCentralityStorageRuntime<'a, G> {
    pub fn new(graph_store: &'a G, orientation: Orientation) -> Result<Self, AlgorithmError> {
        let rel_types: HashSet<RelationshipType> = graph_store.relationship_types();
        let graph = graph_store
            .get_graph_with_types_and_orientation(&rel_types, orientation)
            .map_err(|e| AlgorithmError::Graph(e.to_string()))?;

        Ok(Self {
            graph_store,
            graph,
            orientation,
        })
    }

    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    pub fn graph_store(&self) -> &'a G {
        self.graph_store
    }

    pub fn graph(&self) -> &Arc<dyn Graph> {
        &self.graph
    }

    pub fn neighbors(&self, node_idx: usize) -> Vec<usize> {
        let node_id = match MappedNodeId::try_from(node_idx) {
            Ok(id) => id,
            Err(_) => return Vec::new(),
        };

        let fallback = self.graph.default_property_value();
        match self.orientation {
            Orientation::Natural => self
                .graph
                .stream_relationships(node_id, fallback)
                .map(|cursor| cursor.target_id())
                .filter_map(MappedNodeId::to_usize)
                .collect(),
            Orientation::Reverse => self
                .graph
                .stream_inverse_relationships(node_id, fallback)
                .map(|cursor| cursor.source_id())
                .filter_map(MappedNodeId::to_usize)
                .collect(),
            Orientation::Undirected => {
                let mut neighbors = Vec::new();
                let mut seen = HashSet::new();

                for cursor in self.graph.stream_relationships(node_id, fallback) {
                    let target = cursor.target_id();
                    if seen.insert(target) {
                        if let Some(target_index) = target.to_usize() {
                            neighbors.push(target_index);
                        }
                    }
                }

                for cursor in self.graph.stream_inverse_relationships(node_id, fallback) {
                    let source = cursor.source_id();
                    if seen.insert(source) {
                        if let Some(source_index) = source.to_usize() {
                            neighbors.push(source_index);
                        }
                    }
                }

                neighbors
            }
        }
    }

    /// Storage-owned closeness pipeline.
    pub fn compute_parallel(
        &self,
        computation: &ClosenessCentralityComputationRuntime,
        wasserman_faust: bool,
        concurrency: Concurrency,
        termination: &TerminationFlag,
        on_farness_sources_done: Arc<dyn Fn(usize) + Send + Sync>,
        on_closeness_nodes_done: Arc<dyn Fn(usize) + Send + Sync>,
    ) -> Result<Vec<f64>, TerminatedException> {
        let node_count = self.node_count();
        if node_count == 0 {
            return Ok(Vec::new());
        }

        let neighbors = |n: usize| self.neighbors(n);

        let (farness, component) = computation.compute_farness_parallel(
            node_count,
            concurrency.value(),
            termination,
            on_farness_sources_done,
            &neighbors,
        )?;

        computation.compute_closeness_parallel(
            node_count,
            wasserman_faust,
            concurrency.value(),
            termination,
            &farness,
            &component,
            on_closeness_nodes_done,
        )
    }
}
