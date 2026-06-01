//! Plan-stage mediation (third rational moment of Dialectical Learning).
//!
//! Plan mediation is bifacial. Vertically, it synthesizes Feature mediation into
//! a principle-bearing Plan. Horizontally, it manifests that Plan into Logic as
//! rule, entailment, proof trace, validity, or transformation.

use crate::collections::dataset::feature::mediator::{
    ArtifactKind, DialecticalBinding, DialecticalBindingRelation, FeatureId, ManifestBindingRecord,
    MediatorId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PlanBindingAxis {
    VerticalFeature,
    HorizontalLogic,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlanFeatureBinding {
    pub binding: DialecticalBinding,
    pub axis: PlanBindingAxis,
}

impl PlanFeatureBinding {
    pub fn synthesize(feature_id: MediatorId, plan_id: MediatorId) -> Self {
        Self {
            binding: DialecticalBinding::new(
                feature_id,
                plan_id,
                DialecticalBindingRelation::Synthesizes,
            ),
            axis: PlanBindingAxis::VerticalFeature,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlanLogicBinding {
    pub plan_id: MediatorId,
    pub record: ManifestBindingRecord,
    pub axis: PlanBindingAxis,
}

impl PlanLogicBinding {
    pub fn new(
        plan_id: MediatorId,
        logic_artifact_id: impl Into<String>,
        relation: DialecticalBindingRelation,
    ) -> Self {
        Self {
            plan_id,
            record: ManifestBindingRecord::new(ArtifactKind::Logic, logic_artifact_id, relation),
            axis: PlanBindingAxis::HorizontalLogic,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanMediationMoment {
    pub plan_id: String,
    pub principle_anchor: Option<String>,
    pub bindings: Vec<DialecticalBinding>,
    pub feature_bindings: Vec<PlanFeatureBinding>,
    pub logic_bindings: Vec<PlanLogicBinding>,
}

impl PlanMediationMoment {
    pub fn new(plan_id: impl Into<String>) -> Self {
        Self {
            plan_id: plan_id.into(),
            principle_anchor: None,
            bindings: Vec::new(),
            feature_bindings: Vec::new(),
            logic_bindings: Vec::new(),
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
        let binding =
            PlanFeatureBinding::synthesize(MediatorId::feature(feature_id.0), self.mediator_id());
        self.bindings.push(binding.binding.clone());
        self.feature_bindings.push(binding);
        self
    }

    pub fn manifest_logic(
        mut self,
        logic_artifact_id: impl Into<String>,
        relation: DialecticalBindingRelation,
    ) -> Self {
        let binding = PlanLogicBinding::new(self.mediator_id(), logic_artifact_id, relation);
        self.logic_bindings.push(binding);
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
        assert_eq!(
            mediation.bindings[0].relation,
            DialecticalBindingRelation::Synthesizes
        );
        assert_eq!(
            mediation.feature_bindings[0].axis,
            PlanBindingAxis::VerticalFeature
        );
    }

    #[test]
    fn plan_mediation_binds_vertically_to_feature_and_horizontally_to_logic() {
        let mediation = PlanMediationMoment::new("plan:logic")
            .with_principle_anchor("principle:model-feature-plan")
            .synthesize_feature("feature:agreement")
            .manifest_logic("logic:rule:agreement", DialecticalBindingRelation::Grounds);

        assert_eq!(mediation.feature_bindings.len(), 1);
        assert_eq!(mediation.logic_bindings.len(), 1);
        assert_eq!(
            mediation.feature_bindings[0].axis,
            PlanBindingAxis::VerticalFeature
        );
        assert_eq!(
            mediation.logic_bindings[0].axis,
            PlanBindingAxis::HorizontalLogic
        );
        assert_eq!(
            mediation.logic_bindings[0].record.artifact_kind,
            ArtifactKind::Logic
        );
        assert_eq!(
            mediation.logic_bindings[0].record.relation,
            DialecticalBindingRelation::Grounds
        );
    }
}
