//! GraphSAGE Storage Runtime

use super::spec::{GraphSageConfig, GraphSageResult};
use super::GraphSageComputationRuntime;
use crate::algo::embeddings::graphsage::algo::graph_sage_model_data::GraphSageModelData;
use crate::algo::embeddings::graphsage::graphsage_model_trainer::GraphSageTrainMetrics;
use crate::algo::embeddings::graphsage::types::GraphSageTrainConfig;
use crate::core::model::ModelCatalog;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::task::concurrency::TerminationFlag;
use crate::task::progress::TaskProgressTracker;
use crate::types::graph::Graph;
use crate::types::graph::MappedNodeId;
use std::sync::Arc;

pub struct GraphSageStorageRuntime;

impl Default for GraphSageStorageRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl GraphSageStorageRuntime {
    pub fn new() -> Self {
        Self
    }

    pub fn compute<MC: ModelCatalog>(
        &self,
        graph: Arc<dyn Graph>,
        config: &GraphSageConfig,
        model_catalog: &MC,
        progress_tracker: TaskProgressTracker,
        termination_flag: TerminationFlag,
    ) -> Result<GraphSageResult, AlgorithmError> {
        self.validate_relationship_weights(graph.as_ref())?;
        let model = model_catalog
            .get::<GraphSageModelData, GraphSageTrainConfig, GraphSageTrainMetrics>(
                &config.model_user,
                &config.model_name,
            )
            .map_err(|error| {
                AlgorithmError::Execution(format!(
                    "GraphSAGE model '{}/{}' could not be resolved: {error}",
                    config.model_user, config.model_name
                ))
            })?;
        GraphSageComputationRuntime::run(
            graph,
            model,
            config.concurrency,
            config.batch_size,
            progress_tracker,
            termination_flag,
        )
    }

    fn validate_relationship_weights(&self, graph: &dyn Graph) -> Result<(), AlgorithmError> {
        if !graph.has_relationship_property() {
            return Ok(());
        }
        let fallback = graph.default_property_value();
        for node_index in 0..graph.node_count() {
            let node_id = MappedNodeId::try_from(node_index)
                .expect("validated GraphSAGE node index must fit a mapped node ID");
            for relationship in graph.stream_relationships_weighted(node_id, fallback) {
                if !relationship.weight().is_finite() {
                    return Err(AlgorithmError::Execution(format!(
                        "GraphSAGE relationship weights must be finite, got {}",
                        relationship.weight()
                    )));
                }
            }
        }
        Ok(())
    }
}
