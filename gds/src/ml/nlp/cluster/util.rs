use std::cmp::Ordering;

use super::cluster::Cluster;

#[derive(Debug, Clone, PartialEq)]
pub enum ClusterError {
    EmptyVectors,
    InconsistentDimensions,
    InvalidSvdDimensions,
    EmptyVector,
}

#[derive(Debug, Clone, Default)]
pub struct VectorSpaceState {
    projection_dimensions: Option<usize>,
}

impl VectorSpaceState {
    pub fn new() -> Self {
        Self {
            projection_dimensions: None,
        }
    }

    pub fn preprocess_vectors(
        &mut self,
        vectors: &[Vec<f64>],
        should_normalise: bool,
        svd_dimensions: Option<usize>,
    ) -> Result<Vec<Vec<f64>>, ClusterError> {
        validate_vectors(vectors)?;

        let mut transformed = vectors.to_vec();

        if should_normalise {
            transformed = transformed
                .iter()
                .map(|v| normalise(v))
                .collect::<Result<Vec<_>, _>>()?;
        }

        self.projection_dimensions = None;

        if let Some(dimensions) = svd_dimensions {
            if dimensions == 0 {
                return Err(ClusterError::InvalidSvdDimensions);
            }

            let source_dimensions = transformed[0].len();
            if dimensions > source_dimensions {
                return Err(ClusterError::InvalidSvdDimensions);
            }

            if dimensions < source_dimensions {
                transformed = transformed
                    .into_iter()
                    .map(|v| v.into_iter().take(dimensions).collect())
                    .collect();
                self.projection_dimensions = Some(dimensions);
            }
        }

        Ok(transformed)
    }

    pub fn transform_vector(
        &self,
        vector: &[f64],
        should_normalise: bool,
    ) -> Result<Vec<f64>, ClusterError> {
        if vector.is_empty() {
            return Err(ClusterError::EmptyVector);
        }

        let mut transformed = vector.to_vec();
        if should_normalise {
            transformed = normalise(&transformed)?;
        }

        if let Some(dimensions) = self.projection_dimensions {
            if transformed.len() < dimensions {
                return Err(ClusterError::InconsistentDimensions);
            }
            transformed.truncate(dimensions);
        }

        Ok(transformed)
    }
}

pub trait VectorSpaceCluster: Cluster<Label = usize> {
    fn state(&self) -> &VectorSpaceState;
    fn state_mut(&mut self) -> &mut VectorSpaceState;
    fn should_normalise(&self) -> bool;
    fn svd_dimensions(&self) -> Option<usize>;

    fn cluster_vectorspace(
        &mut self,
        vectors: &[Vec<f64>],
        trace: bool,
    ) -> Result<(), ClusterError>;
    fn classify_vectorspace(&self, vector: &[f64]) -> usize;

    fn likelihood_vectorspace(&self, vector: &[f64], cluster: usize) -> f64 {
        let predicted = self.classify_vectorspace(vector);
        if cluster == predicted {
            1.0
        } else {
            0.0
        }
    }

    fn vector(&self, vector: &[f64]) -> Result<Vec<f64>, ClusterError> {
        self.state()
            .transform_vector(vector, self.should_normalise())
    }
}

pub trait VectorSpaceClusterOps: VectorSpaceCluster {
    fn cluster(
        &mut self,
        vectors: &[Vec<f64>],
        assign_clusters: bool,
        trace: bool,
    ) -> Result<Option<Vec<usize>>, ClusterError> {
        let should_normalise = self.should_normalise();
        let svd_dimensions = self.svd_dimensions();
        let transformed =
            self.state_mut()
                .preprocess_vectors(vectors, should_normalise, svd_dimensions)?;

        self.cluster_vectorspace(&transformed, trace)?;

        if assign_clusters {
            let mut assignments = Vec::with_capacity(transformed.len());
            for vector in &transformed {
                assignments.push(self.cluster_name(self.classify_vectorspace(vector)));
            }
            Ok(Some(assignments))
        } else {
            Ok(None)
        }
    }

    fn classify(&self, vector: &[f64]) -> Result<usize, ClusterError> {
        let transformed = self
            .state()
            .transform_vector(vector, self.should_normalise())?;
        Ok(self.cluster_name(self.classify_vectorspace(&transformed)))
    }

