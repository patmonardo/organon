use super::ConductanceComputationRuntime;
use super::ConductanceStorageRuntime;
use crate::task::concurrency::Concurrency;
use crate::task::concurrency::TerminationFlag;
use crate::config::validation::ConfigError;
use crate::task::progress::EmptyTaskRegistryFactory;
use crate::task::progress::JobId;
use crate::task::progress::Task;
use crate::task::progress::TaskProgressTracker;
use crate::task::progress::Tasks;
use crate::define_algorithm_spec;
use crate::projection::eval::algorithm::AlgorithmError;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

/// Configuration for conductance.
///
/// Java parity reference: `ConductanceParameters` / `ConductanceConfigTransformer`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConductanceConfig {
    /// Concurrency used for relationship counting.
    #[serde(default = "default_concurrency")]
    pub concurrency: usize,

    /// Minimum batch size for degree partitioning.
    #[serde(default = "default_min_batch_size", alias = "minBatchSize")]
    pub min_batch_size: usize,

    /// When `true`, relationship weights are taken from the projected graph.
    /// When `false`, every relationship contributes weight `1.0`.
    #[serde(default, alias = "hasRelationshipWeightProperty")]
    pub has_relationship_weight_property: bool,

    /// Node property key storing community IDs (non-negative long values).
    #[serde(default, alias = "communityProperty")]
    pub community_property: String,
}

fn default_concurrency() -> usize {
    4
}

fn default_min_batch_size() -> usize {
    10_000
}

impl Default for ConductanceConfig {
    fn default() -> Self {
        Self {
            concurrency: default_concurrency(),
            min_batch_size: default_min_batch_size(),
            has_relationship_weight_property: false,
            community_property: String::new(),
        }
    }
}

impl ConductanceConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.community_property.is_empty() {
            return Err(ConfigError::InvalidParameter {
                parameter: "communityProperty".to_string(),
                reason: "community_property cannot be empty".to_string(),
            });
        }
        if self.concurrency == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "concurrency".to_string(),
                reason: "concurrency must be positive".to_string(),
            });
        }
        if self.min_batch_size == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "minBatchSize".to_string(),
                reason: "min_batch_size must be positive".to_string(),
            });
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for ConductanceConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        ConductanceConfig::validate(self)
    }
}

/// Java-shaped progress task for conductance.
pub fn conductance_progress_task(node_count: usize) -> Task {
    Tasks::task(
        "Conductance".to_string(),
        vec![
            Arc::new(
                Tasks::leaf_with_volume("count relationships".to_string(), node_count)
                    .base()
                    .clone(),
            ),
            Arc::new(Tasks::leaf("accumulate counts".to_string()).base().clone()),
            Arc::new(
                Tasks::leaf("perform conductance computations".to_string())
                    .base()
                    .clone(),
            ),
        ],
    )
}

/// Result of conductance computation.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConductanceResult {
    /// Per-community conductance values.
    pub community_conductances: HashMap<u64, f64>,

    /// Global average conductance over communities with counts.
    pub global_average_conductance: f64,

    /// Number of communities evaluated.
    pub community_count: usize,

    /// Number of nodes processed.
    pub node_count: usize,

    /// Execution time for the computation.
    pub execution_time: Duration,
}

/// Statistics for conductance computation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConductanceStats {
    pub node_count: usize,
    pub community_count: usize,
    pub average_conductance: f64,
    pub execution_time_ms: u64,
}

/// Summary of a mutate operation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConductanceMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConductanceWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for conductance: summary + updated store.
#[derive(Debug, Clone)]
pub struct ConductanceMutateResult {
    pub summary: ConductanceMutationSummary,
    pub updated_store: Arc<crate::types::prelude::DefaultGraphStore>,
}

/// Conductance result builder (facade adapter).
pub struct ConductanceResultBuilder {
    result: ConductanceResult,
}

impl ConductanceResultBuilder {
    pub fn new(result: ConductanceResult) -> Self {
        Self { result }
    }

    pub fn stats(&self) -> ConductanceStats {
        ConductanceStats {
            node_count: self.result.node_count,
            community_count: self.result.community_count,
            average_conductance: self.result.global_average_conductance,
            execution_time_ms: self.result.execution_time.as_millis() as u64,
        }
    }

    pub fn execution_time_ms(&self) -> u64 {
        self.result.execution_time.as_millis() as u64
    }
}

define_algorithm_spec! {
    name: "conductance",
    output_type: ConductanceResult,
    projection_hint: Dense,
    modes: [Stream, Stats, MutateNodeProperty, WriteNodeProperty],

    execute: |_self, graph_store, config, _context| {
        let parsed: ConductanceConfig = serde_json::from_value(config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        parsed
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;

        let start = std::time::Instant::now();
        let progress_task = conductance_progress_task(graph_store.node_count());
        let registry_factory = EmptyTaskRegistryFactory;
        let mut progress = TaskProgressTracker::with_registry(
            progress_task,
            Concurrency::of(parsed.concurrency.max(1)),
            JobId::new(),
            &registry_factory,
        );
        let termination_flag = TerminationFlag::default();

        let storage = ConductanceStorageRuntime::new();
        let mut runtime = ConductanceComputationRuntime::new();
        let mut result = storage
            .compute_conductance(
                &mut runtime,
                graph_store,
                &parsed,
                &mut progress,
                &termination_flag,
            )
            .map_err(AlgorithmError::Execution)?;

        result.node_count = graph_store.node_count();
        result.execution_time = start.elapsed();
        Ok(result)
    }
}
