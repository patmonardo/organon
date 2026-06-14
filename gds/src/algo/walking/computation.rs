//! CollapsePath computation runtime.
//!
//! Translation source: `org.neo4j.gds.walking.CollapsePath` and friends.
//!
//! This implementation is a straightforward, sequential traversal that follows
//! each path template depth by depth and records the resulting collapsed edges.

use crate::task::concurrency::TerminationFlag;
use crate::types::graph::Graph;
use std::collections::BTreeSet;
use std::sync::Arc;

/// Ephemeral computation state for CollapsePath.
pub struct CollapsePathComputationRuntime {
    allow_self_loops: bool,
}

impl CollapsePathComputationRuntime {
    pub fn new(allow_self_loops: bool) -> Self {
        Self { allow_self_loops }
    }

    /// Compute collapsed edges for all provided path templates.
    pub fn compute(&self, path_templates: &[Vec<Arc<dyn Graph>>]) -> Vec<(u64, u64)> {
        self.compute_with_controls(path_templates, &TerminationFlag::running_true(), |_| {})
            .unwrap_or_default()
    }

    pub fn compute_with_controls(
        &self,
        path_templates: &[Vec<Arc<dyn Graph>>],
        termination_flag: &TerminationFlag,
        mut on_source_done: impl FnMut(usize),
    ) -> Result<Vec<(u64, u64)>, String> {
        let mut edges: BTreeSet<(u64, u64)> = BTreeSet::new();

        for path in path_templates {
            process_template_with_progress(
                path,
                self.allow_self_loops,
                &mut edges,
                termination_flag,
                &mut on_source_done,
            )?;
        }

        Ok(edges.into_iter().collect())
    }
}

fn process_template_with_progress(
    graphs: &[Arc<dyn Graph>],
    allow_self_loops: bool,
    edges: &mut BTreeSet<(u64, u64)>,
    termination_flag: &TerminationFlag,
    mut on_source_done: impl FnMut(usize),
) -> Result<(), String> {
    if graphs.is_empty() {
        return Ok(());
    }

    let node_count = graphs[0].node_count() as u64;

    for source in 0..node_count {
        if !termination_flag.running() {
            return Err("CollapsePath terminated".to_string());
        }

        let mut frontier: Vec<u64> = vec![source];

        for (depth, graph) in graphs.iter().enumerate() {
            let fallback = graph.default_property_value();
            let mut next_frontier: Vec<u64> = Vec::new();

            for &node in &frontier {
                for cursor in graph.stream_relationships(node as i64, fallback) {
                    let target = cursor.target_id();
                    if target >= 0 {
                        next_frontier.push(target as u64);
                    }
                }
            }

            if next_frontier.is_empty() {
                frontier.clear();
                break;
            }

            frontier = next_frontier;

            // If this was the final depth, stop expanding.
            if depth + 1 == graphs.len() {
                break;
            }
        }

        if frontier.is_empty() {
            on_source_done(1);
            continue;
        }

        for target in frontier {
            if allow_self_loops || target != source {
                edges.insert((source, target));
            }
        }

        on_source_done(1);
    }

    Ok(())
}
