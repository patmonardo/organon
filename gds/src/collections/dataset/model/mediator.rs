//! Model-stage mediation (first rational moment).
//!
//! Model mediation is schema-forward: it conditions and seeds feature
//! articulation. This module defines the first moment of the
//! Model -> Feature -> Plan mediation arc.

use crate::collections::dataset::feature::mediator::{
    DialecticalBinding, DialecticalBindingRelation, FeatureId, MediatorId,
};
use crate::collections::dataset::model::core::ModelId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelMediationMoment {
    pub model_id: ModelId,
    pub schema_anchor: Option<String>,
    pub bindings: Vec<DialecticalBinding>,
}

impl ModelMediationMoment {
    pub fn new(model_id: impl Into<String>) -> Self {
        Self {
            model_id: ModelId(model_id.into()),
            schema_anchor: None,
            bindings: Vec::new(),
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
        self.bindings.push(DialecticalBinding::new(
            self.mediator_id(),
            MediatorId::feature(feature_id.0),
            DialecticalBindingRelation::Conditions,
        ));
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
    }
}
