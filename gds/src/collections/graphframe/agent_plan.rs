//! GraphFrame Agent Plan interface for GraphStore facade replacement.
//!
//! This module provides a typed bridge from legacy TSJSON facade operation names
//! (`graph_store` and `graph_store_catalog`) into a GraphFrame-oriented plan
//! language suitable for Agent ToolChain compilation.

use crate::collections::graphframe::expr::GraphProcedureExpr;
use crate::shell::ShellComponentMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GraphProjectionMoment {
    /// Factory projection materializes or mutates GraphStore/Dataset state.
    Factory,
    /// Eval projection computes over a graph and projects observed results.
    Eval,
}

impl GraphProjectionMoment {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Factory => "factory",
            Self::Eval => "eval",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GraphAgentPlanAction {
    GraphPut,
    GraphExists,
    GraphList,
    GraphMemoryUsage,
    GraphDrop,
    GraphDropMany,
    NodePropertiesDrop,
    RelationshipsDrop,
    GraphPropertyDrop,
    NodePropertiesWrite,
    RelationshipPropertiesWrite,
    RelationshipsWrite,
    NodeLabelWrite,
    NodeLabelMutate,
    GraphPropertyStream,
    NodePropertiesStream,
    RelationshipPropertiesStream,
    RelationshipsStream,
    GraphGenerate,
    GraphSample,
    SubGraphProject,
    NativeProjectEstimate,
    CommonNeighbourAwareRandomWalkEstimate,
}

impl GraphAgentPlanAction {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::GraphPut => "graph.put",
            Self::GraphExists => "graph.exists",
            Self::GraphList => "graph.list",
            Self::GraphMemoryUsage => "graph.memory_usage",
            Self::GraphDrop => "graph.drop",
            Self::GraphDropMany => "graph.drop_many",
            Self::NodePropertiesDrop => "node_properties.drop",
            Self::RelationshipsDrop => "relationships.drop",
            Self::GraphPropertyDrop => "graph_property.drop",
            Self::NodePropertiesWrite => "node_properties.write",
            Self::RelationshipPropertiesWrite => "relationship_properties.write",
            Self::RelationshipsWrite => "relationships.write",
            Self::NodeLabelWrite => "node_label.write",
            Self::NodeLabelMutate => "node_label.mutate",
            Self::GraphPropertyStream => "graph_property.stream",
            Self::NodePropertiesStream => "node_properties.stream",
            Self::RelationshipPropertiesStream => "relationship_properties.stream",
            Self::RelationshipsStream => "relationships.stream",
            Self::GraphGenerate => "graph.generate",
            Self::GraphSample => "graph.sample",
            Self::SubGraphProject => "subgraph.project",
            Self::NativeProjectEstimate => "native_project.estimate",
            Self::CommonNeighbourAwareRandomWalkEstimate => {
                "common_neighbour_aware_random_walk.estimate"
            }
        }
    }

    pub fn requires_task_daemon(self) -> bool {
        matches!(
            self,
            Self::GraphGenerate
                | Self::GraphSample
                | Self::SubGraphProject
                | Self::RelationshipsWrite
                | Self::RelationshipPropertiesWrite
                | Self::NodePropertiesWrite
                | Self::NodeLabelWrite
                | Self::NodeLabelMutate
        )
    }

    pub fn projection_moment(self) -> GraphProjectionMoment {
        match self {
            Self::GraphPropertyStream
            | Self::NodePropertiesStream
            | Self::RelationshipPropertiesStream
            | Self::RelationshipsStream
            | Self::GraphMemoryUsage
            | Self::GraphExists
            | Self::GraphList
            | Self::NativeProjectEstimate
            | Self::CommonNeighbourAwareRandomWalkEstimate => GraphProjectionMoment::Eval,
            Self::GraphPut
            | Self::GraphDrop
            | Self::GraphDropMany
            | Self::NodePropertiesDrop
            | Self::RelationshipsDrop
            | Self::GraphPropertyDrop
            | Self::NodePropertiesWrite
            | Self::RelationshipPropertiesWrite
            | Self::RelationshipsWrite
            | Self::NodeLabelWrite
            | Self::NodeLabelMutate
            | Self::GraphGenerate
            | Self::GraphSample
            | Self::SubGraphProject => GraphProjectionMoment::Factory,
        }
    }

    pub fn shell_component_alias(self) -> &'static str {
        match self {
            Self::GraphPut => "put_graph_store",
            Self::GraphExists => "graph_exists",
            Self::GraphList => "list_graphs",
            Self::GraphMemoryUsage => "graph_memory_usage",
            Self::GraphDrop => "drop_graph",
            Self::GraphDropMany => "drop_graphs",
            Self::NodePropertiesDrop => "drop_node_properties",
            Self::RelationshipsDrop => "drop_relationships",
            Self::GraphPropertyDrop => "drop_graph_property",
            Self::NodePropertiesWrite => "write_node_properties",
            Self::RelationshipPropertiesWrite => "write_relationship_properties",
            Self::RelationshipsWrite => "write_relationships",
            Self::NodeLabelWrite => "write_node_label",
            Self::NodeLabelMutate => "mutate_label",
            Self::GraphPropertyStream => "stream_graph_property",
            Self::NodePropertiesStream => "stream_node_properties",
            Self::RelationshipPropertiesStream => "stream_relationship_properties",
            Self::RelationshipsStream => "stream_relationships",
            Self::GraphGenerate => "generate_graph",
            Self::GraphSample => "sample_graph",
            Self::SubGraphProject => "subgraph_project",
            Self::NativeProjectEstimate => "estimate_native_project",
            Self::CommonNeighbourAwareRandomWalkEstimate => {
                "estimate_common_neighbour_aware_random_walk"
            }
        }
    }

    pub fn shell_mode(self) -> ShellComponentMode {
        match self {
            Self::GraphPut
            | Self::NodePropertiesWrite
            | Self::RelationshipPropertiesWrite
            | Self::RelationshipsWrite
            | Self::NodeLabelWrite => ShellComponentMode::Write,

            Self::GraphDrop
            | Self::GraphDropMany
            | Self::NodePropertiesDrop
            | Self::RelationshipsDrop
            | Self::GraphPropertyDrop
            | Self::NodeLabelMutate
            | Self::GraphGenerate
            | Self::GraphSample
            | Self::SubGraphProject => ShellComponentMode::Mutate,

            Self::GraphPropertyStream
            | Self::NodePropertiesStream
            | Self::RelationshipPropertiesStream
            | Self::RelationshipsStream
            | Self::GraphList => ShellComponentMode::Stream,

            Self::GraphMemoryUsage
            | Self::NativeProjectEstimate
            | Self::CommonNeighbourAwareRandomWalkEstimate => ShellComponentMode::Estimate,

            Self::GraphExists => ShellComponentMode::Invoke,
        }
    }

    pub fn to_graph_procedure_expr(self) -> GraphProcedureExpr {
        GraphProcedureExpr::new(self.shell_component_alias(), self.shell_mode())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphAgentPlanInterface {
    pub facade: String,
    pub op: String,
    pub action: GraphAgentPlanAction,
    pub moment: GraphProjectionMoment,
    pub task_daemon_mediated: bool,
}

impl GraphAgentPlanInterface {
    pub fn language_id() -> &'static str {
        "graphframe.agent-plan.gdsl.v1"
    }

    pub fn classify(facade: &str, op: &str) -> Option<Self> {
        let action = classify_action(facade, op)?;
        Some(Self {
            facade: facade.to_string(),
            op: op.to_string(),
            action,
            moment: action.projection_moment(),
            task_daemon_mediated: action.requires_task_daemon(),
        })
    }

    pub fn to_graph_procedure_expr(&self) -> GraphProcedureExpr {
        self.action.to_graph_procedure_expr()
    }
}

