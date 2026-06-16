use super::labels_and_class_counts_extractor::LabelsAndClassCountsExtractor;
use super::node_classification_pipeline_train_config::NodeClassificationPipelineTrainConfig;
use super::node_classification_train_result::NodeClassificationTrainResult;
use super::node_classification_training_pipeline::NodeClassificationTrainingPipeline;
use crate::collections::long_multiset::LongMultiSet;
use crate::collections::{HugeIntArray, HugeLongArray};
use crate::core::model::ModelCatalog;
use crate::ml::core::subgraph::LocalIdMap;
use crate::ml::metrics::classification::ClassificationMetric;
use crate::ml::metrics::ClassificationMetricSpecification;
use crate::ml::metrics::{Metric, ModelSpecificMetricsHandler};
use crate::ml::models::automl::create_trainer_config_from_map;
use crate::ml::models::{
    base::TrainerConfigTrait, Classifier, ClassifierTrainerFactory, Features,
    TrainingMethod as MlTrainingMethod,
};
use crate::ml::node_prediction::NodeSplitter;
use crate::ml::splitting::TrainingExamplesSplit;
use crate::ml::training::{CrossValidation, TrainingStatistics};
use crate::procedures::AlgorithmsProcedureFacade;
use crate::projection::eval::pipeline::NodeFeatureProducer;
use crate::projection::eval::pipeline::NodePropertyPipelineBaseTrainConfig;
use crate::projection::eval::pipeline::NodePropertyTrainingPipeline;
use crate::projection::eval::pipeline::Pipeline;
use crate::projection::eval::pipeline::PipelineTrainer;
use crate::projection::eval::pipeline::TrainingMethod as PipelineTrainingMethod;
use crate::projection::eval::pipeline::TrainingPipeline;
use crate::task::concurrency::{Concurrency, TerminationFlag};
use crate::task::memory::{Estimate, MemoryEstimation, MemoryEstimations, MemoryRange};
use crate::task::progress::{NoopProgressTracker, ProgressTracker, Task, Tasks};
use crate::types::graph::Graph;
use crate::types::graph_store::DefaultGraphStore;
use crate::types::prelude::GraphStore;
use parking_lot::RwLock;
use std::collections::{BTreeSet, HashMap};
use std::sync::Arc;

/// Core training algorithm for node classification.
///
/// This implements the training loop:
/// 1. Extract target labels from the target property
/// 2. Split data into train/test/validation sets
/// 3. Select model with cross-validation
/// 4. Evaluate on train/test sets
/// 5. Retrain on full training set
pub struct NodeClassificationTrain {
    pipeline: NodeClassificationTrainingPipeline,
    train_config: NodeClassificationPipelineTrainConfig,
    targets: HugeIntArray,
    class_id_map: LocalIdMap,
    class_counts: LongMultiSet,
    node_graph: Arc<dyn Graph>,
    target_node_ids: Arc<Vec<u64>>,
    node_feature_producer: NodeFeatureProducer<NodeClassificationPipelineTrainConfig>,
    progress_tracker: Box<dyn ProgressTracker>,
    termination_flag: TerminationFlag,
}

impl NodeClassificationTrain {
    /// Estimate memory requirements for training.
    pub fn estimate(
        pipeline: &NodeClassificationTrainingPipeline,
        configuration: &NodeClassificationPipelineTrainConfig,
        _model_catalog: &impl ModelCatalog,
        _algorithms_procedure_facade: &AlgorithmsProcedureFacade,
    ) -> Box<dyn MemoryEstimation> {
        Self::estimate_pipeline(pipeline, configuration)
    }

    pub fn estimate_pipeline(
        pipeline: &NodeClassificationTrainingPipeline,
        configuration: &NodeClassificationPipelineTrainConfig,
    ) -> Box<dyn MemoryEstimation> {
        let node_property_steps =
            estimate_node_property_steps(pipeline.node_property_steps().len());
        let training = MemoryEstimations::builder("Training")
            .add(estimate_excluding_node_property_steps(
                configuration.metrics_specs().len(),
                pipeline,
            ))
            .build();

        MemoryEstimations::max_estimation_named(
            "Node Classification Train Pipeline",
            vec![node_property_steps, training],
        )
    }

    /// Create progress task for training.
    pub fn progress_task(pipeline: &NodeClassificationTrainingPipeline, node_count: u64) -> Task {
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
            Tasks::leaf_with_volume("Evaluate on train data".to_string(), train_set_size)
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

        Tasks::task("Node Classification Train Pipeline".to_string(), tasks)
    }

