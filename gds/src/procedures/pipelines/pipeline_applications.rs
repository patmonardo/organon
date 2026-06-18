use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTimings, GraphStoreNodePropertiesWritten, GraphStoreService,
    MutateResultBuilder, NodeProperty, StreamResultBuilder, WriteResultBuilder,
};
use crate::applications::services::logging::Log;
use crate::collections::backends::vec::VecDouble;
use crate::core::graph_dimensions::ConcreteGraphDimensions;
use crate::core::loading::GraphResources;
use crate::ml::metrics::{ClassificationMetricSpecification, RegressionMetric};
use crate::ml::models::Regressor;
use crate::projection::eval::pipeline::LinkFeatureStepFactory;
use crate::projection::eval::pipeline::LinkPredictionSplitConfig;
use crate::projection::eval::pipeline::LinkPredictionTrainingPipeline;
use crate::projection::eval::pipeline::NodeClassificationModelResult;
use crate::projection::eval::pipeline::NodeClassificationPipelineTrainConfig;
use crate::projection::eval::pipeline::NodeClassificationTrainingPipeline;
use crate::projection::eval::pipeline::NodeFeatureStep;
use crate::projection::eval::pipeline::NodePropertyPipelineBaseTrainConfig;
use crate::projection::eval::pipeline::NodePropertyPredictPipeline;
use crate::projection::eval::pipeline::NodePropertyPredictionSplitConfig;
use crate::projection::eval::pipeline::NodePropertyTrainingPipeline;
use crate::projection::eval::pipeline::NodeRegressionPipelineModelInfo;
use crate::projection::eval::pipeline::NodeRegressionPipelineTrainConfig;
use crate::projection::eval::pipeline::NodeRegressionTrainingPipeline;
use crate::projection::eval::pipeline::{
    AutoTuningConfig, ExecutableNodePropertyStep, Pipeline, TrainingMethod, TrainingPipeline,
};
use crate::projection::eval::pipeline::{NodePropertyStep, PipelineCatalogEntry};
use crate::task::concurrency::Concurrency;
use crate::task::memory::{MemoryEstimationResult, MemoryRange, MemoryTree};
use crate::task::runtime::TaskFrame;
use crate::task::runtime::TaskFrameKind;
use crate::task::runtime::TaskFrameStorageBackend;
use crate::types::catalog::{GraphCatalog, InMemoryGraphCatalog};
use crate::types::graph_store::DefaultGraphStore;
use crate::types::graph_store::GraphStore;
use crate::types::properties::node::DefaultDoubleNodePropertyValues;
use crate::types::properties::node::NodePropertyValues;
use crate::types::user::User;
use dyn_clone::clone_box;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

use super::node_classification_predict_computation::NodeClassificationPredictComputation;
use super::node_classification_predict_pipeline_base_config::{
    NodeClassificationPredictPipelineBaseConfig, NodeClassificationPredictPipelineConfig,
};
use super::node_classification_predict_pipeline_mutate_config::NodeClassificationPredictPipelineMutateConfig;
use super::node_classification_predict_pipeline_mutate_or_write_config::NodeClassificationPredictPipelineMutateOrWriteConfig;
use super::node_classification_predict_pipeline_mutate_result_builder::NodeClassificationPredictPipelineMutateResultBuilder;
use super::node_classification_predict_pipeline_stream_config::NodeClassificationPredictPipelineStreamConfig;
use super::node_classification_predict_pipeline_stream_result_builder::NodeClassificationPredictPipelineStreamResultBuilder;
use super::node_classification_predict_pipeline_write_config::NodeClassificationPredictPipelineWriteConfig;
use super::node_classification_predict_pipeline_write_result_builder::NodeClassificationPredictPipelineWriteResultBuilder;
use super::node_classification_train_computation::NodeClassificationTrainComputation;
use super::node_pipeline_info_result_transformer::{
    create_node_classification_info_result, create_node_regression_info_result,
};
use super::node_regression_predict_computation::NodeRegressionPredictComputation;
use super::node_regression_predict_pipeline_base_config::{
    NodeRegressionPredictPipelineBaseConfig, NodeRegressionPredictPipelineConfig,
};
use super::node_regression_predict_pipeline_mutate_config::NodeRegressionPredictPipelineMutateConfig;
use super::node_regression_predict_pipeline_mutate_result_builder::NodeRegressionPredictPipelineMutateResultBuilder;
use super::node_regression_predict_pipeline_stream_result_builder::NodeRegressionPredictPipelineStreamResultBuilder;
use super::node_regression_train_computation::NodeRegressionTrainComputation;
use super::pipeline_catalog_result_transformer::create_pipeline_catalog_result;
use super::pipeline_info_result_transformer::create_pipeline_info_result;
use super::{PipelineName, PipelineRepository};
use crate::procedures::pipelines::predicted_probabilities::as_properties;
use crate::procedures::pipelines::types::{
    AnyMap, LinkPredictionTrainResult, MutateResult, NodeClassificationPipelineTrainResult,
    NodeClassificationStreamResult, NodePipelineInfoResult, NodeRegressionPipelineTrainResult,
    NodeRegressionStreamResult, PipelineCatalogResult, PipelineExistsResult, PipelineInfoResult,
    PredictMutateResult, StreamResult, WriteResult,
};
use crate::procedures::pipelines::NodeClassificationPipelineResult;
use crate::procedures::pipelines::NodeRegressionPipelineResult;

fn executor_backed_pipeline_application_not_wired<T>(procedure: &str) -> Vec<T> {
    panic!(
        "{procedure} is an executor-backed pipeline procedure and PipelineApplications is not wired with graph/model processing dependencies yet"
    );
}

#[derive(Clone)]
pub struct PipelineApplications {
    user: User,
    pipeline_repository: PipelineRepository,
    graph_catalog: Arc<dyn GraphCatalog>,
    node_classification_models:
        Arc<parking_lot::RwLock<HashMap<(String, String), Arc<NodeClassificationModelResult>>>>,
    node_regression_models:
        Arc<parking_lot::RwLock<HashMap<(String, String), Arc<NodeRegressionRuntimeModel>>>>,
}

