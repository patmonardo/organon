use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use super::classify::{Classifier, LabelProbDist};
use super::util::FeatureValue;

const NONE_VALUE: &str = "__NONE__";

#[derive(Debug, Clone)]
pub struct NaiveBayesClassifier<L>
where
    L: Eq + Hash + Clone,
{
    labels: Vec<L>,
    label_log_probs: HashMap<L, f64>,
    feature_log_probs: HashMap<(L, String), HashMap<String, f64>>,
    unseen_feature_log_probs: HashMap<(L, String), f64>,
    known_feature_names: HashSet<String>,
}

impl<L> NaiveBayesClassifier<L>
where
    L: Eq + Hash + Clone,
{
    pub fn train(labeled_featuresets: &[(HashMap<String, FeatureValue>, L)], alpha: f64) -> Self {
        let alpha = alpha.max(f64::EPSILON);

        let mut label_counts: HashMap<L, usize> = HashMap::new();
        let mut feature_counts: HashMap<(L, String), HashMap<String, usize>> = HashMap::new();
        let mut feature_values: HashMap<String, HashSet<String>> = HashMap::new();
        let mut feature_seen_count: HashMap<(L, String), usize> = HashMap::new();
        let mut known_feature_names: HashSet<String> = HashSet::new();

        for (featureset, label) in labeled_featuresets {
            *label_counts.entry(label.clone()).or_insert(0) += 1;

            for (fname, fval) in featureset {
                let encoded_value = encode_feature_value(fval);

                known_feature_names.insert(fname.clone());
                feature_values
                    .entry(fname.clone())
                    .or_default()
                    .insert(encoded_value.clone());

                let key = (label.clone(), fname.clone());
                let counts = feature_counts.entry(key.clone()).or_default();
                *counts.entry(encoded_value).or_insert(0) += 1;
                *feature_seen_count.entry(key).or_insert(0) += 1;
            }
        }

        for values in feature_values.values_mut() {
            values.insert(NONE_VALUE.to_string());
        }

        for (label, label_total) in &label_counts {
            for fname in &known_feature_names {
                let key = (label.clone(), fname.clone());
                let seen = *feature_seen_count.get(&key).unwrap_or(&0);
                let missing = label_total.saturating_sub(seen);
                let counts = feature_counts.entry(key).or_default();
                *counts.entry(NONE_VALUE.to_string()).or_insert(0) += missing;
            }
        }

        let labels: Vec<L> = label_counts.keys().cloned().collect();
        let total_samples = labeled_featuresets.len() as f64;

        let mut label_log_probs = HashMap::new();
        for (label, count) in &label_counts {
            let prior = (*count as f64 / total_samples).max(f64::MIN_POSITIVE);
            label_log_probs.insert(label.clone(), prior.ln());
        }

        let mut feature_log_probs: HashMap<(L, String), HashMap<String, f64>> = HashMap::new();
        let mut unseen_feature_log_probs = HashMap::new();

        for label in &labels {
            for fname in &known_feature_names {
                let key = (label.clone(), fname.clone());
                let counts = feature_counts.get(&key);
                let bins = feature_values.get(fname).map(|s| s.len()).unwrap_or(1) as f64;
                let n = *label_counts.get(label).unwrap_or(&0) as f64;
                let denominator = n + alpha * bins;

                let mut log_map = HashMap::new();
                if let Some(values) = feature_values.get(fname) {
                    for fval in values {
                        let count = counts.and_then(|c| c.get(fval)).copied().unwrap_or(0) as f64;
                        let p = ((count + alpha) / denominator).max(f64::MIN_POSITIVE);
                        log_map.insert(fval.clone(), p.ln());
                    }
                }

                let unseen = (alpha / denominator).max(f64::MIN_POSITIVE).ln();
                unseen_feature_log_probs.insert(key.clone(), unseen);
                feature_log_probs.insert(key, log_map);
            }
        }

        Self {
            labels,
            label_log_probs,
            feature_log_probs,
            unseen_feature_log_probs,
            known_feature_names,
        }
    }
}

