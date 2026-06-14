use super::intersect::{
    AdjacencyProvider, GraphIntersect, RelationshipIntersect, RelationshipIntersectConfig,
};
use super::spec::{TriangleResult, EXCLUDED_NODE_TRIANGLE_COUNT};
use crate::task::concurrency::{install_with_concurrency, Concurrency, TerminationFlag};
use crate::task::progress::{NoopProgressTracker, ProgressTracker};
use rayon::prelude::*;
use std::time::Duration;

pub struct TriangleComputationRuntime {}

impl TriangleComputationRuntime {
    pub fn new() -> Self {
        Self {}
    }

    /// Counts triangles using neighbor-set intersections.
    ///
    /// Contract:
    /// - `get_neighbors(i)` must return a sorted, deduplicated list of neighbor node indices.
    /// - The graph is assumed to be undirected (i.e. if `u` lists `v`, then `v` lists `u`).
    ///
    /// Counting rule:
    /// - Each triangle is counted exactly once by enforcing ordering $a < b < c$.
    pub fn compute(
        &mut self,
        node_count: usize,
        get_neighbors: impl Fn(usize) -> Vec<usize>,
    ) -> TriangleResult {
        self.compute_with_max_degree(node_count, get_neighbors, u64::MAX)
    }

    pub fn compute_with_max_degree(
        &mut self,
        node_count: usize,
        get_neighbors: impl Fn(usize) -> Vec<usize>,
        max_degree: u64,
    ) -> TriangleResult {
        let mut progress_tracker = NoopProgressTracker;
        let termination_flag = TerminationFlag::default();
        self.compute_with_controls(
            node_count,
            get_neighbors,
            max_degree,
            1,
            &mut progress_tracker,
            &termination_flag,
        )
        .expect("default triangle computation should not terminate")
    }

    pub fn compute_with_controls(
        &mut self,
        node_count: usize,
        get_neighbors: impl Fn(usize) -> Vec<usize>,
        max_degree: u64,
        concurrency: usize,
        progress_tracker: &mut dyn ProgressTracker,
        termination_flag: &TerminationFlag,
    ) -> Result<TriangleResult, String> {
        if node_count == 0 {
            return Ok(TriangleResult {
                local_triangles: Vec::new(),
                global_triangles: 0,
                node_count: 0,
                execution_time: Duration::default(),
            });
        }

        let mut neighbors_cache: Vec<Vec<usize>> =
            (0..node_count).map(|n| get_neighbors(n)).collect();
        for n in 0..node_count {
            neighbors_cache[n].sort_unstable();
            neighbors_cache[n].dedup();
        }

        struct CacheProvider<'a> {
            adj: &'a [Vec<usize>],
        }

        impl<'a> AdjacencyProvider for CacheProvider<'a> {
            fn degree(&self, node: usize) -> usize {
                self.adj[node].len()
            }

            fn neighbors(&self, node: usize) -> &[usize] {
                &self.adj[node]
            }
        }

        let provider = CacheProvider {
            adj: &neighbors_cache,
        };

        let max_degree_usize = if max_degree == u64::MAX {
            usize::MAX
        } else {
            max_degree as usize
        };
        let mut local = vec![0i64; node_count];
        for node in 0..node_count {
            if neighbors_cache[node].len() > max_degree_usize {
                local[node] = EXCLUDED_NODE_TRIANGLE_COUNT;
            }
        }

        let concurrency = Concurrency::from_usize(concurrency.max(1));
        let chunk_count = concurrency.value().saturating_mul(8).max(1).min(node_count);
        let chunk_size = node_count.div_ceil(chunk_count);

        let partials: Vec<(Vec<i64>, u64)> = install_with_concurrency(concurrency, || {
            (0..chunk_count)
                .into_par_iter()
                .map(|chunk| {
                    let start = chunk.saturating_mul(chunk_size);
                    let end = (start + chunk_size).min(node_count);
                    let mut local_delta = vec![0i64; node_count];
                    let mut global_delta = 0u64;
                    let mut intersect =
                        GraphIntersect::new(&provider, RelationshipIntersectConfig { max_degree });

                    for a in start..end {
                        termination_flag.assert_running();

                        let mut consumer = |c: usize, b: usize, a: usize| {
                            if a < node_count && b < node_count && c < node_count {
                                local_delta[a] += 1;
                                local_delta[b] += 1;
                                local_delta[c] += 1;
                                global_delta += 1;
                            }
                        };
                        intersect.intersect_all(a, &mut consumer);
                    }

                    (local_delta, global_delta)
                })
                .collect()
        });

        let mut global = 0u64;
        for (local_delta, global_delta) in partials {
            global = global.saturating_add(global_delta);
            for (node, delta) in local_delta.into_iter().enumerate() {
                if local[node] >= 0 {
                    local[node] += delta;
                }
            }
        }
        progress_tracker.log_progress(node_count);

        Ok(TriangleResult {
            local_triangles: local,
            global_triangles: global,
            node_count,
            execution_time: Duration::default(),
        })
    }
}

impl Default for TriangleComputationRuntime {
    fn default() -> Self {
        Self::new()
    }
}
