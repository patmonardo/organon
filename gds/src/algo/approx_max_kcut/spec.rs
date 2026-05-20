//! ApproxMaxKCut algorithm specification.

use super::ApproxMaxKCutComputationRuntime;
use super::ApproxMaxKCutStorageRuntime;
use crate::concurrency::Concurrency;
use crate::concurrency::TerminationFlag;
use crate::config::validation::ConfigError;
use crate::core::utils::progress::EmptyTaskRegistryFactory;
use crate::core::utils::progress::JobId;
use crate::core::utils::progress::Task;
use crate::core::utils::progress::TaskProgressTracker;
use crate::core::utils::progress::Tasks;
use crate::define_algorithm_spec;
use crate::projection::eval::algorithm::AlgorithmError;
use std::sync::Arc;
use std::time::Duration;

/// Configuration for approx max k-cut computation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ApproxMaxKCutConfig {
    #[serde(default = "default_k")]
    pub k: u8,
    #[serde(default = "default_iterations")]
    pub iterations: usize,
    #[serde(default = "default_random_seed")]
    pub random_seed: u64,
    #[serde(default = "default_minimize")]
    pub minimize: bool,
    #[serde(default = "default_has_relationship_weight_property")]
    pub has_relationship_weight_property: bool,
    #[serde(default = "default_min_community_sizes")]
    pub min_community_sizes: Vec<usize>,
    #[serde(default = "default_concurrency")]
    pub concurrency: usize,
    #[serde(default = "default_min_batch_size", alias = "minBatchSize")]
    pub min_batch_size: usize,
    #[serde(
        default = "default_vns_max_neighborhood_order",
        alias = "vnsMaxNeighborhoodOrder"
    )]
    pub vns_max_neighborhood_order: usize,
}

fn default_k() -> u8 {
    2
}

fn default_iterations() -> usize {
    8
}

fn default_random_seed() -> u64 {
    0
}

fn default_minimize() -> bool {
    false
}

fn default_has_relationship_weight_property() -> bool {
    false
}

fn default_min_community_sizes() -> Vec<usize> {
    vec![0, 0]
}

fn default_concurrency() -> usize {
    4
}

fn default_min_batch_size() -> usize {
    crate::core::utils::partition::DEFAULT_BATCH_SIZE
}

fn default_vns_max_neighborhood_order() -> usize {
    0
}

impl Default for ApproxMaxKCutConfig {
    fn default() -> Self {
        Self {
            k: default_k(),
            iterations: default_iterations(),
            random_seed: default_random_seed(),
            minimize: default_minimize(),
            has_relationship_weight_property: default_has_relationship_weight_property(),
            min_community_sizes: default_min_community_sizes(),
            concurrency: default_concurrency(),
            min_batch_size: default_min_batch_size(),
            vns_max_neighborhood_order: default_vns_max_neighborhood_order(),
        }
    }
}

impl ApproxMaxKCutConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.k < 2 || self.k > 127 {
            return Err(ConfigError::InvalidParameter {
                parameter: "k".to_string(),
                reason: "k must be between 2 and 127".to_string(),
            });
        }
        if self.iterations == 0 || self.iterations > 1000 {
            return Err(ConfigError::InvalidParameter {
                parameter: "iterations".to_string(),
                reason: "iterations must be between 1 and 1000".to_string(),
            });
        }
        if self.concurrency == 0 || self.concurrency > 1024 {
            return Err(ConfigError::InvalidParameter {
                parameter: "concurrency".to_string(),
                reason: "concurrency must be between 1 and 1024".to_string(),
            });
        }
        if self.min_batch_size == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "minBatchSize".to_string(),
                reason: "min_batch_size must be positive".to_string(),
            });
        }
        if self.min_community_sizes.len() != self.k as usize {
            return Err(ConfigError::InvalidParameter {
                parameter: "minCommunitySizes".to_string(),
                reason: format!(
                    "min_community_sizes length ({}) must equal k ({})",
                    self.min_community_sizes.len(),
                    self.k
                ),
            });
        }
        Ok(())
    }

    pub fn validate_for_node_count(&self, node_count: usize) -> Result<(), ConfigError> {
        self.validate()?;
        if node_count < self.k as usize {
            return Err(ConfigError::InvalidParameter {
                parameter: "k".to_string(),
                reason: format!("k ({}) must not exceed node count ({node_count})", self.k),
            });
        }
        let total_min: usize = self.min_community_sizes.iter().sum();
        if total_min > node_count {
            return Err(ConfigError::InvalidParameter {
                parameter: "minCommunitySizes".to_string(),
                reason: format!(
                    "sum of min_community_sizes ({total_min}) must not exceed node count ({node_count})"
                ),
            });
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for ApproxMaxKCutConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        ApproxMaxKCutConfig::validate(self)
    }
}

