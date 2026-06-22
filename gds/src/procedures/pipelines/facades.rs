use super::types::*;
use super::{PipelineApplications, PipelineName, PipelineRepository};
use crate::projection::eval::pipeline::AutoTuningConfig;
use crate::projection::eval::pipeline::LinkPredictionSplitConfig;
use crate::projection::eval::pipeline::NodeFeatureStep;
use crate::projection::eval::pipeline::NodePropertyPredictionSplitConfig;
use crate::projection::eval::pipeline::PipelineCatalog;
use crate::projection::eval::pipeline::TrainingMethod;
use crate::task::memory::MemoryEstimationResult;
use crate::task::runtime::TaskFrame;
use crate::types::catalog::GraphCatalog;
use crate::types::catalog::InMemoryGraphCatalog;
use crate::types::user::User;
use serde_json::Value;
use std::sync::Arc;
use std::sync::OnceLock;
pub type RawConfig = AnyMap;

pub trait LinkPredictionFacade {
    fn add_feature(
        &self,
        pipeline_name: &str,
        feature_type: &str,
        raw_configuration: RawConfig,
    ) -> Vec<PipelineInfoResult>;

    fn add_logistic_regression(
        &self,
        pipeline_name: &str,
        configuration: RawConfig,
    ) -> Vec<PipelineInfoResult>;

    fn add_mlp(&self, pipeline_name: &str, configuration: RawConfig) -> Vec<PipelineInfoResult>;

    fn add_node_property(
        &self,
        pipeline_name: &str,
        task_name: &str,
        procedure_config: RawConfig,
    ) -> Vec<PipelineInfoResult>;

    fn add_random_forest(
        &self,
        pipeline_name: &str,
        configuration: RawConfig,
    ) -> Vec<PipelineInfoResult>;

    fn configure_auto_tuning(
        &self,
        pipeline_name: &str,
        configuration: RawConfig,
    ) -> Vec<PipelineInfoResult>;

    fn configure_split(
        &self,
        pipeline_name: &str,
        configuration: RawConfig,
    ) -> Vec<PipelineInfoResult>;

    fn create_pipeline(&self, pipeline_name: &str) -> Vec<PipelineInfoResult>;

    fn mutate(&self, graph_name: &str, configuration: RawConfig) -> Vec<MutateResult>;

    fn mutate_estimate(
        &self,
        graph_name_or_configuration: Value,
        raw_configuration: RawConfig,
    ) -> Vec<MemoryEstimationResult>;

    fn stream(&self, graph_name: &str, configuration: RawConfig) -> Vec<StreamResult>;

    fn stream_estimate(
        &self,
        graph_name_or_configuration: Value,
        raw_configuration: RawConfig,
    ) -> Vec<MemoryEstimationResult>;

    fn train(&self, graph_name: &str, configuration: RawConfig) -> Vec<LinkPredictionTrainResult>;

    fn train_estimate(
        &self,
        graph_name_or_configuration: Value,
        raw_configuration: RawConfig,
    ) -> Vec<MemoryEstimationResult>;
}

pub trait NodeClassificationFacade {
    fn add_logistic_regression(
        &self,
        pipeline_name: &str,
        configuration: RawConfig,
    ) -> Vec<NodePipelineInfoResult>;

    fn add_mlp(&self, pipeline_name: &str, configuration: RawConfig)
        -> Vec<NodePipelineInfoResult>;

    fn add_node_property(
        &self,
        pipeline_name: &str,
        task_name: &str,
        procedure_config: RawConfig,
    ) -> Vec<NodePipelineInfoResult>;

    fn add_random_forest(
        &self,
        pipeline_name: &str,
        configuration: RawConfig,
    ) -> Vec<NodePipelineInfoResult>;

    fn configure_auto_tuning(
        &self,
        pipeline_name: &str,
        configuration: RawConfig,
    ) -> Vec<NodePipelineInfoResult>;

    fn configure_split(
        &self,
        pipeline_name: &str,
        configuration: RawConfig,
    ) -> Vec<NodePipelineInfoResult>;

    fn create_pipeline(&self, pipeline_name: &str) -> Vec<NodePipelineInfoResult>;

    fn mutate(&self, graph_name: &str, configuration: RawConfig) -> Vec<PredictMutateResult>;

    fn mutate_task_frame_plan(&self, graph_name: &str, configuration: RawConfig) -> Vec<TaskFrame>;

    fn mutate_estimate(
        &self,
        graph_name_or_configuration: Value,
        raw_configuration: RawConfig,
    ) -> Vec<MemoryEstimationResult>;

    fn select_features(
        &self,
        pipeline_name: &str,
        node_feature_steps: Value,
    ) -> Vec<NodePipelineInfoResult>;

    fn stream(
        &self,
        graph_name: &str,
        configuration: RawConfig,
    ) -> Vec<NodeClassificationStreamResult>;

    fn stream_estimate(
        &self,
        graph_name_or_configuration: Value,
        raw_configuration: RawConfig,
    ) -> Vec<MemoryEstimationResult>;

    fn train(
        &self,
        graph_name: &str,
        configuration: RawConfig,
    ) -> Vec<NodeClassificationPipelineTrainResult>;

    fn train_estimate(
        &self,
        graph_name_or_configuration: Value,
        raw_configuration: RawConfig,
    ) -> Vec<MemoryEstimationResult>;

    fn write(&self, graph_name: &str, configuration: RawConfig) -> Vec<WriteResult>;

    fn write_estimate(
        &self,
        graph_name_or_configuration: Value,
        raw_configuration: RawConfig,
    ) -> Vec<MemoryEstimationResult>;
}

pub trait NodeRegressionFacade {
    fn add_logistic_regression(
        &self,
        pipeline_name: &str,
        configuration: RawConfig,
    ) -> Vec<NodePipelineInfoResult>;

    fn add_node_property(
        &self,
        pipeline_name: &str,
        task_name: &str,
        procedure_config: RawConfig,
    ) -> Vec<NodePipelineInfoResult>;

    fn add_random_forest(
        &self,
        pipeline_name: &str,
        configuration: RawConfig,
    ) -> Vec<NodePipelineInfoResult>;

    fn configure_auto_tuning(
        &self,
        pipeline_name: &str,
        configuration: RawConfig,
    ) -> Vec<NodePipelineInfoResult>;

