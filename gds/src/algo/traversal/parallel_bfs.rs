use super::{Aggregator, ExitPredicate, ExitPredicateResult};
use crate::concurrency::{install_with_concurrency, Concurrency};
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::graph::{Graph, NodeId};
use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug, Clone, Copy)]
pub struct ParallelBfsConfig {
    pub source_node: NodeId,
    pub node_count: usize,
    pub max_depth: Option<u32>,
    pub concurrency: usize,
    pub delta: usize,
}

#[derive(Debug, Clone)]
pub struct ParallelBfsResult {
    pub visited_nodes: Vec<NodeId>,
    pub visited_depths: Vec<f64>,
    pub relationships_examined: usize,
}

struct ChunkExpansion {
    examined_relationships: usize,
    candidates: Vec<(NodeId, NodeId, f64)>,
}

pub fn run_parallel_bfs(
    graph: &dyn Graph,
    config: ParallelBfsConfig,
    aggregator: &dyn Aggregator,
    exit_predicate: &dyn ExitPredicate,
) -> Result<ParallelBfsResult, AlgorithmError> {
    validate_node_in_graph(config.source_node, config.node_count, "source")?;

    let fallback = graph.default_property_value();
    let visited: Vec<AtomicBool> = (0..config.node_count)
        .map(|_| AtomicBool::new(false))
        .collect();
    let source_idx = node_index_in_graph(config.source_node, "source")?;
    visited[source_idx].store(true, Ordering::SeqCst);

    let mut frontier: Vec<(NodeId, NodeId, f64)> =
        vec![(config.source_node, config.source_node, 0.0)];
    let mut result = Vec::new();
    let mut result_depths = Vec::new();
    let mut relationships_examined = 0usize;

    while !frontier.is_empty() {
        let mut expandable = Vec::new();

        for (source_node, current_node, weight) in frontier.into_iter() {
            match exit_predicate.test(source_node, current_node, weight) {
                ExitPredicateResult::Continue => continue,
                ExitPredicateResult::Break => {
                    result.push(current_node);
                    result_depths.push(weight);
                    return Ok(ParallelBfsResult {
                        visited_nodes: result,
                        visited_depths: result_depths,
                        relationships_examined,
                    });
                }
                ExitPredicateResult::Follow => {
                    result.push(current_node);
                    result_depths.push(weight);
                    if check_max_depth(config.max_depth, weight) {
                        expandable.push((source_node, current_node, weight));
                    }
                }
            }
        }

        if expandable.is_empty() {
            break;
        }

        let chunk_size = config.delta.max(1);
        let chunk_count = expandable.len().div_ceil(chunk_size);
        let concurrency = Concurrency::from_usize(config.concurrency.max(1));

        let chunk_outputs: Vec<ChunkExpansion> = install_with_concurrency(concurrency, || {
            (0..chunk_count)
                .into_par_iter()
                .map(|chunk_idx| -> Result<ChunkExpansion, AlgorithmError> {
                    let start = chunk_idx * chunk_size;
                    let end = (start + chunk_size).min(expandable.len());
                    let mut examined_relationships = 0usize;
                    let mut candidates = Vec::new();
                    let worker_graph = Graph::concurrent_copy(graph);

                    for (_source_node, current_node, weight) in &expandable[start..end] {
                        let stream = worker_graph.stream_relationships(*current_node, fallback);
                        for cursor in stream {
                            examined_relationships += 1;
                            let neighbor = cursor.target_id();
                            validate_node_in_graph(neighbor, config.node_count, "neighbor")?;
                            let next_weight = aggregator.apply(*current_node, neighbor, *weight);
                            candidates.push((*current_node, neighbor, next_weight));
                        }
                    }

                    Ok(ChunkExpansion {
                        examined_relationships,
                        candidates,
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })?;

        let mut next_frontier = Vec::new();
        for chunk in chunk_outputs {
            relationships_examined += chunk.examined_relationships;
            for (source_node, neighbor, next_weight) in chunk.candidates {
                let neighbor_idx = node_index_in_graph(neighbor, "neighbor")?;
                if !visited[neighbor_idx].swap(true, Ordering::SeqCst) {
                    next_frontier.push((source_node, neighbor, next_weight));
                }
            }
        }

        frontier = next_frontier;
    }

    Ok(ParallelBfsResult {
        visited_nodes: result,
        visited_depths: result_depths,
        relationships_examined,
    })
}

fn check_max_depth(max_depth: Option<u32>, current_depth: f64) -> bool {
    match max_depth {
        Some(max_depth) => current_depth < max_depth as f64,
        None => true,
    }
}

fn validate_node_in_graph(
    node_id: NodeId,
    node_count: usize,
    role: &str,
) -> Result<(), AlgorithmError> {
    let node_index = node_index_in_graph(node_id, role)?;
    if node_index >= node_count {
        return Err(AlgorithmError::InvalidGraph(format!(
            "{role} node id out of range: {node_id} (node_count={node_count})"
        )));
    }
    Ok(())
}

fn node_index_in_graph(node_id: NodeId, role: &str) -> Result<usize, AlgorithmError> {
    usize::try_from(node_id)
        .map_err(|_| AlgorithmError::InvalidGraph(format!("Invalid {role} node id: {node_id}")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algo::traversal::{FollowExitPredicate, OneHopAggregator};

    #[test]
    fn rejects_negative_source() {
        let config = ParallelBfsConfig {
            source_node: -1,
            node_count: 4,
            max_depth: None,
            concurrency: 2,
            delta: 64,
        };

        let err =
            validate_node_in_graph(config.source_node, config.node_count, "source").unwrap_err();
        assert!(matches!(err, AlgorithmError::InvalidGraph(_)));
    }

    #[test]
    fn traversal_hooks_are_thread_safe() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<OneHopAggregator>();
        assert_send_sync::<FollowExitPredicate>();
    }
}
