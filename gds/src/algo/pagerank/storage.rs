//! PageRank storage runtime
//!
//! Storage owns graph access and orchestration; computation remains pure state.

use crate::algo::pagerank::{PageRankComputationRuntime, PageRankRunResult};
use crate::task::concurrency::Concurrency;
use crate::core::utils::progress::ProgressTracker;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::{Orientation, RelationshipType};
use crate::types::graph::Graph;
use crate::types::prelude::GraphStore;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub struct PageRankStorageRuntime<'a, G: GraphStore> {
    graph_store: &'a G,
    graph: Arc<dyn Graph>,
    has_weights: bool,
}

impl<'a, G: GraphStore> PageRankStorageRuntime<'a, G> {
    pub fn new(
        graph_store: &'a G,
        orientation: Orientation,
        relationship_weight_property: Option<&str>,
    ) -> Result<Self, AlgorithmError> {
        let rel_types: HashSet<RelationshipType> = graph_store.relationship_types();
        Self::with_relationship_types_orientation_and_weight_property(
            graph_store,
            &rel_types,
            orientation,
            relationship_weight_property,
        )
    }

    pub fn with_relationship_types_and_orientation(
        graph_store: &'a G,
        rel_types: &HashSet<RelationshipType>,
        orientation: Orientation,
    ) -> Result<Self, AlgorithmError> {
        Self::with_relationship_types_orientation_and_weight_property(
            graph_store,
            rel_types,
            orientation,
            None,
        )
    }

    pub fn with_relationship_types_orientation_and_weight_property(
        graph_store: &'a G,
        rel_types: &HashSet<RelationshipType>,
        orientation: Orientation,
        relationship_weight_property: Option<&str>,
    ) -> Result<Self, AlgorithmError> {
        let graph = if let Some(weight_property) = relationship_weight_property {
            let selectors: HashMap<RelationshipType, String> = rel_types
                .iter()
                .map(|rel_type| (rel_type.clone(), weight_property.to_string()))
                .collect();

            graph_store
                .get_graph_with_types_selectors_and_orientation(rel_types, &selectors, orientation)
                .map_err(|e| AlgorithmError::Graph(e.to_string()))?
        } else {
            graph_store
                .get_graph_with_types_and_orientation(rel_types, orientation)
                .map_err(|e| AlgorithmError::Graph(e.to_string()))?
        };

        Ok(Self {
            graph_store,
            graph,
            has_weights: relationship_weight_property.is_some(),
        })
    }

    pub fn with_orientation(
        graph_store: &'a G,
        orientation: Orientation,
    ) -> Result<Self, AlgorithmError> {
        let rel_types: HashSet<RelationshipType> = graph_store.relationship_types();
        Self::with_relationship_types_and_orientation(graph_store, &rel_types, orientation)
    }

    pub fn graph_store(&self) -> &'a G {
        self.graph_store
    }

    pub fn graph(&self) -> Arc<dyn Graph> {
        Arc::clone(&self.graph)
    }

    pub fn has_weights(&self) -> bool {
        self.has_weights
    }

    pub fn run(
        &self,
        computation: &PageRankComputationRuntime,
        concurrency: Concurrency,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> PageRankRunResult {
        let node_count = self.graph.node_count() as usize;

        let mut degree: Vec<f64> = vec![0.0; node_count];
        let weight_fallback = if self.has_weights {
            self.graph.default_property_value()
        } else {
            1.0
        };

        for i in 0..node_count {
            degree[i] = if self.has_weights {
                self.graph
                    .stream_relationships_weighted(i as i64, weight_fallback)
                    .map(|cursor| cursor.weight())
                    .filter(|weight| weight.is_finite() && *weight > 0.0)
                    .sum()
            } else {
                self.graph.degree(i as i64) as f64
            };
        }

        let average_degree = if node_count == 0 {
            0.0
        } else {
            degree.iter().sum::<f64>() / node_count as f64
        };

        let stream_neighbors = |source: usize, push: &mut dyn FnMut(usize, f64)| {
            if self.has_weights {
                for cursor in self
                    .graph
                    .stream_relationships_weighted(source as i64, weight_fallback)
                {
                    let weight = cursor.weight();
                    if weight.is_finite() && weight > 0.0 {
                        push(cursor.target_id() as usize, weight);
                    }
                }
            } else {
                for cursor in self.graph.stream_relationships(source as i64, 0.0) {
                    push(cursor.target_id() as usize, 1.0);
                }
            }
        };

        computation.run_with_progress(
            node_count,
            &degree,
            average_degree,
            concurrency.value(),
            &stream_neighbors,
            |iterations_completed| progress_tracker.log_progress(iterations_completed),
        )
    }
}