    /// Create a new NodeClassificationTrain instance.
    pub fn create(
        graph_store: Arc<DefaultGraphStore>,
        pipeline: NodeClassificationTrainingPipeline,
        config: NodeClassificationPipelineTrainConfig,
        node_feature_producer: NodeFeatureProducer<NodeClassificationPipelineTrainConfig>,
        progress_tracker: Box<dyn ProgressTracker>,
    ) -> Self {
        let node_graph = graph_store.get_graph();
        let target_node_labels = config
            .validate_target_node_label_identifiers(&graph_store)
            .expect("Invalid target node labels for classification");
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
            .expect("Missing target node property for classification");

        let labels_and_counts =
            LabelsAndClassCountsExtractor::extract_labels_and_class_counts_for_node_ids(
                target_node_property.as_ref(),
                &target_node_ids,
            );

        let class_counts = labels_and_counts.class_counts().clone();
        let targets = labels_and_counts.labels().clone();

        let mut class_ids: Vec<u64> = class_counts
            .keys()
            .into_iter()
            .map(|id| id as u64)
            .collect();
        class_ids.sort_unstable();
        let class_id_map = LocalIdMap::of_sorted(&class_ids);

        let termination_flag = TerminationFlag::running_true();

        Self {
            pipeline,
            train_config: config,
            targets,
            class_id_map,
            class_counts,
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
    ) -> Result<NodeClassificationTrainResult, Box<dyn std::error::Error + Send + Sync>> {
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

        let metrics = self.build_metrics()?;
        let classification_metrics =
            NodeClassificationPipelineTrainConfig::classification_metrics(&metrics);
        let mut training_statistics = TrainingStatistics::new(&metrics);

        let metrics_for_cv = self.build_metrics()?;
        let metrics_for_cv_eval = Arc::new(self.build_metrics()?);

        let features = self
            .node_feature_producer
            .procedure_features(&self.pipeline)
            .map_err(|e| format!("Feature production failed: {e}"))?;
        let features: Arc<dyn Features> = Arc::from(features);

        let labels_long = Arc::new(labels_as_long(&self.targets));
        let labels_vec = Arc::new(labels_long.iter().collect::<Vec<_>>());
        let distinct_targets: BTreeSet<i64> =
            (0..self.class_id_map.size()).map(|v| v as i64).collect();

        let candidates = self.collect_candidate_configs();
        let candidate_configs_for_cv: Vec<Box<dyn TrainerConfigTrait>> = candidates
            .iter()
            .map(|(method, config_map)| {
                create_trainer_config_from_map(config_map.clone(), map_training_method(*method))
            })
            .collect();

        let cv_termination_flag = Arc::new(RwLock::new(false));
        let random_seed = self.train_config.random_seed();
        let termination_flag = self.termination_flag.clone();
        let targets = Arc::new(self.targets.clone());
        let class_count = self.class_id_map.size();

        let cv = CrossValidation::new(
            cv_termination_flag,
            metrics_for_cv,
            split_config.validation_folds(),
            random_seed,
            Box::new({
                let features = Arc::clone(&features);
                let targets = Arc::clone(&targets);
                move |train_set, trainer_config, metrics_handler, _name| {
                    let train_set = to_u64_arc(train_set);
                    let progress_tracker = NoopProgressTracker;
                    let trainer = ClassifierTrainerFactory::create(
                        trainer_config,
                        class_count,
                        &termination_flag,
                        &progress_tracker,
                        &Concurrency::available_cores(),
                        random_seed,
                        false,
                        metrics_handler,
                    );

                    trainer.train(features.as_ref(), &targets, &train_set)
                }
            }),
            Box::new({
                let features = Arc::clone(&features);
                let labels_long = Arc::clone(&labels_long);
                let metrics_for_cv_eval = Arc::clone(&metrics_for_cv_eval);
                move |eval_set, model, consumer| {
                    let classification_metrics =
                        NodeClassificationPipelineTrainConfig::classification_metrics(
                            metrics_for_cv_eval.as_ref(),
                        );
                    let scores = evaluate_metrics(
                        &eval_set,
                        model.as_ref(),
                        features.as_ref(),
                        labels_long.as_ref(),
                        &classification_metrics,
                    );

                    for metric in classification_metrics {
                        let value = scores.get(metric.name()).copied().unwrap_or(0.0);
                        consumer.consume(metric as &dyn Metric, value);
                    }
                }
            }),
        );

        let outer_train = node_splits.outer_split().train_set();
        cv.select_model(
            outer_train,
            {
                let labels_vec = Arc::clone(&labels_vec);
                move |node_id| labels_vec[node_id as usize]
            },
            distinct_targets,
            &mut training_statistics,
            candidate_configs_for_cv.into_iter(),
        );

        let best_idx = training_statistics.best_trial_idx();
        let (best_method, best_map) = candidates
            .get(best_idx)
            .expect("At least one trainer config is required");
        let best_config =
            create_trainer_config_from_map(best_map.clone(), map_training_method(*best_method));

        let classifier = self.train_simple_model(
            node_splits.outer_split(),
            features.as_ref(),
            best_config.as_ref(),
        )?;
        let (train_scores, test_scores) = self.evaluate_model(
            node_splits.outer_split(),
            features.as_ref(),
            &classifier,
            &classification_metrics,
        );

        for (metric, score) in &train_scores {
            training_statistics.add_outer_train_score(metric.clone(), *score);
        }
        for (metric, score) in &test_scores {
            training_statistics.add_test_score(metric.clone(), *score);
        }

        let retrained = self.retrain_best_model(
            node_splits.all_training_examples(),
            features.as_ref(),
            best_config.as_ref(),
        )?;

        self.progress_tracker.end_subtask();

        Ok(NodeClassificationTrainResult::new(
            retrained,
            training_statistics,
            self.class_id_map.clone(),
            self.class_counts.clone(),
        ))
    }

    fn build_metrics(&self) -> Result<Vec<Box<dyn Metric>>, String> {
        if self.train_config.metrics_specs().is_empty() {
            let defaults = vec![
                "ACCURACY".to_string(),
                "F1_WEIGHTED".to_string(),
                "F1_MACRO".to_string(),
                "F1(class=*)".to_string(),
                "PRECISION(class=*)".to_string(),
                "RECALL(class=*)".to_string(),
            ];

            let specs = ClassificationMetricSpecification::parse_list(&defaults)?;
            Ok(specs
                .iter()
                .flat_map(|spec| spec.create_metrics(&self.class_id_map, &self.class_counts))
                .collect())
        } else {
            Ok(self
                .train_config
                .metrics(&self.class_id_map, &self.class_counts))
        }
    }
}

impl PipelineTrainer for NodeClassificationTrain {
    type Result = NodeClassificationTrainResult;