struct NodeRegressionRuntimeModel {
    regressor: Arc<dyn Regressor>,
    pipeline: NodePropertyPredictPipeline,
    train_config: NodeRegressionPipelineTrainConfig,
}

#[derive(Clone, Copy)]
struct EstimateCoefficients {
    fixed_bytes: usize,
    per_node_bytes: usize,
    per_relationship_bytes: usize,
}

#[derive(Clone, Copy)]
struct PlannedResourceContract {
    concurrency: usize,
    memory_range: MemoryRange,
}

impl PipelineApplications {
    pub fn new(user: User, pipeline_repository: PipelineRepository) -> Self {
        Self::new_with_graph_catalog(
            user,
            pipeline_repository,
            Arc::new(InMemoryGraphCatalog::new()),
        )
    }

    pub fn new_with_graph_catalog(
        user: User,
        pipeline_repository: PipelineRepository,
        graph_catalog: Arc<dyn GraphCatalog>,
    ) -> Self {
        Self {
            user,
            pipeline_repository,
            graph_catalog,
            node_classification_models: Arc::new(parking_lot::RwLock::new(HashMap::new())),
            node_regression_models: Arc::new(parking_lot::RwLock::new(HashMap::new())),
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
        graph_name_or_configuration: Value,
        raw_configuration: AnyMap,
    ) -> Vec<MemoryEstimationResult> {
        self.estimate_pipeline_memory(
            "linkPrediction.mutateEstimate",
            &graph_name_or_configuration,
            &raw_configuration,
            EstimateCoefficients {
                fixed_bytes: 2 * 1024 * 1024,
                per_node_bytes: 48,
                per_relationship_bytes: 24,
            },
            "writeConcurrency",
        )
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
        graph_name_or_configuration: Value,
        raw_configuration: AnyMap,
    ) -> Vec<MemoryEstimationResult> {
        self.estimate_pipeline_memory(
            "linkPrediction.streamEstimate",
            &graph_name_or_configuration,
            &raw_configuration,
            EstimateCoefficients {
                fixed_bytes: 1024 * 1024,
                per_node_bytes: 32,
                per_relationship_bytes: 16,
            },
            "concurrency",
        )
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
        graph_name_or_configuration: Value,
        raw_configuration: AnyMap,
    ) -> Vec<MemoryEstimationResult> {
        self.estimate_pipeline_memory(
            "linkPrediction.trainEstimate",
            &graph_name_or_configuration,
            &raw_configuration,
            EstimateCoefficients {
                fixed_bytes: 4 * 1024 * 1024,
                per_node_bytes: 64,
                per_relationship_bytes: 48,
            },
            "concurrency",
        )
    }

    pub fn node_classification_predict_mutate(
        &self,
        graph_name: &str,
        configuration: AnyMap,
    ) -> Vec<PredictMutateResult> {
        let graph_store = self.graph_store_for(graph_name);
        let mutate_config = parse_node_classification_mutate_config(
            self.user.username().to_string(),
            configuration,
        );
        mutate_config.validate_mutate_properties_differ();

        let trained_model = self
            .node_classification_model_for(mutate_config.model_user(), mutate_config.model_name());

        let predict_config = mutate_config
            .base()
            .base()
            .clone()
            .with_train_config_defaults(trained_model.train_config());

        let computation = NodeClassificationPredictComputation::new(
            predict_config,
            "NodeClassificationPredictPipeline".to_string(),
            trained_model,
        );

        let predict_result = computation
            .compute(Arc::clone(&graph_store))
            .unwrap_or_else(|e| panic!("nodeClassification.mutate failed: {e}"));

        let mut metadata = None;
        self.graph_catalog
            .with_store_mut(graph_name, &mut |graph_store| {
                metadata = Some(apply_node_classification_mutation(
                    graph_store,
                    &mutate_config,
                    &predict_result,
                ));
            })
            .unwrap_or_else(|e| {
                panic!("nodeClassification.mutate failed to update graph store: {e}")
            });
        let metadata = metadata.expect("nodeClassification.mutate metadata should be set");

        let graph_resources = GraphResources::new(self.graph_store_for(graph_name));
        let builder =
            NodeClassificationPredictPipelineMutateResultBuilder::new(mutate_config.clone());

        vec![builder.build(
            &graph_resources,
            &mutate_config,
            Some(predict_result),
            AlgorithmProcessingTimings::unavailable(),
            Some(metadata),
        )]
    }

    pub fn node_classification_predict_mutate_task_frame_plan(
        &self,
        graph_name: &str,
        configuration: &AnyMap,
    ) -> Vec<TaskFrame> {
        let resource_contract = self.node_pipeline_plan_resource_contract(
            graph_name,
            configuration,
            EstimateCoefficients {
                fixed_bytes: 2 * 1024 * 1024,
                per_node_bytes: 56,
                per_relationship_bytes: 20,
            },
            "writeConcurrency",
        );

        self.build_node_pipeline_task_frame_plan(
            "nodeClassification.predict.mutate",
            graph_name,
            &[
                "graph.load".to_string(),
                "pipeline.config.resolve".to_string(),
            ],
            &[
                "pipeline.predict".to_string(),
                "node.classification".to_string(),
            ],
            &[
                "graph.mutate".to_string(),
                "procedure.result.render".to_string(),
            ],
            resource_contract,
        )
    }

    pub fn node_classification_predict_mutate_estimate(
        &self,
        graph_name_or_configuration: Value,
        raw_configuration: AnyMap,
    ) -> Vec<MemoryEstimationResult> {
        self.estimate_pipeline_memory(
            "nodeClassification.mutateEstimate",
            &graph_name_or_configuration,
            &raw_configuration,
            EstimateCoefficients {
                fixed_bytes: 2 * 1024 * 1024,
                per_node_bytes: 56,
                per_relationship_bytes: 20,
            },
            "writeConcurrency",
        )
    }

    pub fn node_classification_predict_stream(
        &self,
        graph_name: &str,
        configuration: AnyMap,
    ) -> Vec<NodeClassificationStreamResult> {
        let graph_store = self.graph_store_for(graph_name);
        let base_config = NodeClassificationPredictPipelineBaseConfig::from_map(
            self.user.username().to_string(),
            configuration,
        );
        let stream_config = NodeClassificationPredictPipelineStreamConfig::new(base_config.clone());
        let trained_model =
            self.node_classification_model_for(base_config.model_user(), base_config.model_name());

        let computation = NodeClassificationPredictComputation::new(
            base_config,
            "NodeClassificationPredictPipeline".to_string(),
            trained_model,
        );

        let predict_result = computation
            .compute(Arc::clone(&graph_store))
            .unwrap_or_else(|e| panic!("nodeClassification.stream failed: {e}"));

        let graph_resources = GraphResources::new(graph_store);
        let builder = NodeClassificationPredictPipelineStreamResultBuilder::new(stream_config);
        builder
            .build(&graph_resources, Some(predict_result))
            .collect()
    }

    pub fn node_classification_predict_stream_estimate(
        &self,
        graph_name_or_configuration: Value,
        raw_configuration: AnyMap,
    ) -> Vec<MemoryEstimationResult> {
        self.estimate_pipeline_memory(
            "nodeClassification.streamEstimate",
            &graph_name_or_configuration,
            &raw_configuration,
            EstimateCoefficients {
                fixed_bytes: 1024 * 1024,
                per_node_bytes: 40,
                per_relationship_bytes: 12,
            },
            "concurrency",
        )
    }

    pub fn node_classification_train(
        &self,
        graph_name: &str,
        configuration: AnyMap,
    ) -> Vec<NodeClassificationPipelineTrainResult> {
        let graph_store = self.graph_store_for(graph_name);
        let model_name = optional_string(&configuration, "modelName")
            .unwrap_or_else(|| required_string(&configuration, "pipeline"));
        let train_config = parse_node_classification_train_config(&self.user, configuration);
        let computation = NodeClassificationTrainComputation::new(
            self.pipeline_repository.clone(),
            train_config,
            self.user.clone(),
        );

        let model_result = computation
            .compute(Arc::clone(&graph_store))
            .unwrap_or_else(|e| panic!("nodeClassification.train failed: {e}"));

        let model_result = Arc::new(model_result);

        self.node_classification_models.write().insert(
            (self.user.username().to_string(), model_name),
            Arc::clone(&model_result),
        );

        render_node_classification_train_result(
            model_result.as_ref(),
            AlgorithmProcessingTimings::unavailable(),
        )
    }

    pub fn node_classification_train_estimate(
        &self,
        graph_name_or_configuration: Value,
        raw_configuration: AnyMap,
    ) -> Vec<MemoryEstimationResult> {
        self.estimate_pipeline_memory(
            "nodeClassification.trainEstimate",
            &graph_name_or_configuration,
            &raw_configuration,
            EstimateCoefficients {
                fixed_bytes: 6 * 1024 * 1024,
                per_node_bytes: 72,
                per_relationship_bytes: 32,
            },
            "concurrency",
        )
    }

    pub fn node_classification_predict_write(
        &self,
        graph_name: &str,
        configuration: AnyMap,
    ) -> Vec<WriteResult> {
        let graph_store = self.graph_store_for(graph_name);
        let write_configuration =
            parse_node_classification_write_config(self.user.username().to_string(), configuration);
        write_configuration.validate_write_properties_differ();

        let trained_model = self.node_classification_model_for(
            write_configuration.model_user(),
            write_configuration.model_name(),
        );

        let predict_config = write_configuration
            .base()
            .base()
            .clone()
            .with_train_config_defaults(trained_model.train_config());

        let computation = NodeClassificationPredictComputation::new(
            predict_config,
            "NodeClassificationPredictPipeline".to_string(),
            trained_model,
        );

        let predict_result = computation
            .compute(Arc::clone(&graph_store))
            .unwrap_or_else(|e| panic!("nodeClassification.write failed: {e}"));

        let node_properties = as_properties(
            Some(&predict_result),
            write_configuration.write_property(),
            write_configuration.predicted_probability_property(),
        );
        let metadata = GraphStoreNodePropertiesWritten(
            node_properties.len() * predict_result.predicted_node_count(),
        );

        let graph_resources = GraphResources::new(graph_store);
        let builder =
            NodeClassificationPredictPipelineWriteResultBuilder::new(write_configuration.clone());

        vec![builder.build(
            &graph_resources,
            &write_configuration,
            Some(predict_result),
            AlgorithmProcessingTimings::unavailable(),
            Some(metadata),
        )]
    }

    pub fn node_classification_predict_write_estimate(
        &self,
        graph_name_or_configuration: Value,
        raw_configuration: AnyMap,
    ) -> Vec<MemoryEstimationResult> {
        self.estimate_pipeline_memory(
            "nodeClassification.writeEstimate",
            &graph_name_or_configuration,
            &raw_configuration,
            EstimateCoefficients {
                fixed_bytes: 2 * 1024 * 1024,
                per_node_bytes: 56,
                per_relationship_bytes: 20,
            },
            "writeConcurrency",
        )
    }

    pub fn node_regression_predict_mutate(
        &self,
        graph_name: &str,
        configuration: AnyMap,
    ) -> Vec<PredictMutateResult> {
        let graph_store = self.graph_store_for(graph_name);
        let base_config = NodeRegressionPredictPipelineBaseConfig::from_map(
            self.user.username().to_string(),
            configuration.clone(),
        );
        let mutate_property = required_string(&configuration, "mutateProperty");
        let write_concurrency =
            optional_usize(&configuration, "writeConcurrency").unwrap_or_else(num_cpus::get);

        let trained_model =
            self.node_regression_model_for(base_config.model_user(), base_config.model_name());
        let config_with_defaults =
            base_config.with_train_config_defaults(&trained_model.train_config);
        let mutate_config = NodeRegressionPredictPipelineMutateConfig::new(
            config_with_defaults.clone(),
            mutate_property,
            write_concurrency,
        );

        let computation = NodeRegressionPredictComputation::new(
            config_with_defaults,
            "NodeRegressionPredictPipeline".to_string(),
            clone_predict_pipeline(&trained_model.pipeline),
            Arc::clone(&trained_model.regressor),
        );

        let predict_result = computation
            .compute(Arc::clone(&graph_store))
            .unwrap_or_else(|e| panic!("nodeRegression.mutate failed: {e}"));

        let mut metadata = None;
        self.graph_catalog
            .with_store_mut(graph_name, &mut |graph_store| {
                metadata = Some(apply_node_regression_mutation(
                    graph_store,
                    &mutate_config,
                    &predict_result,
                ));
            })
            .unwrap_or_else(|e| panic!("nodeRegression.mutate failed to update graph store: {e}"));
        let metadata = metadata.expect("nodeRegression.mutate metadata should be set");

        let graph_resources = GraphResources::new(self.graph_store_for(graph_name));
        let builder = NodeRegressionPredictPipelineMutateResultBuilder::new(mutate_config.clone());

        vec![builder.build(
            &graph_resources,
            &mutate_config,
            Some(predict_result),
            AlgorithmProcessingTimings::unavailable(),
            Some(metadata),
        )]
    }

    pub fn node_regression_predict_mutate_task_frame_plan(
        &self,
        graph_name: &str,
        configuration: &AnyMap,
    ) -> Vec<TaskFrame> {
        let resource_contract = self.node_pipeline_plan_resource_contract(
            graph_name,
            configuration,
            EstimateCoefficients {
                fixed_bytes: 2 * 1024 * 1024,
                per_node_bytes: 56,
                per_relationship_bytes: 20,
            },
            "writeConcurrency",
        );

        self.build_node_pipeline_task_frame_plan(
            "nodeRegression.predict.mutate",
            graph_name,
            &[
                "graph.load".to_string(),
                "pipeline.config.resolve".to_string(),
            ],
            &[
                "pipeline.predict".to_string(),
                "node.regression".to_string(),
            ],
            &[
                "graph.mutate".to_string(),
                "procedure.result.render".to_string(),
            ],
            resource_contract,
        )
    }

    pub fn node_regression_predict_mutate_estimate(
        &self,
        graph_name_or_configuration: Value,
        raw_configuration: AnyMap,
    ) -> Vec<MemoryEstimationResult> {
        self.estimate_pipeline_memory(
            "nodeRegression.mutateEstimate",
            &graph_name_or_configuration,
            &raw_configuration,
            EstimateCoefficients {
                fixed_bytes: 2 * 1024 * 1024,
                per_node_bytes: 56,
                per_relationship_bytes: 20,
            },
            "writeConcurrency",
        )
    }

    pub fn node_regression_predict_stream(
        &self,
        graph_name: &str,
        configuration: AnyMap,
    ) -> Vec<NodeRegressionStreamResult> {
        let graph_store = self.graph_store_for(graph_name);
        let base_config = NodeRegressionPredictPipelineBaseConfig::from_map(
            self.user.username().to_string(),
            configuration,
        );
        let trained_model =
            self.node_regression_model_for(base_config.model_user(), base_config.model_name());
        let config_with_defaults =
            base_config.with_train_config_defaults(&trained_model.train_config);

        let computation = NodeRegressionPredictComputation::new(
            config_with_defaults,
            "NodeRegressionPredictPipeline".to_string(),
            clone_predict_pipeline(&trained_model.pipeline),
            Arc::clone(&trained_model.regressor),
        );

        let predict_result = computation
            .compute(Arc::clone(&graph_store))
            .unwrap_or_else(|e| panic!("nodeRegression.stream failed: {e}"));

        let graph_resources = GraphResources::new(graph_store);
        let builder = NodeRegressionPredictPipelineStreamResultBuilder;
        builder
            .build(&graph_resources, Some(predict_result))
            .collect()
    }

    pub fn node_regression_predict_stream_estimate(
        &self,
        graph_name_or_configuration: Value,
        raw_configuration: AnyMap,
    ) -> Vec<MemoryEstimationResult> {
        self.estimate_pipeline_memory(
            "nodeRegression.streamEstimate",
            &graph_name_or_configuration,
            &raw_configuration,
            EstimateCoefficients {
                fixed_bytes: 1024 * 1024,
                per_node_bytes: 40,
                per_relationship_bytes: 12,
            },
            "concurrency",
        )
    }

    pub fn node_regression_train(
        &self,
        graph_name: &str,
        configuration: AnyMap,
    ) -> Vec<NodeRegressionPipelineTrainResult> {
        let graph_store = self.graph_store_for(graph_name);
        let model_name = optional_string(&configuration, "modelName")
            .unwrap_or_else(|| required_string(&configuration, "pipeline"));
        let train_config = parse_node_regression_train_config(&self.user, configuration);
        let computation = NodeRegressionTrainComputation::new(
            self.pipeline_repository.clone(),
            train_config,
            self.user.clone(),
        );

        let model_result = computation
            .compute(Arc::clone(&graph_store))
            .unwrap_or_else(|e| panic!("nodeRegression.train failed: {e}"));

        let (regressor, train_config, model_info, training_statistics) = model_result.into_parts();
        self.node_regression_models.write().insert(
            (self.user.username().to_string(), model_name),
            Arc::new(NodeRegressionRuntimeModel {
                regressor: Arc::from(regressor),
                pipeline: clone_predict_pipeline(model_info.pipeline()),
                train_config: train_config.clone(),
            }),
        );

        render_node_regression_train_result(
            &train_config,
            &model_info,
            &training_statistics.to_map(),
            AlgorithmProcessingTimings::unavailable(),
        )
    }

    pub fn node_regression_train_estimate(
        &self,
        graph_name_or_configuration: Value,
        raw_configuration: AnyMap,
    ) -> Vec<MemoryEstimationResult> {
        self.estimate_pipeline_memory(
            "nodeRegression.trainEstimate",
            &graph_name_or_configuration,
            &raw_configuration,
            EstimateCoefficients {
                fixed_bytes: 6 * 1024 * 1024,
                per_node_bytes: 72,
                per_relationship_bytes: 32,
            },
            "concurrency",
        )
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

    fn graph_store_for(&self, graph_name: &str) -> Arc<DefaultGraphStore> {
        self.graph_catalog
            .get(graph_name)
            .unwrap_or_else(|| panic!("Graph not found: {graph_name}"))
    }

    fn node_classification_model_for(
        &self,
        model_user: &str,
        model_name: &str,
    ) -> Arc<NodeClassificationModelResult> {
        self.node_classification_models
            .read()
            .get(&(model_user.to_string(), model_name.to_string()))
            .cloned()
            .unwrap_or_else(|| {
                panic!("Trained node classification model not found: {model_user}/{model_name}")
            })
    }

    fn node_regression_model_for(
        &self,
        model_user: &str,
        model_name: &str,
    ) -> Arc<NodeRegressionRuntimeModel> {
        self.node_regression_models
            .read()
            .get(&(model_user.to_string(), model_name.to_string()))
            .cloned()
            .unwrap_or_else(|| {
                panic!("Trained node regression model not found: {model_user}/{model_name}")
            })
    }

    fn node_pipeline_plan_resource_contract(
        &self,
        graph_name: &str,
        raw_configuration: &AnyMap,
        coefficients: EstimateCoefficients,
        concurrency_key: &str,
    ) -> PlannedResourceContract {
        let graph_store = self.graph_store_for(graph_name);
        let node_count = GraphStore::node_count(graph_store.as_ref());
        let relationship_count = GraphStore::relationship_count(graph_store.as_ref());

        let concurrency = optional_usize(raw_configuration, concurrency_key)
            .or_else(|| optional_usize(raw_configuration, "concurrency"))
            .unwrap_or_else(num_cpus::get)
            .max(1);

        let resident_bytes = coefficients
            .fixed_bytes
            .saturating_add(coefficients.per_node_bytes.saturating_mul(node_count))
            .saturating_add(
                coefficients
                    .per_relationship_bytes
                    .saturating_mul(relationship_count),
            );

        let temporary_min = resident_bytes.checked_div(10).unwrap_or(0).saturating_add(
            coefficients
                .per_node_bytes
                .saturating_mul(node_count)
                .saturating_mul(concurrency.saturating_sub(1))
                .checked_div(5)
                .unwrap_or(0),
        );
        let temporary_max = temporary_min
            .saturating_mul(2)
            .saturating_add(resident_bytes.checked_div(5).unwrap_or(0));

        let total_min = resident_bytes.saturating_add(temporary_min);
        let total_max = resident_bytes.saturating_add(temporary_max).max(total_min);

        PlannedResourceContract {
            concurrency,
            memory_range: MemoryRange::of_range(total_min, total_max),
        }
    }

    fn build_node_pipeline_task_frame_plan(
        &self,
        procedure_name: &str,
        graph_name: &str,
        seed_steps: &[String],
        compute_steps: &[String],
        persist_steps: &[String],
        resource_contract: PlannedResourceContract,
    ) -> Vec<TaskFrame> {
        let seed_memory = scale_memory_range(resource_contract.memory_range, 1, 5);
        let compute_memory = scale_memory_range(resource_contract.memory_range, 7, 10);
        let persist_memory = scale_memory_range(resource_contract.memory_range, 1, 10);

        let seed_output = format!("{procedure_name}::{graph_name}::SeedOutput");
        let compute_output = format!("{procedure_name}::{graph_name}::ComputeOutput");
        let final_output = format!("{procedure_name}::{graph_name}::Result");
        let concurrency = Concurrency::of(resource_contract.concurrency);

        let seed = TaskFrame::new(
            "procedure".to_string(),
            format!("{procedure_name}::Seed"),
            seed_steps.to_vec(),
            1,
            concurrency,
        )
        .with_image_kind(TaskFrameKind::GraphAlgorithm)
        .with_execution_image("seed.graph-store")
        .with_storage_backend(TaskFrameStorageBackend::GraphStore)
        .with_inputs(vec![format!("graph::{graph_name}")])
        .with_outputs(vec![seed_output.clone()])
        .with_memory_range(seed_memory);

        let compute = TaskFrame::new(
            "procedure".to_string(),
            format!("{procedure_name}::ComputeGraph"),
            compute_steps.to_vec(),
            1,
            concurrency,
        )
        .with_image_kind(TaskFrameKind::MachineLearning)
        .with_execution_image("pipeline.predict")
        .with_storage_backend(TaskFrameStorageBackend::GraphStore)
        .with_inputs(vec![seed_output])
        .with_outputs(vec![compute_output.clone()])
        .with_memory_range(compute_memory);

        let persist = TaskFrame::new(
            "procedure".to_string(),
            format!("{procedure_name}::Persist"),
            persist_steps.to_vec(),
            1,
            concurrency,
        )
        .with_image_kind(TaskFrameKind::ProcedurePipeline)
        .with_execution_image("persist.result")
        .with_storage_backend(TaskFrameStorageBackend::RuntimeManaged)
        .with_inputs(vec![compute_output])
        .with_outputs(vec![final_output])
        .with_memory_range(persist_memory);

        vec![seed, compute, persist]
    }

    fn estimate_pipeline_memory(
        &self,
        procedure_name: &str,
        graph_name_or_configuration: &Value,
        raw_configuration: &AnyMap,
        coefficients: EstimateCoefficients,
        concurrency_key: &str,
    ) -> Vec<MemoryEstimationResult> {
        let graph_name = estimate_graph_name(graph_name_or_configuration, raw_configuration);
        let graph_store = self.graph_store_for(&graph_name);

        let node_count = GraphStore::node_count(graph_store.as_ref());
        let relationship_count = GraphStore::relationship_count(graph_store.as_ref());
        let dimensions = ConcreteGraphDimensions::of(node_count, relationship_count);

        let concurrency = optional_usize(raw_configuration, concurrency_key)
            .or_else(|| optional_usize(raw_configuration, "concurrency"))
            .unwrap_or_else(num_cpus::get)
            .max(1);

        let resident_bytes = coefficients
            .fixed_bytes
            .saturating_add(coefficients.per_node_bytes.saturating_mul(node_count))
            .saturating_add(
                coefficients
                    .per_relationship_bytes
                    .saturating_mul(relationship_count),
            );

        let temporary_min = resident_bytes.checked_div(10).unwrap_or(0).saturating_add(
            coefficients
                .per_node_bytes
                .saturating_mul(node_count)
                .saturating_mul(concurrency.saturating_sub(1))
                .checked_div(5)
                .unwrap_or(0),
        );
        let temporary_max = temporary_min
            .saturating_mul(2)
            .saturating_add(resident_bytes.checked_div(5).unwrap_or(0));

        let total_min = resident_bytes.saturating_add(temporary_min);
        let total_max = resident_bytes.saturating_add(temporary_max).max(total_min);

        let memory_tree = MemoryTree::new(
            procedure_name.to_string(),
            MemoryRange::of_range(total_min, total_max),
            vec![
                MemoryTree::leaf(
                    "residentMemory".to_string(),
                    MemoryRange::of(resident_bytes),
                ),
                MemoryTree::leaf(
                    "temporaryMemory".to_string(),
                    MemoryRange::of_range(temporary_min, temporary_max.max(temporary_min)),
                ),
            ],
        );

        vec![MemoryEstimationResult::new(dimensions, memory_tree)]
    }
}

fn scale_memory_range(range: MemoryRange, numerator: usize, denominator: usize) -> MemoryRange {
    let denominator = denominator.max(1);
    let min = range
        .min()
        .saturating_mul(numerator)
        .checked_div(denominator)
        .unwrap_or(0);
    let mut max = range
        .max()
        .saturating_mul(numerator)
        .checked_div(denominator)
        .unwrap_or(0);
    if max < min {
        max = min;
    }
    MemoryRange::of_range(min, max)
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

fn parse_node_classification_train_config(
    user: &User,
    configuration: AnyMap,
) -> NodeClassificationPipelineTrainConfig {
    let pipeline = required_string(&configuration, "pipeline");
    let target_node_labels = string_list_or_default(&configuration, "targetNodeLabels", vec!["*"]);
    let relationship_types =
        string_list_or_default(&configuration, "relationshipTypes", Vec::new());
    let target_property = required_string(&configuration, "targetProperty");
    let random_seed = optional_u64(&configuration, "randomSeed");
    let concurrency = optional_usize(&configuration, "concurrency").unwrap_or_else(num_cpus::get);
    let metric_names = string_list_or_default(&configuration, "metrics", vec!["F1(class=*)"]);
    let metrics = ClassificationMetricSpecification::parse_list(&metric_names)
        .unwrap_or_else(|e| panic!("Invalid node classification metrics: {e}"));

    NodeClassificationPipelineTrainConfig::new_with_username(
        user.username().to_string(),
        pipeline,
        target_node_labels,
        relationship_types,
        target_property,
        random_seed,
        concurrency,
        metrics,
    )
    .with_username(user.username())
}

fn parse_node_regression_train_config(
    user: &User,
    configuration: AnyMap,
) -> NodeRegressionPipelineTrainConfig {
    let pipeline = required_string(&configuration, "pipeline");
    let target_node_labels = string_list_or_default(&configuration, "targetNodeLabels", vec!["*"]);
    let relationship_types =
        string_list_or_default(&configuration, "relationshipTypes", Vec::new());
    let target_property = required_string(&configuration, "targetProperty");
    let random_seed = optional_u64(&configuration, "randomSeed");
    let concurrency = optional_usize(&configuration, "concurrency").unwrap_or_else(num_cpus::get);
    let metrics: Vec<RegressionMetric> =
        string_list_or_default(&configuration, "metrics", vec!["MEAN_SQUARED_ERROR"])
            .into_iter()
            .map(|name| RegressionMetric::parse(&name))
            .collect();

    NodeRegressionPipelineTrainConfig::new_with_username(
        user.username().to_string(),
        pipeline,
        target_node_labels,
        relationship_types,
        target_property,
        random_seed,
        concurrency,
        metrics,
    )
    .with_username(user.username())
}

fn required_string(configuration: &AnyMap, key: &str) -> String {
    configuration
        .get(key)
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToString::to_string)
        .unwrap_or_else(|| panic!("Missing required config field: {key}"))
}

fn string_list_or_default(configuration: &AnyMap, key: &str, default: Vec<&str>) -> Vec<String> {
    match configuration.get(key) {
        None => default.into_iter().map(ToString::to_string).collect(),
        Some(Value::Array(values)) => values
            .iter()
            .map(|value| {
                value
                    .as_str()
                    .map(str::trim)
                    .filter(|entry| !entry.is_empty())
                    .map(ToString::to_string)
                    .unwrap_or_else(|| panic!("Config field `{key}` expects an array of strings"))
            })
            .collect(),
        Some(Value::String(single)) => vec![single.clone()],
        Some(_) => panic!("Config field `{key}` expects a string or string array"),
    }
}

fn optional_u64(configuration: &AnyMap, key: &str) -> Option<u64> {
    configuration.get(key).and_then(|value| match value {
        Value::Null => None,
        Value::Number(number) => number.as_u64(),
        Value::String(text) => text.trim().parse::<u64>().ok(),
        _ => None,
    })
}

fn optional_usize(configuration: &AnyMap, key: &str) -> Option<usize> {
    optional_u64(configuration, key).map(|value| value as usize)
}

fn optional_string(configuration: &AnyMap, key: &str) -> Option<String> {
    configuration
        .get(key)
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToString::to_string)
}

fn estimate_graph_name(graph_name_or_configuration: &Value, raw_configuration: &AnyMap) -> String {
    let from_embedded_object = match graph_name_or_configuration {
        Value::Object(value_map) => value_map
            .get("graphName")
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToString::to_string),
        _ => None,
    };

