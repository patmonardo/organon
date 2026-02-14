use super::cluster::Cluster;
use super::util::{
    cosine_distance, normalise, ClusterError, Dendrogram, VectorSpaceCluster,
    VectorSpaceClusterOps, VectorSpaceState,
};

#[derive(Debug, Clone)]
pub struct GAAClusterer {
    state: VectorSpaceState,
    should_normalise: bool,
    svd_dimensions: Option<usize>,
    num_clusters: usize,
    dendrogram: Option<Dendrogram<Vec<f64>>>,
    centroids: Vec<Vec<f64>>,
}

impl GAAClusterer {
    pub fn new(num_clusters: usize) -> Self {
        Self {
            state: VectorSpaceState::new(),
            should_normalise: true,
            svd_dimensions: None,
            num_clusters,
            dendrogram: None,
            centroids: Vec::new(),
        }
    }

    pub fn with_normalise(mut self, normalise: bool) -> Self {
        self.should_normalise = normalise;
        self
    }

    pub fn with_svd_dimensions(mut self, dimensions: Option<usize>) -> Self {
        self.svd_dimensions = dimensions;
        self
    }

    pub fn dendrogram(&self) -> Option<&Dendrogram<Vec<f64>>> {
        self.dendrogram.as_ref()
    }

    fn merge_similarities(dist: &mut [Vec<f64>], cluster_len: &[usize], i: usize, j: usize) {
        let i_weight = cluster_len[i] as f64;
        let j_weight = cluster_len[j] as f64;
        let weight_sum = i_weight + j_weight;

        for x in 0..i {
            dist[x][i] = (dist[x][i] * i_weight + dist[x][j] * j_weight) / weight_sum;
        }

        for x in (i + 1)..j {
            dist[i][x] = (dist[i][x] * i_weight + dist[x][j] * j_weight) / weight_sum;
        }

        for x in (j + 1)..dist.len() {
            dist[i][x] = (dist[i][x] * i_weight + dist[j][x] * j_weight) / weight_sum;
        }
    }

    fn update_clusters(&mut self, target_clusters: usize) {
        let clusters = self
            .dendrogram
            .as_ref()
            .map(|d| d.groups(target_clusters))
            .unwrap_or_default();

        self.centroids.clear();
        for cluster in clusters {
            if cluster.is_empty() {
                continue;
            }

            let mut centroid = if self.should_normalise {
                normalise(&cluster[0]).unwrap_or_else(|_| cluster[0].clone())
            } else {
                cluster[0].clone()
            };

            for vector in cluster.iter().skip(1) {
                let source = if self.should_normalise {
                    normalise(vector).unwrap_or_else(|_| vector.clone())
                } else {
                    vector.clone()
                };
                for (idx, value) in source.iter().enumerate() {
                    centroid[idx] += value;
                }
            }

            let denom = cluster.len() as f64;
            centroid.iter_mut().for_each(|v| *v /= denom);
            self.centroids.push(centroid);
        }

        self.num_clusters = self.centroids.len();
    }
}

impl Cluster for GAAClusterer {
    type Label = usize;

    fn cluster(
        &mut self,
        vectors: &[Vec<f64>],
        assign_clusters: bool,
        trace: bool,
    ) -> Option<Vec<Self::Label>> {
        self.dendrogram = Some(Dendrogram::new(vectors.to_vec()));
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

impl VectorSpaceCluster for GAAClusterer {
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
        let n = vectors.len();
        if n == 0 {
            return Err(ClusterError::EmptyVectors);
        }

        let mut cluster_len = vec![1usize; n];
        let mut cluster_count = n;
        let mut index_map: Vec<usize> = (0..n).collect();

        let mut dist = vec![vec![f64::INFINITY; n]; n];
        for i in 0..n {
            for j in (i + 1)..n {
                dist[i][j] = cosine_distance(&vectors[i], &vectors[j]);
            }
        }

        while cluster_count > self.num_clusters.max(1) {
            let mut best = (0usize, 0usize, f64::INFINITY);
            for i in 0..n {
                for j in (i + 1)..n {
                    if dist[i][j] < best.2 {
                        best = (i, j, dist[i][j]);
                    }
                }
            }

            let (i, j, _) = best;
            if trace {
                println!("merging {} and {}", i, j);
            }

            Self::merge_similarities(&mut dist, &cluster_len, i, j);

            for row in dist.iter_mut().take(n) {
                row[j] = f64::INFINITY;
            }
            for col in 0..n {
                dist[j][col] = f64::INFINITY;
            }

            cluster_len[i] += cluster_len[j];
            if let Some(d) = self.dendrogram.as_mut() {
                d.merge(&[index_map[i], index_map[j]]);
            }
            cluster_count -= 1;

            for value in index_map.iter_mut().take(n).skip(j + 1) {
                *value = value.saturating_sub(1);
            }
            index_map[j] = n;
        }

        self.update_clusters(self.num_clusters.max(1));
        Ok(())
    }

    fn classify_vectorspace(&self, vector: &[f64]) -> usize {
        let mut best = (f64::INFINITY, 0usize);
        for (i, centroid) in self.centroids.iter().enumerate() {
            let dist = cosine_distance(vector, centroid);
            if dist < best.0 {
                best = (dist, i);
            }
        }
        best.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gaac_clusters_demo_vectors() {
        let vectors = vec![
            vec![3.0, 3.0],
            vec![1.0, 2.0],
            vec![4.0, 2.0],
            vec![4.0, 0.0],
            vec![2.0, 3.0],
            vec![3.0, 1.0],
        ];

        let mut clusterer = GAAClusterer::new(4);
        let clusters = Cluster::cluster(&mut clusterer, &vectors, true, false).unwrap();

        assert_eq!(clusters.len(), vectors.len());
        assert_eq!(clusterer.num_clusters(), 4);
        assert!(clusterer.dendrogram().is_some());
    }
}
