use super::node_regression_pipeline_train_config::NodeRegressionPipelineTrainConfig;
use super::node_regression_train_result::NodeRegressionTrainResult;
use super::node_regression_training_pipeline::NodeRegressionTrainingPipeline;
use crate::collections::HugeDoubleArray;
use crate::core::model::ModelCatalog;
use crate::ml::metrics::regression::RegressionMetric;
use crate::ml::metrics::Metric;
use crate::ml::models::automl::{
    create_trainer_config_from_map, RandomSearch, TunableTrainerConfig as MlTunableTrainerConfig,
};
use crate::ml::models::base::TrainerConfigTrait;
use crate::ml::models::{Features, RegressionTrainerFactory, Regressor};
use crate::ml::node_regression::NodeSplitter;
use crate::ml::splitting::TrainingExamplesSplit;
use crate::ml::training::{CrossValidation, TrainingStatistics};
use crate::projection::eval::pipeline::NodeFeatureProducer;
use crate::projection::eval::pipeline::NodePropertyPipelineBaseTrainConfig;
use crate::projection::eval::pipeline::NodePropertyTrainingPipeline;
use crate::projection::eval::pipeline::Pipeline;
use crate::projection::eval::pipeline::PipelineTrainer;
use crate::projection::eval::pipeline::TrainingMethod as PipelineTrainingMethod;
use crate::projection::eval::pipeline::TrainingPipeline;
use crate::task::concurrency::{Concurrency, TerminationFlag};
use crate::task::memory::{Estimate, MemoryEstimation, MemoryEstimations, MemoryRange};
use crate::task::progress::{LeafTask, ProgressTracker, Task, TaskProgressTracker, Tasks};
use crate::types::graph::id_map::{MappedNodeId, OriginalNodeId};
use crate::types::graph::Graph;
use crate::types::graph_store::DefaultGraphStore;
use crate::types::prelude::GraphStore;
use parking_lot::RwLock;
use std::collections::{BTreeSet, HashMap};
use std::sync::Arc;

/// Core training algorithm for node regression.
///
/// This implements the training loop:
/// 1. Extract target values from the target property
/// 2. Split data into train/test/validation sets
/// 3. Train a regression model
/// 4. Evaluate on train/test sets
/// 5. Retrain on full training set
pub struct NodeRegressionTrain {
    pipeline: NodeRegressionTrainingPipeline,
    train_config: NodeRegressionPipelineTrainConfig,
    targets: HugeDoubleArray,
    node_graph: Arc<dyn Graph>,
    target_node_ids: Arc<Vec<MappedNodeId>>,
    node_feature_producer: NodeFeatureProducer<NodeRegressionPipelineTrainConfig>,
    progress_tracker: Box<dyn ProgressTracker>,
    termination_flag: TerminationFlag,
}

#[derive(Debug, PartialEq)]
pub enum NodeRegressionTrainError {
    InvalidTargetNodeLabels(String),
    InvalidSplitConfiguration(String),
    MissingTargetProperty(String),
    MissingTargetValue { node_id: i64, property: String },
    NonFiniteTargetValue { node_id: i64, property: String },
}

impl std::fmt::Display for NodeRegressionTrainError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidTargetNodeLabels(message) => {
                write!(formatter, "Invalid target node labels: {message}")
            }
            Self::InvalidSplitConfiguration(message) => {
                write!(formatter, "Invalid split configuration: {message}")
            }
            Self::MissingTargetProperty(property) => {
                write!(formatter, "Missing target node property `{property}`")
            }
            Self::MissingTargetValue { node_id, property } => {
                write!(
                    formatter,
                    "Node with id {node_id} has no `{property}` target property value"
                )
            }
            Self::NonFiniteTargetValue { node_id, property } => {
                write!(
                    formatter,
                    "Node with id {node_id} has non-finite `{property}` target property value"
                )
            }
        }
    }
}

impl std::error::Error for NodeRegressionTrainError {}

impl NodeRegressionTrain {
    /// Estimate memory requirements for training.
    pub fn estimate(
        pipeline: &NodeRegressionTrainingPipeline,
        configuration: &NodeRegressionPipelineTrainConfig,
        _model_catalog: &impl ModelCatalog,
    ) -> Box<dyn MemoryEstimation> {
        Self::estimate_pipeline(pipeline, configuration)
    }

    pub fn estimate_pipeline(
        pipeline: &NodeRegressionTrainingPipeline,
        configuration: &NodeRegressionPipelineTrainConfig,
    ) -> Box<dyn MemoryEstimation> {
        let node_property_steps =
            estimate_node_property_steps(pipeline.node_property_steps().len());
        let training = MemoryEstimations::builder("Training")
            .add(estimate_excluding_node_property_steps(
                configuration.metrics().len(),
                pipeline,
            ))
            .build();

        MemoryEstimations::max_estimation_named(
            "Node Regression Train Pipeline",
            vec![node_property_steps, training],
        )
    }