    fn configure_split(
        &self,
        pipeline_name: &str,
        configuration: RawConfig,
    ) -> Vec<NodePipelineInfoResult>;

    fn create_pipeline(&self, pipeline_name: &str) -> Vec<NodePipelineInfoResult>;

    fn mutate(&self, graph_name: &str, configuration: RawConfig) -> Vec<PredictMutateResult>;

    fn mutate_task_frame_plan(&self, graph_name: &str, configuration: RawConfig) -> Vec<TaskFrame>;

    fn mutate_estimate(
        &self,
        graph_name_or_configuration: Value,
        raw_configuration: RawConfig,
    ) -> Vec<MemoryEstimationResult>;

    fn stream(&self, graph_name: &str, configuration: RawConfig)
        -> Vec<NodeRegressionStreamResult>;

    fn stream_estimate(
        &self,
        graph_name_or_configuration: Value,
        raw_configuration: RawConfig,
    ) -> Vec<MemoryEstimationResult>;

    fn select_features(
        &self,
        pipeline_name: &str,
        feature_properties: Value,
    ) -> Vec<NodePipelineInfoResult>;

    fn train(
        &self,
        graph_name: &str,
        configuration: RawConfig,
    ) -> Vec<NodeRegressionPipelineTrainResult>;

    fn train_estimate(
        &self,
        graph_name_or_configuration: Value,
        raw_configuration: RawConfig,
    ) -> Vec<MemoryEstimationResult>;
}

pub trait PipelinesProcedureFacade {
    const NO_VALUE: &'static str = "__NO_VALUE";

    fn drop(&self, pipeline_name: &str, fail_if_missing: bool) -> Vec<PipelineCatalogResult>;

    fn exists(&self, pipeline_name: &str) -> Vec<PipelineExistsResult>;

    fn list(&self, pipeline_name: &str) -> Vec<PipelineCatalogResult>;

    fn link_prediction(&self) -> &dyn LinkPredictionFacade;

    fn node_classification(&self) -> &dyn NodeClassificationFacade;

    fn node_regression(&self) -> &dyn NodeRegressionFacade;
}

// ---------------------------------------------------------------------------
// Local implementation: Java-shaped facade delegates catalog and builder
// operations to PipelineApplications; executor-backed operations are deferred.
// ---------------------------------------------------------------------------

/// Request scoped dependencies for pipeline procedures.
pub struct RequestScopedDependencies {
    pub user: User,
    pub graph_catalog: Arc<dyn GraphCatalog>,
}

impl RequestScopedDependencies {
    pub fn new(user: User) -> Self {
        Self::with_graph_catalog(user, shared_in_memory_graph_catalog())
    }

    pub fn with_graph_catalog(user: User, graph_catalog: Arc<dyn GraphCatalog>) -> Self {
        Self {
            user,
            graph_catalog,
        }
    }
}

pub struct LocalPipelinesProcedureFacade {
    pipeline_applications: PipelineApplications,
    link_prediction: LocalLinkPredictionFacade,
    node_classification: LocalNodeClassificationFacade,
    node_regression: LocalNodeRegressionFacade,
}

fn shared_in_memory_pipeline_catalog() -> Arc<PipelineCatalog> {
    static CATALOG: OnceLock<Arc<PipelineCatalog>> = OnceLock::new();
    Arc::clone(CATALOG.get_or_init(|| Arc::new(PipelineCatalog::new())))
}

fn shared_in_memory_graph_catalog() -> Arc<dyn GraphCatalog> {
    static CATALOG: OnceLock<Arc<dyn GraphCatalog>> = OnceLock::new();
    Arc::clone(CATALOG.get_or_init(|| Arc::new(InMemoryGraphCatalog::new())))
}

impl Default for LocalPipelinesProcedureFacade {
    fn default() -> Self {
        Self::new(
            RequestScopedDependencies::new(User::from("anonymous")),
            shared_in_memory_pipeline_catalog(),
        )
    }
}

impl LocalPipelinesProcedureFacade {
    pub fn new(
        request_scoped_dependencies: RequestScopedDependencies,
        pipeline_catalog: Arc<PipelineCatalog>,
    ) -> Self {
        let pipeline_repository = PipelineRepository::new(Arc::clone(&pipeline_catalog));
        let pipeline_applications = PipelineApplications::new_with_graph_catalog(
            request_scoped_dependencies.user.clone(),
            pipeline_repository,
            Arc::clone(&request_scoped_dependencies.graph_catalog),
        );
        Self {
            pipeline_applications: pipeline_applications.clone(),
            link_prediction: LocalLinkPredictionFacade::new(pipeline_applications.clone()),
            node_classification: LocalNodeClassificationFacade::new(pipeline_applications.clone()),
            node_regression: LocalNodeRegressionFacade::new(pipeline_applications),
        }
    }
}

impl PipelinesProcedureFacade for LocalPipelinesProcedureFacade {
    fn drop(&self, pipeline_name: &str, fail_if_missing: bool) -> Vec<PipelineCatalogResult> {
        let pipeline_name = PipelineName::parse(pipeline_name).unwrap_or_else(|e| panic!("{e}"));

        if fail_if_missing {
            return vec![self.pipeline_applications.drop(&pipeline_name)];
        }

        self.pipeline_applications
            .drop_silencing_failure(&pipeline_name)
            .into_iter()
            .collect()
    }

    fn exists(&self, pipeline_name: &str) -> Vec<PipelineExistsResult> {
        let pipeline_name = PipelineName::parse(pipeline_name).unwrap_or_else(|e| panic!("{e}"));
        vec![self.pipeline_applications.exists(&pipeline_name)]
    }

    fn list(&self, pipeline_name: &str) -> Vec<PipelineCatalogResult> {
        if pipeline_name == Self::NO_VALUE {
            return self.pipeline_applications.list();
        }

        let pipeline_name = PipelineName::parse(pipeline_name).unwrap_or_else(|e| panic!("{e}"));
        self.pipeline_applications
            .get_single(&pipeline_name)
            .into_iter()
            .collect()
    }

    fn link_prediction(&self) -> &dyn LinkPredictionFacade {
        &self.link_prediction
    }

    fn node_classification(&self) -> &dyn NodeClassificationFacade {
        &self.node_classification
    }

