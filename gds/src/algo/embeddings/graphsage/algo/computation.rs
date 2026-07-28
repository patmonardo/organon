//! GraphSAGE model-backed inference runtime.

use super::spec::GraphSageResult;
use super::GraphSage;
use crate::algo::embeddings::graphsage::algo::graph_sage_model_data::GraphSageModelData;
use crate::algo::embeddings::graphsage::graphsage_model_trainer::GraphSageTrainMetrics;
use crate::algo::embeddings::graphsage::types::GraphSageTrainConfig;
use crate::core::model::Model;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::task::concurrency::{Concurrency, TerminationFlag};
use crate::task::progress::TaskProgressTracker;
use crate::types::graph::Graph;
use std::sync::Arc;

pub struct GraphSageComputationRuntime;

impl GraphSageComputationRuntime {
    pub fn run(
        graph: Arc<dyn Graph>,
        model: Arc<Model<GraphSageModelData, GraphSageTrainConfig, GraphSageTrainMetrics>>,
        concurrency: usize,
        batch_size: usize,
        progress_tracker: TaskProgressTracker,
        termination_flag: TerminationFlag,
    ) -> Result<GraphSageResult, AlgorithmError> {
        if !termination_flag.running() {
            return Err(AlgorithmError::Execution(
                "GraphSAGE inference terminated".to_string(),
            ));
        }

        let result = GraphSage::new(
            graph,
            model,
            Concurrency::of(concurrency),
            batch_size,
            progress_tracker,
            termination_flag,
        )
        .compute();
        let node_count = result.embeddings.size();
        let embedding_dimension = if node_count == 0 {
            0
        } else {
            result.embeddings.get(0).len()
        };

        Ok(GraphSageResult {
            embeddings: result.embeddings,
            embedding_dimension,
            node_count,
        })
    }
}