impl<L> Classifier for NaiveBayesClassifier<L>
where
    L: Eq + Hash + Clone,
{
    type Label = L;
    type FeatureSet = HashMap<String, FeatureValue>;

    fn labels(&self) -> Vec<Self::Label> {
        self.labels.clone()
    }

    fn classify_many(&self, featuresets: &[Self::FeatureSet]) -> Vec<Self::Label> {
        featuresets
            .iter()
            .filter_map(|featureset| self.prob_classify(featureset).max())
            .collect()
    }

    fn prob_classify_many(
        &self,
        featuresets: &[Self::FeatureSet],
    ) -> Vec<LabelProbDist<Self::Label>> {
        featuresets
            .iter()
            .map(|featureset| {
                let mut logprob = HashMap::<L, f64>::new();

                for label in &self.labels {
                    let mut score = *self
                        .label_log_probs
                        .get(label)
                        .unwrap_or(&f64::NEG_INFINITY);

                    for (fname, fval) in featureset {
                        if !self.known_feature_names.contains(fname) {
                            continue;
                        }

                        let key = (label.clone(), fname.clone());
                        let encoded_value = encode_feature_value(fval);

                        if let Some(feature_map) = self.feature_log_probs.get(&key) {
                            if let Some(v) = feature_map.get(&encoded_value) {
                                score += *v;
                            } else {
                                score += *self
                                    .unseen_feature_log_probs
                                    .get(&key)
                                    .unwrap_or(&f64::NEG_INFINITY);
                            }
                        } else {
                            score = f64::NEG_INFINITY;
                        }
                    }

                    logprob.insert(label.clone(), score);
                }

                normalize_logprobs(logprob)
            })
            .collect()
    }
}

fn encode_feature_value(v: &FeatureValue) -> String {
    match v {
        FeatureValue::Bool(value) => format!("bool:{value}"),
        FeatureValue::Int(value) => format!("int:{value}"),
        FeatureValue::Float(value) => format!("float:{value:.12}"),
        FeatureValue::Text(value) => format!("text:{value}"),
    }
}

fn normalize_logprobs<L>(log_probs: HashMap<L, f64>) -> LabelProbDist<L>
where
    L: Eq + Hash + Clone,
{
    let max_log = log_probs
        .values()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max);
    let mut probs = HashMap::with_capacity(log_probs.len());

    let mut z = 0.0;
    for (label, logp) in &log_probs {
        let p = if logp.is_finite() {
            (*logp - max_log).exp()
        } else {
            0.0
        };
        probs.insert(label.clone(), p);
        z += p;
    }

    if z > 0.0 {
        for p in probs.values_mut() {
            *p /= z;
        }
    }

    LabelProbDist::new(probs)
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
    fn naive_bayes_trains_and_classifies() {
        let train = vec![
            (
                fs(&[
                    ("contains(goal)", FeatureValue::Bool(true)),
                    ("contains(team)", FeatureValue::Bool(true)),
                ]),
                "sports",
            ),
            (
                fs(&[
                    ("contains(stadium)", FeatureValue::Bool(true)),
                    ("contains(team)", FeatureValue::Bool(true)),
                ]),
                "sports",
            ),
            (
                fs(&[
                    ("contains(stock)", FeatureValue::Bool(true)),
                    ("contains(market)", FeatureValue::Bool(true)),
                ]),
                "finance",
            ),
            (
                fs(&[
                    ("contains(bond)", FeatureValue::Bool(true)),
                    ("contains(market)", FeatureValue::Bool(true)),
                ]),
                "finance",
            ),
        ];

        let clf = NaiveBayesClassifier::train(&train, 0.5);

        let sample = fs(&[
            ("contains(goal)", FeatureValue::Bool(true)),
            ("contains(team)", FeatureValue::Bool(true)),
        ]);

        assert_eq!(Classifier::classify(&clf, &sample), "sports");

        let pd = Classifier::prob_classify(&clf, &sample);
        assert!(pd.prob(&"sports") > pd.prob(&"finance"));
    }

    #[test]
    fn unseen_feature_names_are_ignored() {
        let train = vec![
            (fs(&[("x", FeatureValue::Bool(true))]), "yes"),
            (fs(&[("x", FeatureValue::Bool(false))]), "no"),
        ];

        let clf = NaiveBayesClassifier::train(&train, 0.5);
        let sample = fs(&[
            ("x", FeatureValue::Bool(true)),
            ("never_seen", FeatureValue::Text("v".to_string())),
        ]);

        let label = Classifier::classify(&clf, &sample);
        assert!(label == "yes" || label == "no");
    }
}
