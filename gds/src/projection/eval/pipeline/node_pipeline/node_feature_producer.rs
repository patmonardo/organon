use super::{NodePropertyPipelineBaseTrainConfig, NodePropertyTrainingPipeline};
use crate::ml::models::{Features, FeaturesFactory};
use crate::projection::eval::pipeline::{
    ExecutableNodePropertyStep, NodePropertyStepExecutor, NodePropertyStepExecutorError,
};
use crate::types::graph_store::DefaultGraphStore;
use crate::types::graph_store::GraphStore;
use std::sync::Arc;

// Features and FeaturesFactory are now backed by crate::ml::models.

/// Producer for extracting node features in ML pipelines.
///
/// This struct orchestrates:
/// 1. Executing node property steps to compute intermediate properties
/// 2. Validating that required feature properties exist
/// 3. Extracting features (eager or lazy) from the graph
/// 4. Cleaning up intermediate properties
///
/// **Java equivalent**: `NodeFeatureProducer<PIPELINE_CONFIG extends NodePropertyPipelineBaseTrainConfig>`
pub struct NodeFeatureProducer<C: NodePropertyPipelineBaseTrainConfig> {
    step_executor: NodePropertyStepExecutor,
    graph_store: Arc<DefaultGraphStore>,
    train_config: C,
}

impl<C: NodePropertyPipelineBaseTrainConfig> NodeFeatureProducer<C> {
    /// Creates a new NodeFeatureProducer.
    ///
    /// **Java**: Private constructor
    fn new(
        step_executor: NodePropertyStepExecutor,
        graph_store: Arc<DefaultGraphStore>,
        train_config: C,
    ) -> Self {
        Self {
            step_executor,
            graph_store,
            train_config,
        }
    }

    /// Factory method to create a NodeFeatureProducer.
    ///
    /// **Java**:
    /// ```java
    /// public static <PIPELINE_CONFIG extends NodePropertyPipelineBaseTrainConfig>
    /// NodeFeatureProducer<PIPELINE_CONFIG> create(
    ///     GraphStore graphStore,
    ///     PIPELINE_CONFIG config,
    ///     ExecutionContext executionContext,
    ///     ProgressTracker progressTracker
    /// )
    /// ```
    ///
    /// **Note**: This simplified version removes ExecutionContext and ProgressTracker
    /// following the Direct Integration pattern.
    pub fn create(graph_store: Arc<DefaultGraphStore>, config: C) -> Self {
        use std::collections::HashSet;

        let node_labels_config = config.node_labels();
        let node_labels = if node_labels_config.len() == 1 && node_labels_config[0] == "*" {
            let mut labels: Vec<String> = graph_store
                .node_labels()
                .into_iter()
                .map(|l| l.name().to_string())
                .collect();
            labels.sort();
            labels
        } else {
            node_labels_config
        };

        let mut relationship_types: Vec<String> = graph_store
            .relationship_types()
            .into_iter()
            .map(|t| t.name().to_string())
            .collect();
        relationship_types.sort();

        let available_relationship_types: HashSet<String> =
            relationship_types.iter().cloned().collect();

        let concurrency = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);

        let step_executor = NodePropertyStepExecutor::new(
            node_labels,
            relationship_types,
            available_relationship_types,
            concurrency,
        );

