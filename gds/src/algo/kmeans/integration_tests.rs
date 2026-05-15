use super::spec::{KMeansConfig, KMeansSamplerType};
use super::KMeansComputationRuntime;

#[test]
fn kmeans_two_clusters_converges() {
    // Two clearly separated clusters in 2D.
    let mut points: Vec<Vec<f64>> = Vec::new();
    for i in 0..10 {
        points.push(vec![0.0 + i as f64 * 0.01, 0.0]);
    }
    for i in 0..10 {
        points.push(vec![10.0 + i as f64 * 0.01, 10.0]);
    }

    let config = KMeansConfig {
        k: 2,
        max_iterations: 50,
        delta_threshold: 0.0,
        number_of_restarts: 3,
        compute_silhouette: true,
        concurrency: 4,
        node_property: "p".to_string(),
        sampler_type: KMeansSamplerType::KmeansPlusPlus,
        seed_centroids: Vec::new(),
        random_seed: Some(42),
    };

    let mut runtime = KMeansComputationRuntime::new();
    let result = runtime.compute(&points, &config);

    assert_eq!(result.communities.len(), points.len());
    assert_eq!(result.distance_from_center.len(), points.len());
    assert_eq!(result.centers.len(), 2);
    assert!(result.ran_iterations >= 1);
    assert!(result.average_distance_to_centroid.is_finite());
    assert!(result.silhouette.is_some());
}

#[test]
fn kmeans_seeded_centroids_respected() {
    let points = vec![
        vec![0.0, 0.0],
        vec![0.1, 0.0],
        vec![10.0, 10.0],
        vec![10.1, 10.0],
    ];

    let config = KMeansConfig {
        k: 2,
        max_iterations: 10,
        delta_threshold: 0.0,
        number_of_restarts: 1,
        compute_silhouette: false,
        concurrency: 2,
        node_property: "p".to_string(),
        sampler_type: KMeansSamplerType::Uniform,
        seed_centroids: vec![vec![0.0, 0.0], vec![10.0, 10.0]],
        random_seed: None,
    };

    let mut runtime = KMeansComputationRuntime::new();
    let result = runtime.compute(&points, &config);

    assert_eq!(result.centers.len(), 2);
    // The two ends should likely be split.
    assert_ne!(result.communities[0], result.communities[2]);
}

#[test]
fn kmeans_rejects_invalid_config() {
    let mut config = KMeansConfig::default();
    config.node_property = "p".to_string();

    config.k = 0;
    assert!(config.validate().is_err());

    config = KMeansConfig::default();
    config.node_property = "p".to_string();
    config.max_iterations = 0;
    assert!(config.validate().is_err());

    config = KMeansConfig::default();
    config.node_property = "p".to_string();
    config.delta_threshold = 1.1;
    assert!(config.validate().is_err());

    config = KMeansConfig::default();
    config.node_property = "p".to_string();
    config.number_of_restarts = 0;
    assert!(config.validate().is_err());

    config = KMeansConfig::default();
    config.node_property = "p".to_string();
    config.concurrency = 0;
    assert!(config.validate().is_err());

    config = KMeansConfig::default();
    config.node_property.clear();
    assert!(config.validate().is_err());
}

#[test]
fn kmeans_more_clusters_than_nodes_returns_node_centers() {
    let points = vec![vec![0.0, 0.0], vec![10.0, 10.0]];
    let config = KMeansConfig {
        k: 4,
        max_iterations: 10,
        delta_threshold: 0.0,
        number_of_restarts: 1,
        compute_silhouette: false,
        concurrency: 2,
        node_property: "p".to_string(),
        sampler_type: KMeansSamplerType::KmeansPlusPlus,
        seed_centroids: Vec::new(),
        random_seed: Some(42),
    };

    let mut runtime = KMeansComputationRuntime::new();
    let result = runtime.compute(&points, &config);

    assert_eq!(result.centers.len(), points.len());
    assert_eq!(result.distance_from_center, vec![0.0, 0.0]);
}

#[test]
fn kmeans_restarts_report_configured_count() {
    let points = vec![
        vec![0.0, 0.0],
        vec![0.1, 0.0],
        vec![10.0, 10.0],
        vec![10.1, 10.0],
    ];
    let config = KMeansConfig {
        k: 2,
        max_iterations: 10,
        delta_threshold: 0.0,
        number_of_restarts: 3,
        compute_silhouette: false,
        concurrency: 2,
        node_property: "p".to_string(),
        sampler_type: KMeansSamplerType::Uniform,
        seed_centroids: Vec::new(),
        random_seed: Some(7),
    };

    let mut runtime = KMeansComputationRuntime::new();
    let result = runtime.compute(&points, &config);

    assert_eq!(result.restarts, 3);
    assert_eq!(result.communities.len(), points.len());
}
