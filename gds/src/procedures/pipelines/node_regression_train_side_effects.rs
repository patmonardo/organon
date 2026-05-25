use crate::applications::algorithms::machinery::SideEffect;
use crate::core::loading::GraphResources;
use crate::projection::eval::pipeline::node_pipeline::NodeRegressionPipelineTrainConfig;
use crate::projection::eval::pipeline::node_pipeline::NodeRegressionTrainPipelineResult;

pub struct NodeRegressionTrainSideEffects {
    #[allow(dead_code)]
    configuration: NodeRegressionPipelineTrainConfig,
}

impl NodeRegressionTrainSideEffects {
    pub fn new(configuration: NodeRegressionPipelineTrainConfig) -> Self {
        Self { configuration }
    }
}

impl SideEffect<NodeRegressionTrainPipelineResult, ()> for NodeRegressionTrainSideEffects {
    fn process(
        &self,
        _graph_resources: &GraphResources,
        _result: Option<&NodeRegressionTrainPipelineResult>,
    ) -> Option<()> {
        None
    }
}
