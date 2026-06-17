use super::node_regression_pipeline_train_config::NodeRegressionPipelineTrainConfig;
use super::node_regression_train_result::NodeRegressionTrainResult;
use super::node_regression_training_pipeline::NodeRegressionTrainingPipeline;
use crate::collections::HugeDoubleArray;
use crate::core::model::ModelCatalog;
use crate::ml::metrics::regression::RegressionMetric;
use crate::ml::metrics::{Metric, ModelCandidateStats};
use crate::ml::models::linear_regression::LinearRegressionTrainConfig;
use crate::ml::models::{Features, RegressionTrainerFactory, Regressor};
use crate::ml::node_regression::NodeSplitter;
use crate::ml::splitting::TrainingExamplesSplit;
use crate::ml::training::statistics::TrainingStatistics;
use crate::procedures::AlgorithmsProcedureFacade;
use crate::projection::eval::pipeline::NodeFeatureProducer;
use crate::projection::eval::pipeline::NodePropertyPipelineBaseTrainConfig;
use crate::projection::eval::pipeline::NodePropertyTrainingPipeline;
use crate::projection::eval::pipeline::Pipeline;
use crate::projection::eval::pipeline::PipelineTrainer;
use crate::projection::eval::pipeline::TrainingPipeline;
use crate::task::concurrency::{Concurrency, TerminationFlag};
use crate::task::memory::{Estimate, MemoryEstimation, MemoryEstimations, MemoryRange};
use crate::task::progress::{LeafTask, ProgressTracker, Task, TaskProgressTracker, Tasks};
use crate::types::graph::Graph;
use crate::types::graph_store::DefaultGraphStore;
use crate::types::prelude::GraphStore;
use std::collections::HashMap;
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
    target_node_ids: Arc<Vec<u64>>,
    node_feature_producer: NodeFeatureProducer<NodeRegressionPipelineTrainConfig>,
    progress_tracker: Box<dyn ProgressTracker>,
    termination_flag: TerminationFlag,
}

