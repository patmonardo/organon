//! Feature functionals for dataset-level DSL.
//!
//! Expansion and combinator helpers for feature specs and templates.

pub mod rule;
pub mod token;

use crate::collections::dataset::expressions::feature::{
    FeaturePath, FeaturePosition, FeatureSpec, FeatureTemplate, FeatureValue,
};
use crate::collections::dataset::expressions::tree::TreePos;
use crate::collections::dataset::plan::PlanError;
use crate::collections::dataset::tree::{TreeLeafValue, TreeValue};

pub fn expand_specs(
    property: impl Into<String>,
    starts: &[i32],
    winlens: &[i32],
    exclude_zero: bool,
) -> Result<Vec<FeatureSpec>, PlanError> {
    if winlens.iter().any(|w| *w <= 0) {
        return Err(PlanError::Message(format!(
            "non-positive window length in {winlens:?}"
        )));
    }

    let property = property.into();
    let mut out = Vec::new();
    for window in winlens {
        let window = *window as usize;
        if window == 0 || starts.len() < window {
            continue;
        }
        for i in 0..=starts.len() - window {
            let slice = &starts[i..i + window];
            let positions = FeaturePosition::new(slice.to_vec())?;
            if exclude_zero && positions.contains_zero() {
                continue;
            }
            out.push(FeatureSpec::new(property.clone(), positions));
        }
    }
    Ok(out)
}

pub fn expand_templates(
    feature_lists: &[Vec<FeatureSpec>],
    combinations: Option<(usize, usize)>,
    skip_intersecting: bool,
) -> Vec<FeatureTemplate> {
    let mut out = Vec::new();
    if feature_lists.is_empty() {
        return out;
    }

    let (min_k, max_k) = combinations.unwrap_or((1, feature_lists.len()));
    for k in min_k..=max_k {
        for list_indices in choose_indices(feature_lists.len(), k) {
            let picks = list_indices
                .iter()
                .map(|idx| &feature_lists[*idx])
                .collect::<Vec<_>>();
            let mut current = Vec::new();
            expand_cartesian(&picks, 0, &mut current, skip_intersecting, &mut out);
        }
    }

    out
}

pub fn extract_tree_feature(spec: &FeatureSpec, tree: &TreeValue) -> Option<FeatureValue> {
    let path = spec.path();
    if let FeaturePath::Tree(pos) = path {
        tree_value_at(tree, pos).and_then(feature_value_from_tree_value)
    } else {
        None
    }
}

pub fn tree_value_at<'a>(tree: &'a TreeValue, pos: &TreePos) -> Option<&'a TreeValue> {
    let mut current = tree;
    for idx in pos.path() {
        match current {
            TreeValue::Node(node) => {
                current = node.children().get(*idx)?;
            }
            TreeValue::Leaf(_) => return None,
        }
    }
    Some(current)
}

pub fn feature_value_from_tree_value(value: &TreeValue) -> Option<FeatureValue> {
    match value {
        TreeValue::Leaf(leaf) => feature_value_from_leaf(leaf),
        TreeValue::Node(node) => Some(FeatureValue::text(node.label().to_string())),
    }
}

fn feature_value_from_leaf(leaf: &TreeLeafValue) -> Option<FeatureValue> {
    match leaf {
        TreeLeafValue::Text(value) => Some(FeatureValue::text(value.clone())),
        TreeLeafValue::TokenIndex(index) => {
            i32::try_from(*index).ok().map(FeatureValue::token_index)
        }
        TreeLeafValue::Number(value) => Some(FeatureValue::Number(*value)),
        TreeLeafValue::Bool(value) => Some(FeatureValue::Bool(*value)),
        TreeLeafValue::BytesRange { start, end } => Some(FeatureValue::BytesRange {
            start: *start,
            end: *end,
        }),
        TreeLeafValue::Empty => Some(FeatureValue::Empty),
    }
}

fn choose_indices(n: usize, k: usize) -> Vec<Vec<usize>> {
    let mut out = Vec::new();
    let mut current = Vec::with_capacity(k);
    choose_indices_inner(n, k, 0, &mut current, &mut out);
    out
}

fn choose_indices_inner(
    n: usize,
    k: usize,
    start: usize,
    current: &mut Vec<usize>,
    out: &mut Vec<Vec<usize>>,
) {
    if current.len() == k {
        out.push(current.clone());
        return;
    }
    for i in start..n {
        current.push(i);
        choose_indices_inner(n, k, i + 1, current, out);
        current.pop();
    }
}

fn expand_cartesian(
    lists: &[&Vec<FeatureSpec>],
    index: usize,
    current: &mut Vec<FeatureSpec>,
    skip_intersecting: bool,
    out: &mut Vec<FeatureTemplate>,
) {
    if index == lists.len() {
        let mut sorted = current.clone();
        sorted.sort();
        if has_superset_pair(&sorted) {
            return;
        }
        if skip_intersecting && has_intersections(&sorted) {
            return;
        }
        out.push(FeatureTemplate::new(sorted));
        return;
    }

    for feature in lists[index] {
        current.push(feature.clone());
        expand_cartesian(lists, index + 1, current, skip_intersecting, out);
        current.pop();
    }
}

fn has_superset_pair(features: &[FeatureSpec]) -> bool {
    for (i, x) in features.iter().enumerate() {
        for (j, y) in features.iter().enumerate() {
            if i != j && x.issuperset(y) {
                return true;
            }
        }
    }
    false
}

fn has_intersections(features: &[FeatureSpec]) -> bool {
    for (i, x) in features.iter().enumerate() {
        for (j, y) in features.iter().enumerate() {
            if i != j && x.intersects(y) {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expand_specs_excludes_zero() {
        let specs = expand_specs("pos", &[-2, -1, 0], &[1, 2], true).unwrap();
        assert!(specs
            .iter()
            .all(|spec| !spec.positions().unwrap().contains_zero()));
    }

    #[test]
    fn expand_templates_skips_intersections() {
        let f1 = FeatureSpec::new("word", FeaturePosition::new(vec![0]).unwrap());
        let f2 = FeatureSpec::new("word", FeaturePosition::new(vec![1]).unwrap());
        let f3 = FeatureSpec::new("word", FeaturePosition::new(vec![0, 1]).unwrap());

        let templates = expand_templates(&[vec![f1, f3], vec![f2]], Some((2, 2)), true);
        assert!(templates.iter().all(|t| t.features().len() == 2));
        assert!(templates.iter().all(|t| {
            let a = &t.features()[0];
            let b = &t.features()[1];
            !(a.intersects(b))
        }));
    }

    #[test]
    fn extract_tree_feature_reads_leaf() {
        let tree = TreeValue::node("S", vec![TreeValue::leaf(TreeLeafValue::text("Mary"))]);
        let pos = crate::collections::dataset::tree::TreePos::new(vec![0]);
        let spec = FeatureSpec::new_tree("word", pos);
        let value = extract_tree_feature(&spec, &tree).unwrap();

        assert_eq!(value, FeatureValue::Text("Mary".to_string()));
    }

    #[test]
    fn extract_tree_feature_reads_node_label() {
        let tree = TreeValue::node("S", vec![TreeValue::node("NP", vec![TreeValue::leaf("x")])]);
        let pos = crate::collections::dataset::tree::TreePos::new(vec![0]);
        let spec = FeatureSpec::new_tree("label", pos);
        let value = extract_tree_feature(&spec, &tree).unwrap();

        assert_eq!(value, FeatureValue::Text("NP".to_string()));
    }
}
