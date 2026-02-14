use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;

use super::cluster::Cluster;
use super::util::{
    euclidean_distance, ClusterError, VectorSpaceCluster, VectorSpaceClusterOps, VectorSpaceState,
};

pub type DistanceFn = fn(&[f64], &[f64]) -> f64;

#[derive(Debug, Clone)]
pub struct KMeansClusterer {
    state: VectorSpaceState,
    should_normalise: bool,
    svd_dimensions: Option<usize>,
    num_means: usize,
    distance: DistanceFn,
    repeats: usize,
    max_difference: f64,
    means: Option<Vec<Vec<f64>>>,
    avoid_empty_clusters: bool,
    rng: StdRng,
}

impl KMeansClusterer {
    pub fn new(num_means: usize, distance: DistanceFn) -> Self {
        Self {
            state: VectorSpaceState::new(),
            should_normalise: false,
            svd_dimensions: None,
            num_means,
            distance,
            repeats: 1,
            max_difference: 1e-6,
            means: None,
            avoid_empty_clusters: false,
            rng: StdRng::seed_from_u64(42),
        }
    }

    pub fn with_initial_means(mut self, means: Vec<Vec<f64>>) -> Self {
        self.means = Some(means);
        self
    }

    pub fn with_repeats(mut self, repeats: usize) -> Self {
        self.repeats = repeats.max(1);
        self
    }

    pub fn with_conv_test(mut self, conv_test: f64) -> Self {
        self.max_difference = conv_test.max(0.0);
        self
    }

    pub fn with_normalise(mut self, normalise: bool) -> Self {
        self.should_normalise = normalise;
        self
    }

    pub fn with_svd_dimensions(mut self, dimensions: Option<usize>) -> Self {
        self.svd_dimensions = dimensions;
        self
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.rng = StdRng::seed_from_u64(seed);
        self
    }

    pub fn with_avoid_empty_clusters(mut self, avoid: bool) -> Self {
        self.avoid_empty_clusters = avoid;
        self
    }

    pub fn means(&self) -> Option<&[Vec<f64>]> {
        self.means.as_deref()
    }

    fn sum_distances(&self, vectors1: &[Vec<f64>], vectors2: &[Vec<f64>]) -> f64 {
        vectors1
            .iter()
            .zip(vectors2.iter())
            .map(|(u, v)| (self.distance)(u, v))
            .sum()
    }

    fn centroid(&self, cluster: &[Vec<f64>], mean: &[f64]) -> Vec<f64> {
        if self.avoid_empty_clusters {
            let mut centroid = mean.to_vec();
            for vector in cluster {
                for (idx, value) in vector.iter().enumerate() {
                    centroid[idx] += value;
                }
            }
            let denom = 1.0 + cluster.len() as f64;
            centroid.iter_mut().for_each(|v| *v /= denom);
            centroid
        } else if cluster.is_empty() {
            mean.to_vec()
        } else {
            let mut centroid = cluster[0].clone();
            for vector in cluster.iter().skip(1) {
                for (idx, value) in vector.iter().enumerate() {
                    centroid[idx] += value;
                }
            }
            let denom = cluster.len() as f64;
            centroid.iter_mut().for_each(|v| *v /= denom);
            centroid
        }
    }

    fn run_single_trial(&mut self, vectors: &[Vec<f64>], trace: bool) {
        if self.num_means >= vectors.len() {
            if self.means.is_none() {
                self.means = Some(vectors.iter().take(self.num_means).cloned().collect());
            }
            return;
        }

        let mut converged = false;
        while !converged {
            let means = match self.means.as_ref() {
                Some(means) => means,
                None => return,
            };

            let mut clusters = vec![Vec::<Vec<f64>>::new(); self.num_means];
            for vector in vectors {
                let index = self.classify_vectorspace(vector);
                if let Some(cluster) = clusters.get_mut(index) {
                    cluster.push(vector.clone());
                }
            }

            if trace {
                println!("iteration");
            }

            let new_means: Vec<Vec<f64>> = clusters
                .iter()
                .zip(means.iter())
                .map(|(cluster, mean)| self.centroid(cluster, mean))
                .collect();

            let difference = self.sum_distances(means, &new_means);
            if difference < self.max_difference {
                converged = true;
            }

            self.means = Some(new_means);
        }
    }
}

impl Cluster for KMeansClusterer {
    type Label = usize;