    match graph_name_or_configuration {
        Value::String(graph_name) if !graph_name.trim().is_empty() => graph_name.trim().to_string(),
        _ => from_embedded_object
            .or_else(|| optional_string(raw_configuration, "graphName"))
            .unwrap_or_else(|| {
                panic!(
                    "Missing graph name for estimate call; provide graphName as the first argument or in configuration"
                )
            }),
    }
}

fn render_node_classification_train_result(
    model_result: &NodeClassificationModelResult,
    timings: AlgorithmProcessingTimings,
) -> Vec<NodeClassificationPipelineTrainResult> {
    let mut model_info = HashMap::new();
    model_info.insert(
        "modelType".to_string(),
        Value::String("NodeClassification".to_string()),
    );
    model_info.insert(
        "modelInfo".to_string(),
        Value::Object(model_result.model_info().to_map().into_iter().collect()),
    );

    let train_config = model_result.train_config();
    let mut configuration = HashMap::new();
    configuration.insert(
        "pipeline".to_string(),
        Value::String(train_config.pipeline().to_string()),
    );
    configuration.insert(
        "targetNodeLabels".to_string(),
        Value::Array(
            train_config
                .target_node_labels()
                .into_iter()
                .map(Value::String)
                .collect(),
        ),
    );
    configuration.insert(
        "relationshipTypes".to_string(),
        Value::Array(
            train_config
                .relationship_types()
                .into_iter()
                .map(Value::String)
                .collect(),
        ),
    );
    configuration.insert(
        "targetProperty".to_string(),
        Value::String(train_config.target_property().to_string()),
    );
    configuration.insert(
        "randomSeed".to_string(),
        train_config
            .random_seed()
            .map(|seed| Value::Number(serde_json::Number::from(seed)))
            .unwrap_or(Value::Null),
    );
    configuration.insert(
        "concurrency".to_string(),
        Value::Number(serde_json::Number::from(train_config.concurrency() as i64)),
    );
    configuration.insert(
        "metrics".to_string(),
        Value::Array(
            train_config
                .metrics_specs()
                .iter()
                .map(|spec| Value::String(spec.to_string()))
                .collect(),
        ),
    );

    vec![NodeClassificationPipelineTrainResult {
        base: crate::procedures::pipelines::types::MLTrainResult {
            train_millis: timings.compute_millis,
            model_info,
            configuration,
        },
        model_selection_stats: model_result.training_statistics().to_map(),
    }]
}

