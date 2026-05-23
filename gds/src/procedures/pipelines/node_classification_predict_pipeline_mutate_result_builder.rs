use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTimings, GraphStoreNodePropertiesWritten, MutateResultBuilder,
};
use crate::core::loading::GraphResources;
use crate::procedures::pipelines::types::{PredictMutateResult, StandardMutateResult};
use crate::procedures::pipelines::{
    NodeClassificationPipelineResult, NodeClassificationPredictPipelineMutateConfig,
};

pub struct NodeClassificationPredictPipelineMutateResultBuilder {
    configuration: NodeClassificationPredictPipelineMutateConfig,
}

impl NodeClassificationPredictPipelineMutateResultBuilder {
    pub fn new(configuration: NodeClassificationPredictPipelineMutateConfig) -> Self {
        Self { configuration }
    }
}

impl
    MutateResultBuilder<
        NodeClassificationPredictPipelineMutateConfig,
        NodeClassificationPipelineResult,
        PredictMutateResult,
        GraphStoreNodePropertiesWritten,
    > for NodeClassificationPredictPipelineMutateResultBuilder
{
    fn build(
        &self,
        _graph_resources: &GraphResources,
        _configuration: &NodeClassificationPredictPipelineMutateConfig,
        result: Option<NodeClassificationPipelineResult>,
        timings: AlgorithmProcessingTimings,
        metadata: Option<GraphStoreNodePropertiesWritten>,
    ) -> PredictMutateResult {
        let node_properties_written = if result.is_some() {
            metadata
                .expect(
                    "node classification mutate result requires node-properties-written metadata",
                )
                .0 as i64
        } else {
            0
        };

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
