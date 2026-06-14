//! K1-Coloring Facade
//!
//! Greedy iterative graph coloring.
//!
//! Parameters (Java GDS aligned):
//! - `concurrency`: controls the Rayon worker pool used by coloring/validation.
//! - `max_iterations`: maximum number of coloring/validation iterations (must be >= 1).
//! - `batch_size`: accepted for parity; degree-aware partitioning is deferred.

use crate::algo::algorithms::Result;
use crate::algo::algorithms::{ConfigValidator, WriteResult};
use crate::algo::k1coloring::{
    k1coloring_progress_task, K1ColoringComputationRuntime, K1ColoringConfig,
    K1ColoringMutateResult, K1ColoringMutationSummary, K1ColoringResult, K1ColoringResultBuilder,
    K1ColoringStats, K1ColoringStorageRuntime, INITIAL_FORBIDDEN_COLORS,
};
use crate::collections::backends::vec::VecLong;
use crate::task::concurrency::TerminationFlag;
use crate::task::progress::TaskRegistry;
use crate::task::memory::{Estimate, MemoryRange};
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use crate::types::properties::node::DefaultLongNodePropertyValues;
use crate::types::properties::node::NodePropertyValues;
use crate::types::schema::NodeLabel;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

/// Per-node color assignment row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct K1ColoringRow {
    pub node_id: u64,
    pub color_id: u64,
}

/// K1-Coloring algorithm facade.
#[derive(Clone)]
pub struct K1ColoringFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: K1ColoringConfig,
    task_registry: Option<TaskRegistry>,
}

impl K1ColoringFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: K1ColoringConfig::default(),
            task_registry: None,
        }
    }

    /// Create a facade using the spec.rs config model.
    pub fn from_spec_config(
        graph_store: Arc<DefaultGraphStore>,
        config: K1ColoringConfig,
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
        let parsed: K1ColoringConfig = serde_json::from_value(raw_config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        Self::from_spec_config(graph_store, parsed)
    }

    /// Apply a spec.rs config onto an existing facade.
    pub fn with_spec_config(mut self, config: K1ColoringConfig) -> Result<Self> {
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

    pub fn max_iterations(mut self, max_iterations: u64) -> Self {
        self.config.max_iterations = max_iterations;
        self
    }

    pub fn batch_size(mut self, batch_size: usize) -> Self {
        self.config.min_batch_size = batch_size;
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

    fn compute(&self) -> Result<K1ColoringResult> {
        self.validate()?;
        let start = Instant::now();

        let config = self.config.clone();

        let storage = K1ColoringStorageRuntime::new(self.graph_store.as_ref())?;
        let node_count = storage.node_count();
        if node_count == 0 {
            return Ok(K1ColoringResult {
                colors: Vec::new(),
                ran_iterations: 0,
                did_converge: true,
                node_count: 0,
                execution_time: start.elapsed(),
            });
        }

        let base_task = k1coloring_progress_task(node_count, self.config.max_iterations);
        let mut progress_tracker = super::progress_tracker_for_task(
            base_task,
            self.config.concurrency,
            self.task_registry.as_ref(),
        );

        let termination_flag = TerminationFlag::running_true();

        let mut runtime = K1ColoringComputationRuntime::new(node_count, self.config.max_iterations)
            .concurrency(self.config.concurrency);

        let mut result = storage.compute_k1coloring(
            &mut runtime,
            &config,
            &mut progress_tracker,
            &termination_flag,
        )?;
        result.execution_time = start.elapsed();
        Ok(result)
    }

    /// Stream mode: yields `(node_id, color_id)` for every node.
    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = K1ColoringRow>>> {
        let result = self.compute()?;
        let iter = result
            .colors
            .into_iter()
            .enumerate()
            .map(|(node_id, color_id)| K1ColoringRow {
                node_id: node_id as u64,
                color_id,
            });
        Ok(Box::new(iter))
    }

    /// Stats mode: yields convergence info + number of distinct colors used.
    pub fn stats(&self) -> Result<K1ColoringStats> {
        let result = self.compute()?;
        Ok(K1ColoringResultBuilder::new(result).stats())
    }

    /// Mutate mode: writes color assignments back to the graph store.
    pub fn mutate(self, property_name: &str) -> Result<K1ColoringMutateResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        let result = self.compute()?;

        let node_count = self.graph_store.node_count();
        let nodes_updated = node_count as u64;

        let longs: Vec<i64> = result.colors.into_iter().map(|c| c as i64).collect();
        let backend = VecLong::from(longs);
        let values = DefaultLongNodePropertyValues::from_collection(backend, node_count);
        let values: Arc<dyn NodePropertyValues> = Arc::new(values);

        let mut new_store = self.graph_store.as_ref().clone();
        let labels_set: HashSet<NodeLabel> = new_store.node_labels();
        new_store
            .add_node_property(labels_set, property_name.to_string(), values)
            .map_err(|e| {
                AlgorithmError::Execution(format!("K1Coloring mutate failed to add property: {e}"))
            })?;

        let summary = K1ColoringMutationSummary {
            nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: result.execution_time.as_millis() as u64,
        };

        Ok(K1ColoringMutateResult {
            summary,
            updated_store: Arc::new(new_store),
        })
    }

    /// Write mode: writes color assignments to a new graph.
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
        let node_count = GraphStore::node_count(self.graph_store.as_ref());
        let concurrency = self.config.concurrency.max(1);

        let colors = Estimate::size_of_long_array(node_count);
        let nodes_to_color = Estimate::size_of_bitset(node_count).saturating_mul(2);
        let forbidden_colors =
            Estimate::size_of_bitset(INITIAL_FORBIDDEN_COLORS).saturating_mul(concurrency);

        let total = Estimate::BYTES_OBJECT_HEADER
            .saturating_add(colors)
            .saturating_add(nodes_to_color)
            .saturating_add(forbidden_colors);

        Ok(MemoryRange::of(total))
    }

    /// Full result: returns the procedure-level K1Coloring result.
    pub fn run(&self) -> Result<K1ColoringResult> {
        let result = self.compute()?;
        Ok(result)
    }
}
