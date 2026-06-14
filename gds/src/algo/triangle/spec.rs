use super::TriangleComputationRuntime;
use super::TriangleStorageRuntime;
use crate::task::concurrency::TerminationFlag;
use crate::config::validation::ConfigError;
use crate::core::utils::progress::TaskProgressTracker;
use crate::core::utils::progress::Tasks;
use crate::define_algorithm_spec;
use crate::projection::eval::algorithm::AlgorithmError;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

pub const EXCLUDED_NODE_TRIANGLE_COUNT: i64 = -1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriangleConfig {
    /// Concurrency hint accepted for Java GDS API alignment.
    #[serde(default = "default_concurrency")]
    pub concurrency: usize,
    /// Skip nodes with degree > max_degree (performance / approximation).
    #[serde(default = "default_max_degree", alias = "maxDegree")]
    pub max_degree: u64,
}

fn default_concurrency() -> usize {
    4
}

fn default_max_degree() -> u64 {
    u64::MAX
}

impl TriangleConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.concurrency == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "concurrency".to_string(),
                reason: "concurrency must be > 0".to_string(),
            });
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for TriangleConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        TriangleConfig::validate(self)
    }
}

impl Default for TriangleConfig {
    fn default() -> Self {
        Self {
            concurrency: default_concurrency(),
            max_degree: default_max_degree(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriangleResult {
    pub local_triangles: Vec<i64>,
    pub global_triangles: u64,
    pub node_count: usize,
    pub execution_time: Duration,
}

/// Aggregated triangle count statistics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriangleStats {
    pub node_count: usize,
    pub global_triangles: u64,
    pub execution_time_ms: u64,
}

/// Summary of a mutate operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriangleMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriangleWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for Triangle: summary + updated store.
#[derive(Debug, Clone)]
pub struct TriangleMutateResult {
    pub summary: TriangleMutationSummary,
    pub updated_store: Arc<crate::types::prelude::DefaultGraphStore>,
}

/// Triangle result builder (facade adapter).
pub struct TriangleResultBuilder {
    result: TriangleResult,
}

impl TriangleResultBuilder {
    pub fn new(result: TriangleResult) -> Self {
        Self { result }
    }

    pub fn stats(&self) -> TriangleStats {
        TriangleStats {
            node_count: self.result.node_count,
            global_triangles: self.result.global_triangles,
            execution_time_ms: self.result.execution_time.as_millis() as u64,
        }
    }

    pub fn execution_time_ms(&self) -> u64 {
        self.result.execution_time.as_millis() as u64
    }
}

pub struct TriangleAlgorithmSpec {
    graph_name: String,
}

impl TriangleAlgorithmSpec {
    pub fn new(graph_name: String) -> Self {
        Self { graph_name }
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }
}

define_algorithm_spec! {
    name: "triangle",
    output_type: TriangleResult,
    projection_hint: Dense,
    modes: [Stream, Stats, MutateNodeProperty, WriteNodeProperty],

    execute: |_self, graph_store, config_input, _context| {
        let parsed_config: TriangleConfig = serde_json::from_value(config_input.clone())
            .map_err(|e| AlgorithmError::InvalidGraph(format!("Failed to parse config: {}", e)))?;

        parsed_config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;

        let start = std::time::Instant::now();
        let storage = TriangleStorageRuntime::new();
        let mut computation = TriangleComputationRuntime::new();
        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("triangle".to_string(), graph_store.node_count()),
            parsed_config.concurrency,
        );
        let termination_flag = TerminationFlag::default();

        let result = storage
            .compute_triangle(
                &mut computation,
                graph_store,
                &parsed_config,
                &mut progress_tracker,
                &termination_flag,
            )
            .map_err(AlgorithmError::Execution)?;

        Ok(TriangleResult {
            execution_time: start.elapsed(),
            ..result
        })
    }
}