    fn set_termination_flag(&mut self, termination_flag: TerminationFlag) {
        NodeClassificationTrain::set_termination_flag(self, termination_flag);
    }

    fn run(&mut self) -> Result<Self::Result, Box<dyn std::error::Error + Send + Sync>> {
        self.run()
    }

    fn is_terminated(&self) -> bool {
        !self.termination_flag.running()
    }
}

impl NodeClassificationTrain {
    fn train_simple_model(
        &self,
        split: &TrainingExamplesSplit,
        features: &dyn Features,
        trainer_config: &dyn TrainerConfigTrait,
    ) -> Result<Box<dyn Classifier>, Box<dyn std::error::Error + Send + Sync>> {
        let train_set = to_u64_arc(split.train_set());
        let trainer = ClassifierTrainerFactory::create(
            trainer_config,
            self.class_id_map.size(),
            &self.termination_flag,
            &*self.progress_tracker,
            &Concurrency::available_cores(),
            self.train_config.random_seed(),
            false,
            &ModelSpecificMetricsHandler::noop(),
        );

        Ok(trainer.train(features, &self.targets, &train_set))
    }

    fn retrain_best_model(
        &self,
        all_training_examples: &Arc<Vec<i64>>,
        features: &dyn Features,
        trainer_config: &dyn TrainerConfigTrait,
    ) -> Result<Box<dyn Classifier>, Box<dyn std::error::Error + Send + Sync>> {
        let train_set = to_u64_arc(all_training_examples.clone());
        let trainer = ClassifierTrainerFactory::create(
            trainer_config,
            self.class_id_map.size(),
            &self.termination_flag,
            &*self.progress_tracker,
            &Concurrency::available_cores(),
            self.train_config.random_seed(),
            false,
            &ModelSpecificMetricsHandler::noop(),
        );

        Ok(trainer.train(features, &self.targets, &train_set))
    }

    fn evaluate_model(
        &self,
        split: &TrainingExamplesSplit,
        features: &dyn Features,
        classifier: &Box<dyn Classifier>,
        classification_metrics: &[&dyn ClassificationMetric],
    ) -> (HashMap<String, f64>, HashMap<String, f64>) {
        let labels_long = labels_as_long(&self.targets);
        let train_scores = evaluate_metrics(
            &split.train_set(),
            classifier.as_ref(),
            features,
            &labels_long,
            classification_metrics,
        );
        let test_scores = evaluate_metrics(
            &split.test_set(),
            classifier.as_ref(),
            features,
            &labels_long,
            classification_metrics,
        );

        (train_scores, test_scores)
    }

