use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTimings, GraphStoreNodePropertiesWritten, MutateResultBuilder,
};
use crate::core::loading::GraphResources;
use crate::procedures::pipelines::types::{PredictMutateResult, StandardMutateResult};
use crate::procedures::pipelines::{
    NodeRegressionPipelineResult, NodeRegressionPredictPipelineMutateConfig,
};

pub struct NodeRegressionPredictPipelineMutateResultBuilder {
    configuration: NodeRegressionPredictPipelineMutateConfig,
}

impl NodeRegressionPredictPipelineMutateResultBuilder {
    pub fn new(configuration: NodeRegressionPredictPipelineMutateConfig) -> Self {
        Self { configuration }
    }
}

impl
    MutateResultBuilder<
        NodeRegressionPredictPipelineMutateConfig,
        NodeRegressionPipelineResult,
        PredictMutateResult,
        GraphStoreNodePropertiesWritten,
    > for NodeRegressionPredictPipelineMutateResultBuilder
{
    fn build(
        &self,
        _graph_resources: &GraphResources,
        _configuration: &NodeRegressionPredictPipelineMutateConfig,
        result: Option<NodeRegressionPipelineResult>,
        timings: AlgorithmProcessingTimings,
        metadata: Option<GraphStoreNodePropertiesWritten>,
    ) -> PredictMutateResult {
        let node_properties_written = result
            .as_ref()
            .map(|_| {
                metadata
                    .expect("Expected node properties written metadata for mutation result")
                    .0 as i64
            })
            .unwrap_or(0);

        PredictMutateResult {
            base: StandardMutateResult {
                pre_processing_millis: timings.pre_processing_millis,
                compute_millis: timings.compute_millis,
                mutate_millis: timings.side_effect_millis,
                configuration: self.configuration.to_map(),
            },
            node_properties_written,
        }
    }
}
