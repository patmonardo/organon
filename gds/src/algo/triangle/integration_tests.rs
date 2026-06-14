use super::TriangleComputationRuntime;
use crate::task::concurrency::TerminationFlag;
use crate::task::progress::NoopProgressTracker;

#[test]
fn triangle_empty() {
    let mut rt = TriangleComputationRuntime::new();
    let result = rt.compute(0, |_| Vec::new());
    assert_eq!(result.global_triangles, 0);
    assert!(result.local_triangles.is_empty());
}

#[test]
fn triangle_single_triangle() {
    // 0-1-2-0
    let mut rt = TriangleComputationRuntime::new();
    let neighbors = |n: usize| match n {
        0 => vec![1, 2],
        1 => vec![0, 2],
        2 => vec![0, 1],
        _ => vec![],
    };
    let result = rt.compute(3, neighbors);

    assert_eq!(result.global_triangles, 1);
    assert_eq!(result.local_triangles, vec![1, 1, 1]);
}

#[test]
fn triangle_two_share_edge() {
    // Triangles: (0,1,2) and (0,1,3)
    let mut rt = TriangleComputationRuntime::new();
    let neighbors = |n: usize| match n {
        0 => vec![1, 2, 3],
        1 => vec![0, 2, 3],
        2 => vec![0, 1],
        3 => vec![0, 1],
        _ => vec![],
    };
    let result = rt.compute(4, neighbors);

    assert_eq!(result.global_triangles, 2);
    assert_eq!(result.local_triangles, vec![2, 2, 1, 1]);
}

#[test]
fn triangle_parallel_matches_single_threaded() {
    // Triangles: (0,1,2), (0,1,3), (0,2,3), (1,2,3), and (3,4,5)
    let neighbors = |n: usize| match n {
        0 => vec![1, 2, 3],
        1 => vec![0, 2, 3],
        2 => vec![0, 1, 3],
        3 => vec![0, 1, 2, 4, 5],
        4 => vec![3, 5],
        5 => vec![3, 4],
        _ => vec![],
    };

    let mut rt = TriangleComputationRuntime::new();
    let mut progress_tracker = NoopProgressTracker;
    let termination_flag = TerminationFlag::default();
    let single = rt
        .compute_with_controls(
            6,
            neighbors,
            u64::MAX,
            1,
            &mut progress_tracker,
            &termination_flag,
        )
        .unwrap();

    let mut rt = TriangleComputationRuntime::new();
    let mut progress_tracker = NoopProgressTracker;
    let parallel = rt
        .compute_with_controls(
            6,
            neighbors,
            u64::MAX,
            4,
            &mut progress_tracker,
            &termination_flag,
        )
        .unwrap();

    assert_eq!(parallel.global_triangles, single.global_triangles);
    assert_eq!(parallel.local_triangles, single.local_triangles);
    assert_eq!(parallel.global_triangles, 5);
    assert_eq!(parallel.local_triangles, vec![3, 3, 3, 4, 1, 1]);
}

#[test]
fn triangle_max_degree_marks_excluded_nodes() {
    let mut rt = TriangleComputationRuntime::new();
    let neighbors = |n: usize| match n {
        0 => vec![1, 2, 3],
        1 => vec![0, 2, 3],
        2 => vec![0, 1],
        3 => vec![0, 1],
        _ => vec![],
    };
    let result = rt.compute_with_max_degree(4, neighbors, 2);

    assert_eq!(result.global_triangles, 0);
    assert_eq!(result.local_triangles, vec![-1, -1, 0, 0]);
}

#[test]
fn triangle_stats_include_node_count() {
    let mut rt = TriangleComputationRuntime::new();
    let neighbors = |n: usize| match n {
        0 => vec![1, 2],
        1 => vec![0, 2],
        2 => vec![0, 1],
        _ => vec![],
    };
    let result = rt.compute(3, neighbors);

    let stats = super::TriangleResultBuilder::new(result).stats();
    assert_eq!(stats.node_count, 3);
    assert_eq!(stats.global_triangles, 1);
}
