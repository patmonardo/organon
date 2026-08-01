//! Label Propagation storage runtime

use super::computation::{
    LabelPropComputationRuntime, LabelPropResult as LabelPropComputationResult,
};
use super::spec::LabelPropConfig;
use crate::task::concurrency::TerminationFlag;
use crate::task::progress::ProgressTracker;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::Orientation;
use crate::projection::RelationshipType;
use crate::types::default_value::LONG_DEFAULT_FALLBACK;
use crate::types::graph::MappedNodeId;
use crate::types::prelude::GraphStore;
use crate::types::properties::node::NodePropertyValues;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::types::graph::Graph;

#[derive(Clone)]
pub struct LabelPropStorageRuntime {
    graph: Arc<dyn Graph>,
    node_properties: HashMap<String, Arc<dyn NodePropertyValues>>,
}

impl LabelPropStorageRuntime {
    pub fn new<G: GraphStore>(graph_store: &G) -> Result<Self, AlgorithmError> {
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
        })
    }

    pub fn graph(&self) -> Arc<dyn Graph> {
        Arc::clone(&self.graph)
    }

    pub fn node_count(&self) -> usize {
        self.graph.node_count() as usize
    }

    /// Controller entrypoint: owns graph access, materializes weights/seeds, wires progress,
    /// enforces termination, and delegates to the pure computation runtime.
    pub fn compute_label_propagation(
        &self,
        computation: LabelPropComputationRuntime,
        config: &LabelPropConfig,
        progress_tracker: &mut dyn ProgressTracker,
        termination_flag: &TerminationFlag,
    ) -> Result<LabelPropComputationResult, AlgorithmError> {
        let node_count = self.graph.node_count() as usize;
        if node_count == 0 {
            return Ok(LabelPropComputationResult {
                labels: Vec::new(),
                did_converge: true,
                ran_iterations: 0,
            });
        }

        termination_flag.assert_running();

        let weights: Vec<f64> = match &config.node_weight_property {
            Some(key) => {
                let property = self.node_properties.get(key).ok_or_else(|| {
                    AlgorithmError::Execution(format!("node weight property '{key}' not found"))
                })?;
                (0..node_count)
                    .map(|node| {
                        termination_flag.assert_running();
                        let weight = property.double_value(node as u64).map_err(|error| {
                            AlgorithmError::Execution(format!(
                                "cannot read node weight property '{key}' for node {node}: {error}"
                            ))
                        })?;
                        if !weight.is_finite() || weight < 0.0 {
                            return Err(AlgorithmError::Execution(format!(
                                "node weight property '{key}' must be finite and non-negative for node {node}, got {weight}"
                            )));
                        }
                        Ok(weight)
                    })
                    .collect::<Result<Vec<_>, AlgorithmError>>()?
            }
            None => vec![1.0; node_count],
        };

        // Seed labels (Java InitStep parity):
        // - If a seedProperty exists and has a value: use it.
        // - Otherwise: label = maxLabelId + originalNodeId + 1.
        // This avoids collisions with node IDs while keeping determinism.
        let relationship_volume = self
            .graph
            .relationship_count()
            .saturating_mul(config.max_iterations as usize);
        progress_tracker.begin_subtask_with_volume(node_count.saturating_add(relationship_volume));

        let seed_pv = config
            .seed_property
            .as_ref()
            .map(|key| {
                self.node_properties.get(key).cloned().ok_or_else(|| {
                    AlgorithmError::Execution(format!("seed property '{key}' not found"))
                })
            })
            .transpose()?;

        let max_label_id: i64 = seed_pv
            .as_deref()
            .and_then(|pv| pv.get_max_long_property_value())
            .unwrap_or(-1);

        let mut seeds: Vec<u64> = Vec::with_capacity(node_count);
        for i in 0..node_count {
            termination_flag.assert_running();

            let node_id = MappedNodeId::try_from(i).map_err(|_| {
                AlgorithmError::Execution(format!(
                    "node index {i} exceeds the mapped ID domain"
                ))
            })?;
            let original = self
                .graph
                .to_original_node_id(node_id)
                .ok_or_else(|| {
                    AlgorithmError::Execution(format!(
                        "mapped node {node_id} has no original node ID"
                    ))
                })?
                .get();

            let label = match seed_pv.as_deref() {
                Some(pv) if pv.has_value(i as u64) => {
                    let seed = pv.long_value(i as u64).map_err(|error| {
                        AlgorithmError::Execution(format!(
                            "cannot read seed property for node {i}: {error}"
                        ))
                    })?;
                    if seed == LONG_DEFAULT_FALLBACK || seed < 0 {
                        return Err(AlgorithmError::Execution(format!(
                            "seed property must be non-negative for node {i}, got {seed}"
                        )));
                    }
                    seed as u64
                }
                _ => (max_label_id + original + 1) as u64,
            };

            seeds.push(label);
            progress_tracker.log_progress(1);
        }

        let fallback = 1.0;
        let graph = Arc::clone(&self.graph);
        let neighbors = move |node_idx: usize| -> Vec<(usize, f64)> {
            let node_id = MappedNodeId::try_from(node_idx)
                .expect("graph node count must fit the mapped ID domain");
            graph
                .stream_relationships_weighted(node_id, fallback)
                .map(|cursor| (cursor.target_id(), cursor.weight()))
                .map(|(target, weight)| {
                    (
                        target
                            .to_usize()
                            .expect("mapped target must fit the dense index domain"),
                        weight,
                    )
                })
                .collect()
        };

        let result = computation
            .with_weights(weights)
            .with_seeds(seeds)
            .compute_with_controls(
                node_count as u64,
                neighbors,
                progress_tracker,
                termination_flag,
            )
            .map_err(AlgorithmError::Execution)?;
        progress_tracker.end_subtask();

        Ok(result)
    }
}