    /// Create progress task for training.
    pub fn progress_task(pipeline: &NodeRegressionTrainingPipeline, node_count: u64) -> Task {
        let split_config = pipeline.split_config();
        let train_set_size = split_config.train_set_size(node_count as usize);
        let test_set_size = split_config.test_set_size(node_count as usize);
        let validation_folds = split_config.validation_folds();
        let trials = pipeline.number_of_model_selection_trials();

        let mut tasks = Vec::new();
        let node_property_step_volume = pipeline
            .node_property_steps()
            .len()
            .saturating_mul(node_count as usize);
        tasks.push(Arc::new(
            Tasks::leaf_with_volume("Node property steps".to_string(), node_property_step_volume)
                .base()
                .clone(),
        ));

        let cv_volume = train_set_size
            .saturating_mul(validation_folds)
            .saturating_mul(trials);
        tasks.push(Arc::new(
            Tasks::leaf_with_volume(
                format!("Cross-validation ({validation_folds} folds, {trials} trials)"),
                cv_volume,
            )
            .base()
            .clone(),
        ));

        tasks.push(Arc::new(
            Tasks::leaf_with_volume("Train best model".to_string(), 5 * train_set_size)
                .base()
                .clone(),
        ));
        tasks.push(Arc::new(
            Tasks::leaf_with_volume("Evaluate on test data".to_string(), test_set_size)
                .base()
                .clone(),
        ));
        tasks.push(Arc::new(
            Tasks::leaf_with_volume("Retrain best model".to_string(), 5 * node_count as usize)
                .base()
                .clone(),
        ));

        Tasks::task("Node Regression Train Pipeline".to_string(), tasks)
    }

    /// Create a new NodeRegressionTrain instance.
    pub fn create(
        graph_store: Arc<DefaultGraphStore>,
        pipeline: NodeRegressionTrainingPipeline,
        config: NodeRegressionPipelineTrainConfig,
        node_feature_producer: NodeFeatureProducer<NodeRegressionPipelineTrainConfig>,
        progress_tracker: Box<dyn ProgressTracker>,
    ) -> Self {
        Self::try_create(
            graph_store,
            pipeline,
            config,
            node_feature_producer,
            progress_tracker,
        )
        .expect("Invalid node regression training configuration")
    }

    pub fn try_create(
        graph_store: Arc<DefaultGraphStore>,
        pipeline: NodeRegressionTrainingPipeline,
        config: NodeRegressionPipelineTrainConfig,
        node_feature_producer: NodeFeatureProducer<NodeRegressionPipelineTrainConfig>,
        progress_tracker: Box<dyn ProgressTracker>,
    ) -> Result<Self, NodeRegressionTrainError> {
        let node_graph = graph_store.get_graph();
        let target_node_labels = config
            .validate_target_node_label_identifiers(&graph_store)
            .map_err(|error| {
                NodeRegressionTrainError::InvalidTargetNodeLabels(error.to_string())
            })?;
        let target_node_ids = node_graph
            .iter_with_labels(&target_node_labels)
            .collect::<Vec<_>>();

        pipeline
            .split_config()
            .validate_min_num_nodes_in_split_sets(target_node_ids.len())
            .map_err(NodeRegressionTrainError::InvalidSplitConfiguration)?;

        let target_node_property = node_graph
            .node_properties(config.target_property())
            .ok_or_else(|| {
                NodeRegressionTrainError::MissingTargetProperty(
                    config.target_property().to_string(),
                )
            })?;

        let mut targets = HugeDoubleArray::new(target_node_ids.len());
        for (target_idx, root_node_id) in target_node_ids.iter().enumerate() {
            let original_node_id = node_graph
                .to_original_node_id(*root_node_id)
                .expect("target node id must belong to the training graph")
                .get();
            let value = target_node_property
                .double_value(root_node_id.get())
                .map_err(|_| NodeRegressionTrainError::MissingTargetValue {
                    node_id: original_node_id,
                    property: config.target_property().to_string(),
                })?;
            if !value.is_finite() {
                return Err(NodeRegressionTrainError::NonFiniteTargetValue {
                    node_id: original_node_id,
                    property: config.target_property().to_string(),
                });
            }
            targets.set(target_idx, value);
        }

        let termination_flag = TerminationFlag::running_true();

        Ok(Self {
            pipeline,
            train_config: config,
            targets,
            node_graph,
            target_node_ids: Arc::new(target_node_ids),
            node_feature_producer,
            progress_tracker,
            termination_flag,
        })
    }

    /// Set termination flag for early stopping.
    pub fn set_termination_flag(&mut self, termination_flag: TerminationFlag) {
        self.termination_flag = termination_flag;
    }

