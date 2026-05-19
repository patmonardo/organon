use super::{Aggregator, ExitPredicate, ExitPredicateResult};
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::graph::NodeId;

#[derive(Debug, Clone, Copy)]
pub struct SequentialDfsConfig {
    pub source_node: NodeId,
    pub node_count: usize,
    pub max_depth: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct SequentialDfsResult {
    pub visited_nodes: Vec<NodeId>,
    pub relationships_examined: usize,
}

pub fn run_sequential_dfs<F>(
    config: SequentialDfsConfig,
    aggregator: &dyn Aggregator,
    exit_predicate: &dyn ExitPredicate,
    get_neighbors: F,
) -> Result<SequentialDfsResult, AlgorithmError>
where
    F: Fn(NodeId) -> Vec<NodeId>,
{
    validate_node_in_graph(config.source_node, config.node_count, "source")?;

    let mut visited = vec![false; config.node_count];
    let source_idx = node_index_in_graph(config.source_node, "source")?;
    visited[source_idx] = true;

    let mut stack = vec![(config.source_node, config.source_node, 0.0)];
    let mut result = Vec::new();
    let mut relationships_examined = 0usize;

    while let Some((source_node, current_node, weight)) = stack.pop() {
        match exit_predicate.test(source_node, current_node, weight) {
            ExitPredicateResult::Continue => continue,
            ExitPredicateResult::Break => {
                result.push(current_node);
                break;
            }
            ExitPredicateResult::Follow => result.push(current_node),
        }

        let neighbors = get_neighbors(current_node);
        relationships_examined += neighbors.len();

        if check_max_depth(config.max_depth, weight) {
            for neighbor in neighbors {
                validate_node_in_graph(neighbor, config.node_count, "neighbor")?;
                let neighbor_idx = node_index_in_graph(neighbor, "neighbor")?;
                if !visited[neighbor_idx] {
                    visited[neighbor_idx] = true;
                    stack.push((
                        current_node,
                        neighbor,
                        aggregator.apply(current_node, neighbor, weight),
                    ));
                }
            }
        }
    }

    Ok(SequentialDfsResult {
        visited_nodes: result,
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
    fn traverses_with_depth_first_order() {
        let result = run_sequential_dfs(
            SequentialDfsConfig {
                source_node: 0,
                node_count: 4,
                max_depth: None,
            },
            &OneHopAggregator,
            &FollowExitPredicate,
            |node| match node {
                0 => vec![1, 2],
                1 => vec![3],
                2 => vec![],
                3 => vec![],
                _ => vec![],
            },
        )
        .unwrap();

        assert_eq!(result.visited_nodes, vec![0, 2, 1, 3]);
        assert_eq!(result.relationships_examined, 3);
    }

    #[test]
    fn rejects_out_of_range_source() {
        let err = run_sequential_dfs(
            SequentialDfsConfig {
                source_node: 4,
                node_count: 4,
                max_depth: None,
            },
            &OneHopAggregator,
            &FollowExitPredicate,
            |_| Vec::new(),
        )
        .unwrap_err();

        assert!(matches!(err, AlgorithmError::InvalidGraph(_)));
    }
}
