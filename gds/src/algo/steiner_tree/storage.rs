use crate::algo::steiner_tree::spec::{SteinerTreeConfig, SteinerTreeParent, SteinerTreeResult};
use crate::algo::steiner_tree::SteinerTreeComputationRuntime;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::task::concurrency::TerminationFlag;
use crate::task::progress::{ProgressTracker, UNKNOWN_VOLUME};
use crate::types::graph::Graph;
use crate::types::graph::MappedNodeId;
use std::collections::HashSet;

/// Steiner Tree Storage Runtime (controller).
///
/// Heuristic (Java-parity direction): iteratively connect the closest remaining
/// terminal to the current merged-to-source set using a delta-stepping style
/// shortest path search seeded with multiple sources at distance 0.
pub struct SteinerTreeStorageRuntime {
    pub config: SteinerTreeConfig,
    pub concurrency: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::progress::NoopProgressTracker;

    #[test]
    fn context_computation_honors_pre_terminated_request() {
        let config = SteinerTreeConfig {
            source_node: MappedNodeId::new(0),
            target_nodes: vec![MappedNodeId::new(1)],
            relationship_weight_property: None,
            delta: 1.0,
            apply_rerouting: true,
        };
        let storage = SteinerTreeStorageRuntime::new(config, 1);
        let mut computation = SteinerTreeComputationRuntime::new(1.0, 0);
        let mut progress_tracker = NoopProgressTracker;
        let termination = TerminationFlag::stop_running();

        let error = storage
            .compute_steiner_tree_with_context(
                &mut computation,
                None,
                &mut progress_tracker,
                &termination,
            )
            .unwrap_err();

        assert!(error.to_string().contains("terminated"));
    }
}

impl SteinerTreeStorageRuntime {
    pub fn new(config: SteinerTreeConfig, concurrency: usize) -> Self {
        Self {
            config,
            concurrency,
        }
    }

