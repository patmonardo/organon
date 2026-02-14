use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct DictionaryProbDist<L>
where
    L: Eq + Hash + Clone,
{
    probs: HashMap<L, f64>,
}

impl<L> DictionaryProbDist<L>
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

    pub fn as_map(&self) -> &HashMap<L, f64> {
        &self.probs
    }
}

pub trait Cluster {
    type Label: Eq + Hash + Clone;

    fn cluster(
        &mut self,
        vectors: &[Vec<f64>],
        assign_clusters: bool,
        trace: bool,
    ) -> Option<Vec<Self::Label>>;

    fn classify(&self, vector: &[f64]) -> Self::Label;

    fn num_clusters(&self) -> usize;

    fn likelihood(&self, vector: &[f64], label: &Self::Label) -> f64 {
        if self.classify(vector) == *label {
            1.0
        } else {
            0.0
        }
    }

    fn classification_probdist(&self, vector: &[f64]) -> DictionaryProbDist<Self::Label> {
        let clusters = self.cluster_names();
        let mut likelihoods = HashMap::with_capacity(clusters.len());
        let mut total = 0.0;

        for cluster in &clusters {
            let value = self.likelihood(vector, cluster);
            likelihoods.insert(cluster.clone(), value);
            total += value;
        }

        if total > 0.0 {
            for cluster in &clusters {
                if let Some(prob) = likelihoods.get_mut(cluster) {
                    *prob /= total;
                }
            }
        }

        DictionaryProbDist::new(likelihoods)
    }

    fn cluster_names(&self) -> Vec<Self::Label>;

    fn cluster_name(&self, index: usize) -> Self::Label;
}

pub trait ClusterUsize: Cluster<Label = usize> {
    fn cluster_names(&self) -> Vec<usize> {
        (0..self.num_clusters()).collect()
    }

    fn cluster_name(&self, index: usize) -> usize {
        index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    struct DummyClusterer {
        clusters: usize,
    }

    impl Cluster for DummyClusterer {
        type Label = usize;

        fn cluster(
            &mut self,
            vectors: &[Vec<f64>],
            assign_clusters: bool,
            _trace: bool,
        ) -> Option<Vec<Self::Label>> {
            if assign_clusters {
                Some(vectors.iter().map(|v| self.classify(v)).collect())
            } else {
                None
            }
        }

        fn classify(&self, vector: &[f64]) -> Self::Label {
            if vector.iter().sum::<f64>() >= 0.0 {
                1.min(self.clusters.saturating_sub(1))
            } else {
                0
            }
        }

        fn num_clusters(&self) -> usize {
            self.clusters
        }

        fn cluster_names(&self) -> Vec<Self::Label> {
            (0..self.num_clusters()).collect()
        }

        fn cluster_name(&self, index: usize) -> Self::Label {
            index
        }
    }

    #[test]
    fn classification_probdist_normalises() {
        let model = DummyClusterer { clusters: 2 };
        let probdist = model.classification_probdist(&[1.0, 2.0]);
        let sum: f64 = probdist.as_map().values().sum();
        assert!((sum - 1.0).abs() < 1e-12);
        assert_eq!(probdist.prob(&1), 1.0);
        assert_eq!(probdist.prob(&0), 0.0);
    }
}