    fn node_regression(&self) -> &dyn NodeRegressionFacade {
        &self.node_regression
    }
}

pub struct LocalLinkPredictionFacade {
    pipeline_applications: PipelineApplications,
}

impl LocalLinkPredictionFacade {
    pub fn new(pipeline_applications: PipelineApplications) -> Self {
        Self {
            pipeline_applications,
        }
    }

    fn parse_split_config(configuration: &RawConfig) -> LinkPredictionSplitConfig {
        let mut builder = LinkPredictionSplitConfig::builder();

        if let Some(v) = configuration
            .get("validationFolds")
            .and_then(|v| v.as_u64())
        {
            builder = builder.validation_folds(v as u32);
        }
        if let Some(v) = configuration.get("testFraction").and_then(|v| v.as_f64()) {
            builder = builder.test_fraction(v);
        }
        if let Some(v) = configuration.get("trainFraction").and_then(|v| v.as_f64()) {
            builder = builder.train_fraction(v);
        }
        if let Some(v) = configuration
            .get("negativeSamplingRatio")
            .and_then(|v| v.as_f64())
        {
            builder = builder.negative_sampling_ratio(v);
        }
        if let Some(v) = configuration
            .get("negativeRelationshipType")
            .and_then(|v| v.as_str())
        {
            builder = builder.negative_relationship_type(v.to_string());
        }

        builder.build().unwrap_or_else(|e| panic!("{e}"))
    }

    fn parse_auto_tuning_config(configuration: &RawConfig) -> AutoTuningConfig {
        let max_trials = configuration
            .get("maxTrials")
            .and_then(|v| v.as_u64())
            .map(|v| v as usize)
            .unwrap_or(AutoTuningConfig::MAX_TRIALS);

        AutoTuningConfig::new(max_trials).unwrap_or_else(|e| panic!("{e}"))
    }
}

impl LinkPredictionFacade for LocalLinkPredictionFacade {
    fn add_feature(
        &self,
        pipeline_name: &str,
        feature_type: &str,
        raw_configuration: RawConfig,
    ) -> Vec<PipelineInfoResult> {
        let pipeline_name = PipelineName::parse(pipeline_name).unwrap_or_else(|e| panic!("{e}"));
        let node_props_value = raw_configuration
            .get("nodeProperties")
            .unwrap_or_else(|| panic!("add_feature expects configuration key `nodeProperties`"));

        vec![self
            .pipeline_applications
            .add_feature_to_link_prediction_pipeline(
                &pipeline_name,
                feature_type,
                node_props_value.clone(),
            )]
    }

    fn add_logistic_regression(
        &self,
        pipeline_name: &str,
        _configuration: RawConfig,
    ) -> Vec<PipelineInfoResult> {
        let pipeline_name = PipelineName::parse(pipeline_name).unwrap_or_else(|e| panic!("{e}"));
        vec![self
            .pipeline_applications
            .add_training_method_to_link_prediction_pipeline(
                &pipeline_name,
                TrainingMethod::LogisticRegression,
            )]
    }

    fn add_mlp(&self, pipeline_name: &str, _configuration: RawConfig) -> Vec<PipelineInfoResult> {
        let pipeline_name = PipelineName::parse(pipeline_name).unwrap_or_else(|e| panic!("{e}"));
        vec![self
            .pipeline_applications
            .add_training_method_to_link_prediction_pipeline(
                &pipeline_name,
                TrainingMethod::MLPClassification,
            )]
    }

    fn add_node_property(
        &self,
        pipeline_name: &str,
        task_name: &str,
        procedure_config: RawConfig,
    ) -> Vec<PipelineInfoResult> {
        let pipeline_name = PipelineName::parse(pipeline_name).unwrap_or_else(|e| panic!("{e}"));
        vec![self
            .pipeline_applications
            .add_node_property_to_link_prediction_pipeline(
                &pipeline_name,
                task_name,
                procedure_config,
            )]
    }

    fn add_random_forest(
        &self,
        pipeline_name: &str,
        _configuration: RawConfig,
    ) -> Vec<PipelineInfoResult> {
        let pipeline_name = PipelineName::parse(pipeline_name).unwrap_or_else(|e| panic!("{e}"));
        vec![self
            .pipeline_applications
            .add_training_method_to_link_prediction_pipeline(
                &pipeline_name,
                TrainingMethod::RandomForestClassification,
            )]
    }

    fn configure_auto_tuning(
        &self,
        pipeline_name: &str,
        configuration: RawConfig,
    ) -> Vec<PipelineInfoResult> {
        let pipeline_name = PipelineName::parse(pipeline_name).unwrap_or_else(|e| panic!("{e}"));
        let config = Self::parse_auto_tuning_config(&configuration);
        vec![self
            .pipeline_applications
            .configure_link_prediction_auto_tuning(&pipeline_name, config)]
    }

    fn configure_split(
        &self,
        pipeline_name: &str,
        configuration: RawConfig,
    ) -> Vec<PipelineInfoResult> {
        let pipeline_name = PipelineName::parse(pipeline_name).unwrap_or_else(|e| panic!("{e}"));
        let config = Self::parse_split_config(&configuration);
        vec![self
            .pipeline_applications
            .configure_link_prediction_split(&pipeline_name, config)]
    }

    fn create_pipeline(&self, pipeline_name: &str) -> Vec<PipelineInfoResult> {
        let pipeline_name = PipelineName::parse(pipeline_name).unwrap_or_else(|e| panic!("{e}"));
        vec![self
            .pipeline_applications
            .create_link_prediction_training_pipeline(&pipeline_name)]
    }

    fn mutate(&self, graph_name: &str, configuration: RawConfig) -> Vec<MutateResult> {
        self.pipeline_applications
            .link_prediction_mutate(graph_name, configuration)
    }

    fn mutate_estimate(
        &self,
        graph_name_or_configuration: Value,
        raw_configuration: RawConfig,
    ) -> Vec<MemoryEstimationResult> {
        self.pipeline_applications
            .link_prediction_mutate_estimate(graph_name_or_configuration, raw_configuration)
    }

    fn stream(&self, graph_name: &str, configuration: RawConfig) -> Vec<StreamResult> {
        self.pipeline_applications
            .link_prediction_stream(graph_name, configuration)
    }

