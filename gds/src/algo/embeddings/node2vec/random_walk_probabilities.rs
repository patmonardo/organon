//! Builder for random walk probabilities used in negative sampling.

use crate::task::concurrency::Concurrency;
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug)]
pub struct RandomWalkProbabilities {
    pub node_frequencies: HashMap<i64, f64>,
    pub total_frequency: f64,
}

#[derive(Debug, Clone)]
pub struct NegativeSamplingDistribution {
    weights: Vec<f64>,
    cumulative_weights: Vec<f64>,
    total_weight: f64,
}

impl NegativeSamplingDistribution {
    pub fn sample_excluding(&self, excluded: usize, rng: &mut impl Rng) -> Option<usize> {
        let excluded_weight = self.weights.get(excluded).copied().unwrap_or(0.0);
        let available_weight = self.total_weight - excluded_weight;
        if available_weight <= 0.0 {
            return None;
        }

        let mut target = rng.gen_range(0.0..available_weight);
        let weight_before_excluded = excluded
            .checked_sub(1)
            .and_then(|index| self.cumulative_weights.get(index))
            .copied()
            .unwrap_or(0.0);
        if target >= weight_before_excluded {
            target += excluded_weight;
        }

        let sampled = self
            .cumulative_weights
            .partition_point(|&cumulative| cumulative <= target);
        (sampled < self.weights.len()).then_some(sampled)
    }
}

impl RandomWalkProbabilities {
    pub fn positive_sampling_probabilities(
        &self,
        node_count: usize,
        sampling_factor: f64,
    ) -> Vec<f64> {
        (0..node_count)
            .map(|node_id| {
                let frequency = self
                    .node_frequencies
                    .get(&(node_id as i64))
                    .copied()
                    .unwrap_or(0.0)
                    / self.total_frequency;
                if frequency == 0.0 {
                    0.0
                } else {
                    ((frequency / sampling_factor).sqrt() + 1.0) * (sampling_factor / frequency)
                }
                .min(1.0)
            })
            .collect()
    }

    pub fn negative_sampling_distribution(
        &self,
        node_count: usize,
        exponent: f64,
    ) -> NegativeSamplingDistribution {
        let weights: Vec<f64> = (0..node_count)
            .map(|node_id| {
                self.node_frequencies
                    .get(&(node_id as i64))
                    .copied()
                    .unwrap_or(0.0)
                    .powf(exponent)
            })
            .collect();
        let mut total_weight = 0.0;
        let cumulative_weights = weights
            .iter()
            .map(|weight| {
                total_weight += weight;
                total_weight
            })
            .collect();

        NegativeSamplingDistribution {
            weights,
            cumulative_weights,
            total_weight,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    fn probabilities() -> RandomWalkProbabilities {
        RandomWalkProbabilities {
            node_frequencies: HashMap::from([(0, 1.0), (1, 9.0), (2, 90.0)]),
            total_frequency: 100.0,
        }
    }

    #[test]
    fn negative_sampling_follows_frequency_weights() {
        let distribution = probabilities().negative_sampling_distribution(3, 0.75);
        let mut rng = StdRng::seed_from_u64(42);
        let mut counts = [0usize; 3];
        for _ in 0..2_000 {
            counts[distribution.sample_excluding(0, &mut rng).unwrap()] += 1;
        }

        assert_eq!(counts[0], 0);
        assert!(counts[2] > counts[1] * 3);
    }

    #[test]
    fn positive_sampling_retains_rare_nodes_more_often() {
        let retention = probabilities().positive_sampling_probabilities(3, 0.001);
        assert!(retention[0] > retention[1]);
        assert!(retention[1] > retention[2]);
    }
}

#[derive(Debug)]
pub struct RandomWalkProbabilitiesBuilder {
    _concurrency: Concurrency,
    node_frequencies: HashMap<i64, f64>,
}

impl RandomWalkProbabilitiesBuilder {
    pub fn new(concurrency: Concurrency) -> Self {
        Self {
            _concurrency: concurrency,
            node_frequencies: HashMap::new(),
        }
    }

    pub fn register_walk(&mut self, walk: &[i64]) {
        for &node_id in walk {
            *self.node_frequencies.entry(node_id).or_insert(0.0) += 1.0;
        }
    }

    pub fn build(self) -> RandomWalkProbabilities {
        let total_frequency = self.node_frequencies.values().sum();
        RandomWalkProbabilities {
            node_frequencies: self.node_frequencies,
            total_frequency,
        }
    }
}
