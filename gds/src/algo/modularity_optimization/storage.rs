use super::spec::{ModularityOptimizationConfig, ModularityOptimizationResult};
use super::{ModularityOptimizationComputationRuntime, ModularityOptimizationInput};
use crate::task::concurrency::TerminationFlag;
use crate::task::progress::ProgressTracker;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::{Orientation, RelationshipType};
use crate::types::graph::Graph;
use crate::types::prelude::GraphStore;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

#[derive(Clone)]
pub struct ModularityOptimizationStorageRuntime {
    graph: Arc<dyn Graph>,
    relationship_count: usize,
}

impl ModularityOptimizationStorageRuntime {
    pub fn new<G: GraphStore>(
        graph_store: &G,
        config: &ModularityOptimizationConfig,
    ) -> Result<Self, AlgorithmError> {
        let rel_types: HashSet<RelationshipType> = graph_store.relationship_types();
        let graph = if let Some(weight_property) = &config.relationship_weight_property {
            let selectors: HashMap<RelationshipType, String> = rel_types
                .iter()
                .map(|rel_type| (rel_type.clone(), weight_property.clone()))
                .collect();
            graph_store
                .get_graph_with_types_selectors_and_orientation(
                    &rel_types,
                    &selectors,
                    Orientation::Undirected,
                )
                .map_err(|e| AlgorithmError::Graph(e.to_string()))?
        } else {
            graph_store
                .get_graph_with_types_and_orientation(&rel_types, Orientation::Undirected)
                .map_err(|e| AlgorithmError::Graph(e.to_string()))?
        };

        let relationship_count = graph.relationship_count();
        Ok(Self {
            graph,
            relationship_count,
        })
    }

    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    pub fn relationship_count(&self) -> usize {
        self.relationship_count
    }

    pub fn compute_modularity_optimization(
        &self,
        computation: &mut ModularityOptimizationComputationRuntime,
        config: &ModularityOptimizationConfig,
        progress_tracker: &mut dyn ProgressTracker,
        termination_flag: &TerminationFlag,
    ) -> Result<ModularityOptimizationResult, AlgorithmError> {
        let node_count = self.graph.node_count();
        if node_count == 0 {
            return Ok(
                computation.compute(&ModularityOptimizationInput::new(0, Vec::new()), config)
            );
        }

        progress_tracker.begin_subtask_with_description("ModularityOptimization");
        progress_tracker.begin_subtask_with_description_and_volume("initialization", node_count);

        let weight_fallback = if config.relationship_weight_property.is_some() {
            self.graph.default_property_value()
        } else {
            1.0
        };

        let mut adj: Vec<Vec<(usize, f64)>> = vec![Vec::new(); node_count];
        for (node_id, neighbors) in adj.iter_mut().enumerate() {
            termination_flag.assert_running();
            let stream = self
                .graph
                .stream_relationships_weighted(node_id as i64, weight_fallback);
            for cursor in stream {
                let target = cursor.target_id();
                if target >= 0 {
                    neighbors.push((target as usize, cursor.weight()));
                }
            }
            progress_tracker.log_progress(1);
        }
        progress_tracker.end_subtask_with_description("initialization");

        progress_tracker.begin_subtask_with_description("compute modularity");
        progress_tracker
            .begin_subtask_with_description_and_volume("optimizeForColor", self.relationship_count);
        let result =
            computation.compute(&ModularityOptimizationInput::new(node_count, adj), config);
        progress_tracker.log_progress(self.relationship_count);
        progress_tracker.end_subtask_with_description("optimizeForColor");
        progress_tracker.end_subtask_with_description("compute modularity");
        progress_tracker.end_subtask_with_description("ModularityOptimization");

        Ok(result)
    }
}