fn render_node_regression_train_result(
    train_config: &NodeRegressionPipelineTrainConfig,
    model_info: &NodeRegressionPipelineModelInfo,
    training_statistics: &AnyMap,
    timings: AlgorithmProcessingTimings,
) -> Vec<NodeRegressionPipelineTrainResult> {
    let mut rendered_model_info = HashMap::new();
    rendered_model_info.insert(
        "modelType".to_string(),
        Value::String("NodeRegression".to_string()),
    );
    rendered_model_info.insert(
        "modelInfo".to_string(),
        Value::Object(model_info.to_map().into_iter().collect()),
    );

    let mut configuration = HashMap::new();
    configuration.insert(
        "pipeline".to_string(),
        Value::String(train_config.pipeline().to_string()),
    );
    configuration.insert(
        "targetNodeLabels".to_string(),
        Value::Array(
            train_config
                .target_node_labels()
                .into_iter()
                .map(Value::String)
                .collect(),
        ),
    );
    configuration.insert(
        "relationshipTypes".to_string(),
        Value::Array(
            train_config
                .relationship_types()
                .into_iter()
                .map(Value::String)
                .collect(),
        ),
    );
    configuration.insert(
        "targetProperty".to_string(),
        Value::String(train_config.target_property().to_string()),
    );
    configuration.insert(
        "randomSeed".to_string(),
        train_config
            .random_seed()
            .map(|seed| Value::Number(serde_json::Number::from(seed)))
            .unwrap_or(Value::Null),
    );
    configuration.insert(
        "concurrency".to_string(),
        Value::Number(serde_json::Number::from(train_config.concurrency() as i64)),
    );
    configuration.insert(
        "metrics".to_string(),
        Value::Array(
            train_config
                .metrics()
                .iter()
                .map(|metric| Value::String(metric.to_string()))
                .collect(),
        ),
    );

    vec![NodeRegressionPipelineTrainResult {
        base: crate::procedures::pipelines::types::MLTrainResult {
            train_millis: timings.compute_millis,
            model_info: rendered_model_info,
            configuration,
        },
        model_selection_stats: training_statistics.clone(),
    }]
}

