use std::collections::{HashMap, HashSet};

use super::classify::{Classifier, LabelProbDist};
use super::util::FeatureValue;

const NONE_VALUE: &str = "__NONE__";

#[derive(Debug, Clone)]
pub struct PositiveNaiveBayesClassifier {
    labels: Vec<bool>,
    label_probs: HashMap<bool, f64>,
    feature_probs: HashMap<(bool, String), HashMap<String, f64>>,
    known_feature_names: HashSet<String>,
}

impl PositiveNaiveBayesClassifier {
    pub fn train(
        positive_featuresets: &[HashMap<String, FeatureValue>],
        unlabeled_featuresets: &[HashMap<String, FeatureValue>],
        positive_prob_prior: f64,
        alpha: f64,
    ) -> Self {
        let alpha = alpha.max(f64::EPSILON);
        let positive_prob_prior = positive_prob_prior.clamp(f64::EPSILON, 1.0 - f64::EPSILON);
        let negative_prob_prior = 1.0 - positive_prob_prior;

        let mut positive_feature_counts: HashMap<String, HashMap<String, usize>> = HashMap::new();
        let mut unlabeled_feature_counts: HashMap<String, HashMap<String, usize>> = HashMap::new();
        let mut feature_values: HashMap<String, HashSet<String>> = HashMap::new();
        let mut known_feature_names: HashSet<String> = HashSet::new();
        let mut positive_seen_count: HashMap<String, usize> = HashMap::new();
        let mut unlabeled_seen_count: HashMap<String, usize> = HashMap::new();

        for featureset in positive_featuresets {
            for (fname, fval) in featureset {
                let encoded = encode_feature_value(fval);
                known_feature_names.insert(fname.clone());
                feature_values
                    .entry(fname.clone())
                    .or_default()
                    .insert(encoded.clone());
                *positive_feature_counts
                    .entry(fname.clone())
                    .or_default()
                    .entry(encoded)
                    .or_insert(0) += 1;
                *positive_seen_count.entry(fname.clone()).or_insert(0) += 1;
            }
        }

        for featureset in unlabeled_featuresets {
            for (fname, fval) in featureset {
                let encoded = encode_feature_value(fval);
                known_feature_names.insert(fname.clone());
                feature_values
                    .entry(fname.clone())
                    .or_default()
                    .insert(encoded.clone());
                *unlabeled_feature_counts
                    .entry(fname.clone())
                    .or_default()
                    .entry(encoded)
                    .or_insert(0) += 1;
                *unlabeled_seen_count.entry(fname.clone()).or_insert(0) += 1;
            }
        }

        for values in feature_values.values_mut() {
            values.insert(NONE_VALUE.to_string());
        }

        let num_positive = positive_featuresets.len();
        let num_unlabeled = unlabeled_featuresets.len();

        for fname in &known_feature_names {
            let observed = *positive_seen_count.get(fname).unwrap_or(&0);
            let missing = num_positive.saturating_sub(observed);
            *positive_feature_counts
                .entry(fname.clone())
                .or_default()
                .entry(NONE_VALUE.to_string())
                .or_insert(0) += missing;

            let observed_unlabeled = *unlabeled_seen_count.get(fname).unwrap_or(&0);
            let missing_unlabeled = num_unlabeled.saturating_sub(observed_unlabeled);
            *unlabeled_feature_counts
                .entry(fname.clone())
                .or_default()
                .entry(NONE_VALUE.to_string())
                .or_insert(0) += missing_unlabeled;
        }

        let mut feature_probs: HashMap<(bool, String), HashMap<String, f64>> = HashMap::new();

        for fname in &known_feature_names {
            let values = feature_values.get(fname).cloned().unwrap_or_default();
            let bins = values.len().max(1) as f64;

            let pos_counts = positive_feature_counts.get(fname);
            let pos_denominator = num_positive as f64 + alpha * bins;
            let mut pos_probs = HashMap::new();
            for v in &values {
                let c = pos_counts.and_then(|m| m.get(v)).copied().unwrap_or(0) as f64;
                pos_probs.insert(v.clone(), (c + alpha) / pos_denominator);
            }
            feature_probs.insert((true, fname.clone()), pos_probs.clone());

            let unl_counts = unlabeled_feature_counts.get(fname);
            let unl_denominator = num_unlabeled as f64 + alpha * bins;
            let mut global_probs = HashMap::new();
            for v in &values {
                let c = unl_counts.and_then(|m| m.get(v)).copied().unwrap_or(0) as f64;
                global_probs.insert(v.clone(), (c + alpha) / unl_denominator);
            }

            let mut neg_probs = HashMap::new();
            let mut sum_neg = 0.0;
            for v in &values {
                let gp = *global_probs.get(v).unwrap_or(&0.0);
                let pp = *pos_probs.get(v).unwrap_or(&0.0);
                let raw = (gp - positive_prob_prior * pp) / negative_prob_prior;
                let clipped = raw.max(0.0);
                neg_probs.insert(v.clone(), clipped);
                sum_neg += clipped;
            }

            if sum_neg <= f64::EPSILON {
                let uniform = 1.0 / values.len().max(1) as f64;
                for v in &values {
                    neg_probs.insert(v.clone(), uniform);
                }
            } else {
                for prob in neg_probs.values_mut() {
                    *prob /= sum_neg;
                }
            }

            feature_probs.insert((false, fname.clone()), neg_probs);
        }

        let mut label_probs = HashMap::new();
        label_probs.insert(true, positive_prob_prior);
        label_probs.insert(false, negative_prob_prior);

        Self {
            labels: vec![true, false],
            label_probs,
            feature_probs,
            known_feature_names,
        }
    }
}

