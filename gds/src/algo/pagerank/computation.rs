//! PageRank computation runtime
//!
//! Implements a standard PageRank power-iteration with:
//! - damping factor $d$
//! - convergence by max absolute delta < tolerance
//! - dangling mass redistribution according to the teleport distribution
//!   (uniform for vanilla PR, uniform-over-sources for personalized PR).

use crate::algo::pagerank::PageRankVariant;
use crate::collections::HugeAtomicDoubleArray;
use crate::concurrency::{install_with_concurrency, Concurrency};
use crate::core::graph_dimensions::GraphDimensions;
use crate::mem::{Estimate, MemoryEstimation, MemoryRange, MemoryTree};
use rayon::prelude::*;
use std::collections::HashSet;

#[derive(Debug, Clone, serde::Serialize)]
pub struct PageRankRunResult {
    pub scores: Vec<f64>,
    pub ran_iterations: usize,
    pub did_converge: bool,
}

/// Pure PageRank state/runtime.
///
/// Holds parameters and executes the power-iteration given graph callbacks.
#[derive(Clone, Debug)]
pub struct PageRankComputationRuntime {
    max_iterations: usize,
    damping_factor: f64,
    tolerance: f64,
    source_nodes: Option<HashSet<u64>>,
    variant: PageRankVariant,
}

impl PageRankComputationRuntime {
    pub fn new(
        max_iterations: usize,
        damping_factor: f64,
        tolerance: f64,
        source_nodes: Option<HashSet<u64>>,
        variant: PageRankVariant,
    ) -> Self {
        Self {
            max_iterations,
            damping_factor,
            tolerance,
            source_nodes,
            variant,
        }
    }

    pub fn max_iterations(&self) -> usize {
        self.max_iterations
    }

    fn initial_rank(&self, node_count: usize) -> Vec<f64> {
        match self.variant {
            PageRankVariant::PageRank | PageRankVariant::ArticleRank => {
                let alpha = 1.0 - self.damping_factor;
                self.source_seed_values(node_count, alpha, alpha)
            }
            PageRankVariant::Eigenvector => {
                if node_count == 0 {
                    return Vec::new();
                }
                match &self.source_nodes {
                    None => vec![1.0 / node_count as f64; node_count],
                    Some(s) if s.is_empty() => vec![1.0 / node_count as f64; node_count],
                    Some(source_nodes) => {
                        let valid_source_count = source_nodes
                            .iter()
                            .filter(|node_id| (**node_id as usize) < node_count)
                            .count();
                        if valid_source_count == 0 {
                            vec![1.0 / node_count as f64; node_count]
                        } else {
                            let source_value = 1.0 / valid_source_count as f64;
                            (0..node_count)
                                .map(|node_id| {
                                    if source_nodes.contains(&(node_id as u64)) {
                                        source_value
                                    } else {
                                        0.0
                                    }
                                })
                                .collect()
                        }
                    }
                }
            }
        }
    }

    fn source_seed_values(
        &self,
        node_count: usize,
        seeded_value: f64,
        fallback_value: f64,
    ) -> Vec<f64> {
        if node_count == 0 {
            return Vec::new();
        }

        match &self.source_nodes {
            None => vec![fallback_value; node_count],
            Some(s) if s.is_empty() => vec![fallback_value; node_count],
            Some(source_nodes) => {
                let valid_source_count = source_nodes
                    .iter()
                    .filter(|node_id| (**node_id as usize) < node_count)
                    .count();
                if valid_source_count == 0 {
                    return vec![fallback_value; node_count];
                }
                (0..node_count)
                    .map(|node_id| {
                        if source_nodes.contains(&(node_id as u64)) {
                            seeded_value
                        } else {
                            0.0
                        }
                    })
                    .collect()
            }
        }
    }

    pub fn run(
        &self,
        node_count: usize,
        degree: &[f64],
        average_degree: f64,
        concurrency: usize,
        stream_neighbors: &(impl Fn(usize, &mut dyn FnMut(usize, f64)) + Sync),
    ) -> PageRankRunResult {
        self.run_with_progress(
            node_count,
            degree,
            average_degree,
            concurrency,
            stream_neighbors,
            |_| {},
        )
    }

    pub fn run_with_progress(
        &self,
        node_count: usize,
        degree: &[f64],
        average_degree: f64,
        concurrency: usize,
        stream_neighbors: &(impl Fn(usize, &mut dyn FnMut(usize, f64)) + Sync),
        mut on_iteration_complete: impl FnMut(usize),
    ) -> PageRankRunResult {
        if node_count == 0 {
            return PageRankRunResult {
                scores: Vec::new(),
                ran_iterations: 0,
                did_converge: true,
            };
        }

        match self.variant {
            PageRankVariant::PageRank | PageRankVariant::ArticleRank => self.run_delta_rank(
                node_count,
                degree,
                average_degree,
                concurrency,
                stream_neighbors,
                &mut on_iteration_complete,
            ),
            PageRankVariant::Eigenvector => self.run_eigenvector(
                node_count,
                degree,
                concurrency,
                stream_neighbors,
                &mut on_iteration_complete,
            ),
        }
    }

