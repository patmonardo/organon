//! GraphPlan namespace for GraphFrame DSL.

use crate::collections::graphframe::expr::GraphPlanExpr;
use crate::collections::graphframe::frame::GraphFrame;
use crate::collections::graphframe::lazy::GraphFramePlan;

#[derive(Clone)]
pub struct GraphPlanNameSpace {
    plan: GraphFramePlan,
}

impl GraphPlanNameSpace {
    pub fn new(plan: GraphFramePlan) -> Self {
        Self { plan }
    }

    pub fn id(self, plan_id: impl Into<String>) -> Self {
        Self {
            plan: self.plan.push_expr(GraphPlanExpr::new(plan_id)),
        }
    }

    pub fn into_plan(self) -> GraphFramePlan {
        self.plan
    }
}

pub trait GraphFramePlanExt {
    fn gp(self) -> GraphPlanNameSpace;
}

impl GraphFramePlanExt for GraphFrame {
    fn gp(self) -> GraphPlanNameSpace {
        GraphPlanNameSpace::new(self.plan())
    }
}

impl GraphFramePlanExt for GraphFramePlan {
    fn gp(self) -> GraphPlanNameSpace {
        GraphPlanNameSpace::new(self)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::collections::graphframe::expr::GraphFrameExpr;
    use crate::collections::graphframe::frame::GraphFrame;
    use crate::types::graph_store::DefaultGraphStore;
    use crate::types::random::RandomGraphConfig;

    use super::GraphFramePlanExt;

    fn random_store() -> Arc<DefaultGraphStore> {
        Arc::new(
            DefaultGraphStore::random(&RandomGraphConfig::seeded(21))
                .expect("seeded random graph store should build"),
        )
    }

    #[test]
    fn graph_plan_namespace_emits_plan_expression() {
        let frame = GraphFrame::from_store(random_store()).expect("frame should build");
        let plan = frame.gp().id("pagerank_plan_v1").into_plan();

        assert_eq!(plan.expressions().len(), 1);
        assert!(matches!(plan.expressions()[0], GraphFrameExpr::Plan(_)));
    }
}
