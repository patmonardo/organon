//! WCC Computation Runtime
//!
//! Java parity reference:
//! - `org.neo4j.gds.wcc.Wcc`
//! - `org.neo4j.gds.wcc.SampledStrategy`
//! - `org.neo4j.gds.wcc.UnsampledStrategy`
//!
//! Notes:
//! - Uses `HugeAtomicDisjointSetStruct` for wait-free parallel union-find.
//! - Uses optional relationship-property threshold filtering (`property > threshold`).

use crate::core::utils::paged::dss::{DisjointSetStruct, HugeAtomicDisjointSetStruct};
use crate::core::utils::partition::{Partition, PartitionUtils, DEFAULT_BATCH_SIZE};
use crate::task::concurrency::virtual_threads::Executor;
use crate::task::concurrency::{Concurrency, TerminationFlag};
use crate::task::progress::ProgressTracker;
use crate::types::graph::Graph;
use crate::types::graph::MappedNodeId;
use crate::types::properties::node::NodePropertyValues;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// WCC computation result.
#[derive(Clone)]
pub struct WccComputationResult {
    pub components: Vec<u64>,
    pub component_count: usize,
}

pub struct WccComputationRuntime {
    concurrency: usize,
    min_batch_size: usize,
    threshold: Option<f64>,
    seed_property_values: Option<Arc<dyn NodePropertyValues>>,
}

impl Default for WccComputationRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl WccComputationRuntime {
    pub fn new() -> Self {
        Self {
            concurrency: 4,
            min_batch_size: DEFAULT_BATCH_SIZE,
            threshold: None,
            seed_property_values: None,
        }
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency.max(1);
        self
    }

    pub fn threshold(mut self, threshold: Option<f64>) -> Self {
        self.threshold = threshold;
        self
    }

    pub fn min_batch_size(mut self, min_batch_size: usize) -> Self {
        self.min_batch_size = min_batch_size.max(1);
        self
    }

    pub fn seed_property_values(
        mut self,
        seed_property_values: Arc<dyn NodePropertyValues>,
    ) -> Self {
        self.seed_property_values = Some(seed_property_values);
        self
    }

    pub fn compute(
        &mut self,
        graph: &dyn Graph,
        progress_tracker: &mut dyn ProgressTracker,
        termination_flag: &TerminationFlag,
    ) -> Result<WccComputationResult, String> {
        let node_count = graph.node_count();
        if node_count == 0 {
            return Ok(WccComputationResult {
                components: Vec::new(),
                component_count: 0,
            });
        }

        let dss = Arc::new(match self.seed_property_values.clone() {
            Some(seed_values) => {
                HugeAtomicDisjointSetStruct::with_communities(node_count, |node| {
                    let property_index = u64::try_from(node)
                        .expect("graph node count must fit node property index space");
                    if seed_values.has_value(property_index) {
                        seed_values.long_value(property_index).unwrap_or(-1)
                    } else {
                        -1
                    }
                })
            }
            None => HugeAtomicDisjointSetStruct::new(node_count),
        });
        let threshold = self.threshold;
        let concurrency = self.concurrency;
        let min_batch_size = self.min_batch_size;

        let characteristics = graph.characteristics();
        if characteristics.is_undirected() || characteristics.is_inverse_indexed() {
            sampled_strategy(
                graph,
                &dss,
                concurrency,
                threshold,
                progress_tracker,
                termination_flag,
            )?;
        } else {
            unsampled_strategy(
                graph,
                &dss,
                concurrency,
                min_batch_size,
                threshold,
                progress_tracker,
                termination_flag,
            )?;
        }

        let (components, component_count) = components_from_dss(node_count, &dss);
        Ok(WccComputationResult {
            components,
            component_count,
        })
    }
}

const NEIGHBOR_ROUNDS: usize = 2;
const SAMPLING_SIZE: usize = 1024;
const RUN_CHECK_NODE_COUNT: usize = 1024;

