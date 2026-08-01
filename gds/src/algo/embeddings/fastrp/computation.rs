//! FastRP computation runtime.
//!
//! This is the **Subtle pole**: ephemeral computation state for FastRP.

use crate::ml::core::features::{feature_extraction, AnyFeatureExtractor, FeatureConsumer};
use crate::ml::core::tensor::operations::vector_operations::{
    add_in_place, add_weighted_in_place, l2_norm, scale,
};
use crate::projection::eval::algorithm::AlgorithmError;
use crate::task::concurrency::virtual_threads::Executor;
use crate::task::concurrency::{Concurrency, TerminationFlag};
use crate::task::progress::{NoopProgressTracker, ProgressTracker};
use crate::types::graph::Graph;
use crate::types::graph::MappedNodeId;
use std::sync::Arc;

use super::spec::{FastRPConfig, FastRPResult};

/// Runs the FastRP algorithm.
#[derive(Debug, Default, Clone)]
pub struct FastRPComputationRuntime;

impl FastRPComputationRuntime {
    pub fn run(
        graph: Arc<dyn Graph>,
        config: &FastRPConfig,
        feature_extractors: Vec<AnyFeatureExtractor>,
    ) -> Result<FastRPResult, AlgorithmError> {
        Self::run_with_controls(
            graph,
            config,
            feature_extractors,
            &mut NoopProgressTracker,
            &TerminationFlag::running_true(),
        )
    }

    pub fn run_with_controls(
        graph: Arc<dyn Graph>,
        config: &FastRPConfig,
        feature_extractors: Vec<AnyFeatureExtractor>,
        progress_tracker: &mut dyn ProgressTracker,
        termination_flag: &TerminationFlag,
    ) -> Result<FastRPResult, AlgorithmError> {
        let parameters = FastRPParameters {
            iteration_weights: config.iteration_weights.clone(),
            embedding_dimension: config.embedding_dimension,
            property_dimension: config.property_dimension,
            relationship_weight_property: config.relationship_weight_property.clone(),
            normalization_strength: config.normalization_strength,
            node_self_influence: config.node_self_influence,
        };

        let algo = FastRP::new(
            graph,
            parameters,
            Concurrency::of(config.concurrency.max(1)),
            config.min_batch_size,
            feature_extractors,
            config.random_seed,
            termination_flag.clone(),
        );

        let result = algo.compute(progress_tracker)?;
        Ok(FastRPResult {
            embeddings: result.embeddings.to_vec(),
        })
    }
}

// =============================================================================
// Implementation (ported from the earlier literal translation)
// =============================================================================

#[derive(Debug, Clone)]
struct FastRPParameters {
    iteration_weights: Vec<f32>,
    embedding_dimension: usize,
    property_dimension: usize,
    relationship_weight_property: Option<String>,
    normalization_strength: f32,
    node_self_influence: f32,
}

/// Fast Random Projection (FastRP) node embeddings.
///
/// Internal implementation type used by the computation runtime.
struct FastRP {
    graph: Arc<dyn Graph>,
    parameters: FastRPParameters,
    executor: Executor,
    _min_batch_size: usize,
    feature_extractors: Vec<AnyFeatureExtractor>,
    random_seed: u64,
    termination_flag: TerminationFlag,
}

impl FastRP {
    const SPARSITY: i32 = 3;
    const ENTRY_PROBABILITY: f64 = 1.0 / (2.0 * Self::SPARSITY as f64);
    // Java: EPSILON = 10f / Float.MAX_VALUE
    const EPSILON: f64 = 10.0 / f64::MAX;

    #[allow(clippy::too_many_arguments)]
    fn new(
        graph: Arc<dyn Graph>,
        parameters: FastRPParameters,
        concurrency: Concurrency,
        min_batch_size: usize,
        feature_extractors: Vec<AnyFeatureExtractor>,
        random_seed: Option<u64>,
        termination_flag: TerminationFlag,
    ) -> Self {
        let seed = random_seed.unwrap_or(42);
        let improved = HighQualityRandom::new(seed).next_u64();

        Self {
            graph,
            parameters,
            executor: Executor::new(concurrency),
            _min_batch_size: min_batch_size,
            feature_extractors,
            random_seed: improved,
            termination_flag,
        }
    }

