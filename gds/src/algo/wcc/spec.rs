//! WCC Algorithm Specification (executor integration)

use super::WccComputationRuntime;
use super::WccStorageRuntime;
use crate::task::concurrency::TerminationFlag;
use crate::config::validation::ConfigError;
use crate::core::utils::progress::TaskProgressTracker;
use crate::core::utils::progress::Tasks;
use crate::define_algorithm_spec;
use crate::projection::eval::algorithm::AlgorithmError;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WccConfig {
    #[serde(default = "default_concurrency")]
    pub concurrency: usize,

    /// Minimum batch size for range partitioning in the unsampled strategy.
    #[serde(default = "default_min_batch_size", alias = "minBatchSize")]
    pub min_batch_size: usize,

    /// Optional threshold over relationship property values.
    /// Only relationships with `property > threshold` are considered.
    #[serde(default, alias = "threshold")]
    pub threshold: Option<f64>,

    /// Optional node property containing initial component ids.
    #[serde(default, alias = "seedProperty")]
    pub seed_property: Option<String>,
}

fn default_concurrency() -> usize {
    4
}

fn default_min_batch_size() -> usize {
    crate::core::utils::partition::DEFAULT_BATCH_SIZE
}

impl Default for WccConfig {
    fn default() -> Self {
        Self {
            concurrency: default_concurrency(),
            min_batch_size: default_min_batch_size(),
            threshold: None,
            seed_property: None,
        }
    }
}

impl WccConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.concurrency == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "concurrency".to_string(),
                reason: "concurrency must be > 0".to_string(),
            });
        }
        if self.min_batch_size == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "min_batch_size".to_string(),
                reason: "min_batch_size must be > 0".to_string(),
            });
        }
        if self
            .seed_property
            .as_ref()
            .is_some_and(|property| property.trim().is_empty())
        {
            return Err(ConfigError::InvalidParameter {
                parameter: "seed_property".to_string(),
                reason: "seed_property must be non-empty".to_string(),
            });
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for WccConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        WccConfig::validate(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WccResult {
    pub components: Vec<u64>,
    pub component_count: usize,
    pub node_count: usize,
    pub execution_time: Duration,
}

/// Aggregated WCC stats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WccStats {
    pub node_count: usize,
    pub component_count: usize,
    pub execution_time_ms: u64,
}

/// Summary of a mutate operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WccMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WccWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for WCC: summary + updated graph store
#[derive(Debug, Clone)]
pub struct WccMutateResult {
    pub summary: WccMutationSummary,
    pub updated_store: Arc<crate::types::prelude::DefaultGraphStore>,
}

/// WCC result builder (facade adapter).
pub struct WccResultBuilder {
    result: WccResult,
}

impl WccResultBuilder {
    pub fn new(result: WccResult) -> Self {
        Self { result }
    }

    pub fn stats(&self) -> WccStats {
        WccStats {
            node_count: self.result.node_count,
            component_count: self.result.component_count,
            execution_time_ms: self.result.execution_time.as_millis() as u64,
        }
    }

    pub fn execution_time_ms(&self) -> u64 {
        self.result.execution_time.as_millis() as u64
    }
}

define_algorithm_spec! {
    name: "wcc",
    output_type: WccResult,
    projection_hint: Dense,
    modes: [Stream, Stats, MutateNodeProperty, WriteNodeProperty],

    execute: |_self, graph_store, config_input, _context| {
        let parsed_config: WccConfig = serde_json::from_value(config_input.clone())
            .map_err(|e| AlgorithmError::InvalidGraph(format!("Failed to parse config: {}", e)))?;

        parsed_config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;

        let storage = WccStorageRuntime::new(parsed_config.concurrency);
        let mut computation = WccComputationRuntime::new()
            .concurrency(parsed_config.concurrency)
            .min_batch_size(parsed_config.min_batch_size)
            .threshold(parsed_config.threshold);

        if let Some(seed_property) = &parsed_config.seed_property {
            let seed_values = graph_store
                .node_property_values(seed_property)
                .map_err(|e| AlgorithmError::Execution(format!(
                    "WCC seed property `{}` not found in graph store: {}",
                    seed_property, e
                )))?;
            computation = computation.seed_property_values(seed_values);
        }

        let start = std::time::Instant::now();

        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("wcc".to_string(), graph_store.relationship_count()),
            parsed_config.concurrency,
        );
        let termination_flag = TerminationFlag::default();

        let result = storage
            .compute_wcc(
                &mut computation,
                graph_store,
                &mut progress_tracker,
                &termination_flag,
            )
            .map_err(AlgorithmError::Execution)?;

        Ok(WccResult {
            components: result.components,
            component_count: result.component_count,
            node_count: graph_store.node_count(),
            execution_time: start.elapsed(),
        })
    }
}
