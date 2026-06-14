//! ApproxMaxKCut storage runtime.
//!
//! Acts as the controller: obtains the undirected (natural) graph view, builds
//! adjacency once, tracks progress, and hands neighbors to the computation
//! runtime.

use super::spec::{ApproxMaxKCutConfig, ApproxMaxKCutResult};
use super::ApproxMaxKCutComputationRuntime;
use crate::task::concurrency::TerminationFlag;
use crate::task::progress::ProgressTracker;
use crate::projection::Orientation;
use crate::projection::RelationshipType;
use crate::types::prelude::GraphStore;
use std::collections::HashSet;

#[derive(Debug, Default, Clone)]
pub struct ApproxMaxKCutStorageRuntime;

impl ApproxMaxKCutStorageRuntime {
    pub fn new() -> Self {
        Self
    }

    pub fn compute_approx_max_kcut(
        &self,
        computation: &mut ApproxMaxKCutComputationRuntime,
        graph_store: &impl GraphStore,
        config: &ApproxMaxKCutConfig,
        progress_tracker: &mut dyn ProgressTracker,
        termination_flag: &TerminationFlag,
    ) -> Result<ApproxMaxKCutResult, String> {
        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph_view = graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Natural)
            .map_err(|e| format!("failed to build graph view: {e}"))?;

        let node_count = graph_view.node_count();
        config
            .validate_for_node_count(node_count)
            .map_err(|e| format!("invalid config: {e}"))?;

        if node_count == 0 {
            return Ok(ApproxMaxKCutResult {
                communities: Vec::new(),
                cut_cost: 0.0,
                k: config.k,
                node_count,
                execution_time: std::time::Duration::default(),
            });
        }

        let default_weight = if config.has_relationship_weight_property {
            graph_view.default_property_value()
        } else {
            1.0
        };

        progress_tracker.begin_subtask_with_description("ApproxMaxKCut");
        progress_tracker
            .begin_subtask_with_description_and_volume("place nodes randomly", node_count);
        let mut adjacency: Vec<Vec<(usize, f64)>> = vec![Vec::new(); node_count];
        for node_id in 0..node_count {
            termination_flag.assert_running();

            for cursor in graph_view.stream_relationships(node_id as i64, default_weight) {
                let target = cursor.target_id();
                if target < 0 {
                    continue;
                }

                let weight = if config.has_relationship_weight_property {
                    cursor.property()
                } else {
                    1.0
                };

                adjacency[node_id].push((target as usize, weight));
            }

            progress_tracker.log_progress(1);
        }
        progress_tracker.end_subtask_with_description("place nodes randomly");

        let get_neighbors =
            |node: usize| -> Vec<(usize, f64)> { adjacency.get(node).cloned().unwrap_or_default() };

        let search_description = if config.vns_max_neighborhood_order > 0 {
            "variable neighborhood search"
        } else {
            "local search"
        };
        progress_tracker.begin_subtask_with_description_and_volume(search_description, node_count);
        let result = computation.compute(node_count, get_neighbors);
        termination_flag.assert_running();
        progress_tracker.log_progress(node_count);
        progress_tracker.end_subtask_with_description(search_description);
        progress_tracker.end_subtask_with_description("ApproxMaxKCut");

        Ok(result)
    }
}
