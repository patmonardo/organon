use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use super::classify::Classifier;
use super::util::FeatureValue;

#[derive(Debug, Clone)]
pub struct DecisionTreeClassifier<L>
where
    L: Eq + Hash + Clone,
{
    label: L,
    feature_name: Option<String>,
    decisions: HashMap<String, DecisionTreeClassifier<L>>,
    default: Option<Box<DecisionTreeClassifier<L>>>,
}

impl<L> DecisionTreeClassifier<L>
where
    L: Eq + Hash + Clone,
{
    pub fn new(
        label: L,
        feature_name: Option<String>,
        decisions: HashMap<String, DecisionTreeClassifier<L>>,
        default: Option<DecisionTreeClassifier<L>>,
    ) -> Self {
        Self {
            label,
            feature_name,
            decisions,
            default: default.map(Box::new),
        }
    }

    pub fn labels(&self) -> Vec<L> {
        let mut labels = HashSet::new();
        self.collect_labels(&mut labels);
        labels.into_iter().collect()
    }

    fn collect_labels(&self, out: &mut HashSet<L>) {
        out.insert(self.label.clone());
        for subtree in self.decisions.values() {
            subtree.collect_labels(out);
        }
        if let Some(default) = &self.default {
            default.collect_labels(out);
        }
    }

    pub fn classify(&self, featureset: &HashMap<String, FeatureValue>) -> L {
        if self.feature_name.is_none() {
            return self.label.clone();
        }

        let fname = self.feature_name.as_ref().expect("checked above");
        let fval = featureset
            .get(fname)
            .map(encode_feature_value)
            .unwrap_or_else(|| "None".to_string());

        if let Some(subtree) = self.decisions.get(&fval) {
            subtree.classify(featureset)
        } else if let Some(default) = &self.default {
            default.classify(featureset)
        } else {
            self.label.clone()
        }
    }

    pub fn error(&self, labeled_featuresets: &[(HashMap<String, FeatureValue>, L)]) -> f64 {
        if labeled_featuresets.is_empty() {
            return 0.0;
        }

        let mut errors = 0usize;
        for (featureset, label) in labeled_featuresets {
            if self.classify(featureset) != *label {
                errors += 1;
            }
        }
        errors as f64 / labeled_featuresets.len() as f64
    }

    pub fn train(
        labeled_featuresets: &[(HashMap<String, FeatureValue>, L)],
        entropy_cutoff: f64,
        depth_cutoff: usize,
        support_cutoff: usize,
    ) -> Self {
        let mut feature_names = HashSet::new();
        for (featureset, _) in labeled_featuresets {
            for key in featureset.keys() {
                feature_names.insert(key.clone());
            }
        }

        let mut tree = Self::best_stump(&feature_names, labeled_featuresets);
        tree.refine(
            labeled_featuresets,
            entropy_cutoff,
            depth_cutoff.saturating_sub(1),
            support_cutoff,
            &feature_names,
        );
        tree
    }

    pub fn leaf(labeled_featuresets: &[(HashMap<String, FeatureValue>, L)]) -> Self {
        let label = majority_label(labeled_featuresets)
            .expect("leaf requires at least one labeled featureset");
        Self::new(label, None, HashMap::new(), None)
    }

    pub fn stump(
        feature_name: &str,
        labeled_featuresets: &[(HashMap<String, FeatureValue>, L)],
    ) -> Self {
        let label = majority_label(labeled_featuresets)
            .expect("stump requires at least one labeled featureset");

        let mut grouped: HashMap<String, Vec<(HashMap<String, FeatureValue>, L)>> = HashMap::new();
        for (featureset, y) in labeled_featuresets {
            let value = featureset
                .get(feature_name)
                .map(encode_feature_value)
                .unwrap_or_else(|| "None".to_string());
            grouped
                .entry(value)
                .or_default()
                .push((featureset.clone(), y.clone()));
        }

        let mut decisions = HashMap::new();
        for (value, subset) in grouped {
            let child_label = majority_label(&subset).unwrap_or_else(|| label.clone());
            decisions.insert(value, Self::new(child_label, None, HashMap::new(), None));
        }

        Self::new(label, Some(feature_name.to_string()), decisions, None)
    }

    pub fn best_stump(
        feature_names: &HashSet<String>,
        labeled_featuresets: &[(HashMap<String, FeatureValue>, L)],
    ) -> Self {
        let mut best_stump = Self::leaf(labeled_featuresets);
        let mut best_error = best_stump.error(labeled_featuresets);

        for fname in feature_names {
            let stump = Self::stump(fname, labeled_featuresets);
            let stump_error = stump.error(labeled_featuresets);
            if stump_error < best_error {
                best_error = stump_error;
                best_stump = stump;
            }
        }

        best_stump
    }

    pub fn refine(
        &mut self,
        labeled_featuresets: &[(HashMap<String, FeatureValue>, L)],
        entropy_cutoff: f64,
        depth_cutoff: usize,
        support_cutoff: usize,
        feature_names: &HashSet<String>,
    ) {
        if labeled_featuresets.len() <= support_cutoff {
            return;
        }
        if self.feature_name.is_none() || depth_cutoff == 0 {
            return;
        }

        let fname = self.feature_name.clone().expect("checked above");
        let decision_keys: Vec<String> = self.decisions.keys().cloned().collect();

        for fval in decision_keys {
            let subset: Vec<(HashMap<String, FeatureValue>, L)> = labeled_featuresets
                .iter()
                .filter(|(featureset, _)| {
                    featureset
                        .get(&fname)
                        .map(encode_feature_value)
                        .unwrap_or_else(|| "None".to_string())
                        == fval
                })
                .cloned()
                .collect();

            if subset.is_empty() {
                continue;
            }

            if entropy_of_labels(&subset) > entropy_cutoff {
                let mut subtree = Self::best_stump(feature_names, &subset);
                subtree.refine(
                    &subset,
                    entropy_cutoff,
                    depth_cutoff.saturating_sub(1),
                    support_cutoff,
                    feature_names,
                );
                self.decisions.insert(fval, subtree);
            }
        }

        if let Some(default) = self.default.as_mut() {
            let subset: Vec<(HashMap<String, FeatureValue>, L)> = labeled_featuresets
                .iter()
                .filter(|(featureset, _)| {
                    let current = featureset
                        .get(&fname)
                        .map(encode_feature_value)
                        .unwrap_or_else(|| "None".to_string());
                    !self.decisions.contains_key(&current)
                })
                .cloned()
                .collect();

            if !subset.is_empty() && entropy_of_labels(&subset) > entropy_cutoff {
                let mut subtree = Self::best_stump(feature_names, &subset);
                subtree.refine(
                    &subset,
                    entropy_cutoff,
                    depth_cutoff.saturating_sub(1),
                    support_cutoff,
                    feature_names,
                );
                *default = Box::new(subtree);
            }
        }
    }

    pub fn pretty_format(&self, width: usize, prefix: &str, depth: usize) -> String
    where
        L: std::fmt::Debug,
    {
        if self.feature_name.is_none() {
            let n = width.saturating_sub(prefix.len() + 15);
            return format!("{}{} {:?}\n", prefix, ".".repeat(n), self.label);
        }

        let fname = self.feature_name.as_ref().expect("checked above");
        let mut out = String::new();
        let mut entries: Vec<_> = self.decisions.iter().collect();
        entries.sort_by(|a, b| a.0.cmp(b.0));

        for (fval, subtree) in entries {
            let hdr = format!("{prefix}{fname}={fval}? ");
            let n = width.saturating_sub(15 + hdr.len());
            out.push_str(&format!("{}{} {:?}\n", hdr, ".".repeat(n), subtree.label));
            if subtree.feature_name.is_some() && depth > 1 {
                out.push_str(&subtree.pretty_format(width, &(prefix.to_owned() + "  "), depth - 1));
            }
        }

        out
    }
}