    /// Run the training algorithm.
    pub fn run(
        &mut self,
    ) -> Result<NodeRegressionTrainResult, Box<dyn std::error::Error + Send + Sync>> {
        self.progress_tracker.begin_subtask();

        let split_config = self.pipeline.split_config();
        let node_count = self.target_node_ids.len();
        let root_to_target_index = Arc::new(
            self.target_node_ids
                .iter()
                .enumerate()
                .map(|(target_id, root_node_id)| (*root_node_id, target_id))
                .collect::<HashMap<_, _>>(),
        );

        let concurrency = Concurrency::of(self.train_config.concurrency());
        let node_splitter = NodeSplitter::new(
            concurrency,
            node_count,
            Arc::new({
                let graph = Arc::clone(&self.node_graph);
                let target_node_ids = Arc::clone(&self.target_node_ids);
                move |id| {
                    let root_node_id = target_node_ids[id];
                    graph
                        .to_original_node_id(root_node_id)
                        .expect("target node id must belong to the training graph")
                        .get()
                }
            }),
            Arc::new({
                let graph = Arc::clone(&self.node_graph);
                let root_to_target_index = Arc::clone(&root_to_target_index);
                move |id| {
                    let root_node_id = graph
                        .to_mapped_node_id(OriginalNodeId::new(id))
                        .expect("original node id must belong to the training graph");
                    *root_to_target_index
                        .get(&root_node_id)
                        .expect("Mapped node id not found in target label set")
                }
            }),
        );

        let node_splits = node_splitter.split(
            split_config.test_fraction(),
            split_config.validation_folds(),
            self.train_config.random_seed(),
            self.progress_tracker.as_mut(),
        );

        let metrics = resolve_metrics(&self.train_config);
        let metric_boxes: Vec<Box<dyn Metric>> = metrics
            .iter()
            .copied()
            .map(|m| Box::new(m) as Box<dyn Metric>)
            .collect();

        let mut training_statistics = TrainingStatistics::new(&metric_boxes);

        let features = self
            .node_feature_producer
            .procedure_features(&self.pipeline)
            .map_err(|e| format!("Feature production failed: {e}"))?;
        let features: Arc<dyn Features> = Arc::from(features);

        if !self.termination_flag.running() {
            return Err("Node regression training was terminated".into());
        }

        let candidates = self.collect_candidate_configs()?;
        let candidate_configs_for_cv: Vec<Box<dyn TrainerConfigTrait>> = candidates
            .iter()
            .map(|(method, config_map)| create_trainer_config_from_map(config_map.clone(), *method))
            .collect();

        let metrics_for_cv: Vec<Box<dyn Metric>> = metrics
            .iter()
            .copied()
            .map(|metric| Box::new(metric) as Box<dyn Metric>)
            .collect();
        let metrics_for_evaluation = Arc::new(metrics.clone());
        let targets = Arc::new(self.targets.clone());
        let termination_flag = self.termination_flag.clone();
        let random_seed = self.train_config.random_seed();
        let cv = CrossValidation::new(
            Arc::new(RwLock::new(false)),
            metrics_for_cv,
            split_config.validation_folds(),
            random_seed,
            Box::new({
                let features = Arc::clone(&features);
                let targets = Arc::clone(&targets);
                move |train_set, trainer_config, _metrics_handler, _name| {
                    let progress = TaskProgressTracker::new(LeafTask::new(
                        "Train regression candidate".to_string(),
                        0,
                    ));
                    let trainer = RegressionTrainerFactory::create(
                        trainer_config,
                        &termination_flag,
                        progress,
                        &concurrency,
                        random_seed,
                    );
                    trainer.train(features.as_ref(), targets.as_ref(), &to_u64_arc(train_set))
                }
            }),
            Box::new({
                let features = Arc::clone(&features);
                let targets = Arc::clone(&targets);
                move |evaluation_set, model, consumer| {
                    let scores = evaluate_metrics(
                        &evaluation_set,
                        features.as_ref(),
                        model.as_ref(),
                        targets.as_ref(),
                        metrics_for_evaluation.as_ref(),
                    );
                    for metric in metrics_for_evaluation.iter() {
                        consumer.consume(
                            metric as &dyn Metric,
                            scores.get(metric.name()).copied().unwrap_or(0.0),
                        );
                    }
                }
            }),
        );

        cv.select_model(
            node_splits.outer_split().train_set(),
            |_| 0,
            BTreeSet::from([0]),
            &mut training_statistics,
            candidate_configs_for_cv.into_iter(),
        );

        if !self.termination_flag.running() {
            return Err("Node regression training was terminated".into());
        }

        let (best_method, best_map) = candidates
            .get(training_statistics.best_trial_idx())
            .ok_or("Node regression model selection produced no winning candidate")?;
        let best_config = create_trainer_config_from_map(best_map.clone(), *best_method);

        let regressor = self.train_simple_model(
            node_splits.outer_split(),
            features.as_ref(),
            best_config.as_ref(),
        )?;
        self.evaluate_model(
            node_splits.outer_split(),
            features.as_ref(),
            &regressor,
            &metrics,
            &mut training_statistics,
        );

        let retrained = self.retrain_best_model(
            node_splits.all_training_examples(),
            features.as_ref(),
            best_config.as_ref(),
        )?;

        self.progress_tracker.end_subtask();

        Ok(NodeRegressionTrainResult::new(
            retrained,
            training_statistics,
        ))
    }
}

