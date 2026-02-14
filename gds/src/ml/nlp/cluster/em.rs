use super::cluster::Cluster;
use super::util::{ClusterError, VectorSpaceCluster, VectorSpaceClusterOps, VectorSpaceState};

#[derive(Debug, Clone)]
pub struct EMClusterer {
    state: VectorSpaceState,
    should_normalise: bool,
    svd_dimensions: Option<usize>,
    means: Vec<Vec<f64>>,
    num_clusters: usize,
    conv_threshold: f64,
    covariance_matrices: Option<Vec<Vec<Vec<f64>>>>,
    priors: Option<Vec<f64>>,
    bias: f64,
}

impl EMClusterer {
    pub fn new(initial_means: Vec<Vec<f64>>) -> Self {
        let num_clusters = initial_means.len();
        Self {
            state: VectorSpaceState::new(),
            should_normalise: false,
            svd_dimensions: None,
            means: initial_means,
            num_clusters,
            conv_threshold: 1e-6,
            covariance_matrices: None,
            priors: None,
            bias: 0.1,
        }
    }

    pub fn with_priors(mut self, priors: Vec<f64>) -> Self {
        self.priors = Some(priors);
        self
    }

    pub fn with_covariance_matrices(mut self, covariance_matrices: Vec<Vec<Vec<f64>>>) -> Self {
        self.covariance_matrices = Some(covariance_matrices);
        self
    }

    pub fn with_conv_threshold(mut self, conv_threshold: f64) -> Self {
        self.conv_threshold = conv_threshold.max(0.0);
        self
    }

    pub fn with_bias(mut self, bias: f64) -> Self {
        self.bias = bias.max(0.0);
        self
    }

    pub fn with_normalise(mut self, normalise: bool) -> Self {
        self.should_normalise = normalise;
        self
    }

    pub fn with_svd_dimensions(mut self, svd_dimensions: Option<usize>) -> Self {
        self.svd_dimensions = svd_dimensions;
        self
    }

    pub fn means(&self) -> &[Vec<f64>] {
        &self.means
    }

    pub fn priors(&self) -> Option<&[f64]> {
        self.priors.as_deref()
    }

    pub fn covariance_matrices(&self) -> Option<&[Vec<Vec<f64>>]> {
        self.covariance_matrices.as_deref()
    }

    fn gaussian(&self, mean: &[f64], cvm: &[Vec<f64>], x: &[f64]) -> f64 {
        let m = mean.len();
        if cvm.len() != m || cvm.iter().any(|row| row.len() != m) {
            return 0.0;
        }

        let Some((inv, det)) = invert_and_det(cvm) else {
            return 0.0;
        };

        if det <= f64::EPSILON {
            return 0.0;
        }

        let a = det.powf(-0.5) * (2.0 * std::f64::consts::PI).powf(-(m as f64) / 2.0);
        let dx: Vec<f64> = x.iter().zip(mean.iter()).map(|(xi, mi)| xi - mi).collect();
        let b = -0.5 * quad_form(&dx, &inv);
        a * b.exp()
    }

    fn loglikelihood(
        &self,
        vectors: &[Vec<f64>],
        priors: &[f64],
        means: &[Vec<f64>],
        covariances: &[Vec<Vec<f64>>],
    ) -> f64 {
        let mut llh = 0.0;
        for vector in vectors {
            let mut p = 0.0;
            for j in 0..priors.len() {
                p += priors[j] * self.gaussian(&means[j], &covariances[j], vector);
            }
            llh += p.max(f64::MIN_POSITIVE).ln();
        }
        llh
    }
}

impl Cluster for EMClusterer {
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
        self.num_clusters
    }

    fn cluster_names(&self) -> Vec<Self::Label> {
        (0..self.num_clusters()).collect()
    }

    fn cluster_name(&self, index: usize) -> Self::Label {
        index
    }
}

impl VectorSpaceCluster for EMClusterer {
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

        let dimensions = vectors[0].len();
        let mut means = self.means.clone();

        let mut priors = self
            .priors
            .clone()
            .unwrap_or_else(|| vec![1.0 / self.num_clusters as f64; self.num_clusters]);

        let mut covariances = self.covariance_matrices.clone().unwrap_or_else(|| {
            (0..self.num_clusters)
                .map(|_| identity_matrix(dimensions))
                .collect()
        });

        let mut lastl = self.loglikelihood(vectors, &priors, &means, &covariances);
        let mut converged = false;

