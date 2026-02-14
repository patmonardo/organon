use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct LabelProbDist<L>
where
    L: Eq + Hash + Clone,
{
    probs: HashMap<L, f64>,
}

impl<L> LabelProbDist<L>
where
    L: Eq + Hash + Clone,
{
    pub fn new(probs: HashMap<L, f64>) -> Self {
        Self { probs }
    }

    pub fn prob(&self, label: &L) -> f64 {
        *self.probs.get(label).unwrap_or(&0.0)
    }

    pub fn samples(&self) -> Vec<L> {
        self.probs.keys().cloned().collect()
    }

    pub fn max(&self) -> Option<L> {
        self.probs
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(label, _)| label.clone())
    }

    pub fn as_map(&self) -> &HashMap<L, f64> {
        &self.probs
    }
}

pub trait Classifier {
    type Label: Eq + Hash + Clone;
    type FeatureSet;

    fn labels(&self) -> Vec<Self::Label>;

    fn classify(&self, featureset: &Self::FeatureSet) -> Self::Label {
        self.classify_many(std::slice::from_ref(featureset))
            .into_iter()
            .next()
            .expect("classify_many must return one label for one featureset")
    }

    fn prob_classify(&self, featureset: &Self::FeatureSet) -> LabelProbDist<Self::Label> {
        self.prob_classify_many(std::slice::from_ref(featureset))
            .into_iter()
            .next()
            .expect("prob_classify_many must return one dist for one featureset")
    }

    fn classify_many(&self, featuresets: &[Self::FeatureSet]) -> Vec<Self::Label>;

    fn prob_classify_many(
        &self,
        featuresets: &[Self::FeatureSet],
    ) -> Vec<LabelProbDist<Self::Label>>;
}

pub trait MultiClassifier {
    type Label: Eq + Hash + Clone;
    type FeatureSet;

    fn labels(&self) -> Vec<Self::Label>;

    fn classify(&self, featureset: &Self::FeatureSet) -> Vec<Self::Label> {
        self.classify_many(std::slice::from_ref(featureset))
            .into_iter()
            .next()
            .expect("classify_many must return one label-set for one featureset")
    }

    fn prob_classify(&self, featureset: &Self::FeatureSet) -> LabelProbDist<Vec<Self::Label>> {
        self.prob_classify_many(std::slice::from_ref(featureset))
            .into_iter()
            .next()
            .expect("prob_classify_many must return one dist for one featureset")
    }

    fn classify_many(&self, featuresets: &[Self::FeatureSet]) -> Vec<Vec<Self::Label>>;

    fn prob_classify_many(
        &self,
        featuresets: &[Self::FeatureSet],
    ) -> Vec<LabelProbDist<Vec<Self::Label>>>;
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[derive(Debug, Clone)]
    struct SignClassifier;

    impl Classifier for SignClassifier {
        type Label = &'static str;
        type FeatureSet = Vec<f64>;

        fn labels(&self) -> Vec<Self::Label> {
            vec!["neg", "pos"]
        }

        fn classify_many(&self, featuresets: &[Self::FeatureSet]) -> Vec<Self::Label> {
            featuresets
                .iter()
                .map(|fs| {
                    if fs.iter().sum::<f64>() >= 0.0 {
                        "pos"
                    } else {
                        "neg"
                    }
                })
                .collect()
        }

        fn prob_classify_many(
            &self,
            featuresets: &[Self::FeatureSet],
        ) -> Vec<LabelProbDist<Self::Label>> {
            featuresets
                .iter()
                .map(|fs| {
                    let mut probs = HashMap::new();
                    if fs.iter().sum::<f64>() >= 0.0 {
                        probs.insert("pos", 0.9);
                        probs.insert("neg", 0.1);
                    } else {
                        probs.insert("pos", 0.1);
                        probs.insert("neg", 0.9);
                    }
                    LabelProbDist::new(probs)
                })
                .collect()
        }
    }

    #[test]
    fn classify_and_prob_classify_defaults_work() {
        let model = SignClassifier;
        assert_eq!(model.classify(&vec![1.0, -0.2]), "pos");

        let dist = model.prob_classify(&vec![-2.0]);
        assert_eq!(dist.max(), Some("neg"));
    }
}
