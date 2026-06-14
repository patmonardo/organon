//! ToUndirected computation runtime.
//!
//! Translation source: `org.neo4j.gds.undirected.ToUndirected` task logic.
//!
//! Symmetrizes a single relationship type by adding reverse edges for every
//! observed edge, producing a deduplicated edge set.

use crate::task::concurrency::TerminationFlag;
use crate::core::Aggregation;
use crate::types::graph::Graph;
use std::collections::{BTreeMap, BTreeSet};

pub struct ToUndirectedComputationRuntime;

impl ToUndirectedComputationRuntime {
    pub fn new() -> Self {
        Self
    }

    /// Symmetrize the given graph view, returning unique directed edges that
    /// collectively represent an undirected projection.
    pub fn compute(&self, graph: &dyn Graph, _mutate_relationship_type: &str) -> Vec<(u64, u64)> {
        self.compute_with_controls(
            graph,
            _mutate_relationship_type,
            &TerminationFlag::running_true(),
            |_| {},
        )
        .unwrap_or_default()
    }

    pub fn compute_with_controls(
        &self,
        graph: &dyn Graph,
        _mutate_relationship_type: &str,
        termination_flag: &TerminationFlag,
        mut on_source_done: impl FnMut(usize),
    ) -> Result<Vec<(u64, u64)>, String> {
        let node_count = graph.node_count();
        let fallback = graph.default_property_value();

        let mut edges: BTreeSet<(u64, u64)> = BTreeSet::new();

        for source in 0..node_count {
            if !termination_flag.running() {
                return Err("ToUndirected terminated".to_string());
            }

            for cursor in graph.stream_relationships(source as i64, fallback) {
                let target = cursor.target_id();
                if target < 0 {
                    continue;
                }
                let u = source as u64;
                let v = target as u64;
                edges.insert((u, v));
                edges.insert((v, u));
            }
            on_source_done(1);
        }

        Ok(edges.into_iter().collect())
    }

    /// Aggregate a selected relationship property by unordered node pair.
    pub fn aggregate_property(
        &self,
        graph: &dyn Graph,
        aggregation: Aggregation,
    ) -> Result<BTreeMap<(u64, u64), f64>, String> {
        self.aggregate_property_with_controls(
            graph,
            aggregation,
            &TerminationFlag::running_true(),
            |_| {},
        )
    }

    pub fn aggregate_property_with_controls(
        &self,
        graph: &dyn Graph,
        aggregation: Aggregation,
        termination_flag: &TerminationFlag,
        mut on_source_done: impl FnMut(usize),
    ) -> Result<BTreeMap<(u64, u64), f64>, String> {
        let node_count = graph.node_count();
        let fallback = graph.default_property_value();
        let aggregation = aggregation.resolve();
        let mut values: BTreeMap<(u64, u64), f64> = BTreeMap::new();

        for source in 0..node_count {
            if !termination_flag.running() {
                return Err("ToUndirected terminated".to_string());
            }

            for cursor in graph.stream_relationships(source as i64, fallback) {
                let target = cursor.target_id();
                if target < 0 {
                    continue;
                }

                let u = source as u64;
                let v = target as u64;
                let key = if u <= v { (u, v) } else { (v, u) };
                let value = aggregation.normalize_property_value(cursor.property());

                if let Some(running_total) = values.get_mut(&key) {
                    *running_total = aggregation.merge(*running_total, value).map_err(|e| {
                        format!(
                            "failed to aggregate property for edge ({},{}): {e}",
                            key.0, key.1
                        )
                    })?;
                } else {
                    values.insert(key, value);
                }
            }
            on_source_done(1);
        }

        Ok(values)
    }
}
