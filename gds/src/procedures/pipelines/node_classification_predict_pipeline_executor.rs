use std::collections::HashSet;
use std::sync::Arc;

use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::utils::progress::{Task, TaskProgressTracker, Tasks};
use crate::ml::core::subgraph::LocalIdMap;
use crate::ml::models::{ClassifierData, ClassifierFactory, FeaturesFactory};
use crate::ml::node_classification::NodeClassificationPredict;
use crate::projection::eval::pipeline::node_pipeline::NodePropertyPredictPipeline;
use crate::projection::eval::pipeline::PipelineGraphFilter;
use crate::projection::eval::pipeline::PredictPipelineExecutor;
use crate::projection::eval::pipeline::PredictPipelineExecutorError;
use crate::projection::NodeLabel;
use crate::types::graph_store::{DefaultGraphStore, GraphStore};

use super::{
    node_classification_predict_pipeline_constants::MIN_BATCH_SIZE,
    NodeClassificationPipelineResult, NodeClassificationPredictPipelineConfig,
};

pub struct NodeClassificationPredictPipelineExecutor<'a> {
    pipeline: &'a NodePropertyPredictPipeline,
    configuration: &'a dyn NodeClassificationPredictPipelineConfig,
    graph_store: Arc<DefaultGraphStore>,
    progress_tracker: TaskProgressTracker,
    classifier_data: &'a dyn ClassifierData,
    class_id_map: LocalIdMap,
    termination_flag: TerminationFlag,
}

impl<'a> NodeClassificationPredictPipelineExecutor<'a> {
    pub fn new(
        pipeline: &'a NodePropertyPredictPipeline,
        configuration: &'a dyn NodeClassificationPredictPipelineConfig,
        graph_store: Arc<DefaultGraphStore>,
        progress_tracker: TaskProgressTracker,
        classifier_data: &'a dyn ClassifierData,
        class_id_map: LocalIdMap,
        termination_flag: TerminationFlag,
    ) -> Self {
        Self {
            pipeline,
            configuration,
            graph_store,
            progress_tracker,
            classifier_data,
            class_id_map,
            termination_flag,
        }
    }

    pub fn progress_task(task_name: &str, graph_store: &DefaultGraphStore) -> Task {
        Tasks::leaf_with_volume(format!("{task_name} progress"), graph_store.node_count())
            .base()
            .clone()
    }

    fn resolved_node_labels(&self) -> Vec<String> {
        let labels = self.configuration.node_labels();
        if labels.is_empty() || (labels.len() == 1 && labels[0] == "*") {
            let mut labels: Vec<String> = self
                .graph_store
                .node_labels()
                .into_iter()
                .map(|label| label.name().to_string())
                .collect();
            labels.sort();
            labels
        } else {
            labels.to_vec()
        }
    }

    fn prediction_node_ids(&self, graph: &Arc<dyn crate::types::graph::Graph>) -> Vec<u64> {
        let labels = self.resolved_node_labels();
        if labels.is_empty() {
            return graph.iter().map(|node_id| node_id as u64).collect();
        }

        let label_set: HashSet<NodeLabel> = labels
            .iter()
            .map(|label| NodeLabel::of(label.as_str()))
            .collect();

        graph
            .iter_with_labels(&label_set)
            .map(|node_id| node_id as u64)
            .collect()
    }
}

impl<'a> PredictPipelineExecutor<NodePropertyPredictPipeline, NodeClassificationPipelineResult>
    for NodeClassificationPredictPipelineExecutor<'a>
{
    fn pipeline(&self) -> &NodePropertyPredictPipeline {
        self.pipeline
    }

    fn pipeline_and_graph_store_mut(
        &mut self,
    ) -> (&NodePropertyPredictPipeline, &mut Arc<DefaultGraphStore>) {
        (self.pipeline, &mut self.graph_store)
    }

    fn graph_store_mut(&mut self) -> &mut Arc<DefaultGraphStore> {
        &mut self.graph_store
    }

    fn graph_store(&self) -> &Arc<DefaultGraphStore> {
        &self.graph_store
    }

    fn node_labels(&self) -> &[String] {
        self.configuration.node_labels()
    }

    fn relationship_types(&self) -> &[String] {
        self.configuration.relationship_types()
    }

    fn concurrency(&self) -> usize {
        self.configuration.concurrency()
    }

    fn execute(
        &mut self,
    ) -> Result<NodeClassificationPipelineResult, PredictPipelineExecutorError> {
        let graph = self.graph_store.get_graph();
        let root_node_count = graph.node_count();
        let prediction_node_ids = self.prediction_node_ids(&graph);
        let features = FeaturesFactory::extract_lazy_features_for_node_ids(
            Arc::clone(&graph),
            &self.pipeline.feature_properties(),
            prediction_node_ids.clone(),
        );

        let classifier = ClassifierFactory::create(self.classifier_data);

        let classifier = Arc::from(classifier);

        let expected_dimension = self.classifier_data.feature_dimension();
        if features.feature_dimension() != expected_dimension {
            return Err(PredictPipelineExecutorError::ExecutionFailed(format!(
                "Model expected features {:?} to have a total dimension of `{}`, but got `{}`.",
                self.pipeline.feature_properties(),
                expected_dimension,
                features.feature_dimension()
            )));
        }

        let predict = NodeClassificationPredict::new(
            classifier,
            Arc::from(features),
            MIN_BATCH_SIZE,
            self.configuration.include_predicted_probabilities(),
            Concurrency::of(self.configuration.concurrency()),
            self.termination_flag.clone(),
            self.progress_tracker.clone(),
        );

        let result = predict.compute();
        Ok(NodeClassificationPipelineResult::of_for_node_ids(
            &result,
            &self.class_id_map,
            prediction_node_ids,
            root_node_count,
        ))
    }

    fn node_property_step_filter(&self) -> PipelineGraphFilter {
        PipelineGraphFilter::new(
            self.resolved_node_labels(),
            Some(self.configuration.relationship_types().to_vec()),
        )
    }
}
