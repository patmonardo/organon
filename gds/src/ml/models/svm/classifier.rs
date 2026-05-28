use crate::ml::core::tensor::Matrix;
use crate::ml::models::svm::compute;
use crate::ml::models::svm::SVMClassifierData;
use crate::ml::models::svm::SVMOneVsRestModel;
use crate::ml::models::Classifier;
use crate::ml::models::ClassifierData;
use crate::ml::models::Features;

#[derive(Clone)]
pub struct SVMClassifier {
    data: SVMClassifierData,
}

impl std::fmt::Debug for SVMClassifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SVMClassifier")
            .field("number_of_classes", &self.data.number_of_classes())
            .field("feature_dimension", &self.data.feature_dimension())
            .finish()
    }
}

impl SVMClassifier {
    pub fn new(data: SVMClassifierData) -> Self {
        Self { data }
    }

    pub fn decision_value(model: &SVMOneVsRestModel, features: &[f64]) -> f64 {
        let weighted_kernel_sum = model
            .support_vectors
            .iter()
            .zip(model.support_alphas.iter())
            .zip(model.support_labels.iter())
            .map(|((support_vector, alpha), label)| {
                alpha * label * compute(&model.kernel, support_vector, features)
            })
            .sum::<f64>();
        weighted_kernel_sum + model.bias
    }

    fn normalize(probabilities: &[f64]) -> Vec<f64> {
        let sum = probabilities.iter().sum::<f64>();

        if sum == 0.0 || !sum.is_finite() {
            return vec![1.0 / probabilities.len() as f64; probabilities.len()];
        }

        probabilities
            .iter()
            .map(|probability| probability / sum)
            .collect()
    }
}

impl Classifier for SVMClassifier {
    fn data(&self) -> &dyn ClassifierData {
        &self.data
    }

    fn predict_probabilities(&self, features: &[f64]) -> Vec<f64> {
        let probabilities = self
            .data
            .models()
            .iter()
            .map(|model| {
                model
                    .platt_scaler
                    .calibrate(Self::decision_value(model, features))
            })
            .collect::<Vec<f64>>();

        Self::normalize(&probabilities)
    }

    fn predict_probabilities_batch(&self, batch: &[usize], features: &dyn Features) -> Matrix {
        let mut flat = Vec::with_capacity(batch.len() * self.data.number_of_classes());
        for &index in batch {
            let probabilities = self.predict_probabilities(features.get(index));
            flat.extend(probabilities);
        }

        Matrix::new(flat, batch.len(), self.data.number_of_classes())
    }
}