    fn stream_estimate(
        &self,
        graph_name_or_configuration: Value,
        raw_configuration: RawConfig,
    ) -> Vec<MemoryEstimationResult> {
        self.pipeline_applications
            .link_prediction_stream_estimate(graph_name_or_configuration, raw_configuration)
    }

    fn train(&self, graph_name: &str, configuration: RawConfig) -> Vec<LinkPredictionTrainResult> {
        self.pipeline_applications
            .link_prediction_train(graph_name, configuration)
    }

    fn train_estimate(
        &self,
        graph_name_or_configuration: Value,
        raw_configuration: RawConfig,
    ) -> Vec<MemoryEstimationResult> {
        self.pipeline_applications
            .link_prediction_train_estimate(graph_name_or_configuration, raw_configuration)
    }
}

pub struct LocalNodeClassificationFacade {
    pipeline_applications: PipelineApplications,
}

impl LocalNodeClassificationFacade {
    pub fn new(pipeline_applications: PipelineApplications) -> Self {
        Self {
            pipeline_applications,
        }
    }

    fn parse_split_config(configuration: &RawConfig) -> NodePropertyPredictionSplitConfig {
        let test_fraction = configuration
            .get("testFraction")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.3);

        let validation_folds = configuration
            .get("validationFolds")
            .and_then(|v| v.as_u64())
            .map(|v| v as usize)
            .unwrap_or(3);

        NodePropertyPredictionSplitConfig::new(test_fraction, validation_folds)
            .unwrap_or_else(|e| panic!("{e}"))
    }

    fn parse_auto_tuning_config(configuration: &RawConfig) -> AutoTuningConfig {
        let max_trials = configuration
            .get("maxTrials")
            .and_then(|v| v.as_u64())
            .map(|v| v as usize)
            .unwrap_or(AutoTuningConfig::MAX_TRIALS);

        AutoTuningConfig::new(max_trials).unwrap_or_else(|e| panic!("{e}"))
    }
}

impl NodeClassificationFacade for LocalNodeClassificationFacade {
    fn add_logistic_regression(
        &self,
        pipeline_name: &str,
        _configuration: RawConfig,
    ) -> Vec<NodePipelineInfoResult> {
        let pipeline_name = PipelineName::parse(pipeline_name).unwrap_or_else(|e| panic!("{e}"));
        vec![self.pipeline_applications.add_training_method(
            &pipeline_name,
            TrainingMethod::LogisticRegression,
            true,
        )]
    }

    fn add_mlp(
        &self,
        pipeline_name: &str,
        _configuration: RawConfig,
    ) -> Vec<NodePipelineInfoResult> {
        let pipeline_name = PipelineName::parse(pipeline_name).unwrap_or_else(|e| panic!("{e}"));
        vec![self.pipeline_applications.add_training_method(
            &pipeline_name,
            TrainingMethod::MLPClassification,
            true,
        )]
    }

    fn add_node_property(
        &self,
        pipeline_name: &str,
        task_name: &str,
        procedure_config: RawConfig,
    ) -> Vec<NodePipelineInfoResult> {
        let pipeline_name = PipelineName::parse(pipeline_name).unwrap_or_else(|e| panic!("{e}"));
        vec![self
            .pipeline_applications
            .add_node_property_to_node_classification_pipeline(
                &pipeline_name,
                task_name,
                procedure_config,
            )]
    }

    fn add_random_forest(
        &self,
        pipeline_name: &str,
        _configuration: RawConfig,
    ) -> Vec<NodePipelineInfoResult> {
        let pipeline_name = PipelineName::parse(pipeline_name).unwrap_or_else(|e| panic!("{e}"));
        vec![self.pipeline_applications.add_training_method(
            &pipeline_name,
            TrainingMethod::RandomForestClassification,
            true,
        )]
    }

    fn configure_auto_tuning(
        &self,
        pipeline_name: &str,
        configuration: RawConfig,
    ) -> Vec<NodePipelineInfoResult> {
        let pipeline_name = PipelineName::parse(pipeline_name).unwrap_or_else(|e| panic!("{e}"));
        let config = Self::parse_auto_tuning_config(&configuration);
        vec![self
            .pipeline_applications
            .configure_auto_tuning(&pipeline_name, config, true)]
    }

    fn configure_split(
        &self,
        pipeline_name: &str,
        configuration: RawConfig,
    ) -> Vec<NodePipelineInfoResult> {
        let pipeline_name = PipelineName::parse(pipeline_name).unwrap_or_else(|e| panic!("{e}"));
        let config = Self::parse_split_config(&configuration);
        vec![self
            .pipeline_applications
            .configure_node_property_split(&pipeline_name, config, true)]
    }

    fn create_pipeline(&self, pipeline_name: &str) -> Vec<NodePipelineInfoResult> {
        let pipeline_name = PipelineName::parse(pipeline_name).unwrap_or_else(|e| panic!("{e}"));
        vec![self
            .pipeline_applications
            .create_node_classification_training_pipeline(&pipeline_name)]
    }

    fn mutate(&self, graph_name: &str, configuration: RawConfig) -> Vec<PredictMutateResult> {
        self.pipeline_applications
            .node_classification_predict_mutate(graph_name, configuration)
    }

    fn mutate_task_frame_plan(&self, graph_name: &str, configuration: RawConfig) -> Vec<TaskFrame> {
        self.pipeline_applications
            .node_classification_predict_mutate_task_frame_plan(graph_name, &configuration)
    }

    fn mutate_estimate(
        &self,
        graph_name_or_configuration: Value,
        raw_configuration: RawConfig,
    ) -> Vec<MemoryEstimationResult> {
        self.pipeline_applications
            .node_classification_predict_mutate_estimate(
                graph_name_or_configuration,
                raw_configuration,
            )
    }

    fn select_features(
        &self,
        pipeline_name: &str,
        node_feature_steps: Value,
    ) -> Vec<NodePipelineInfoResult> {
        let pipeline_name = PipelineName::parse(pipeline_name).unwrap_or_else(|e| panic!("{e}"));
        let steps = match node_feature_steps {
            Value::Array(values) => values,
            _ => panic!("select_features expects a JSON array"),
        };

        let mut feature_steps = Vec::with_capacity(steps.len());
        for v in steps {
            let prop = v
                .as_str()
                .unwrap_or_else(|| panic!("feature entry must be a string"));
            feature_steps.push(NodeFeatureStep::of(prop));
        }

        vec![self
            .pipeline_applications
            .select_features_for_classification(&pipeline_name, feature_steps)]
    }