    fn run_delta_rank(
        &self,
        node_count: usize,
        degree: &[f64],
        average_degree: f64,
        concurrency: usize,
        stream_neighbors: &(impl Fn(usize, &mut dyn FnMut(usize, f64)) + Sync),
        on_iteration_complete: &mut impl FnMut(usize),
    ) -> PageRankRunResult {
        let damping = self.damping_factor;
        let tolerance = self.tolerance;
        let max_iterations = self.max_iterations;

        let mut rank = self.initial_rank(node_count);
        let mut delta = rank.clone();

        let concurrency = Concurrency::from_usize(concurrency.max(1));

        let mut did_converge = false;
        let mut ran_iterations = 0;

        for iter in 0..max_iterations {
            ran_iterations = iter + 1;

            let messages = HugeAtomicDoubleArray::new(node_count);

            install_with_concurrency(concurrency, || {
                (0..node_count).into_par_iter().for_each(|source| {
                    let source_delta = delta[source];
                    if source_delta <= tolerance && iter > 0 {
                        return;
                    }

                    let denominator = match self.variant {
                        PageRankVariant::PageRank => degree[source],
                        PageRankVariant::ArticleRank => degree[source] + average_degree,
                        PageRankVariant::Eigenvector => unreachable!(),
                    };
                    if denominator <= 0.0 || !denominator.is_finite() {
                        return;
                    }

                    let contribution = source_delta / denominator;
                    let mut push = |target: usize, weight: f64| {
                        messages.get_and_add(target, contribution * weight);
                    };
                    stream_neighbors(source, &mut push);
                });
            });

            let mut max_delta = 0.0;
            for i in 0..node_count {
                delta[i] = damping * messages.get(i);
                rank[i] += delta[i];
                let abs_delta = delta[i].abs();
                if abs_delta > max_delta {
                    max_delta = abs_delta;
                }
            }

            on_iteration_complete(1);

            if max_delta <= tolerance {
                did_converge = true;
                break;
            }
        }

        PageRankRunResult {
            scores: rank,
            ran_iterations,
            did_converge,
        }
    }

    fn run_eigenvector(
        &self,
        node_count: usize,
        degree: &[f64],
        concurrency: usize,
        stream_neighbors: &(impl Fn(usize, &mut dyn FnMut(usize, f64)) + Sync),
        on_iteration_complete: &mut impl FnMut(usize),
    ) -> PageRankRunResult {
        let tolerance = self.tolerance;
        let max_iterations = self.max_iterations;
        let concurrency = Concurrency::from_usize(concurrency.max(1));

        let mut rank = self.initial_rank(node_count);
        let mut incoming = vec![0.0; node_count];
        let mut did_converge = false;
        let mut ran_iterations = 0;

        for iter in 0..max_iterations {
            ran_iterations = iter + 1;

            let mut next_rank: Vec<f64> = rank
                .iter()
                .zip(incoming.iter())
                .map(|(current, message)| current + message)
                .collect();

            let norm = next_rank
                .iter()
                .map(|value| value * value)
                .sum::<f64>()
                .sqrt();
            if norm > 0.0 && norm.is_finite() {
                for value in &mut next_rank {
                    *value /= norm;
                }
            }

            let max_delta = next_rank
                .iter()
                .zip(rank.iter())
                .map(|(next, current)| (next - current).abs())
                .fold(0.0, f64::max);

            let messages = HugeAtomicDoubleArray::new(node_count);
            install_with_concurrency(concurrency, || {
                (0..node_count).into_par_iter().for_each(|source| {
                    let denominator = degree[source];
                    if denominator <= 0.0 || !denominator.is_finite() {
                        return;
                    }

                    let contribution = next_rank[source] / denominator;
                    let mut push = |target: usize, weight: f64| {
                        messages.get_and_add(target, contribution * weight);
                    };
                    stream_neighbors(source, &mut push);
                });
            });

            for i in 0..node_count {
                incoming[i] = messages.get(i);
            }

            rank = next_rank;
            on_iteration_complete(1);

            if iter > 0 && max_delta <= tolerance {
                did_converge = true;
                break;
            }
        }

        PageRankRunResult {
            scores: rank,
            ran_iterations,
            did_converge,
        }
    }
}

// -------------------------------------------------------------------------------------------------
// Memory estimation
// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct PageRankMemoryEstimation;

impl MemoryEstimation for PageRankMemoryEstimation {
    fn description(&self) -> String {
        "PageRank".to_string()
    }

    fn estimate(&self, dimensions: &dyn GraphDimensions, _concurrency: usize) -> MemoryTree {
        let node_count = dimensions.node_count();

        let rank_vec = Estimate::size_of_double_array(node_count);
        let next_vec = Estimate::size_of_double_array(node_count);
        // Approximate out-degree as an int array (usize varies, but this keeps estimates stable).
        let out_degree = Estimate::size_of_int_array(node_count);

        let components = vec![
            MemoryTree::leaf(
                "this.instance".into(),
                MemoryRange::of(Estimate::BYTES_OBJECT_HEADER),
            ),
            MemoryTree::leaf("rank".into(), MemoryRange::of(rank_vec)),
            MemoryTree::leaf("next".into(), MemoryRange::of(next_vec)),
            MemoryTree::leaf("outDegree".into(), MemoryRange::of(out_degree)),
        ];

        let total = components
            .iter()
            .fold(MemoryRange::empty(), |acc, t| acc.add(t.memory_usage()));

        MemoryTree::new("PageRank".into(), total, components)
    }
}

pub fn estimate_pagerank_memory(
    dimensions: &dyn GraphDimensions,
    concurrency: usize,
) -> MemoryTree {
    PageRankMemoryEstimation.estimate(dimensions, concurrency)
}