fn sampled_strategy(
    graph: &dyn Graph,
    dss: &Arc<HugeAtomicDisjointSetStruct>,
    concurrency: usize,
    threshold: Option<f64>,
    progress_tracker: &mut dyn ProgressTracker,
    termination_flag: &TerminationFlag,
) -> Result<(), String> {
    let node_count = graph.node_count();

    let partitions: Vec<Partition> =
        PartitionUtils::range_partition(concurrency, node_count, |p| p, None);
    let executor = Executor::new(Concurrency::of(concurrency));

    // 1) Sample sparse subgraph.
    let sampled_progress: Vec<usize> = executor
        .parallel_map(0, partitions.len(), termination_flag, |idx| {
            let partition = partitions[idx];
            let g = Graph::concurrent_view(graph);
            let fallback = g.default_property_value();

            let mut processed: usize = 0;
            for node in partition.iter() {
                if node % RUN_CHECK_NODE_COUNT == 0 {
                    termination_flag.assert_running();
                }
                let node_id = mapped_node_id(node);

                let mut remaining = NEIGHBOR_ROUNDS;
                if let Some(t) = threshold {
                    for cursor in g.stream_relationships(node_id, t + 1.0) {
                        if remaining == 0 {
                            break;
                        }
                        if cursor.property() > t {
                            dss.union(node, physical_node_index(cursor.target_id()));
                            remaining -= 1;
                        }
                    }
                } else {
                    for cursor in g.stream_relationships(node_id, fallback) {
                        if remaining == 0 {
                            break;
                        }
                        dss.union(node, physical_node_index(cursor.target_id()));
                        remaining -= 1;
                    }
                }

                processed += NEIGHBOR_ROUNDS.min(g.degree(node_id));
            }

            processed
        })
        .map_err(|e| format!("terminated: {e}"))?;

    progress_tracker.log_progress(sampled_progress.into_iter().sum());

    // 2) Approximate largest component.
    let largest_component = find_largest_component(node_count, dss);

    // 3) Link remaining relationships, skipping nodes in the largest component.
    let linked_progress: Vec<usize> = executor
        .parallel_map(0, partitions.len(), termination_flag, |idx| {
            let partition = partitions[idx];
            let g = Graph::concurrent_view(graph);
            let fallback = g.default_property_value();
            let use_inverse = g.characteristics().is_inverse_indexed();

            let mut processed: usize = 0;
            for node in partition.iter() {
                if node % RUN_CHECK_NODE_COUNT == 0 {
                    termination_flag.assert_running();
                }
                let node_id = mapped_node_id(node);

                if dss.set_id_of(node) == largest_component {
                    continue;
                }

                let degree = g.degree(node_id);
                if degree > NEIGHBOR_ROUNDS {
                    if let Some(t) = threshold {
                        let mut skipped = 0usize;
                        for cursor in g.stream_relationships(node_id, t + 1.0) {
                            if cursor.property() > t {
                                skipped += 1;
                                if skipped > NEIGHBOR_ROUNDS {
                                    dss.union(node, physical_node_index(cursor.target_id()));
                                }
                            }
                        }
                    } else {
                        let mut skipped = 0usize;
                        for cursor in g.stream_relationships(node_id, fallback) {
                            skipped += 1;
                            if skipped > NEIGHBOR_ROUNDS {
                                dss.union(node, physical_node_index(cursor.target_id()));
                            }
                        }
                    }

                    processed += degree.saturating_sub(NEIGHBOR_ROUNDS);
                }

                if use_inverse {
                    if let Some(t) = threshold {
                        for cursor in g.stream_inverse_relationships(node_id, t + 1.0) {
                            if cursor.property() > t {
                                dss.union(node, physical_node_index(cursor.source_id()));
                            }
                        }
                    } else {
                        for cursor in g.stream_inverse_relationships(node_id, fallback) {
                            dss.union(node, physical_node_index(cursor.source_id()));
                        }
                    }
                }
            }

            processed
        })
        .map_err(|e| format!("terminated: {e}"))?;

    progress_tracker.log_progress(linked_progress.into_iter().sum());

    Ok(())
}

