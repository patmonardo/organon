//! Louvain Facade
//!
//! Louvain is a modularity-optimization community detection algorithm.
//!
//! Note: the underlying Louvain implementation in this crate is currently a
//! placeholder. The facade is wired to the procedure module and supports seeding
//! to keep API parity and determinism while the full modularity optimization
//! runtime is built out.
//!
//! Parameters:
//! - `concurrency`

use crate::algo::algorithms::Result;
use crate::algo::algorithms::{ConfigValidator, WriteResult};
use crate::algo::louvain::{
    LouvainComputationRuntime, LouvainConfig, LouvainMutateResult, LouvainMutationSummary,
    LouvainResult, LouvainResultBuilder, LouvainStats, LouvainStorageRuntime,
};
use crate::collections::backends::vec::VecLong;
use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::{TaskRegistry, Tasks};
use crate::mem::MemoryRange;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use crate::types::properties::node::DefaultLongNodePropertyValues;
use crate::types::properties::node::NodePropertyValues;
use crate::types::schema::NodeLabel;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

/// Per-node Louvain assignment row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct LouvainRow {
    pub node_id: u64,
    pub community_id: u64,
}

/// Louvain algorithm facade.
#[derive(Clone)]
pub struct LouvainFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: LouvainConfig,
    task_registry: Option<TaskRegistry>,
}

impl LouvainFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: LouvainConfig {
                concurrency: 4,
                ..Default::default()
            },
            task_registry: None,
        }
    }

    /// Create a facade using the spec.rs config model.
    pub fn from_spec_config(
        graph_store: Arc<DefaultGraphStore>,
        config: LouvainConfig,
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
        let parsed: LouvainConfig = serde_json::from_value(raw_config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        Self::from_spec_config(graph_store, parsed)
    }

    /// Apply a spec.rs config onto an existing facade.
    pub fn with_spec_config(mut self, config: LouvainConfig) -> Result<Self> {
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

    pub fn max_iterations(mut self, max_iterations: usize) -> Self {
        self.config.max_iterations = max_iterations;
        self
    }

    pub fn max_levels(mut self, max_levels: usize) -> Self {
        self.config.max_levels = max_levels;
        self
    }

    pub fn tolerance(mut self, tolerance: f64) -> Self {
        self.config.tolerance = tolerance;
        self
    }

    pub fn gamma(mut self, gamma: f64) -> Self {
        self.config.gamma = gamma;
        self
    }

    pub fn theta(mut self, theta: f64) -> Self {
        self.config.theta = theta;
        self
    }

    pub fn include_intermediate_communities(
        mut self,
        include_intermediate_communities: bool,
    ) -> Self {
        self.config.include_intermediate_communities = include_intermediate_communities;
        self
    }

    pub fn seed_property(mut self, seed_property: impl Into<String>) -> Self {
        self.config.seed_property = Some(seed_property.into());
        self
    }

    pub fn task_registry(mut self, task_registry: TaskRegistry) -> Self {
        self.task_registry = Some(task_registry);
        self
    }

    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = LouvainRow>>> {
        let result = self.compute()?;
        let iter = result
            .data
            .into_iter()
            .enumerate()
            .map(|(node_id, community_id)| LouvainRow {
                node_id: node_id as u64,
                community_id,
            });
        Ok(Box::new(iter))
    }

    pub fn stats(&self) -> Result<LouvainStats> {
        let result = self.compute()?;
        Ok(LouvainResultBuilder::new(result).stats())
    }

    pub fn mutate(self, property_name: &str) -> Result<LouvainMutateResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        let result = self.compute()?;

        let node_count = self.graph_store.node_count();
        let nodes_updated = node_count as u64;

        // Convert community ids (u64) into i64 backend for VecLong
        let longs: Vec<i64> = result.data.into_iter().map(|c| c as i64).collect();
        let backend = VecLong::from(longs);
        let values = DefaultLongNodePropertyValues::from_collection(backend, node_count);
        let values: Arc<dyn NodePropertyValues> = Arc::new(values);

        let mut new_store = self.graph_store.as_ref().clone();
        let labels: HashSet<NodeLabel> = new_store.node_labels();
        new_store
            .add_node_property(labels, property_name.to_string(), values)
            .map_err(|e| {
                AlgorithmError::Execution(format!("Louvain mutate failed to add property: {e}"))
            })?;

        let summary = LouvainMutationSummary {
            nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: result.execution_time.as_millis() as u64,
        };

        Ok(LouvainMutateResult {
            summary,
            updated_store: Arc::new(new_store),
        })
    }

    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        // For Louvain, write is the same as mutate since it's node properties
        let res = self.mutate(property_name)?;
        Ok(WriteResult::new(
            res.summary.nodes_updated,
            property_name.to_string(),
            std::time::Duration::from_millis(res.summary.execution_time_ms),
        ))
    }

    pub fn estimate_memory(&self) -> MemoryRange {
        // Estimate memory for Louvain computation
        // - Community assignments: node_count * 8 bytes
        // - Modularity calculations: node_count * 8 bytes
        // - Graph view overhead: roughly node_count * 16 bytes
        let node_count = self.graph_store.node_count();
        let assignment_memory = node_count * 8;
        let modularity_memory = node_count * 8;
        let graph_memory = node_count * 16;
        let dendrogram_memory = if self.config.include_intermediate_communities {
            node_count
                .saturating_mul(self.config.max_levels)
                .saturating_mul(8)
        } else {
            node_count.saturating_mul(16)
        };

        let total = assignment_memory + modularity_memory + graph_memory + dendrogram_memory;
        MemoryRange::of_range(total, total * 2) // Conservative upper bound
    }

    fn validate(&self) -> Result<()> {
        self.config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))
    }

    fn compute(&self) -> Result<LouvainResult> {
        self.validate()?;
        let start = Instant::now();

        let storage =
            LouvainStorageRuntime::new(self.graph_store.as_ref(), self.config.concurrency)?;
        let mut computation = LouvainComputationRuntime::new();
        let termination_flag = TerminationFlag::default();

        let mut progress_tracker = super::progress_tracker(
            Tasks::leaf_with_volume("louvain".to_string(), storage.node_count()),
            self.config.concurrency,
            self.task_registry.as_ref(),
        );

        let result = storage.compute_louvain(
            &mut computation,
            &self.config,
            &mut progress_tracker,
            &termination_flag,
        )?;
        Ok(LouvainResult {
            execution_time: start.elapsed(),
            ..result
        })
    }

    /// Full result: returns the procedure-level Louvain result.
    pub fn run(&self) -> Result<LouvainResult> {
        let result = self.compute()?;
        Ok(result)
    }
}
