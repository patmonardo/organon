use crate::ml::models::svm::PlattScaler;
use crate::ml::models::svm::SVMKernelType;
use crate::ml::models::BaseModelData;
use crate::ml::models::ClassifierData;
use crate::ml::models::TrainingMethod;

#[derive(Clone, Debug)]
pub struct SVMOneVsRestModel {
    pub support_vectors: Vec<Vec<f64>>,
    pub support_alphas: Vec<f64>,
    pub support_labels: Vec<f64>,
    pub bias: f64,
    pub kernel: SVMKernelType,
    pub platt_scaler: PlattScaler,
}

#[derive(Clone)]
pub struct SVMClassifierData {
    models: Vec<SVMOneVsRestModel>,
    number_of_classes: usize,
    feature_dimension: usize,
}

impl std::fmt::Debug for SVMClassifierData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SVMClassifierData")
            .field("models", &self.models.len())
            .field("number_of_classes", &self.number_of_classes)
            .field("feature_dimension", &self.feature_dimension)
            .finish()
    }
}

impl SVMClassifierData {
    pub fn new(
        models: Vec<SVMOneVsRestModel>,
        number_of_classes: usize,
        feature_dimension: usize,
    ) -> Self {
        Self {
            models,
            number_of_classes,
            feature_dimension,
        }
    }

    pub fn models(&self) -> &[SVMOneVsRestModel] {
        &self.models
    }

    pub fn number_of_classes(&self) -> usize {
        self.number_of_classes
    }

    pub fn feature_dimension(&self) -> usize {
        self.feature_dimension
    }
}

impl BaseModelData for SVMClassifierData {
    fn trainer_method(&self) -> TrainingMethod {
        TrainingMethod::SVMClassification
    }

    fn feature_dimension(&self) -> usize {
        self.feature_dimension
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ClassifierData for SVMClassifierData {
    fn number_of_classes(&self) -> usize {
        self.number_of_classes
    }
}
