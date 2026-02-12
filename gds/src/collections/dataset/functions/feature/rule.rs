//! Feature rule executor for tagged token sequences.

use crate::collections::dataset::expressions::feature::{
    FeatureCondition, FeaturePath, FeatureRule, FeatureSpec, FeatureValue,
};
use crate::collections::dataset::functions::feature::token::{extract_at, TaggedToken};

pub fn apply_rule(rule: &FeatureRule, tokens: &mut [TaggedToken]) -> Vec<usize> {
    apply_rule_at(rule, tokens, None)
}

pub fn apply_rule_at(
    rule: &FeatureRule,
    tokens: &mut [TaggedToken],
    positions: Option<&[usize]>,
) -> Vec<usize> {
    let indices: Vec<usize> = match positions {
        Some(list) => list.to_vec(),
        None => (0..tokens.len()).collect(),
    };

    let mut changed = Vec::new();
    for index in indices {
        if index >= tokens.len() {
            continue;
        }
        if rule_applies(rule, tokens, index) {
            let replacement = rule.replacement().to_string();
            tokens[index].1 = replacement;
            changed.push(index);
        }
    }

    changed
}

pub fn rule_applies(rule: &FeatureRule, tokens: &[TaggedToken], index: usize) -> bool {
    if tokens.get(index).map(|t| t.1.as_str()) != Some(rule.original()) {
        return false;
    }

    for condition in rule.conditions() {
        if !condition_holds(condition, tokens, index) {
            return false;
        }
    }

    true
}

fn condition_holds(condition: &FeatureCondition, tokens: &[TaggedToken], index: usize) -> bool {
    let spec = condition.feature();
    match spec.path() {
        FeaturePath::Offsets(positions) => positions
            .positions()
            .iter()
            .any(|offset| match_offset(spec, condition.value(), tokens, index, *offset)),
        FeaturePath::Tree(_) => false,
    }
}

fn match_offset(
    spec: &FeatureSpec,
    value: &FeatureValue,
    tokens: &[TaggedToken],
    index: usize,
    offset: i32,
) -> bool {
    let target = index as i32 + offset;
    if target < 0 {
        return false;
    }
    let target = target as usize;
    if target >= tokens.len() {
        return false;
    }

    extract_at(spec.property(), tokens, target)
        .map(|candidate| &candidate == value)
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::dataset::expressions::feature::{
        FeatureCondition, FeaturePosition, FeatureSpec,
    };

    #[test]
    fn apply_rule_changes_tag() {
        let spec = FeatureSpec::new("word", FeaturePosition::new(vec![0]).unwrap());
        let cond = FeatureCondition::new(spec, FeatureValue::text("Mary"));
        let rule = FeatureRule::new("NN", "NNP", vec![cond]);

        let mut tokens = vec![
            ("Mary".to_string(), "NN".to_string()),
            ("walks".to_string(), "VBZ".to_string()),
        ];

        let changed = apply_rule(&rule, &mut tokens);
        assert_eq!(changed, vec![0]);
        assert_eq!(tokens[0].1, "NNP".to_string());
    }
}