impl NodeRegressionTrain {
    /// Estimate memory requirements for training.
    pub fn estimate(
        pipeline: &NodeRegressionTrainingPipeline,
        configuration: &NodeRegressionPipelineTrainConfig,
        _model_catalog: &impl ModelCatalog,
        _algorithms_procedure_facade: &AlgorithmsProcedureFacade,
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
        let node_graph = graph_store.get_graph();
        let target_node_labels = config
            .validate_target_node_label_identifiers(&graph_store)
            .expect("Invalid target node labels for regression");
        let target_node_ids = node_graph
            .iter_with_labels(&target_node_labels)
            .map(|node_id| node_id as u64)
            .collect::<Vec<_>>();

        pipeline
            .split_config()
            .validate_min_num_nodes_in_split_sets(target_node_ids.len())
            .expect("Invalid split configuration for node count");

        let target_node_property = node_graph
            .node_properties(config.target_property())
            .expect("Missing target node property for regression");

        let mut targets = HugeDoubleArray::new(target_node_ids.len());
        for (target_idx, root_node_id) in target_node_ids.iter().enumerate() {
            let value = target_node_property
                .double_value(*root_node_id)
                .unwrap_or(0.0);
            if value.is_nan() {
                panic!(
                    "Node with id {} has `{}` target property value `NaN`",
                    node_graph
                        .to_original_node_id(*root_node_id as i64)
                        .unwrap_or(*root_node_id as i64),
                    config.target_property()
                );
            }
            if value.is_infinite() {
                panic!(
                    "Node with id {} has infinite `{}` target property value",
                    node_graph
                        .to_original_node_id(*root_node_id as i64)
                        .unwrap_or(*root_node_id as i64),
                    config.target_property()
                );
            }
            targets.set(target_idx, value);
        }

        let termination_flag = TerminationFlag::running_true();

        Self {
            pipeline,
            train_config: config,
            targets,
            node_graph,
            target_node_ids: Arc::new(target_node_ids),
            node_feature_producer,
            progress_tracker,
            termination_flag,
        }
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

        let node_splitter = NodeSplitter::new(
            Concurrency::available_cores(),
            node_count,
            Arc::new({
                let graph = Arc::clone(&self.node_graph);
                let target_node_ids = Arc::clone(&self.target_node_ids);
                move |id| {
                    let root_node_id = target_node_ids[id] as i64;
                    graph
                        .to_original_node_id(root_node_id)
                        .unwrap_or(root_node_id)
                }
            }),
            Arc::new({
                let graph = Arc::clone(&self.node_graph);
                let root_to_target_index = Arc::clone(&root_to_target_index);
                move |id| {
                    let root_node_id = graph
                        .to_mapped_node_id(id)
                        .expect("Mapped node id not found")
                        as u64;
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
        training_statistics.add_candidate_stats(ModelCandidateStats::new(
            serde_json::json!({}),
            HashMap::new(),
            HashMap::new(),
        ));

        let features = self
            .node_feature_producer
            .procedure_features(&self.pipeline)
            .map_err(|e| format!("Feature production failed: {e}"))?;

        let regressor = self.train_simple_model(node_splits.outer_split(), &features)?;
        self.evaluate_model(
            node_splits.outer_split(),
            &features,
            &regressor,
            &metrics,
            &mut training_statistics,
        );

        let retrained = self.retrain_best_model(node_splits.all_training_examples(), &features)?;

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
        features: &Box<dyn Features>,
    ) -> Result<Box<dyn Regressor>, Box<dyn std::error::Error + Send + Sync>> {
        let train_set = to_u64_arc(split.train_set());
        let trainer_config = LinearRegressionTrainConfig::default();
        let progress =
            TaskProgressTracker::new(LeafTask::new("Train regression model".to_string(), 0));
        let trainer = RegressionTrainerFactory::create(
            &trainer_config,
            &self.termination_flag,
            progress,
            &Concurrency::available_cores(),
            self.train_config.random_seed(),
        );

        Ok(trainer.train(features.as_ref(), &self.targets, &train_set))
    }

    fn retrain_best_model(
        &self,
        all_training_examples: &Arc<Vec<i64>>,
        features: &Box<dyn Features>,
    ) -> Result<Box<dyn Regressor>, Box<dyn std::error::Error + Send + Sync>> {
        let train_set = to_u64_arc(all_training_examples.clone());
        let trainer_config = LinearRegressionTrainConfig::default();
        let progress =
            TaskProgressTracker::new(LeafTask::new("Retrain regression model".to_string(), 0));
        let trainer = RegressionTrainerFactory::create(
            &trainer_config,
            &self.termination_flag,
            progress,
            &Concurrency::available_cores(),
            self.train_config.random_seed(),
        );

        Ok(trainer.train(features.as_ref(), &self.targets, &train_set))
    }

    fn evaluate_model(
        &self,
        split: &TrainingExamplesSplit,
        features: &Box<dyn Features>,
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
}

fn resolve_metrics(config: &NodeRegressionPipelineTrainConfig) -> Vec<RegressionMetric> {
    if config.metrics().is_empty() {
        vec![RegressionMetric::MeanSquaredError]
    } else {
        config.metrics().to_vec()
    }
}

fn to_u64_arc(values: Arc<Vec<i64>>) -> Arc<Vec<u64>> {
    Arc::new(values.iter().map(|v| *v as u64).collect())
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
    features: &Box<dyn Features>,
    regressor: &dyn Regressor,
    targets: &HugeDoubleArray,
    metrics: &[RegressionMetric],
) -> HashMap<String, f64> {
    let mut predictions: Vec<f64> = Vec::with_capacity(eval_ids.len());
    let mut actuals: Vec<f64> = Vec::with_capacity(eval_ids.len());

    for node_id in eval_ids.iter() {
        let idx = *node_id as usize;
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
    use crate::task::progress::NoopProgressTracker;
    use crate::types::properties::node::DefaultDoubleNodePropertyValues;
    use crate::types::random::RandomGraphConfig;

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
        let graph_store = Arc::new(
            DefaultGraphStore::random(&RandomGraphConfig::default())
                .expect("random graph generation"),
        );
        let algorithms_facade = AlgorithmsProcedureFacade::from_store(graph_store);

        let _est =
            NodeRegressionTrain::estimate(&pipeline, &config, &model_catalog, &algorithms_facade);
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
    #[should_panic(expected = "Missing target node property for regression")]
    fn test_run() {
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

        let mut trainer = NodeRegressionTrain::create(
            graph_store,
            pipeline,
            config,
            node_feature_producer,
            progress_tracker,
        );

        let _result = trainer.run();
    }
}
