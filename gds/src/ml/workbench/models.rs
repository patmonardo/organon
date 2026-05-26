use crate::collections::HugeDoubleArray;
use crate::collections::HugeIntArray;
use crate::collections::HugeLongArray;
use crate::concurrency::Concurrency;
use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::NoopProgressTracker;
use crate::core::utils::progress::TaskProgressTracker;
use crate::ml::gradient_descent::GradientDescentConfig;
use crate::ml::metrics::Accuracy;
use crate::ml::metrics::ClassificationMetric;
use crate::ml::metrics::F1Score;
use crate::ml::metrics::GlobalAccuracy;
use crate::ml::metrics::Precision;
use crate::ml::metrics::Recall;
use crate::ml::metrics::RegressionMetric;
use crate::ml::models::features::DenseFeatures;
use crate::ml::models::linear_regression::LinearRegressionTrainConfig;
use crate::ml::models::linear_regression::LinearRegressionTrainer;
use crate::ml::models::logistic_regression::LogisticRegressionTrainConfig;
use crate::ml::models::logistic_regression::LogisticRegressionTrainer;
use crate::ml::models::mlp::MLPClassifierTrainConfig;
use crate::ml::models::mlp::MLPClassifierTrainer;
use crate::ml::models::ClassifierTrainer;
use crate::ml::models::Features;
use crate::ml::models::RegressorTrainer;
use crate::ml::node_classification::ClassificationMetricComputer;
use crate::ml::node_classification::NodeClassificationPredict;
use crate::ml::node_prediction::NodeRegressionPredict;
use crate::ml::node_prediction::NodeSplitter;
use parking_lot::RwLock;
use serde::Deserialize;
use serde::Serialize;
use std::sync::Arc;
use std::time::Instant;

const LOGISTIC_SEPARATED_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/fixtures/ml-models/logistic-separated.json"
));
const LOGISTIC_THREE_CLASS_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/fixtures/ml-models/logistic-three-class.json"
));
const LOGISTIC_THREE_CLASS_OVERLAP_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/fixtures/ml-models/logistic-three-class-overlap.json"
));
const LOGISTIC_THREE_CLASS_LARGE_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/fixtures/ml-models/logistic-three-class-large.json"
));
const LINEAR_AFFINE_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/fixtures/ml-models/linear-affine.json"
));
const LOGISTIC_SEPARATED_LARGE_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/fixtures/ml-models/logistic-separated-large.json"
));
const LINEAR_AFFINE_LARGE_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/fixtures/ml-models/linear-affine-large.json"
));
const SEMANTIC_LOGISTIC_PROTOTYPE_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/fixtures/ml-models/semantic-logistic-prototype.json"
));

#[derive(Clone, Debug, Serialize)]
pub struct FixtureSummary {
    pub id: &'static str,
    pub name: &'static str,
    pub task: &'static str,
    pub source_path: &'static str,
}

#[derive(Clone, Debug, Serialize)]
pub struct NodeClassificationMetricsReport {
    pub experiment: &'static str,
    pub fixture: FixtureReport,
    pub samples: usize,
    pub number_of_classes: usize,
    pub metrics: Vec<MetricScore>,
    pub confusion: Vec<ConfusionCell>,
}

