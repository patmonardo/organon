use crate::task::concurrency::virtual_threads::{Executor, WorkerContext};
use crate::task::concurrency::{Concurrency, TerminatedException, TerminationFlag};
use crate::types::graph::graph::Graph;
use crate::types::graph::MappedNodeId;
use std::collections::HashSet;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use super::{
    NodeSimilarityComputationReport, NodeSimilarityComputationResult,
    NodeSimilarityComputationRuntime,
};

pub struct NodeSimilarityStorageRuntime {
    concurrency: usize,
}

impl NodeSimilarityStorageRuntime {
    pub fn new(concurrency: usize) -> Self {
        Self { concurrency }
    }

    pub fn compute(
        &self,
        computation: &NodeSimilarityComputationRuntime,
        graph: &dyn Graph,
        config: &super::spec::NodeSimilarityConfig,
        termination: &TerminationFlag,
        on_sources_done: Arc<dyn Fn(usize) + Send + Sync>,
    ) -> Result<Vec<NodeSimilarityComputationResult>, TerminatedException> {
        Ok(self
            .compute_report(computation, graph, config, termination, on_sources_done)?
            .results)
    }

    pub fn compute_report(
        &self,
        computation: &NodeSimilarityComputationRuntime,
        graph: &dyn Graph,
        config: &super::spec::NodeSimilarityConfig,
        termination: &TerminationFlag,
        on_sources_done: Arc<dyn Fn(usize) + Send + Sync>,
    ) -> Result<NodeSimilarityComputationReport, TerminatedException> {
        let node_count = graph.node_count();
        let sources: Vec<u64> = (0..node_count as u64).collect();
        self.compute_with_filters_report(
            computation,
            graph,
            config,
            sources,
            None,
            true,
            termination,
            on_sources_done,
        )
    }

    pub fn compute_with_filters(
        &self,
        computation: &NodeSimilarityComputationRuntime,
        graph: &dyn Graph,
        config: &super::spec::NodeSimilarityConfig,
        sources: Vec<u64>,
        target_mask: Option<Vec<bool>>,
        enforce_ordering: bool,
        termination: &TerminationFlag,
        on_sources_done: Arc<dyn Fn(usize) + Send + Sync>,
    ) -> Result<Vec<NodeSimilarityComputationResult>, TerminatedException> {
        Ok(self
            .compute_with_filters_report(
                computation,
                graph,
                config,
                sources,
                target_mask,
                enforce_ordering,
                termination,
                on_sources_done,
            )?
            .results)
    }

    pub fn compute_with_filters_report(
        &self,
        computation: &NodeSimilarityComputationRuntime,
        graph: &dyn Graph,
        config: &super::spec::NodeSimilarityConfig,
        sources: Vec<u64>,
        target_mask: Option<Vec<bool>>,
        enforce_ordering: bool,
        termination: &TerminationFlag,
        on_sources_done: Arc<dyn Fn(usize) + Send + Sync>,
    ) -> Result<NodeSimilarityComputationReport, TerminatedException> {
        if !termination.running() {
            return Err(TerminatedException);
        }

        let node_count = graph.node_count();
        let metric = Arc::from(config.similarity_metric.create(config.similarity_cutoff));
        let worker_count = self.concurrency.max(1);
        let executor = Executor::new(Concurrency::of(worker_count));

        let use_weights = config.weight_property.is_some();
        let (vectors, weights) =
            build_vectors(graph, node_count, use_weights, &executor, termination)?;
        let weights_ref = weights.as_deref();

        let target_mask_ref = target_mask.as_deref();
        let compared_nodes = sources.len() as u64;
        let source_counter = AtomicUsize::new(0);
        let completed_sources = AtomicUsize::new(0);
        let worker_results = WorkerContext::new(Vec::<NodeSimilarityComputationResult>::new);

        executor.scope(termination, |scope| {
            scope.spawn_many(worker_count, |_| {
                worker_results.with(|local| {
                    if !termination.running() {
                        return;
                    }

                    loop {
                        if !termination.running() {
                            break;
                        }

                        let idx = source_counter.fetch_add(1, Ordering::Relaxed);
                        if idx >= sources.len() {
                            break;
                        }

                        let source = sources[idx];
                        let source_vec = &vectors[source as usize];

                        if source_vec.is_empty() {
                            (on_sources_done.as_ref())(1);
                            completed_sources.fetch_add(1, Ordering::Relaxed);
                            continue;
                        }

                        let candidates = enumerate_candidates(
                            graph,
                            source,
                            source_vec,
                            target_mask_ref,
                            enforce_ordering,
                        );

                        local.extend(computation.score_source_from_candidates(
                            source,
                            candidates,
                            &vectors,
                            weights_ref,
                            &metric,
                            config.top_k,
                        ));

                        (on_sources_done.as_ref())(1);
                        completed_sources.fetch_add(1, Ordering::Relaxed);
                    }
                });
            });
        })?;

        let mut merged = Vec::new();
        for mut part in worker_results.collect() {
            merged.append(&mut part);
        }

        Ok(NodeSimilarityComputationReport {
            results: computation.select_top_n(merged, config.top_n),
            compared_nodes,
            completed_sources: completed_sources.load(Ordering::Relaxed),
        })
    }
}

