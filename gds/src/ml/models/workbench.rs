use crate::collections::HugeDoubleArray;
use crate::collections::HugeIntArray;
use crate::ml::gradient_descent::GradientDescentConfig;
use crate::ml::models::features::DenseFeatures;
use crate::ml::models::linear_regression::LinearRegressionTrainConfig;
use crate::ml::models::linear_regression::LinearRegressionTrainer;
use crate::ml::models::logistic_regression::LogisticRegressionTrainConfig;
use crate::ml::models::logistic_regression::LogisticRegressionTrainer;
use crate::ml::models::ClassifierTrainer;
use crate::ml::models::Features;
use crate::ml::models::RegressorTrainer;
use parking_lot::RwLock;
use serde::Serialize;
use std::sync::Arc;

#[derive(Clone, Debug, Serialize)]
pub struct LogisticDemoReport {
    pub name: &'static str,
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
    pub name: &'static str,
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

pub fn available_experiments() -> &'static [&'static str] {
    &["logistic-demo", "linear-demo"]
}

pub fn run_logistic_demo() -> LogisticDemoReport {
    let features = DenseFeatures::new(vec![
        vec![-3.0, -2.0],
        vec![-2.5, -1.7],
        vec![-2.0, -1.4],
        vec![-1.5, -1.1],
        vec![1.5, 1.1],
        vec![2.0, 1.4],
        vec![2.5, 1.7],
        vec![3.0, 2.0],
    ]);
    let labels_vec = vec![0, 0, 0, 0, 1, 1, 1, 1];
    let labels = HugeIntArray::from_vec(labels_vec.clone());
    let train_set = Arc::new((0..features.size() as u64).collect::<Vec<u64>>());

    let config = LogisticRegressionTrainConfig {
        batch_size: 2,
        learning_rate: 0.05,
        max_epochs: 150,
        tolerance: 1e-6,
        ..Default::default()
    };
    let trainer = LogisticRegressionTrainer::new(config, 2, false, Arc::new(RwLock::new(false)), 1);
    let classifier = trainer.train(&features, &labels, &train_set);

    let mut correct = 0usize;
    let mut sample_probabilities = Vec::new();

    for (row, label) in labels_vec.iter().copied().enumerate() {
        let probabilities = classifier.predict_probabilities(features.get(row));
        let predicted_class = argmax(&probabilities);
        if predicted_class as i32 == label {
            correct += 1;
        }
        if row == 0 || row == 3 || row == 4 || row == 7 {
            sample_probabilities.push(SampleProbabilities {
                row,
                label,
                probabilities,
                predicted_class,
            });
        }
    }

    LogisticDemoReport {
        name: "logistic-demo",
        samples: features.size(),
        feature_dimension: classifier.data().feature_dimension(),
        number_of_classes: classifier.number_of_classes(),
        accuracy: correct as f64 / features.size() as f64,
        sample_probabilities,
    }
}

pub fn run_linear_demo() -> LinearDemoReport {
    let features = DenseFeatures::new(vec![
        vec![-3.0, 1.0],
        vec![-2.0, 1.0],
        vec![-1.0, 1.0],
        vec![0.0, 1.0],
        vec![1.0, 1.0],
        vec![2.0, 1.0],
        vec![3.0, 1.0],
        vec![4.0, 1.0],
    ]);
    let targets_vec = vec![-5.0, -3.0, -1.0, 1.0, 3.0, 5.0, 7.0, 9.0];
    let targets = HugeDoubleArray::from_vec(targets_vec.clone());
    let train_set = Arc::new((0..features.size() as u64).collect::<Vec<u64>>());

    let gradient = GradientDescentConfig::builder()
        .batch_size(2)
        .learning_rate(0.03)
        .max_epochs(300)
        .tolerance(1e-7)
        .build()
        .expect("linear demo gradient config should be valid");
    let config = LinearRegressionTrainConfig::new(gradient, 0.0);
    let trainer = LinearRegressionTrainer::new(1, config, Arc::new(RwLock::new(false)));
    let regressor = trainer.train(&features, &targets, &train_set);

    let mut total_absolute_error = 0.0;
    let mut sample_predictions = Vec::new();

    for (row, target) in targets_vec.iter().copied().enumerate() {
        let prediction = regressor.predict(features.get(row));
        let absolute_error = (prediction - target).abs();
        total_absolute_error += absolute_error;
        if row == 0 || row == 3 || row == 4 || row == 7 {
            sample_predictions.push(SamplePrediction {
                row,
                target,
                prediction,
                absolute_error,
            });
        }
    }

    LinearDemoReport {
        name: "linear-demo",
        samples: features.size(),
        feature_dimension: regressor.data().feature_dimension(),
        mean_absolute_error: total_absolute_error / features.size() as f64,
        sample_predictions,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logistic_demo_trains_classifier() {
        let report = run_logistic_demo();

        assert_eq!(report.samples, 8);
        assert_eq!(report.feature_dimension, 2);
        assert_eq!(report.number_of_classes, 2);
        assert!(report.accuracy >= 0.75, "accuracy={}", report.accuracy);
        assert_eq!(report.sample_probabilities.len(), 4);
    }

    #[test]
    fn linear_demo_trains_regressor() {
        let report = run_linear_demo();

        assert_eq!(report.samples, 8);
        assert_eq!(report.feature_dimension, 2);
        assert!(
            report.mean_absolute_error < 1.0,
            "mean_absolute_error={}",
            report.mean_absolute_error
        );
        assert_eq!(report.sample_predictions.len(), 4);
    }
}