    fn likelihood(&self, vector: &[f64], label: usize) -> Result<f64, ClusterError> {
        let transformed = self
            .state()
            .transform_vector(vector, self.should_normalise())?;
        Ok(self.likelihood_vectorspace(&transformed, label))
    }
}

impl<T> VectorSpaceClusterOps for T where T: VectorSpaceCluster {}

pub fn normalise(vector: &[f64]) -> Result<Vec<f64>, ClusterError> {
    if vector.is_empty() {
        return Err(ClusterError::EmptyVector);
    }
    let norm_sq = dot(vector, vector);
    if norm_sq <= f64::EPSILON {
        return Err(ClusterError::EmptyVector);
    }

    let norm = norm_sq.sqrt();
    Ok(vector.iter().map(|v| v / norm).collect())
}

pub fn dot(u: &[f64], v: &[f64]) -> f64 {
    u.iter().zip(v.iter()).map(|(a, b)| a * b).sum()
}

pub fn euclidean_distance(u: &[f64], v: &[f64]) -> f64 {
    let diff_norm_sq: f64 = u
        .iter()
        .zip(v.iter())
        .map(|(a, b)| {
            let d = a - b;
            d * d
        })
        .sum();
    diff_norm_sq.sqrt()
}

pub fn cosine_distance(u: &[f64], v: &[f64]) -> f64 {
    let denominator = dot(u, u).sqrt() * dot(v, v).sqrt();
    if denominator <= f64::EPSILON {
        1.0
    } else {
        1.0 - (dot(u, v) / denominator)
    }
}

#[derive(Debug, Clone)]
struct DendrogramNode<T>
where
    T: Clone,
{
    value: usize,
    leaf: Option<T>,
    children: Vec<DendrogramNode<T>>,
}

impl<T> DendrogramNode<T>
where
    T: Clone,
{
    fn leaf(value: T) -> Self {
        Self {
            value: 0,
            leaf: Some(value),
            children: Vec::new(),
        }
    }

    fn merge(value: usize, children: Vec<DendrogramNode<T>>) -> Self {
        Self {
            value,
            leaf: None,
            children,
        }
    }

    fn leaves(&self) -> Vec<T> {
        if self.children.is_empty() {
            return self.leaf.iter().cloned().collect();
        }

        let mut out = Vec::new();
        for child in &self.children {
            out.extend(child.leaves());
        }
        out
    }

    fn groups(&self, n: usize) -> Vec<Vec<T>> {
        let target = n.max(1);
        let mut queue: Vec<(usize, DendrogramNode<T>)> = vec![(self.value, self.clone())];

        while queue.len() < target {
            queue.sort_by(|(pa, _), (pb, _)| pa.cmp(pb));
            let (_, node) = match queue.pop() {
                Some(item) => item,
                None => break,
            };

            if node.children.is_empty() {
                queue.push((node.value, node));
                break;
            }

            for child in node.children {
                let priority = if child.children.is_empty() {
                    0
                } else {
                    child.value
                };
                queue.push((priority, child));
            }
        }

        queue.into_iter().map(|(_, node)| node.leaves()).collect()
    }
}

#[derive(Debug, Clone)]
pub struct Dendrogram<T>
where
    T: Clone,
{
    items: Vec<DendrogramNode<T>>,
    original_items: Vec<DendrogramNode<T>>,
    merge_value: usize,
}

impl<T> Dendrogram<T>
where
    T: Clone,
{
    pub fn new(items: Vec<T>) -> Self {
        let nodes: Vec<DendrogramNode<T>> = items.into_iter().map(DendrogramNode::leaf).collect();
        Self {
            items: nodes.clone(),
            original_items: nodes,
            merge_value: 1,
        }
    }

    pub fn merge(&mut self, indices: &[usize]) {
        assert!(indices.len() >= 2);

        let children: Vec<DendrogramNode<T>> =
            indices.iter().map(|&i| self.items[i].clone()).collect();
        let node = DendrogramNode::merge(self.merge_value, children);
        self.merge_value += 1;

        self.items[indices[0]] = node;
        for &idx in indices.iter().skip(1).rev() {
            self.items.remove(idx);
        }
    }

    pub fn groups(&self, n: usize) -> Vec<Vec<T>> {
        if self.items.len() > 1 {
            let root = DendrogramNode::merge(self.merge_value, self.items.clone());
            root.groups(n)
        } else if let Some(root) = self.items.first() {
            root.groups(n)
        } else {
            Vec::new()
        }
    }

    pub fn ascii_rows(&self, leaf_labels: Option<Vec<String>>) -> Vec<String> {
        if self.original_items.is_empty() {
            return vec![String::new()];
        }

        let labels = if let Some(input) = leaf_labels {
            input
        } else {
            self.original_items
                .iter()
                .enumerate()
                .map(|(i, _)| i.to_string())
                .collect()
        };

        vec![labels.join(" ")]
    }

    pub fn len(&self) -> usize {
        self.original_items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.original_items.is_empty()
    }
}

