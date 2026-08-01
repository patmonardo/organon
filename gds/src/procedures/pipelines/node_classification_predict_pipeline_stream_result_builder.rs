use crate::applications::algorithms::machinery::StreamResultBuilder;
use crate::core::loading::GraphResources;
use crate::procedures::pipelines::types::NodeClassificationStreamResult;
use crate::procedures::pipelines::{
    NodeClassificationPipelineResult, NodeClassificationPredictPipelineStreamConfig,
};
use crate::types::graph_store::GraphStore;
use crate::types::graph::MappedNodeId;

pub struct NodeClassificationPredictPipelineStreamResultBuilder {
    _configuration: NodeClassificationPredictPipelineStreamConfig,
}

impl NodeClassificationPredictPipelineStreamResultBuilder {
    pub fn new(configuration: NodeClassificationPredictPipelineStreamConfig) -> Self {
        Self {
            _configuration: configuration,
        }
    }
}

impl StreamResultBuilder<NodeClassificationPipelineResult, NodeClassificationStreamResult>
    for NodeClassificationPredictPipelineStreamResultBuilder
{
    type Stream = std::vec::IntoIter<NodeClassificationStreamResult>;

    fn build(
        &self,
        graph_resources: &GraphResources,
        result: Option<NodeClassificationPipelineResult>,
    ) -> Self::Stream {
        let Some(result) = result else {
            return Vec::new().into_iter();
        };

        let graph = graph_resources.graph_store.get_graph();
        let predicted_classes = result.predicted_classes();
        let predicted_probabilities = result.predicted_probabilities();

        let node_ids: Vec<u64> = result
            .predicted_node_ids()
            .map(|node_ids| node_ids.to_vec())
            .unwrap_or_else(|| {
                (0..graph.node_count())
                    .map(|node_index| {
                        u64::from(
                            MappedNodeId::try_from(node_index)
                                .expect("graph node count exceeds the mapped node ID domain"),
                        )
                    })
                    .collect()
            });

        let mut rows = Vec::with_capacity(node_ids.len());
        for (row_id, node_id) in node_ids.into_iter().enumerate() {
            let mapped_node_id = MappedNodeId::new(node_id);
            let original = graph
                .to_original_node_id(mapped_node_id)
                .expect("prediction returned an unknown mapped node ID");
            let predicted = predicted_classes.get(row_id) as i64;
            let probs = predicted_probabilities.map(|p| p.get(row_id).clone());

            rows.push(NodeClassificationStreamResult {
                node_id: original.get(),
                predicted_class: predicted,
                predicted_probabilities: probs,
            });
        }

        rows.into_iter()
    }
}