    fn cluster(
        &mut self,
        vectors: &[Vec<f64>],
        assign_clusters: bool,
        trace: bool,
    ) -> Option<Vec<Self::Label>> {
        VectorSpaceClusterOps::cluster(self, vectors, assign_clusters, trace)
            .ok()
            .flatten()
    }

    fn classify(&self, vector: &[f64]) -> Self::Label {
        VectorSpaceClusterOps::classify(self, vector).unwrap_or(0)
    }

    fn num_clusters(&self) -> usize {
        self.means
            .as_ref()
            .map(|means| means.len())
            .unwrap_or(self.num_means)
    }

    fn cluster_names(&self) -> Vec<Self::Label> {
        (0..self.num_clusters()).collect()
    }

    fn cluster_name(&self, index: usize) -> Self::Label {
        index
    }
}

impl VectorSpaceCluster for KMeansClusterer {
    fn state(&self) -> &VectorSpaceState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut VectorSpaceState {
        &mut self.state
    }

    fn should_normalise(&self) -> bool {
        self.should_normalise
    }

    fn svd_dimensions(&self) -> Option<usize> {
        self.svd_dimensions
    }

    fn cluster_vectorspace(
        &mut self,
        vectors: &[Vec<f64>],
        trace: bool,
    ) -> Result<(), ClusterError> {
        if vectors.is_empty() {
            return Err(ClusterError::EmptyVectors);
        }

        if self.num_means == 0 {
            return Err(ClusterError::EmptyVectors);
        }

        if self.means.is_some() && self.repeats > 1 && trace {
            println!("Warning: means will be discarded for subsequent trials");
        }

        let mut meanss: Vec<Vec<Vec<f64>>> = Vec::new();
        for trial in 0..self.repeats {
            if trace {
                println!("k-means trial {}", trial);
            }

            if self.means.is_none() || trial > 0 {
                self.means = Some(
                    vectors
                        .choose_multiple(&mut self.rng, self.num_means)
                        .cloned()
                        .collect(),
                );
            }

            self.run_single_trial(vectors, trace);

            if let Some(means) = self.means.as_ref() {
                meanss.push(means.clone());
            }
        }

        if meanss.len() > 1 {
            for means in &mut meanss {
                means.sort_by(|a, b| {
                    a.iter()
                        .sum::<f64>()
                        .partial_cmp(&b.iter().sum::<f64>())
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }

            let mut min_difference: Option<f64> = None;
            let mut min_means: Option<Vec<Vec<f64>>> = None;

            for i in 0..meanss.len() {
                let mut d = 0.0;
                for j in 0..meanss.len() {
                    if i != j {
                        d += self.sum_distances(&meanss[i], &meanss[j]);
                    }
                }

                if min_difference.is_none() || d < min_difference.unwrap_or(f64::INFINITY) {
                    min_difference = Some(d);
                    min_means = Some(meanss[i].clone());
                }
            }

            self.means = min_means;
        }

        Ok(())
    }

    fn classify_vectorspace(&self, vector: &[f64]) -> usize {
        let mut best_distance: Option<f64> = None;
        let mut best_index = 0usize;

        if let Some(means) = self.means.as_ref() {
            for (index, mean) in means.iter().enumerate() {
                let dist = (self.distance)(vector, mean);
                if best_distance.is_none() || dist < best_distance.unwrap_or(f64::INFINITY) {
                    best_index = index;
                    best_distance = Some(dist);
                }
            }
        }

        best_index
    }
}

impl Default for KMeansClusterer {
    fn default() -> Self {
        Self::new(2, euclidean_distance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kmeans_clusters_simple_points() {
        let vectors = vec![
            vec![2.0, 1.0],
            vec![1.0, 3.0],
            vec![4.0, 7.0],
            vec![6.0, 7.0],
        ];

        let mut clusterer = KMeansClusterer::new(2, euclidean_distance)
            .with_initial_means(vec![vec![4.0, 3.0], vec![5.0, 5.0]])
            .with_repeats(1)
            .with_conv_test(1e-6);

        let clusters = Cluster::cluster(&mut clusterer, &vectors, true, false).unwrap();
        assert_eq!(clusters.len(), vectors.len());

        let c0 = Cluster::classify(&clusterer, &[2.0, 1.0]);
        let c1 = Cluster::classify(&clusterer, &[6.0, 7.0]);
        assert_ne!(c0, c1);
    }
}
