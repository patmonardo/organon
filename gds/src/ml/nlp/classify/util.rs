use std::collections::HashMap;

use super::classify::{Classifier, LabelProbDist};

pub fn apply_features<T, F, O>(feature_func: F, toks: &[T]) -> Vec<O>
where
    F: Fn(&T) -> O,
{
    toks.iter().map(feature_func).collect()
}

pub fn apply_features_labeled<T, L, F, O>(feature_func: F, toks: &[(T, L)]) -> Vec<(O, L)>
where
    F: Fn(&T) -> O,
    L: Clone,
{
    toks.iter()
        .map(|(tok, label)| (feature_func(tok), label.clone()))
        .collect()
}

pub fn attested_labels<T, L>(tokens: &[(T, L)]) -> Vec<L>
where
    L: Clone + Eq + std::hash::Hash,
{
    let mut seen = HashMap::<L, ()>::new();
    for (_, label) in tokens {
        seen.entry(label.clone()).or_insert(());
    }
    seen.into_keys().collect()
}

pub fn log_likelihood<C>(classifier: &C, gold: &[(C::FeatureSet, C::Label)]) -> f64
where
    C: Classifier,
    C::FeatureSet: Clone,
{
    if gold.is_empty() {
        return 0.0;
    }

    let featuresets: Vec<C::FeatureSet> = gold.iter().map(|(fs, _)| fs.clone()).collect();
    let results = classifier.prob_classify_many(&featuresets);

    let mut total = 0.0;
    for ((_, label), pdist) in gold.iter().zip(results.iter()) {
        total += pdist.prob(label);
    }

    (total / gold.len() as f64).max(f64::MIN_POSITIVE).ln()
}

pub fn accuracy<C>(classifier: &C, gold: &[(C::FeatureSet, C::Label)]) -> f64
where
    C: Classifier,
    C::FeatureSet: Clone,
{
    if gold.is_empty() {
        return 0.0;
    }

    let featuresets: Vec<C::FeatureSet> = gold.iter().map(|(fs, _)| fs.clone()).collect();
    let results = classifier.classify_many(&featuresets);

    let correct = gold
        .iter()
        .zip(results.iter())
        .filter(|((_, expected), predicted)| expected == *predicted)
        .count();

    correct as f64 / gold.len() as f64
}

pub fn names_demo_features(name: &str) -> HashMap<String, FeatureValue> {
    let mut features = HashMap::new();
    let lower = name.to_lowercase();

    features.insert("alwayson".to_string(), FeatureValue::Bool(true));

    if let Some(first) = lower.chars().next() {
        features.insert(
            "startswith".to_string(),
            FeatureValue::Text(first.to_string()),
        );
    }

    if let Some(last) = lower.chars().last() {
        features.insert("endswith".to_string(), FeatureValue::Text(last.to_string()));
    }

    for letter in "abcdefghijklmnopqrstuvwxyz".chars() {
        let count = lower.chars().filter(|c| *c == letter).count() as i64;
        features.insert(format!("count({letter})"), FeatureValue::Int(count));
        features.insert(
            format!("has({letter})"),
            FeatureValue::Bool(lower.contains(letter)),
        );
    }

    features
}

#[derive(Debug, Clone, PartialEq)]
pub enum FeatureValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(String),
}

pub fn deterministic_probdist<L>(label: L) -> LabelProbDist<L>
where
    L: Eq + std::hash::Hash + Clone,
{
    let mut probs = HashMap::new();
    probs.insert(label, 1.0);
    LabelProbDist::new(probs)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::ml::nlp::classify::classify::Classifier;

    #[derive(Debug, Clone)]
    struct ThresholdClassifier;

    impl Classifier for ThresholdClassifier {
        type Label = &'static str;
        type FeatureSet = HashMap<String, FeatureValue>;

        fn labels(&self) -> Vec<Self::Label> {
            vec!["low", "high"]
        }

        fn classify_many(&self, featuresets: &[Self::FeatureSet]) -> Vec<Self::Label> {
            featuresets
                .iter()
                .map(|fs| match fs.get("value") {
                    Some(FeatureValue::Float(v)) if *v > 0.5 => "high",
                    _ => "low",
                })
                .collect()
        }

        fn prob_classify_many(
            &self,
            featuresets: &[Self::FeatureSet],
        ) -> Vec<LabelProbDist<Self::Label>> {
            featuresets
                .iter()
                .map(|fs| match fs.get("value") {
                    Some(FeatureValue::Float(v)) if *v > 0.5 => {
                        let mut p = HashMap::new();
                        p.insert("high", 0.8);
                        p.insert("low", 0.2);
                        LabelProbDist::new(p)
                    }
                    _ => {
                        let mut p = HashMap::new();
                        p.insert("high", 0.2);
                        p.insert("low", 0.8);
                        LabelProbDist::new(p)
                    }
                })
                .collect()
        }
    }

    #[test]
    fn utility_metrics_work() {
        let classifier = ThresholdClassifier;
        let mut fs1 = HashMap::new();
        fs1.insert("value".to_string(), FeatureValue::Float(0.9));
        let mut fs2 = HashMap::new();
        fs2.insert("value".to_string(), FeatureValue::Float(0.1));

        let gold = vec![(fs1, "high"), (fs2, "low")];
        let acc = accuracy(&classifier, &gold);
        assert!((acc - 1.0).abs() < 1e-12);

        let ll = log_likelihood(&classifier, &gold);
        assert!(ll.is_finite());
    }

    #[test]
    fn names_features_extracts_expected_keys() {
        let features = names_demo_features("Alice");
        assert!(features.contains_key("alwayson"));
        assert!(features.contains_key("startswith"));
        assert!(features.contains_key("endswith"));
        assert!(features.contains_key("count(a)"));
    }
}