impl<L> Classifier for DecisionTreeClassifier<L>
where
    L: Eq + Hash + Clone,
{
    type Label = L;
    type FeatureSet = HashMap<String, FeatureValue>;

    fn labels(&self) -> Vec<Self::Label> {
        self.labels()
    }

    fn classify_many(&self, featuresets: &[Self::FeatureSet]) -> Vec<Self::Label> {
        featuresets.iter().map(|fs| self.classify(fs)).collect()
    }

    fn prob_classify_many(
        &self,
        featuresets: &[Self::FeatureSet],
    ) -> Vec<super::classify::LabelProbDist<Self::Label>> {
        featuresets
            .iter()
            .map(|fs| {
                let mut probs = HashMap::new();
                probs.insert(self.classify(fs), 1.0);
                super::classify::LabelProbDist::new(probs)
            })
            .collect()
    }
}

fn majority_label<L>(labeled_featuresets: &[(HashMap<String, FeatureValue>, L)]) -> Option<L>
where
    L: Eq + Hash + Clone,
{
    let mut counts = HashMap::<L, usize>::new();
    for (_, label) in labeled_featuresets {
        *counts.entry(label.clone()).or_insert(0) += 1;
    }

    counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(label, _)| label)
}

fn entropy_of_labels<L>(labeled_featuresets: &[(HashMap<String, FeatureValue>, L)]) -> f64
where
    L: Eq + Hash + Clone,
{
    if labeled_featuresets.is_empty() {
        return 0.0;
    }

    let mut counts = HashMap::<L, usize>::new();
    for (_, label) in labeled_featuresets {
        *counts.entry(label.clone()).or_insert(0) += 1;
    }

    let n = labeled_featuresets.len() as f64;
    counts
        .values()
        .map(|c| {
            let p = *c as f64 / n;
            if p <= f64::EPSILON {
                0.0
            } else {
                -p * p.log2()
            }
        })
        .sum()
}

