use super::LouvainComputationRuntime;
use super::LouvainStorageRuntime;
use crate::concurrency::TerminationFlag;
use crate::config::validation::ConfigError;
use crate::core::utils::progress::TaskProgressTracker;
use crate::core::utils::progress::Tasks;
use crate::define_algorithm_spec;
use crate::projection::eval::algorithm::AlgorithmError;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LouvainConfig {
    #[serde(default = "default_concurrency")]
    pub concurrency: usize,

    #[serde(default = "default_max_iterations", alias = "maxIterations")]
    pub max_iterations: usize,

    #[serde(default = "default_max_levels", alias = "maxLevels")]
    pub max_levels: usize,

    #[serde(default = "default_tolerance")]
    pub tolerance: f64,

    #[serde(
        default = "default_include_intermediate_communities",
        alias = "includeIntermediateCommunities"
    )]
    pub include_intermediate_communities: bool,

    #[serde(default)]
    pub seed_property: Option<String>,

    #[serde(default = "default_gamma")]
    pub gamma: f64,

    #[serde(default = "default_theta")]
    pub theta: f64,
}

fn default_max_iterations() -> usize {
    10
}

fn default_max_levels() -> usize {
    10
}

fn default_tolerance() -> f64 {
    0.0001
}

fn default_include_intermediate_communities() -> bool {
    false
}

fn default_gamma() -> f64 {
    1.0
}

fn default_theta() -> f64 {
    0.01
}

fn default_concurrency() -> usize {
    4
}

impl Default for LouvainConfig {
    fn default() -> Self {
        Self {
            concurrency: default_concurrency(),
            max_iterations: default_max_iterations(),
            max_levels: default_max_levels(),
            tolerance: default_tolerance(),
            include_intermediate_communities: default_include_intermediate_communities(),
            seed_property: None,
            gamma: default_gamma(),
            theta: default_theta(),
        }
    }
}

impl LouvainConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        crate::config::validate_positive(self.concurrency as f64, "concurrency")?;
        crate::config::validate_positive(self.max_iterations as f64, "maxIterations")?;
        crate::config::validate_positive(self.max_levels as f64, "maxLevels")?;
        crate::config::validate_positive(self.tolerance, "tolerance")?;
        crate::config::validate_range(self.gamma, 0.0, 10.0, "gamma")?;
        crate::config::validate_range(self.theta, 0.0, 1.0, "theta")?;
        Ok(())
    }
}

impl crate::config::ValidatedConfig for LouvainConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        LouvainConfig::validate(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LouvainResult {
    pub data: Vec<u64>,
    pub ran_levels: usize,
    pub modularities: Vec<f64>,
    pub modularity: f64,
    pub intermediate_communities: Option<Vec<Vec<u64>>>,
    pub node_count: usize,
    pub execution_time: Duration,
}

impl LouvainResult {
    pub fn community(&self, node_id: usize) -> Option<u64> {
        self.data.get(node_id).copied()
    }

    pub fn intermediate_communities(&self, node_id: usize) -> Vec<u64> {
        match &self.intermediate_communities {
            Some(levels) => levels
                .iter()
                .filter_map(|level| level.get(node_id).copied())
                .collect(),
            None => self.community(node_id).into_iter().collect(),
        }
    }
}

/// Aggregated Louvain stats.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LouvainStats {
    pub node_count: usize,
    pub community_count: usize,
    pub ran_levels: usize,
    pub modularity: f64,
    pub execution_time_ms: u64,
}

/// Summary of a mutate operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LouvainMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LouvainWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for Louvain: summary + updated store.
#[derive(Debug, Clone)]
pub struct LouvainMutateResult {
    pub summary: LouvainMutationSummary,
    pub updated_store: Arc<crate::types::prelude::DefaultGraphStore>,
}

/// Louvain result builder (facade adapter).
pub struct LouvainResultBuilder {
    result: LouvainResult,
}

impl LouvainResultBuilder {
    pub fn new(result: LouvainResult) -> Self {
        Self { result }
    }

    pub fn stats(&self) -> LouvainStats {
        let community_count = self
            .result
            .data
            .iter()
            .copied()
            .collect::<std::collections::HashSet<u64>>()
            .len();

        LouvainStats {
            node_count: self.result.node_count,
            community_count,
            ran_levels: self.result.ran_levels,
            modularity: self.result.modularity,
            execution_time_ms: self.result.execution_time.as_millis() as u64,
        }
    }

    pub fn execution_time_ms(&self) -> u64 {
        self.result.execution_time.as_millis() as u64
    }
}

pub struct LouvainAlgorithmSpec {
    graph_name: String,
}

impl LouvainAlgorithmSpec {
    pub fn new(graph_name: String) -> Self {
        Self { graph_name }
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }
}

define_algorithm_spec! {
    name: "louvain",
    output_type: LouvainResult,
    projection_hint: Dense,
    modes: [Stream, Stats, MutateNodeProperty, WriteNodeProperty],

    execute: |_self, graph_store, config, _context| {
        let parsed: LouvainConfig = serde_json::from_value(config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        parsed
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;

        let start = std::time::Instant::now();
        let storage = LouvainStorageRuntime::new(graph_store, parsed.concurrency)?;
        let mut computation = LouvainComputationRuntime::new();
        let termination_flag = TerminationFlag::default();
        let mut progress = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("louvain".to_string(), storage.node_count()),
            parsed.concurrency,
        );

        let result = storage.compute_louvain(
            &mut computation,
            &parsed,
            &mut progress,
            &termination_flag,
        )?;

        Ok(LouvainResult {
            execution_time: start.elapsed(),
            ..result
        })
    }
}
