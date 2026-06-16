use std::collections::HashSet;
use std::error::Error as StdError;
use std::sync::Arc;

use crate::projection::eval::pipeline::{
    PipelineTrainer, ResultToModelConverter, TrainingPipeline,
};
use crate::projection::{NodeLabel, RelationshipType};
use crate::task::concurrency::TerminationFlag;
use crate::types::graph_store::{DefaultGraphStore, GraphStore};

/// Abstract algorithm for training ML pipelines.
///
/// This trait orchestrates the complete training process:
/// 1. Propagate termination into the pipeline trainer
/// 2. Validate training parameter space
/// 3. Validate pipeline against graph
/// 4. Run pipeline trainer (model selection + training)
/// 5. Convert result to catalog model
///
/// # Direct Integration Approach
///
/// Unlike Java's extends Algorithm<RESULT>, this is a trait with a default
/// `compute()` implementation. Takes Arc<DefaultGraphStore> directly.
///
/// # Type Parameters
///
/// * `RESULT` - Training result type (before conversion to model)
/// * `MODEL` - Catalog model container type
/// * `FEATURE_STEP` - Feature step type (e.g., NodeFeatureStep)
///
/// # Java Source (PipelineTrainAlgorithm.java)
pub trait PipelineTrainAlgorithm<RESULT, MODEL, P: TrainingPipeline + ?Sized> {
    /// Access the training pipeline.
    fn pipeline(&self) -> &P;

    /// Access the graph store.
    fn graph_store(&self) -> &Arc<DefaultGraphStore>;

    /// Access the node labels for training.
    fn node_labels(&self) -> &[String];

    /// Access the relationship types for training.
    fn relationship_types(&self) -> &[String];

    /// Access the pipeline trainer.
    fn pipeline_trainer_mut(&mut self) -> &mut dyn PipelineTrainer<Result = RESULT>;

    /// Access the result-to-model converter.
    fn result_to_model_converter(&self) -> &dyn ResultToModelConverter<MODEL, RESULT>;

    /// Access the termination flag for this training run.
    ///
    /// Java gets this from `Algorithm.terminationFlag`; Rust implementations can
    /// override when wired into the broader algorithm runtime.
    fn termination_flag(&self) -> TerminationFlag {
        TerminationFlag::running_true()
    }

    /// Execute the complete training process (template method).
    ///
    /// This orchestrates:
    /// 1. Validate training parameter space (at least one model candidate)
    /// 2. Validate pipeline against graph
    /// 3. Capture original schema (before node property steps)
    /// 4. Run pipeline trainer (model selection + training)
    /// 5. Convert result to catalog model
    ///
    fn compute(&mut self) -> Result<MODEL, PipelineTrainAlgorithmError> {
        // 1. Propagate termination into the trainer, matching Java PipelineTrainAlgorithm.
        let termination_flag = self.termination_flag();
        self.pipeline_trainer_mut()
            .set_termination_flag(termination_flag);

        // 2. Validate training parameter space
        self.pipeline()
            .validate_training_parameter_space()
            .map_err(PipelineTrainAlgorithmError::ValidationFailed)?;

        // 3. Validate pipeline before execution
        self.pipeline()
            .validate_before_execution(self.graph_store(), self.node_labels())
            .map_err(|e| PipelineTrainAlgorithmError::ValidationFailed(Box::new(e)))?;

        // 4. Capture original schema (before node property steps)
        let node_labels_set: HashSet<NodeLabel> = self
            .node_labels()
            .iter()
            .map(|s| NodeLabel::of(s.as_str()))
            .collect();
        let rel_types_set: HashSet<RelationshipType> = self
            .relationship_types()
            .iter()
            .map(|s| RelationshipType::of(s.as_str()))
            .collect();

        let original_schema = self
            .graph_store()
            .schema()
            .filter_node_labels(&node_labels_set)
            .filter_relationship_types(&rel_types_set);

        // 5. Run pipeline trainer
        let pipeline_train_result = self
            .pipeline_trainer_mut()
            .run()
            .map_err(PipelineTrainAlgorithmError::TrainingFailed)?;

        // 6. Convert result to catalog model
        let model = self
            .result_to_model_converter()
            .to_model(pipeline_train_result, &original_schema);

        Ok(model)
    }
}

/// Errors that can occur during pipeline training.
#[derive(Debug)]
pub enum PipelineTrainAlgorithmError {
    /// Pipeline validation failed.
    ValidationFailed(Box<dyn StdError + Send + Sync>),

    /// Training execution failed.
    TrainingFailed(Box<dyn StdError + Send + Sync>),

    /// Model conversion failed.
    ConversionFailed(String),
}

impl std::fmt::Display for PipelineTrainAlgorithmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ValidationFailed(e) => write!(f, "Pipeline validation failed: {}", e),
            Self::TrainingFailed(e) => write!(f, "Training failed: {}", e),
            Self::ConversionFailed(msg) => write!(f, "Model conversion failed: {}", msg),
        }
    }
}

