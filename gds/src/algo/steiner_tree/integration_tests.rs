use crate::algo::steiner_tree::{
    SteinerTreeComputationRuntime, SteinerTreeConfig, SteinerTreeParent, SteinerTreeStorageRuntime,
};
use crate::task::progress::{TaskProgressTracker, Tasks};
use crate::types::graph::MappedNodeId;

fn node(value: u64) -> MappedNodeId {
    MappedNodeId::new(value)
}

fn parent(value: u64) -> SteinerTreeParent {
    SteinerTreeParent::Parent(node(value))
}

fn create_neighbors(
    edges: Vec<Vec<(usize, f64)>>,
) -> impl Fn(MappedNodeId) -> Vec<(MappedNodeId, f64)> {
    move |node: MappedNodeId| {
        let Some(node_index) = node.to_usize() else {
            return Vec::new();
        };
        if node_index < edges.len() {
            edges[node_index]
                .iter()
                .map(|(target, weight)| {
                    (
                        MappedNodeId::try_from(*target)
                            .expect("test target must fit mapped ID space"),
                        *weight,
                    )
                })
                .collect()
        } else {
            Vec::new()
        }
    }
}

#[test]
fn test_steiner_tree_simple_path() {
    // Linear graph: 0 -> 1 -> 2 -> 3
    // Source: 0, Terminals: [3]
    // Should find path 0 -> 1 -> 2 -> 3
    let edges = vec![vec![(1, 1.0)], vec![(2, 1.0)], vec![(3, 1.0)], vec![]];

    let get_neighbors = create_neighbors(edges);
    let config = SteinerTreeConfig {
        source_node: node(0),
        target_nodes: vec![node(3)],
        relationship_weight_property: Some("weight".to_string()),
        delta: 1.0,
        apply_rerouting: true,
    };

    let storage = SteinerTreeStorageRuntime::new(config, 1);
    let mut computation = SteinerTreeComputationRuntime::new(1.0, 4);
    let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("steiner_tree".to_string()));
    let result = storage
        .compute_steiner_tree_with_neighbors(
            &mut computation,
            4,
            &get_neighbors,
            &mut progress_tracker,
        )
        .unwrap();

    // Check path: 0 is root, 1->0, 2->1, 3->2
    assert_eq!(result.parent_array[0], SteinerTreeParent::Root);
    assert_eq!(result.parent_array[1], parent(0));
    assert_eq!(result.parent_array[2], parent(1));
    assert_eq!(result.parent_array[3], parent(2));

    // Total cost: 3 edges of weight 1.0
    assert!((result.total_cost - 3.0).abs() < 0.01);

    // 4 nodes in tree
    assert_eq!(result.effective_node_count, 4);
    assert_eq!(result.effective_target_nodes_count, 1);
}

#[test]
fn test_steiner_tree_multiple_terminals() {
    // Tree structure:
    //     0
    //    / \
    //   1   2
    //  /     \
    // 3       4
    // Source: 0, Terminals: [3, 4]
    let edges = vec![
        vec![(1, 1.0), (2, 1.0)],
        vec![(3, 1.0)],
        vec![(4, 1.0)],
        vec![],
        vec![],
    ];

    let get_neighbors = create_neighbors(edges);
    let config = SteinerTreeConfig {
        source_node: node(0),
        target_nodes: vec![node(3), node(4)],
        relationship_weight_property: Some("weight".to_string()),
        delta: 1.0,
        apply_rerouting: true,
    };

    let storage = SteinerTreeStorageRuntime::new(config, 1);
    let mut computation = SteinerTreeComputationRuntime::new(1.0, 5);
    let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("steiner_tree".to_string()));
    let result = storage
        .compute_steiner_tree_with_neighbors(
            &mut computation,
            5,
            &get_neighbors,
            &mut progress_tracker,
        )
        .unwrap();

    // All nodes should be in tree
    assert_eq!(result.parent_array[0], SteinerTreeParent::Root);
    assert_eq!(result.parent_array[1], parent(0));
    assert_eq!(result.parent_array[2], parent(0));
    assert_eq!(result.parent_array[3], parent(1));
    assert_eq!(result.parent_array[4], parent(2));

    // Total cost: 4 edges
    assert!((result.total_cost - 4.0).abs() < 0.01);

    assert_eq!(result.effective_node_count, 5);
    assert_eq!(result.effective_target_nodes_count, 2);
}