    fn stream(
        &self,
        graph_name: &str,
        configuration: RawConfig,
    ) -> Vec<NodeClassificationStreamResult> {
        self.pipeline_applications
            .node_classification_predict_stream(graph_name, configuration)
    }

    fn stream_estimate(
        &self,
        graph_name_or_configuration: Value,
        raw_configuration: RawConfig,
    ) -> Vec<MemoryEstimationResult> {
        self.pipeline_applications
            .node_classification_predict_stream_estimate(
                graph_name_or_configuration,
                raw_configuration,
            )
    }

    fn train(
        &self,
        graph_name: &str,
        configuration: RawConfig,
    ) -> Vec<NodeClassificationPipelineTrainResult> {
        self.pipeline_applications
            .node_classification_train(graph_name, configuration)
    }

    fn train_estimate(
        &self,
        graph_name_or_configuration: Value,
        raw_configuration: RawConfig,
    ) -> Vec<MemoryEstimationResult> {
        self.pipeline_applications
            .node_classification_train_estimate(graph_name_or_configuration, raw_configuration)
    }

    fn write(&self, graph_name: &str, configuration: RawConfig) -> Vec<WriteResult> {
        self.pipeline_applications
            .node_classification_predict_write(graph_name, configuration)
    }

    fn write_estimate(
        &self,
        graph_name_or_configuration: Value,
        raw_configuration: RawConfig,
    ) -> Vec<MemoryEstimationResult> {
        self.pipeline_applications
            .node_classification_predict_write_estimate(
                graph_name_or_configuration,
                raw_configuration,
            )
    }
}

// Regression facade mirrors node classification behavior.
pub struct LocalNodeRegressionFacade {
    pipeline_applications: PipelineApplications,
}

impl LocalNodeRegressionFacade {
    pub fn new(pipeline_applications: PipelineApplications) -> Self {
        Self {
            pipeline_applications,
        }
    }

    fn parse_split_config(configuration: &RawConfig) -> NodePropertyPredictionSplitConfig {
        let test_fraction = configuration
            .get("testFraction")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.3);

        let validation_folds = configuration
            .get("validationFolds")
            .and_then(|v| v.as_u64())
            .map(|v| v as usize)
            .unwrap_or(3);

        NodePropertyPredictionSplitConfig::new(test_fraction, validation_folds)
            .unwrap_or_else(|e| panic!("{e}"))
    }

    fn parse_auto_tuning_config(configuration: &RawConfig) -> AutoTuningConfig {
        let max_trials = configuration
            .get("maxTrials")
            .and_then(|v| v.as_u64())
            .map(|v| v as usize)
            .unwrap_or(AutoTuningConfig::MAX_TRIALS);

        AutoTuningConfig::new(max_trials).unwrap_or_else(|e| panic!("{e}"))
    }
}

impl NodeRegressionFacade for LocalNodeRegressionFacade {
    fn add_logistic_regression(
        &self,
        pipeline_name: &str,
        _configuration: RawConfig,
    ) -> Vec<NodePipelineInfoResult> {
        let pipeline_name = PipelineName::parse(pipeline_name).unwrap_or_else(|e| panic!("{e}"));
        vec![self.pipeline_applications.add_training_method(
            &pipeline_name,
            TrainingMethod::LinearRegression,
            false,
        )]
    }

    fn add_node_property(
        &self,
        pipeline_name: &str,
        task_name: &str,
        procedure_config: RawConfig,
    ) -> Vec<NodePipelineInfoResult> {
        let pipeline_name = PipelineName::parse(pipeline_name).unwrap_or_else(|e| panic!("{e}"));
        vec![self
            .pipeline_applications
            .add_node_property_to_node_regression_pipeline(
                &pipeline_name,
                task_name,
                procedure_config,
            )]
    }

    fn add_random_forest(
        &self,
        pipeline_name: &str,
        _configuration: RawConfig,
    ) -> Vec<NodePipelineInfoResult> {
        let pipeline_name = PipelineName::parse(pipeline_name).unwrap_or_else(|e| panic!("{e}"));
        vec![self.pipeline_applications.add_training_method(
            &pipeline_name,
            TrainingMethod::RandomForestRegression,
            false,
        )]
    }

    fn configure_auto_tuning(
        &self,
        pipeline_name: &str,
        configuration: RawConfig,
    ) -> Vec<NodePipelineInfoResult> {
        let pipeline_name = PipelineName::parse(pipeline_name).unwrap_or_else(|e| panic!("{e}"));
        let config = Self::parse_auto_tuning_config(&configuration);
        vec![self
            .pipeline_applications
            .configure_auto_tuning(&pipeline_name, config, false)]
    }

    fn configure_split(
        &self,
        pipeline_name: &str,
        configuration: RawConfig,
    ) -> Vec<NodePipelineInfoResult> {
        let pipeline_name = PipelineName::parse(pipeline_name).unwrap_or_else(|e| panic!("{e}"));
        let config = Self::parse_split_config(&configuration);
        vec![self.pipeline_applications.configure_node_property_split(
            &pipeline_name,
            config,
            false,
        )]
    }

    fn create_pipeline(&self, pipeline_name: &str) -> Vec<NodePipelineInfoResult> {
        let pipeline_name = PipelineName::parse(pipeline_name).unwrap_or_else(|e| panic!("{e}"));
        vec![self
            .pipeline_applications
            .create_node_regression_training_pipeline(&pipeline_name)]
    }

    fn mutate(&self, graph_name: &str, configuration: RawConfig) -> Vec<PredictMutateResult> {
        self.pipeline_applications
            .node_regression_predict_mutate(graph_name, configuration)
    }

    fn mutate_task_frame_plan(&self, graph_name: &str, configuration: RawConfig) -> Vec<TaskFrame> {
        self.pipeline_applications
            .node_regression_predict_mutate_task_frame_plan(graph_name, &configuration)
    }

    fn mutate_estimate(
        &self,
        graph_name_or_configuration: Value,
        raw_configuration: RawConfig,
    ) -> Vec<MemoryEstimationResult> {
        self.pipeline_applications
            .node_regression_predict_mutate_estimate(graph_name_or_configuration, raw_configuration)
    }

