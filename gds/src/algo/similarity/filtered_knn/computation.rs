use crate::algo::similarity::knn::metrics::SimilarityComputer;
use crate::algo::similarity::knn::KnnComputationRuntime;
use crate::algo::similarity::knn::KnnNnDescentConfig;
use crate::algo::similarity::knn::KnnNnDescentStats;
use crate::concurrency::virtual_threads::Executor;
use crate::concurrency::{TerminatedException, TerminationFlag};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct FilteredKnnComputationResult {
    pub source: u64,
    pub target: u64,
    pub similarity: f64,
}

#[derive(Default)]
pub struct FilteredKnnComputationRuntime;

impl FilteredKnnComputationRuntime {
    pub fn new() -> Self {
        Self
    }

    pub fn compute_nn_descent(
        &self,
        node_count: usize,
        initial_neighbors: Vec<Vec<u64>>,
        cfg: KnnNnDescentConfig,
        similarity: Arc<dyn SimilarityComputer>,
        source_allowed: Option<Arc<Vec<bool>>>,
        target_allowed: Option<Arc<Vec<bool>>>,
        executor: &Executor,
        termination: &TerminationFlag,
    ) -> Result<Vec<FilteredKnnComputationResult>, TerminatedException> {
        self.compute_nn_descent_with_stats(
            node_count,
            initial_neighbors,
            cfg,
            similarity,
            source_allowed,
            target_allowed,
            executor,
            termination,
        )
        .map(|result| result.0)
    }

    pub fn compute_nn_descent_with_stats(
        &self,
        node_count: usize,
        initial_neighbors: Vec<Vec<u64>>,
        cfg: KnnNnDescentConfig,
        similarity: Arc<dyn SimilarityComputer>,
        source_allowed: Option<Arc<Vec<bool>>>,
        target_allowed: Option<Arc<Vec<bool>>>,
        executor: &Executor,
        termination: &TerminationFlag,
    ) -> Result<(Vec<FilteredKnnComputationResult>, KnnNnDescentStats), TerminatedException> {
        let engine = KnnComputationRuntime::new();
        let (rows, stats) = engine.compute_nn_descent(
            node_count,
            initial_neighbors,
            cfg,
            similarity,
            source_allowed,
            target_allowed,
            executor,
            termination,
        )?;

        let mapped = rows
            .into_iter()
            .map(|r| FilteredKnnComputationResult {
                source: r.source,
                target: r.target,
                similarity: r.similarity,
            })
            .collect();

        Ok((mapped, stats))
    }
}
