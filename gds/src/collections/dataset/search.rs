use crate::collections::dataset::featstruct::{
    parse_featstruct, subsumes_featstruct, FeatStruct, FeatStructParseError,
};
use crate::collections::dataset::semantic::{SdlEdge, SdlNode, SdlSubgraph};

/// Represents a query constraint for a single node in the SDL graph.
#[derive(Debug, Clone)]
pub struct SdlQueryNode {
    /// The parsed Feature Structure constraint for this node.
    pub constraint: FeatStruct,
}

impl SdlQueryNode {
    pub fn new(constraint_dsl: &str) -> Result<Self, FeatStructParseError> {
        let constraint = parse_featstruct(constraint_dsl)?;
        Ok(Self { constraint })
    }

    /// Checks if this query node constraint subsumes (matches) the target SDL node.
    pub fn matches(&self, target: &SdlNode) -> bool {
        if let Some(target_features) = &target.features {
            subsumes_featstruct(&self.constraint, target_features)
        } else {
            false // No features on target to unify against
        }
    }
}

/// Represents a query constraint for a directed edge in the SDL graph.
#[derive(Debug, Clone)]
pub struct SdlQueryEdge {
    /// The structural relation name to pre-filter by (Topological Phase 1)
    pub relation: String,

    /// Optional Deep Semantic constraint for this edge features (Semantic Phase 2)
    pub constraint: Option<FeatStruct>,
}

impl SdlQueryEdge {
    pub fn new(
        relation: impl Into<String>,
        constraint_dsl: Option<&str>,
    ) -> Result<Self, FeatStructParseError> {
        let constraint = if let Some(dsl) = constraint_dsl {
            Some(parse_featstruct(dsl)?)
        } else {
            None
        };
        Ok(Self {
            relation: relation.into(),
            constraint,
        })
    }

    /// Checks if this query edge constraint subsumes (matches) the target SDL edge.
    pub fn matches(&self, target: &SdlEdge) -> bool {
        if target.relation != self.relation {
            return false;
        }

        if let Some(query_constraint) = &self.constraint {
            if let Some(target_features) = &target.features {
                subsumes_featstruct(query_constraint, target_features)
            } else {
                false
            }
        } else {
            true // No semantic constraint required beyond structural relation
        }
    }
}

/// Represents a full Subgraph search query over a dataset.
#[derive(Debug, Clone, Default)]
pub struct SdlSearchQuery {
    pub nodes: Vec<SdlQueryNode>,
    pub edges: Vec<SdlQueryEdge>,
}

impl SdlSearchQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_node(mut self, constraint_dsl: &str) -> Result<Self, FeatStructParseError> {
        self.nodes.push(SdlQueryNode::new(constraint_dsl)?);
        Ok(self)
    }

    pub fn with_edge(
        mut self,
        relation: impl Into<String>,
        constraint_dsl: Option<&str>,
    ) -> Result<Self, FeatStructParseError> {
        self.edges
            .push(SdlQueryEdge::new(relation, constraint_dsl)?);
        Ok(self)
    }

    /// Extractor: Attempts to find the SdlSearchQuery graph pattern inside the target SdlSubgraph.
    /// This represents Phase 2 of the Two-Phase Search (Semantic Unification).
    /// Returns true if a subgraph homomorphism is found where all semantic constraints unify.
    pub fn is_match(&self, target_subgraph: &SdlSubgraph) -> bool {
        // Quick topological exit: If the target doesn't have enough nodes/edges, it can't match.
        if target_subgraph.nodes.len() < self.nodes.len()
            || target_subgraph.edges.len() < self.edges.len()
        {
            return false;
        }

        // 1. Edge Topological Fast-Fail
        for query_edge in &self.edges {
            let has_relation = target_subgraph
                .edges
                .iter()
                .any(|tg_edge| tg_edge.relation == query_edge.relation);
            if !has_relation {
                return false;
            }
        }

        // 2. Node Semantic Unification
        // In a full Subgraph Isomorphism implementation, we would try multiple bindings.
        // For the minimal Universal Search, we assure that for every query node,
        // there exists AT LEAST ONE target node that unifies.
        for query_node in &self.nodes {
            let node_matches = target_subgraph
                .nodes
                .iter()
                .any(|tg_node| query_node.matches(tg_node));
            if !node_matches {
                return false;
            }
        }

        // 3. Edge Semantic Unification
        for query_edge in &self.edges {
            let edge_matches = target_subgraph
                .edges
                .iter()
                .any(|tg_edge| query_edge.matches(tg_edge));
            if !edge_matches {
                return false;
            }
        }

        true
    }
}
