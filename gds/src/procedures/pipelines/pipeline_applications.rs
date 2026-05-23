use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Value;

use crate::mem::MemoryEstimationResult;
use crate::projection::eval::pipeline::link_pipeline::LinkFeatureStepFactory;
use crate::projection::eval::pipeline::link_pipeline::LinkPredictionSplitConfig;
use crate::projection::eval::pipeline::link_pipeline::LinkPredictionTrainingPipeline;
use crate::projection::eval::pipeline::node_pipeline::classification::node_classification_training_pipeline::NodeClassificationTrainingPipeline;
use crate::projection::eval::pipeline::node_pipeline::node_feature_step::NodeFeatureStep;
use crate::projection::eval::pipeline::node_pipeline::node_property_prediction_split_config::NodePropertyPredictionSplitConfig;
use crate::projection::eval::pipeline::node_pipeline::node_property_training_pipeline::NodePropertyTrainingPipeline;
use crate::projection::eval::pipeline::node_pipeline::regression::node_regression_training_pipeline::NodeRegressionTrainingPipeline;
use crate::projection::eval::pipeline::{
    AutoTuningConfig, ExecutableNodePropertyStep, Pipeline, TrainingMethod, TrainingPipeline,
};
use crate::projection::eval::pipeline::{NodePropertyStep, PipelineCatalogEntry};
use crate::types::user::User;

use super::node_pipeline_info_result_transformer::{
    create_node_classification_info_result, create_node_regression_info_result,
};
use super::pipeline_catalog_result_transformer::create_pipeline_catalog_result;
use super::pipeline_info_result_transformer::create_pipeline_info_result;
use super::{PipelineName, PipelineRepository};
use crate::procedures::pipelines::types::{
    AnyMap, LinkPredictionTrainResult, MutateResult, NodeClassificationPipelineTrainResult,
    NodeClassificationStreamResult, NodePipelineInfoResult, NodeRegressionPipelineTrainResult,
    NodeRegressionStreamResult, PipelineCatalogResult, PipelineExistsResult, PipelineInfoResult,
    PredictMutateResult, StreamResult, WriteResult,
};

fn executor_backed_pipeline_application_not_wired<T>(procedure: &str) -> Vec<T> {
    panic!(
        "{procedure} is an executor-backed pipeline procedure and PipelineApplications is not wired with graph/model processing dependencies yet"
    );
}

#[derive(Clone)]
pub struct PipelineApplications {
    user: User,
    pipeline_repository: PipelineRepository,
}

impl PipelineApplications {
    pub fn new(user: User, pipeline_repository: PipelineRepository) -> Self {
        Self {
            user,
            pipeline_repository,
        }
    }

    pub fn create_link_prediction_training_pipeline(
        &self,
        pipeline_name: &PipelineName,
    ) -> PipelineInfoResult {
        let pipeline = self
            .pipeline_repository
            .create_link_prediction_training_pipeline(&self.user, pipeline_name);

        create_pipeline_info_result(pipeline_name.as_str(), pipeline.as_ref())
    }

    pub fn create_node_classification_training_pipeline(
        &self,
        pipeline_name: &PipelineName,
    ) -> NodePipelineInfoResult {
        let pipeline = self
            .pipeline_repository
            .create_node_classification_training_pipeline(&self.user, pipeline_name);

        create_node_classification_info_result(pipeline_name.as_str(), pipeline.as_ref())
    }

    pub fn create_node_regression_training_pipeline(
        &self,
        pipeline_name: &PipelineName,
    ) -> NodePipelineInfoResult {
        let pipeline = self
            .pipeline_repository
            .create_node_regression_training_pipeline(&self.user, pipeline_name);

        create_node_regression_info_result(pipeline_name.as_str(), pipeline.as_ref())
    }

    pub fn add_feature_to_link_prediction_pipeline(
        &self,
        pipeline_name: &PipelineName,
        feature_type: &str,
        configuration: Value,
    ) -> PipelineInfoResult {
        let existing = self
            .pipeline_repository
            .get_link_prediction_training_pipeline(&self.user, pipeline_name);

        let mut next = (*existing).clone();
        let step = LinkFeatureStepFactory::create_from_config(feature_type, &configuration)
            .unwrap_or_else(|e| panic!("{e}"));
        next.add_feature_step(step);

        let next = Arc::new(next);
        self.pipeline_repository
            .replace(&self.user, pipeline_name, Arc::clone(&next));

        create_pipeline_info_result(pipeline_name.as_str(), next.as_ref())
    }