impl StdError for PipelineTrainAlgorithmError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::ValidationFailed(e) | Self::TrainingFailed(e) => Some(e.as_ref()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::eval::pipeline::{
        AutoTuningConfig, ExecutableNodePropertyStep, FeatureStep, Pipeline,
        PipelineValidationError, TrainingMethod, TunableTrainerConfig,
    };
    use crate::types::random::random_graph::RandomGraphConfig;
    use std::collections::HashMap;

    #[test]
    fn test_error_display() {
        let error = PipelineTrainAlgorithmError::ConversionFailed("bad format".to_string());
        let display = format!("{}", error);
        assert!(display.contains("Model conversion failed"));
        assert!(display.contains("bad format"));
    }

    #[derive(Clone)]
    struct TestTrainerConfig;

    impl TunableTrainerConfig for TestTrainerConfig {
        fn training_method(&self) -> TrainingMethod {
            TrainingMethod::LogisticRegression
        }

        fn is_concrete(&self) -> bool {
            true
        }

        fn to_map(&self) -> HashMap<String, serde_json::Value> {
            HashMap::new()
        }
    }

    struct TestFeatureStep;

    impl FeatureStep for TestFeatureStep {
        fn input_node_properties(&self) -> &[String] {
            &[]
        }

        fn name(&self) -> &str {
            "test"
        }

        fn configuration(&self) -> &HashMap<String, serde_json::Value> {
            static EMPTY: std::sync::OnceLock<HashMap<String, serde_json::Value>> =
                std::sync::OnceLock::new();
            EMPTY.get_or_init(HashMap::new)
        }

        fn to_map(&self) -> HashMap<String, serde_json::Value> {
            HashMap::new()
        }
    }

    struct TestPipeline {
        feature_steps: Vec<TestFeatureStep>,
        node_property_steps: Vec<Box<dyn ExecutableNodePropertyStep>>,
        parameter_space: HashMap<TrainingMethod, Vec<Box<dyn TunableTrainerConfig>>>,
        auto_tuning_config: AutoTuningConfig,
    }

    impl TestPipeline {
        fn new() -> Self {
            let mut parameter_space: HashMap<TrainingMethod, Vec<Box<dyn TunableTrainerConfig>>> =
                HashMap::new();
            parameter_space.insert(
                TrainingMethod::LogisticRegression,
                vec![Box::new(TestTrainerConfig)],
            );

            Self {
                feature_steps: Vec::new(),
                node_property_steps: Vec::new(),
                parameter_space,
                auto_tuning_config: AutoTuningConfig::default(),
            }
        }
    }

    impl Pipeline for TestPipeline {
        type FeatureStep = TestFeatureStep;

        fn node_property_steps(&self) -> &[Box<dyn ExecutableNodePropertyStep>] {
            &self.node_property_steps
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

        fn to_map(&self) -> HashMap<String, serde_json::Value> {
            HashMap::new()
        }
    }

    impl TrainingPipeline for TestPipeline {
        fn pipeline_type(&self) -> &str {
            "test"
        }

        fn training_parameter_space(
            &self,
        ) -> &HashMap<TrainingMethod, Vec<Box<dyn TunableTrainerConfig>>> {
            &self.parameter_space
        }

        fn training_parameter_space_mut(
            &mut self,
        ) -> &mut HashMap<TrainingMethod, Vec<Box<dyn TunableTrainerConfig>>> {
            &mut self.parameter_space
        }

        fn auto_tuning_config(&self) -> &AutoTuningConfig {
            &self.auto_tuning_config
        }

        fn set_auto_tuning_config(&mut self, config: AutoTuningConfig) {
            self.auto_tuning_config = config;
        }
    }

    struct TestTrainer {
        saw_terminated_flag: bool,
    }

    impl PipelineTrainer for TestTrainer {
        type Result = &'static str;

        fn set_termination_flag(&mut self, termination_flag: TerminationFlag) {
            self.saw_terminated_flag = !termination_flag.running();
        }

        fn run(&mut self) -> Result<Self::Result, Box<dyn StdError + Send + Sync>> {
            Ok("trained")
        }
    }

    struct TestConverter;

    impl ResultToModelConverter<String, &'static str> for TestConverter {
        fn to_model(
            &self,
            result: &'static str,
            _original_schema: &crate::types::schema::GraphSchema,
        ) -> String {
            result.to_string()
        }
    }

    struct TestTrainAlgorithm {
        pipeline: TestPipeline,
        graph_store: Arc<DefaultGraphStore>,
        trainer: TestTrainer,
        converter: TestConverter,
        node_labels: Vec<String>,
        relationship_types: Vec<String>,
    }

    impl PipelineTrainAlgorithm<&'static str, String, TestPipeline> for TestTrainAlgorithm {
        fn pipeline(&self) -> &TestPipeline {
            &self.pipeline
        }

        fn graph_store(&self) -> &Arc<DefaultGraphStore> {
            &self.graph_store
        }

        fn node_labels(&self) -> &[String] {
            &self.node_labels
        }

        fn relationship_types(&self) -> &[String] {
            &self.relationship_types
        }

        fn pipeline_trainer_mut(&mut self) -> &mut dyn PipelineTrainer<Result = &'static str> {
            &mut self.trainer
        }

        fn result_to_model_converter(&self) -> &dyn ResultToModelConverter<String, &'static str> {
            &self.converter
        }

        fn termination_flag(&self) -> TerminationFlag {
            TerminationFlag::stop_running()
        }
    }

    #[test]
    fn test_compute_propagates_termination_flag_to_trainer() {
        let graph_config = RandomGraphConfig {
            node_count: 4,
            seed: Some(42),
            ..RandomGraphConfig::default()
        };
        let graph_store = Arc::new(DefaultGraphStore::random(&graph_config).expect("random graph"));
        let mut algorithm = TestTrainAlgorithm {
            pipeline: TestPipeline::new(),
            graph_store,
            trainer: TestTrainer {
                saw_terminated_flag: false,
            },
            converter: TestConverter,
            node_labels: Vec::new(),
            relationship_types: Vec::new(),
        };

        let model = PipelineTrainAlgorithm::compute(&mut algorithm).expect("compute succeeds");

        assert_eq!(model, "trained");
        assert!(algorithm.trainer.saw_terminated_flag);
    }
}
