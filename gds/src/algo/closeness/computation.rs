//! Closeness Centrality compute kernels
//!
//! Java parity reference:
//! - `org.neo4j.gds.closeness.ClosenessCentrality`
//!
//! This module intentionally contains no GraphStore/projection logic.
//! The storage runtime owns orchestration and provides neighbor access.

use crate::algo::msbfs::{AggregatedNeighborProcessingMsBfs, OMEGA};
use crate::collections::{HugeAtomicDoubleArray, HugeAtomicLongArray};
use crate::concurrency::{
    install_with_concurrency, Concurrency, TerminatedException, TerminationFlag,
};
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

/// Computation runtime for closeness centrality (state-only, no graph access)
pub struct ClosenessCentralityComputationRuntime;

impl ClosenessCentralityComputationRuntime {
    pub fn new() -> Self {
        Self
    }

    /// Phase 1: compute per-node farness and component size using ANP MSBFS batching.
    pub fn compute_farness_parallel(
        &self,
        node_count: usize,
        concurrency: usize,
        termination: &TerminationFlag,
        on_sources_done: Arc<dyn Fn(usize) + Send + Sync>,
        get_neighbors: &(impl Fn(usize) -> Vec<usize> + Send + Sync),
    ) -> Result<(Vec<u64>, Vec<u64>), TerminatedException> {
        if node_count == 0 {
            return Ok((Vec::new(), Vec::new()));
        }

        let farness = HugeAtomicLongArray::new(node_count);
        let component = HugeAtomicLongArray::new(node_count);
        let concurrency = concurrency.max(1);
        let batch_count = (node_count + OMEGA - 1) / OMEGA;
        let counter = AtomicUsize::new(0);

        install_with_concurrency(Concurrency::of(concurrency), || {
            (0..concurrency)
                .into_par_iter()
                .try_for_each(|_| -> Result<(), TerminatedException> {
                    let mut msbfs = AggregatedNeighborProcessingMsBfs::new(node_count);

                    loop {
                        if !termination.running() {
                            return Err(TerminatedException);
                        }

                        let batch_idx = counter.fetch_add(1, Ordering::Relaxed);
                        if batch_idx >= batch_count {
                            break;
                        }

                        let source_offset = batch_idx * OMEGA;
                        let source_len = (source_offset + OMEGA).min(node_count) - source_offset;

                        msbfs.run_with_termination(
                            source_offset,
                            source_len,
                            false,
                            Some(termination),
                            |n| (get_neighbors)(n),
                            |node_id, depth, sources_mask| {
                                if depth == 0 {
                                    return;
                                }

                                let len = sources_mask.count_ones() as i64;
                                let d = depth as i64;
                                let far_delta = len.saturating_mul(d);

                                farness.get_and_add(node_id, far_delta);
                                component.get_and_add(node_id, len);
                                (on_sources_done.as_ref())(len as usize);
                            },
                        );

                        if !termination.running() {
                            return Err(TerminatedException);
                        }
                    }

                    Ok(())
                })
        })?;

        let mut farness_out = vec![0u64; node_count];
        let mut component_out = vec![0u64; node_count];
        for i in 0..node_count {
            let far = farness.get(i);
            let comp = component.get(i);
            farness_out[i] = far.max(0) as u64;
            component_out[i] = comp.max(0) as u64;
        }

        Ok((farness_out, component_out))
    }