    pub fn add_node_property_to_link_prediction_pipeline(
        &self,
        pipeline_name: &PipelineName,
        task_name: &str,
        procedure_config: HashMap<String, Value>,
    ) -> PipelineInfoResult {
        let existing = self
            .pipeline_repository
            .get_link_prediction_training_pipeline(&self.user, pipeline_name);

        let mut next = (*existing).clone();
        let step = NodePropertyStep::new(task_name.to_string(), procedure_config);
        let step_box: Box<dyn ExecutableNodePropertyStep> = Box::new(step);

        next.validate_unique_mutate_property(step_box.as_ref())
            .unwrap_or_else(|e| panic!("{e}"));
        next.add_node_property_step(step_box);

        let next = Arc::new(next);
        self.pipeline_repository
            .replace(&self.user, pipeline_name, Arc::clone(&next));

        create_pipeline_info_result(pipeline_name.as_str(), next.as_ref())
    }

    pub fn configure_link_prediction_split(
        &self,
        pipeline_name: &PipelineName,
        split_config: LinkPredictionSplitConfig,
    ) -> PipelineInfoResult {
        let existing = self
            .pipeline_repository
            .get_link_prediction_training_pipeline(&self.user, pipeline_name);

        let mut next = (*existing).clone();
        next.set_split_config(split_config);

        let next = Arc::new(next);
        self.pipeline_repository
            .replace(&self.user, pipeline_name, Arc::clone(&next));

        create_pipeline_info_result(pipeline_name.as_str(), next.as_ref())
    }

    pub fn configure_link_prediction_auto_tuning(
        &self,
        pipeline_name: &PipelineName,
        auto_tuning_config: AutoTuningConfig,
    ) -> PipelineInfoResult {
        let existing = self
            .pipeline_repository
            .get_link_prediction_training_pipeline(&self.user, pipeline_name);

        let mut next = (*existing).clone();
        next.set_auto_tuning_config(auto_tuning_config);

        let next = Arc::new(next);
        self.pipeline_repository
            .replace(&self.user, pipeline_name, Arc::clone(&next));

        create_pipeline_info_result(pipeline_name.as_str(), next.as_ref())
    }

    pub fn add_training_method_to_link_prediction_pipeline(
        &self,
        pipeline_name: &PipelineName,
        method: TrainingMethod,
    ) -> PipelineInfoResult {
        let existing = self
            .pipeline_repository
            .get_link_prediction_training_pipeline(&self.user, pipeline_name);

        let mut next = (*existing).clone();
        next.training_parameter_space_mut()
            .entry(method)
            .or_default();

        let next = Arc::new(next);
        self.pipeline_repository
            .replace(&self.user, pipeline_name, Arc::clone(&next));

        create_pipeline_info_result(pipeline_name.as_str(), next.as_ref())
    }

    pub fn add_node_property_to_node_classification_pipeline(
        &self,
        pipeline_name: &PipelineName,
        task_name: &str,
        procedure_config: HashMap<String, Value>,
    ) -> NodePipelineInfoResult {
        let existing = self
            .pipeline_repository
            .get_node_classification_training_pipeline(&self.user, pipeline_name);

        let mut next = (*existing).clone();
        let step = NodePropertyStep::new(task_name.to_string(), procedure_config);
        let step_box: Box<dyn ExecutableNodePropertyStep> = Box::new(step);

        next.validate_unique_mutate_property(step_box.as_ref())
            .unwrap_or_else(|e| panic!("{e}"));
        next.add_node_property_step(step_box);

        let next = Arc::new(next);
        self.pipeline_repository
            .replace(&self.user, pipeline_name, Arc::clone(&next));

        create_node_classification_info_result(pipeline_name.as_str(), next.as_ref())
    }

    pub fn add_node_property_to_node_regression_pipeline(
        &self,
        pipeline_name: &PipelineName,
        task_name: &str,
        procedure_config: HashMap<String, Value>,
    ) -> NodePipelineInfoResult {
        let existing = self
            .pipeline_repository
            .get_node_regression_training_pipeline(&self.user, pipeline_name);

        let mut next = (*existing).clone();
        let step = NodePropertyStep::new(task_name.to_string(), procedure_config);
        let step_box: Box<dyn ExecutableNodePropertyStep> = Box::new(step);

        next.validate_unique_mutate_property(step_box.as_ref())
            .unwrap_or_else(|e| panic!("{e}"));
        next.add_node_property_step(step_box);

        let next = Arc::new(next);
        self.pipeline_repository
            .replace(&self.user, pipeline_name, Arc::clone(&next));

        create_node_regression_info_result(pipeline_name.as_str(), next.as_ref())
    }