        Self::new(step_executor, graph_store, config)
    }

    /// Executes the pipeline and extracts features.
    ///
    /// This method:
    /// 1. Executes node property steps to compute intermediate properties
    /// 2. Validates that feature properties exist
    /// 3. Creates a filtered graph with target node labels
    /// 4. Extracts features (eager or lazy depending on pipeline requirements)
    /// 5. Cleans up intermediate properties (in finally block)
    ///
    /// **Java**:
    /// ```java
    /// public Features procedureFeatures(NodePropertyTrainingPipeline pipeline) {
    ///     try {
    ///         stepExecutor.executeNodePropertySteps(pipeline.nodePropertySteps());
    ///         Collection<NodeLabel> targetNodeLabels = trainConfig.nodeLabelIdentifiers(graphStore);
    ///         pipeline.validateFeatureProperties(graphStore, targetNodeLabels);
    ///
    ///         var targetNodeLabelGraph = graphStore.getGraph(targetNodeLabels);
    ///         if (pipeline.requireEagerFeatures()) {
    ///             return FeaturesFactory.extractEagerFeatures(targetNodeLabelGraph, pipeline.featureProperties());
    ///         } else {
    ///             return FeaturesFactory.extractLazyFeatures(targetNodeLabelGraph, pipeline.featureProperties());
    ///         }
    ///     } finally {
    ///         stepExecutor.cleanupIntermediateProperties(pipeline.nodePropertySteps());
    ///     }
    /// }
    /// ```
    pub fn procedure_features<P: NodePropertyTrainingPipeline>(
        &mut self,
        pipeline: &P,
    ) -> Result<Box<dyn Features>, NodeFeatureProducerError> {
        let has_steps = !pipeline.node_property_steps().is_empty();

        // Execute node property steps to compute intermediate properties
        if has_steps {
            self.step_executor
                .execute_node_property_steps(&mut self.graph_store, pipeline.node_property_steps())
                .map_err(NodeFeatureProducerError::StepExecutionFailed)?;
        }

        // Get target node labels
        let target_node_label_set = self
            .train_config
            .validate_target_node_label_identifiers(&self.graph_store)
            .map_err(|e| NodeFeatureProducerError::TargetLabelValidationFailed(e.to_string()))?;
        let mut target_node_labels: Vec<String> = target_node_label_set
            .into_iter()
            .map(|l| l.name().to_string())
            .collect();
        target_node_labels.sort();
        let target_node_label_set = target_node_labels
            .iter()
            .map(|label| crate::types::schema::NodeLabel::of(label.as_str()))
            .collect();

        pipeline
            .validate_feature_properties(&self.graph_store, &target_node_labels)
            .map_err(|e| NodeFeatureProducerError::FeatureValidationFailed(e.to_string()))?;

        let target_graph = self.graph_store.get_graph();
        let target_node_ids = target_graph
            .iter_with_labels(&target_node_label_set)
            .map(|node_id| node_id as u64)
            .collect::<Vec<_>>();
        let features = if pipeline.require_eager_features() {
            FeaturesFactory::extract_eager_features_for_node_ids(
                target_graph,
                &pipeline.feature_properties(),
                target_node_ids,
            )
        } else {
            FeaturesFactory::extract_lazy_features_for_node_ids(
                target_graph,
                &pipeline.feature_properties(),
                target_node_ids,
            )
        };

        // Cleanup intermediate properties (only if steps executed)
        if has_steps {
            self.step_executor
                .cleanup_intermediate_properties(
                    &mut self.graph_store,
                    pipeline.node_property_steps(),
                )
                .map_err(NodeFeatureProducerError::CleanupFailed)?;
        }

        Ok(features)
    }

    /// Validates node property step context configurations.
    ///
    /// **Java**:
    /// ```java
    /// public void validateNodePropertyStepsContextConfigs(List<ExecutableNodePropertyStep> steps) {
    ///     stepExecutor.validNodePropertyStepsContextConfigs(steps);
    /// }
    /// ```
    pub fn validate_node_property_steps_context_configs(
        &self,
        steps: &[Box<dyn ExecutableNodePropertyStep>],
    ) -> Result<(), NodePropertyStepExecutorError> {
        self.step_executor
            .validate_node_property_steps_context_configs(&self.graph_store, steps)
    }
}

/// Errors that can occur during feature production.
#[derive(Debug)]
pub enum NodeFeatureProducerError {
    /// Error executing node property steps
    StepExecutionFailed(NodePropertyStepExecutorError),
    /// Error validating target labels
    TargetLabelValidationFailed(String),
    /// Error validating feature properties
    FeatureValidationFailed(String),
    /// Error filtering graph by node labels
    GraphFilterFailed(String),
    /// Error extracting features
    FeatureExtractionFailed(String),
    /// Error cleaning up intermediate properties
    CleanupFailed(NodePropertyStepExecutorError),
}

impl std::fmt::Display for NodeFeatureProducerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StepExecutionFailed(e) => {
                write!(f, "Failed to execute node property steps: {}", e)
            }
            Self::TargetLabelValidationFailed(msg) => {
                write!(f, "Target label validation failed: {}", msg)
            }
            Self::FeatureValidationFailed(msg) => write!(f, "Feature validation failed: {}", msg),
            Self::GraphFilterFailed(msg) => write!(f, "Failed to filter graph: {}", msg),
            Self::FeatureExtractionFailed(msg) => write!(f, "Failed to extract features: {}", msg),
            Self::CleanupFailed(e) => write!(f, "Failed to cleanup intermediate properties: {}", e),
        }
    }
}