impl PipelineTrainer for NodeRegressionTrain {
    type Result = NodeRegressionTrainResult;

    fn set_termination_flag(&mut self, termination_flag: TerminationFlag) {
        NodeRegressionTrain::set_termination_flag(self, termination_flag);
    }

    fn run(&mut self) -> Result<Self::Result, Box<dyn std::error::Error + Send + Sync>> {
        self.run()
    }

    fn is_terminated(&self) -> bool {
        !self.termination_flag.running()
    }
}

impl NodeRegressionTrain {
    fn train_simple_model(
        &self,
        split: &TrainingExamplesSplit,
        features: &dyn Features,
        trainer_config: &dyn TrainerConfigTrait,
    ) -> Result<Box<dyn Regressor>, Box<dyn std::error::Error + Send + Sync>> {
        let train_set = to_u64_arc(split.train_set());
        let progress =
            TaskProgressTracker::new(LeafTask::new("Train regression model".to_string(), 0));
        let trainer = RegressionTrainerFactory::create(
            trainer_config,
            &self.termination_flag,
            progress,
            &Concurrency::of(self.train_config.concurrency()),
            self.train_config.random_seed(),
        );

        Ok(trainer.train(features, &self.targets, &train_set))
    }

    fn retrain_best_model(
        &self,
        all_training_examples: &Arc<Vec<i64>>,
        features: &dyn Features,
        trainer_config: &dyn TrainerConfigTrait,
    ) -> Result<Box<dyn Regressor>, Box<dyn std::error::Error + Send + Sync>> {
        let train_set = to_u64_arc(all_training_examples.clone());
        let progress =
            TaskProgressTracker::new(LeafTask::new("Retrain regression model".to_string(), 0));
        let trainer = RegressionTrainerFactory::create(
            trainer_config,
            &self.termination_flag,
            progress,
            &Concurrency::of(self.train_config.concurrency()),
            self.train_config.random_seed(),
        );

        Ok(trainer.train(features, &self.targets, &train_set))
    }

    fn evaluate_model(
        &self,
        split: &TrainingExamplesSplit,
        features: &dyn Features,
        regressor: &Box<dyn Regressor>,
        metrics: &[RegressionMetric],
        training_statistics: &mut TrainingStatistics,
    ) {
        let outer_train_scores = evaluate_metrics(
            &split.train_set(),
            features,
            regressor.as_ref(),
            &self.targets,
            metrics,
        );

        for (metric, score) in outer_train_scores {
            training_statistics.add_outer_train_score(metric, score);
        }

        let test_scores = evaluate_metrics(
            &split.test_set(),
            features,
            regressor.as_ref(),
            &self.targets,
            metrics,
        );

        for (metric, score) in test_scores {
            training_statistics.add_test_score(metric, score);
        }
    }

    fn collect_candidate_configs(
        &self,
    ) -> Result<
        Vec<(
            crate::ml::models::TrainingMethod,
            HashMap<String, serde_json::Value>,
        )>,
        Box<dyn std::error::Error + Send + Sync>,
    > {
        let mut parameter_space = HashMap::new();
        for method in [
            PipelineTrainingMethod::LinearRegression,
            PipelineTrainingMethod::RandomForestRegression,
        ] {
            let Some(configs) = self.pipeline.training_parameter_space().get(&method) else {
                continue;
            };
            let ml_method = map_training_method(method);
            let method_configs = parameter_space.entry(ml_method).or_insert_with(Vec::new);
            for config in configs {
                let tunable = MlTunableTrainerConfig::of(&config.to_map(), ml_method)
                    .map_err(|error| format!("Invalid {method} parameter space: {error}"))?;
                method_configs.push(tunable);
            }
        }

        if parameter_space.values().all(Vec::is_empty) {
            return Err("Need at least one regression model candidate for training.".into());
        }

        let candidates = RandomSearch::new_with_seed(
            parameter_space,
            self.pipeline.auto_tuning_config().max_trials(),
            self.train_config.random_seed(),
        )
        .map(|config| (config.method(), config.to_map()))
        .collect();

        Ok(candidates)
    }
}

fn map_training_method(method: PipelineTrainingMethod) -> crate::ml::models::TrainingMethod {
    match method {
        PipelineTrainingMethod::LinearRegression => {
            crate::ml::models::TrainingMethod::LinearRegression
        }
        PipelineTrainingMethod::RandomForestRegression => {
            crate::ml::models::TrainingMethod::RandomForestRegression
        }
        other => panic!("Unsupported training method for regression: {other:?}"),
    }
}

fn resolve_metrics(config: &NodeRegressionPipelineTrainConfig) -> Vec<RegressionMetric> {
    if config.metrics().is_empty() {
        vec![RegressionMetric::MeanSquaredError]
    } else {
        config.metrics().to_vec()
    }
}

fn to_u64_arc(values: Arc<Vec<i64>>) -> Arc<Vec<u64>> {
    Arc::new(
        values
            .iter()
            .map(|value| {
                u64::try_from(*value).expect("training example id must fit the model row domain")
            })
            .collect(),
    )
}