    pub fn select_features_for_classification(
        &self,
        pipeline_name: &PipelineName,
        feature_steps: Vec<NodeFeatureStep>,
    ) -> NodePipelineInfoResult {
        let existing = self
            .pipeline_repository
            .get_node_classification_training_pipeline(&self.user, pipeline_name);

        let next = rebuild_node_classification_with_features(existing.as_ref(), feature_steps);
        let next = Arc::new(next);
        self.pipeline_repository
            .replace(&self.user, pipeline_name, Arc::clone(&next));

        create_node_classification_info_result(pipeline_name.as_str(), next.as_ref())
    }

    pub fn select_features_for_regression(
        &self,
        pipeline_name: &PipelineName,
        feature_steps: Vec<NodeFeatureStep>,
    ) -> NodePipelineInfoResult {
        let existing = self
            .pipeline_repository
            .get_node_regression_training_pipeline(&self.user, pipeline_name);

        let next = rebuild_node_regression_with_features(existing.as_ref(), feature_steps);
        let next = Arc::new(next);
        self.pipeline_repository
            .replace(&self.user, pipeline_name, Arc::clone(&next));

        create_node_regression_info_result(pipeline_name.as_str(), next.as_ref())
    }

    pub fn configure_node_property_split(
        &self,
        pipeline_name: &PipelineName,
        split_config: NodePropertyPredictionSplitConfig,
        is_classification: bool,
    ) -> NodePipelineInfoResult {
        if is_classification {
            let existing = self
                .pipeline_repository
                .get_node_classification_training_pipeline(&self.user, pipeline_name);
            let mut next = (*existing).clone();
            next.set_split_config(split_config);
            let next = Arc::new(next);
            self.pipeline_repository
                .replace(&self.user, pipeline_name, Arc::clone(&next));
            create_node_classification_info_result(pipeline_name.as_str(), next.as_ref())
        } else {
            let existing = self
                .pipeline_repository
                .get_node_regression_training_pipeline(&self.user, pipeline_name);
            let mut next = (*existing).clone();
            next.set_split_config(split_config);
            let next = Arc::new(next);
            self.pipeline_repository
                .replace(&self.user, pipeline_name, Arc::clone(&next));
            create_node_regression_info_result(pipeline_name.as_str(), next.as_ref())
        }
    }

    pub fn configure_auto_tuning(
        &self,
        pipeline_name: &PipelineName,
        auto_tuning_config: AutoTuningConfig,
        is_classification: bool,
    ) -> NodePipelineInfoResult {
        if is_classification {
            let existing = self
                .pipeline_repository
                .get_node_classification_training_pipeline(&self.user, pipeline_name);
            let mut next = (*existing).clone();
            next.set_auto_tuning_config(auto_tuning_config);
            let next = Arc::new(next);
            self.pipeline_repository
                .replace(&self.user, pipeline_name, Arc::clone(&next));
            create_node_classification_info_result(pipeline_name.as_str(), next.as_ref())
        } else {
            let existing = self
                .pipeline_repository
                .get_node_regression_training_pipeline(&self.user, pipeline_name);
            let mut next = (*existing).clone();
            next.set_auto_tuning_config(auto_tuning_config);
            let next = Arc::new(next);
            self.pipeline_repository
                .replace(&self.user, pipeline_name, Arc::clone(&next));
            create_node_regression_info_result(pipeline_name.as_str(), next.as_ref())
        }
    }

    pub fn add_training_method(
        &self,
        pipeline_name: &PipelineName,
        method: TrainingMethod,
        is_classification: bool,
    ) -> NodePipelineInfoResult {
        if is_classification {
            let existing = self
                .pipeline_repository
                .get_node_classification_training_pipeline(&self.user, pipeline_name);
            let mut next = (*existing).clone();
            next.training_parameter_space_mut()
                .entry(method)
                .or_default();
            let next = Arc::new(next);
            self.pipeline_repository
                .replace(&self.user, pipeline_name, Arc::clone(&next));
            create_node_classification_info_result(pipeline_name.as_str(), next.as_ref())
        } else {
            let existing = self
                .pipeline_repository
                .get_node_regression_training_pipeline(&self.user, pipeline_name);
            let mut next = (*existing).clone();
            next.training_parameter_space_mut()
                .entry(method)
                .or_default();
            let next = Arc::new(next);
            self.pipeline_repository
                .replace(&self.user, pipeline_name, Arc::clone(&next));
            create_node_regression_info_result(pipeline_name.as_str(), next.as_ref())
        }
    }