#[test]
fn test_steiner_tree_with_pruning() {
    // Graph with extra branch:
    //     0
    //    /|\
    //   1 2 3
    //  /
    // 4
    // Source: 0, Terminals: [2, 4]
    // Node 3 should be pruned (not on path to any terminal)
    let edges = vec![
        vec![(1, 1.0), (2, 1.0), (3, 1.0)],
        vec![(4, 1.0)],
        vec![],
        vec![],
        vec![],
    ];

    let get_neighbors = create_neighbors(edges);
    let config = SteinerTreeConfig {
        source_node: node(0),
        target_nodes: vec![node(2), node(4)],
        relationship_weight_property: Some("weight".to_string()),
        delta: 1.0,
        apply_rerouting: true,
    };

    let storage = SteinerTreeStorageRuntime::new(config, 1);
    let mut computation = SteinerTreeComputationRuntime::new(1.0, 5);
    let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("steiner_tree".to_string()));
    let result = storage
        .compute_steiner_tree_with_neighbors(
            &mut computation,
            5,
            &get_neighbors,
            &mut progress_tracker,
        )
        .unwrap();

    // Check tree structure
    assert_eq!(result.parent_array[0], SteinerTreeParent::Root);
    assert_eq!(result.parent_array[1], parent(0));
    assert_eq!(result.parent_array[2], parent(0));
    assert_eq!(result.parent_array[3], SteinerTreeParent::Pruned);
    assert_eq!(result.parent_array[4], parent(1));

    // Total cost: 3 edges (0->1, 0->2, 1->4)
    assert!((result.total_cost - 3.0).abs() < 0.01);

    assert_eq!(result.effective_node_count, 4); // 0, 1, 2, 4
    assert_eq!(result.effective_target_nodes_count, 2);
}

#[test]
fn test_steiner_tree_weighted_edges() {
    // Diamond graph with different weights:
    //     0
    //    / \
    //   1   2  (weights: 0->1: 5.0, 0->2: 1.0)
    //    \ /
    //     3    (weights: 1->3: 1.0, 2->3: 1.0)
    // Source: 0, Terminals: [3]
    // Should prefer path 0 -> 2 -> 3 (cost 2.0) over 0 -> 1 -> 3 (cost 6.0)
    let edges = vec![
        vec![(1, 5.0), (2, 1.0)],
        vec![(3, 1.0)],
        vec![(3, 1.0)],
        vec![],
    ];

    let get_neighbors = create_neighbors(edges);
    let config = SteinerTreeConfig {
        source_node: node(0),
        target_nodes: vec![node(3)],
        relationship_weight_property: Some("weight".to_string()),
        delta: 1.0,
        apply_rerouting: true,
    };

    let storage = SteinerTreeStorageRuntime::new(config, 1);
    let mut computation = SteinerTreeComputationRuntime::new(1.0, 4);
    let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("steiner_tree".to_string()));
    let result = storage
        .compute_steiner_tree_with_neighbors(
            &mut computation,
            4,
            &get_neighbors,
            &mut progress_tracker,
        )
        .unwrap();

    // Should take cheaper path through node 2
    assert_eq!(result.parent_array[0], SteinerTreeParent::Root);
    assert_eq!(result.parent_array[2], parent(0));
    assert_eq!(result.parent_array[3], parent(2));
    assert_eq!(result.parent_array[1], SteinerTreeParent::Pruned);

    // Total cost: 0->2: 1.0, 2->3: 1.0
    assert!((result.total_cost - 2.0).abs() < 0.01);

    assert_eq!(result.effective_node_count, 3); // 0, 2, 3
    assert_eq!(result.effective_target_nodes_count, 1);
}

#[test]
fn test_steiner_tree_pruning_is_always_applied() {
    // Same as pruning test but with apply_rerouting disabled.
    // We still prune non-terminal leaves because pruning is part of Steiner semantics;
    // apply_rerouting controls optional post-optimizations.
    let edges = vec![
        vec![(1, 1.0), (2, 1.0), (3, 1.0)],
        vec![(4, 1.0)],
        vec![],
        vec![],
        vec![],
    ];

    let get_neighbors = create_neighbors(edges);
    let config = SteinerTreeConfig {
        source_node: node(0),
        target_nodes: vec![node(2), node(4)],
        relationship_weight_property: Some("weight".to_string()),
        delta: 1.0,
        apply_rerouting: false, // Pruning disabled
    };

    let storage = SteinerTreeStorageRuntime::new(config, 1);
    let mut computation = SteinerTreeComputationRuntime::new(1.0, 5);
    let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("steiner_tree".to_string()));
    let result = storage
        .compute_steiner_tree_with_neighbors(
            &mut computation,
            5,
            &get_neighbors,
            &mut progress_tracker,
        )
        .unwrap();

    // Node 3 should be pruned (not on any terminal path)
    assert_eq!(result.parent_array[0], SteinerTreeParent::Root);
    assert_eq!(result.parent_array[1], parent(0));
    assert_eq!(result.parent_array[2], parent(0));
    assert_eq!(result.parent_array[3], SteinerTreeParent::Pruned);
    assert_eq!(result.parent_array[4], parent(1));

    // Total cost: 3 edges (0->1, 0->2, 1->4)
    assert!((result.total_cost - 3.0).abs() < 0.01);

    assert_eq!(result.effective_node_count, 4);
    assert_eq!(result.effective_target_nodes_count, 2);
}

