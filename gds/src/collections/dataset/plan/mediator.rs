//! Plan-stage mediation (third rational moment).
//!
//! Plan mediation is synthesis-forward: it gathers feature bindings into a
//! logic-bearing plan relation.

use crate::collections::dataset::feature::mediator::{
    DialecticalBinding, DialecticalBindingRelation, FeatureId, MediatorId,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanMediationMoment {
    pub plan_id: String,
    pub principle_anchor: Option<String>,
    pub bindings: Vec<DialecticalBinding>,
}

impl PlanMediationMoment {
    pub fn new(plan_id: impl Into<String>) -> Self {
        Self {
            plan_id: plan_id.into(),
            principle_anchor: None,
            bindings: Vec::new(),
        }
    }

    pub fn with_principle_anchor(mut self, principle_anchor: impl Into<String>) -> Self {
        self.principle_anchor = Some(principle_anchor.into());
        self
    }

    pub fn mediator_id(&self) -> MediatorId {
        MediatorId::plan(self.plan_id.clone())
    }

    pub fn synthesize_feature(mut self, feature_id: impl Into<FeatureId>) -> Self {
        let feature_id = feature_id.into();
        self.bindings.push(DialecticalBinding::new(
            MediatorId::feature(feature_id.0),
            self.mediator_id(),
            DialecticalBindingRelation::Synthesizes,
        ));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plan_mediation_synthesizes_feature() {
        let mediation = PlanMediationMoment::new("plan:logic")
            .with_principle_anchor("principle:model-feature-plan")
            .synthesize_feature("feature:agreement");

        assert_eq!(mediation.bindings.len(), 1);
        assert_eq!(mediation.bindings[0].relation, DialecticalBindingRelation::Synthesizes);
    }
}