    fn stream(
        &self,
        graph_name: &str,
        configuration: RawConfig,
    ) -> Vec<NodeRegressionStreamResult> {
        self.pipeline_applications
            .node_regression_predict_stream(graph_name, configuration)
    }

    fn stream_estimate(
        &self,
        graph_name_or_configuration: Value,
        raw_configuration: RawConfig,
    ) -> Vec<MemoryEstimationResult> {
        self.pipeline_applications
            .node_regression_predict_stream_estimate(graph_name_or_configuration, raw_configuration)
    }

    fn select_features(
        &self,
        pipeline_name: &str,
        feature_properties: Value,
    ) -> Vec<NodePipelineInfoResult> {
        let pipeline_name = PipelineName::parse(pipeline_name).unwrap_or_else(|e| panic!("{e}"));
        let steps = match feature_properties {
            Value::Array(values) => values,
            _ => panic!("select_features expects a JSON array"),
        };

        let mut feature_steps = Vec::with_capacity(steps.len());
        for v in steps {
            let prop = v
                .as_str()
                .unwrap_or_else(|| panic!("feature entry must be a string"));
            feature_steps.push(NodeFeatureStep::of(prop));
        }

        vec![self
            .pipeline_applications
            .select_features_for_regression(&pipeline_name, feature_steps)]
    }

    fn train(
        &self,
        graph_name: &str,
        configuration: RawConfig,
    ) -> Vec<NodeRegressionPipelineTrainResult> {
        self.pipeline_applications
            .node_regression_train(graph_name, configuration)
    }

    fn train_estimate(
        &self,
        graph_name_or_configuration: Value,
        raw_configuration: RawConfig,
    ) -> Vec<MemoryEstimationResult> {
        self.pipeline_applications
            .node_regression_train_estimate(graph_name_or_configuration, raw_configuration)
    }
}

#[cfg(test)]
mod top_level_facade_tests {
    use super::*;
    use crate::projection::eval::pipeline::{
        LinkPredictionTrainingPipeline, NodeClassificationTrainingPipeline,
    };

    #[test]
    fn top_level_list_and_exists_use_pipeline_applications() {
        let catalog = Arc::new(PipelineCatalog::new());
        let facade = LocalPipelinesProcedureFacade::new(
            RequestScopedDependencies::new(User::from("alice")),
            Arc::clone(&catalog),
        );

        facade.link_prediction().create_pipeline("lp1");
        facade.node_classification().create_pipeline("nc1");

        let all = facade.list(LocalPipelinesProcedureFacade::NO_VALUE);
        let names: Vec<_> = all
            .iter()
            .map(|result| result.pipeline_name.as_str())
            .collect();
        assert_eq!(names, vec!["lp1", "nc1"]);
        assert!(all.iter().all(|result| !result.pipeline_info.is_empty()));

        let single = facade.list("nc1");
        assert_eq!(single.len(), 1);
        assert_eq!(single[0].pipeline_name, "nc1");
        assert_eq!(
            single[0].pipeline_type,
            NodeClassificationTrainingPipeline::PIPELINE_TYPE
        );

        let exists = facade.exists("lp1");
        assert_eq!(exists.len(), 1);
        assert!(exists[0].exists);
        assert_eq!(
            exists[0].pipeline_type,
            LinkPredictionTrainingPipeline::PIPELINE_TYPE
        );

        let missing = facade.exists("missing");
        assert_eq!(missing.len(), 1);
        assert!(!missing[0].exists);
        assert_eq!(missing[0].pipeline_type, "n/a");
        assert!(facade.list("missing").is_empty());
    }

    #[test]
    fn top_level_drop_silences_missing_when_requested() {
        let catalog = Arc::new(PipelineCatalog::new());
        let facade = LocalPipelinesProcedureFacade::new(
            RequestScopedDependencies::new(User::from("alice")),
            Arc::clone(&catalog),
        );

        assert!(facade.drop("missing", false).is_empty());

        facade.link_prediction().create_pipeline("lp1");
        let dropped = facade.drop("lp1", false);
        assert_eq!(dropped.len(), 1);
        assert_eq!(dropped[0].pipeline_name, "lp1");
        assert!(facade.list("lp1").is_empty());
    }

    #[test]
    #[should_panic(expected = "does not exist")]
    fn top_level_drop_accepts_missing_failure() {
        let catalog = Arc::new(PipelineCatalog::new());
        let facade = LocalPipelinesProcedureFacade::new(
            RequestScopedDependencies::new(User::from("alice")),
            Arc::clone(&catalog),
        );

        facade.drop("missing", true);
    }
}

#[cfg(test)]
mod executor_backed_facade_tests {
    use super::*;
    use crate::task::runtime::TaskFrameKind;
    use crate::types::graph_store::DefaultGraphStore;
    use crate::types::random::{RandomGraphConfig, RandomRelationshipConfig};

    fn facade_with_graph() -> LocalPipelinesProcedureFacade {
        let pipeline_catalog = Arc::new(PipelineCatalog::new());
        let graph_catalog: Arc<dyn GraphCatalog> = Arc::new(InMemoryGraphCatalog::new());
        let graph = Arc::new(
            DefaultGraphStore::random(&RandomGraphConfig {
                seed: Some(7),
                node_count: 8,
                relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
                ..RandomGraphConfig::default()
            })
            .expect("random graph generation"),
        );
        graph_catalog.set("graph", graph);

        LocalPipelinesProcedureFacade::new(
            RequestScopedDependencies::with_graph_catalog(User::from("alice"), graph_catalog),
            pipeline_catalog,
        )
    }

    fn graph_name_config() -> Value {
        Value::Object(serde_json::Map::from_iter([(
            "graphName".to_string(),
            Value::String("graph".to_string()),
        )]))
    }

    #[test]
    #[should_panic(expected = "linkPrediction.train is an executor-backed pipeline procedure")]
    fn link_prediction_train_fails_fast_until_wired() {
        let catalog = Arc::new(PipelineCatalog::new());
        let facade = LocalPipelinesProcedureFacade::new(
            RequestScopedDependencies::new(User::from("alice")),
            Arc::clone(&catalog),
        );

        facade.link_prediction().train("graph", AnyMap::new());
    }

