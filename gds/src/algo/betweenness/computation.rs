//! Betweenness Centrality computation runtime (Brandes).
//!
//! This implements unweighted (BFS) and weighted (Dijkstra) Brandes betweenness.
//!
//! The procedure facade pre-selects source nodes (full or sampled) and supplies
//! neighbor accessors. This module focuses on compute + parallel execution.

use crate::collections::HugeAtomicDoubleArray;
use crate::task::concurrency::{
    virtual_threads::{Executor, WorkerContext},
    Concurrency, TerminatedException, TerminationFlag,
};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque};
use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};
use std::sync::Arc;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BetweennessCentralityComputationResult {
    pub centralities: Vec<f64>,
}

pub struct BetweennessCentralityComputationRuntime {
    centrality: HugeAtomicDoubleArray,
    node_count: usize,
}

impl BetweennessCentralityComputationRuntime {
    pub fn new(node_count: usize) -> Self {
        Self {
            centrality: HugeAtomicDoubleArray::new(node_count),
            node_count,
        }
    }

    pub fn finalize_result(&self) -> BetweennessCentralityComputationResult {
        let mut centralities = vec![0.0; self.node_count];
        for i in 0..self.node_count {
            centralities[i] = self.centrality.get(i);
        }
        BetweennessCentralityComputationResult { centralities }
    }
    pub fn compute_parallel_unweighted(
        &self,
        sources: &[usize],
        divisor: f64,
        concurrency: usize,
        termination: &TerminationFlag,
        on_source_done: Arc<dyn Fn() + Send + Sync>,
        get_neighbors: &(impl Fn(usize) -> Vec<usize> + Send + Sync),
    ) -> Result<(), TerminatedException> {
        let node_count = self.node_count;
        let concurrency = concurrency.max(1);
        let counter = Arc::new(AtomicUsize::new(0));
        let executor = Executor::new(Concurrency::of(concurrency));
        let worker_states = WorkerContext::new(move || UnweightedState::new(node_count));

        executor.scope(termination, |scope| {
            scope.spawn_many(concurrency, |_worker_id| {
                worker_states.with(|state| loop {
                    if !termination.running() {
                        return;
                    }

                    let idx = counter.fetch_add(1, AtomicOrdering::Relaxed);
                    if idx >= sources.len() {
                        break;
                    }

                    let source = sources[idx];
                    if !state.compute_source(
                        source,
                        divisor,
                        &self.centrality,
                        termination,
                        get_neighbors,
                    ) {
                        return;
                    }

                    (on_source_done.as_ref())();
                });
            });
        })?;

        Ok(())
    }

    pub fn compute_parallel_weighted(
        &self,
        sources: &[usize],
        divisor: f64,
        concurrency: usize,
        termination: &TerminationFlag,
        on_source_done: Arc<dyn Fn() + Send + Sync>,
        get_neighbors_weighted: &(impl Fn(usize) -> Vec<(usize, f64)> + Send + Sync),
    ) -> Result<(), TerminatedException> {
        let node_count = self.node_count;
        let concurrency = concurrency.max(1);
        let counter = Arc::new(AtomicUsize::new(0));
        let executor = Executor::new(Concurrency::of(concurrency));
        let worker_states = WorkerContext::new(move || WeightedState::new(node_count));

        executor.scope(termination, |scope| {
            scope.spawn_many(concurrency, |_worker_id| {
                worker_states.with(|state| loop {
                    if !termination.running() {
                        return;
                    }

                    let idx = counter.fetch_add(1, AtomicOrdering::Relaxed);
                    if idx >= sources.len() {
                        break;
                    }

                    let source = sources[idx];
                    if !state.compute_source(
                        source,
                        divisor,
                        &self.centrality,
                        termination,
                        get_neighbors_weighted,
                    ) {
                        return;
                    }

                    (on_source_done.as_ref())();
                });
            });
        })?;

        Ok(())
    }
}

impl Default for BetweennessCentralityComputationRuntime {
    fn default() -> Self {
        Self::new(0)
    }
}

struct UnweightedState {
    predecessors: Vec<Vec<usize>>,
    queue: VecDeque<usize>,
    stack: Vec<usize>,
    touched: Vec<usize>,
    dist: Vec<i32>,
    sigma: Vec<u64>,
    delta: Vec<f64>,
}