fn parse_node_classification_write_config(
    username: String,
    configuration: AnyMap,
) -> NodeClassificationPredictPipelineWriteConfig {
    let base =
        NodeClassificationPredictPipelineBaseConfig::from_map(username, configuration.clone());
    let mutate_or_write = NodeClassificationPredictPipelineMutateOrWriteConfig::new(
        base,
        optional_string(&configuration, "predictedProbabilityProperty"),
    );
    let write_property = required_string(&configuration, "writeProperty");
    let write_concurrency =
        optional_usize(&configuration, "writeConcurrency").unwrap_or_else(num_cpus::get);

    NodeClassificationPredictPipelineWriteConfig::new(
        mutate_or_write,
        write_property,
        write_concurrency,
    )
}

fn parse_node_classification_mutate_config(
    username: String,
    configuration: AnyMap,
) -> NodeClassificationPredictPipelineMutateConfig {
    let base =
        NodeClassificationPredictPipelineBaseConfig::from_map(username, configuration.clone());
    let mutate_or_write = NodeClassificationPredictPipelineMutateOrWriteConfig::new(
        base,
        optional_string(&configuration, "predictedProbabilityProperty"),
    );
    let mutate_property = required_string(&configuration, "mutateProperty");
    let write_concurrency =
        optional_usize(&configuration, "writeConcurrency").unwrap_or_else(num_cpus::get);

    NodeClassificationPredictPipelineMutateConfig::new(
        mutate_or_write,
        mutate_property,
        write_concurrency,
    )
}

