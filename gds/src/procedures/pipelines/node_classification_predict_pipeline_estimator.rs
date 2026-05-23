use crate::mem::{MemoryEstimation, MemoryEstimations};
use crate::ml::models::ClassifierData;
use crate::ml::node_classification::estimate_predict_memory_with_derived_batch_size;

use super::node_classification_predict_pipeline_constants::MIN_BATCH_SIZE;

pub struct NodeClassificationPredictPipelineEstimator;

impl NodeClassificationPredictPipelineEstimator {
    pub fn new() -> Self {
        Self
    }

    pub fn estimate(
        &self,
        model_data: &dyn ClassifierData,
        class_count: usize,
        include_predicted_probabilities: bool,
    ) -> Box<dyn MemoryEstimation> {
        let feature_count = model_data.feature_dimension();

        let prediction_estimation = estimate_predict_memory_with_derived_batch_size(
            model_data.trainer_method(),
            include_predicted_probabilities,
            MIN_BATCH_SIZE,
            feature_count,
            class_count,
            false,
        );

        MemoryEstimations::builder("Node Classification Predict Pipeline")
            .add_as("Pipeline Predict", prediction_estimation)
            .build()
    }

    pub fn estimate_with_fallback(
        &self,
        model_data: Option<&dyn ClassifierData>,
        class_count: usize,
        include_predicted_probabilities: bool,
    ) -> Box<dyn MemoryEstimation> {
        match model_data {
            Some(data) => self.estimate(data, class_count, include_predicted_probabilities),
            None => MemoryEstimations::empty(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::any::Any;

    use crate::ml::models::base::{BaseModelData, ClassifierData};
    use crate::ml::models::training_method::TrainingMethod;

    use super::NodeClassificationPredictPipelineEstimator;

    #[derive(Debug)]
    struct TestClassifierData;

    impl BaseModelData for TestClassifierData {
        fn trainer_method(&self) -> TrainingMethod {
            TrainingMethod::LogisticRegression
        }

        fn feature_dimension(&self) -> usize {
            2
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl ClassifierData for TestClassifierData {
        fn number_of_classes(&self) -> usize {
            3
        }
    }

    #[test]
    fn test_estimate_uses_pipeline_predict_description() {
        let estimator = NodeClassificationPredictPipelineEstimator::new();
        let estimation = estimator.estimate(&TestClassifierData, 3, true);

        assert_eq!(
            estimation.description(),
            "Node Classification Predict Pipeline"
        );
    }
}
