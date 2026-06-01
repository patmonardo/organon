//! Model-stage mediation (first rational moment of Dialectical Learning).
//!
//! Model mediation is not a generic mediator wrapper. It is the conditioning
//! moment: schema-bearing form grounds and seeds Feature articulation without
//! yet performing the middle transformation or Plan-level logic manifestation.

use crate::collections::dataset::feature::mediator::{
    DialecticalBinding, DialecticalBindingRelation, FeatureId, MediatorId,
};
use crate::collections::dataset::model::core::ModelId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ModelBindingNature {
    ConditionsFeature,
    GroundsFeature,
}

impl ModelBindingNature {
    pub fn relation(self) -> DialecticalBindingRelation {
        match self {
            Self::ConditionsFeature => DialecticalBindingRelation::Conditions,
            Self::GroundsFeature => DialecticalBindingRelation::Grounds,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModelFeatureBinding {
    pub binding: DialecticalBinding,
    pub nature: ModelBindingNature,
}

impl ModelFeatureBinding {
    pub fn new(model_id: MediatorId, feature_id: MediatorId, nature: ModelBindingNature) -> Self {
        Self {
            binding: DialecticalBinding::new(model_id, feature_id, nature.relation()),
            nature,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelMediationMoment {
    pub model_id: ModelId,
    pub schema_anchor: Option<String>,
    pub bindings: Vec<DialecticalBinding>,
    pub feature_bindings: Vec<ModelFeatureBinding>,
}

impl ModelMediationMoment {
    pub fn new(model_id: impl Into<String>) -> Self {
        Self {
            model_id: ModelId(model_id.into()),
            schema_anchor: None,
            bindings: Vec::new(),
            feature_bindings: Vec::new(),
        }
    }

    pub fn with_schema_anchor(mut self, schema_anchor: impl Into<String>) -> Self {
        self.schema_anchor = Some(schema_anchor.into());
        self
    }

    pub fn mediator_id(&self) -> MediatorId {
        MediatorId::model(self.model_id.0.clone())
    }

    pub fn condition_feature(mut self, feature_id: impl Into<FeatureId>) -> Self {
        let feature_id = feature_id.into();
        let binding = ModelFeatureBinding::new(
            self.mediator_id(),
            MediatorId::feature(feature_id.0),
            ModelBindingNature::ConditionsFeature,
        );
        self.bindings.push(binding.binding.clone());
        self.feature_bindings.push(binding);
        self
    }

    pub fn ground_feature(mut self, feature_id: impl Into<FeatureId>) -> Self {
        let feature_id = feature_id.into();
        let binding = ModelFeatureBinding::new(
            self.mediator_id(),
            MediatorId::feature(feature_id.0),
            ModelBindingNature::GroundsFeature,
        );
        self.bindings.push(binding.binding.clone());
        self.feature_bindings.push(binding);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn model_mediation_conditions_feature() {
        let mediation = ModelMediationMoment::new("model:syntax")
            .with_schema_anchor("schema:agreement")
            .condition_feature("feature:pos");

        assert_eq!(mediation.bindings.len(), 1);
        assert_eq!(
            mediation.bindings[0].relation,
            DialecticalBindingRelation::Conditions
        );
        assert_eq!(
            mediation.feature_bindings[0].nature,
            ModelBindingNature::ConditionsFeature
        );
        assert_eq!(
            mediation.feature_bindings[0].binding.from.kind().as_str(),
            "model"
        );
        assert_eq!(
            mediation.feature_bindings[0].binding.to.kind().as_str(),
            "feature"
        );
    }

    #[test]
    fn model_mediation_keeps_grounding_distinct_from_conditioning() {
        let mediation = ModelMediationMoment::new("model:syntax")
            .with_schema_anchor("schema:agreement")
            .ground_feature("feature:agreement");

        assert_eq!(
            mediation.bindings[0].relation,
            DialecticalBindingRelation::Grounds
        );
        assert_eq!(
            mediation.feature_bindings[0].nature,
            ModelBindingNature::GroundsFeature
        );
    }
}
