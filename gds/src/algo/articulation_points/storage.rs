//! Articulation Points Storage Runtime
//!
//! Articulation points are defined on undirected connectivity; this storage layer
//! always projects an undirected graph view and exposes a neighbor callback.

use crate::algo::articulation_points::{
    ArticulationPointsComputationResult, ArticulationPointsComputationRuntime, StackEvent,
};
use crate::core::utils::progress::ProgressTracker;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::{Orientation, RelationshipType};
use crate::types::graph::Graph;
use crate::types::graph::NodeId;
use crate::types::prelude::GraphStore;
use std::collections::HashSet;
use std::sync::Arc;

pub struct ArticulationPointsStorageRuntime<'a, G: GraphStore> {
    graph_store: &'a G,
    graph: Arc<dyn Graph>,
}

impl<'a, G: GraphStore> ArticulationPointsStorageRuntime<'a, G> {
    pub fn new(graph_store: &'a G) -> Result<Self, AlgorithmError> {
        let rel_types: HashSet<RelationshipType> = graph_store.relationship_types();
        let graph = graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Undirected)
            .map_err(|e| AlgorithmError::Graph(e.to_string()))?;

        Ok(Self { graph_store, graph })
    }

    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    pub fn relationship_count(&self) -> usize {
        self.graph.relationship_count()
    }

    pub fn graph_store(&self) -> &'a G {
        self.graph_store
    }

    pub fn graph(&self) -> &Arc<dyn Graph> {
        &self.graph
    }

    pub fn neighbors(&self, node_idx: usize) -> Vec<usize> {
        let node_id = match NodeId::try_from(node_idx as i64) {
            Ok(id) => id,
            Err(_) => return Vec::new(),
        };

        let fallback = self.graph.default_property_value();
        let mut neighbors = Vec::new();
        let mut seen = HashSet::new();

        for cursor in self.graph.stream_relationships(node_id, fallback) {
            let target = cursor.target_id();
            if target >= 0 && seen.insert(target) {
                neighbors.push(target as usize);
            }
        }

        for cursor in self.graph.stream_inverse_relationships(node_id, fallback) {
            let source = cursor.source_id();
            if source >= 0 && seen.insert(source) {
                neighbors.push(source as usize);
            }
        }

        neighbors
    }

    pub fn compute_articulation_points(
        &self,
        computation: &mut ArticulationPointsComputationRuntime,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<ArticulationPointsComputationResult, AlgorithmError> {
        let node_count = self.graph.node_count();
        let relationship_count = self.graph.relationship_count();

        computation.initialize(node_count);

        for node in 0..node_count {
            if !computation.is_visited(node) {
                self.dfs_component_root(computation, node, relationship_count, progress_tracker);
            }
        }

        Ok(computation.finalize_result())
    }

    fn dfs_component_root(
        &self,
        computation: &mut ArticulationPointsComputationRuntime,
        root: usize,
        relationship_count: usize,
        progress_tracker: &mut dyn ProgressTracker,
    ) {
        let mut stack: Vec<StackEvent> = Vec::new();
        if relationship_count > 0 {
            let _ = stack.try_reserve(relationship_count);
        }
        stack.push(StackEvent::upcoming_visit(root, None));

        while let Some(event) = stack.pop() {
            self.visit_event(computation, event, &mut stack, progress_tracker);
        }

        // Root rule: root is articulation point iff it has > 1 DFS-tree children.
        if computation.get_children(root) > 1 {
            computation.set_articulation_point(root);
        } else {
            computation.clear_articulation_point(root);
        }
        progress_tracker.log_progress(1);
    }

    fn visit_event(
        &self,
        computation: &mut ArticulationPointsComputationRuntime,
        event: StackEvent,
        stack: &mut Vec<StackEvent>,
        progress_tracker: &mut dyn ProgressTracker,
    ) {
        if event.is_last_visit() {
            let to = event.event_node;
            let v = match event.trigger_node {
                Some(v) => v,
                None => return,
            };

            let low_v = computation.get_low(v);
            let low_to = computation.get_low(to);
            computation.set_low(v, std::cmp::min(low_v, low_to));

            let tin_v = computation.get_tin(v);
            if low_to >= tin_v {
                computation.set_articulation_point(v);
            }

            computation.add_to_children(v, 1);
            progress_tracker.log_progress(1);
            return;
        }

        let v = event.event_node;
        let parent = event.trigger_node;

        if !computation.is_visited(v) {
            computation.set_visited(v);
            computation.set_children(v, 0);

            let timer_value = computation.increment_timer();
            computation.set_tin(v, timer_value);
            computation.set_low(v, timer_value);

            // Post event must be pushed before exploring neighbors.
            if let Some(p) = parent {
                stack.push(StackEvent::last_visit(v, p));
            }

            for to in self.neighbors(v) {
                if Some(to) == parent {
                    continue;
                }
                stack.push(StackEvent::upcoming_visit(to, Some(v)));
            }
        } else if let Some(p) = parent {
            // Back edge: update low(parent) with tin(v)
            let low_p = computation.get_low(p);
            let tin_v = computation.get_tin(v);
            computation.set_low(p, std::cmp::min(low_p, tin_v));
        }
    }
}