    fn compute(
        self,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<FastRPInternalResult, AlgorithmError> {
        let embedding_dimension = self.parameters.embedding_dimension;
        let base_embedding_dimension =
            embedding_dimension.saturating_sub(self.parameters.property_dimension);

        let feature_dim = feature_extraction::feature_count(&self.feature_extractors);
        let property_vectors = self.init_property_vectors(feature_dim);

        let node_count = self.graph.node_count();
        let embedding_b = self.init_random_vectors(
            base_embedding_dimension,
            embedding_dimension,
            &property_vectors,
        )?;
        progress_tracker.log_progress(node_count);

        let mut embeddings = self.add_initial_vectors_to_embedding(&embedding_b)?;
        if self.parameters.node_self_influence != 0.0 {
            progress_tracker.log_progress(node_count);
        }

        self.propagate_embeddings(&mut embeddings, embedding_b, progress_tracker)?;

        Ok(FastRPInternalResult { embeddings })
    }

    fn init_property_vectors(&self, feature_dim: usize) -> Vec<Vec<f64>> {
        let property_dimension = self.parameters.property_dimension;
        if property_dimension == 0 || feature_dim == 0 {
            return vec![vec![]; feature_dim];
        }

        let entry_value =
            (Self::SPARSITY as f64).sqrt() / (self.parameters.embedding_dimension as f64).sqrt();

        let mut random = HighQualityRandom::new(self.random_seed);
        let mut property_vectors = vec![vec![0.0; property_dimension]; feature_dim];

        for d in 0..property_dimension {
            for pv in property_vectors.iter_mut().take(feature_dim) {
                pv[d] = compute_random_entry(&mut random, entry_value);
            }
        }

        property_vectors
    }

    fn init_random_vectors(
        &self,
        base_embedding_dimension: usize,
        embedding_dimension: usize,
        property_vectors: &[Vec<f64>],
    ) -> Result<Vec<Vec<f64>>, AlgorithmError> {
        let sqrt_embedding_dimension = (embedding_dimension as f64).sqrt();
        let sqrt_sparsity = (Self::SPARSITY as f64).sqrt();

        self.executor
            .parallel_map(0, self.graph.node_count(), &self.termination_flag, |node| {
                let node_id = MappedNodeId::try_from(node)
                    .expect("graph node count must fit the mapped ID domain");

                let degree = self.graph.degree(node_id);
                let scaling = if degree == 0 {
                    1.0
                } else {
                    (degree as f64).powf(self.parameters.normalization_strength as f64)
                };
                let entry_value = scaling * sqrt_sparsity / sqrt_embedding_dimension;

                let original_id = self
                    .graph
                    .to_original_node_id(node_id)
                    .expect("mapped graph node must have an original ID");
                let original_seed = u64::from_ne_bytes(original_id.get().to_ne_bytes());
                let mut random = HighQualityRandom::new(self.random_seed ^ original_seed);

                let mut vec = vec![0.0; embedding_dimension];
                for (_i, val) in vec.iter_mut().enumerate().take(base_embedding_dimension) {
                    *val = compute_random_entry(&mut random, entry_value);
                }

                // Property feature contribution for tail dimensions.
                if self.parameters.property_dimension > 0 && !property_vectors.is_empty() {
                    let mut adder = PropertyVectorAdder::new(
                        base_embedding_dimension,
                        embedding_dimension,
                        property_vectors,
                        &mut vec,
                    );
                    feature_extraction::extract(
                        node_id.get(),
                        0,
                        &self.feature_extractors,
                        &mut adder,
                    );
                }

                vec
            })
            .map_err(|_| AlgorithmError::Execution("FastRP initialization terminated".to_string()))
    }

    fn add_initial_vectors_to_embedding(
        &self,
        embedding_b: &[Vec<f64>],
    ) -> Result<Vec<Vec<f64>>, AlgorithmError> {
        let influence = self.parameters.node_self_influence as f64;
        self.executor
            .parallel_map(0, self.graph.node_count(), &self.termination_flag, |node| {
                let initial = &embedding_b[node];
                let mut embedding = vec![0.0; self.parameters.embedding_dimension];
                if influence != 0.0 {
                    let norm = l2_norm(initial);
                    let adjusted = if norm < Self::EPSILON { 1.0 } else { norm };
                    add_weighted_in_place(&mut embedding, initial, influence / adjusted);
                }
                embedding
            })
            .map_err(|_| AlgorithmError::Execution("FastRP self-influence terminated".to_string()))
    }

    fn propagate_embeddings(
        &self,
        embeddings: &mut [Vec<f64>],
        mut previous: Vec<Vec<f64>>,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<(), AlgorithmError> {
        let relationship_weight_fallback = if self.parameters.relationship_weight_property.is_some()
        {
            f64::NAN
        } else {
            1.0
        };

        for (i, &iteration_weight) in self.parameters.iteration_weights.iter().enumerate() {
            self.termination_flag.assert_running();
            let first_iteration = i == 0;
            let rows = self
                .executor
                .parallel_map(0, self.graph.node_count(), &self.termination_flag, |node| {
                let node_id = MappedNodeId::try_from(node)
                    .expect("graph node count must fit the mapped ID domain");
                let mut current = vec![0.0; self.parameters.embedding_dimension];
                let mut relationships_processed = 0usize;

                if self.graph.has_relationship_property() {
                    for cursor in self
                        .graph
                        .stream_relationships_weighted(node_id, relationship_weight_fallback)
                    {
                        relationships_processed += 1;
                        let weight = cursor.weight();
                        if first_iteration && weight.is_nan() {
                            let source = cursor.source_id();
                            let target = cursor.target_id();
                            let source_orig =
                                self.graph.to_original_node_id(source).expect(
                                    "relationship source must have an original node ID",
                                );
                            let target_orig =
                                self.graph.to_original_node_id(target).expect(
                                    "relationship target must have an original node ID",
                                );

                            return Err(AlgorithmError::InvalidGraph(format!(
                                "Missing relationship property `{}` on relationship between nodes with ids `{}` and `{}`.",
                                self.parameters
                                    .relationship_weight_property
                                    .as_deref()
                                    .unwrap_or(""),
                                source_orig,
                                target_orig
                            )));
                        }

                        let target_idx = cursor
                            .target_id()
                            .to_usize()
                            .expect("mapped node ID must fit embedding storage");
                        add_weighted_in_place(&mut current, &previous[target_idx], weight);
                    }
                } else {
                    for cursor in self
                        .graph
                        .stream_relationships(node_id, relationship_weight_fallback)
                    {
                        relationships_processed += 1;
                        let target_idx = cursor
                            .target_id()
                            .to_usize()
                            .expect("mapped node ID must fit embedding storage");
                        add_in_place(&mut current, &previous[target_idx]);
                    }
                }

                let degree = self.graph.degree(node_id);
                let adjusted_degree = if degree == 0 { 1 } else { degree };
                let degree_scale = 1.0 / adjusted_degree as f64;
                scale(&mut current, degree_scale);

                let inv_l2 = 1.0 / l2_norm(&current);
                let safe_inv = if inv_l2.is_finite() { inv_l2 } else { 1.0 };
                Ok((current, relationships_processed, safe_inv))
            })
            .map_err(|_| AlgorithmError::Execution("FastRP propagation terminated".to_string()))?;

            let rows = rows
                .into_iter()
                .collect::<Result<Vec<_>, AlgorithmError>>()?;
            let relationships_processed = rows.iter().map(|(_, count, _)| count).sum();
            for (node, (current, _, safe_inv)) in rows.into_iter().enumerate() {
                add_weighted_in_place(
                    &mut embeddings[node],
                    &current,
                    safe_inv * iteration_weight as f64,
                );
                previous[node] = current;
            }
            progress_tracker.log_progress(relationships_processed);
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
struct FastRPInternalResult {
    embeddings: Vec<Vec<f64>>,
}

// =============================================================================
// Feature consumer: adds projected property vectors into the embedding tail.
// =============================================================================

struct PropertyVectorAdder<'a> {
    base_embedding_dimension: usize,
    embedding_dimension: usize,
    property_vectors: &'a [Vec<f64>],
    output: &'a mut [f64],
}

impl<'a> PropertyVectorAdder<'a> {
    fn new(
        base_embedding_dimension: usize,
        embedding_dimension: usize,
        property_vectors: &'a [Vec<f64>],
        output: &'a mut [f64],
    ) -> Self {
        Self {
            base_embedding_dimension,
            embedding_dimension,
            property_vectors,
            output,
        }
    }
}

impl FeatureConsumer for PropertyVectorAdder<'_> {
    fn accept_scalar(&mut self, _node_offset: u64, offset: usize, value: f64) {
        let d = self.base_embedding_dimension + offset;
        if d >= self.embedding_dimension {
            return;
        }

        // Equivalent to Java: addPropertyVector( offset, value )
        let pv = &self.property_vectors[offset];
        for (i, &pv_val) in pv.iter().enumerate() {
            let idx = self.base_embedding_dimension + i;
            if idx < self.embedding_dimension {
                self.output[idx] = value.mul_add(pv_val, self.output[idx]);
            }
        }
    }

    fn accept_array(&mut self, _node_offset: u64, mut offset: usize, values: &[f64]) {
        for &value in values {
            self.accept_scalar(0, offset, value);
            offset += 1;
        }
    }
}

// =============================================================================
// Random number generation utilities (Java parity)
// =============================================================================

/// SplitMix64-based generator (good statistical quality, cheap).
///
/// Matches the Java GDS "HighQualityRandom" usage pattern.
struct HighQualityRandom {
    state: u64,
}

impl HighQualityRandom {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u64(&mut self) -> u64 {
        // SplitMix64
        self.state = self.state.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }

    fn next_f64(&mut self) -> f64 {
        let x = self.next_u64();
        // 53-bit mantissa
        ((x >> 11) as f64) * (1.0 / ((1u64 << 53) as f64))
    }
}

#[inline]
fn compute_random_entry(random: &mut HighQualityRandom, entry_value: f64) -> f64 {
    let r = random.next_f64();

    if r < FastRP::ENTRY_PROBABILITY {
        entry_value
    } else if r < FastRP::ENTRY_PROBABILITY * 2.0 {
        -entry_value
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::progress::NoopProgressTracker;
    use crate::types::graph_store::DefaultGraphStore;
    use crate::types::random::{RandomGraphConfig, RandomRelationshipConfig};

    #[test]
    fn fastrp_smoke_produces_embeddings() {
        let config = RandomGraphConfig {
            graph_name: "fastrp".into(),
            database_name: "in-memory".into(),
            node_count: 16,
            node_labels: vec!["N".into()],
            relationships: vec![RandomRelationshipConfig::new("R", 0.3)],
            directed: true,
            inverse_indexed: false,
            seed: Some(7),
        };
        let store = DefaultGraphStore::random(&config).unwrap();
        let graph = store.graph();

        let cfg = FastRPConfig {
            feature_properties: vec![],
            iteration_weights: vec![1.0, 1.0],
            embedding_dimension: 8,
            property_dimension: 0,
            relationship_weight_property: None,
            normalization_strength: 0.0,
            node_self_influence: 1.0,
            concurrency: 1,
            min_batch_size: 10_000,
            random_seed: Some(7),
        };

        let result = FastRPComputationRuntime::run(graph, &cfg, vec![]).unwrap();
        assert_eq!(result.embeddings.len(), config.node_count);
        assert_eq!(result.embeddings[0].len(), 8);
        assert!(result
            .embeddings
            .iter()
            .flatten()
            .all(|value| value.is_finite()));
    }

    #[test]
    fn fastrp_seeded_results_are_equal_across_worker_counts() {
        let graph_config = RandomGraphConfig {
            graph_name: "fastrp-concurrency".into(),
            database_name: "in-memory".into(),
            node_count: 64,
            node_labels: vec!["N".into()],
            relationships: vec![RandomRelationshipConfig::new("R", 0.2)],
            directed: true,
            inverse_indexed: false,
            seed: Some(11),
        };
        let graph: Arc<dyn Graph> = DefaultGraphStore::random(&graph_config).unwrap().graph();
        let mut config = FastRPConfig {
            embedding_dimension: 16,
            iteration_weights: vec![0.5, 1.0, 1.5],
            random_seed: Some(17),
            concurrency: 1,
            ..FastRPConfig::default()
        };

        let sequential =
            FastRPComputationRuntime::run(Arc::clone(&graph), &config, vec![]).unwrap();
        config.concurrency = 4;
        let parallel = FastRPComputationRuntime::run(graph, &config, vec![]).unwrap();

        assert_eq!(sequential.embeddings, parallel.embeddings);
    }

    #[test]
    fn fastrp_honors_pre_cancelled_execution() {
        let graph_config = RandomGraphConfig {
            node_count: 8,
            relationships: vec![RandomRelationshipConfig::new("R", 0.5)],
            ..RandomGraphConfig::default()
        };
        let graph = DefaultGraphStore::random(&graph_config).unwrap().graph();
        let config = FastRPConfig {
            embedding_dimension: 8,
            concurrency: 2,
            ..FastRPConfig::default()
        };
        let mut progress = NoopProgressTracker;

        let error = FastRPComputationRuntime::run_with_controls(
            graph,
            &config,
            vec![],
            &mut progress,
            &TerminationFlag::stop_running(),
        )
        .unwrap_err();

        assert!(error.to_string().contains("terminated"));
    }
}