        while !converged {
            if trace {
                println!("iteration; loglikelihood {}", lastl);
            }

            let mut h = vec![vec![0.0; self.num_clusters]; vectors.len()];
            for i in 0..vectors.len() {
                for j in 0..self.num_clusters {
                    h[i][j] = priors[j] * self.gaussian(&means[j], &covariances[j], &vectors[i]);
                }
                let row_sum: f64 = h[i].iter().sum();
                if row_sum > f64::EPSILON {
                    for j in 0..self.num_clusters {
                        h[i][j] /= row_sum;
                    }
                } else {
                    for j in 0..self.num_clusters {
                        h[i][j] = 1.0 / self.num_clusters as f64;
                    }
                }
            }

            for j in 0..self.num_clusters {
                let mut new_covariance = vec![vec![0.0; dimensions]; dimensions];
                let mut new_mean = vec![0.0; dimensions];
                let mut sum_hj = 0.0;

                for i in 0..vectors.len() {
                    let delta: Vec<f64> = vectors[i]
                        .iter()
                        .zip(means[j].iter())
                        .map(|(a, b)| a - b)
                        .collect();

                    let outer = outer_product(&delta, &delta);
                    for r in 0..dimensions {
                        for c in 0..dimensions {
                            new_covariance[r][c] += h[i][j] * outer[r][c];
                        }
                    }

                    sum_hj += h[i][j];
                    for (idx, value) in vectors[i].iter().enumerate() {
                        new_mean[idx] += h[i][j] * value;
                    }
                }

                if sum_hj > f64::EPSILON {
                    for r in 0..dimensions {
                        for c in 0..dimensions {
                            new_covariance[r][c] /= sum_hj;
                        }
                    }

                    for value in &mut new_mean {
                        *value /= sum_hj;
                    }

                    for d in 0..dimensions {
                        new_covariance[d][d] += self.bias;
                    }

                    covariances[j] = new_covariance;
                    means[j] = new_mean;
                    priors[j] = sum_hj / vectors.len() as f64;
                }
            }

            let l = self.loglikelihood(vectors, &priors, &means, &covariances);
            if (lastl - l).abs() < self.conv_threshold {
                converged = true;
            }
            lastl = l;
        }

        self.means = means;
        self.priors = Some(priors);
        self.covariance_matrices = Some(covariances);
        Ok(())
    }

    fn classify_vectorspace(&self, vector: &[f64]) -> usize {
        let mut best = (f64::NEG_INFINITY, 0usize);

        let Some(priors) = self.priors.as_ref() else {
            return 0;
        };
        let Some(covariances) = self.covariance_matrices.as_ref() else {
            return 0;
        };

        for j in 0..self.num_clusters {
            let p = priors[j] * self.gaussian(&self.means[j], &covariances[j], vector);
            if p > best.0 {
                best = (p, j);
            }
        }
        best.1
    }

    fn likelihood_vectorspace(&self, vector: &[f64], cluster: usize) -> f64 {
        let Some(priors) = self.priors.as_ref() else {
            return 0.0;
        };
        let Some(covariances) = self.covariance_matrices.as_ref() else {
            return 0.0;
        };

        if cluster >= self.num_clusters {
            return 0.0;
        }

        priors[cluster] * self.gaussian(&self.means[cluster], &covariances[cluster], vector)
    }
}

fn identity_matrix(dim: usize) -> Vec<Vec<f64>> {
    let mut matrix = vec![vec![0.0; dim]; dim];
    for (i, row) in matrix.iter_mut().enumerate().take(dim) {
        row[i] = 1.0;
    }
    matrix
}

fn outer_product(a: &[f64], b: &[f64]) -> Vec<Vec<f64>> {
    let mut out = vec![vec![0.0; b.len()]; a.len()];
    for (i, ai) in a.iter().enumerate() {
        for (j, bj) in b.iter().enumerate() {
            out[i][j] = ai * bj;
        }
    }
    out
}

fn quad_form(v: &[f64], m: &[Vec<f64>]) -> f64 {
    let mut temp = vec![0.0; v.len()];
    for (i, row) in m.iter().enumerate() {
        temp[i] = row.iter().zip(v.iter()).map(|(a, b)| a * b).sum();
    }
    v.iter().zip(temp.iter()).map(|(a, b)| a * b).sum()
}

fn invert_and_det(matrix: &[Vec<f64>]) -> Option<(Vec<Vec<f64>>, f64)> {
    let n = matrix.len();
    if n == 0 || matrix.iter().any(|row| row.len() != n) {
        return None;
    }

    let mut a = matrix.to_vec();
    let mut inv = identity_matrix(n);
    let mut det = 1.0;

    for i in 0..n {
        let mut pivot_row = i;
        let mut pivot_abs = a[i][i].abs();
        for (r, row) in a.iter().enumerate().take(n).skip(i + 1) {
            let candidate = row[i].abs();
            if candidate > pivot_abs {
                pivot_abs = candidate;
                pivot_row = r;
            }
        }

        if pivot_abs <= f64::EPSILON {
            return None;
        }

        if pivot_row != i {
            a.swap(i, pivot_row);
            inv.swap(i, pivot_row);
            det = -det;
        }

        let pivot = a[i][i];
        det *= pivot;

        for c in 0..n {
            a[i][c] /= pivot;
            inv[i][c] /= pivot;
        }

        for r in 0..n {
            if r == i {
                continue;
            }
            let factor = a[r][i];
            if factor.abs() <= f64::EPSILON {
                continue;
            }
            for c in 0..n {
                a[r][c] -= factor * a[i][c];
                inv[r][c] -= factor * inv[i][c];
            }
        }
    }

    Some((inv, det))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn em_clusters_simple_data() {
        let vectors = vec![vec![0.5, 0.5], vec![1.5, 0.5], vec![1.0, 3.0]];
        let means = vec![vec![4.0, 2.0], vec![4.0, 2.01]];

        let mut clusterer = EMClusterer::new(means)
            .with_bias(0.1)
            .with_conv_threshold(1e-5);
        let clusters = Cluster::cluster(&mut clusterer, &vectors, true, false).unwrap();

        assert_eq!(clusters.len(), vectors.len());
        assert!(clusterer.priors().is_some());
        assert!(clusterer.covariance_matrices().is_some());

        let predicted = Cluster::classify(&clusterer, &[2.0, 2.0]);
        assert!(predicted < clusterer.num_clusters());
    }
}
