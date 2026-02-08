//! Feature expressions for dataset-level DSL.
//!
//! These are declarative data structures that describe feature paths,
//! templates, and rule conditions (NLTK TBL inspired).

use crate::collections::dataset::expressions::tree::TreePos;
use crate::collections::dataset::plan::PlanError;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FeatureValue {
    Text(String),
    TokenIndex(i32),
    Number(i64),
    Bool(bool),
    BytesRange { start: usize, end: usize },
    Empty,
}

impl FeatureValue {
    pub fn text(value: impl Into<String>) -> Self {
        FeatureValue::Text(value.into())
    }

    pub fn token_index(index: i32) -> Self {
        FeatureValue::TokenIndex(index)
    }

    pub fn bytes_range(start: usize, end: usize) -> Self {
        FeatureValue::BytesRange { start, end }
    }
}

impl From<String> for FeatureValue {
    fn from(value: String) -> Self {
        FeatureValue::Text(value)
    }
}

impl From<&str> for FeatureValue {
    fn from(value: &str) -> Self {
        FeatureValue::Text(value.to_string())
    }
}

impl From<i32> for FeatureValue {
    fn from(value: i32) -> Self {
        FeatureValue::TokenIndex(value)
    }
}

impl From<i64> for FeatureValue {
    fn from(value: i64) -> Self {
        FeatureValue::Number(value)
    }
}