impl std::error::Error for NodeFeatureProducerError {}

// Placeholder config for testing
#[derive(Clone)]
pub struct PlaceholderNodePropertyConfig;

impl NodePropertyPipelineBaseTrainConfig for PlaceholderNodePropertyConfig {
    fn pipeline(&self) -> &str {
        "placeholder"
    }
    fn target_node_labels(&self) -> Vec<String> {
        vec![]
    }
    fn target_property(&self) -> &str {
        "target"
    }
    fn random_seed(&self) -> Option<u64> {
        Some(42)
    }
}

impl NodeFeatureProducer<PlaceholderNodePropertyConfig> {
    /// Create a placeholder NodeFeatureProducer for testing.
    pub fn placeholder() -> Self {
        use crate::types::graph_store::DefaultGraphStore;
        use crate::types::random::RandomGraphConfig;
        use std::collections::HashSet;

        let config = RandomGraphConfig {
            node_count: 10,
            seed: Some(42),
            ..RandomGraphConfig::default()
        };
        let graph_store =
            Arc::new(DefaultGraphStore::random(&config).expect("Failed to generate random graph"));
        let placeholder_config = PlaceholderNodePropertyConfig;
        let node_labels = placeholder_config.node_labels();
        let step_executor = NodePropertyStepExecutor::new(node_labels, vec![], HashSet::new(), 1);

        Self::new(step_executor, graph_store, placeholder_config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::eval::pipeline::auto_tuning_config::AutoTuningConfig;
    use crate::projection::eval::pipeline::node_pipeline::{
        NodeFeatureStep, NodePropertyPredictionSplitConfig,
    };
    use crate::projection::eval::pipeline::training_pipeline::TunableTrainerConfig;
    use crate::projection::eval::pipeline::{Pipeline, PipelineValidationError, TrainingMethod};
    use crate::types::random::RandomGraphConfig;
    use serde_json::Value;
    use std::collections::HashMap;
    use std::sync::Arc;

    // Mock config for testing
    struct MockTrainConfig {
        pipeline_name: String,
        target_labels: Vec<String>,
        target_prop: String,
    }

    impl NodePropertyPipelineBaseTrainConfig for MockTrainConfig {
        fn pipeline(&self) -> &str {
            &self.pipeline_name
        }

        fn target_node_labels(&self) -> Vec<String> {
            self.target_labels.clone()
        }

        fn target_property(&self) -> &str {
            &self.target_prop
        }

        fn random_seed(&self) -> Option<u64> {
            Some(42)
        }
    }

    struct FeatureOnlyPipeline {
        feature_steps: Vec<NodeFeatureStep>,
        split_config: NodePropertyPredictionSplitConfig,
        training_parameter_space: HashMap<TrainingMethod, Vec<Box<dyn TunableTrainerConfig>>>,
        auto_tuning_config: AutoTuningConfig,
        eager_features: bool,
    }

    impl FeatureOnlyPipeline {
        fn new(feature_property: &str, eager_features: bool) -> Self {
            Self {
                feature_steps: vec![NodeFeatureStep::of(feature_property)],
                split_config: NodePropertyPredictionSplitConfig::default(),
                training_parameter_space: HashMap::new(),
                auto_tuning_config: AutoTuningConfig::default(),
                eager_features,
            }
        }
    }

    impl Pipeline for FeatureOnlyPipeline {
        type FeatureStep = NodeFeatureStep;

        fn to_map(&self) -> HashMap<String, Value> {
            HashMap::new()
        }

        fn node_property_steps(&self) -> &[Box<dyn ExecutableNodePropertyStep>] {
            &[]
        }

        fn feature_steps(&self) -> &[Self::FeatureStep] {
            &self.feature_steps
        }

        fn specific_validate_before_execution(
            &self,
            _graph_store: &DefaultGraphStore,
        ) -> Result<(), PipelineValidationError> {
            Ok(())
        }
    }

    impl crate::projection::eval::pipeline::training_pipeline::TrainingPipeline
        for FeatureOnlyPipeline
    {
        fn pipeline_type(&self) -> &str {
            "feature-only"
        }

        fn training_parameter_space(
            &self,
        ) -> &HashMap<TrainingMethod, Vec<Box<dyn TunableTrainerConfig>>> {
            &self.training_parameter_space
        }

        fn training_parameter_space_mut(
            &mut self,
        ) -> &mut HashMap<TrainingMethod, Vec<Box<dyn TunableTrainerConfig>>> {
            &mut self.training_parameter_space
        }

        fn auto_tuning_config(&self) -> &AutoTuningConfig {
            &self.auto_tuning_config
        }

        fn set_auto_tuning_config(&mut self, config: AutoTuningConfig) {
            self.auto_tuning_config = config;
        }
    }

    impl NodePropertyTrainingPipeline for FeatureOnlyPipeline {
        fn split_config(&self) -> &NodePropertyPredictionSplitConfig {
            &self.split_config
        }

        fn set_split_config(&mut self, split_config: NodePropertyPredictionSplitConfig) {
            self.split_config = split_config;
        }

        fn require_eager_features(&self) -> bool {
            self.eager_features
        }
    }

    #[test]
    fn test_create_node_feature_producer() {
        let config = RandomGraphConfig {
            seed: Some(42),
            node_count: 100,
            ..RandomGraphConfig::default()
        };
        let graph_store = Arc::new(DefaultGraphStore::random(&config).expect("random graph"));

        let config = MockTrainConfig {
            pipeline_name: "test-pipeline".to_string(),
            target_labels: vec!["Person".to_string()],
            target_prop: "label".to_string(),
        };

        let _producer = NodeFeatureProducer::create(graph_store, config);
        // Success if it doesn't panic
    }

    #[test]
    fn test_create_with_wildcard_labels() {
        let config = RandomGraphConfig {
            seed: Some(42),
            node_count: 100,
            ..RandomGraphConfig::default()
        };
        let graph_store = Arc::new(DefaultGraphStore::random(&config).expect("random graph"));

        let config = MockTrainConfig {
            pipeline_name: "test-pipeline".to_string(),
            target_labels: vec!["*".to_string()],
            target_prop: "label".to_string(),
        };

        let _producer = NodeFeatureProducer::create(graph_store, config);
        // Success if it doesn't panic
    }

    #[test]
    fn test_validate_steps_context_configs() {
        let config = RandomGraphConfig {
            seed: Some(42),
            node_count: 100,
            ..RandomGraphConfig::default()
        };
        let graph_store = Arc::new(DefaultGraphStore::random(&config).expect("random graph"));

        let config = MockTrainConfig {
            pipeline_name: "test-pipeline".to_string(),
            target_labels: vec!["Person".to_string()],
            target_prop: "label".to_string(),
        };

        let producer = NodeFeatureProducer::create(graph_store, config);

        // Empty steps should validate successfully
        let steps: Vec<Box<dyn ExecutableNodePropertyStep>> = vec![];
        assert!(producer
            .validate_node_property_steps_context_configs(&steps)
            .is_ok());
    }

    #[test]
    fn test_procedure_features_filters_to_target_node_labels() {
        let graph_store = Arc::new(
            DefaultGraphStore::random(&RandomGraphConfig {
                seed: Some(42),
                node_count: 200,
                node_labels: vec!["Person".to_string(), "Company".to_string()],
                ..RandomGraphConfig::default()
            })
            .expect("random graph"),
        );

        let target_label = graph_store
            .node_labels()
            .into_iter()
            .find(|label| {
                let count = graph_store.node_count_for_label(label);
                count > 0 && count < graph_store.node_count()
            })
            .expect("expected at least one proper target label subset");
        let target_count = graph_store.node_count_for_label(&target_label);
        let config = MockTrainConfig {
            pipeline_name: "test-pipeline".to_string(),
            target_labels: vec![target_label.name().to_string()],
            target_prop: "label".to_string(),
        };
        let pipeline = FeatureOnlyPipeline::new("random_score", true);

        let mut producer = NodeFeatureProducer::create(graph_store, config);
        let features = producer
            .procedure_features(&pipeline)
            .expect("features should extract for target labels");

        assert_eq!(features.size(), target_count);
    }
}
