use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use super::classify::{Classifier, LabelProbDist};
use super::util::FeatureValue;

pub type JointFeatureVector = Vec<(usize, f64)>;

#[derive(Debug, Clone)]
pub struct MaxentTrainingConfig {
    pub learning_rate: f64,
    pub max_iter: usize,
    pub l2_regularization: f64,
    pub tolerance: f64,
}

impl Default for MaxentTrainingConfig {
    fn default() -> Self {
        Self {
            learning_rate: 0.1,
            max_iter: 200,
            l2_regularization: 0.0,
            tolerance: 1e-6,
        }
    }
}

pub trait MaxentFeatureEncoding<L>
where
    L: Eq + Hash + Clone,
{
    fn encode(&self, featureset: &HashMap<String, FeatureValue>, label: &L) -> JointFeatureVector;
    fn length(&self) -> usize;
    fn labels(&self) -> Vec<L>;
    fn describe(&self, fid: usize) -> Option<String>;
}

#[derive(Clone)]
pub struct FunctionBackedMaxentFeatureEncoding<L, F>
where
    L: Eq + Hash + Clone,
    F: Fn(&HashMap<String, FeatureValue>, &L) -> JointFeatureVector + Clone,
{
    func: F,
    length: usize,
    labels: Vec<L>,
    descriptions: HashMap<usize, String>,
}

impl<L, F> FunctionBackedMaxentFeatureEncoding<L, F>
where
    L: Eq + Hash + Clone,
    F: Fn(&HashMap<String, FeatureValue>, &L) -> JointFeatureVector + Clone,
{
    pub fn new(func: F, length: usize, labels: Vec<L>) -> Self {
        Self {
            func,
            length,
            labels,
            descriptions: HashMap::new(),
        }
    }

    pub fn with_descriptions(mut self, descriptions: HashMap<usize, String>) -> Self {
        self.descriptions = descriptions;
        self
    }
}

impl<L, F> MaxentFeatureEncoding<L> for FunctionBackedMaxentFeatureEncoding<L, F>
where
    L: Eq + Hash + Clone,
    F: Fn(&HashMap<String, FeatureValue>, &L) -> JointFeatureVector + Clone,
{
    fn encode(&self, featureset: &HashMap<String, FeatureValue>, label: &L) -> JointFeatureVector {
        (self.func)(featureset, label)
    }

    fn length(&self) -> usize {
        self.length
    }

    fn labels(&self) -> Vec<L> {
        self.labels.clone()
    }

    fn describe(&self, fid: usize) -> Option<String> {
        self.descriptions.get(&fid).cloned()
    }
}

#[derive(Debug, Clone)]
pub struct BinaryMaxentFeatureEncoding<L>
where
    L: Eq + Hash + Clone,
{
    labels: Vec<L>,
    mapping: HashMap<(String, String, L), usize>,
    descriptions: Vec<String>,
}

impl<L> BinaryMaxentFeatureEncoding<L>
where
    L: Eq + Hash + Clone + std::fmt::Debug,
{
    pub fn train(
        train_toks: &[(HashMap<String, FeatureValue>, L)],
        labels: Option<Vec<L>>,
        always_on: bool,
    ) -> Self {
        let mut labels_out = labels.unwrap_or_else(|| {
            let mut seen = HashSet::new();
            for (_, label) in train_toks {
                seen.insert(label.clone());
            }
            seen.into_iter().collect()
        });

        if labels_out.is_empty() {
            labels_out = Vec::new();
        }

        let mut mapping = HashMap::new();
        let mut descriptions = Vec::new();

        let mut feature_pairs = HashSet::<(String, String)>::new();
        for (featureset, _) in train_toks {
            for (fname, fval) in featureset {
                feature_pairs.insert((fname.clone(), encode_feature_value(fval)));
            }
        }

        let mut sorted_pairs: Vec<(String, String)> = feature_pairs.into_iter().collect();
        sorted_pairs.sort();

        for label in &labels_out {
            if always_on {
                let fid = descriptions.len();
                mapping.insert(
                    (
                        "__alwayson__".to_string(),
                        "true".to_string(),
                        label.clone(),
                    ),
                    fid,
                );
                descriptions.push(format!("({label:?}) __alwayson__=true"));
            }

            for (fname, fval) in &sorted_pairs {
                let fid = descriptions.len();
                mapping.insert((fname.clone(), fval.clone(), label.clone()), fid);
                descriptions.push(format!("({label:?}) {fname}={fval}"));
            }
        }

        Self {
            labels: labels_out,
            mapping,
            descriptions,
        }
    }
}