    pub fn link_prediction_mutate(
        &self,
        _graph_name: &str,
        _configuration: AnyMap,
    ) -> Vec<MutateResult> {
        executor_backed_pipeline_application_not_wired("linkPrediction.mutate")
    }

    pub fn link_prediction_mutate_estimate(
        &self,
        _graph_name_or_configuration: Value,
        _raw_configuration: AnyMap,
    ) -> Vec<MemoryEstimationResult> {
        executor_backed_pipeline_application_not_wired("linkPrediction.mutateEstimate")
    }

    pub fn link_prediction_stream(
        &self,
        _graph_name: &str,
        _configuration: AnyMap,
    ) -> Vec<StreamResult> {
        executor_backed_pipeline_application_not_wired("linkPrediction.stream")
    }

    pub fn link_prediction_stream_estimate(
        &self,
        _graph_name_or_configuration: Value,
        _raw_configuration: AnyMap,
    ) -> Vec<MemoryEstimationResult> {
        executor_backed_pipeline_application_not_wired("linkPrediction.streamEstimate")
    }

    pub fn link_prediction_train(
        &self,
        _graph_name: &str,
        _configuration: AnyMap,
    ) -> Vec<LinkPredictionTrainResult> {
        executor_backed_pipeline_application_not_wired("linkPrediction.train")
    }

    pub fn link_prediction_train_estimate(
        &self,
        _graph_name_or_configuration: Value,
        _raw_configuration: AnyMap,
    ) -> Vec<MemoryEstimationResult> {
        executor_backed_pipeline_application_not_wired("linkPrediction.trainEstimate")
    }

    pub fn node_classification_predict_mutate(
        &self,
        _graph_name: &str,
        _configuration: AnyMap,
    ) -> Vec<PredictMutateResult> {
        executor_backed_pipeline_application_not_wired("nodeClassification.mutate")
    }

    pub fn node_classification_predict_mutate_estimate(
        &self,
        _graph_name_or_configuration: Value,
        _raw_configuration: AnyMap,
    ) -> Vec<MemoryEstimationResult> {
        executor_backed_pipeline_application_not_wired("nodeClassification.mutateEstimate")
    }

    pub fn node_classification_predict_stream(
        &self,
        _graph_name: &str,
        _configuration: AnyMap,
    ) -> Vec<NodeClassificationStreamResult> {
        executor_backed_pipeline_application_not_wired("nodeClassification.stream")
    }

    pub fn node_classification_predict_stream_estimate(
        &self,
        _graph_name_or_configuration: Value,
        _raw_configuration: AnyMap,
    ) -> Vec<MemoryEstimationResult> {
        executor_backed_pipeline_application_not_wired("nodeClassification.streamEstimate")
    }

    pub fn node_classification_train(
        &self,
        _graph_name: &str,
        _configuration: AnyMap,
    ) -> Vec<NodeClassificationPipelineTrainResult> {
        executor_backed_pipeline_application_not_wired("nodeClassification.train")
    }

    pub fn node_classification_train_estimate(
        &self,
        _graph_name_or_configuration: Value,
        _raw_configuration: AnyMap,
    ) -> Vec<MemoryEstimationResult> {
        executor_backed_pipeline_application_not_wired("nodeClassification.trainEstimate")
    }

    pub fn node_classification_predict_write(
        &self,
        _graph_name: &str,
        _configuration: AnyMap,
    ) -> Vec<WriteResult> {
        executor_backed_pipeline_application_not_wired("nodeClassification.write")
    }

    pub fn node_classification_predict_write_estimate(
        &self,
        _graph_name_or_configuration: Value,
        _raw_configuration: AnyMap,
    ) -> Vec<MemoryEstimationResult> {
        executor_backed_pipeline_application_not_wired("nodeClassification.writeEstimate")
    }

    pub fn node_regression_predict_mutate(
        &self,
        _graph_name: &str,
        _configuration: AnyMap,
    ) -> Vec<PredictMutateResult> {
        executor_backed_pipeline_application_not_wired("nodeRegression.mutate")
    }

    pub fn node_regression_predict_stream(
        &self,
        _graph_name: &str,
        _configuration: AnyMap,
    ) -> Vec<NodeRegressionStreamResult> {
        executor_backed_pipeline_application_not_wired("nodeRegression.stream")
    }