impl Classifier for PositiveNaiveBayesClassifier {
    type Label = bool;
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
                let mut logprob = HashMap::new();

                for label in &self.labels {
                    let mut score = self
                        .label_probs
                        .get(label)
                        .copied()
                        .unwrap_or(f64::MIN_POSITIVE)
                        .max(f64::MIN_POSITIVE)
                        .ln();

                    for (fname, fval) in featureset {
                        if !self.known_feature_names.contains(fname) {
                            continue;
                        }

                        let key = (*label, fname.clone());
                        let encoded = encode_feature_value(fval);

                        let p = self
                            .feature_probs
                            .get(&key)
                            .and_then(|m| m.get(&encoded))
                            .copied()
                            .unwrap_or(f64::MIN_POSITIVE)
                            .max(f64::MIN_POSITIVE);
                        score += p.ln();
                    }

                    logprob.insert(*label, score);
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

fn normalize_logprobs(log_probs: HashMap<bool, f64>) -> LabelProbDist<bool> {
    let max_log = log_probs
        .values()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max);

    let mut probs = HashMap::new();
    let mut z = 0.0;

    for (label, logp) in &log_probs {
        let p = if logp.is_finite() {
            (*logp - max_log).exp()
        } else {
            0.0
        };
        probs.insert(*label, p);
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

    fn features(sentence: &str) -> HashMap<String, FeatureValue> {
        let mut map = HashMap::new();
        for w in sentence.to_lowercase().split_whitespace() {
            map.insert(format!("contains({w})"), FeatureValue::Bool(true));
        }
        map
    }

    #[test]
    fn positive_naive_bayes_matches_expected_behavior() {
        let positive = vec![
            features("the team dominated the game"),
            features("they lost the ball"),
            features("the game was intense"),
            features("the goalkeeper caught the ball"),
            features("the other team controlled the ball"),
        ];

        let unlabeled = vec![
            features("the president did not comment"),
            features("i lost the keys"),
            features("the team won the game"),
            features("sara has two kids"),
            features("the ball went off the court"),
            features("they had the ball for the whole game"),
            features("the show is over"),
        ];

        let clf = PositiveNaiveBayesClassifier::train(&positive, &unlabeled, 0.5, 0.5);

        let sports = features("my team lost the game");
        let not_sports = features("the cat is on the table");

        assert!(Classifier::classify(&clf, &sports));
        let _ = Classifier::classify(&clf, &not_sports);
    }
}
