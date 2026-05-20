//! Degree Centrality storage runtime
//!
//! Storage owns top-level control:
//! - graph projection (orientation + optional weight selector)
//! - concurrency and partitioning
//! - termination checks
//! - progress callbacks

use crate::algo::degree_centrality::DegreeCentralityComputationRuntime;
use crate::collections::HugeAtomicDoubleArray;
use crate::concurrency::virtual_threads::Executor;
use crate::concurrency::{Concurrency, TerminatedException, TerminationFlag};
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::{Orientation as ProjectionOrientation, RelationshipType};
use crate::types::graph::Graph;
use crate::types::graph::NodeId;
use crate::types::prelude::GraphStore;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Orientation {
    Natural,
    Reverse,
    Undirected,
}

impl Default for Orientation {
    fn default() -> Self {
        Self::Natural
    }
}

impl Orientation {
    fn to_projection(self) -> ProjectionOrientation {
        match self {
            Orientation::Natural => ProjectionOrientation::Natural,
            Orientation::Reverse => ProjectionOrientation::Reverse,
            Orientation::Undirected => ProjectionOrientation::Undirected,
        }
    }
}

pub struct DegreeCentralityStorageRuntime<'a, G: GraphStore> {
    graph_store: &'a G,
    graph: Arc<dyn Graph>,
    orientation: Orientation,
    weighted: bool,
}

impl<'a, G: GraphStore> DegreeCentralityStorageRuntime<'a, G> {
    pub fn with_settings(
        graph_store: &'a G,
        orientation: Orientation,
        weighted: bool,
    ) -> Result<Self, AlgorithmError> {
        Self::with_relationship_weight_property(
            graph_store,
            orientation,
            weighted.then_some("weight"),
        )
    }

    pub fn with_relationship_weight_property(
        graph_store: &'a G,
        orientation: Orientation,
        relationship_weight_property: Option<&str>,
    ) -> Result<Self, AlgorithmError> {
        let rel_types: HashSet<RelationshipType> = graph_store.relationship_types();

        let graph = if let Some(weight_property) = relationship_weight_property {
            let selectors: HashMap<RelationshipType, String> = rel_types
                .iter()
                .map(|t| (t.clone(), weight_property.to_string()))
                .collect();

            graph_store
                .get_graph_with_types_selectors_and_orientation(
                    &rel_types,
                    &selectors,
                    orientation.to_projection(),
                )
                .map_err(|e| AlgorithmError::Graph(e.to_string()))?
        } else {
            graph_store
                .get_graph_with_types_and_orientation(&rel_types, orientation.to_projection())
                .map_err(|e| AlgorithmError::Graph(e.to_string()))?
        };

        Ok(Self {
            graph_store,
            graph,
            orientation,
            weighted: relationship_weight_property.is_some(),
        })
    }

    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    pub fn graph_store(&self) -> &'a G {
        self.graph_store
    }

    pub fn weighted(&self) -> bool {
        self.weighted
    }

    fn degree_unweighted(&self, node_idx: usize) -> f64 {
        let node_id = match NodeId::try_from(node_idx as i64) {
            Ok(id) => id,
            Err(_) => return 0.0,
        };

        match self.orientation {
            Orientation::Natural => self.graph.degree(node_id) as f64,
            Orientation::Reverse => self
                .graph
                .stream_inverse_relationships(node_id, 0.0)
                .count() as f64,
            Orientation::Undirected => {
                let outgoing = self.graph.degree(node_id) as f64;
                let incoming = self
                    .graph
                    .stream_inverse_relationships(node_id, 0.0)
                    .count() as f64;
                outgoing + incoming
            }
        }
    }

    fn degree_weighted(&self, node_idx: usize) -> f64 {
        let node_id = match NodeId::try_from(node_idx as i64) {
            Ok(id) => id,
            Err(_) => return 0.0,
        };

        match self.orientation {
            Orientation::Natural => {
                positive_weight_sum(self.graph.stream_relationships(node_id, 0.0))
            }
            Orientation::Reverse => {
                positive_weight_sum(self.graph.stream_inverse_relationships(node_id, 0.0))
            }
            Orientation::Undirected => {
                positive_weight_sum(self.graph.stream_relationships(node_id, 0.0))
                    + positive_weight_sum(self.graph.stream_inverse_relationships(node_id, 0.0))
            }
        }
    }

    pub fn compute_parallel(
        &self,
        computation: &DegreeCentralityComputationRuntime,
        concurrency: Concurrency,
        termination: &TerminationFlag,
        on_nodes_done: Arc<dyn Fn(usize) + Send + Sync>,
    ) -> Result<Vec<f64>, TerminatedException> {
        let node_count = self.node_count();
        if node_count == 0 {
            return Ok(Vec::new());
        }

        let concurrency = concurrency.value();
        let out = HugeAtomicDoubleArray::new(node_count);

        let target_batches = concurrency * 8;
        let batch_size = ((node_count + target_batches - 1) / target_batches).max(1);
        let batch_count = (node_count + batch_size - 1) / batch_size;

        let executor = Executor::new(Concurrency::of(concurrency));

        if self.weighted {
            let degree = |n: usize| self.degree_weighted(n);
            executor.parallel_for(0, batch_count, termination, |batch_idx| {
                if !termination.running() {
                    return;
                }

                let start = batch_idx * batch_size;
                let end = (start + batch_size).min(node_count);

                computation.compute_range(start, end, termination, &out, &degree);

                (on_nodes_done.as_ref())(end - start);
            })?;
        } else {
            let degree = |n: usize| self.degree_unweighted(n);
            executor.parallel_for(0, batch_count, termination, |batch_idx| {
                if !termination.running() {
                    return;
                }

                let start = batch_idx * batch_size;
                let end = (start + batch_size).min(node_count);

                computation.compute_range(start, end, termination, &out, &degree);

                (on_nodes_done.as_ref())(end - start);
            })?;
        }

        let mut result = vec![0.0f64; node_count];
        for i in 0..node_count {
            result[i] = out.get(i);
        }

        Ok(result)
    }
}

fn positive_weight_sum(
    relationships: crate::types::properties::relationship::RelationshipStream<'_>,
) -> f64 {
    relationships
        .map(|cursor| cursor.property())
        .filter(|weight| *weight > 0.0)
        .sum::<f64>()
}
