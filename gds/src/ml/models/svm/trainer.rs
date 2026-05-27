use crate::collections::HugeIntArray;
use crate::ml::models::svm::compute;
use crate::ml::models::svm::PlattScaler;
use crate::ml::models::svm::SVMClassifier;
use crate::ml::models::svm::SVMClassifierData;
use crate::ml::models::svm::SVMClassifierTrainConfig;
use crate::ml::models::svm::SVMKernelType;
use crate::ml::models::svm::SVMOneVsRestModel;
use crate::ml::models::Classifier;
use crate::ml::models::ClassifierTrainer;
use crate::ml::models::Features;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct SVMClassifierTrainer {
    number_of_classes: usize,
    config: SVMClassifierTrainConfig,
}

impl SVMClassifierTrainer {
    pub fn new(number_of_classes: usize, config: SVMClassifierTrainConfig) -> Self {
        Self {
            number_of_classes,
            config,
        }
    }
}

impl ClassifierTrainer for SVMClassifierTrainer {
    fn train(
        &self,
        features: &dyn Features,
        labels: &HugeIntArray,
        train_set: &Arc<Vec<u64>>,
    ) -> Box<dyn Classifier> {
        let train_indices = train_set
            .iter()
            .map(|&index| index as usize)
            .collect::<Vec<usize>>();

        let train_features = train_indices
            .iter()
            .map(|&index| features.get(index).to_vec())
            .collect::<Vec<Vec<f64>>>();

        let models = (0..self.number_of_classes)
            .map(|class_id| {
                let binary_labels = train_indices
                    .iter()
                    .map(|&index| {
                        if labels.get(index) == class_id as i32 {
                            1.0
                        } else {
                            -1.0
                        }
                    })
                    .collect::<Vec<f64>>();

                train_binary_svm(&train_features, &binary_labels, &self.config)
            })
            .collect::<Vec<SVMOneVsRestModel>>();

        let data =
            SVMClassifierData::new(models, self.number_of_classes, features.feature_dimension());

        Box::new(SVMClassifier::new(data))
    }
}