    fn collect_candidate_configs(
        &self,
    ) -> Vec<(PipelineTrainingMethod, HashMap<String, serde_json::Value>)> {
        let mut candidates: Vec<(PipelineTrainingMethod, HashMap<String, serde_json::Value>)> =
            Vec::new();

        for (method, configs) in self.pipeline.training_parameter_space() {
            if configs.is_empty() {
                candidates.push((*method, HashMap::<String, serde_json::Value>::new()));
                continue;
            }

            for cfg in configs {
                candidates.push((*method, cfg.to_map()));
            }
        }

        if candidates.is_empty() {
            candidates.push((
                PipelineTrainingMethod::MLPClassification,
                HashMap::<String, serde_json::Value>::new(),
            ));
        }

        let max_trials = self.pipeline.auto_tuning_config().max_trials();
        if max_trials > 0 && candidates.len() > max_trials {
            candidates.truncate(max_trials);
        }

        candidates
    }
}

fn map_training_method(method: PipelineTrainingMethod) -> MlTrainingMethod {
    match method {
        PipelineTrainingMethod::LogisticRegression => MlTrainingMethod::LogisticRegression,
        PipelineTrainingMethod::RandomForestClassification => {
            MlTrainingMethod::RandomForestClassification
        }
        PipelineTrainingMethod::MLPClassification => MlTrainingMethod::MLPClassification,
        other => panic!("Unsupported training method for classification: {other:?}"),
    }
}

fn labels_as_long(labels: &HugeIntArray) -> HugeLongArray {
    let mut out = HugeLongArray::new(labels.size());
    for idx in 0..labels.size() {
        out.set(idx, labels.get(idx) as i64);
    }
    out
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
    pipeline: &NodeClassificationTrainingPipeline,
) -> Box<dyn MemoryEstimation> {
    const FUDGED_CLASS_COUNT: usize = 1000;
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
            MemoryRange::of(HugeIntArray::memory_estimation(dim.node_count()))
        })
        .fixed_range(
            "global class counts",
            MemoryRange::of_range(
                2 * std::mem::size_of::<u64>(),
                FUDGED_CLASS_COUNT * std::mem::size_of::<u64>(),
            ),
        )
        .add(ClassificationMetricSpecification::memory_estimation(
            FUDGED_CLASS_COUNT,
        ))
        .range_per_graph_dimension("node IDs", |dim, _| {
            MemoryRange::of(Estimate::size_of_long_array(dim.node_count()))
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

fn to_u64_arc(values: Arc<Vec<i64>>) -> Arc<Vec<u64>> {
    Arc::new(values.iter().map(|v| *v as u64).collect())
}

fn evaluate_metrics(
    evaluation_set: &Arc<Vec<i64>>,
    classifier: &dyn Classifier,
    features: &dyn Features,
    labels: &HugeLongArray,
    metrics: &[&dyn ClassificationMetric],
) -> HashMap<String, f64> {
    let eval_ids: Vec<usize> = evaluation_set.iter().map(|v| *v as usize).collect();
    let mut predictions = HugeLongArray::new(eval_ids.len());
    let mut eval_labels = HugeLongArray::new(eval_ids.len());

    for (idx, node_id) in eval_ids.iter().enumerate() {
        let probs = classifier.predict_probabilities(features.get(*node_id));
        let predicted = probs
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(idx, _)| idx as i64)
            .unwrap_or(0);
        predictions.set(idx, predicted);
        eval_labels.set(idx, labels.get(*node_id));
    }

    metrics
        .iter()
        .map(|metric| {
            (
                metric.name().to_string(),
                metric.compute(&eval_labels, &predictions),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::backends::vec::VecLong;
    use crate::core::graph_dimensions::ConcreteGraphDimensions;
    use crate::core::model::EmptyModelCatalog;
    use crate::task::progress::NoopProgressTracker;
    use crate::types::graph_store::DefaultGraphStore;
    use crate::types::properties::node::DefaultLongNodePropertyValues;
    use crate::types::random::RandomGraphConfig;
    use std::sync::Arc;

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
        let pipeline = NodeClassificationTrainingPipeline::new();
        let train_config = NodeClassificationPipelineTrainConfig::default();
        let node_feature_producer =
            NodeFeatureProducer::create(graph_store.clone(), train_config.clone());
        let progress_tracker = Box::new(NoopProgressTracker);

        let _trainer = NodeClassificationTrain::create(
            graph_store,
            pipeline,
            train_config,
            node_feature_producer,
            progress_tracker,
        );
    }

    #[test]
    fn test_progress_task() {
        let pipeline = NodeClassificationTrainingPipeline::new();
        let node_count = 1000_usize;

        let task = NodeClassificationTrain::progress_task(&pipeline, node_count as u64);

        let split_config = pipeline.split_config();
        let train_set_size = split_config.train_set_size(node_count);
        let test_set_size = split_config.test_set_size(node_count);
        let validation_folds = split_config.validation_folds();
        let trials = pipeline.number_of_model_selection_trials();
        let sub_tasks = task.sub_tasks();

        assert_eq!(task.description(), "Node Classification Train Pipeline");
        assert_eq!(sub_tasks.len(), 6);
        assert_eq!(sub_tasks[0].description(), "Node property steps");
        assert_eq!(sub_tasks[0].current_volume(), 0);
        assert_eq!(
            sub_tasks[1].description(),
            format!("Cross-validation ({validation_folds} folds, {trials} trials)")
        );
        assert_eq!(
            sub_tasks[1].current_volume(),
            train_set_size * validation_folds * trials
        );
        assert_eq!(sub_tasks[2].description(), "Train best model");
        assert_eq!(sub_tasks[2].current_volume(), 5 * train_set_size);
        assert_eq!(sub_tasks[3].description(), "Evaluate on train data");
        assert_eq!(sub_tasks[3].current_volume(), train_set_size);
        assert_eq!(sub_tasks[4].description(), "Evaluate on test data");
        assert_eq!(sub_tasks[4].current_volume(), test_set_size);
        assert_eq!(sub_tasks[5].description(), "Retrain best model");
        assert_eq!(sub_tasks[5].current_volume(), 5 * node_count);
    }

    #[test]
    fn test_estimate() {
        let pipeline = NodeClassificationTrainingPipeline::new();
        let config = NodeClassificationPipelineTrainConfig::default();
        let model_catalog = EmptyModelCatalog;
        let graph_store = Arc::new(
            DefaultGraphStore::random(&RandomGraphConfig::default())
                .expect("random graph generation"),
        );
        let algorithms_facade = AlgorithmsProcedureFacade::from_store(graph_store);

        let estimation = NodeClassificationTrain::estimate(
            &pipeline,
            &config,
            &model_catalog,
            &algorithms_facade,
        );
        let tree = estimation.estimate(&ConcreteGraphDimensions::of(100, 0), 4);

        assert_eq!(tree.description(), "Node Classification Train Pipeline");
        assert_eq!(tree.components().len(), 2);
        assert_eq!(tree.components()[0].description(), "Node property steps");
        assert_eq!(tree.components()[1].description(), "Training");
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
        let property_values = Arc::new(DefaultLongNodePropertyValues::from_collection(
            VecLong::from(
                (0..graph_store.node_count())
                    .map(|node_id| (node_id % 2) as i64)
                    .collect::<Vec<_>>(),
            ),
            graph_store.node_count(),
        ));
        graph_store
            .add_node_property(graph_store.node_labels(), "target", property_values)
            .expect("target property should be added");

        let graph_store = Arc::new(graph_store);
        let train_config = NodeClassificationPipelineTrainConfig::new(
            "test-pipeline".to_string(),
            vec![target_label.name().to_string()],
            "target".to_string(),
            Some(42),
            vec![],
        );
        let node_feature_producer =
            NodeFeatureProducer::create(graph_store.clone(), train_config.clone());

        let trainer = NodeClassificationTrain::create(
            graph_store,
            NodeClassificationTrainingPipeline::new(),
            train_config,
            node_feature_producer,
            Box::new(NoopProgressTracker),
        );

        assert_eq!(trainer.target_node_ids.len(), target_count);
        assert_eq!(trainer.targets.size(), target_count);
        assert_eq!(trainer.class_counts.sum() as usize, target_count);
    }

    #[test]
    #[should_panic(expected = "Missing target node property for classification")]
    fn test_run() {
        let config = RandomGraphConfig {
            node_count: 10,
            seed: Some(42),
            ..RandomGraphConfig::default()
        };
        let graph_store =
            Arc::new(DefaultGraphStore::random(&config).expect("Failed to generate random graph"));
        let pipeline = NodeClassificationTrainingPipeline::new();
        let config = NodeClassificationPipelineTrainConfig::default();
        let node_feature_producer =
            NodeFeatureProducer::create(graph_store.clone(), config.clone());
        let progress_tracker = Box::new(NoopProgressTracker);

        let mut trainer = NodeClassificationTrain::create(
            graph_store,
            pipeline,
            config,
            node_feature_producer,
            progress_tracker,
        );

        let _result = trainer.run();
    }
}
