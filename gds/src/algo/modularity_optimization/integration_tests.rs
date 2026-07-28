use super::spec::ModularityOptimizationConfig;
use super::{ModularityOptimizationComputationRuntime, ModularityOptimizationInput};
use crate::task::concurrency::TerminationFlag;

#[test]
fn modopt_empty_ok() {
    let mut rt = ModularityOptimizationComputationRuntime::new();
    let cfg = ModularityOptimizationConfig::default();
    let input = ModularityOptimizationInput::new(0, Vec::new());
    let result = rt.compute(&input, &cfg);
    assert!(result.communities.is_empty());
    assert_eq!(result.modularity, 0.0);
    assert_eq!(result.node_count, 0);
}

#[test]
fn modopt_separates_isolated_node() {
    // 0--1 connected, 2 isolated.
    let mut rt = ModularityOptimizationComputationRuntime::new();
    let cfg = ModularityOptimizationConfig::default();
    let input = ModularityOptimizationInput::new(3, vec![vec![(1, 1.0)], vec![(0, 1.0)], vec![]]);
    let result = rt.compute(&input, &cfg);

    assert_eq!(result.communities.len(), 3);
    assert_eq!(result.node_count, 3);
    assert_eq!(result.communities[0], result.communities[1]);
    assert_ne!(result.communities[2], result.communities[0]);
}

#[test]
fn modopt_preserves_partition_independent_of_representative_label() {
    let mut rt = ModularityOptimizationComputationRuntime::new();
    let cfg = ModularityOptimizationConfig::default();
    let input = ModularityOptimizationInput::new(3, vec![vec![(1, 1.0)], vec![(0, 1.0)], vec![]]);
    let result = rt.compute(&input, &cfg);

    assert_eq!(normalized_partition(&result.communities), vec![0, 0, 1]);
}

#[test]
fn modopt_k1_schedule_handles_self_loops_consistently() {
    let mut rt = ModularityOptimizationComputationRuntime::new();
    let cfg = ModularityOptimizationConfig::default();
    let input = ModularityOptimizationInput::new(
        3,
        vec![
            vec![(0, 2.0), (1, 2.0)],
            vec![(0, 2.0), (1, 2.0), (2, 2.0)],
            vec![(1, 2.0), (2, 2.0)],
        ],
    );

    let result = rt.compute(&input, &cfg);

    assert_eq!(normalized_partition(&result.communities), vec![0, 0, 0]);
    assert!(result.modularity.abs() < 1e-12);
}

#[test]
fn modopt_self_loop_fixture_is_equivalent_across_concurrency() {
    let input = ModularityOptimizationInput::new(
        3,
        vec![
            vec![(0, 2.0), (1, 2.0)],
            vec![(0, 2.0), (1, 2.0), (2, 2.0)],
            vec![(1, 2.0), (2, 2.0)],
        ],
    );
    let run = |concurrency| {
        let mut runtime = ModularityOptimizationComputationRuntime::new();
        runtime.compute(
            &input,
            &ModularityOptimizationConfig {
                concurrency,
                ..ModularityOptimizationConfig::default()
            },
        )
    };

    let sequential = run(1);
    let parallel = run(4);

    assert_eq!(
        normalized_partition(&sequential.communities),
        normalized_partition(&parallel.communities)
    );
    assert!((sequential.modularity - parallel.modularity).abs() < 1e-12);
}

#[test]
fn modopt_preserves_seeded_communities_without_relationships() {
    let mut rt = ModularityOptimizationComputationRuntime::new();
    let cfg = ModularityOptimizationConfig::default();
    let input = ModularityOptimizationInput::new(3, vec![vec![], vec![], vec![]]);

    let result = rt
        .compute_with_initial_communities(&input, &cfg, Some(&[0, 0, 1]))
        .unwrap();

    assert_eq!(result.communities, vec![0, 0, 1]);
    assert!(result.did_converge);
}

#[test]
#[should_panic(expected = "The execution has been terminated.")]
fn modopt_honors_termination_during_computation() {
    let mut rt = ModularityOptimizationComputationRuntime::new();
    let cfg = ModularityOptimizationConfig::default();
    let input = ModularityOptimizationInput::new(2, vec![vec![(1, 1.0)], vec![(0, 1.0)]]);

    let _ = rt.compute_with_controls(&input, &cfg, None, &TerminationFlag::stop_running(), |_| {});
}

#[test]
fn modopt_stats_include_node_count() {
    let result = super::spec::ModularityOptimizationResult {
        communities: vec![1, 1, 2],
        modularity: 0.25,
        ran_iterations: 2,
        did_converge: true,
        node_count: 3,
        execution_time: std::time::Duration::default(),
    };

    let stats = super::spec::ModularityOptimizationResultBuilder::new(result).stats();

    assert_eq!(stats.node_count, 3);
    assert_eq!(stats.community_count, 2);
    assert_eq!(stats.ran_iterations, 2);
    assert!(stats.did_converge);
}

#[test]
fn modopt_rejects_invalid_config() {
    let cfg = ModularityOptimizationConfig {
        max_iterations: 0,
        ..ModularityOptimizationConfig::default()
    };
    assert!(cfg.validate().is_err());
}

#[test]
fn modopt_k1_scheduler_is_equivalent_across_concurrency() {
    let input = ModularityOptimizationInput::new(
        6,
        vec![
            vec![(1, 3.0), (2, 3.0)],
            vec![(0, 3.0), (2, 3.0)],
            vec![(0, 3.0), (1, 3.0), (3, 0.25)],
            vec![(2, 0.25), (4, 3.0), (5, 3.0)],
            vec![(3, 3.0), (5, 3.0)],
            vec![(3, 3.0), (4, 3.0)],
        ],
    );
    let run = |concurrency| {
        let mut runtime = ModularityOptimizationComputationRuntime::new();
        runtime.compute(
            &input,
            &ModularityOptimizationConfig {
                concurrency,
                ..ModularityOptimizationConfig::default()
            },
        )
    };

    let sequential = run(1);
    let parallel = run(4);

    assert_eq!(
        normalized_partition(&sequential.communities),
        normalized_partition(&parallel.communities)
    );
    assert!((sequential.modularity - parallel.modularity).abs() < 1e-12);
    assert_eq!(sequential.did_converge, parallel.did_converge);
}

#[test]
fn modopt_k1_scheduler_reports_completed_relationship_volume() {
    let input = ModularityOptimizationInput::new(
        4,
        vec![
            vec![(1, 1.0)],
            vec![(0, 1.0), (2, 1.0)],
            vec![(1, 1.0), (3, 1.0)],
            vec![(2, 1.0)],
        ],
    );
    let expected_volume = input.total_relationship_count();
    let mut progress = Vec::new();
    let mut runtime = ModularityOptimizationComputationRuntime::new();

    let result = runtime
        .compute_with_controls(
            &input,
            &ModularityOptimizationConfig {
                concurrency: 4,
                ..ModularityOptimizationConfig::default()
            },
            None,
            &TerminationFlag::running_true(),
            |completed| progress.push(completed),
        )
        .unwrap();

    assert_eq!(progress.len(), result.ran_iterations);
    assert!(progress
        .iter()
        .all(|completed| *completed == expected_volume));
}

fn normalized_partition(communities: &[u64]) -> Vec<usize> {
    let mut dense = std::collections::HashMap::new();
    communities
        .iter()
        .map(|community| {
            let next = dense.len();
            *dense.entry(*community).or_insert(next)
        })
        .collect()
}