fn train_binary_svm(
    train_features: &[Vec<f64>],
    binary_labels: &[f64],
    config: &SVMClassifierTrainConfig,
) -> SVMOneVsRestModel {
    let sample_count = train_features.len();
    let positive_count = binary_labels.iter().filter(|&&label| label > 0.0).count();

    if sample_count < 2 || positive_count == 0 || positive_count == sample_count {
        return SVMOneVsRestModel {
            support_vectors: Vec::new(),
            support_alphas: Vec::new(),
            support_labels: Vec::new(),
            bias: if positive_count > 0 { 1.0 } else { -1.0 },
            kernel: config.kernel.clone(),
            platt_scaler: PlattScaler::default(),
        };
    }

    let kernel = config.kernel.clone();
    let kernel_matrix = precompute_kernel_matrix(train_features, &kernel);

    let mut alphas = vec![0.0; sample_count];
    let mut bias = 0.0;
    let mut passes_without_change = 0usize;
    let mut iterations = 0usize;

    while passes_without_change < config.max_passes && iterations < config.max_iterations {
        let mut alpha_updates = 0usize;

        for i in 0..sample_count {
            let error_i =
                decision_at(i, &alphas, binary_labels, bias, &kernel_matrix) - binary_labels[i];

            let violates_kkt = (binary_labels[i] * error_i < -config.tolerance
                && alphas[i] < config.c)
                || (binary_labels[i] * error_i > config.tolerance && alphas[i] > 0.0);

            if !violates_kkt {
                continue;
            }

            let j = select_pair_index(i, sample_count);
            let error_j =
                decision_at(j, &alphas, binary_labels, bias, &kernel_matrix) - binary_labels[j];

            let alpha_i_old = alphas[i];
            let alpha_j_old = alphas[j];

            let (lower, upper) = if binary_labels[i] != binary_labels[j] {
                (
                    (alpha_j_old - alpha_i_old).max(0.0),
                    (config.c + alpha_j_old - alpha_i_old).min(config.c),
                )
            } else {
                (
                    (alpha_i_old + alpha_j_old - config.c).max(0.0),
                    (alpha_i_old + alpha_j_old).min(config.c),
                )
            };

            if (upper - lower).abs() < 1e-12 {
                continue;
            }

            let eta = 2.0 * kernel_matrix[i][j] - kernel_matrix[i][i] - kernel_matrix[j][j];
            if eta >= 0.0 {
                continue;
            }

            let mut alpha_j_new = alpha_j_old - binary_labels[j] * (error_i - error_j) / eta;
            alpha_j_new = alpha_j_new.clamp(lower, upper);

            if (alpha_j_new - alpha_j_old).abs() < 1e-8 {
                continue;
            }

            let alpha_i_new =
                alpha_i_old + binary_labels[i] * binary_labels[j] * (alpha_j_old - alpha_j_new);

            alphas[i] = alpha_i_new;
            alphas[j] = alpha_j_new;

            let b1 = bias
                - error_i
                - binary_labels[i] * (alpha_i_new - alpha_i_old) * kernel_matrix[i][i]
                - binary_labels[j] * (alpha_j_new - alpha_j_old) * kernel_matrix[i][j];
            let b2 = bias
                - error_j
                - binary_labels[i] * (alpha_i_new - alpha_i_old) * kernel_matrix[i][j]
                - binary_labels[j] * (alpha_j_new - alpha_j_old) * kernel_matrix[j][j];

            if alpha_i_new > 0.0 && alpha_i_new < config.c {
                bias = b1;
            } else if alpha_j_new > 0.0 && alpha_j_new < config.c {
                bias = b2;
            } else {
                bias = 0.5 * (b1 + b2);
            }

            alpha_updates += 1;
        }

        if alpha_updates == 0 {
            passes_without_change += 1;
        } else {
            passes_without_change = 0;
        }

        iterations += 1;
    }

    let mut support_vectors = Vec::new();
    let mut support_alphas = Vec::new();
    let mut support_labels = Vec::new();

    for index in 0..sample_count {
        if alphas[index] > 1e-8 {
            support_vectors.push(train_features[index].clone());
            support_alphas.push(alphas[index]);
            support_labels.push(binary_labels[index]);
        }
    }

    let mut model = SVMOneVsRestModel {
        support_vectors,
        support_alphas,
        support_labels,
        bias,
        kernel,
        platt_scaler: PlattScaler::default(),
    };

    let decision_values = train_features
        .iter()
        .map(|feature_row| decision_value(&model, feature_row))
        .collect::<Vec<f64>>();
    model.platt_scaler = PlattScaler::fit(&decision_values, binary_labels);

    model
}

fn precompute_kernel_matrix(features: &[Vec<f64>], kernel: &SVMKernelType) -> Vec<Vec<f64>> {
    let n = features.len();
    let mut matrix = vec![vec![0.0; n]; n];

    for i in 0..n {
        for j in i..n {
            let value = compute(kernel, &features[i], &features[j]);
            matrix[i][j] = value;
            matrix[j][i] = value;
        }
    }

    matrix
}

fn decision_at(
    i: usize,
    alphas: &[f64],
    labels: &[f64],
    bias: f64,
    kernel_matrix: &[Vec<f64>],
) -> f64 {
    let weighted_sum = alphas
        .iter()
        .zip(labels.iter())
        .enumerate()
        .map(|(j, (alpha, label))| alpha * label * kernel_matrix[i][j])
        .sum::<f64>();

    weighted_sum + bias
}

fn select_pair_index(i: usize, sample_count: usize) -> usize {
    if i + 1 < sample_count {
        i + 1
    } else {
        0
    }
}

fn decision_value(model: &SVMOneVsRestModel, features: &[f64]) -> f64 {
    model
        .support_vectors
        .iter()
        .zip(model.support_alphas.iter())
        .zip(model.support_labels.iter())
        .map(|((support_vector, alpha), label)| {
            alpha * label * compute(&model.kernel, support_vector, features)
        })
        .sum::<f64>()
        + model.bias
}
