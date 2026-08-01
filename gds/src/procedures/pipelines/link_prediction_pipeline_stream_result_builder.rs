use crate::applications::algorithms::machinery::StreamResultBuilder;
use crate::core::loading::GraphResources;
use crate::ml::link_models::LinkPredictionResult;
use crate::procedures::pipelines::types::StreamResult;
use crate::types::graph::MappedNodeId;
use crate::types::graph_store::GraphStore;

pub struct LinkPredictionPipelineStreamResultBuilder;

impl StreamResultBuilder<Box<dyn LinkPredictionResult>, StreamResult>
    for LinkPredictionPipelineStreamResultBuilder
{
    type Stream = std::vec::IntoIter<StreamResult>;

    fn build(
        &self,
        graph_resources: &GraphResources,
        result: Option<Box<dyn LinkPredictionResult>>,
    ) -> Self::Stream {
        let Some(predictions) = result else {
            return Vec::new().into_iter();
        };

        let graph = graph_resources.graph_store.get_graph();
        let mut rows = Vec::new();

        for link in predictions.iter() {
            let mapped_source = MappedNodeId::try_from(link.source_id())
                .expect("link prediction returned a negative mapped source node ID");
            let mapped_target = MappedNodeId::try_from(link.target_id())
                .expect("link prediction returned a negative mapped target node ID");
            let source = graph
                .to_original_node_id(mapped_source)
                .expect("link prediction returned an unknown mapped source node ID");
            let target = graph
                .to_original_node_id(mapped_target)
                .expect("link prediction returned an unknown mapped target node ID");
            rows.push(StreamResult {
                node1: source.get(),
                node2: target.get(),
                probability: link.probability(),
            });
        }

        rows.into_iter()
    }
}
