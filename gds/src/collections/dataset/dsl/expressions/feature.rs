//! Feature expressions for dataset-level DSL automation.
//!
//! This module defines expression data only. Namespaces and functions may make
//! these expressions convenient to construct, but this module should not become
//! a factory surface or dialectical Feature machinery.

use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FeatureRef {
    name: String,
}

impl FeatureRef {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureSlot {
    feature: FeatureRef,
    view: Option<String>,
}

impl FeatureSlot {
    pub fn new(feature: impl Into<String>) -> Self {
        Self {
            feature: FeatureRef::new(feature),
            view: None,
        }
    }

    pub fn viewed(mut self, view: impl Into<String>) -> Self {
        self.view = Some(view.into());
        self
    }

    pub fn feature(&self) -> &FeatureRef {
        &self.feature
    }

    pub fn view(&self) -> Option<&str> {
        self.view.as_deref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureRuleExpr {
    name: String,
    when: BTreeMap<String, String>,
    then: BTreeMap<String, String>,
}

impl FeatureRuleExpr {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            when: BTreeMap::new(),
            then: BTreeMap::new(),
        }
    }

    pub fn when(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.when.insert(key.into(), value.into());
        self
    }

    pub fn then(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.then.insert(key.into(), value.into());
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn conditions(&self) -> &BTreeMap<String, String> {
        &self.when
    }

    pub fn outputs(&self) -> &BTreeMap<String, String> {
        &self.then
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FeatureExpr {
    Ref(FeatureRef),
    Slot(FeatureSlot),
    Rule(FeatureRuleExpr),
    Mark(String),
}

impl FeatureExpr {
    pub fn as_ref_expr(&self) -> Option<&FeatureRef> {
        if let FeatureExpr::Ref(feature) = self {
            Some(feature)
        } else {
            None
        }
    }

    pub fn as_slot(&self) -> Option<&FeatureSlot> {
        if let FeatureExpr::Slot(slot) = self {
            Some(slot)
        } else {
            None
        }
    }

    pub fn as_rule(&self) -> Option<&FeatureRuleExpr> {
        if let FeatureExpr::Rule(rule) = self {
            Some(rule)
        } else {
            None
        }
    }

    pub fn as_mark(&self) -> Option<&str> {
        if let FeatureExpr::Mark(mark) = self {
            Some(mark)
        } else {
            None
        }
    }
}

impl From<FeatureRef> for FeatureExpr {
    fn from(value: FeatureRef) -> Self {
        Self::Ref(value)
    }
}

impl From<FeatureSlot> for FeatureExpr {
    fn from(value: FeatureSlot) -> Self {
        Self::Slot(value)
    }
}

impl From<FeatureRuleExpr> for FeatureExpr {
    fn from(value: FeatureRuleExpr) -> Self {
        Self::Rule(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn feature_slot_names_view_without_core_objects() {
        let slot = FeatureSlot::new("lemma").viewed("token");

        assert_eq!(slot.feature().name(), "lemma");
        assert_eq!(slot.view(), Some("token"));
    }

    #[test]
    fn feature_rule_expr_is_script_facing() {
        let rule = FeatureRuleExpr::new("proper-noun")
            .when("pos", "NN")
            .then("pos", "NNP");

        assert_eq!(rule.name(), "proper-noun");
        assert_eq!(rule.conditions().get("pos"), Some(&"NN".to_string()));
        assert_eq!(rule.outputs().get("pos"), Some(&"NNP".to_string()));
    }

    #[test]
    fn feature_expr_accessors_keep_module_expression_only() {
        let expr = FeatureExpr::Slot(FeatureSlot::new("lemma"));
        assert!(expr.as_slot().is_some());
        assert!(expr.as_rule().is_none());
    }
}
