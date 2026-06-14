//! RandomWalk Computation
//!
//! **Translation Source**: `org.neo4j.gds.traversal.RandomWalk`
//!
//! Implements node2vec-style biased random walks with return and in-out factors.

use super::spec::RandomWalkResult;
use super::RandomWalkStorageRuntime;
use crate::task::concurrency::{Concurrency, Executor, TerminatedException, TerminationFlag};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub struct RandomWalkComputationRuntime {
    storage: Arc<RandomWalkStorageRuntime>,
    walks_per_node: usize,
    walk_length: usize,
    return_factor: f64,
    in_out_factor: f64,
    source_nodes: Vec<usize>,
    random_seed: u64,
}

impl RandomWalkComputationRuntime {
    pub fn new(
        walks_per_node: usize,
        walk_length: usize,
        return_factor: f64,
        in_out_factor: f64,
        source_nodes: Vec<usize>,
        random_seed: u64,
    ) -> Self {
        Self {
            storage: Arc::new(RandomWalkStorageRuntime::new()),
            walks_per_node,
            walk_length,
            return_factor,
            in_out_factor,
            source_nodes,
            random_seed,
        }
    }

    /// Compute random walks.
    pub fn compute(
        &self,
        node_count: usize,
        get_neighbors: impl Fn(usize) -> Vec<usize> + Send + Sync,
    ) -> RandomWalkResult {
        let get_neighbors = Arc::new(get_neighbors);
        let nodes_to_walk: Vec<usize> = if self.source_nodes.is_empty() {
            (0..node_count).collect()
        } else {
            self.source_nodes.clone()
        };

        for start_node in nodes_to_walk {
            let mut rng = StdRng::seed_from_u64(self.random_seed.wrapping_add(start_node as u64));

            for _ in 0..self.walks_per_node {
                let walk = self.perform_walk(start_node, &get_neighbors, &mut rng);
                self.storage.add_walk(walk);
            }
        }

        RandomWalkResult {
            walks: self.storage.take_walks(),
        }
    }

    pub fn compute_with_concurrency(
        &self,
        node_count: usize,
        concurrency: usize,
        termination: &TerminationFlag,
        get_neighbors: impl Fn(usize) -> Vec<usize> + Send + Sync + 'static,
    ) -> Result<RandomWalkResult, TerminatedException> {
        let get_neighbors = Arc::new(get_neighbors);
        let nodes_to_walk: Vec<usize> = if self.source_nodes.is_empty() {
            (0..node_count).collect()
        } else {
            self.source_nodes.clone()
        };

        let concurrency = concurrency.max(1);
        let executor = Executor::new(Concurrency::of(concurrency));
        let nodes_to_walk = Arc::new(nodes_to_walk);
        let node_index = AtomicUsize::new(0);
        let total_nodes = nodes_to_walk.len();

        executor.scope(termination, |scope| {
            scope.spawn_many(concurrency, |_worker_id| loop {
                if !termination.running() {
                    return;
                }

                let idx = node_index.fetch_add(1, Ordering::SeqCst);
                if idx >= total_nodes {
                    break;
                }

                let start_node = nodes_to_walk[idx];
                let mut rng =
                    StdRng::seed_from_u64(self.random_seed.wrapping_add(start_node as u64));

                for _ in 0..self.walks_per_node {
                    let walk = self.perform_walk(start_node, &get_neighbors, &mut rng);
                    self.storage.add_walk(walk);
                }
            });
        })?;

        Ok(RandomWalkResult {
            walks: self.storage.take_walks(),
        })
    }

    fn perform_walk(
        &self,
        start_node: usize,
        get_neighbors: &Arc<impl Fn(usize) -> Vec<usize>>,
        rng: &mut StdRng,
    ) -> Vec<u64> {
        let mut walk = Vec::with_capacity(self.walk_length);
        walk.push(start_node as u64);

        let mut current = start_node;
        let mut previous: Option<usize> = None;

        for _ in 1..self.walk_length {
            let neighbors = get_neighbors(current);

            if neighbors.is_empty() {
                break;
            }

            let next = self.sample_next_node(previous, &neighbors, rng);

            walk.push(next as u64);
            previous = Some(current);
            current = next;
        }

        walk
    }

    fn sample_next_node(
        &self,
        previous: Option<usize>,
        neighbors: &[usize],
        rng: &mut StdRng,
    ) -> usize {
        if neighbors.len() == 1 {
            return neighbors[0];
        }

        let weights: Vec<f64> = neighbors
            .iter()
            .map(|&neighbor| {
                if let Some(prev) = previous {
                    if neighbor == prev {
                        1.0 / self.return_factor
                    } else {
                        1.0 / self.in_out_factor
                    }
                } else {
                    1.0
                }
            })
            .collect();

        let total_weight: f64 = weights.iter().sum();
        let mut threshold = rng.gen::<f64>() * total_weight;

        for (i, &weight) in weights.iter().enumerate() {
            threshold -= weight;
            if threshold <= 0.0 {
                return neighbors[i];
            }
        }

        neighbors[neighbors.len() - 1]
    }
}