    pub fn compute_steiner_tree(
        &self,
        computation: &mut SteinerTreeComputationRuntime,
        graph: Option<&dyn Graph>,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<SteinerTreeResult, AlgorithmError> {
        self.compute_steiner_tree_with_termination(
            computation,
            graph,
            progress_tracker,
            &TerminationFlag::running_true(),
        )
    }

    pub fn compute_steiner_tree_with_termination(
        &self,
        computation: &mut SteinerTreeComputationRuntime,
        graph: Option<&dyn Graph>,
        progress_tracker: &mut dyn ProgressTracker,
        termination: &TerminationFlag,
    ) -> Result<SteinerTreeResult, AlgorithmError> {
        let volume = graph
            .map(|g| g.relationship_count())
            .unwrap_or(UNKNOWN_VOLUME);
        if volume == UNKNOWN_VOLUME {
            progress_tracker.begin_subtask_unknown();
        } else {
            progress_tracker.begin_subtask_with_volume(volume);
        }

        let result = self.compute_steiner_tree_with_context(
            computation,
            graph,
            progress_tracker,
            termination,
        );

        match result {
            Ok(ok) => {
                progress_tracker.end_subtask();
                Ok(ok)
            }
            Err(e) => {
                progress_tracker.end_subtask_with_failure();
                Err(e)
            }
        }
    }

    pub fn compute_steiner_tree_with_context(
        &self,
        computation: &mut SteinerTreeComputationRuntime,
        graph: Option<&dyn Graph>,
        progress_tracker: &mut dyn ProgressTracker,
        termination: &TerminationFlag,
    ) -> Result<SteinerTreeResult, AlgorithmError> {
        Self::ensure_running(termination)?;
        let node_count = graph.map(|graph| graph.node_count()).unwrap_or(0);
        let neighbor_fn =
            |node: MappedNodeId| self.get_neighbors_with_weights(graph, node, termination);

        self.compute_core(
            computation,
            node_count,
            &neighbor_fn,
            progress_tracker,
            termination,
        )
    }

    /// Test/utility entrypoint: run with an explicit neighbor provider.
    #[cfg(test)]
    pub(crate) fn compute_steiner_tree_with_neighbors<F>(
        &self,
        computation: &mut SteinerTreeComputationRuntime,
        node_count: usize,
        get_neighbors: &F,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<SteinerTreeResult, AlgorithmError>
    where
        F: Fn(MappedNodeId) -> Vec<(MappedNodeId, f64)>,
    {
        progress_tracker.begin_subtask_unknown();
        let termination = TerminationFlag::running_true();
        let neighbor_fn = |node| Ok(get_neighbors(node));
        let out = self.compute_core(
            computation,
            node_count,
            &neighbor_fn,
            progress_tracker,
            &termination,
        );
        match out {
            Ok(v) => {
                progress_tracker.end_subtask();
                Ok(v)
            }
            Err(e) => {
                progress_tracker.end_subtask_with_failure();
                Err(e)
            }
        }
    }

    fn compute_core<F>(
        &self,
        computation: &mut SteinerTreeComputationRuntime,
        node_count: usize,
        get_neighbors: &F,
        progress_tracker: &mut dyn ProgressTracker,
        termination: &TerminationFlag,
    ) -> Result<SteinerTreeResult, AlgorithmError>
    where
        F: Fn(MappedNodeId) -> Result<Vec<(MappedNodeId, f64)>, AlgorithmError>,
    {
        Self::ensure_running(termination)?;
        if node_count == 0 {
            return Ok(SteinerTreeResult {
                parent_array: vec![],
                relationship_to_parent_cost: vec![],
                total_cost: 0.0,
                effective_node_count: 0,
                effective_target_nodes_count: 0,
            });
        }

        if self.config.delta <= 0.0 {
            return Err(AlgorithmError::InvalidGraph(
                "delta must be > 0".to_string(),
            ));
        }

        let source = self.config.source_node;
        let source_index = source.to_usize().ok_or_else(|| {
            AlgorithmError::InvalidGraph("source_node exceeds physical index space".to_string())
        })?;
        if source_index >= node_count {
            return Err(AlgorithmError::InvalidGraph(
                "source_node out of bounds".to_string(),
            ));
        }
        if self.config.target_nodes.is_empty() {
            return Err(AlgorithmError::InvalidGraph(
                "target_nodes must not be empty".to_string(),
            ));
        }

        let mut terminals: Vec<MappedNodeId> = Vec::new();
        let mut seen = HashSet::new();
        for &t in &self.config.target_nodes {
            Self::ensure_running(termination)?;
            let target_index = t.to_usize().ok_or_else(|| {
                AlgorithmError::InvalidGraph("target node exceeds physical index space".to_string())
            })?;
            if target_index >= node_count {
                return Err(AlgorithmError::InvalidGraph(
                    "target_nodes contains out-of-bounds node".to_string(),
                ));
            }
            if seen.insert(t) {
                terminals.push(t);
            }
        }

        let mut is_terminal = vec![false; node_count];
        for &t in &terminals {
            Self::ensure_running(termination)?;
            is_terminal[physical_node_index(t)] = true;
        }

        computation.initialize_tree(source);

        let mut merged_to_source = vec![false; node_count];
        merged_to_source[source_index] = true;

        let mut remaining: Vec<MappedNodeId> =
            terminals.into_iter().filter(|&t| t != source).collect();

        while !remaining.is_empty() {
            Self::ensure_running(termination)?;

            self.run_multi_source_delta_stepping(
                computation,
                &merged_to_source,
                get_neighbors,
                progress_tracker,
                termination,
            )?;

            // Choose closest reachable terminal.
            let mut best_idx: Option<usize> = None;
            let mut best_dist = f64::INFINITY;
            for (idx, &t) in remaining.iter().enumerate() {
                Self::ensure_running(termination)?;
                let d = computation.distance(t);
                if d < best_dist {
                    best_dist = d;
                    best_idx = Some(idx);
                }
            }

            let Some(chosen_idx) = best_idx else {
                break;
            };
            if best_dist == f64::INFINITY {
                break; // no reachable remaining terminal
            }

            let chosen = remaining.swap_remove(chosen_idx);
            let _ = computation.merge_path_into_tree_with_context(
                chosen,
                &mut merged_to_source,
                termination,
            )?;
        }

        // Always prune non-terminal leaves; rerouting is optional and separate.
        computation.prune_non_terminal_leaves_with_context(&is_terminal, source, termination)?;

        // Optional rerouting/post-optimization stage (not implemented yet).
        let _apply_rerouting = self.config.apply_rerouting;

        // Aggregate totals.
        let mut total_cost = 0.0;
        let mut effective_node_count = 0u64;
        let mut effective_target_nodes_count = 0u64;
        for node_id in 0..node_count {
            Self::ensure_running(termination)?;
            let parent = computation.parent_array()[node_id];
            if parent == SteinerTreeParent::Pruned {
                continue;
            }
            effective_node_count += 1;
            if is_terminal[node_id] {
                effective_target_nodes_count += 1;
            }
            if matches!(parent, SteinerTreeParent::Parent(_)) {
                total_cost += computation.parent_cost_array()[node_id];
            }
        }

        Ok(SteinerTreeResult {
            parent_array: computation.parent_array().to_vec(),
            relationship_to_parent_cost: computation.parent_cost_array().to_vec(),
            total_cost,
            effective_node_count,
            effective_target_nodes_count,
        })
    }

    fn run_multi_source_delta_stepping<F>(
        &self,
        computation: &mut SteinerTreeComputationRuntime,
        merged_to_source: &[bool],
        get_neighbors: &F,
        progress_tracker: &mut dyn ProgressTracker,
        termination: &TerminationFlag,
    ) -> Result<(), AlgorithmError>
    where
        F: Fn(MappedNodeId) -> Result<Vec<(MappedNodeId, f64)>, AlgorithmError>,
    {
        let mut scanned_relationships: usize = 0;
        const LOG_BATCH: usize = 256;

        let mut frontier = computation.reset_search_with_context(merged_to_source, termination)?;
        let mut current_bin: usize = 0;

        let max_iterations = merged_to_source.len().saturating_mul(2).max(1);
        let mut iteration = 0usize;

        while !frontier.is_empty() && iteration < max_iterations {
            Self::ensure_running(termination)?;

            let mut next_frontier = std::collections::VecDeque::new();

            while let Some(node) = frontier.pop_front() {
                Self::ensure_running(termination)?;
                let node_distance = computation.distance(node);
                if node_distance >= self.config.delta * current_bin as f64 {
                    let neighbors = get_neighbors(node)?;
                    scanned_relationships = scanned_relationships.saturating_add(neighbors.len());
                    if scanned_relationships >= LOG_BATCH {
                        progress_tracker.log_progress(scanned_relationships);
                        scanned_relationships = 0;
                    }

                    for (nbr, weight) in neighbors {
                        Self::ensure_running(termination)?;
                        if weight.is_nan() || weight.is_infinite() || weight < 0.0 {
                            continue;
                        }
                        if computation.try_relax(node, nbr, weight) {
                            let new_dist = computation.distance(nbr);
                            let dest_bin = (new_dist / self.config.delta) as usize;
                            if dest_bin == current_bin {
                                next_frontier.push_back(nbr);
                            } else {
                                computation.add_to_bin(nbr, dest_bin);
                            }
                        }
                    }
                }
            }

            frontier = next_frontier;
            let Some(next_bin) = computation.find_next_non_empty_bin(current_bin) else {
                break;
            };
            current_bin = next_bin;
            for node in computation.drain_bin(current_bin) {
                Self::ensure_running(termination)?;
                frontier.push_back(node);
            }

            iteration += 1;
        }

        if scanned_relationships > 0 {
            progress_tracker.log_progress(scanned_relationships);
        }

        Ok(())
    }

    fn get_neighbors_with_weights(
        &self,
        graph: Option<&dyn Graph>,
        node_id: MappedNodeId,
        termination: &TerminationFlag,
    ) -> Result<Vec<(MappedNodeId, f64)>, AlgorithmError> {
        if let Some(g) = graph {
            let fallback: f64 = 1.0;
            let mut neighbors = Vec::new();
            for cursor in g.stream_relationships(node_id, fallback) {
                Self::ensure_running(termination)?;
                neighbors.push((cursor.target_id(), cursor.property()));
            }
            Ok(neighbors)
        } else {
            // Minimal mock for storage/computation integration tests.
            Ok(match node_id.get() {
                0 => vec![(MappedNodeId::new(1), 1.0), (MappedNodeId::new(2), 1.0)],
                1 => vec![(MappedNodeId::new(3), 1.0)],
                2 => vec![(MappedNodeId::new(4), 1.0)],
                _ => vec![],
            })
        }
    }

    fn ensure_running(termination: &TerminationFlag) -> Result<(), AlgorithmError> {
        if !termination.running() {
            return Err(AlgorithmError::Execution(
                "Steiner tree computation terminated".to_string(),
            ));
        }
        Ok(())
    }
}

fn physical_node_index(node_id: MappedNodeId) -> usize {
    node_id
        .to_usize()
        .expect("mapped graph node must fit physical index space")
}
