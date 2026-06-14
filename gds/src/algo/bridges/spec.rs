//! Bridges Algorithm Specification
//!
//! Java parity reference: `org.neo4j.gds.bridges.Bridges`.

use crate::task::concurrency::TerminationFlag;
use crate::config::validation::ConfigError;
use crate::task::progress::{LeafTask, ProgressTracker, TaskProgressTracker, Tasks};
use crate::core::LogLevel;
use crate::define_algorithm_spec;
use crate::projection::eval::algorithm::AlgorithmError;
use std::sync::Arc;
use std::time::{Duration, Instant};

use super::BridgesStorageRuntime;
use super::{Bridge, BridgesComputationRuntime};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BridgesConfig {
    #[serde(default = "default_concurrency")]
    pub concurrency: usize,
}

fn default_concurrency() -> usize {
    4
}

impl Default for BridgesConfig {
    fn default() -> Self {
        Self {
            concurrency: default_concurrency(),
        }
    }
}

impl BridgesConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.concurrency == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "concurrency".to_string(),
                reason: "concurrency must be positive".to_string(),
            });
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for BridgesConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        BridgesConfig::validate(self)
    }
}

pub fn bridges_progress_task(node_count: usize) -> LeafTask {
    Tasks::leaf_with_volume("Bridges".to_string(), node_count)
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BridgesResult {
    pub bridges: Vec<Bridge>,
    pub node_count: usize,
    pub execution_time: Duration,
}

/// Result row for bridges stream mode
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct BridgesRow {
    pub from: u64,
    pub to: u64,
}

/// Statistics for bridges computation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BridgesStats {
    pub node_count: usize,
    pub bridge_count: usize,
    pub execution_time_ms: u64,
}

/// Summary of a mutate operation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BridgesMutationSummary {
    pub edges_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BridgesWriteSummary {
    pub edges_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for bridges: summary + updated store
#[derive(Debug, Clone)]
pub struct BridgesMutateResult {
    pub summary: BridgesMutationSummary,
    pub updated_store: Arc<crate::types::prelude::DefaultGraphStore>,
}

fn result_rows(result: &BridgesResult) -> Vec<BridgesRow> {
    result
        .bridges
        .iter()
        .map(|bridge| BridgesRow {
            from: bridge.from,
            to: bridge.to,
        })
        .collect()
}

/// Bridges result builder (facade adapter).
pub struct BridgesResultBuilder {
    result: BridgesResult,
}

impl BridgesResultBuilder {
    pub fn new(result: BridgesResult) -> Self {
        Self { result }
    }

    pub fn rows(&self) -> Vec<BridgesRow> {
        result_rows(&self.result)
    }

    pub fn stats(&self) -> BridgesStats {
        BridgesStats {
            node_count: self.result.node_count,
            bridge_count: self.result.bridges.len(),
            execution_time_ms: self.result.execution_time.as_millis() as u64,
        }
    }

    pub fn execution_time_ms(&self) -> u64 {
        self.result.execution_time.as_millis() as u64
    }
}

define_algorithm_spec! {
    name: "bridges",
    output_type: BridgesResult,
    projection_hint: Dense,
    modes: [Stream, Stats],

    execute: |self, graph_store, config, context| {
        let parsed_config: BridgesConfig = serde_json::from_value(config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {}", e)))?;
        parsed_config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {}", e)))?;

        let start = Instant::now();

        let storage = BridgesStorageRuntime::new(graph_store)?;
        let node_count = storage.node_count();

        context.log(
            LogLevel::Info,
            &format!(
                "Computing bridges (concurrency={}) on graph with {} nodes",
                parsed_config.concurrency,
                node_count
            ),
        );

        let mut tracker = TaskProgressTracker::with_concurrency(
            bridges_progress_task(node_count),
            parsed_config.concurrency,
        );
        tracker.begin_subtask_with_volume(node_count);

        let on_progress = {
            let tracker = tracker.clone();
            Arc::new(move || {
                let mut progress = tracker.clone();
                progress.log_progress(1);
            })
        };

        let termination = TerminationFlag::running_true();

        let mut computation = BridgesComputationRuntime::new_with_stack_capacity(
            storage.node_count(),
            storage.relationship_count(),
        );

        let bridges = storage
            .compute_bridges(&mut computation, &termination, on_progress)
            .map_err(|e| AlgorithmError::Execution(format!("Bridges terminated: {e}")))?
            .bridges;

        tracker.end_subtask();

        Ok(BridgesResult {
            bridges,
            node_count,
            execution_time: start.elapsed(),
        })
    }
}