impl UnweightedState {
    fn new(node_count: usize) -> Self {
        Self {
            predecessors: (0..node_count).map(|_| Vec::new()).collect(),
            queue: VecDeque::new(),
            stack: Vec::new(),
            touched: Vec::new(),
            dist: vec![-1; node_count],
            sigma: vec![0; node_count],
            delta: vec![0.0; node_count],
        }
    }

    fn reset(&mut self) {
        for &n in &self.touched {
            self.predecessors[n].clear();
            self.dist[n] = -1;
            self.sigma[n] = 0;
            self.delta[n] = 0.0;
        }
        self.touched.clear();
        self.queue.clear();
        self.stack.clear();
    }

    fn touch(&mut self, node: usize) {
        self.touched.push(node);
    }

    fn compute_source(
        &mut self,
        source: usize,
        divisor: f64,
        centrality: &HugeAtomicDoubleArray,
        termination: &TerminationFlag,
        get_neighbors: &(impl Fn(usize) -> Vec<usize> + Send + Sync),
    ) -> bool {
        self.reset();

        self.dist[source] = 0;
        self.sigma[source] = 1;
        self.touch(source);
        self.queue.push_back(source);

        while let Some(v) = self.queue.pop_front() {
            if !termination.running() {
                return false;
            }

            self.stack.push(v);
            let dv = self.dist[v];

            for w in (get_neighbors)(v) {
                let dw = self.dist[w];
                if dw < 0 {
                    self.dist[w] = dv + 1;
                    self.touch(w);
                    self.queue.push_back(w);
                }

                if self.dist[w] == dv + 1 {
                    self.sigma[w] = self.sigma[w].saturating_add(self.sigma[v]);
                    self.predecessors[w].push(v);
                }
            }
        }

        while let Some(w) = self.stack.pop() {
            if !termination.running() {
                return false;
            }

            let sigma_w = self.sigma[w] as f64;
            if sigma_w > 0.0 {
                let dependency_w = self.delta[w];
                let coeff = (1.0 + dependency_w) / sigma_w;

                for &v in &self.predecessors[w] {
                    let add = (self.sigma[v] as f64) * coeff;
                    self.delta[v] += add;
                }
            }

            if w != source {
                centrality.get_and_add(w, self.delta[w] / divisor);
            }
        }

        true
    }
}

struct WeightedState {
    predecessors: Vec<Vec<usize>>,
    stack: Vec<usize>,
    touched: Vec<usize>,
    dist: Vec<f64>,
    sigma: Vec<u64>,
    delta: Vec<f64>,
    heap: BinaryHeap<WeightedQueueItem>,
}

#[derive(Debug, Clone)]
struct WeightedQueueItem {
    cost: f64,
    node: usize,
}

impl PartialEq for WeightedQueueItem {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node && self.cost == other.cost
    }
}

impl Eq for WeightedQueueItem {}

impl PartialOrd for WeightedQueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for WeightedQueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap (lower cost = higher priority)
        other
            .cost
            .partial_cmp(&self.cost)
            .unwrap_or(Ordering::Equal)
    }
}

impl WeightedState {
    fn new(node_count: usize) -> Self {
        Self {
            predecessors: (0..node_count).map(|_| Vec::new()).collect(),
            stack: Vec::new(),
            touched: Vec::new(),
            dist: vec![f64::INFINITY; node_count],
            sigma: vec![0; node_count],
            delta: vec![0.0; node_count],
            heap: BinaryHeap::new(),
        }
    }

    fn reset(&mut self) {
        for &n in &self.touched {
            self.predecessors[n].clear();
            self.dist[n] = f64::INFINITY;
            self.sigma[n] = 0;
            self.delta[n] = 0.0;
        }
        self.touched.clear();
        self.stack.clear();
        self.heap.clear();
    }

    fn touch(&mut self, node: usize) {
        self.touched.push(node);
    }