    #[test]
    #[should_panic(expected = "Graph not found")]
    fn node_classification_stream_fails_fast_until_wired() {
        let catalog = Arc::new(PipelineCatalog::new());
        let facade = LocalPipelinesProcedureFacade::new(
            RequestScopedDependencies::new(User::from("alice")),
            Arc::clone(&catalog),
        );

        facade.node_classification().stream("graph", AnyMap::new());
    }

    #[test]
    #[should_panic(expected = "Graph not found")]
    fn node_classification_mutate_fails_on_missing_graph() {
        let catalog = Arc::new(PipelineCatalog::new());
        let facade = LocalPipelinesProcedureFacade::new(
            RequestScopedDependencies::new(User::from("alice")),
            Arc::clone(&catalog),
        );

        facade.node_classification().mutate(
            "graph",
            AnyMap::from([(
                "mutateProperty".to_string(),
                Value::String("pred".to_string()),
            )]),
        );
    }

    #[test]
    #[should_panic(expected = "Graph not found")]
    fn node_classification_write_fails_on_missing_graph() {
        let catalog = Arc::new(PipelineCatalog::new());
        let facade = LocalPipelinesProcedureFacade::new(
            RequestScopedDependencies::new(User::from("alice")),
            Arc::clone(&catalog),
        );

        facade.node_classification().write(
            "graph",
            AnyMap::from([(
                "writeProperty".to_string(),
                Value::String("pred".to_string()),
            )]),
        );
    }

    #[test]
    #[should_panic(expected = "Graph not found")]
    fn node_regression_mutate_fails_fast_until_wired() {
        let catalog = Arc::new(PipelineCatalog::new());
        let facade = LocalPipelinesProcedureFacade::new(
            RequestScopedDependencies::new(User::from("alice")),
            Arc::clone(&catalog),
        );

        facade.node_regression().mutate("graph", AnyMap::new());
    }

    #[test]
    #[should_panic(expected = "Graph not found")]
    fn node_regression_stream_fails_on_missing_graph() {
        let catalog = Arc::new(PipelineCatalog::new());
        let facade = LocalPipelinesProcedureFacade::new(
            RequestScopedDependencies::new(User::from("alice")),
            Arc::clone(&catalog),
        );

        facade.node_regression().stream("graph", AnyMap::new());
    }

    #[test]
    fn link_prediction_estimates_use_graph_dimensions() {
        let facade = facade_with_graph();

        let mutate = facade
            .link_prediction()
            .mutate_estimate(Value::String("graph".to_string()), AnyMap::new());
        let stream = facade
            .link_prediction()
            .stream_estimate(Value::String("graph".to_string()), AnyMap::new());
        let train = facade
            .link_prediction()
            .train_estimate(Value::String("graph".to_string()), AnyMap::new());

        assert_eq!(mutate.len(), 1);
        assert_eq!(stream.len(), 1);
        assert_eq!(train.len(), 1);
        assert!(mutate[0].memory_usage() > 0);
        assert!(stream[0].memory_usage() > 0);
        assert!(train[0].memory_usage() > 0);
    }

    #[test]
    fn node_classification_estimates_use_graph_dimensions() {
        let facade = facade_with_graph();

        let mutate = facade
            .node_classification()
            .mutate_estimate(Value::String("graph".to_string()), AnyMap::new());
        let stream = facade
            .node_classification()
            .stream_estimate(Value::String("graph".to_string()), AnyMap::new());
        let train = facade
            .node_classification()
            .train_estimate(Value::String("graph".to_string()), AnyMap::new());
        let write = facade
            .node_classification()
            .write_estimate(Value::String("graph".to_string()), AnyMap::new());

        assert_eq!(mutate.len(), 1);
        assert_eq!(stream.len(), 1);
        assert_eq!(train.len(), 1);
        assert_eq!(write.len(), 1);
        assert!(mutate[0].memory_usage() > 0);
        assert!(stream[0].memory_usage() > 0);
        assert!(train[0].memory_usage() > 0);
        assert!(write[0].memory_usage() > 0);
    }

    #[test]
    fn node_regression_estimates_use_graph_dimensions() {
        let facade = facade_with_graph();

        let mutate = facade
            .node_regression()
            .mutate_estimate(Value::String("graph".to_string()), AnyMap::new());
        let stream = facade
            .node_regression()
            .stream_estimate(Value::String("graph".to_string()), AnyMap::new());
        let train = facade
            .node_regression()
            .train_estimate(Value::String("graph".to_string()), AnyMap::new());

        assert_eq!(mutate.len(), 1);
        assert_eq!(stream.len(), 1);
        assert_eq!(train.len(), 1);
        assert!(mutate[0].memory_usage() > 0);
        assert!(stream[0].memory_usage() > 0);
        assert!(train[0].memory_usage() > 0);
    }

    #[test]
    fn node_regression_estimates_accept_embedded_graph_name() {
        let facade = facade_with_graph();

        let mutate = facade
            .node_regression()
            .mutate_estimate(graph_name_config(), AnyMap::new());

        assert_eq!(mutate.len(), 1);
        assert!(mutate[0].memory_usage() > 0);
    }

    #[test]
    fn node_classification_mutate_task_frame_plan_uses_staged_contract() {
        let facade = facade_with_graph();

        let plan = facade
            .node_classification()
            .mutate_task_frame_plan("graph", AnyMap::new());

        assert_eq!(plan.len(), 3);
        assert!(plan[0].pipeline().ends_with("::Seed"));
        assert!(plan[1].pipeline().ends_with("::ComputeGraph"));
        assert!(plan[2].pipeline().ends_with("::Persist"));
        assert_eq!(plan[1].image_spec().kind(), TaskFrameKind::MachineLearning);
    }

    #[test]
    fn node_regression_mutate_task_frame_plan_uses_staged_contract() {
        let facade = facade_with_graph();

        let plan = facade
            .node_regression()
            .mutate_task_frame_plan("graph", AnyMap::new());

        assert_eq!(plan.len(), 3);
        assert!(plan[0].pipeline().ends_with("::Seed"));
        assert!(plan[1].pipeline().ends_with("::ComputeGraph"));
        assert!(plan[2].pipeline().ends_with("::Persist"));
        assert_eq!(plan[1].image_spec().kind(), TaskFrameKind::MachineLearning);
    }
}