fn encode_feature_value(v: &FeatureValue) -> String {
    match v {
        FeatureValue::Bool(value) => format!("bool:{value}"),
        FeatureValue::Int(value) => format!("int:{value}"),
        FeatureValue::Float(value) => format!("float:{value:.12}"),
        FeatureValue::Text(value) => format!("text:{value}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fs(pairs: &[(&str, FeatureValue)]) -> HashMap<String, FeatureValue> {
        let mut out = HashMap::new();
        for (k, v) in pairs {
            out.insert((*k).to_string(), v.clone());
        }
        out
    }

    #[test]
    fn decision_tree_train_and_classify() {
        let train = vec![
            (
                fs(&[
                    ("endswith", FeatureValue::Text("a".to_string())),
                    ("startswith", FeatureValue::Text("m".to_string())),
                ]),
                "female",
            ),
            (
                fs(&[
                    ("endswith", FeatureValue::Text("n".to_string())),
                    ("startswith", FeatureValue::Text("j".to_string())),
                ]),
                "male",
            ),
            (
                fs(&[
                    ("endswith", FeatureValue::Text("a".to_string())),
                    ("startswith", FeatureValue::Text("l".to_string())),
                ]),
                "female",
            ),
            (
                fs(&[
                    ("endswith", FeatureValue::Text("k".to_string())),
                    ("startswith", FeatureValue::Text("m".to_string())),
                ]),
                "male",
            ),
        ];

        let tree = DecisionTreeClassifier::train(&train, 0.0, 10, 0);
        let sample = fs(&[("endswith", FeatureValue::Text("a".to_string()))]);
        let predicted = tree.classify(&sample);

        assert_eq!(predicted, "female");
        assert!(tree.error(&train) <= 0.5);
    }

    #[test]
    fn pretty_format_is_non_empty() {
        let train = vec![
            (fs(&[("x", FeatureValue::Bool(true))]), "yes"),
            (fs(&[("x", FeatureValue::Bool(false))]), "no"),
        ];

        let tree = DecisionTreeClassifier::train(&train, 0.0, 5, 0);
        let text = tree.pretty_format(60, "", 3);
        assert!(!text.is_empty());
    }
}