    fn compute_source(
        &mut self,
        source: usize,
        divisor: f64,
        centrality: &HugeAtomicDoubleArray,
        termination: &TerminationFlag,
        get_neighbors: &(impl Fn(usize) -> Vec<(usize, f64)> + Send + Sync),
    ) -> bool {
        self.reset();

        self.dist[source] = 0.0;
        self.sigma[source] = 1;
        self.touch(source);
        self.heap.push(WeightedQueueItem {
            cost: 0.0,
            node: source,
        });

        while let Some(WeightedQueueItem {
            cost: cost_v,
            node: v,
        }) = self.heap.pop()
        {
            if !termination.running() {
                return false;
            }

            if cost_v != self.dist[v] {
                continue; // stale heap entry
            }

            self.stack.push(v);

            for (w, weight) in (get_neighbors)(v) {
                if !weight.is_finite() || weight < 0.0 {
                    continue;
                }

                let alt = cost_v + weight;
                let current = self.dist[w];

                if alt < current {
                    if current == f64::INFINITY {
                        self.touch(w);
                    }
                    self.dist[w] = alt;
                    self.sigma[w] = self.sigma[v];
                    self.predecessors[w].clear();
                    self.predecessors[w].push(v);
                    self.heap.push(WeightedQueueItem { cost: alt, node: w });
                } else if alt == current {
                    self.sigma[w] = self.sigma[w].saturating_add(self.sigma[v]);
                    self.predecessors[w].push(v);
                }
            }
        }

        while let Some(w) = self.stack.pop() {
            if !termination.running() {
                return false;
            }

            let sigma_w = self.sigma[w] as f64;
            if sigma_w > 0.0 {
                let dependency_w = self.delta[w];
                let coeff = (1.0 + dependency_w) / sigma_w;

                for &v in &self.predecessors[w] {
                    let add = (self.sigma[v] as f64) * coeff;
                    self.delta[v] += add;
                }
            }

            if w != source {
                centrality.get_and_add(w, self.delta[w] / divisor);
            }
        }

        true
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
    fn unweighted_path_middle_has_betweenness_one() {
        // 0-1-2
        let node_count = 3;
        let adj = undirected_adj(&[(0, 1), (1, 2)], node_count);
        let neighbors = |n: usize| adj[n].clone();

        let termination = TerminationFlag::running_true();
        let noop = Arc::new(|| {});
        let computation = BetweennessCentralityComputationRuntime::new(node_count);
        computation
            .compute_parallel_unweighted(&[0, 1, 2], 2.0, 2, &termination, noop, &neighbors)
            .unwrap();
        let result = computation.finalize_result();

        assert!((result.centralities[1] - 1.0).abs() < 1e-9);
        assert!(result.centralities[0].abs() < 1e-9);
        assert!(result.centralities[2].abs() < 1e-9);
    }

    #[test]
    fn unweighted_cycle_has_equal_scores() {
        // 0-1-2-3-0
        let node_count = 4;
        let adj = undirected_adj(&[(0, 1), (1, 2), (2, 3), (3, 0)], node_count);
        let neighbors = |n: usize| adj[n].clone();

        let termination = TerminationFlag::running_true();
        let noop = Arc::new(|| {});
        let computation = BetweennessCentralityComputationRuntime::new(node_count);
        computation
            .compute_parallel_unweighted(&[0, 1, 2, 3], 2.0, 2, &termination, noop, &neighbors)
            .unwrap();
        let result = computation.finalize_result();

        // In a 4-cycle, each node has the same betweenness.
        for i in 1..node_count {
            assert!((result.centralities[i] - result.centralities[0]).abs() < 1e-9);
        }
    }

    #[test]
    fn unweighted_parallel_matches_single_worker() {
        let node_count = 6;
        let adj = undirected_adj(
            &[(0, 1), (1, 2), (2, 3), (1, 4), (4, 5), (2, 5)],
            node_count,
        );
        let sources: Vec<usize> = (0..node_count).collect();

        let termination = TerminationFlag::running_true();
        let single = BetweennessCentralityComputationRuntime::new(node_count);
        single
            .compute_parallel_unweighted(&sources, 2.0, 1, &termination, Arc::new(|| {}), &|n| {
                adj[n].clone()
            })
            .unwrap();

        let termination = TerminationFlag::running_true();
        let parallel = BetweennessCentralityComputationRuntime::new(node_count);
        parallel
            .compute_parallel_unweighted(&sources, 2.0, 4, &termination, Arc::new(|| {}), &|n| {
                adj[n].clone()
            })
            .unwrap();

        assert_scores_close(
            &single.finalize_result().centralities,
            &parallel.finalize_result().centralities,
        );
    }

    #[test]
    fn weighted_prefers_unique_shortest_path() {
        // 0-1-2 (weight 1 each), 0-2 direct weight 10.
        // Unique shortest path between 0 and 2 goes through 1.
        let node_count = 3;
        let mut adj = vec![Vec::<(usize, f64)>::new(); node_count];
        adj[0].push((1, 1.0));
        adj[1].push((0, 1.0));
        adj[1].push((2, 1.0));
        adj[2].push((1, 1.0));
        adj[0].push((2, 10.0));
        adj[2].push((0, 10.0));

        let neighbors = |n: usize| adj[n].clone();

        let termination = TerminationFlag::running_true();
        let noop = Arc::new(|| {});
        let computation = BetweennessCentralityComputationRuntime::new(node_count);
        computation
            .compute_parallel_weighted(&[0, 1, 2], 2.0, 2, &termination, noop, &neighbors)
            .unwrap();
        let result = computation.finalize_result();

        assert!(result.centralities[1] > 0.0);
        assert!(result.centralities[0] >= 0.0);
        assert!(result.centralities[2] >= 0.0);
    }

    #[test]
    fn weighted_parallel_matches_single_worker_with_equal_paths() {
        let node_count = 5;
        let mut adj = vec![Vec::<(usize, f64)>::new(); node_count];
        let mut add_edge = |source: usize, target: usize, weight: f64| {
            adj[source].push((target, weight));
            adj[target].push((source, weight));
        };

        add_edge(0, 1, 1.0);
        add_edge(1, 3, 1.0);
        add_edge(0, 2, 1.0);
        add_edge(2, 3, 1.0);
        add_edge(3, 4, 1.0);
        add_edge(1, 2, 2.0);

        let sources: Vec<usize> = (0..node_count).collect();

        let termination = TerminationFlag::running_true();
        let single = BetweennessCentralityComputationRuntime::new(node_count);
        single
            .compute_parallel_weighted(&sources, 2.0, 1, &termination, Arc::new(|| {}), &|n| {
                adj[n].clone()
            })
            .unwrap();

        let termination = TerminationFlag::running_true();
        let parallel = BetweennessCentralityComputationRuntime::new(node_count);
        parallel
            .compute_parallel_weighted(&sources, 2.0, 4, &termination, Arc::new(|| {}), &|n| {
                adj[n].clone()
            })
            .unwrap();

        assert_scores_close(
            &single.finalize_result().centralities,
            &parallel.finalize_result().centralities,
        );
    }

    #[test]
    fn uniform_weighted_matches_unweighted() {
        let node_count = 4;
        let adj = undirected_adj(&[(0, 1), (1, 2), (2, 3)], node_count);
        let weighted_adj: Vec<Vec<(usize, f64)>> = adj
            .iter()
            .map(|neighbors| neighbors.iter().map(|node| (*node, 1.0)).collect())
            .collect();

        let termination = TerminationFlag::running_true();
        let noop = Arc::new(|| {});
        let unweighted = BetweennessCentralityComputationRuntime::new(node_count);
        unweighted
            .compute_parallel_unweighted(&[0, 1, 2, 3], 2.0, 2, &termination, noop, &|n| {
                adj[n].clone()
            })
            .unwrap();

        let termination = TerminationFlag::running_true();
        let noop = Arc::new(|| {});
        let weighted = BetweennessCentralityComputationRuntime::new(node_count);
        weighted
            .compute_parallel_weighted(&[0, 1, 2, 3], 2.0, 2, &termination, noop, &|n| {
                weighted_adj[n].clone()
            })
            .unwrap();

        let unweighted_result = unweighted.finalize_result();
        let weighted_result = weighted.finalize_result();
        for (left, right) in unweighted_result
            .centralities
            .iter()
            .zip(weighted_result.centralities.iter())
        {
            assert!((left - right).abs() < 1e-9);
        }
    }
}