#[cfg(test)]
mod node_classification_facade_tests {
    use super::*;

    #[test]
    fn node_classification_create_and_configure_pipeline() {
        let catalog = Arc::new(PipelineCatalog::new());
        let facade = LocalPipelinesProcedureFacade::new(
            RequestScopedDependencies::new(User::from("alice")),
            Arc::clone(&catalog),
        );

        let created = facade.node_classification().create_pipeline("p1");
        assert_eq!(created.len(), 1);
        assert_eq!(created[0].name, "p1");
        assert!(created[0].node_property_steps.is_empty());
        assert!(created[0].feature_properties.is_empty());

        let configured = facade.node_classification().configure_split(
            "p1",
            AnyMap::from([
                ("testFraction".to_string(), Value::from(0.2)),
                ("validationFolds".to_string(), Value::from(5)),
            ]),
        );
        assert_eq!(
            configured[0].split_config.get("testFraction"),
            Some(&Value::String("0.2".to_string()))
        );
        assert_eq!(
            configured[0].split_config.get("validationFolds"),
            Some(&Value::String("5".to_string()))
        );
    }

    #[test]
    fn node_classification_add_node_property_and_select_features() {
        let catalog = Arc::new(PipelineCatalog::new());
        let facade = LocalPipelinesProcedureFacade::new(
            RequestScopedDependencies::new(User::from("alice")),
            Arc::clone(&catalog),
        );

        facade.node_classification().create_pipeline("p1");

        let updated = facade.node_classification().add_node_property(
            "p1",
            "gds.debug.writeConstantDouble.mutate",
            AnyMap::from([
                (
                    "mutateProperty".to_string(),
                    Value::String("pagerank".to_string()),
                ),
                ("value".to_string(), Value::from(1.0)),
            ]),
        );

        assert_eq!(updated.len(), 1);
        assert_eq!(updated[0].node_property_steps.len(), 1);

        let with_features = facade.node_classification().select_features(
            "p1",
            Value::Array(vec![Value::String("pagerank".to_string())]),
        );
        assert_eq!(
            with_features[0].feature_properties,
            vec!["pagerank".to_string()]
        );
    }
}

#[cfg(test)]
mod link_prediction_facade_tests {
    use super::*;

    #[test]
    fn link_prediction_create_add_feature_and_configure_split() {
        let catalog = Arc::new(PipelineCatalog::new());
        let facade = LocalPipelinesProcedureFacade::new(
            RequestScopedDependencies::new(User::from("alice")),
            Arc::clone(&catalog),
        );

        let created = facade.link_prediction().create_pipeline("lp1");
        assert_eq!(created.len(), 1);
        assert_eq!(created[0].name, "lp1");
        assert!(created[0].feature_steps.is_empty());

        let with_feature = facade.link_prediction().add_feature(
            "lp1",
            "COSINE",
            AnyMap::from([(
                "nodeProperties".to_string(),
                Value::Array(vec![Value::String("embedding".to_string())]),
            )]),
        );
        assert_eq!(with_feature.len(), 1);
        assert_eq!(with_feature[0].feature_steps.len(), 1);
        assert_eq!(
            with_feature[0].feature_steps[0].get("name"),
            Some(&Value::String("COSINE".to_string()))
        );

        let configured = facade.link_prediction().configure_split(
            "lp1",
            AnyMap::from([
                ("testFraction".to_string(), Value::from(0.2)),
                ("trainFraction".to_string(), Value::from(0.3)),
                ("validationFolds".to_string(), Value::from(5)),
            ]),
        );

        assert_eq!(
            configured[0].split_config.get("testFraction"),
            Some(&Value::from(0.2))
        );
        assert_eq!(
            configured[0].split_config.get("trainFraction"),
            Some(&Value::from(0.3))
        );
        assert_eq!(
            configured[0].split_config.get("validationFolds"),
            Some(&Value::from(5))
        );
    }

    #[test]
    fn link_prediction_add_node_property() {
        let catalog = Arc::new(PipelineCatalog::new());
        let facade = LocalPipelinesProcedureFacade::new(
            RequestScopedDependencies::new(User::from("alice")),
            Arc::clone(&catalog),
        );

        facade.link_prediction().create_pipeline("lp1");

        let updated = facade.link_prediction().add_node_property(
            "lp1",
            "gds.debug.writeConstantDouble.mutate",
            AnyMap::from([
                (
                    "mutateProperty".to_string(),
                    Value::String("pagerank".to_string()),
                ),
                ("value".to_string(), Value::from(1.0)),
            ]),
        );

        assert_eq!(updated.len(), 1);
        assert_eq!(updated[0].node_property_steps.len(), 1);
    }
}

#[cfg(test)]
mod node_regression_facade_tests {
    use super::*;

    #[test]
    fn node_regression_create_configure_and_select_features() {
        let catalog = Arc::new(PipelineCatalog::new());
        let facade = LocalPipelinesProcedureFacade::new(
            RequestScopedDependencies::new(User::from("alice")),
            Arc::clone(&catalog),
        );

        let created = facade.node_regression().create_pipeline("nr1");
        assert_eq!(created.len(), 1);
        assert_eq!(created[0].name, "nr1");
        assert!(created[0].node_property_steps.is_empty());
        assert!(created[0].feature_properties.is_empty());

        let with_property = facade.node_regression().add_node_property(
            "nr1",
            "gds.debug.writeConstantDouble.mutate",
            AnyMap::from([
                (
                    "mutateProperty".to_string(),
                    Value::String("score".to_string()),
                ),
                ("value".to_string(), Value::from(1.0)),
            ]),
        );
        assert_eq!(with_property[0].node_property_steps.len(), 1);

        let configured = facade.node_regression().configure_split(
            "nr1",
            AnyMap::from([
                ("testFraction".to_string(), Value::from(0.2)),
                ("validationFolds".to_string(), Value::from(5)),
            ]),
        );
        assert_eq!(
            configured[0].split_config.get("testFraction"),
            Some(&Value::String("0.2".to_string()))
        );

        let with_features = facade.node_regression().select_features(
            "nr1",
            Value::Array(vec![Value::String("score".to_string())]),
        );
        assert_eq!(
            with_features[0].feature_properties,
            vec!["score".to_string()]
        );
    }
}