    pub fn node_regression_train(
        &self,
        _graph_name: &str,
        _configuration: AnyMap,
    ) -> Vec<NodeRegressionPipelineTrainResult> {
        executor_backed_pipeline_application_not_wired("nodeRegression.train")
    }

    pub fn list(&self) -> Vec<PipelineCatalogResult> {
        let mut results: Vec<_> = self
            .pipeline_repository
            .get_all(&self.user)
            .into_iter()
            .map(|entry| pipeline_catalog_entry_to_result(entry))
            .collect();
        results.sort_by(|a, b| a.pipeline_name.cmp(&b.pipeline_name));
        results
    }

    pub fn get_single(&self, pipeline_name: &PipelineName) -> Option<PipelineCatalogResult> {
        self.pipeline_repository
            .try_get_entry(&self.user, pipeline_name)
            .ok()
            .map(pipeline_catalog_entry_to_result)
    }

    pub fn exists(&self, pipeline_name: &PipelineName) -> PipelineExistsResult {
        if !self.pipeline_repository.exists(&self.user, pipeline_name) {
            return PipelineExistsResult::empty(pipeline_name.as_str());
        }

        let entry = self
            .pipeline_repository
            .get_entry(&self.user, pipeline_name);

        PipelineExistsResult {
            pipeline_name: pipeline_name.as_str().to_string(),
            pipeline_type: entry.pipeline_type().to_string(),
            exists: true,
        }
    }

    pub fn drop(&self, pipeline_name: &PipelineName) -> PipelineCatalogResult {
        let entry = self.pipeline_repository.drop(&self.user, pipeline_name);
        pipeline_catalog_entry_to_result(entry)
    }

    pub fn drop_silencing_failure(
        &self,
        pipeline_name: &PipelineName,
    ) -> Option<PipelineCatalogResult> {
        self.pipeline_repository
            .try_drop(&self.user, pipeline_name)
            .ok()
            .map(pipeline_catalog_entry_to_result)
    }
}

fn pipeline_catalog_entry_to_result(entry: PipelineCatalogEntry) -> PipelineCatalogResult {
    match entry.pipeline_as::<LinkPredictionTrainingPipeline>() {
        Some(pipeline) => {
            return create_pipeline_catalog_result(pipeline.as_ref(), entry.pipeline_name());
        }
        None => {}
    }

    if let Some(pipeline) = entry.pipeline_as::<NodeClassificationTrainingPipeline>() {
        return create_pipeline_catalog_result(pipeline.as_ref(), entry.pipeline_name());
    }

    if let Some(pipeline) = entry.pipeline_as::<NodeRegressionTrainingPipeline>() {
        return create_pipeline_catalog_result(pipeline.as_ref(), entry.pipeline_name());
    }

    PipelineCatalogResult {
        pipeline_info: HashMap::new(),
        pipeline_name: entry.pipeline_name().to_string(),
        pipeline_type: entry.pipeline_type().to_string(),
        creation_time: chrono::Utc::now()
            .with_timezone(&chrono::FixedOffset::east_opt(0).expect("UTC offset")),
    }
}

fn rebuild_node_classification_with_features(
    pipeline: &NodeClassificationTrainingPipeline,
    feature_steps: Vec<NodeFeatureStep>,
) -> NodeClassificationTrainingPipeline {
    let mut next = NodeClassificationTrainingPipeline::new();

    for step in pipeline.node_property_steps() {
        next.add_node_property_step(step.clone());
    }

    for step in feature_steps {
        next.add_feature_step(step);
    }

    for (method, configs) in pipeline.training_parameter_space() {
        next.training_parameter_space_mut()
            .insert(*method, configs.iter().cloned().collect());
    }

    next.set_auto_tuning_config(pipeline.auto_tuning_config().clone());
    next.set_split_config(pipeline.split_config().clone());

    next
}

fn rebuild_node_regression_with_features(
    pipeline: &NodeRegressionTrainingPipeline,
    feature_steps: Vec<NodeFeatureStep>,
) -> NodeRegressionTrainingPipeline {
    let mut next = NodeRegressionTrainingPipeline::new();

    for step in pipeline.node_property_steps() {
        next.add_node_property_step(step.clone());
    }

    for step in feature_steps {
        next.add_feature_step(step);
    }

    for (method, configs) in pipeline.training_parameter_space() {
        next.training_parameter_space_mut()
            .insert(*method, configs.iter().cloned().collect());
    }

    next.set_auto_tuning_config(pipeline.auto_tuning_config().clone());
    next.set_split_config(pipeline.split_config().clone());

    next
}