fn build_vectors(
    graph: &dyn Graph,
    node_count: usize,
    with_weights: bool,
    executor: &Executor,
    termination: &TerminationFlag,
) -> Result<(Vec<Vec<u64>>, Option<Vec<Vec<f64>>>), TerminatedException> {
    // Use 1.0 as the fallback weight so missing/unselected properties behave as unweighted.
    let fallback_weight = 1.0;

    if with_weights {
        let slots: Vec<parking_lot::Mutex<Option<(Vec<u64>, Vec<f64>)>>> = (0..node_count)
            .map(|_| parking_lot::Mutex::new(None))
            .collect();

        executor.scope(termination, |scope| {
            scope.spawn_many(node_count, |node| {
                if !termination.running() {
                    return;
                }

                let node_id = node as MappedNodeId;
                let mut pairs: Vec<(u64, f64)> = graph
                    .stream_relationships(node_id, fallback_weight)
                    .map(|cursor| (cursor.target_id() as u64, cursor.property()))
                    .collect();

                pairs.sort_unstable_by_key(|(t, _)| *t);

                let mut neighbors = Vec::with_capacity(pairs.len());
                let mut node_weights = Vec::with_capacity(pairs.len());
                for (t, w) in pairs {
                    neighbors.push(t);
                    node_weights.push(w);
                }
                *slots[node].lock() = Some((neighbors, node_weights));
            });
        })?;

        let mut vectors = Vec::with_capacity(node_count);
        let mut weights = Vec::with_capacity(node_count);
        for slot in slots {
            let (v, w) = slot.into_inner().unwrap_or_default();
            vectors.push(v);
            weights.push(w);
        }

        Ok((vectors, Some(weights)))
    } else {
        let slots: Vec<parking_lot::Mutex<Option<Vec<u64>>>> = (0..node_count)
            .map(|_| parking_lot::Mutex::new(None))
            .collect();

        executor.scope(termination, |scope| {
            scope.spawn_many(node_count, |node| {
                if !termination.running() {
                    return;
                }

                let node_id = node as MappedNodeId;
                let mut neighbors: Vec<u64> = graph
                    .stream_relationships(node_id, fallback_weight)
                    .map(|cursor| cursor.target_id() as u64)
                    .collect();
                neighbors.sort_unstable();
                *slots[node].lock() = Some(neighbors);
            });
        })?;

        let vectors: Vec<Vec<u64>> = slots
            .into_iter()
            .map(|slot| slot.into_inner().unwrap_or_default())
            .collect();

        Ok((vectors, None))
    }
}

fn enumerate_candidates(
    graph: &dyn Graph,
    source: u64,
    source_vector: &[u64],
    target_mask: Option<&[bool]>,
    enforce_ordering: bool,
) -> HashSet<u64> {
    // Use a HashSet to dedupe candidates across multiple shared neighbors.
    // Use 1.0 fallback for property access; candidate enumeration is topology-only.
    let fallback_weight = 1.0;
    let mut candidates: HashSet<u64> = HashSet::new();

    // Inverse traversal for each neighbor to find nodes that share that neighbor.
    // (This matches the classic two-hop candidate enumeration in Node Similarity.)
    for &neighbor in source_vector {
        let inverse = graph.stream_inverse_relationships(neighbor as MappedNodeId, fallback_weight);
        for inv_rel in inverse {
            let potential_target = inv_rel.source_id() as u64;
            if potential_target == source {
                continue;
            }

            if enforce_ordering && potential_target <= source {
                continue;
            }

            if let Some(mask) = target_mask {
                let idx = potential_target as usize;
                if idx >= mask.len() || !mask[idx] {
                    continue;
                }
            }

            candidates.insert(potential_target);
        }
    }

    candidates
}