fn clone_predict_pipeline(pipeline: &NodePropertyPredictPipeline) -> NodePropertyPredictPipeline {
    NodePropertyPredictPipeline::new(
        pipeline
            .node_property_steps()
            .iter()
            .map(|step| clone_box(&**step))
            .collect(),
        pipeline.feature_steps().to_vec(),
    )
}

fn apply_node_regression_mutation(
    graph_store: &mut DefaultGraphStore,
    configuration: &NodeRegressionPredictPipelineMutateConfig,
    result: &NodeRegressionPipelineResult,
) -> GraphStoreNodePropertiesWritten {
    let labels_to_update = if configuration.target_node_labels().is_empty()
        || (configuration.target_node_labels().len() == 1
            && configuration.target_node_labels()[0] == "*")
    {
        graph_store.node_labels()
    } else {
        configuration
            .target_node_labels()
            .iter()
            .map(|label| crate::projection::NodeLabel::of(label.clone()))
            .collect()
    };

    let predicted = result.predicted_values();
    let node_count = result.root_node_count();
    let mut values_vec = vec![0.0; node_count];
    if let Some(node_ids) = result.predicted_node_ids() {
        for (row_id, node_id) in node_ids.iter().enumerate() {
            values_vec[*node_id as usize] = predicted.get(row_id);
        }
    } else {
        for node_id in 0..node_count {
            values_vec[node_id] = predicted.get(node_id);
        }
    }

    let values: Arc<dyn NodePropertyValues> = Arc::new(
        DefaultDoubleNodePropertyValues::<VecDouble>::from_collection(
            VecDouble::from(values_vec),
            node_count,
        ),
    );
    let node_properties = [NodeProperty::new(configuration.mutate_property(), values)];
    let service = GraphStoreService::new(Log::new());
    let written = service
        .add_node_properties(graph_store, labels_to_update, &node_properties)
        .unwrap_or_else(|e| panic!("nodeRegression.mutate failed to update graph store: {e}"));

    GraphStoreNodePropertiesWritten(written.0)
}

