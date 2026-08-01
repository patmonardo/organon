use super::{Aggregator, ExitPredicate, ExitPredicateResult};
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::graph::MappedNodeId;

#[derive(Debug, Clone, Copy)]
pub struct SequentialDfsConfig {
    pub source_node: MappedNodeId,
    pub node_count: usize,
    pub max_depth: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct SequentialDfsResult {
    pub visited_nodes: Vec<MappedNodeId>,
    pub visited_depths: Vec<f64>,
    pub relationships_examined: usize,
}

pub fn run_sequential_dfs<F>(
    config: SequentialDfsConfig,
    aggregator: &dyn Aggregator,
    exit_predicate: &dyn ExitPredicate,
    get_neighbors: F,
) -> Result<SequentialDfsResult, AlgorithmError>
where
    F: Fn(MappedNodeId) -> Vec<MappedNodeId>,
{
    validate_node_in_graph(config.source_node, config.node_count, "source")?;

    let mut visited = vec![false; config.node_count];
    let source_idx = node_index_in_graph(config.source_node, "source")?;
    visited[source_idx] = true;

    let mut stack = vec![(config.source_node, config.source_node, 0.0)];
    let mut result = Vec::new();
    let mut result_depths = Vec::new();
    let mut relationships_examined = 0usize;

    while let Some((source_node, current_node, weight)) = stack.pop() {
        match exit_predicate.test(source_node, current_node, weight) {
            ExitPredicateResult::Continue => continue,
            ExitPredicateResult::Break => {
                result.push(current_node);
                result_depths.push(weight);
                break;
            }
            ExitPredicateResult::Follow => {
                result.push(current_node);
                result_depths.push(weight);
            }
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
    node_id: MappedNodeId,
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

fn node_index_in_graph(node_id: MappedNodeId, role: &str) -> Result<usize, AlgorithmError> {
    usize::try_from(node_id)
        .map_err(|_| AlgorithmError::InvalidGraph(format!("Invalid {role} node id: {node_id}")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algo::traversal::{FollowExitPredicate, OneHopAggregator, TargetExitPredicate};
    use std::sync::Mutex;

    fn mapped(node_id: u64) -> MappedNodeId {
        MappedNodeId::new(node_id)
    }

    fn neighbors(node: MappedNodeId) -> Vec<MappedNodeId> {
        match node {
            node if node == mapped(0) => vec![mapped(1), mapped(2)],
            node if node == mapped(1) => vec![mapped(3)],
            _ => vec![],
        }
    }

    #[derive(Default)]
    struct RecordingExitPredicate {
        visits: Mutex<Vec<(Option<MappedNodeId>, MappedNodeId)>>,
    }

    impl ExitPredicate for RecordingExitPredicate {
        fn test(
            &self,
            source_node: MappedNodeId,
            current_node: MappedNodeId,
            _weight_at_source: f64,
        ) -> ExitPredicateResult {
            let parent = (source_node != current_node).then_some(source_node);
            self.visits.lock().unwrap().push((parent, current_node));
            ExitPredicateResult::Follow
        }
    }

    #[test]
    fn traverses_with_depth_first_order() {
        let exit_predicate = RecordingExitPredicate::default();
        let result = run_sequential_dfs(
            SequentialDfsConfig {
                source_node: mapped(0),
                node_count: 4,
                max_depth: None,
            },
            &OneHopAggregator,
            &exit_predicate,
            neighbors,
        )
        .unwrap();

        assert_eq!(
            result.visited_nodes,
            vec![mapped(0), mapped(2), mapped(1), mapped(3)]
        );
        assert_eq!(result.visited_depths, vec![0.0, 1.0, 1.0, 2.0]);
        assert_eq!(result.relationships_examined, 3);
        assert_eq!(
            *exit_predicate.visits.lock().unwrap(),
            vec![
                (None, mapped(0)),
                (Some(mapped(0)), mapped(2)),
                (Some(mapped(0)), mapped(1)),
                (Some(mapped(1)), mapped(3)),
            ]
        );
    }

    #[test]
    fn target_exit_stops_before_remaining_stack_entries() {
        let result = run_sequential_dfs(
            SequentialDfsConfig {
                source_node: mapped(0),
                node_count: 4,
                max_depth: None,
            },
            &OneHopAggregator,
            &TargetExitPredicate::new(vec![mapped(2)]),
            neighbors,
        )
        .unwrap();

        assert_eq!(result.visited_nodes, vec![mapped(0), mapped(2)]);
        assert_eq!(result.visited_depths, vec![0.0, 1.0]);
        assert_eq!(result.relationships_examined, 2);
    }

    #[test]
    fn rejects_out_of_range_source() {
        let err = run_sequential_dfs(
            SequentialDfsConfig {
                source_node: mapped(4),
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
