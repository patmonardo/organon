use crate::algo::modularity_optimization::ModularityOptimizationInput;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::Orientation;
use crate::projection::RelationshipType;
use crate::task::concurrency::TerminationFlag;
use crate::task::progress::ProgressTracker;
use crate::types::graph::Graph;
use crate::types::graph::MappedNodeId;
use crate::types::prelude::GraphStore;
use crate::types::properties::node::NodePropertyValues;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

use super::spec::{LouvainConfig, LouvainResult};
use super::LouvainComputationRuntime;

#[derive(Clone)]
pub struct LouvainStorageRuntime {
    graph: Arc<dyn Graph>,
    node_properties: HashMap<String, Arc<dyn NodePropertyValues>>,
    #[allow(dead_code)]
    concurrency: usize,
}

impl LouvainStorageRuntime {
    pub fn new<G: GraphStore>(graph_store: &G, concurrency: usize) -> Result<Self, AlgorithmError> {
        let mut node_properties = HashMap::new();
        for key in graph_store.node_property_keys() {
            if let Ok(values) = graph_store.node_property_values(&key) {
                node_properties.insert(key, values);
            }
        }

        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph = graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Undirected)
            .map_err(|e| AlgorithmError::Graph(e.to_string()))?;
        Ok(Self {
            graph,
            node_properties,
            concurrency,
        })
    }

    pub fn node_count(&self) -> usize {
        self.graph.node_count() as usize
    }

    pub fn compute_louvain(
        &self,
        computation: &mut LouvainComputationRuntime,
        config: &LouvainConfig,
        progress_tracker: &mut dyn ProgressTracker,
        termination_flag: &TerminationFlag,
    ) -> Result<LouvainResult, AlgorithmError> {
        let node_count = self.graph.node_count();
        if node_count == 0 {
            return Ok(LouvainResult {
                data: Vec::new(),
                ran_levels: 0,
                modularities: Vec::new(),
                modularity: 0.0,
                intermediate_communities: config.include_intermediate_communities.then(Vec::new),
                node_count: 0,
                execution_time: std::time::Duration::default(),
            });
        }

        let seed_mapping = self.seed_mapping(config, termination_flag)?;

        // For Louvain, treat unweighted relationships as weight=1.0 (matches other procedures).
        let weight_fallback = 1.0;

        progress_tracker.begin_subtask_with_volume(node_count.saturating_add(config.max_levels));

        let mut adj: Vec<Vec<(usize, f64)>> = vec![Vec::new(); node_count];
        for node_id in 0..node_count {
            termination_flag.assert_running();
            let mapped_node_id = MappedNodeId::try_from(node_id).map_err(|_| {
                AlgorithmError::Execution(format!(
                    "node index {node_id} exceeds the mapped ID domain"
                ))
            })?;
            let stream = self
                .graph
                .stream_relationships_weighted(mapped_node_id, weight_fallback);
            for cursor in stream {
                let target = cursor.target_id().to_usize().ok_or_else(|| {
                    AlgorithmError::Execution(format!(
                        "mapped target {} exceeds the dense index domain",
                        cursor.target_id()
                    ))
                })?;
                adj[node_id].push((target, cursor.weight()));
            }
            progress_tracker.log_progress(1);
        }

        let input = ModularityOptimizationInput::new(node_count, adj);
        *computation = std::mem::take(computation).with_config(config);
        let result = computation
            .compute_with_controls(
                &input,
                config,
                seed_mapping
                    .as_ref()
                    .map(|mapping| mapping.assignments.as_slice()),
                seed_mapping
                    .as_ref()
                    .map(|mapping| mapping.external_ids.as_slice()),
                termination_flag,
                || progress_tracker.log_progress(1),
            )
            .map_err(AlgorithmError::Execution)?;

        progress_tracker.end_subtask();

        Ok(result)
    }

    fn seed_mapping(
        &self,
        config: &LouvainConfig,
        termination_flag: &TerminationFlag,
    ) -> Result<Option<SeedMapping>, AlgorithmError> {
        let Some(property_name) = &config.seed_property else {
            return Ok(None);
        };
        let values = self.node_properties.get(property_name).ok_or_else(|| {
            AlgorithmError::Execution(format!("Seed property `{property_name}` does not exist"))
        })?;

        let max_seed = values.get_max_long_property_value().unwrap_or(0);
        let mut internal_by_external = HashMap::new();
        let mut external_ids = Vec::new();
        let mut assignments = Vec::with_capacity(self.graph.node_count());

        for node in 0..self.graph.node_count() {
            termination_flag.assert_running();
            let external = if values.has_value(node as u64) {
                let seed = values.long_value(node as u64).map_err(|error| {
                    AlgorithmError::Execution(format!(
                        "Failed to read seed property `{property_name}` for node {node}: {error}"
                    ))
                })?;
                if seed < 0 {
                    return Err(AlgorithmError::Execution(format!(
                        "Seed property `{property_name}` must contain non-negative values"
                    )));
                }
                seed as u64
            } else {
                let mapped_node_id = MappedNodeId::try_from(node).map_err(|_| {
                    AlgorithmError::Execution(format!(
                        "node index {node} exceeds the mapped ID domain"
                    ))
                })?;
                let original = self
                    .graph
                    .to_original_node_id(mapped_node_id)
                    .ok_or_else(|| {
                        AlgorithmError::Execution(format!(
                            "mapped node {mapped_node_id} has no original node ID"
                        ))
                    })?
                    .get();
                max_seed.saturating_add(original) as u64
            };

            let next_id = internal_by_external.len() as u64;
            let internal = *internal_by_external.entry(external).or_insert_with(|| {
                external_ids.push(external);
                next_id
            });
            assignments.push(internal);
        }

        Ok(Some(SeedMapping {
            assignments,
            external_ids,
        }))
    }
}

struct SeedMapping {
    assignments: Vec<u64>,
    external_ids: Vec<u64>,
}
