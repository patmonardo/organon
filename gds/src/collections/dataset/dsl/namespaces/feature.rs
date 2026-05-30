//! Feature namespace for dataset-level DSL.
//!
//! This is the high-level Feature authoring surface. It keeps scripts close to
//! named features, slots, rules, and spaces instead of exposing the full
//! structural matching vocabulary as the primary interface.

use crate::collections::dataset::dsl::expressions::feature::{
    FeatureExpr, FeatureRef, FeatureRuleExpr, FeatureSlot,
};
use crate::collections::dataset::feature::Feature;
use crate::collections::dataset::feature::FeatureSpace;
use crate::collections::dataset::feature::FeatureView;
use crate::collections::dataset::plan::Plan;
use crate::collections::dataset::plan::PlanError;

#[derive(Debug, Clone, Default)]
pub struct FeatureNs;

impl FeatureNs {
    pub fn from_plan(plan: Plan) -> Feature {
        Feature::new(plan)
    }

    pub fn requiring_item(plan: Plan) -> Result<Feature, PlanError> {
        Feature::requiring_item(plan)
    }

    pub fn view(feature: impl Into<String>, view: impl Into<String>) -> FeatureView {
        FeatureView::new(feature, view)
    }

    pub fn space() -> FeatureSpace {
        FeatureSpace::new()
    }

    pub fn insert(space: FeatureSpace, name: impl Into<String>, feature: Feature) -> FeatureSpace {
        space.insert(name, feature)
    }

    pub fn feature(name: impl Into<String>) -> FeatureRef {
        FeatureRef::new(name)
    }

    pub fn slot(feature: impl Into<String>) -> FeatureSlot {
        FeatureSlot::new(feature)
    }

    pub fn slot_view(feature: impl Into<String>, view: impl Into<String>) -> FeatureSlot {
        FeatureSlot::new(feature).viewed(view)
    }

    pub fn rule_expr(name: impl Into<String>) -> FeatureRuleExpr {
        FeatureRuleExpr::new(name)
    }
}

#[derive(Debug, Clone, Default)]
pub struct FeatureExprNs;

impl FeatureExprNs {
    pub fn new() -> Self {
        Self
    }

    pub fn feature(name: impl Into<String>) -> FeatureExpr {
        FeatureExpr::Ref(FeatureRef::new(name))
    }

    pub fn slot(feature: impl Into<String>) -> FeatureExpr {
        FeatureExpr::Slot(FeatureSlot::new(feature))
    }

    pub fn slot_view(feature: impl Into<String>, view: impl Into<String>) -> FeatureExpr {
        FeatureExpr::Slot(FeatureSlot::new(feature).viewed(view))
    }

    pub fn rule(rule: FeatureRuleExpr) -> FeatureExpr {
        FeatureExpr::Rule(rule)
    }

    pub fn mark(mark: impl Into<String>) -> FeatureExpr {
        FeatureExpr::Mark(mark.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::collections::dataframe::col;
    use crate::collections::dataset::plan::Plan;

    #[test]
    fn test_feature_ns_builds_feature_space_and_exprs() {
        let plan = Plan::from_var("ds").project_item(col("text").alias("item"));
        let feature = FeatureNs::requiring_item(plan).expect("item feature should be accepted");
        let space = FeatureNs::insert(FeatureNs::space(), "text_item", feature);

        assert_eq!(space.len(), 1);

        let expr = FeatureExprNs::slot_view("lemma", "token");
        assert!(matches!(expr, FeatureExpr::Slot(_)));
    }
}