#[test]
fn test_steiner_tree_unreachable_terminal() {
    // Disconnected graph:
    // 0 -> 1
    // 2 (isolated)
    // Source: 0, Terminals: [1, 2]
    let edges = vec![
        vec![(1, 1.0)],
        vec![],
        vec![], // Isolated
    ];

    let get_neighbors = create_neighbors(edges);
    let config = SteinerTreeConfig {
        source_node: node(0),
        target_nodes: vec![node(1), node(2)],
        relationship_weight_property: Some("weight".to_string()),
        delta: 1.0,
        apply_rerouting: true,
    };

    let storage = SteinerTreeStorageRuntime::new(config, 1);
    let mut computation = SteinerTreeComputationRuntime::new(1.0, 3);
    let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("steiner_tree".to_string()));
    let result = storage
        .compute_steiner_tree_with_neighbors(
            &mut computation,
            3,
            &get_neighbors,
            &mut progress_tracker,
        )
        .unwrap();

    // Node 1 reachable, node 2 not
    assert_eq!(result.parent_array[0], SteinerTreeParent::Root);
    assert_eq!(result.parent_array[1], parent(0));
    assert_eq!(result.parent_array[2], SteinerTreeParent::Pruned);

    assert_eq!(result.effective_node_count, 2); // 0, 1
    assert_eq!(result.effective_target_nodes_count, 1); // Only node 1
}

#[test]
fn test_steiner_tree_delta_must_be_positive() {
    let edges = vec![vec![(1, 1.0)], vec![]];
    let get_neighbors = create_neighbors(edges);
    let config = SteinerTreeConfig {
        source_node: node(0),
        target_nodes: vec![node(1)],
        relationship_weight_property: None,
        delta: 0.0,
        apply_rerouting: true,
    };

    let storage = SteinerTreeStorageRuntime::new(config, 1);
    let mut computation = SteinerTreeComputationRuntime::new(0.0, 2);
    let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("steiner_tree".to_string()));
    let out = storage.compute_steiner_tree_with_neighbors(
        &mut computation,
        2,
        &get_neighbors,
        &mut progress_tracker,
    );

    assert!(out.is_err());
}

#[test]
fn test_steiner_tree_target_nodes_must_not_be_empty() {
    let edges = vec![vec![(1, 1.0)], vec![]];
    let get_neighbors = create_neighbors(edges);
    let config = SteinerTreeConfig {
        source_node: node(0),
        target_nodes: vec![],
        relationship_weight_property: None,
        delta: 1.0,
        apply_rerouting: true,
    };

    let storage = SteinerTreeStorageRuntime::new(config, 1);
    let mut computation = SteinerTreeComputationRuntime::new(1.0, 2);
    let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("steiner_tree".to_string()));
    let out = storage.compute_steiner_tree_with_neighbors(
        &mut computation,
        2,
        &get_neighbors,
        &mut progress_tracker,
    );

    assert!(out.is_err());
}

#[test]
fn test_steiner_tree_source_node_out_of_bounds_errors() {
    let edges = vec![vec![(1, 1.0)], vec![]];
    let get_neighbors = create_neighbors(edges);
    let config = SteinerTreeConfig {
        source_node: node(9),
        target_nodes: vec![node(1)],
        relationship_weight_property: None,
        delta: 1.0,
        apply_rerouting: true,
    };

    let storage = SteinerTreeStorageRuntime::new(config, 1);
    let mut computation = SteinerTreeComputationRuntime::new(1.0, 2);
    let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("steiner_tree".to_string()));
    let out = storage.compute_steiner_tree_with_neighbors(
        &mut computation,
        2,
        &get_neighbors,
        &mut progress_tracker,
    );

    assert!(out.is_err());
}

#[test]
fn test_steiner_tree_target_node_out_of_bounds_errors() {
    let edges = vec![vec![(1, 1.0)], vec![]];
    let get_neighbors = create_neighbors(edges);
    let config = SteinerTreeConfig {
        source_node: node(0),
        target_nodes: vec![node(9)],
        relationship_weight_property: None,
        delta: 1.0,
        apply_rerouting: true,
    };

    let storage = SteinerTreeStorageRuntime::new(config, 1);
    let mut computation = SteinerTreeComputationRuntime::new(1.0, 2);
    let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("steiner_tree".to_string()));
    let out = storage.compute_steiner_tree_with_neighbors(
        &mut computation,
        2,
        &get_neighbors,
        &mut progress_tracker,
    );

    assert!(out.is_err());
}