fn apply_node_classification_mutation(
    graph_store: &mut DefaultGraphStore,
    configuration: &NodeClassificationPredictPipelineMutateConfig,
    result: &NodeClassificationPipelineResult,
) -> GraphStoreNodePropertiesWritten {
    let labels_to_update = if configuration.target_node_labels().is_empty()
        || (configuration.target_node_labels().len() == 1
            && configuration.target_node_labels()[0] == "*")
    {
        graph_store.node_labels()
    } else {
        configuration
            .target_node_labels()
            .iter()
            .map(|label| crate::projection::NodeLabel::of(label.clone()))
            .collect()
    };

    let node_properties = as_properties(
        Some(result),
        configuration.mutate_property(),
        configuration.predicted_probability_property(),
    );

    let service = GraphStoreService::new(Log::new());
    let written = service
        .add_node_properties(graph_store, labels_to_update, &node_properties)
        .unwrap_or_else(|e| panic!("nodeClassification.mutate failed to update graph store: {e}"));

    GraphStoreNodePropertiesWritten(written.0)
}

#[cfg(test)]
mod task_frame_plan_tests {
    use super::*;
    use crate::projection::eval::pipeline::PipelineCatalog;
    use crate::types::catalog::GraphCatalog;
    use crate::types::random::RandomGraphConfig;
    use crate::types::random::RandomRelationshipConfig;
    use serde_json::Value;