pub fn approx_max_kcut_progress_task(
    node_count: usize,
    iterations: usize,
    vns_max_neighborhood_order: usize,
) -> Task {
    let iteration_supplier = Arc::new(move || {
        let search_description = if vns_max_neighborhood_order > 0 {
            "variable neighborhood search"
        } else {
            "local search"
        };

        vec![
            Arc::new(
                Tasks::leaf_with_volume("place nodes randomly".to_string(), node_count)
                    .base()
                    .clone(),
            ),
            Arc::new(
                Tasks::leaf_with_volume(search_description.to_string(), node_count)
                    .base()
                    .clone(),
            ),
        ]
    });

    Tasks::iterative_fixed("ApproxMaxKCut".to_string(), iteration_supplier, iterations)
        .base()
        .clone()
}

/// Result for approx max k-cut computation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ApproxMaxKCutResult {
    pub communities: Vec<u8>,
    pub cut_cost: f64,
    pub k: u8,
    pub node_count: usize,
    pub execution_time: Duration,
}

/// Statistics for approx max k-cut computation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ApproxMaxKCutStats {
    pub cut_cost: f64,
    pub k: u8,
    pub node_count: usize,
    pub execution_time_ms: u64,
}

/// Summary of a mutate operation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ApproxMaxKCutMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ApproxMaxKCutWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for ApproxMaxKCut: summary + updated store
#[derive(Debug, Clone)]
pub struct ApproxMaxKCutMutateResult {
    pub summary: ApproxMaxKCutMutationSummary,
    pub updated_store: Arc<crate::types::prelude::DefaultGraphStore>,
}

/// ApproxMaxKCut result builder (facade adapter).
pub struct ApproxMaxKCutResultBuilder {
    result: ApproxMaxKCutResult,
}

impl ApproxMaxKCutResultBuilder {
    pub fn new(result: ApproxMaxKCutResult) -> Self {
        Self { result }
    }

    pub fn stats(&self) -> ApproxMaxKCutStats {
        ApproxMaxKCutStats {
            cut_cost: self.result.cut_cost,
            k: self.result.k,
            node_count: self.result.node_count,
            execution_time_ms: self.result.execution_time.as_millis() as u64,
        }
    }

    pub fn execution_time_ms(&self) -> u64 {
        self.result.execution_time.as_millis() as u64
    }
}

define_algorithm_spec! {
    name: "approx_max_kcut",
    output_type: ApproxMaxKCutResult,
    projection_hint: Dense,
    modes: [Stream, Stats, MutateNodeProperty, WriteNodeProperty],

    execute: |_self, graph_store, config, _context| {
        let parsed: ApproxMaxKCutConfig = serde_json::from_value(config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        parsed
            .validate_for_node_count(graph_store.node_count())
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;

        let start = std::time::Instant::now();
        let progress_task = approx_max_kcut_progress_task(
            graph_store.node_count(),
            parsed.iterations,
            parsed.vns_max_neighborhood_order,
        );
        let registry_factory = EmptyTaskRegistryFactory;
        let mut progress = TaskProgressTracker::with_registry(
            progress_task,
            Concurrency::of(parsed.concurrency.max(1)),
            JobId::new(),
            &registry_factory,
        );
        let termination_flag = TerminationFlag::default();

        let storage = ApproxMaxKCutStorageRuntime::new();
        let mut runtime = ApproxMaxKCutComputationRuntime::new(parsed.clone());
        let mut result = storage
            .compute_approx_max_kcut(
                &mut runtime,
                graph_store,
                &parsed,
                &mut progress,
                &termination_flag,
            )
            .map_err(AlgorithmError::Execution)?;

        result.node_count = graph_store.node_count();
        result.k = parsed.k;
        result.execution_time = start.elapsed();
        Ok(result)
    }
}
