use super::spec::ModularityOptimizationConfig;
use super::{ModularityOptimizationComputationRuntime, ModularityOptimizationInput};

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
fn modopt_preserves_raw_community_labels() {
    let mut rt = ModularityOptimizationComputationRuntime::new();
    let cfg = ModularityOptimizationConfig::default();
    let input = ModularityOptimizationInput::new(3, vec![vec![(1, 1.0)], vec![(0, 1.0)], vec![]]);
    let result = rt.compute(&input, &cfg);

    assert_eq!(result.communities, vec![1, 1, 2]);
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