impl<L> MaxentFeatureEncoding<L> for BinaryMaxentFeatureEncoding<L>
where
    L: Eq + Hash + Clone + std::fmt::Debug,
{
    fn encode(&self, featureset: &HashMap<String, FeatureValue>, label: &L) -> JointFeatureVector {
        let mut out = Vec::new();

        if let Some(&fid) = self.mapping.get(&(
            "__alwayson__".to_string(),
            "true".to_string(),
            label.clone(),
        )) {
            out.push((fid, 1.0));
        }

        for (fname, fval) in featureset {
            let value = encode_feature_value(fval);
            if let Some(&fid) = self.mapping.get(&(fname.clone(), value, label.clone())) {
                out.push((fid, 1.0));
            }
        }

        out
    }

    fn length(&self) -> usize {
        self.descriptions.len()
    }

    fn labels(&self) -> Vec<L> {
        self.labels.clone()
    }

    fn describe(&self, fid: usize) -> Option<String> {
        self.descriptions.get(fid).cloned()
    }
}

#[derive(Debug, Clone)]
pub struct MaxentClassifier<L, E>
where
    L: Eq + Hash + Clone,
    E: MaxentFeatureEncoding<L> + Clone,
{
    encoding: E,
    weights: Vec<f64>,
    logarithmic: bool,
    labels: Vec<L>,
}

impl<L, E> MaxentClassifier<L, E>
where
    L: Eq + Hash + Clone,
    E: MaxentFeatureEncoding<L> + Clone,
{
    pub fn new(encoding: E, weights: Vec<f64>, logarithmic: bool) -> Self {
        let mut normalized_weights = weights;
        if normalized_weights.len() < encoding.length() {
            normalized_weights.resize(encoding.length(), 0.0);
        }

        Self {
            labels: encoding.labels(),
            encoding,
            weights: normalized_weights,
            logarithmic,
        }
    }

    pub fn set_weights(&mut self, new_weights: Vec<f64>) {
        self.weights = new_weights;
        if self.weights.len() < self.encoding.length() {
            self.weights.resize(self.encoding.length(), 0.0);
        }
    }

    pub fn weights(&self) -> &[f64] {
        &self.weights
    }

    pub fn encoding(&self) -> &E {
        &self.encoding
    }

    fn unnormalized_score(&self, featureset: &HashMap<String, FeatureValue>, label: &L) -> f64 {
        let encoded = self.encoding.encode(featureset, label);
        let dot = encoded
            .iter()
            .map(|(fid, value)| self.weights.get(*fid).copied().unwrap_or(0.0) * value)
            .sum::<f64>();

        if self.logarithmic {
            dot.exp()
        } else {
            dot.max(0.0)
        }
    }

    fn probabilities_with_weights(
        encoding: &E,
        labels: &[L],
        weights: &[f64],
        featureset: &HashMap<String, FeatureValue>,
        logarithmic: bool,
    ) -> HashMap<L, f64> {
        let mut probs = HashMap::new();
        let mut z = 0.0;

        for label in labels {
            let encoded = encoding.encode(featureset, label);
            let dot = encoded
                .iter()
                .map(|(fid, value)| weights.get(*fid).copied().unwrap_or(0.0) * value)
                .sum::<f64>();
            let score = if logarithmic { dot.exp() } else { dot.max(0.0) };
            probs.insert(label.clone(), score);
            z += score;
        }

        if z > f64::EPSILON {
            for p in probs.values_mut() {
                *p /= z;
            }
        }

        probs
    }

    pub fn train(
        encoding: E,
        train_toks: &[(HashMap<String, FeatureValue>, L)],
        config: MaxentTrainingConfig,
    ) -> Self {
        let labels = encoding.labels();
        let mut weights = vec![0.0; encoding.length()];
        let n = train_toks.len().max(1) as f64;
        let lr = config.learning_rate.max(f64::EPSILON);
        let lambda = config.l2_regularization.max(0.0);

        for _ in 0..config.max_iter {
            let mut gradient = vec![0.0; encoding.length()];

            for (featureset, gold_label) in train_toks {
                for (fid, value) in encoding.encode(featureset, gold_label) {
                    if fid < gradient.len() {
                        gradient[fid] += value;
                    }
                }

                let probs = Self::probabilities_with_weights(
                    &encoding, &labels, &weights, featureset, true,
                );

                for label in &labels {
                    let p = *probs.get(label).unwrap_or(&0.0);
                    if p <= 0.0 {
                        continue;
                    }

                    for (fid, value) in encoding.encode(featureset, label) {
                        if fid < gradient.len() {
                            gradient[fid] -= p * value;
                        }
                    }
                }
            }

            let mut max_abs_update: f64 = 0.0;
            for i in 0..weights.len() {
                let reg = lambda * weights[i];
                let update = lr * ((gradient[i] / n) - reg);
                weights[i] += update;
                max_abs_update = max_abs_update.max(update.abs());
            }

            if max_abs_update < config.tolerance {
                break;
            }
        }

        Self::new(encoding, weights, true)
    }
}