fn classify_action(facade: &str, op: &str) -> Option<GraphAgentPlanAction> {
    match (facade, op) {
        ("graph_store", "put") => Some(GraphAgentPlanAction::GraphPut),

        ("graph_store_catalog", "graphExists") => Some(GraphAgentPlanAction::GraphExists),
        ("graph_store_catalog", "listGraphs") => Some(GraphAgentPlanAction::GraphList),
        ("graph_store_catalog", "graphMemoryUsage") => Some(GraphAgentPlanAction::GraphMemoryUsage),
        ("graph_store_catalog", "dropGraph") => Some(GraphAgentPlanAction::GraphDrop),
        ("graph_store_catalog", "dropGraphs") => Some(GraphAgentPlanAction::GraphDropMany),
        ("graph_store_catalog", "dropNodeProperties") => {
            Some(GraphAgentPlanAction::NodePropertiesDrop)
        }
        ("graph_store_catalog", "dropRelationships") => {
            Some(GraphAgentPlanAction::RelationshipsDrop)
        }
        ("graph_store_catalog", "dropGraphProperty") => {
            Some(GraphAgentPlanAction::GraphPropertyDrop)
        }
        ("graph_store_catalog", "writeNodeProperties") => {
            Some(GraphAgentPlanAction::NodePropertiesWrite)
        }
        ("graph_store_catalog", "writeRelationshipProperties") => {
            Some(GraphAgentPlanAction::RelationshipPropertiesWrite)
        }
        ("graph_store_catalog", "writeRelationships") => {
            Some(GraphAgentPlanAction::RelationshipsWrite)
        }
        ("graph_store_catalog", "writeNodeLabel") => Some(GraphAgentPlanAction::NodeLabelWrite),
        ("graph_store_catalog", "mutateLabel") => Some(GraphAgentPlanAction::NodeLabelMutate),
        ("graph_store_catalog", "streamGraphProperty") => {
            Some(GraphAgentPlanAction::GraphPropertyStream)
        }
        ("graph_store_catalog", "streamNodeProperties") => {
            Some(GraphAgentPlanAction::NodePropertiesStream)
        }
        ("graph_store_catalog", "streamRelationshipProperties") => {
            Some(GraphAgentPlanAction::RelationshipPropertiesStream)
        }
        ("graph_store_catalog", "streamRelationships") => {
            Some(GraphAgentPlanAction::RelationshipsStream)
        }
        ("graph_store_catalog", "generateGraph") => Some(GraphAgentPlanAction::GraphGenerate),
        ("graph_store_catalog", "sampleGraph") => Some(GraphAgentPlanAction::GraphSample),
        ("graph_store_catalog", "subGraphProject") => Some(GraphAgentPlanAction::SubGraphProject),
        ("graph_store_catalog", "estimateNativeProject") => {
            Some(GraphAgentPlanAction::NativeProjectEstimate)
        }
        ("graph_store_catalog", "estimateCommonNeighbourAwareRandomWalk") => {
            Some(GraphAgentPlanAction::CommonNeighbourAwareRandomWalkEstimate)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::GraphAgentPlanAction;
    use super::GraphAgentPlanInterface;
    use super::GraphProjectionMoment;

    #[test]
    fn classifies_graph_store_put_as_factory() {
        let plan = GraphAgentPlanInterface::classify("graph_store", "put")
            .expect("graph_store.put should classify");

        assert_eq!(plan.action, GraphAgentPlanAction::GraphPut);
        assert_eq!(plan.moment, GraphProjectionMoment::Factory);
        assert!(!plan.task_daemon_mediated);
    }

    #[test]
    fn classifies_stream_as_eval() {
        let plan = GraphAgentPlanInterface::classify("graph_store_catalog", "streamNodeProperties")
            .expect("graph_store_catalog.streamNodeProperties should classify");

        assert_eq!(plan.action, GraphAgentPlanAction::NodePropertiesStream);
        assert_eq!(plan.moment, GraphProjectionMoment::Eval);
        assert!(!plan.task_daemon_mediated);
    }

    #[test]
    fn classifies_generate_graph_as_factory_task_daemon() {
        let plan = GraphAgentPlanInterface::classify("graph_store_catalog", "generateGraph")
            .expect("graph_store_catalog.generateGraph should classify");

        assert_eq!(plan.action, GraphAgentPlanAction::GraphGenerate);
        assert_eq!(plan.moment, GraphProjectionMoment::Factory);
        assert!(plan.task_daemon_mediated);
    }

    #[test]
    fn rejects_unknown_operation() {
        assert!(GraphAgentPlanInterface::classify("graph_store_catalog", "unknown").is_none());
    }

    #[test]
    fn graph_action_lowers_to_store_component_alias_and_mode() {
        let action = GraphAgentPlanAction::NodePropertiesWrite;
        let expr = action.to_graph_procedure_expr();

        assert_eq!(action.shell_component_alias(), "write_node_properties");
        assert_eq!(action.shell_mode(), crate::shell::ShellComponentMode::Write);
        assert_eq!(expr.component(), "write_node_properties");
        assert_eq!(expr.mode(), crate::shell::ShellComponentMode::Write);
    }
}