fn estimate_node_property_steps(step_count: usize) -> Box<dyn MemoryEstimation> {
    MemoryEstimations::of_resident("Node property steps", move |dim, _| {
        MemoryRange::of(
            step_count
                .saturating_mul(dim.node_count())
                .saturating_mul(Estimate::BYTES_OBJECT_REF),
        )
    })
}

fn estimate_excluding_node_property_steps(
    metrics_size: usize,
    pipeline: &NodeRegressionTrainingPipeline,
) -> Box<dyn MemoryEstimation> {
    const FUDGED_FEATURE_COUNT: usize = 500;

    let split_config = pipeline.split_config().clone();
    let validation_folds = split_config.validation_folds();
    let test_fraction = split_config.test_fraction();
    let number_of_model_candidates = pipeline.number_of_model_selection_trials();
    let needs_cached_features = pipeline.require_eager_features();

    let model_selection = model_train_and_evaluate_memory_usage(
        "model selection",
        {
            let split_config = split_config.clone();
            move |node_count| split_config.fold_train_set_size(node_count as usize) as u64
        },
        {
            let split_config = split_config.clone();
            move |node_count| split_config.fold_test_set_size(node_count as usize) as u64
        },
    );
    let best_model_evaluation = MemoryEstimations::delegate_estimation(
        model_train_and_evaluate_memory_usage(
            "model evaluation",
            {
                let split_config = split_config.clone();
                move |node_count| split_config.train_set_size(node_count as usize) as u64
            },
            move |node_count| split_config.test_set_size(node_count as usize) as u64,
        ),
        "best model evaluation",
    );
    let model_training_estimation =
        MemoryEstimations::max_estimation(vec![model_selection, best_model_evaluation]);

    let mut builder = MemoryEstimations::builder("Training without node property steps")
        .range_per_graph_dimension("global targets", |dim, _| {
            MemoryRange::of(Estimate::size_of_double_array(dim.node_count()))
        })
        .range_per_graph_dimension("outer split", move |dim, _| {
            let train_size = ((1.0 - test_fraction) * dim.node_count() as f64) as usize;
            MemoryRange::of(Estimate::size_of_long_array(train_size))
        })
        .range_per_graph_dimension("inner split", move |dim, _| {
            let train_size = ((1.0 - test_fraction) * dim.node_count() as f64) as usize;
            MemoryRange::of(
                Estimate::size_of_int_array(train_size).saturating_mul(validation_folds),
            )
        })
        .fixed_range(
            "stats map train",
            estimate_stats_map(metrics_size, number_of_model_candidates),
        )
        .fixed_range(
            "stats map validation",
            estimate_stats_map(metrics_size, number_of_model_candidates),
        )
        .add_as(
            "max of model selection and best model evaluation",
            model_training_estimation,
        );

    if needs_cached_features {
        builder = builder.range_per_graph_dimension("cached feature vectors", |dim, _| {
            let node_count = dim.node_count();
            let object_refs = Estimate::size_of_object_array(node_count);
            let min = object_refs
                .saturating_add(node_count.saturating_mul(Estimate::size_of_double_array(10)));
            let max = object_refs.saturating_add(
                node_count.saturating_mul(Estimate::size_of_double_array(FUDGED_FEATURE_COUNT)),
            );
            MemoryRange::of_range(min, max)
        });
    }

    builder.build()
}

fn model_train_and_evaluate_memory_usage(
    description: &'static str,
    train_set_size: impl Fn(u64) -> u64 + Send + Sync + 'static,
    test_set_size: impl Fn(u64) -> u64 + Send + Sync + 'static,
) -> Box<dyn MemoryEstimation> {
    MemoryEstimations::builder(description)
        .range_per_graph_dimension("training data", move |dim, _| {
            let rows = train_set_size(dim.node_count() as u64) as usize;
            MemoryRange::of(Estimate::size_of_double_array(rows.saturating_mul(500)))
        })
        .range_per_graph_dimension("evaluation data", move |dim, _| {
            let rows = test_set_size(dim.node_count() as u64) as usize;
            MemoryRange::of(Estimate::size_of_double_array(rows.saturating_mul(500)))
        })
        .build()
}

fn estimate_stats_map(metrics_size: usize, number_of_model_candidates: usize) -> MemoryRange {
    let entries = metrics_size.saturating_mul(number_of_model_candidates);
    let min_entries = entries.max(1);
    MemoryRange::of_range(
        Estimate::size_of_double_array(min_entries),
        Estimate::size_of_double_array(min_entries.saturating_mul(3)),
    )
}