impl From<bool> for FeatureValue {
    fn from(value: bool) -> Self {
        FeatureValue::Bool(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FeaturePosition {
    positions: Vec<i32>,
}

impl FeaturePosition {
    pub fn new(positions: impl Into<Vec<i32>>) -> Result<Self, PlanError> {
        let mut positions = positions.into();
        positions.sort();
        positions.dedup();
        if positions.is_empty() {
            return Err(PlanError::Message(
                "feature positions cannot be empty".to_string(),
            ));
        }
        Ok(Self { positions })
    }

    pub fn from_range(start: i32, end: i32) -> Result<Self, PlanError> {
        if start > end {
            return Err(PlanError::Message(format!(
                "illegal interval specification: (start={start}, end={end})"
            )));
        }
        Ok(Self {
            positions: (start..=end).collect(),
        })
    }

    pub fn positions(&self) -> &[i32] {
        &self.positions
    }

    pub fn contains_zero(&self) -> bool {
        self.positions.iter().any(|p| *p == 0)
    }

    pub fn issuperset(&self, other: &FeaturePosition) -> bool {
        other.positions.iter().all(|p| self.positions.contains(p))
    }

    pub fn intersects(&self, other: &FeaturePosition) -> bool {
        other.positions.iter().any(|p| self.positions.contains(p))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FeaturePath {
    Offsets(FeaturePosition),
    Tree(TreePos),
}

impl FeaturePath {
    pub fn offsets(positions: FeaturePosition) -> Self {
        FeaturePath::Offsets(positions)
    }

    pub fn tree(pos: TreePos) -> Self {
        FeaturePath::Tree(pos)
    }

    pub fn as_offsets(&self) -> Option<&FeaturePosition> {
        if let FeaturePath::Offsets(positions) = self {
            Some(positions)
        } else {
            None
        }
    }

    pub fn as_tree(&self) -> Option<&TreePos> {
        if let FeaturePath::Tree(pos) = self {
            Some(pos)
        } else {
            None
        }
    }
}

impl From<FeaturePosition> for FeaturePath {
    fn from(positions: FeaturePosition) -> Self {
        FeaturePath::Offsets(positions)
    }
}

impl From<TreePos> for FeaturePath {
    fn from(pos: TreePos) -> Self {
        FeaturePath::Tree(pos)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FeatureSpec {
    property: String,
    path: FeaturePath,
}

impl FeatureSpec {
    pub fn new(property: impl Into<String>, positions: FeaturePosition) -> Self {
        Self {
            property: property.into(),
            path: FeaturePath::offsets(positions),
        }
    }

    pub fn new_tree(property: impl Into<String>, pos: TreePos) -> Self {
        Self {
            property: property.into(),
            path: FeaturePath::tree(pos),
        }
    }

    pub fn new_path(property: impl Into<String>, path: FeaturePath) -> Self {
        Self {
            property: property.into(),
            path,
        }
    }

    pub fn property(&self) -> &str {
        &self.property
    }

    pub fn path(&self) -> &FeaturePath {
        &self.path
    }

    pub fn positions(&self) -> Option<&FeaturePosition> {
        self.path.as_offsets()
    }

    pub fn tree_pos(&self) -> Option<&TreePos> {
        self.path.as_tree()
    }

    pub fn issuperset(&self, other: &FeatureSpec) -> bool {
        if self.property != other.property {
            return false;
        }
        match (&self.path, &other.path) {
            (FeaturePath::Offsets(a), FeaturePath::Offsets(b)) => a.issuperset(b),
            (FeaturePath::Tree(a), FeaturePath::Tree(b)) => a == b,
            _ => false,
        }
    }

    pub fn intersects(&self, other: &FeatureSpec) -> bool {
        if self.property != other.property {
            return false;
        }
        match (&self.path, &other.path) {
            (FeaturePath::Offsets(a), FeaturePath::Offsets(b)) => a.intersects(b),
            (FeaturePath::Tree(a), FeaturePath::Tree(b)) => a == b,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureCondition {
    feature: FeatureSpec,
    value: FeatureValue,
}

impl FeatureCondition {
    pub fn new(feature: FeatureSpec, value: impl Into<FeatureValue>) -> Self {
        Self {
            feature,
            value: value.into(),
        }
    }

    pub fn feature(&self) -> &FeatureSpec {
        &self.feature
    }

    pub fn value(&self) -> &FeatureValue {
        &self.value
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureRule {
    original: String,
    replacement: String,
    template_id: Option<String>,
    conditions: Vec<FeatureCondition>,
}

impl FeatureRule {
    pub fn new(
        original: impl Into<String>,
        replacement: impl Into<String>,
        conditions: Vec<FeatureCondition>,
    ) -> Self {
        Self {
            original: original.into(),
            replacement: replacement.into(),
            template_id: None,
            conditions,
        }
    }

    pub fn with_template_id(mut self, template_id: impl Into<String>) -> Self {
        self.template_id = Some(template_id.into());
        self
    }

    pub fn original(&self) -> &str {
        &self.original
    }

    pub fn replacement(&self) -> &str {
        &self.replacement
    }

    pub fn conditions(&self) -> &[FeatureCondition] {
        &self.conditions
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureTemplate {
    features: Vec<FeatureSpec>,
}

impl FeatureTemplate {
    pub fn new(features: Vec<FeatureSpec>) -> Self {
        Self { features }
    }

    pub fn features(&self) -> &[FeatureSpec] {
        &self.features
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FeatureExpr {
    Value(FeatureValue),
    Position(FeaturePosition),
    Path(FeaturePath),
    Spec(FeatureSpec),
    Condition(FeatureCondition),
    Rule(FeatureRule),
    Template(FeatureTemplate),
}

impl From<FeatureValue> for FeatureExpr {
    fn from(value: FeatureValue) -> Self {
        FeatureExpr::Value(value)
    }
}

impl From<FeaturePosition> for FeatureExpr {
    fn from(value: FeaturePosition) -> Self {
        FeatureExpr::Position(value)
    }
}

impl From<FeaturePath> for FeatureExpr {
    fn from(value: FeaturePath) -> Self {
        FeatureExpr::Path(value)
    }
}

impl From<FeatureSpec> for FeatureExpr {
    fn from(value: FeatureSpec) -> Self {
        FeatureExpr::Spec(value)
    }
}

impl From<FeatureCondition> for FeatureExpr {
    fn from(value: FeatureCondition) -> Self {
        FeatureExpr::Condition(value)
    }
}

impl From<FeatureRule> for FeatureExpr {
    fn from(value: FeatureRule) -> Self {
        FeatureExpr::Rule(value)
    }
}

impl From<FeatureTemplate> for FeatureExpr {
    fn from(value: FeatureTemplate) -> Self {
        FeatureExpr::Template(value)
    }
}
