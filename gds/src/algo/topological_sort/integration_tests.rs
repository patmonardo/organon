#[cfg(test)]
mod tests {
    use crate::algo::topological_sort::TopologicalSortComputationRuntime;
    use crate::types::graph::MappedNodeId;

    fn mapped(node_id: u64) -> MappedNodeId {
        MappedNodeId::new(node_id)
    }

    fn index(node_id: MappedNodeId) -> usize {
        usize::try_from(node_id).expect("test node id must fit a physical index")
    }

    #[test]
    fn test_simple_dag() {
        // 0 -> 1 -> 2
        let edges = vec![vec![(mapped(1), 1.0)], vec![(mapped(2), 1.0)], vec![]];

        let get_neighbors = |node: MappedNodeId| edges[index(node)].clone();

        let mut runtime = TopologicalSortComputationRuntime::new(3, false);
        let result = runtime.compute(3, get_neighbors).unwrap();

        assert_eq!(result.sorted_nodes.len(), 3);
        assert_eq!(result.sorted_nodes, vec![mapped(0), mapped(1), mapped(2)]);
        assert_eq!(result.max_source_distances, None);
    }

    #[test]
    fn test_with_max_distance() {
        // 0 -> 1 (weight 2) -> 2 (weight 3)
        let edges = vec![vec![(mapped(1), 2.0)], vec![(mapped(2), 3.0)], vec![]];

        let get_neighbors = |node: MappedNodeId| edges[index(node)].clone();

        let mut runtime = TopologicalSortComputationRuntime::new(3, true);
        let result = runtime.compute(3, get_neighbors).unwrap();

        assert_eq!(result.sorted_nodes, vec![mapped(0), mapped(1), mapped(2)]);
        let distances = result.max_source_distances.unwrap();
        assert_eq!(distances[0], 0.0);
        assert_eq!(distances[1], 2.0);
        assert_eq!(distances[2], 5.0);
    }

    #[test]
    fn test_diamond_dag() {
        // 0 -> 1 -> 3
        //  \-> 2 ->/
        let edges: Vec<Vec<(MappedNodeId, f64)>> = vec![
            vec![(mapped(1), 1.0), (mapped(2), 2.0)],
            vec![(mapped(3), 1.0)],
            vec![(mapped(3), 1.0)],
            vec![],
        ];

        let get_neighbors = |node: MappedNodeId| edges[index(node)].clone();

        let mut runtime = TopologicalSortComputationRuntime::new(4, true);
        let result = runtime.compute(4, get_neighbors).unwrap();

        assert_eq!(result.sorted_nodes.len(), 4);
        assert_eq!(result.sorted_nodes[0], mapped(0));
        // Longest path to node 3 should be through node 2
        let distances = result.max_source_distances.unwrap();
        assert_eq!(distances[3], 3.0); // 0->2->3
    }

    #[test]
    fn test_disconnected_graph() {
        // 0 -> 1, 2 -> 3 (two separate chains)
        let edges = vec![
            vec![(mapped(1), 1.0)],
            vec![],
            vec![(mapped(3), 1.0)],
            vec![],
        ];

        let get_neighbors = |node: MappedNodeId| edges[index(node)].clone();

        let mut runtime = TopologicalSortComputationRuntime::new(4, false);
        let result = runtime.compute(4, get_neighbors).unwrap();

        assert_eq!(result.sorted_nodes.len(), 4);
        assert_eq!(result.max_source_distances, None);
        // Should include all nodes from both components
    }

    #[test]
    fn test_cycle_nodes_are_ignored_like_java_gds() {
        // 0 -> 1 <-> 2 -> 3; only 0 can be sorted because the cycle blocks 1, 2, and 3.
        let edges: Vec<Vec<(MappedNodeId, f64)>> = vec![
            vec![(mapped(1), 1.0)],
            vec![(mapped(2), 1.0)],
            vec![(mapped(1), 1.0), (mapped(3), 1.0)],
            vec![],
        ];

        let get_neighbors = |node: MappedNodeId| edges[index(node)].clone();

        let mut runtime = TopologicalSortComputationRuntime::new(4, false);
        let result = runtime.compute(4, get_neighbors).unwrap();

        assert_eq!(result.sorted_nodes, vec![mapped(0)]);
    }

    #[test]
    fn test_rejects_out_of_range_neighbor() {
        let edges = vec![vec![(mapped(3), 1.0)], vec![], vec![]];
        let get_neighbors = |node: MappedNodeId| edges[index(node)].clone();

        let mut runtime = TopologicalSortComputationRuntime::new(3, false);
        assert!(runtime.compute(3, get_neighbors).is_err());
    }

    #[test]
    fn test_runtime_resets_between_computations() {
        let first_edges = vec![vec![(mapped(1), 1.0)], vec![]];
        let second_edges = vec![vec![], vec![(mapped(0), 1.0)]];

        let mut runtime = TopologicalSortComputationRuntime::new(2, false);
        let first = runtime
            .compute(2, |node| first_edges[index(node)].clone())
            .unwrap();
        let second = runtime
            .compute(2, |node| second_edges[index(node)].clone())
            .unwrap();

        assert_eq!(first.sorted_nodes, vec![mapped(0), mapped(1)]);
        assert_eq!(second.sorted_nodes, vec![mapped(1), mapped(0)]);
        assert_eq!(first.max_source_distances, None);
        assert_eq!(second.max_source_distances, None);
    }
}
