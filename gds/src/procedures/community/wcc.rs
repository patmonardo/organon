//! Weakly Connected Components (WCC) Facade
//!
//! Finds connected components in a graph under *undirected* semantics.
//!
//! Parameters (Java GDS aligned):
//! - `concurrency`: parallel worker hint.
//! - `min_batch_size`: minimum range-partition batch size for directed graphs.
//! - `threshold`: optional relationship property threshold (`property > threshold`).
//! - `seed_property`: optional node property containing initial component ids.

use crate::algo::algorithms::Result;
use crate::algo::algorithms::{ConfigValidator, WriteResult};
use crate::algo::wcc::{
    WccComputationRuntime, WccConfig, WccMutateResult, WccMutationSummary, WccResult,
    WccResultBuilder, WccStats, WccStorageRuntime,
};
use crate::collections::backends::vec::VecLong;
use crate::task::concurrency::TerminationFlag;
use crate::task::progress::{TaskRegistry, Tasks};
use crate::task::memory::MemoryRange;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use crate::types::properties::node::DefaultLongNodePropertyValues;
use crate::types::properties::node::NodePropertyValues;
use crate::types::schema::NodeLabel;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

/// Per-node WCC assignment row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct WccRow {
    pub node_id: u64,
    pub component_id: u64,
}

/// WCC algorithm facade.
#[derive(Clone)]
pub struct WccFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: WccConfig,
    task_registry: Option<TaskRegistry>,
}

impl WccFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: WccConfig::default(),
            task_registry: None,
        }
    }

    /// Create a facade using the spec.rs config model.
    pub fn from_spec_config(
        graph_store: Arc<DefaultGraphStore>,
        config: WccConfig,
    ) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;

        Ok(Self {
            graph_store,
            config,
            task_registry: None,
        })
    }

    /// Parse JSON into spec.rs config and return a configured facade.
    pub fn from_spec_json(
        graph_store: Arc<DefaultGraphStore>,
        raw_config: &serde_json::Value,
    ) -> Result<Self> {
        let parsed: WccConfig = serde_json::from_value(raw_config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        Self::from_spec_config(graph_store, parsed)
    }

    /// Apply a spec.rs config onto an existing facade.
    pub fn with_spec_config(mut self, config: WccConfig) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;
        self.config = config;
        Ok(self)
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.config.concurrency = concurrency;
        self
    }

    pub fn threshold(mut self, threshold: f64) -> Self {
        self.config.threshold = Some(threshold);
        self
    }

    pub fn min_batch_size(mut self, min_batch_size: usize) -> Self {
        self.config.min_batch_size = min_batch_size;
        self
    }

    pub fn seed_property(mut self, seed_property: &str) -> Self {
        self.config.seed_property = Some(seed_property.to_string());
        self
    }

    pub fn task_registry(mut self, task_registry: TaskRegistry) -> Self {
        self.task_registry = Some(task_registry);
        self
    }

    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = WccRow>>> {
        let result = self.compute()?;
        let iter = result
            .components
            .into_iter()
            .enumerate()
            .map(|(node_id, component_id)| WccRow {
                node_id: node_id as u64,
                component_id,
            });
        Ok(Box::new(iter))
    }

    pub fn stats(&self) -> Result<WccStats> {
        let result = self.compute()?;
        Ok(WccResultBuilder::new(result).stats())
    }

    /// Mutate mode: compute components and add as a node property, returning updated store.
    pub fn mutate(self, property_name: &str) -> Result<WccMutateResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        let result = self.compute()?;

        let node_count = self.graph_store.node_count();
        let nodes_updated = node_count as u64;

        let longs: Vec<i64> = result.components.into_iter().map(|c| c as i64).collect();
        let backend = VecLong::from(longs);
        let values = DefaultLongNodePropertyValues::from_collection(backend, node_count);
        let values: Arc<dyn NodePropertyValues> = Arc::new(values);

        let mut new_store = self.graph_store.as_ref().clone();
        let labels: HashSet<NodeLabel> = new_store.node_labels();
        new_store
            .add_node_property(labels, property_name.to_string(), values)
            .map_err(|e| {
                AlgorithmError::Execution(format!("WCC mutate failed to add property: {e}"))
            })?;

        let summary = WccMutationSummary {
            nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: result.execution_time.as_millis() as u64,
        };

        Ok(WccMutateResult {
            summary,
            updated_store: Arc::new(new_store),
        })
    }

    /// Compatibility alias for older application code.
    pub fn mutate_with_store(self, property_name: &str) -> Result<WccMutateResult> {
        self.mutate(property_name)
    }

    /// Write mode: writes component assignments to a new graph/store surface.
    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        let res = self.mutate(property_name)?;
        Ok(WriteResult::new(
            res.summary.nodes_updated,
            property_name.to_string(),
            std::time::Duration::from_millis(res.summary.execution_time_ms),
        ))
    }

    pub fn estimate_memory(&self) -> MemoryRange {
        // Estimate memory for WCC computation
        // - Component assignments: node_count * 8 bytes
        // - Union-find structures: node_count * 16 bytes
        // - Graph view overhead: roughly node_count * 16 bytes
        let node_count = self.graph_store.node_count();
        let assignment_memory = node_count * 8;
        let union_find_memory = node_count * 16;
        let graph_memory = node_count * 16;

        let total = assignment_memory + union_find_memory + graph_memory;
        MemoryRange::of_range(total, total * 2) // Conservative upper bound
    }

    fn validate(&self) -> Result<()> {
        self.config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))
    }

    fn compute(&self) -> Result<WccResult> {
        self.validate()?;
        let start = Instant::now();
        let config = self.config.clone();

        let storage = WccStorageRuntime::new(config.concurrency);
        let mut computation = WccComputationRuntime::new()
            .concurrency(config.concurrency)
            .min_batch_size(config.min_batch_size)
            .threshold(config.threshold);

        if let Some(seed_property) = &config.seed_property {
            let seed_values = self
                .graph_store
                .node_property_values(seed_property)
                .map_err(|e| {
                    AlgorithmError::Execution(format!(
                        "WCC seed property `{}` not found in graph store: {}",
                        seed_property, e
                    ))
                })?;
            computation = computation.seed_property_values(seed_values);
        }

        let leaf =
            Tasks::leaf_with_volume("wcc".to_string(), self.graph_store.relationship_count());
        let mut progress_tracker =
            super::progress_tracker(leaf, config.concurrency, self.task_registry.as_ref());

        let termination_flag = TerminationFlag::running_true();

        let result = storage
            .compute_wcc(
                &mut computation,
                self.graph_store.as_ref(),
                &mut progress_tracker,
                &termination_flag,
            )
            .map_err(AlgorithmError::Execution)?;

        Ok(WccResult {
            components: result.components,
            component_count: result.component_count,
            node_count: self.graph_store.node_count(),
            execution_time: start.elapsed(),
        })
    }

    /// Full result: returns the procedure-level WCC result.
    pub fn run(&self) -> Result<WccResult> {
        let result = self.compute()?;
        Ok(result)
    }
}