    /// Phase 2: compute closeness scores from farness + component.
    pub fn compute_closeness_parallel(
        &self,
        node_count: usize,
        wasserman_faust: bool,
        concurrency: usize,
        termination: &TerminationFlag,
        farness: &[u64],
        component: &[u64],
        on_nodes_done: Arc<dyn Fn(usize) + Send + Sync>,
    ) -> Result<Vec<f64>, TerminatedException> {
        if node_count == 0 {
            return Ok(Vec::new());
        }

        let concurrency = concurrency.max(1);
        let scores = HugeAtomicDoubleArray::new(node_count);
        let counter = AtomicUsize::new(0);

        install_with_concurrency(Concurrency::of(concurrency), || {
            (0..concurrency)
                .into_par_iter()
                .try_for_each(|_| -> Result<(), TerminatedException> {
                    let mut completed = 0usize;

                    loop {
                        if !termination.running() {
                            if completed > 0 {
                                (on_nodes_done.as_ref())(completed);
                            }
                            return Err(TerminatedException);
                        }

                        let i = counter.fetch_add(1, Ordering::Relaxed);
                        if i >= node_count {
                            break;
                        }

                        let far = farness[i];
                        if far == 0 {
                            scores.set(i, 0.0);
                            completed += 1;
                            continue;
                        }

                        let comp = component[i] as f64;
                        let base = comp / (far as f64);

                        let value = if wasserman_faust {
                            if node_count <= 1 {
                                0.0
                            } else {
                                // Java: (comp/far) * (comp/(nodeCount-1))
                                base * (comp / (node_count as f64 - 1.0))
                            }
                        } else {
                            base
                        };

                        scores.set(i, value);
                        completed += 1;
                    }

                    if completed > 0 {
                        (on_nodes_done.as_ref())(completed);
                    }

                    Ok(())
                })
        })?;

        let mut out = vec![0.0f64; node_count];
        for i in 0..node_count {
            out[i] = scores.get(i);
        }

        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn undirected_adj(edges: &[(usize, usize)], node_count: usize) -> Vec<Vec<usize>> {
        let mut adj = vec![Vec::<usize>::new(); node_count];
        for &(a, b) in edges {
            adj[a].push(b);
            if a != b {
                adj[b].push(a);
            }
        }
        for v in adj.iter_mut() {
            v.sort_unstable();
            v.dedup();
        }
        adj
    }

    fn assert_scores_close(left: &[f64], right: &[f64]) {
        assert_eq!(left.len(), right.len());
        for (idx, (left_score, right_score)) in left.iter().zip(right.iter()).enumerate() {
            assert!(
                (left_score - right_score).abs() < 1e-9,
                "score mismatch at node {idx}: {left_score} != {right_score}"
            );
        }
    }

    #[test]
    fn farness_and_scores_are_deterministic() {
        let node_count = 6;
        let adj = undirected_adj(&[(0, 1), (1, 2), (2, 3), (3, 4), (4, 5)], node_count);
        let neighbors = |n: usize| adj[n].clone();

        let termination = TerminationFlag::running_true();
        let noop_sources = Arc::new(|_n: usize| {});

        let runtime = ClosenessCentralityComputationRuntime::new();

        let (f1, c1) = runtime
            .compute_farness_parallel(
                node_count,
                1,
                &termination,
                noop_sources.clone(),
                &neighbors,
            )
            .unwrap();

        let (f2, c2) = runtime
            .compute_farness_parallel(node_count, 4, &termination, noop_sources, &neighbors)
            .unwrap();

        assert_eq!(f1, f2);
        assert_eq!(c1, c2);

        let noop_nodes = Arc::new(|_n: usize| {});
        let s1 = runtime
            .compute_closeness_parallel(
                node_count,
                false,
                2,
                &termination,
                &f1,
                &c1,
                noop_nodes.clone(),
            )
            .unwrap();

        let s2 = runtime
            .compute_closeness_parallel(node_count, true, 2, &termination, &f1, &c1, noop_nodes)
            .unwrap();

        // Wasserman-Faust should not exceed base when comp <= node_count.
        for (base, wf) in s1.iter().zip(s2.iter()) {
            assert!(*wf <= *base + 1e-12 || node_count <= 1);
        }
    }

    #[test]
    fn path_graph_scores_match_formula() {
        let node_count = 3;
        let adj = undirected_adj(&[(0, 1), (1, 2)], node_count);
        let runtime = ClosenessCentralityComputationRuntime::new();
        let termination = TerminationFlag::running_true();

        let (farness, component) = runtime
            .compute_farness_parallel(
                node_count,
                4,
                &termination,
                Arc::new(|_n: usize| {}),
                &|n| adj[n].clone(),
            )
            .unwrap();

        assert_eq!(farness, vec![3, 2, 3]);
        assert_eq!(component, vec![2, 2, 2]);

        let scores = runtime
            .compute_closeness_parallel(
                node_count,
                false,
                4,
                &termination,
                &farness,
                &component,
                Arc::new(|_n: usize| {}),
            )
            .unwrap();

        assert_scores_close(&scores, &[2.0 / 3.0, 1.0, 2.0 / 3.0]);
    }

    #[test]
    fn disconnected_graph_uses_component_size_and_wasserman_faust() {
        let node_count = 3;
        let adj = undirected_adj(&[(0, 1)], node_count);
        let runtime = ClosenessCentralityComputationRuntime::new();
        let termination = TerminationFlag::running_true();

        let (farness, component) = runtime
            .compute_farness_parallel(
                node_count,
                4,
                &termination,
                Arc::new(|_n: usize| {}),
                &|n| adj[n].clone(),
            )
            .unwrap();

        assert_eq!(farness, vec![1, 1, 0]);
        assert_eq!(component, vec![1, 1, 0]);

        let base = runtime
            .compute_closeness_parallel(
                node_count,
                false,
                4,
                &termination,
                &farness,
                &component,
                Arc::new(|_n: usize| {}),
            )
            .unwrap();
        let wf = runtime
            .compute_closeness_parallel(
                node_count,
                true,
                4,
                &termination,
                &farness,
                &component,
                Arc::new(|_n: usize| {}),
            )
            .unwrap();

        assert_scores_close(&base, &[1.0, 1.0, 0.0]);
        assert_scores_close(&wf, &[0.5, 0.5, 0.0]);
    }

    #[test]
    fn closeness_parallel_matches_single_worker() {
        let node_count = 7;
        let adj = undirected_adj(
            &[(0, 1), (1, 2), (2, 3), (1, 4), (4, 5), (5, 6), (2, 6)],
            node_count,
        );
        let runtime = ClosenessCentralityComputationRuntime::new();
        let termination = TerminationFlag::running_true();

        let (single_farness, single_component) = runtime
            .compute_farness_parallel(
                node_count,
                1,
                &termination,
                Arc::new(|_n: usize| {}),
                &|n| adj[n].clone(),
            )
            .unwrap();
        let single_scores = runtime
            .compute_closeness_parallel(
                node_count,
                true,
                1,
                &termination,
                &single_farness,
                &single_component,
                Arc::new(|_n: usize| {}),
            )
            .unwrap();

        let termination = TerminationFlag::running_true();
        let (parallel_farness, parallel_component) = runtime
            .compute_farness_parallel(
                node_count,
                4,
                &termination,
                Arc::new(|_n: usize| {}),
                &|n| adj[n].clone(),
            )
            .unwrap();
        let parallel_scores = runtime
            .compute_closeness_parallel(
                node_count,
                true,
                4,
                &termination,
                &parallel_farness,
                &parallel_component,
                Arc::new(|_n: usize| {}),
            )
            .unwrap();

        assert_eq!(single_farness, parallel_farness);
        assert_eq!(single_component, parallel_component);
        assert_scores_close(&single_scores, &parallel_scores);
    }
}