#[derive(Clone, Debug, Serialize)]
pub struct MetricScore {
    pub name: String,
    pub value: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct ConfusionCell {
    pub actual: i64,
    pub predicted: i64,
    pub count: usize,
}

#[derive(Clone, Debug, Serialize)]
pub struct LogisticDemoReport {
    pub experiment: &'static str,
    pub fixture: FixtureReport,
    pub samples: usize,
    pub feature_dimension: usize,
    pub number_of_classes: usize,
    pub accuracy: f64,
    pub sample_probabilities: Vec<SampleProbabilities>,
}

#[derive(Clone, Debug, Serialize)]
pub struct SampleProbabilities {
    pub row: usize,
    pub label: i32,
    pub probabilities: Vec<f64>,
    pub predicted_class: usize,
}

#[derive(Clone, Debug, Serialize)]
pub struct LinearDemoReport {
    pub experiment: &'static str,
    pub fixture: FixtureReport,
    pub samples: usize,
    pub feature_dimension: usize,
    pub mean_absolute_error: f64,
    pub sample_predictions: Vec<SamplePrediction>,
}

#[derive(Clone, Debug, Serialize)]
pub struct SamplePrediction {
    pub row: usize,
    pub target: f64,
    pub prediction: f64,
    pub absolute_error: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct NodeClassificationPreviewReport {
    pub experiment: &'static str,
    pub fixture: FixtureReport,
    pub samples: usize,
    pub number_of_classes: usize,
    pub global_accuracy: f64,
    pub predicted_classes_sample: Vec<i64>,
    pub sample_probabilities: Vec<SampleProbabilities>,
}

#[derive(Clone, Debug, Serialize)]
pub struct NodePredictionSplitPreviewReport {
    pub experiment: &'static str,
    pub fixture: FixtureReport,
    pub total_examples: usize,
    pub train_size: usize,
    pub test_size: usize,
    pub all_training_examples_sample: Vec<i64>,
    pub train_set_sample: Vec<i64>,
    pub test_set_sample: Vec<i64>,
}

#[derive(Clone, Debug, Serialize)]
pub struct NodeRegressionPreviewReport {
    pub experiment: &'static str,
    pub fixture: FixtureReport,
    pub samples: usize,
    pub mean_absolute_error: f64,
    pub sample_predictions: Vec<SamplePrediction>,
}

#[derive(Clone, Debug, Serialize)]
pub struct NodeRegressionMetricsReport {
    pub experiment: &'static str,
    pub fixture: FixtureReport,
    pub samples: usize,
    pub metrics: Vec<MetricScore>,
    pub residuals_sample: Vec<ResidualSample>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ResidualSample {
    pub row: usize,
    pub target: f64,
    pub prediction: f64,
    pub residual: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct FixtureReport {
    pub id: String,
    pub name: String,
    pub task: String,
    pub feature_names: Vec<String>,
    pub seed: Option<u64>,
    pub metadata: Option<FixtureMetadata>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FixtureMetadata {
    pub variant: String,
    pub noise_level: f64,
    pub generator: String,
    pub size_profile: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct LargeBenchmarkReport {
    pub experiment: &'static str,
    pub fixture: FixtureReport,
    pub samples: usize,
    pub feature_dimension: usize,
    pub train_size: usize,
    pub test_size: usize,
    pub split_seed: u64,
    pub metric_name: &'static str,
    pub metric_value: f64,
    pub runtime_millis: u128,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbose: Option<BenchmarkVerboseReport>,
}

#[derive(Clone, Debug, Serialize)]
pub struct BenchmarkVerboseReport {
    pub progression: Vec<MetricCheckpoint>,
    pub sample_observations: Vec<BenchmarkObservation>,
}

#[derive(Clone, Debug, Serialize)]
pub struct MetricCheckpoint {
    pub max_epochs: usize,
    pub metric_name: &'static str,
    pub metric_value: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct BenchmarkObservation {
    pub row: usize,
    pub expected: f64,
    pub predicted: f64,
    pub absolute_error: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct SemanticProjectionPreviewReport {
    pub experiment: &'static str,
    pub fixture: FixtureReport,
    pub samples: usize,
    pub semantic_rows: usize,
    pub projection_dimension: usize,
    pub accuracy: f64,
    pub sample_rows: Vec<SemanticProjectionRow>,
}

#[derive(Clone, Debug, Serialize)]
pub struct SemanticProjectionRow {
    pub row: usize,
    pub node_id: String,
    pub concept: String,
    pub polarity: i32,
    pub confidence: f64,
    pub numeric_projection: Vec<f64>,
    pub label: i32,
    pub predicted_class: usize,
}

#[derive(Clone, Debug, Serialize)]
pub struct SweepReport {
    pub experiment: &'static str,
    pub fixture: FixtureReport,
    pub runs: Vec<SweepRun>,
}

#[derive(Clone, Debug, Serialize)]
pub struct SweepRun {
    pub learning_rate: f64,
    pub max_epochs: usize,
    pub penalty: f64,
    pub metric_name: &'static str,
    pub metric_value: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct ModelComparisonReport {
    pub experiment: &'static str,
    pub fixture: FixtureReport,
    pub samples: usize,
    pub train_size: usize,
    pub test_size: usize,
    pub split_seed: u64,
    pub logistic: ModelScore,
    pub mlp: ModelScore,
    pub accuracy_delta: f64,
    pub sample_rows: Vec<ModelComparisonRow>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ModelScore {
    pub accuracy: f64,
    pub runtime_millis: u128,
}

#[derive(Clone, Debug, Serialize)]
pub struct ModelComparisonRow {
    pub row: usize,
    pub label: i32,
    pub logistic_predicted_class: usize,
    pub logistic_probabilities: Vec<f64>,
    pub mlp_predicted_class: usize,
    pub mlp_probabilities: Vec<f64>,
}

#[derive(Clone, Debug, Deserialize)]
struct ClassificationFixture {
    id: String,
    name: String,
    task: String,
    feature_names: Vec<String>,
    features: Vec<Vec<f64>>,
    labels: Vec<i32>,
    sample_rows: Vec<usize>,
    #[serde(default)]
    seed: Option<u64>,
    #[serde(default)]
    metadata: Option<FixtureMetadata>,
    #[serde(default)]
    semantic_rows: Vec<SemanticFixtureRow>,
}

#[derive(Clone, Debug, Deserialize)]
struct RegressionFixture {
    id: String,
    name: String,
    task: String,
    feature_names: Vec<String>,
    features: Vec<Vec<f64>>,
    targets: Vec<f64>,
    sample_rows: Vec<usize>,
    #[serde(default)]
    seed: Option<u64>,
    #[serde(default)]
    metadata: Option<FixtureMetadata>,
}

#[derive(Clone, Debug, Deserialize)]
struct SemanticFixtureRow {
    row: usize,
    node_id: String,
    concept: String,
    relation_tags: Vec<String>,
    polarity: i32,
    confidence: f64,
    provenance_span: String,
}

pub fn available_experiments() -> &'static [&'static str] {
    &[
        "fixtures",
        "logistic-demo",
        "logistic-three-class-demo",
        "logistic-three-class-overlap-demo",
        "logistic-three-class-large-demo",
        "mlp-demo",
        "mlp-three-class-demo",
        "mlp-three-class-overlap-demo",
        "mlp-three-class-large-demo",
        "logistic-vs-mlp-overlap-comparison",
        "mlp-three-class-sweep",
        "linear-demo",
        "logistic-sweep",
        "linear-sweep",
        "node-classification-preview",
        "node-classification-metrics",
        "node-prediction-split-preview",
        "node-regression-preview",
        "node-regression-metrics",
        "large-logistic-benchmark",
        "large-linear-benchmark",
    ]
}

pub fn fixture_catalog() -> &'static [FixtureSummary] {
    &[
        FixtureSummary {
            id: "logistic-separated",
            name: "Separated Binary Classification",
            task: "classification",
            source_path: "gds/fixtures/ml-models/logistic-separated.json",
        },
        FixtureSummary {
            id: "logistic-three-class",
            name: "Three-Class Spiral Classification",
            task: "classification",
            source_path: "gds/fixtures/ml-models/logistic-three-class.json",
        },
        FixtureSummary {
            id: "logistic-three-class-overlap",
            name: "Three-Class Overlap Classification",
            task: "classification",
            source_path: "gds/fixtures/ml-models/logistic-three-class-overlap.json",
        },
        FixtureSummary {
            id: "logistic-three-class-large",
            name: "Three-Class Noisy Classification (Large)",
            task: "classification",
            source_path: "gds/fixtures/ml-models/logistic-three-class-large.json",
        },
        FixtureSummary {
            id: "linear-affine",
            name: "Affine Regression Line",
            task: "regression",
            source_path: "gds/fixtures/ml-models/linear-affine.json",
        },
        FixtureSummary {
            id: "logistic-separated-large",
            name: "Separated Binary Classification (Large)",
            task: "classification",
            source_path: "gds/fixtures/ml-models/logistic-separated-large.json",
        },
        FixtureSummary {
            id: "linear-affine-large",
            name: "Affine Regression Line (Large)",
            task: "regression",
            source_path: "gds/fixtures/ml-models/linear-affine-large.json",
        },
        FixtureSummary {
            id: "semantic-logistic-prototype",
            name: "Semantic Sidecar Binary Classification Prototype",
            task: "classification",
            source_path: "gds/fixtures/ml-models/semantic-logistic-prototype.json",
        },
    ]
}

pub fn run_logistic_demo() -> LogisticDemoReport {
    let config = LogisticRegressionTrainConfig {
        batch_size: 2,
        learning_rate: 0.05,
        max_epochs: 150,
        tolerance: 1e-6,
        ..Default::default()
    };
    run_logistic_with_config("logistic-demo", logistic_fixture(), config)
}

pub fn run_logistic_three_class_demo() -> LogisticDemoReport {
    let config = LogisticRegressionTrainConfig {
        batch_size: 3,
        learning_rate: 0.04,
        max_epochs: 220,
        tolerance: 1e-6,
        ..Default::default()
    };
    run_logistic_with_config(
        "logistic-three-class-demo",
        logistic_three_class_fixture(),
        config,
    )
}

pub fn run_logistic_three_class_overlap_demo() -> LogisticDemoReport {
    let config = LogisticRegressionTrainConfig {
        batch_size: 6,
        learning_rate: 0.03,
        max_epochs: 300,
        tolerance: 1e-6,
        ..Default::default()
    };
    run_logistic_with_config(
        "logistic-three-class-overlap-demo",
        logistic_three_class_overlap_fixture(),
        config,
    )
}

pub fn run_logistic_three_class_large_demo() -> LogisticDemoReport {
    let config = LogisticRegressionTrainConfig {
        batch_size: 12,
        learning_rate: 0.03,
        max_epochs: 320,
        tolerance: 1e-6,
        ..Default::default()
    };
    run_logistic_with_config(
        "logistic-three-class-large-demo",
        logistic_three_class_large_fixture(),
        config,
    )
}

pub fn run_mlp_demo() -> LogisticDemoReport {
    let config = MLPClassifierTrainConfig::builder()
        .batch_size(2)
        .learning_rate(0.03)
        .max_epochs(200)
        .hidden_layer_sizes(vec![8, 4])
        .build()
        .expect("mlp demo config should be valid");
    run_mlp_with_config("mlp-demo", logistic_fixture(), config, Some(42))
}

pub fn run_mlp_three_class_demo() -> LogisticDemoReport {
    let config = MLPClassifierTrainConfig::builder()
        .batch_size(3)
        .min_epochs(20)
        .patience(20)
        .learning_rate(0.02)
        .max_epochs(500)
        .tolerance(1e-6)
        .hidden_layer_sizes(vec![16, 8])
        .build()
        .expect("mlp three class demo config should be valid");
    run_mlp_with_config(
        "mlp-three-class-demo",
        logistic_three_class_fixture(),
        config,
        Some(42),
    )
}

pub fn run_mlp_three_class_overlap_demo() -> LogisticDemoReport {
    let config = MLPClassifierTrainConfig::builder()
        .batch_size(6)
        .min_epochs(30)
        .patience(20)
        .learning_rate(0.02)
        .max_epochs(420)
        .tolerance(1e-6)
        .hidden_layer_sizes(vec![24, 12])
        .build()
        .expect("mlp three class overlap demo config should be valid");
    run_mlp_with_config(
        "mlp-three-class-overlap-demo",
        logistic_three_class_overlap_fixture(),
        config,
        Some(42),
    )
}

pub fn run_mlp_three_class_large_demo() -> LogisticDemoReport {
    let config = MLPClassifierTrainConfig::builder()
        .batch_size(12)
        .min_epochs(30)
        .patience(25)
        .learning_rate(0.015)
        .max_epochs(420)
        .tolerance(1e-6)
        .hidden_layer_sizes(vec![32, 16])
        .build()
        .expect("mlp three class large demo config should be valid");
    run_mlp_with_config(
        "mlp-three-class-large-demo",
        logistic_three_class_large_fixture(),
        config,
        Some(42),
    )
}

pub fn run_logistic_vs_mlp_overlap_comparison() -> ModelComparisonReport {
    let fixture = logistic_three_class_overlap_fixture();
    let split_seed = 4242;

    let features = DenseFeatures::new(fixture.features.clone());
    let labels = HugeIntArray::from_vec(fixture.labels.clone());
    let (train_set, test_set) = split_train_test_indices(features.size(), 0.25, split_seed);
    let train_set_arc = Arc::new(train_set.clone());
    let number_of_classes = fixture
        .labels
        .iter()
        .copied()
        .max()
        .map(|class_id| class_id as usize + 1)
        .unwrap_or(0);

    let logistic_start = Instant::now();
    let logistic_trainer = LogisticRegressionTrainer::new(
        LogisticRegressionTrainConfig {
            batch_size: 6,
            learning_rate: 0.03,
            max_epochs: 300,
            tolerance: 1e-6,
            ..Default::default()
        },
        number_of_classes,
        false,
        Arc::new(RwLock::new(false)),
        1,
    );
    let logistic_classifier = logistic_trainer.train(&features, &labels, &train_set_arc);
    let logistic_runtime = logistic_start.elapsed().as_millis();

    let logistic_correct = test_set
        .iter()
        .filter(|&&index| {
            let row = index as usize;
            argmax(&logistic_classifier.predict_probabilities(features.get(row))) as i32
                == fixture.labels[row]
        })
        .count();
    let logistic_accuracy = logistic_correct as f64 / test_set.len() as f64;

    let mlp_start = Instant::now();
    let mlp_config = MLPClassifierTrainConfig::builder()
        .batch_size(6)
        .min_epochs(30)
        .patience(20)
        .learning_rate(0.02)
        .max_epochs(420)
        .tolerance(1e-6)
        .hidden_layer_sizes(vec![24, 12])
        .build()
        .expect("mlp overlap comparison config should be valid");
    let mlp_trainer = MLPClassifierTrainer::new(number_of_classes, mlp_config, Some(42), 1);
    let mlp_classifier = mlp_trainer.train(&features, &labels, &train_set_arc);
    let mlp_runtime = mlp_start.elapsed().as_millis();

    let mlp_correct = test_set
        .iter()
        .filter(|&&index| {
            let row = index as usize;
            argmax(&mlp_classifier.predict_probabilities(features.get(row))) as i32
                == fixture.labels[row]
        })
        .count();
    let mlp_accuracy = mlp_correct as f64 / test_set.len() as f64;

    let sample_rows = fixture
        .sample_rows
        .iter()
        .map(|&row| {
            let logistic_probabilities =
                logistic_classifier.predict_probabilities(features.get(row));
            let mlp_probabilities = mlp_classifier.predict_probabilities(features.get(row));
            ModelComparisonRow {
                row,
                label: fixture.labels[row],
                logistic_predicted_class: argmax(&logistic_probabilities),
                logistic_probabilities,
                mlp_predicted_class: argmax(&mlp_probabilities),
                mlp_probabilities,
            }
        })
        .collect::<Vec<ModelComparisonRow>>();

    ModelComparisonReport {
        experiment: "logistic-vs-mlp-overlap-comparison",
        fixture: fixture.report(),
        samples: features.size(),
        train_size: train_set.len(),
        test_size: test_set.len(),
        split_seed,
        logistic: ModelScore {
            accuracy: logistic_accuracy,
            runtime_millis: logistic_runtime,
        },
        mlp: ModelScore {
            accuracy: mlp_accuracy,
            runtime_millis: mlp_runtime,
        },
        accuracy_delta: mlp_accuracy - logistic_accuracy,
        sample_rows,
    }
}

pub fn run_mlp_three_class_sweep() -> SweepReport {
    let fixture = logistic_three_class_fixture();
    let configs = [
        (0.01, 240, 0.0),
        (0.02, 360, 0.0),
        (0.02, 500, 0.001),
        (0.03, 500, 0.0),
    ];

    let runs = configs
        .into_iter()
        .map(|(learning_rate, max_epochs, penalty)| {
            let config = MLPClassifierTrainConfig::builder()
                .batch_size(3)
                .min_epochs(20)
                .patience(20)
                .learning_rate(learning_rate)
                .max_epochs(max_epochs)
                .tolerance(1e-6)
                .penalty(penalty)
                .hidden_layer_sizes(vec![16, 8])
                .build()
                .expect("mlp three class sweep config should be valid");

            let report =
                run_mlp_with_config("mlp-three-class-sweep", fixture.clone(), config, Some(42));

            SweepRun {
                learning_rate,
                max_epochs,
                penalty,
                metric_name: "accuracy",
                metric_value: report.accuracy,
            }
        })
        .collect();

    SweepReport {
        experiment: "mlp-three-class-sweep",
        fixture: fixture.report(),
        runs,
    }
}

pub fn run_logistic_sweep() -> SweepReport {
    let fixture = logistic_fixture();
    let configs = [
        (0.01, 80, 0.0),
        (0.03, 120, 0.0),
        (0.05, 150, 0.0),
        (0.05, 150, 0.01),
    ];
    let runs = configs
        .into_iter()
        .map(|(learning_rate, max_epochs, penalty)| {
            let config = LogisticRegressionTrainConfig {
                batch_size: 2,
                learning_rate,
                max_epochs,
                tolerance: 1e-6,
                penalty,
                ..Default::default()
            };
            let report = run_logistic_with_config("logistic-sweep", fixture.clone(), config);
            SweepRun {
                learning_rate,
                max_epochs,
                penalty,
                metric_name: "accuracy",
                metric_value: report.accuracy,
            }
        })
        .collect();

    SweepReport {
        experiment: "logistic-sweep",
        fixture: fixture.report(),
        runs,
    }
}

pub fn run_linear_demo() -> LinearDemoReport {
    let gradient = GradientDescentConfig::builder()
        .batch_size(2)
        .learning_rate(0.03)
        .max_epochs(300)
        .tolerance(1e-7)
        .build()
        .expect("linear demo gradient config should be valid");
    let config = LinearRegressionTrainConfig::new(gradient, 0.0);
    run_linear_with_config("linear-demo", linear_fixture(), config)
}

pub fn run_linear_sweep() -> SweepReport {
    let fixture = linear_fixture();
    let configs = [
        (0.01, 180, 0.0),
        (0.03, 300, 0.0),
        (0.05, 300, 0.0),
        (0.03, 300, 0.01),
    ];
    let runs = configs
        .into_iter()
        .map(|(learning_rate, max_epochs, penalty)| {
            let gradient = GradientDescentConfig::builder()
                .batch_size(2)
                .learning_rate(learning_rate)
                .max_epochs(max_epochs)
                .tolerance(1e-7)
                .build()
                .expect("linear sweep gradient config should be valid");
            let config = LinearRegressionTrainConfig::new(gradient, penalty);
            let report = run_linear_with_config("linear-sweep", fixture.clone(), config);
            SweepRun {
                learning_rate,
                max_epochs,
                penalty,
                metric_name: "mean_absolute_error",
                metric_value: report.mean_absolute_error,
            }
        })
        .collect();

    SweepReport {
        experiment: "linear-sweep",
        fixture: fixture.report(),
        runs,
    }
}

pub fn run_large_logistic_benchmark() -> LargeBenchmarkReport {
    run_large_logistic_benchmark_with_verbose(false)
}

pub fn run_large_logistic_benchmark_with_verbose(verbose: bool) -> LargeBenchmarkReport {
    let fixture = logistic_large_fixture();
    let split_seed = 1337;
    let start = Instant::now();

    let features = DenseFeatures::new(fixture.features.clone());
    let labels = HugeIntArray::from_vec(fixture.labels.clone());
    let (train_set, test_set) = split_train_test_indices(features.size(), 0.2, split_seed);
    let number_of_classes = fixture
        .labels
        .iter()
        .copied()
        .max()
        .map(|class_id| class_id as usize + 1)
        .unwrap_or(0);

    let trainer = LogisticRegressionTrainer::new(
        LogisticRegressionTrainConfig {
            batch_size: 64,
            learning_rate: 0.03,
            max_epochs: 160,
            tolerance: 1e-6,
            ..Default::default()
        },
        number_of_classes,
        false,
        Arc::new(RwLock::new(false)),
        1,
    );

    let classifier = trainer.train(&features, &labels, &Arc::new(train_set.clone()));
    let mut correct = 0usize;
    for &index in &test_set {
        let predicted_class =
            argmax(&classifier.predict_probabilities(features.get(index as usize)));
        if predicted_class as i32 == fixture.labels[index as usize] {
            correct += 1;
        }
    }

    let verbose_report = if verbose {
        Some(logistic_benchmark_verbose(
            &fixture,
            &train_set,
            &test_set,
            number_of_classes,
        ))
    } else {
        None
    };

    LargeBenchmarkReport {
        experiment: "large-logistic-benchmark",
        fixture: fixture.report(),
        samples: features.size(),
        feature_dimension: classifier.data().feature_dimension(),
        train_size: train_set.len(),
        test_size: test_set.len(),
        split_seed,
        metric_name: "accuracy",
        metric_value: correct as f64 / test_set.len() as f64,
        runtime_millis: start.elapsed().as_millis(),
        verbose: verbose_report,
    }
}

pub fn run_large_linear_benchmark() -> LargeBenchmarkReport {
    run_large_linear_benchmark_with_verbose(false)
}

pub fn run_large_linear_benchmark_with_verbose(verbose: bool) -> LargeBenchmarkReport {
    let fixture = linear_large_fixture();
    let split_seed = 1337;
    let start = Instant::now();

    let features = DenseFeatures::new(fixture.features.clone());
    let targets = HugeDoubleArray::from_vec(fixture.targets.clone());
    let (train_set, test_set) = split_train_test_indices(features.size(), 0.2, split_seed);

    let gradient = GradientDescentConfig::builder()
        .batch_size(64)
        .learning_rate(0.02)
        .max_epochs(220)
        .tolerance(1e-7)
        .build()
        .expect("large linear benchmark gradient config should be valid");
    let trainer = LinearRegressionTrainer::new(
        1,
        LinearRegressionTrainConfig::new(gradient, 0.0),
        Arc::new(RwLock::new(false)),
    );

    let regressor = trainer.train(&features, &targets, &Arc::new(train_set.clone()));
    let mut absolute_error = 0.0;
    for &index in &test_set {
        let row = index as usize;
        absolute_error += (regressor.predict(features.get(row)) - fixture.targets[row]).abs();
    }

    let verbose_report = if verbose {
        Some(linear_benchmark_verbose(&fixture, &train_set, &test_set))
    } else {
        None
    };

    LargeBenchmarkReport {
        experiment: "large-linear-benchmark",
        fixture: fixture.report(),
        samples: features.size(),
        feature_dimension: regressor.data().feature_dimension(),
        train_size: train_set.len(),
        test_size: test_set.len(),
        split_seed,
        metric_name: "mean_absolute_error",
        metric_value: absolute_error / test_set.len() as f64,
        runtime_millis: start.elapsed().as_millis(),
        verbose: verbose_report,
    }
}

pub fn run_semantic_projection_preview() -> SemanticProjectionPreviewReport {
    let fixture = semantic_logistic_fixture();
    let projected = project_semantic_features(&fixture);
    let features = DenseFeatures::new(projected.clone());
    let labels = HugeIntArray::from_vec(fixture.labels.clone());
    let train_set = Arc::new((0..features.size() as u64).collect::<Vec<u64>>());
    let number_of_classes = fixture
        .labels
        .iter()
        .copied()
        .max()
        .map(|class_id| class_id as usize + 1)
        .unwrap_or(0);

    let trainer = LogisticRegressionTrainer::new(
        LogisticRegressionTrainConfig {
            batch_size: 2,
            learning_rate: 0.05,
            max_epochs: 180,
            tolerance: 1e-6,
            ..Default::default()
        },
        number_of_classes,
        false,
        Arc::new(RwLock::new(false)),
        1,
    );
    let classifier = trainer.train(&features, &labels, &train_set);

    let mut correct = 0usize;
    for (row, label) in fixture.labels.iter().copied().enumerate() {
        let predicted = argmax(&classifier.predict_probabilities(features.get(row)));
        if predicted as i32 == label {
            correct += 1;
        }
    }

    let sample_rows = fixture
        .sample_rows
        .iter()
        .filter_map(|&row| {
            fixture
                .semantic_rows
                .iter()
                .find(|semantic| semantic.row == row)
                .map(|semantic| SemanticProjectionRow {
                    row,
                    node_id: semantic.node_id.clone(),
                    concept: semantic.concept.clone(),
                    polarity: semantic.polarity,
                    confidence: semantic.confidence,
                    numeric_projection: projected[row].clone(),
                    label: fixture.labels[row],
                    predicted_class: argmax(&classifier.predict_probabilities(features.get(row))),
                })
        })
        .collect::<Vec<SemanticProjectionRow>>();

    SemanticProjectionPreviewReport {
        experiment: "semantic-projection-preview",
        fixture: fixture.report(),
        samples: features.size(),
        semantic_rows: fixture.semantic_rows.len(),
        projection_dimension: projected.first().map(Vec::len).unwrap_or(0),
        accuracy: correct as f64 / features.size() as f64,
        sample_rows,
    }
}

pub fn run_node_classification_preview() -> NodeClassificationPreviewReport {
    let fixture = logistic_fixture();
    let features: Arc<dyn Features> = Arc::new(DenseFeatures::new(fixture.features.clone()));
    let labels_for_training = HugeIntArray::from_vec(fixture.labels.clone());
    let train_set = Arc::new((0..features.size() as u64).collect::<Vec<u64>>());
    let number_of_classes = fixture
        .labels
        .iter()
        .copied()
        .max()
        .map(|class_id| class_id as usize + 1)
        .unwrap_or(0);

    let trainer = LogisticRegressionTrainer::new(
        LogisticRegressionTrainConfig {
            batch_size: 2,
            learning_rate: 0.05,
            max_epochs: 150,
            tolerance: 1e-6,
            ..Default::default()
        },
        number_of_classes,
        false,
        Arc::new(RwLock::new(false)),
        1,
    );

    let classifier = Arc::from(trainer.train(features.as_ref(), &labels_for_training, &train_set));

    let predictor = NodeClassificationPredict::new(
        Arc::clone(&classifier),
        Arc::clone(&features),
        2,
        false,
        Concurrency::single_threaded(),
        TerminationFlag::default(),
        TaskProgressTracker::new(NodeClassificationPredict::progress_task(
            features.size() as u64
        )),
    );
    let prediction_result = predictor.compute();

    let labels_for_metrics = HugeLongArray::from_vec(
        fixture
            .labels
            .iter()
            .map(|&label| label as i64)
            .collect::<Vec<i64>>(),
    );
    let metric_computer = ClassificationMetricComputer::new(
        Arc::clone(prediction_result.predicted_classes()),
        Arc::new(labels_for_metrics),
    );
    let global_accuracy = metric_computer.score(&GlobalAccuracy::new());

    let predicted_classes_sample = fixture
        .sample_rows
        .iter()
        .map(|&row| prediction_result.predicted_classes().get(row))
        .collect::<Vec<i64>>();

    let sample_probabilities = fixture
        .sample_rows
        .iter()
        .map(|&row| {
            let probs = classifier.predict_probabilities(features.get(row));
            SampleProbabilities {
                row,
                label: fixture.labels[row],
                predicted_class: argmax(&probs),
                probabilities: probs,
            }
        })
        .collect::<Vec<SampleProbabilities>>();

    NodeClassificationPreviewReport {
        experiment: "node-classification-preview",
        fixture: fixture.report(),
        samples: features.size(),
        number_of_classes,
        global_accuracy,
        predicted_classes_sample,
        sample_probabilities,
    }
}

pub fn run_node_classification_metrics() -> NodeClassificationMetricsReport {
    let fixture = logistic_fixture();
    let features: Arc<dyn Features> = Arc::new(DenseFeatures::new(fixture.features.clone()));
    let labels_for_training = HugeIntArray::from_vec(fixture.labels.clone());
    let train_set = Arc::new((0..features.size() as u64).collect::<Vec<u64>>());
    let number_of_classes = fixture
        .labels
        .iter()
        .copied()
        .max()
        .map(|class_id| class_id as usize + 1)
        .unwrap_or(0);

    let trainer = LogisticRegressionTrainer::new(
        LogisticRegressionTrainConfig {
            batch_size: 2,
            learning_rate: 0.05,
            max_epochs: 150,
            tolerance: 1e-6,
            ..Default::default()
        },
        number_of_classes,
        false,
        Arc::new(RwLock::new(false)),
        1,
    );
    let classifier = Arc::from(trainer.train(features.as_ref(), &labels_for_training, &train_set));

    let predictor = NodeClassificationPredict::new(
        classifier,
        Arc::clone(&features),
        2,
        false,
        Concurrency::single_threaded(),
        TerminationFlag::default(),
        TaskProgressTracker::new(NodeClassificationPredict::progress_task(
            features.size() as u64
        )),
    );
    let prediction_result = predictor.compute();
    let labels = HugeLongArray::from_vec(
        fixture
            .labels
            .iter()
            .map(|&label| label as i64)
            .collect::<Vec<i64>>(),
    );
    let metric_computer = ClassificationMetricComputer::new(
        Arc::clone(prediction_result.predicted_classes()),
        Arc::new(labels.clone()),
    );

    let mut metrics = vec![score_metric(&metric_computer, &GlobalAccuracy::new())];
    for class_id in 0..number_of_classes as i64 {
        metrics.push(score_metric(
            &metric_computer,
            &Accuracy::new(class_id, class_id),
        ));
        metrics.push(score_metric(
            &metric_computer,
            &Precision::new(class_id, class_id),
        ));
        metrics.push(score_metric(
            &metric_computer,
            &Recall::new(class_id, class_id),
        ));
        metrics.push(score_metric(
            &metric_computer,
            &F1Score::new(class_id, class_id),
        ));
    }

    NodeClassificationMetricsReport {
        experiment: "node-classification-metrics",
        fixture: fixture.report(),
        samples: features.size(),
        number_of_classes,
        metrics,
        confusion: confusion_cells(&labels, prediction_result.predicted_classes()),
    }
}

pub fn run_node_prediction_split_preview() -> NodePredictionSplitPreviewReport {
    let fixture = linear_fixture();
    let total_examples = fixture.features.len();

    let splitter = NodeSplitter::new(
        Concurrency::single_threaded(),
        total_examples,
        Arc::new(|index| index as i64),
        Arc::new(|original_id| original_id as usize),
    );

    let mut progress_tracker = NoopProgressTracker;
    let splits = splitter.split(0.25, 3, Some(42), &mut progress_tracker);
    let outer_split = splits.outer_split();
    let train_set = outer_split.train_set();
    let test_set = outer_split.test_set();

    NodePredictionSplitPreviewReport {
        experiment: "node-prediction-split-preview",
        fixture: fixture.report(),
        total_examples,
        train_size: train_set.len(),
        test_size: test_set.len(),
        all_training_examples_sample: sample_i64(&splits.all_training_examples(), 6),
        train_set_sample: sample_i64(&train_set, 6),
        test_set_sample: sample_i64(&test_set, 6),
    }
}

pub fn run_node_regression_preview() -> NodeRegressionPreviewReport {
    let fixture = linear_fixture();
    let features: Arc<dyn Features> = Arc::new(DenseFeatures::new(fixture.features.clone()));
    let targets = HugeDoubleArray::from_vec(fixture.targets.clone());
    let train_set = Arc::new((0..features.size() as u64).collect::<Vec<u64>>());

    let gradient = GradientDescentConfig::builder()
        .batch_size(2)
        .learning_rate(0.03)
        .max_epochs(300)
        .tolerance(1e-7)
        .build()
        .expect("node regression gradient config should be valid");
    let trainer = LinearRegressionTrainer::new(
        1,
        LinearRegressionTrainConfig::new(gradient, 0.0),
        Arc::new(RwLock::new(false)),
    );
    let regressor = Arc::from(trainer.train(features.as_ref(), &targets, &train_set));

    let predictor = NodeRegressionPredict::new(
        Arc::clone(&regressor),
        Arc::clone(&features),
        Concurrency::single_threaded(),
        TaskProgressTracker::new(NodeRegressionPredict::progress_task(features.size() as u64)),
        TerminationFlag::default(),
    );
    let predictions = predictor.compute();

    let mut total_absolute_error = 0.0;
    let mut sample_predictions = Vec::new();
    for (row, target) in fixture.targets.iter().copied().enumerate() {
        let prediction = predictions.get(row);
        let absolute_error = (prediction - target).abs();
        total_absolute_error += absolute_error;
        if fixture.sample_rows.contains(&row) {
            sample_predictions.push(SamplePrediction {
                row,
                target,
                prediction,
                absolute_error,
            });
        }
    }

    NodeRegressionPreviewReport {
        experiment: "node-regression-preview",
        fixture: fixture.report(),
        samples: features.size(),
        mean_absolute_error: total_absolute_error / features.size() as f64,
        sample_predictions,
    }
}

pub fn run_node_regression_metrics() -> NodeRegressionMetricsReport {
    let fixture = linear_fixture();
    let features: Arc<dyn Features> = Arc::new(DenseFeatures::new(fixture.features.clone()));
    let targets = HugeDoubleArray::from_vec(fixture.targets.clone());
    let train_set = Arc::new((0..features.size() as u64).collect::<Vec<u64>>());

    let gradient = GradientDescentConfig::builder()
        .batch_size(2)
        .learning_rate(0.03)
        .max_epochs(300)
        .tolerance(1e-7)
        .build()
        .expect("node regression metric gradient config should be valid");
    let trainer = LinearRegressionTrainer::new(
        1,
        LinearRegressionTrainConfig::new(gradient, 0.0),
        Arc::new(RwLock::new(false)),
    );
    let regressor = Arc::from(trainer.train(features.as_ref(), &targets, &train_set));

    let predictor = NodeRegressionPredict::new(
        Arc::clone(&regressor),
        Arc::clone(&features),
        Concurrency::single_threaded(),
        TaskProgressTracker::new(NodeRegressionPredict::progress_task(features.size() as u64)),
        TerminationFlag::default(),
    );
    let predictions = predictor.compute();
    let metrics = [
        RegressionMetric::MeanSquaredError,
        RegressionMetric::RootMeanSquaredError,
        RegressionMetric::MeanAbsoluteError,
    ]
    .iter()
    .map(|metric| MetricScore {
        name: metric.to_string(),
        value: metric.compute(&targets, &predictions),
    })
    .collect::<Vec<MetricScore>>();

    let residuals_sample = fixture
        .sample_rows
        .iter()
        .map(|&row| {
            let target = targets.get(row);
            let prediction = predictions.get(row);
            ResidualSample {
                row,
                target,
                prediction,
                residual: prediction - target,
            }
        })
        .collect::<Vec<ResidualSample>>();

    NodeRegressionMetricsReport {
        experiment: "node-regression-metrics",
        fixture: fixture.report(),
        samples: features.size(),
        metrics,
        residuals_sample,
    }
}

fn run_logistic_with_config(
    experiment: &'static str,
    fixture: ClassificationFixture,
    config: LogisticRegressionTrainConfig,
) -> LogisticDemoReport {
    let features = DenseFeatures::new(fixture.features.clone());
    let labels = HugeIntArray::from_vec(fixture.labels.clone());
    let train_set = Arc::new((0..features.size() as u64).collect::<Vec<u64>>());
    let number_of_classes = fixture
        .labels
        .iter()
        .copied()
        .max()
        .map(|class_id| class_id as usize + 1)
        .unwrap_or(0);
    let trainer = LogisticRegressionTrainer::new(
        config,
        number_of_classes,
        false,
        Arc::new(RwLock::new(false)),
        1,
    );
    let classifier = trainer.train(&features, &labels, &train_set);

    let mut correct = 0usize;
    let mut sample_probabilities = Vec::new();

    for (row, label) in fixture.labels.iter().copied().enumerate() {
        let probabilities = classifier.predict_probabilities(features.get(row));
        let predicted_class = argmax(&probabilities);
        if predicted_class as i32 == label {
            correct += 1;
        }
        if fixture.sample_rows.contains(&row) {
            sample_probabilities.push(SampleProbabilities {
                row,
                label,
                probabilities,
                predicted_class,
            });
        }
    }

    LogisticDemoReport {
        experiment,
        fixture: fixture.report(),
        samples: features.size(),
        feature_dimension: classifier.data().feature_dimension(),
        number_of_classes: classifier.number_of_classes(),
        accuracy: correct as f64 / features.size() as f64,
        sample_probabilities,
    }
}

fn run_mlp_with_config(
    experiment: &'static str,
    fixture: ClassificationFixture,
    config: MLPClassifierTrainConfig,
    random_seed: Option<u64>,
) -> LogisticDemoReport {
    let features = DenseFeatures::new(fixture.features.clone());
    let labels = HugeIntArray::from_vec(fixture.labels.clone());
    let train_set = Arc::new((0..features.size() as u64).collect::<Vec<u64>>());
    let number_of_classes = fixture
        .labels
        .iter()
        .copied()
        .max()
        .map(|class_id| class_id as usize + 1)
        .unwrap_or(0);

    let trainer = MLPClassifierTrainer::new(number_of_classes, config, random_seed, 1);
    let classifier = trainer.train(&features, &labels, &train_set);

    let mut correct = 0usize;
    let mut sample_probabilities = Vec::new();

    for (row, label) in fixture.labels.iter().copied().enumerate() {
        let probabilities = classifier.predict_probabilities(features.get(row));
        let predicted_class = argmax(&probabilities);
        if predicted_class as i32 == label {
            correct += 1;
        }
        if fixture.sample_rows.contains(&row) {
            sample_probabilities.push(SampleProbabilities {
                row,
                label,
                probabilities,
                predicted_class,
            });
        }
    }

    LogisticDemoReport {
        experiment,
        fixture: fixture.report(),
        samples: features.size(),
        feature_dimension: classifier.data().feature_dimension(),
        number_of_classes: classifier.number_of_classes(),
        accuracy: correct as f64 / features.size() as f64,
        sample_probabilities,
    }
}

fn run_linear_with_config(
    experiment: &'static str,
    fixture: RegressionFixture,
    config: LinearRegressionTrainConfig,
) -> LinearDemoReport {
    let features = DenseFeatures::new(fixture.features.clone());
    let targets = HugeDoubleArray::from_vec(fixture.targets.clone());
    let train_set = Arc::new((0..features.size() as u64).collect::<Vec<u64>>());
    let trainer = LinearRegressionTrainer::new(1, config, Arc::new(RwLock::new(false)));
    let regressor = trainer.train(&features, &targets, &train_set);

    let mut total_absolute_error = 0.0;
    let mut sample_predictions = Vec::new();

    for (row, target) in fixture.targets.iter().copied().enumerate() {
        let prediction = regressor.predict(features.get(row));
        let absolute_error = (prediction - target).abs();
        total_absolute_error += absolute_error;
        if fixture.sample_rows.contains(&row) {
            sample_predictions.push(SamplePrediction {
                row,
                target,
                prediction,
                absolute_error,
            });
        }
    }

    LinearDemoReport {
        experiment,
        fixture: fixture.report(),
        samples: features.size(),
        feature_dimension: regressor.data().feature_dimension(),
        mean_absolute_error: total_absolute_error / features.size() as f64,
        sample_predictions,
    }
}

fn logistic_fixture() -> ClassificationFixture {
    serde_json::from_str(LOGISTIC_SEPARATED_FIXTURE)
        .expect("logistic-separated fixture should be valid JSON")
}

fn logistic_three_class_fixture() -> ClassificationFixture {
    serde_json::from_str(LOGISTIC_THREE_CLASS_FIXTURE)
        .expect("logistic-three-class fixture should be valid JSON")
}

fn logistic_three_class_overlap_fixture() -> ClassificationFixture {
    serde_json::from_str(LOGISTIC_THREE_CLASS_OVERLAP_FIXTURE)
        .expect("logistic-three-class-overlap fixture should be valid JSON")
}

fn logistic_three_class_large_fixture() -> ClassificationFixture {
    serde_json::from_str(LOGISTIC_THREE_CLASS_LARGE_FIXTURE)
        .expect("logistic-three-class-large fixture should be valid JSON")
}

fn logistic_large_fixture() -> ClassificationFixture {
    serde_json::from_str(LOGISTIC_SEPARATED_LARGE_FIXTURE)
        .expect("logistic-separated-large fixture should be valid JSON")
}

fn linear_fixture() -> RegressionFixture {
    serde_json::from_str(LINEAR_AFFINE_FIXTURE).expect("linear-affine fixture should be valid JSON")
}

fn linear_large_fixture() -> RegressionFixture {
    serde_json::from_str(LINEAR_AFFINE_LARGE_FIXTURE)
        .expect("linear-affine-large fixture should be valid JSON")
}

fn semantic_logistic_fixture() -> ClassificationFixture {
    serde_json::from_str(SEMANTIC_LOGISTIC_PROTOTYPE_FIXTURE)
        .expect("semantic-logistic-prototype fixture should be valid JSON")
}

impl ClassificationFixture {
    fn report(&self) -> FixtureReport {
        FixtureReport {
            id: self.id.clone(),
            name: self.name.clone(),
            task: self.task.clone(),
            feature_names: self.feature_names.clone(),
            seed: self.seed,
            metadata: self.metadata.clone(),
        }
    }
}

impl RegressionFixture {
    fn report(&self) -> FixtureReport {
        FixtureReport {
            id: self.id.clone(),
            name: self.name.clone(),
            task: self.task.clone(),
            feature_names: self.feature_names.clone(),
            seed: self.seed,
            metadata: self.metadata.clone(),
        }
    }
}

fn split_train_test_indices(
    total_examples: usize,
    test_fraction: f64,
    seed: u64,
) -> (Vec<u64>, Vec<u64>) {
    let splitter = NodeSplitter::new(
        Concurrency::single_threaded(),
        total_examples,
        Arc::new(|index| index as i64),
        Arc::new(|original_id| original_id as usize),
    );

    let mut progress_tracker = NoopProgressTracker;
    let splits = splitter.split(test_fraction, 1, Some(seed), &mut progress_tracker);
    let outer_split = splits.outer_split();

    let train = outer_split
        .train_set()
        .iter()
        .map(|&index| index as u64)
        .collect::<Vec<u64>>();
    let test = outer_split
        .test_set()
        .iter()
        .map(|&index| index as u64)
        .collect::<Vec<u64>>();

    (train, test)
}

fn project_semantic_features(fixture: &ClassificationFixture) -> Vec<Vec<f64>> {
    fixture
        .features
        .iter()
        .enumerate()
        .map(|(row, base)| {
            let mut projected = base.clone();
            if let Some(semantic) = fixture.semantic_rows.iter().find(|entry| entry.row == row) {
                projected.push(semantic.polarity as f64);
                projected.push(semantic.confidence);
                projected.push(semantic.relation_tags.len() as f64);
                projected.push(semantic.provenance_span.len() as f64);
            } else {
                projected.extend_from_slice(&[0.0, 0.0, 0.0, 0.0]);
            }
            projected
        })
        .collect::<Vec<Vec<f64>>>()
}

fn logistic_benchmark_verbose(
    fixture: &ClassificationFixture,
    train_set: &[u64],
    test_set: &[u64],
    number_of_classes: usize,
) -> BenchmarkVerboseReport {
    let features = DenseFeatures::new(fixture.features.clone());
    let labels = HugeIntArray::from_vec(fixture.labels.clone());

    let checkpoints = [20usize, 40, 80, 160];
    let progression = checkpoints
        .iter()
        .map(|&max_epochs| {
            let trainer = LogisticRegressionTrainer::new(
                LogisticRegressionTrainConfig {
                    batch_size: 64,
                    learning_rate: 0.03,
                    max_epochs,
                    tolerance: 1e-6,
                    ..Default::default()
                },
                number_of_classes,
                false,
                Arc::new(RwLock::new(false)),
                1,
            );
            let classifier = trainer.train(&features, &labels, &Arc::new(train_set.to_vec()));
            let mut correct = 0usize;
            for &index in test_set {
                let row = index as usize;
                let predicted = argmax(&classifier.predict_probabilities(features.get(row)));
                if predicted as i32 == fixture.labels[row] {
                    correct += 1;
                }
            }

            MetricCheckpoint {
                max_epochs,
                metric_name: "accuracy",
                metric_value: correct as f64 / test_set.len() as f64,
            }
        })
        .collect::<Vec<MetricCheckpoint>>();

    let final_trainer = LogisticRegressionTrainer::new(
        LogisticRegressionTrainConfig {
            batch_size: 64,
            learning_rate: 0.03,
            max_epochs: 160,
            tolerance: 1e-6,
            ..Default::default()
        },
        number_of_classes,
        false,
        Arc::new(RwLock::new(false)),
        1,
    );
    let final_model = final_trainer.train(&features, &labels, &Arc::new(train_set.to_vec()));
    let sample_observations = test_set
        .iter()
        .take(8)
        .map(|&index| {
            let row = index as usize;
            let predicted = argmax(&final_model.predict_probabilities(features.get(row)));
            let expected = fixture.labels[row] as f64;
            let predicted_value = predicted as f64;
            BenchmarkObservation {
                row,
                expected,
                predicted: predicted_value,
                absolute_error: (predicted_value - expected).abs(),
            }
        })
        .collect::<Vec<BenchmarkObservation>>();

    BenchmarkVerboseReport {
        progression,
        sample_observations,
    }
}

fn linear_benchmark_verbose(
    fixture: &RegressionFixture,
    train_set: &[u64],
    test_set: &[u64],
) -> BenchmarkVerboseReport {
    let features = DenseFeatures::new(fixture.features.clone());
    let targets = HugeDoubleArray::from_vec(fixture.targets.clone());

    let checkpoints = [40usize, 80, 160, 220];
    let progression = checkpoints
        .iter()
        .map(|&max_epochs| {
            let gradient = GradientDescentConfig::builder()
                .batch_size(64)
                .learning_rate(0.02)
                .max_epochs(max_epochs)
                .tolerance(1e-7)
                .build()
                .expect("linear verbose gradient config should be valid");

            let trainer = LinearRegressionTrainer::new(
                1,
                LinearRegressionTrainConfig::new(gradient, 0.0),
                Arc::new(RwLock::new(false)),
            );
            let regressor = trainer.train(&features, &targets, &Arc::new(train_set.to_vec()));
            let mut absolute_error = 0.0;
            for &index in test_set {
                let row = index as usize;
                absolute_error +=
                    (regressor.predict(features.get(row)) - fixture.targets[row]).abs();
            }

            MetricCheckpoint {
                max_epochs,
                metric_name: "mean_absolute_error",
                metric_value: absolute_error / test_set.len() as f64,
            }
        })
        .collect::<Vec<MetricCheckpoint>>();

    let final_gradient = GradientDescentConfig::builder()
        .batch_size(64)
        .learning_rate(0.02)
        .max_epochs(220)
        .tolerance(1e-7)
        .build()
        .expect("linear verbose final gradient config should be valid");
    let final_trainer = LinearRegressionTrainer::new(
        1,
        LinearRegressionTrainConfig::new(final_gradient, 0.0),
        Arc::new(RwLock::new(false)),
    );
    let final_model = final_trainer.train(&features, &targets, &Arc::new(train_set.to_vec()));
    let sample_observations = test_set
        .iter()
        .take(8)
        .map(|&index| {
            let row = index as usize;
            let expected = fixture.targets[row];
            let predicted = final_model.predict(features.get(row));
            BenchmarkObservation {
                row,
                expected,
                predicted,
                absolute_error: (predicted - expected).abs(),
            }
        })
        .collect::<Vec<BenchmarkObservation>>();

    BenchmarkVerboseReport {
        progression,
        sample_observations,
    }
}

fn argmax(values: &[f64]) -> usize {
    values
        .iter()
        .enumerate()
        .max_by(|(_, left), (_, right)| left.total_cmp(right))
        .map(|(index, _)| index)
        .unwrap_or(0)
}

fn sample_i64(values: &[i64], limit: usize) -> Vec<i64> {
    values.iter().take(limit).copied().collect::<Vec<i64>>()
}

fn score_metric<M>(metric_computer: &ClassificationMetricComputer, metric: &M) -> MetricScore
where
    M: ClassificationMetric,
{
    MetricScore {
        name: metric.name().to_string(),
        value: metric_computer.score(metric),
    }
}

fn confusion_cells(labels: &HugeLongArray, predictions: &HugeLongArray) -> Vec<ConfusionCell> {
    let mut cells = Vec::new();

    for index in 0..labels.size() {
        let actual = labels.get(index);
        let predicted = predictions.get(index);
        if let Some(cell) = cells
            .iter_mut()
            .find(|cell: &&mut ConfusionCell| cell.actual == actual && cell.predicted == predicted)
        {
            cell.count += 1;
        } else {
            cells.push(ConfusionCell {
                actual,
                predicted,
                count: 1,
            });
        }
    }

    cells.sort_by_key(|cell| (cell.actual, cell.predicted));
    cells
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logistic_demo_trains_classifier() {
        let report = run_logistic_demo();

        assert_eq!(report.fixture.id, "logistic-separated");
        assert_eq!(report.samples, 8);
        assert_eq!(report.feature_dimension, 2);
        assert_eq!(report.number_of_classes, 2);
        assert!(report.accuracy >= 0.75, "accuracy={}", report.accuracy);
        assert_eq!(report.sample_probabilities.len(), 4);
    }

    #[test]
    fn logistic_three_class_demo_trains_classifier() {
        let report = run_logistic_three_class_demo();

        assert_eq!(report.fixture.id, "logistic-three-class");
        assert_eq!(report.samples, 12);
        assert_eq!(report.feature_dimension, 2);
        assert_eq!(report.number_of_classes, 3);
        assert!(report.accuracy >= 0.75, "accuracy={}", report.accuracy);
        assert_eq!(report.sample_probabilities.len(), 6);
    }

    #[test]
    fn logistic_three_class_overlap_demo_trains_classifier() {
        let report = run_logistic_three_class_overlap_demo();

        assert_eq!(report.fixture.id, "logistic-three-class-overlap");
        assert_eq!(report.samples, 60);
        assert_eq!(report.feature_dimension, 2);
        assert_eq!(report.number_of_classes, 3);
        assert!(report.accuracy >= 0.65, "accuracy={}", report.accuracy);
        assert_eq!(report.sample_probabilities.len(), 7);
    }

    #[test]
    fn logistic_three_class_large_demo_trains_classifier() {
        let report = run_logistic_three_class_large_demo();

        assert_eq!(report.fixture.id, "logistic-three-class-large");
        assert_eq!(report.samples, 180);
        assert_eq!(report.feature_dimension, 2);
        assert_eq!(report.number_of_classes, 3);
        assert!(report.accuracy >= 0.7, "accuracy={}", report.accuracy);
        assert_eq!(report.sample_probabilities.len(), 8);
    }

    #[test]
    fn mlp_demo_trains_classifier() {
        let report = run_mlp_demo();

        assert_eq!(report.fixture.id, "logistic-separated");
        assert_eq!(report.samples, 8);
        assert_eq!(report.feature_dimension, 2);
        assert_eq!(report.number_of_classes, 2);
        assert!(report.accuracy >= 0.75, "accuracy={}", report.accuracy);
        assert_eq!(report.sample_probabilities.len(), 4);
    }

    #[test]
    fn mlp_three_class_demo_trains_classifier() {
        let report = run_mlp_three_class_demo();

        assert_eq!(report.fixture.id, "logistic-three-class");
        assert_eq!(report.samples, 12);
        assert_eq!(report.feature_dimension, 2);
        assert_eq!(report.number_of_classes, 3);
        assert!(report.accuracy >= 0.66, "accuracy={}", report.accuracy);
        assert_eq!(report.sample_probabilities.len(), 6);
    }

    #[test]
    fn mlp_three_class_overlap_demo_trains_classifier() {
        let report = run_mlp_three_class_overlap_demo();

        assert_eq!(report.fixture.id, "logistic-three-class-overlap");
        assert_eq!(report.samples, 60);
        assert_eq!(report.feature_dimension, 2);
        assert_eq!(report.number_of_classes, 3);
        assert!(report.accuracy >= 0.75, "accuracy={}", report.accuracy);
        assert_eq!(report.sample_probabilities.len(), 7);
    }

    #[test]
    fn mlp_three_class_large_demo_trains_classifier() {
        let report = run_mlp_three_class_large_demo();

        assert_eq!(report.fixture.id, "logistic-three-class-large");
        assert_eq!(report.samples, 180);
        assert_eq!(report.feature_dimension, 2);
        assert_eq!(report.number_of_classes, 3);
        assert!(report.accuracy >= 0.75, "accuracy={}", report.accuracy);
        assert_eq!(report.sample_probabilities.len(), 8);
    }

    #[test]
    fn logistic_vs_mlp_overlap_comparison_reports_both_models() {
        let report = run_logistic_vs_mlp_overlap_comparison();

        assert_eq!(report.fixture.id, "logistic-three-class-overlap");
        assert_eq!(report.samples, 60);
        assert_eq!(report.train_size + report.test_size, report.samples);
        assert_eq!(report.split_seed, 4242);
        assert_eq!(report.sample_rows.len(), 7);
        assert!((0.0..=1.0).contains(&report.logistic.accuracy));
        assert!((0.0..=1.0).contains(&report.mlp.accuracy));
    }

    #[test]
    fn linear_demo_trains_regressor() {
        let report = run_linear_demo();

        assert_eq!(report.fixture.id, "linear-affine");
        assert_eq!(report.samples, 8);
        assert_eq!(report.feature_dimension, 2);
        assert!(
            report.mean_absolute_error < 1.0,
            "mean_absolute_error={}",
            report.mean_absolute_error
        );
        assert_eq!(report.sample_predictions.len(), 4);
    }

    #[test]
    fn sweeps_report_multiple_runs() {
        assert_eq!(run_logistic_sweep().runs.len(), 4);
        assert_eq!(run_linear_sweep().runs.len(), 4);
        assert_eq!(run_mlp_three_class_sweep().runs.len(), 4);
    }

    #[test]
    fn node_oriented_previews_execute() {
        let classification = run_node_classification_preview();
        let metrics = run_node_classification_metrics();
        let split = run_node_prediction_split_preview();
        let regression = run_node_regression_preview();
        let regression_metrics = run_node_regression_metrics();

        assert_eq!(classification.samples, 8);
        assert_eq!(classification.number_of_classes, 2);
        assert!(classification.global_accuracy >= 0.75);
        assert!(!classification.sample_probabilities.is_empty());

        assert_eq!(metrics.samples, 8);
        assert!(metrics.metrics.len() >= 5);
        assert!(!metrics.confusion.is_empty());

        assert_eq!(split.total_examples, 8);
        assert_eq!(split.train_size + split.test_size, 8);

        assert_eq!(regression.samples, 8);
        assert!(regression.mean_absolute_error < 1.0);

        assert_eq!(regression_metrics.samples, 8);
        assert_eq!(regression_metrics.metrics.len(), 3);
        assert_eq!(regression_metrics.residuals_sample.len(), 4);
    }

    #[test]
    fn large_benchmarks_are_reproducible() {
        let logistic_one = run_large_logistic_benchmark();
        let logistic_two = run_large_logistic_benchmark();
        let linear_one = run_large_linear_benchmark();
        let linear_two = run_large_linear_benchmark();

        assert_eq!(logistic_one.fixture.id, "logistic-separated-large");
        assert_eq!(linear_one.fixture.id, "linear-affine-large");
        assert_eq!(logistic_one.samples, 1024);
        assert_eq!(linear_one.samples, 1024);
        assert_eq!(logistic_one.split_seed, logistic_two.split_seed);
        assert_eq!(linear_one.split_seed, linear_two.split_seed);
        assert!((logistic_one.metric_value - logistic_two.metric_value).abs() < 1e-12);
        assert!((linear_one.metric_value - linear_two.metric_value).abs() < 1e-12);
    }

    #[test]
    fn semantic_projection_preview_reports_shape() {
        let report = run_semantic_projection_preview();

        assert_eq!(report.fixture.id, "semantic-logistic-prototype");
        assert_eq!(report.samples, 10);
        assert_eq!(report.semantic_rows, 10);
        assert!(report.projection_dimension > 2);
        assert!(!report.sample_rows.is_empty());
        assert!(report.accuracy >= 0.7, "accuracy={}", report.accuracy);
    }

    #[test]
    fn large_benchmark_verbose_contains_progression() {
        let linear_report = run_large_linear_benchmark_with_verbose(true);
        let logistic_report = run_large_logistic_benchmark_with_verbose(true);

        assert!(linear_report.verbose.is_some());
        assert!(logistic_report.verbose.is_some());

        let linear_verbose = linear_report
            .verbose
            .expect("linear verbose report should exist");
        let logistic_verbose = logistic_report
            .verbose
            .expect("logistic verbose report should exist");

        assert_eq!(linear_verbose.progression.len(), 4);
        assert_eq!(logistic_verbose.progression.len(), 4);
        assert!(!linear_verbose.sample_observations.is_empty());
        assert!(!logistic_verbose.sample_observations.is_empty());
    }

    #[test]
    fn deeper_three_class_large_fixture_loads() {
        let fixture = logistic_three_class_large_fixture();
        assert_eq!(fixture.id, "logistic-three-class-large");
        assert_eq!(fixture.features.len(), 180);
        assert_eq!(fixture.labels.len(), 180);
        assert_eq!(fixture.sample_rows.len(), 8);
    }
}
