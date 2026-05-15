use super::spec::ApproxMaxKCutConfig;
use super::ApproxMaxKCutComputationRuntime;

#[test]
fn assigns_all_nodes() {
    let config = ApproxMaxKCutConfig {
        k: 3,
        iterations: 5,
        random_seed: 42,
        minimize: false,
        has_relationship_weight_property: false,
        min_community_sizes: vec![0, 0, 0],
        concurrency: 4,
        min_batch_size: 1,
        vns_max_neighborhood_order: 0,
    };
    let runtime = ApproxMaxKCutComputationRuntime::new(config);

    let adjacency = vec![
        vec![(1, 1.0), (2, 1.0)],
        vec![(0, 1.0), (2, 1.0)],
        vec![(0, 1.0), (1, 1.0)],
    ];

    let result = runtime.compute(3, |node| adjacency[node].clone());
    assert_eq!(result.communities.len(), 3);
    for &c in &result.communities {
        assert!(c < 3);
    }
}

#[test]
fn rejects_invalid_config() {
    let mut config = ApproxMaxKCutConfig::default();

    config.k = 1;
    assert!(config.validate().is_err());

    config = ApproxMaxKCutConfig::default();
    config.iterations = 0;
    assert!(config.validate().is_err());

    config = ApproxMaxKCutConfig::default();
    config.concurrency = 0;
    assert!(config.validate().is_err());

    config = ApproxMaxKCutConfig::default();
    config.min_batch_size = 0;
    assert!(config.validate().is_err());

    config = ApproxMaxKCutConfig::default();
    config.min_community_sizes = vec![0];
    assert!(config.validate().is_err());

    config = ApproxMaxKCutConfig::default();
    config.min_community_sizes = vec![3, 2];
    assert!(config.validate_for_node_count(4).is_err());
}

#[test]
fn deterministic_for_seed() {
    let config = ApproxMaxKCutConfig {
        iterations: 4,
        random_seed: 7,
        min_batch_size: 1,
        ..ApproxMaxKCutConfig::default()
    };
    let runtime = ApproxMaxKCutComputationRuntime::new(config);
    let adjacency = vec![
        vec![(1, 1.0), (2, 1.0)],
        vec![(0, 1.0), (2, 1.0)],
        vec![(0, 1.0), (1, 1.0)],
    ];

    let first = runtime.compute(3, |node| adjacency[node].clone());
    let second = runtime.compute(3, |node| adjacency[node].clone());

    assert_eq!(first.communities, second.communities);
    assert_eq!(first.cut_cost, second.cut_cost);
}

#[test]
fn enforces_min_community_sizes() {
    let config = ApproxMaxKCutConfig {
        k: 3,
        iterations: 3,
        random_seed: 13,
        min_community_sizes: vec![1, 1, 1],
        min_batch_size: 1,
        ..ApproxMaxKCutConfig::default()
    };
    let runtime = ApproxMaxKCutComputationRuntime::new(config);
    let adjacency = vec![
        vec![(1, 1.0)],
        vec![(0, 1.0), (2, 1.0)],
        vec![(1, 1.0), (3, 1.0)],
        vec![(2, 1.0)],
    ];

    let result = runtime.compute(4, |node| adjacency[node].clone());
    let mut sizes = [0usize; 3];
    for community in result.communities {
        sizes[community as usize] += 1;
    }

    assert!(sizes.iter().all(|size| *size >= 1));
}

#[test]
fn computes_weighted_cut_cost() {
    let config = ApproxMaxKCutConfig {
        iterations: 3,
        random_seed: 21,
        min_batch_size: 1,
        ..ApproxMaxKCutConfig::default()
    };
    let runtime = ApproxMaxKCutComputationRuntime::new(config);
    let adjacency = vec![vec![(1, 5.0)], vec![(0, 7.0)]];

    let result = runtime.compute(2, |node| adjacency[node].clone());

    assert_eq!(result.cut_cost, 12.0);
}

#[test]
fn vns_config_runs() {
    let config = ApproxMaxKCutConfig {
        iterations: 3,
        random_seed: 42,
        min_batch_size: 1,
        vns_max_neighborhood_order: 2,
        ..ApproxMaxKCutConfig::default()
    };
    let runtime = ApproxMaxKCutComputationRuntime::new(config);
    let adjacency = vec![
        vec![(1, 1.0), (2, 1.0)],
        vec![(0, 1.0), (2, 1.0)],
        vec![(0, 1.0), (1, 1.0)],
    ];

    let result = runtime.compute(3, |node| adjacency[node].clone());

    assert_eq!(result.communities.len(), 3);
    assert!(result.cut_cost >= 0.0);
}
