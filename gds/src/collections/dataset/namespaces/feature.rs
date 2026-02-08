//! Feature namespace for dataset-level DSL.
//!
//! Builders here are thin constructors for Feature expressions and
//! delegate to feature functionals when expansion is requested.

use crate::collections::dataset::expressions::feature::{
    FeatureCondition, FeatureExpr, FeaturePath, FeaturePosition, FeatureRule, FeatureSpec,
    FeatureTemplate, FeatureValue,
};
use crate::collections::dataset::expressions::tree::TreePos;
use crate::collections::dataset::functions::feature as feature_fn;
use crate::collections::dataset::plan::PlanError;

#[derive(Debug, Clone, Default)]
pub struct FeatureNs;

impl FeatureNs {
    pub fn pos(positions: impl Into<Vec<i32>>) -> Result<FeaturePosition, PlanError> {
        FeaturePosition::new(positions)
    }

    pub fn range(start: i32, end: i32) -> Result<FeaturePosition, PlanError> {
        FeaturePosition::from_range(start, end)
    }

    pub fn path_offsets(positions: FeaturePosition) -> FeaturePath {
        FeaturePath::offsets(positions)
    }

    pub fn path_tree(pos: TreePos) -> FeaturePath {
        FeaturePath::tree(pos)
    }

    pub fn spec(property: impl Into<String>, positions: FeaturePosition) -> FeatureSpec {
        FeatureSpec::new(property, positions)
    }

    pub fn spec_tree(property: impl Into<String>, pos: TreePos) -> FeatureSpec {
        FeatureSpec::new_tree(property, pos)
    }

    pub fn spec_path(property: impl Into<String>, path: FeaturePath) -> FeatureSpec {
        FeatureSpec::new_path(property, path)
    }

    pub fn condition(feature: FeatureSpec, value: impl Into<FeatureValue>) -> FeatureCondition {
        FeatureCondition::new(feature, value)
    }

    pub fn rule(
        original: impl Into<String>,
        replacement: impl Into<String>,
        conditions: Vec<FeatureCondition>,
    ) -> FeatureRule {
        FeatureRule::new(original, replacement, conditions)
    }

    pub fn template(features: Vec<FeatureSpec>) -> FeatureTemplate {
        FeatureTemplate::new(features)
    }

    pub fn expand_specs(
        property: impl Into<String>,
        starts: &[i32],
        winlens: &[i32],
        exclude_zero: bool,
    ) -> Result<Vec<FeatureSpec>, PlanError> {
        feature_fn::expand_specs(property, starts, winlens, exclude_zero)
    }

    pub fn expand_templates(
        feature_lists: &[Vec<FeatureSpec>],
        combinations: Option<(usize, usize)>,
        skip_intersecting: bool,
    ) -> Vec<FeatureTemplate> {
        feature_fn::expand_templates(feature_lists, combinations, skip_intersecting)
    }
}

#[derive(Debug, Clone, Default)]
pub struct FeatureExprNameSpace;

impl FeatureExprNameSpace {
    pub fn new() -> Self {
        Self
    }

    pub fn value(value: impl Into<FeatureValue>) -> FeatureExpr {
        FeatureExpr::Value(value.into())
    }

    pub fn position(positions: FeaturePosition) -> FeatureExpr {
        FeatureExpr::Position(positions)
    }

    pub fn path(path: FeaturePath) -> FeatureExpr {
        FeatureExpr::Path(path)
    }

    pub fn spec(spec: FeatureSpec) -> FeatureExpr {
        FeatureExpr::Spec(spec)
    }

    pub fn condition(condition: FeatureCondition) -> FeatureExpr {
        FeatureExpr::Condition(condition)
    }

    pub fn rule(rule: FeatureRule) -> FeatureExpr {
        FeatureExpr::Rule(rule)
    }

    pub fn template(template: FeatureTemplate) -> FeatureExpr {
        FeatureExpr::Template(template)
    }
}