pub fn validate_vectors(vectors: &[Vec<f64>]) -> Result<(), ClusterError> {
    if vectors.is_empty() {
        return Err(ClusterError::EmptyVectors);
    }

    let dimensions = vectors[0].len();
    if dimensions == 0 {
        return Err(ClusterError::EmptyVector);
    }

    if vectors.iter().any(|v| v.len() != dimensions) {
        return Err(ClusterError::InconsistentDimensions);
    }

    Ok(())
}

pub fn argmin(values: &[f64]) -> Option<usize> {
    values
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .map(|(idx, _)| idx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ml::nlp::cluster::cluster::Cluster;

    #[derive(Debug, Clone)]
    struct DummyVectorClusterer {
        state: VectorSpaceState,
        normalise: bool,
        svd_dimensions: Option<usize>,
        centroids: Vec<Vec<f64>>,
    }

    impl DummyVectorClusterer {
        fn new(normalise: bool, svd_dimensions: Option<usize>, centroids: Vec<Vec<f64>>) -> Self {
            Self {
                state: VectorSpaceState::new(),
                normalise,
                svd_dimensions,
                centroids,
            }
        }
    }

    impl Cluster for DummyVectorClusterer {
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
            self.centroids.len()
        }

        fn cluster_names(&self) -> Vec<Self::Label> {
            (0..self.num_clusters()).collect()
        }

        fn cluster_name(&self, index: usize) -> Self::Label {
            index
        }
    }

    impl VectorSpaceCluster for DummyVectorClusterer {
        fn state(&self) -> &VectorSpaceState {
            &self.state
        }

        fn state_mut(&mut self) -> &mut VectorSpaceState {
            &mut self.state
        }

        fn should_normalise(&self) -> bool {
            self.normalise
        }

        fn svd_dimensions(&self) -> Option<usize> {
            self.svd_dimensions
        }

        fn cluster_vectorspace(
            &mut self,
            _vectors: &[Vec<f64>],
            _trace: bool,
        ) -> Result<(), ClusterError> {
            Ok(())
        }

        fn classify_vectorspace(&self, vector: &[f64]) -> usize {
            self.centroids
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| {
                    euclidean_distance(vector, a)
                        .partial_cmp(&euclidean_distance(vector, b))
                        .unwrap_or(Ordering::Equal)
                })
                .map(|(idx, _)| idx)
                .unwrap_or(0)
        }
    }

    #[test]
    fn distance_functions_behave() {
        assert!((euclidean_distance(&[0.0, 0.0], &[3.0, 4.0]) - 5.0).abs() < 1e-12);
        assert!(cosine_distance(&[1.0, 0.0], &[1.0, 0.0]).abs() < 1e-12);
        assert!((cosine_distance(&[1.0, 0.0], &[0.0, 1.0]) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn vector_space_pipeline_assigns_clusters() {
        let mut clusterer =
            DummyVectorClusterer::new(false, Some(2), vec![vec![0.0, 0.0], vec![1.0, 1.0]]);
        let vectors = vec![vec![0.1, 0.2, 9.0], vec![1.2, 1.1, -3.0]];

        let assignments = VectorSpaceClusterOps::cluster(&mut clusterer, &vectors, true, false)
            .unwrap()
            .unwrap();

        assert_eq!(assignments, vec![0, 1]);
    }

    #[test]
    fn dendrogram_groups_are_constructed() {
        let mut d = Dendrogram::new(vec![1, 2, 3]);
        d.merge(&[0, 1]);
        let groups = d.groups(2);
        assert_eq!(groups.len(), 2);
    }
}
