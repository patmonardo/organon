//! Node2Vec model implementation with skip-gram training.

use super::compressed_random_walks::CompressedRandomWalks;
use super::random_walk_probabilities::NegativeSamplingDistribution;
use super::train_parameters::{EmbeddingInitializer, TrainParameters};
use crate::task::concurrency::TerminatedException;
use crate::task::concurrency::{Concurrency, TerminationFlag};
use crate::task::progress::ProgressTracker;
use rand::prelude::*;

#[derive(Debug)]
pub struct TrainedNode2Vec {
    pub embeddings: Vec<Vec<f64>>,
    pub loss_per_iteration: Vec<f64>,
}

pub struct Node2VecModel {
    node_count: usize,
    train_params: TrainParameters,
    _concurrency: Concurrency,
    random_seed: Option<u64>,
    walks: CompressedRandomWalks,
    positive_sampling_probabilities: Vec<f64>,
    negative_sampling_distribution: NegativeSamplingDistribution,
    termination_flag: TerminationFlag,
}

impl Node2VecModel {
    pub fn new(
        node_count: usize,
        train_params: TrainParameters,
        concurrency: Concurrency,
        random_seed: Option<u64>,
        walks: CompressedRandomWalks,
        positive_sampling_probabilities: Vec<f64>,
        negative_sampling_distribution: NegativeSamplingDistribution,
        termination_flag: TerminationFlag,
    ) -> Self {
        Self {
            node_count,
            train_params,
            _concurrency: concurrency,
            random_seed,
            walks,
            positive_sampling_probabilities,
            negative_sampling_distribution,
            termination_flag,
        }
    }

    pub fn train(
        self,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<TrainedNode2Vec, TerminatedException> {
        let mut rng = match self.random_seed {
            Some(seed) => StdRng::seed_from_u64(seed),
            None => StdRng::from_entropy(),
        };

        let initialization_bound = match self.train_params.embedding_initializer {
            EmbeddingInitializer::Uniform => 1.0,
            EmbeddingInitializer::Normalized => 0.5 / self.train_params.embedding_dimension as f64,
        };
        let mut center_embeddings = self.initialize_embeddings(&mut rng, initialization_bound);
        let mut context_embeddings = center_embeddings.clone();

        let mut loss_per_iteration = Vec::new();
        let learning_rate_alpha = (self.train_params.initial_learning_rate
            - self.train_params.min_learning_rate)
            / self.train_params.iterations as f64;

        for iteration in 0..self.train_params.iterations {
            if !self.termination_flag.running() {
                return Err(TerminatedException);
            }

            let mut total_loss = 0.0;
            let learning_rate = self.train_params.min_learning_rate.max(
                self.train_params.initial_learning_rate - iteration as f64 * learning_rate_alpha,
            );

            for walk in self.walks.walks() {
                if !self.termination_flag.running() {
                    return Err(TerminatedException);
                }
                let sampled_walk: Vec<usize> = walk
                    .iter()
                    .copied()
                    .filter_map(|node| {
                        let node_index = usize::try_from(node).ok()?;
                        (node_index < self.node_count
                            && rng.gen::<f64>() < self.positive_sampling_probabilities[node_index])
                            .then_some(node_index)
                    })
                    .collect();
                if sampled_walk.len() < 2 {
                    continue;
                }

                let prefix_window_size = self.train_params.window_size / 2;
                let postfix_window_size = self.train_params.window_size.saturating_sub(1) / 2;
                for (i, &center_node) in sampled_walk.iter().enumerate() {
                    let start = i.saturating_sub(prefix_window_size);
                    let end = (i + postfix_window_size + 1).min(sampled_walk.len());

                    for (j, &context_node) in sampled_walk.iter().enumerate().take(end).skip(start)
                    {
                        if i == j {
                            continue;
                        }

                        let loss = self.train_pair(
                            center_node,
                            context_node,
                            &mut center_embeddings,
                            &mut context_embeddings,
                            learning_rate,
                            &mut rng,
                        );
                        total_loss += loss;
                    }
                }
            }

            loss_per_iteration.push(total_loss);
            progress_tracker.log_progress(1);
        }

        Ok(TrainedNode2Vec {
            embeddings: center_embeddings,
            loss_per_iteration,
        })
    }

    fn initialize_embeddings(&self, rng: &mut impl Rng, bound: f64) -> Vec<Vec<f64>> {
        (0..self.node_count)
            .map(|_| {
                (0..self.train_params.embedding_dimension)
                    .map(|_| rng.gen_range(-bound..bound))
                    .collect()
            })
            .collect()
    }

    fn train_pair(
        &self,
        center_idx: usize,
        context_idx: usize,
        center_embeddings: &mut [Vec<f64>],
        context_embeddings: &mut [Vec<f64>],
        learning_rate: f64,
        rng: &mut impl Rng,
    ) -> f64 {
        const EPSILON: f64 = 1e-10;

        let center_embedding = center_embeddings[center_idx].clone();
        let context_embedding = context_embeddings[context_idx].clone();
        let affinity = inner_product(&center_embedding, &context_embedding);
        let positive_probability = sigmoid(affinity);
        let mut loss = -(positive_probability + EPSILON).ln();
        update_embeddings(
            &mut center_embeddings[center_idx],
            &mut context_embeddings[context_idx],
            (1.0 - positive_probability) * learning_rate,
        );

        for _ in 0..self.train_params.negative_sampling_rate {
            let Some(neg_idx) = self
                .negative_sampling_distribution
                .sample_excluding(center_idx, rng)
            else {
                break;
            };
            let center_embedding = center_embeddings[center_idx].clone();
            let negative_embedding = context_embeddings[neg_idx].clone();
            let negative_probability =
                sigmoid(inner_product(&center_embedding, &negative_embedding));
            loss -= (1.0 - negative_probability + EPSILON).ln();
            update_embeddings(
                &mut center_embeddings[center_idx],
                &mut context_embeddings[neg_idx],
                -negative_probability * learning_rate,
            );
        }

        loss
    }
}

fn inner_product(left: &[f64], right: &[f64]) -> f64 {
    left.iter().zip(right).map(|(a, b)| a * b).sum()
}

fn sigmoid(value: f64) -> f64 {
    1.0 / (1.0 + (-value).exp())
}

fn update_embeddings(center: &mut [f64], context: &mut [f64], scaled_gradient: f64) {
    let center_before = center.to_vec();
    for ((center_value, context_value), center_original) in
        center.iter_mut().zip(context.iter_mut()).zip(center_before)
    {
        let context_original = *context_value;
        *center_value += scaled_gradient * context_original;
        *context_value += scaled_gradient * center_original;
    }
}
