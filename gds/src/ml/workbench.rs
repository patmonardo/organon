use crate::collections::HugeDoubleArray;
use crate::collections::HugeIntArray;
use crate::collections::HugeLongArray;
use crate::concurrency::Concurrency;
use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::NoopProgressTracker;
use crate::core::utils::progress::TaskProgressTracker;
use crate::ml::gradient_descent::GradientDescentConfig;
use crate::ml::metrics::GlobalAccuracy;
use crate::ml::models::features::DenseFeatures;
use crate::ml::models::linear_regression::LinearRegressionTrainConfig;
use crate::ml::models::linear_regression::LinearRegressionTrainer;
use crate::ml::models::logistic_regression::LogisticRegressionTrainConfig;
use crate::ml::models::logistic_regression::LogisticRegressionTrainer;
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

const LOGISTIC_SEPARATED_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/fixtures/ml-models/logistic-separated.json"
));
const LINEAR_AFFINE_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/fixtures/ml-models/linear-affine.json"
));

#[derive(Clone, Debug, Serialize)]
pub struct FixtureSummary {
    pub id: &'static str,
    pub name: &'static str,
    pub task: &'static str,
    pub source_path: &'static str,
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
pub struct FixtureReport {
    pub id: String,
    pub name: String,
    pub task: String,
    pub feature_names: Vec<String>,
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

#[derive(Clone, Debug, Deserialize)]
struct ClassificationFixture {
    id: String,
    name: String,
    task: String,
    feature_names: Vec<String>,
    features: Vec<Vec<f64>>,
    labels: Vec<i32>,
    sample_rows: Vec<usize>,
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
}

pub fn available_experiments() -> &'static [&'static str] {
    &[
        "fixtures",
        "logistic-demo",
        "linear-demo",
        "logistic-sweep",
        "linear-sweep",
        "node-classification-preview",
        "node-prediction-split-preview",
        "node-regression-preview",
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
            id: "linear-affine",
            name: "Affine Regression Line",
            task: "regression",
            source_path: "gds/fixtures/ml-models/linear-affine.json",
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

fn linear_fixture() -> RegressionFixture {
    serde_json::from_str(LINEAR_AFFINE_FIXTURE).expect("linear-affine fixture should be valid JSON")
}

impl ClassificationFixture {
    fn report(&self) -> FixtureReport {
        FixtureReport {
            id: self.id.clone(),
            name: self.name.clone(),
            task: self.task.clone(),
            feature_names: self.feature_names.clone(),
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
        }
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
    }

    #[test]
    fn node_oriented_previews_execute() {
        let classification = run_node_classification_preview();
        let split = run_node_prediction_split_preview();
        let regression = run_node_regression_preview();

        assert_eq!(classification.samples, 8);
        assert_eq!(classification.number_of_classes, 2);
        assert!(classification.global_accuracy >= 0.75);
        assert!(!classification.sample_probabilities.is_empty());

        assert_eq!(split.total_examples, 8);
        assert_eq!(split.train_size + split.test_size, 8);

        assert_eq!(regression.samples, 8);
        assert!(regression.mean_absolute_error < 1.0);
    }
}
