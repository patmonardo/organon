use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTimings, GraphStoreNodePropertiesWritten, WriteResultBuilder,
};
use crate::core::loading::GraphResources;
use crate::procedures::pipelines::types::{StandardWriteResult, WriteResult};
use crate::procedures::pipelines::{
    NodeClassificationPipelineResult, NodeClassificationPredictPipelineWriteConfig,
};

pub struct NodeClassificationPredictPipelineWriteResultBuilder {
    configuration: NodeClassificationPredictPipelineWriteConfig,
}

impl NodeClassificationPredictPipelineWriteResultBuilder {
    pub fn new(configuration: NodeClassificationPredictPipelineWriteConfig) -> Self {
        Self { configuration }
    }
}

impl
    WriteResultBuilder<
        NodeClassificationPredictPipelineWriteConfig,
        NodeClassificationPipelineResult,
        WriteResult,
        GraphStoreNodePropertiesWritten,
    > for NodeClassificationPredictPipelineWriteResultBuilder
{
    fn build(
        &self,
        _graph_resources: &GraphResources,
        _configuration: &NodeClassificationPredictPipelineWriteConfig,
        result: Option<NodeClassificationPipelineResult>,
        timings: AlgorithmProcessingTimings,
        metadata: Option<GraphStoreNodePropertiesWritten>,
    ) -> WriteResult {
        let node_properties_written = if result.is_some() {
            metadata
                .expect(
                    "node classification write result requires node-properties-written metadata",
                )
                .0 as i64
        } else {
            0
        };

        WriteResult {
            base: StandardWriteResult {
                pre_processing_millis: timings.pre_processing_millis,
                compute_millis: timings.compute_millis,
                write_millis: timings.side_effect_millis,
                configuration: self.configuration.to_map(),
            },
            node_properties_written,
        }
    }
}
