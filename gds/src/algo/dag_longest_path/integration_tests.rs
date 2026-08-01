#[cfg(test)]
mod tests {
    use crate::algo::dag_longest_path::DagLongestPathComputationRuntime;
    use crate::types::graph::MappedNodeId;

    fn mapped_node_id(value: u64) -> MappedNodeId {
        MappedNodeId::new(value)
    }

    fn physical_node_index(node_id: MappedNodeId) -> usize {
        usize::try_from(node_id.get()).expect("fixture node ID must fit usize")
    }

    #[test]
    fn test_simple_dag() {
        // 0 -> 1 -> 2
        let edges = vec![
            vec![(mapped_node_id(1), 1.0)],
            vec![(mapped_node_id(2), 1.0)],
            vec![],
        ];

        let get_neighbors = move |node| edges[physical_node_index(node)].clone();

        let mut runtime = DagLongestPathComputationRuntime::new(3);
        let result = runtime.compute(3, get_neighbors).unwrap();

        // Should have paths from 0 to all reachable nodes
        assert!(!result.paths.is_empty());

        // Find path to node 2
        let path_to_2 = result
            .paths
            .iter()
            .find(|path| path.target_node == mapped_node_id(2))
            .unwrap();
        assert_eq!(path_to_2.source_node, MappedNodeId::ZERO);
        assert_eq!(path_to_2.total_cost, 2.0);
        assert_eq!(
            path_to_2.node_ids,
            vec![MappedNodeId::ZERO, mapped_node_id(1), mapped_node_id(2)]
        );
    }

    #[test]
    fn test_diamond_dag() {
        // 0 -> 1 (weight 1) -> 3 (weight 1)
        //  \-> 2 (weight 2) ->/  (weight 1)
        let edges = vec![
            vec![(mapped_node_id(1), 1.0), (mapped_node_id(2), 2.0)],
            vec![(mapped_node_id(3), 1.0)],
            vec![(mapped_node_id(3), 1.0)],
            vec![],
        ];

        let get_neighbors = move |node| edges[physical_node_index(node)].clone();

        let mut runtime = DagLongestPathComputationRuntime::new(4);
        let result = runtime.compute(4, get_neighbors).unwrap();

        // Find path to node 3
        let path_to_3 = result
            .paths
            .iter()
            .find(|path| path.target_node == mapped_node_id(3))
            .unwrap();

        // Longest path should be 0->2->3 (cost 3.0)
        assert_eq!(path_to_3.total_cost, 3.0);
        assert_eq!(
            path_to_3.node_ids,
            vec![MappedNodeId::ZERO, mapped_node_id(2), mapped_node_id(3)]
        );
    }

    #[test]
    fn test_weighted_dag() {
        // 0 -> 1 (weight 5) -> 2 (weight 3)
        let edges = vec![
            vec![(mapped_node_id(1), 5.0)],
            vec![(mapped_node_id(2), 3.0)],
            vec![],
        ];

        let get_neighbors = move |node| edges[physical_node_index(node)].clone();

        let mut runtime = DagLongestPathComputationRuntime::new(3);
        let result = runtime.compute(3, get_neighbors).unwrap();

        let path_to_2 = result
            .paths
            .iter()
            .find(|path| path.target_node == mapped_node_id(2))
            .unwrap();
        assert_eq!(path_to_2.total_cost, 8.0);
    }

    #[test]
    fn test_disconnected_dag() {
        // 0 -> 1, 2 -> 3 (two separate chains)
        let edges = vec![
            vec![(mapped_node_id(1), 1.0)],
            vec![],
            vec![(mapped_node_id(3), 1.0)],
            vec![],
        ];

        let get_neighbors = move |node| edges[physical_node_index(node)].clone();

        let mut runtime = DagLongestPathComputationRuntime::new(4);
        let result = runtime.compute(4, get_neighbors).unwrap();

        // Should have paths from both components
        assert!(result.paths.len() >= 2);
    }

    #[test]
    fn test_cycle_nodes_are_ignored_like_java_gds() {
        // 0 -> 1 <-> 2 -> 3. Java GDS emits the source and the first reached cycle-entry
        // predecessor, but the blocked cycle node and nodes reachable only from it are absent.
        let edges = vec![
            vec![(mapped_node_id(1), 1.0)],
            vec![(mapped_node_id(2), 1.0)],
            vec![(mapped_node_id(1), 1.0), (mapped_node_id(3), 1.0)],
            vec![],
        ];

        let get_neighbors = move |node| edges[physical_node_index(node)].clone();

        let mut runtime = DagLongestPathComputationRuntime::new(4);
        let result = runtime.compute(4, get_neighbors).unwrap();

        let targets: Vec<MappedNodeId> = result.paths.iter().map(|path| path.target_node).collect();
        assert_eq!(targets, vec![MappedNodeId::ZERO, mapped_node_id(1)]);
        assert!(result
            .paths
            .iter()
            .all(|path| path.target_node != mapped_node_id(2)));
        assert!(result
            .paths
            .iter()
            .all(|path| path.target_node != mapped_node_id(3)));
    }

    #[test]
    fn test_rejects_invalid_neighbor_and_weight() {
        let bad_neighbor = vec![vec![(mapped_node_id(3), 1.0)], vec![], vec![]];
        let mut runtime = DagLongestPathComputationRuntime::new(3);
        assert!(runtime
            .compute(3, move |node| bad_neighbor[physical_node_index(node)]
                .clone())
            .is_err());

        let bad_weight = vec![vec![(mapped_node_id(1), f64::NAN)], vec![]];
        let mut runtime = DagLongestPathComputationRuntime::new(2);
        assert!(runtime
            .compute(2, move |node| bad_weight[physical_node_index(node)].clone())
            .is_err());
    }
}