fn evaluate_metrics(
    eval_ids: &Arc<Vec<i64>>,
    features: &dyn Features,
    regressor: &dyn Regressor,
    targets: &HugeDoubleArray,
    metrics: &[RegressionMetric],
) -> HashMap<String, f64> {
    let mut predictions: Vec<f64> = Vec::with_capacity(eval_ids.len());
    let mut actuals: Vec<f64> = Vec::with_capacity(eval_ids.len());

    for node_id in eval_ids.iter() {
        let idx =
            usize::try_from(*node_id).expect("evaluation example id must fit the model row domain");
        let prediction = regressor.predict(features.get(idx));
        predictions.push(prediction);
        actuals.push(targets.get(idx));
    }

    let actuals = HugeDoubleArray::from_vec(actuals);
    let predictions = HugeDoubleArray::from_vec(predictions);

    metrics
        .iter()
        .map(|metric| {
            (
                metric.name().to_string(),
                metric.compute(&actuals, &predictions),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::backends::vec::VecDouble;
    use crate::core::model::EmptyModelCatalog;
    use crate::projection::eval::pipeline::NodeFeatureStep;
    use crate::projection::eval::pipeline::TunableTrainerConfig;
    use crate::task::progress::NoopProgressTracker;
    use crate::types::properties::node::DefaultDoubleNodePropertyValues;
    use crate::types::random::RandomGraphConfig;

    #[derive(Clone)]
    struct FixedRegressionTrainerConfig {
        method: PipelineTrainingMethod,
        parameters: HashMap<String, serde_json::Value>,
    }

    impl TunableTrainerConfig for FixedRegressionTrainerConfig {
        fn training_method(&self) -> PipelineTrainingMethod {
            self.method
        }

        fn is_concrete(&self) -> bool {
            !self.parameters.values().any(serde_json::Value::is_object)
        }

        fn to_map(&self) -> HashMap<String, serde_json::Value> {
            self.parameters.clone()
        }
    }

    #[test]
    #[ignore]
    fn test_create_train_algorithm() {
        let config = RandomGraphConfig {
            node_count: 10,
            seed: Some(42),
            ..RandomGraphConfig::default()
        };
        let graph_store =
            Arc::new(DefaultGraphStore::random(&config).expect("Failed to generate random graph"));
        let pipeline = NodeRegressionTrainingPipeline::new();
        let train_config = NodeRegressionPipelineTrainConfig::default();
        let node_feature_producer =
            NodeFeatureProducer::create(graph_store.clone(), train_config.clone());
        let progress_tracker = Box::new(NoopProgressTracker);

        let _trainer = NodeRegressionTrain::create(
            graph_store,
            pipeline,
            train_config,
            node_feature_producer,
            progress_tracker,
        );
    }

    #[test]
    fn test_progress_task() {
        let pipeline = NodeRegressionTrainingPipeline::new();
        let node_count = 1000;

        let _task = NodeRegressionTrain::progress_task(&pipeline, node_count);
    }

    #[test]
    fn test_estimate() {
        let pipeline = NodeRegressionTrainingPipeline::new();
        let config = NodeRegressionPipelineTrainConfig::default();
        let model_catalog = EmptyModelCatalog;

        let _est = NodeRegressionTrain::estimate(&pipeline, &config, &model_catalog);
    }

    #[test]
    fn test_create_uses_target_label_node_universe() {
        let mut graph_store = DefaultGraphStore::random(&RandomGraphConfig {
            seed: Some(42),
            node_count: 200,
            node_labels: vec!["Person".to_string(), "Company".to_string()],
            ..RandomGraphConfig::default()
        })
        .expect("random graph");

        let target_label = graph_store
            .node_labels()
            .into_iter()
            .find(|label| {
                let count = graph_store.node_count_for_label(label);
                count > 0 && count < graph_store.node_count()
            })
            .expect("expected a non-empty proper target label subset");
        let target_count = graph_store.node_count_for_label(&target_label);
        let property_values = Arc::new(DefaultDoubleNodePropertyValues::from_collection(
            VecDouble::from(
                (0..graph_store.node_count())
                    .map(|node_id| node_id as f64)
                    .collect::<Vec<_>>(),
            ),
            graph_store.node_count(),
        ));
        graph_store
            .add_node_property(graph_store.node_labels(), "target", property_values)
            .expect("target property should be added");

        let graph_store = Arc::new(graph_store);
        let train_config = NodeRegressionPipelineTrainConfig::new(
            "test-pipeline".to_string(),
            vec![target_label.name().to_string()],
            "target".to_string(),
            Some(42),
            vec![RegressionMetric::MeanSquaredError],
        );
        let node_feature_producer =
            NodeFeatureProducer::create(graph_store.clone(), train_config.clone());

        let trainer = NodeRegressionTrain::create(
            graph_store,
            NodeRegressionTrainingPipeline::new(),
            train_config,
            node_feature_producer,
            Box::new(NoopProgressTracker),
        );

        assert_eq!(trainer.target_node_ids.len(), target_count);
        assert_eq!(trainer.targets.size(), target_count);
    }

    #[test]
    fn test_run_selects_configured_linear_regression_candidate() {
        let mut graph_store = DefaultGraphStore::random(&RandomGraphConfig {
            seed: Some(42),
            node_count: 60,
            ..RandomGraphConfig::default()
        })
        .expect("random graph");
        let node_count = graph_store.node_count();
        let labels = graph_store.node_labels();
        let features = Arc::new(DefaultDoubleNodePropertyValues::from_collection(
            VecDouble::from(
                (0..node_count)
                    .map(|node_id| node_id as f64)
                    .collect::<Vec<_>>(),
            ),
            node_count,
        ));
        let targets = Arc::new(DefaultDoubleNodePropertyValues::from_collection(
            VecDouble::from(
                (0..node_count)
                    .map(|node_id| 2.0 * node_id as f64 + 1.0)
                    .collect::<Vec<_>>(),
            ),
            node_count,
        ));
        graph_store
            .add_node_property(labels.clone(), "feature", features)
            .expect("feature property");
        graph_store
            .add_node_property(labels, "target", targets)
            .expect("target property");

        let mut pipeline = NodeRegressionTrainingPipeline::new();
        pipeline.add_feature_step(NodeFeatureStep::of("feature"));
        pipeline.add_trainer_config(Box::new(FixedRegressionTrainerConfig {
            method: PipelineTrainingMethod::LinearRegression,
            parameters: HashMap::from([
                ("penalty".to_string(), serde_json::json!(0.0)),
                ("maxEpochs".to_string(), serde_json::json!(50)),
                ("learningRate".to_string(), serde_json::json!(0.01)),
            ]),
        }));

        let graph_store = Arc::new(graph_store);
        let config = NodeRegressionPipelineTrainConfig::new(
            "test-pipeline".to_string(),
            vec!["*".to_string()],
            "target".to_string(),
            Some(42),
            vec![RegressionMetric::MeanSquaredError],
        )
        .with_concurrency(1);
        let producer = NodeFeatureProducer::create(Arc::clone(&graph_store), config.clone());
        let mut trainer = NodeRegressionTrain::create(
            graph_store,
            pipeline,
            config,
            producer,
            Box::new(NoopProgressTracker),
        );

        let result = trainer.run().expect("regression training should run");
        let statistics = result.training_statistics();

        assert_eq!(
            result.regressor().data().trainer_method(),
            crate::ml::models::TrainingMethod::LinearRegression
        );
        assert_eq!(statistics.best_parameters()["method"], "LinearRegression");
        assert!(statistics
            .best_candidate()
            .training_stats
            .contains_key("MEAN_SQUARED_ERROR"));
        assert!(statistics
            .best_candidate()
            .validation_stats
            .contains_key("MEAN_SQUARED_ERROR"));
        assert!(statistics
            .winning_model_test_metrics()
            .contains_key("MEAN_SQUARED_ERROR"));
        assert!(statistics
            .winning_model_outer_train_metrics()
            .contains_key("MEAN_SQUARED_ERROR"));
    }

    #[test]
    fn test_run_selects_configured_random_forest_regression_candidate() {
        let mut graph_store = DefaultGraphStore::random(&RandomGraphConfig {
            seed: Some(42),
            node_count: 60,
            ..RandomGraphConfig::default()
        })
        .expect("random graph");
        let node_count = graph_store.node_count();
        let labels = graph_store.node_labels();
        let features = Arc::new(DefaultDoubleNodePropertyValues::from_collection(
            VecDouble::from(
                (0..node_count)
                    .map(|node_id| node_id as f64)
                    .collect::<Vec<_>>(),
            ),
            node_count,
        ));
        let targets = Arc::new(DefaultDoubleNodePropertyValues::from_collection(
            VecDouble::from(
                (0..node_count)
                    .map(|node_id| 2.0 * node_id as f64 + 1.0)
                    .collect::<Vec<_>>(),
            ),
            node_count,
        ));
        graph_store
            .add_node_property(labels.clone(), "feature", features)
            .expect("feature property");
        graph_store
            .add_node_property(labels, "target", targets)
            .expect("target property");

        let mut pipeline = NodeRegressionTrainingPipeline::new();
        pipeline.add_feature_step(NodeFeatureStep::of("feature"));
        pipeline.add_trainer_config(Box::new(FixedRegressionTrainerConfig {
            method: PipelineTrainingMethod::RandomForestRegression,
            parameters: HashMap::from([
                ("numberOfDecisionTrees".to_string(), serde_json::json!(5)),
                ("numberOfSamplesRatio".to_string(), serde_json::json!(1.0)),
                ("maxFeaturesRatio".to_string(), serde_json::json!(1.0)),
                ("maxDepth".to_string(), serde_json::json!(3)),
            ]),
        }));

        let graph_store = Arc::new(graph_store);
        let config = NodeRegressionPipelineTrainConfig::new(
            "test-pipeline".to_string(),
            vec!["*".to_string()],
            "target".to_string(),
            Some(42),
            vec![RegressionMetric::MeanSquaredError],
        )
        .with_concurrency(1);
        let producer = NodeFeatureProducer::create(Arc::clone(&graph_store), config.clone());
        let mut trainer = NodeRegressionTrain::create(
            graph_store,
            pipeline,
            config,
            producer,
            Box::new(NoopProgressTracker),
        );

        let result = trainer.run().expect("regression training should run");
        let statistics = result.training_statistics();

        assert_eq!(
            result.regressor().data().trainer_method(),
            crate::ml::models::TrainingMethod::RandomForestRegression
        );
        assert_eq!(
            statistics.best_parameters()["method"],
            "RandomForestRegression"
        );
        assert_eq!(statistics.best_parameters()["numberOfDecisionTrees"], 5);
        assert!(statistics
            .best_candidate()
            .validation_stats
            .contains_key("MEAN_SQUARED_ERROR"));
        assert!(statistics
            .winning_model_test_metrics()
            .contains_key("MEAN_SQUARED_ERROR"));
    }

    #[test]
    fn test_run_materializes_tunable_linear_regression_candidates() {
        let mut graph_store = DefaultGraphStore::random(&RandomGraphConfig {
            seed: Some(42),
            node_count: 60,
            ..RandomGraphConfig::default()
        })
        .expect("random graph");
        let node_count = graph_store.node_count();
        let labels = graph_store.node_labels();
        let features = Arc::new(DefaultDoubleNodePropertyValues::from_collection(
            VecDouble::from(
                (0..node_count)
                    .map(|node_id| node_id as f64 / (node_count - 1) as f64)
                    .collect::<Vec<_>>(),
            ),
            node_count,
        ));
        let targets = Arc::new(DefaultDoubleNodePropertyValues::from_collection(
            VecDouble::from(
                (0..node_count)
                    .map(|node_id| 2.0 * node_id as f64 / (node_count - 1) as f64 + 1.0)
                    .collect::<Vec<_>>(),
            ),
            node_count,
        ));
        graph_store
            .add_node_property(labels.clone(), "feature", features)
            .expect("feature property");
        graph_store
            .add_node_property(labels, "target", targets)
            .expect("target property");

        let mut pipeline = NodeRegressionTrainingPipeline::new();
        pipeline.add_feature_step(NodeFeatureStep::of("feature"));
        pipeline.add_trainer_config(Box::new(FixedRegressionTrainerConfig {
            method: PipelineTrainingMethod::LinearRegression,
            parameters: HashMap::from([
                (
                    "penalty".to_string(),
                    serde_json::json!({"range": [0.001, 0.1]}),
                ),
                (
                    "learningRate".to_string(),
                    serde_json::json!({"range": [0.01, 0.05]}),
                ),
                (
                    "maxEpochs".to_string(),
                    serde_json::json!({"range": [50, 100]}),
                ),
            ]),
        }));
        pipeline.set_auto_tuning_config(
            crate::projection::eval::pipeline::AutoTuningConfig::new(3)
                .expect("valid AutoML trial count"),
        );

        let graph_store = Arc::new(graph_store);
        let config = NodeRegressionPipelineTrainConfig::new(
            "test-pipeline".to_string(),
            vec!["*".to_string()],
            "target".to_string(),
            Some(42),
            vec![RegressionMetric::MeanSquaredError],
        )
        .with_concurrency(1);
        let producer = NodeFeatureProducer::create(Arc::clone(&graph_store), config.clone());
        let mut trainer = NodeRegressionTrain::create(
            graph_store,
            pipeline,
            config,
            producer,
            Box::new(NoopProgressTracker),
        );

        let result = trainer
            .run()
            .expect("AutoML regression training should run");
        let statistics = result.training_statistics();
        let statistics_map = statistics.to_map();
        let candidates = statistics_map["modelCandidates"]
            .as_array()
            .expect("model candidate statistics");
        let best_parameters = statistics.best_parameters();

        assert_eq!(candidates.len(), 3);
        assert_eq!(best_parameters["method"], "LinearRegression");
        assert!((0.001..0.1).contains(
            &best_parameters["penalty"]
                .as_f64()
                .expect("materialized penalty")
        ));
        assert!((0.01..0.05).contains(
            &best_parameters["learning_rate"]
                .as_f64()
                .expect("materialized learning rate")
        ));
        assert!((50..100).contains(
            &best_parameters["max_epochs"]
                .as_u64()
                .expect("materialized max epochs")
        ));
    }

    #[test]
    fn test_try_create_reports_missing_target_property() {
        let config = RandomGraphConfig {
            node_count: 10,
            seed: Some(42),
            ..RandomGraphConfig::default()
        };
        let graph_store =
            Arc::new(DefaultGraphStore::random(&config).expect("Failed to generate random graph"));
        let pipeline = NodeRegressionTrainingPipeline::new();
        let config = NodeRegressionPipelineTrainConfig::default();
        let node_feature_producer =
            NodeFeatureProducer::create(graph_store.clone(), config.clone());
        let progress_tracker = Box::new(NoopProgressTracker);

        let result = NodeRegressionTrain::try_create(
            graph_store,
            pipeline,
            config,
            node_feature_producer,
            progress_tracker,
        );

        assert!(matches!(
            result,
            Err(NodeRegressionTrainError::MissingTargetProperty(property))
                if property == "target"
        ));
    }
}
