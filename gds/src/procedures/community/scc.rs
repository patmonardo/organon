//! Strongly Connected Components (SCC) Facade
//!
//! Finds SCCs in a directed graph and returns:
//! - per-node component assignment
//! - component count and execution time stats
//!
//! Parameters:
//! - `concurrency`: accepted for Java GDS/Pipeline alignment; SCC computation is sequential.

use crate::algo::algorithms::Result;
use crate::algo::algorithms::{ConfigValidator, WriteResult};
use crate::algo::scc::{
    SccComputationRuntime, SccConfig, SccMutateResult, SccMutationSummary, SccResult,
    SccResultBuilder, SccStats, SccStorageRuntime,
};
use crate::collections::backends::vec::VecLong;
use crate::task::concurrency::TerminationFlag;
use crate::core::utils::progress::{TaskRegistry, Tasks};
use crate::task::memory::MemoryRange;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use crate::types::properties::node::DefaultLongNodePropertyValues;
use crate::types::properties::node::NodePropertyValues;
use crate::types::schema::NodeLabel;
use std::collections::HashSet;
use std::sync::Arc;

/// Per-node SCC assignment row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct SccRow {
    pub node_id: u64,
    pub component_id: u64,
}

/// SCC algorithm facade.
#[derive(Clone)]
pub struct SccFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: SccConfig,
    task_registry: Option<TaskRegistry>,
}

impl SccFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: SccConfig::default(),
            task_registry: None,
        }
    }

    /// Create a facade using the spec.rs config model.
    pub fn from_spec_config(
        graph_store: Arc<DefaultGraphStore>,
        config: SccConfig,
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
        let parsed: SccConfig = serde_json::from_value(raw_config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        Self::from_spec_config(graph_store, parsed)
    }

    /// Apply a spec.rs config onto an existing facade.
    pub fn with_spec_config(mut self, config: SccConfig) -> Result<Self> {
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

    pub fn task_registry(mut self, task_registry: TaskRegistry) -> Self {
        self.task_registry = Some(task_registry);
        self
    }

    fn validate(&self) -> Result<()> {
        self.config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))
    }

    fn compute(&self) -> Result<SccResult> {
        self.validate()?;
        let start = std::time::Instant::now();
        let config = self.config.clone();

        let mut computation = SccComputationRuntime::new();
        let storage = SccStorageRuntime::new(config.concurrency);

        let leaf = Tasks::leaf_with_volume("scc".to_string(), self.graph_store.node_count());
        let mut progress_tracker =
            super::progress_tracker(leaf, config.concurrency, self.task_registry.as_ref());
        let termination_flag = TerminationFlag::running_true();

        let result = storage
            .compute_scc(
                &mut computation,
                self.graph_store.as_ref(),
                &mut progress_tracker,
                &termination_flag,
            )
            .map_err(AlgorithmError::Execution)?;

        let node_count = self.graph_store.node_count();
        let mut out = SccResult::new(
            result.components,
            result.component_count,
            result.computation_time_ms,
        );
        out.node_count = node_count;
        out.execution_time = start.elapsed();
        Ok(out)
    }

    /// Stream mode: yields `(node_id, component_id)` for every node.
    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = SccRow>>> {
        let result = self.compute()?;
        let iter = result
            .components
            .into_iter()
            .enumerate()
            .map(|(node_id, component_id)| SccRow {
                node_id: node_id as u64,
                component_id,
            });
        Ok(Box::new(iter))
    }

    /// Stats mode: yields component count and execution time.
    pub fn stats(&self) -> Result<SccStats> {
        let result = self.compute()?;
        Ok(SccResultBuilder::new(result).stats())
    }

    /// Mutate mode: writes component assignments back to the graph store.
    pub fn mutate(self, property_name: &str) -> Result<SccMutateResult> {
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
        let labels_set: HashSet<NodeLabel> = new_store.node_labels();
        new_store
            .add_node_property(labels_set, property_name.to_string(), values)
            .map_err(|e| {
                AlgorithmError::Execution(format!("SCC mutate failed to add property: {e}"))
            })?;

        let summary = SccMutationSummary {
            nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: result.execution_time.as_millis() as u64,
        };

        Ok(SccMutateResult {
            summary,
            updated_store: Arc::new(new_store),
        })
    }

    /// Write mode: writes component assignments to a new graph.
    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        let res = self.mutate(property_name)?;
        Ok(WriteResult::new(
            res.summary.nodes_updated,
            property_name.to_string(),
            std::time::Duration::from_millis(res.summary.execution_time_ms),
        ))
    }

    /// Estimate memory usage.
    pub fn estimate_memory(&self) -> Result<MemoryRange> {
        let node_count = self.graph_store.node_count();
        let relationship_count = self.graph_store.relationship_count();

        let base: usize = 64 * 1024;
        let index_bytes = node_count.saturating_mul(8);
        let component_bytes = node_count.saturating_mul(8);
        let visited_bytes = node_count.saturating_add(7) / 8;
        let boundaries_bytes = node_count.saturating_mul(8);
        let stack_bytes = node_count.saturating_mul(8);
        let todo_min_bytes = node_count.saturating_mul(8);
        let todo_max_bytes = node_count.max(relationship_count).saturating_mul(8);

        let fixed = base
            .saturating_add(index_bytes)
            .saturating_add(component_bytes)
            .saturating_add(visited_bytes)
            .saturating_add(boundaries_bytes)
            .saturating_add(stack_bytes);

        Ok(MemoryRange::of_range(
            fixed.saturating_add(todo_min_bytes),
            fixed.saturating_add(todo_max_bytes),
        ))
    }

    /// Full result: returns the procedure-level SCC result.
    pub fn run(&self) -> Result<SccResult> {
        self.compute()
    }
}