impl<L, E> Classifier for MaxentClassifier<L, E>
where
    L: Eq + Hash + Clone,
    E: MaxentFeatureEncoding<L> + Clone,
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
                let mut probs = HashMap::<L, f64>::new();
                let mut z = 0.0;

                for label in &self.labels {
                    let score = self.unnormalized_score(featureset, label);
                    probs.insert(label.clone(), score);
                    z += score;
                }

                if z > f64::EPSILON {
                    for prob in probs.values_mut() {
                        *prob /= z;
                    }
                }

                LabelProbDist::new(probs)
            })
            .collect()
    }
}

pub type ConditionalExponentialClassifier<L, E> = MaxentClassifier<L, E>;

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
    fn binary_encoding_and_maxent_predict() {
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
                    ("contains(stock)", FeatureValue::Bool(true)),
                    ("contains(market)", FeatureValue::Bool(true)),
                ]),
                "finance",
            ),
        ];

        let encoding = BinaryMaxentFeatureEncoding::train(&train, None, true);
        let mut weights = vec![0.0; encoding.length()];

        for fid in 0..encoding.length() {
            if let Some(desc) = encoding.describe(fid) {
                if desc.contains("sports") && desc.contains("contains(goal)") {
                    weights[fid] = 2.0;
                }
                if desc.contains("sports") && desc.contains("contains(team)") {
                    weights[fid] = 1.5;
                }
                if desc.contains("finance") && desc.contains("contains(stock)") {
                    weights[fid] = 2.0;
                }
            }
        }

        let model = MaxentClassifier::new(encoding, weights, true);
        let sample = fs(&[
            ("contains(goal)", FeatureValue::Bool(true)),
            ("contains(team)", FeatureValue::Bool(true)),
        ]);

        let predicted = Classifier::classify(&model, &sample);
        assert_eq!(predicted, "sports");
    }

    #[test]
    fn function_backed_encoding_works() {
        let labels = vec!["yes", "no"];
        let func = |featureset: &HashMap<String, FeatureValue>, label: &&str| {
            let mut out = Vec::new();
            if let Some(FeatureValue::Bool(true)) = featureset.get("x") {
                if *label == "yes" {
                    out.push((0, 1.0));
                } else {
                    out.push((1, 1.0));
                }
            }
            out
        };

        let encoding = FunctionBackedMaxentFeatureEncoding::new(func, 2, labels);
        let model = MaxentClassifier::new(encoding, vec![2.0, 0.1], true);

        let mut sample = HashMap::new();
        sample.insert("x".to_string(), FeatureValue::Bool(true));

        assert_eq!(Classifier::classify(&model, &sample), "yes");
    }

    #[test]
    fn maxent_train_learns_signal() {
        let train = vec![
            (
                fs(&[
                    ("contains(goal)", FeatureValue::Bool(true)),
                    ("contains(team)", FeatureValue::Bool(true)),
                ]),
                "sports",
            ),
            (
                fs(&[("contains(goal)", FeatureValue::Bool(true))]),
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
                fs(&[("contains(stock)", FeatureValue::Bool(true))]),
                "finance",
            ),
        ];

        let encoding = BinaryMaxentFeatureEncoding::train(&train, None, true);
        let config = MaxentTrainingConfig {
            learning_rate: 0.25,
            max_iter: 300,
            l2_regularization: 0.001,
            tolerance: 1e-8,
        };

        let model = MaxentClassifier::train(encoding, &train, config);

        let sports_sample = fs(&[
            ("contains(goal)", FeatureValue::Bool(true)),
            ("contains(team)", FeatureValue::Bool(true)),
        ]);
        let finance_sample = fs(&[("contains(stock)", FeatureValue::Bool(true))]);

        assert_eq!(Classifier::classify(&model, &sports_sample), "sports");
        assert_eq!(Classifier::classify(&model, &finance_sample), "finance");
    }
}