    fn pipeline_applications_with_graph() -> PipelineApplications {
        let graph_catalog: Arc<dyn GraphCatalog> = Arc::new(InMemoryGraphCatalog::new());
        let graph = Arc::new(
            DefaultGraphStore::random(&RandomGraphConfig {
                seed: Some(19),
                node_count: 7,
                relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
                ..RandomGraphConfig::default()
            })
            .expect("random graph generation"),
        );
        graph_catalog.set("graph", graph);

        let repository = PipelineRepository::new(Arc::new(PipelineCatalog::new()));
        PipelineApplications::new_with_graph_catalog(User::from("alice"), repository, graph_catalog)
    }

    #[test]
    fn node_classification_mutate_task_frame_plan_is_three_stage() {
        let apps = pipeline_applications_with_graph();
        let config = AnyMap::from([("writeConcurrency".to_string(), Value::from(2))]);

        let plan = apps.node_classification_predict_mutate_task_frame_plan("graph", &config);
        assert_eq!(plan.len(), 3);
        assert!(plan[0].pipeline().ends_with("::Seed"));
        assert!(plan[1].pipeline().ends_with("::ComputeGraph"));
        assert!(plan[2].pipeline().ends_with("::Persist"));
        assert_eq!(plan[1].image_spec().kind(), TaskFrameKind::MachineLearning);
    }

    #[test]
    fn node_regression_mutate_task_frame_plan_is_three_stage() {
        let apps = pipeline_applications_with_graph();
        let config = AnyMap::from([("writeConcurrency".to_string(), Value::from(2))]);

        let plan = apps.node_regression_predict_mutate_task_frame_plan("graph", &config);
        assert_eq!(plan.len(), 3);
        assert!(plan[0].pipeline().ends_with("::Seed"));
        assert!(plan[1].pipeline().ends_with("::ComputeGraph"));
        assert!(plan[2].pipeline().ends_with("::Persist"));
        assert_eq!(plan[1].image_spec().kind(), TaskFrameKind::MachineLearning);
    }
}
