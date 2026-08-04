//! GraphModel namespace for GraphFrame DSL.

use crate::collections::graphframe::expr::GraphFeatureGrammarExpr;
use crate::collections::graphframe::expr::GraphModelExpr;
use crate::collections::graphframe::frame::GraphFrame;
use crate::collections::graphframe::lazy::GraphFramePlan;

#[derive(Clone)]
pub struct GraphModelNameSpace {
    plan: GraphFramePlan,
}

impl GraphModelNameSpace {
    pub fn new(plan: GraphFramePlan) -> Self {
        Self { plan }
    }

    pub fn model(self, model_id: impl Into<String>) -> Self {
        Self {
            plan: self.plan.push_expr(GraphModelExpr::new(model_id)),
        }
    }

    pub fn grammar(self, grammar_name: impl Into<String>) -> Self {
        Self {
            plan: self
                .plan
                .push_expr(GraphFeatureGrammarExpr::new(grammar_name)),
        }
    }

    pub fn grammar_with_version(
        self,
        grammar_name: impl Into<String>,
        grammar_version: impl Into<String>,
    ) -> Self {
        Self {
            plan: self.plan.push_expr(
                GraphFeatureGrammarExpr::new(grammar_name).with_version(grammar_version),
            ),
        }
    }

    pub fn into_plan(self) -> GraphFramePlan {
        self.plan
    }
}

pub trait GraphFrameModelExt {
    fn gm(self) -> GraphModelNameSpace;
}

impl GraphFrameModelExt for GraphFrame {
    fn gm(self) -> GraphModelNameSpace {
        GraphModelNameSpace::new(self.plan())
    }
}

impl GraphFrameModelExt for GraphFramePlan {
    fn gm(self) -> GraphModelNameSpace {
        GraphModelNameSpace::new(self)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::collections::graphframe::expr::GraphFrameExpr;
    use crate::collections::graphframe::frame::GraphFrame;
    use crate::types::graph_store::DefaultGraphStore;
    use crate::types::random::RandomGraphConfig;

    use super::GraphFrameModelExt;

    fn random_store() -> Arc<DefaultGraphStore> {
        Arc::new(
            DefaultGraphStore::random(&RandomGraphConfig::seeded(21))
                .expect("seeded random graph store should build"),
        )
    }

    #[test]
    fn graph_model_namespace_emits_model_and_grammar_expressions() {
        let frame = GraphFrame::from_store(random_store()).expect("frame should build");
        let plan = frame
            .gm()
            .model("citation_model_v1")
            .grammar_with_version("citation_graph", "v1")
            .into_plan();

        assert_eq!(plan.expressions().len(), 2);
        assert!(matches!(plan.expressions()[0], GraphFrameExpr::Model(_)));
        assert!(matches!(
            plan.expressions()[1],
            GraphFrameExpr::FeatureGrammar(_)
        ));
    }
}