fn unsampled_strategy(
    graph: &dyn Graph,
    dss: &Arc<HugeAtomicDisjointSetStruct>,
    concurrency: usize,
    min_batch_size: usize,
    threshold: Option<f64>,
    progress_tracker: &mut dyn ProgressTracker,
    termination_flag: &TerminationFlag,
) -> Result<(), String> {
    let node_count = graph.node_count();

    let partitions: Vec<Partition> =
        PartitionUtils::range_partition(concurrency, node_count, |p| p, Some(min_batch_size));

    let executor = Executor::new(Concurrency::of(concurrency));
    let processed: Vec<usize> = executor
        .parallel_map(0, partitions.len(), termination_flag, |idx| {
            let partition = partitions[idx];
            let g = Graph::concurrent_view(graph);
            let fallback = g.default_property_value();

            let mut count = 0usize;
            for node in partition.iter() {
                if node % RUN_CHECK_NODE_COUNT == 0 {
                    termination_flag.assert_running();
                }
                let node_id = mapped_node_id(node);

                if let Some(t) = threshold {
                    for cursor in g.stream_relationships(node_id, t + 1.0) {
                        if cursor.property() > t {
                            dss.union(node, physical_node_index(cursor.target_id()));
                        }
                    }
                } else {
                    for cursor in g.stream_relationships(node_id, fallback) {
                        dss.union(node, physical_node_index(cursor.target_id()));
                    }
                }

                count += g.degree(node_id);
            }

            count
        })
        .map_err(|e| format!("terminated: {e}"))?;

    progress_tracker.log_progress(processed.into_iter().sum());

    Ok(())
}

fn components_from_dss(
    node_count: usize,
    dss: &Arc<HugeAtomicDisjointSetStruct>,
) -> (Vec<u64>, usize) {
    let mut components = vec![0u64; node_count];
    let mut unique_components: HashSet<usize> = HashSet::new();

    for i in 0..node_count {
        let root = dss.set_id_of(i);
        unique_components.insert(root);
        components[i] = u64::try_from(root).expect("component root must fit result ID space");
    }

    (components, unique_components.len())
}

fn find_largest_component(node_count: usize, dss: &Arc<HugeAtomicDisjointSetStruct>) -> usize {
    if node_count == 0 {
        return 0;
    }

    // Deterministic xorshift64* sampler (no external deps).
    let node_count_u64 = u64::try_from(node_count).expect("node count must fit sampling ID space");
    let mut state: u64 = node_count_u64.wrapping_mul(0x9E3779B97F4A7C15);
    let mut counts: HashMap<usize, usize> = HashMap::new();

    let samples = SAMPLING_SIZE.min(node_count.max(1));
    for _ in 0..samples {
        state ^= state >> 12;
        state ^= state << 25;
        state ^= state >> 27;
        let rnd = state.wrapping_mul(0x2545F4914F6CDD1D);
        let node = usize::try_from(rnd % node_count_u64)
            .expect("sampled node must fit physical index space");
        let root = dss.set_id_of(node);
        *counts.entry(root).or_insert(0) += 1;
    }

    let mut best_root = 0usize;
    let mut best_count = 0usize;
    for (root, count) in counts {
        if count > best_count || (count == best_count && root < best_root) {
            best_root = root;
            best_count = count;
        }
    }

    best_root
}

fn mapped_node_id(index: usize) -> MappedNodeId {
    MappedNodeId::try_from(index).expect("graph node count must fit mapped ID space")
}

fn physical_node_index(node_id: MappedNodeId) -> usize {
    node_id
        .to_usize()
        .expect("mapped graph node must fit physical index space")
}
