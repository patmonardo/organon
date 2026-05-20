use super::LeidenComputationRuntime;
use super::LeidenStorageRuntime;
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
pub struct LeidenConfig {
    /// Concurrency hint used by storage/progress plumbing.
    #[serde(default = "default_concurrency")]
    pub concurrency: usize,

    /// Resolution parameter. Higher values lead to more (smaller) communities.
    #[serde(default = "default_gamma")]
    pub gamma: f64,

    /// Randomness parameter used in the refinement phase.
    ///
    /// Kept for API parity with the Java implementation.
    #[serde(default = "default_theta")]
    pub theta: f64,

    /// Convergence tolerance on modularity improvement.
    #[serde(default = "default_tolerance")]
    pub tolerance: f64,

    /// Maximum number of Leiden levels.
    #[serde(default = "default_max_iterations", alias = "maxIterations")]
    pub max_iterations: usize,

    /// Whether to retain per-level community assignments.
    #[serde(
        default = "default_include_intermediate_communities",
        alias = "includeIntermediateCommunities"
    )]
    pub include_intermediate_communities: bool,

    /// RNG seed for reproducibility.
    #[serde(default = "default_random_seed", alias = "randomSeed")]
    pub random_seed: u64,

    /// Optional starting communities for each node.
    ///
    /// If present, must be empty (treated as None) or match `node_count`.
    #[serde(default, alias = "seedCommunities")]
    pub seed_communities: Option<Vec<u64>>,
}

fn default_concurrency() -> usize {
    4
}

fn default_gamma() -> f64 {
    1.0
}

fn default_theta() -> f64 {
    0.01
}

fn default_tolerance() -> f64 {
    0.0001
}

fn default_max_iterations() -> usize {
    10
}

fn default_random_seed() -> u64 {
    42
}

fn default_include_intermediate_communities() -> bool {
    false
}

impl Default for LeidenConfig {
    fn default() -> Self {
        Self {
            concurrency: default_concurrency(),
            gamma: default_gamma(),
            theta: default_theta(),
            tolerance: default_tolerance(),
            max_iterations: default_max_iterations(),
            include_intermediate_communities: default_include_intermediate_communities(),
            random_seed: default_random_seed(),
            seed_communities: None,
        }
    }
}

impl LeidenConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.concurrency == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "concurrency".to_string(),
                reason: "concurrency must be positive".to_string(),
            });
        }
        if !(0.0..=100.0).contains(&self.gamma) {
            return Err(ConfigError::InvalidParameter {
                parameter: "gamma".to_string(),
                reason: "gamma must be between 0 and 100".to_string(),
            });
        }
        if !(0.0..=1.0).contains(&self.theta) {
            return Err(ConfigError::InvalidParameter {
                parameter: "theta".to_string(),
                reason: "theta must be between 0 and 1".to_string(),
            });
        }
        if !(0.0..=1.0).contains(&self.tolerance) {
            return Err(ConfigError::InvalidParameter {
                parameter: "tolerance".to_string(),
                reason: "tolerance must be between 0 and 1".to_string(),
            });
        }
        if self.max_iterations == 0 || self.max_iterations > 1000 {
            return Err(ConfigError::InvalidParameter {
                parameter: "maxIterations".to_string(),
                reason: "max_iterations must be between 1 and 1000".to_string(),
            });
        }
        Ok(())
    }

    pub fn validate_for_node_count(&self, node_count: usize) -> Result<(), ConfigError> {
        self.validate()?;
        if let Some(seeds) = &self.seed_communities {
            if !seeds.is_empty() && seeds.len() != node_count {
                return Err(ConfigError::InvalidParameter {
                    parameter: "seedCommunities".to_string(),
                    reason: format!(
                        "seed_communities length ({}) must be empty or equal node count ({node_count})",
                        seeds.len()
                    ),
                });
            }
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for LeidenConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        LeidenConfig::validate(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeidenResult {
    /// Community assignment per original node.
    pub communities: Vec<u64>,

    /// Number of distinct communities.
    pub community_count: u64,

    /// Final modularity value.
    pub modularity: f64,

    /// Number of Leiden levels executed.
    pub levels: usize,

    /// Java-shaped alias for `levels`.
    pub ran_levels: usize,

    /// Whether convergence was reached within `tolerance`.
    pub converged: bool,

    /// Java-shaped alias for `converged`.
    pub did_converge: bool,

    /// Modularity values accepted during the level loop.
    pub modularities: Vec<f64>,

    /// Optional per-level community assignments over original nodes.
    pub intermediate_communities: Option<Vec<Vec<u64>>>,

    /// Number of nodes processed.
    pub node_count: usize,

    /// Execution time for the computation.
    pub execution_time: Duration,
}

impl LeidenResult {
    pub fn community(&self, node_id: usize) -> Option<u64> {
        self.communities.get(node_id).copied()
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

/// Aggregated Leiden stats.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LeidenStats {
    pub node_count: usize,
    pub community_count: u64,
    pub modularity: f64,
    pub levels: usize,
    pub converged: bool,
    pub execution_time_ms: u64,
}

/// Summary of a mutate operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeidenMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeidenWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for Leiden: summary + updated store.
#[derive(Debug, Clone)]
pub struct LeidenMutateResult {
    pub summary: LeidenMutationSummary,
    pub updated_store: Arc<crate::types::prelude::DefaultGraphStore>,
}

/// Leiden result builder (facade adapter).
pub struct LeidenResultBuilder {
    result: LeidenResult,
}

impl LeidenResultBuilder {
    pub fn new(result: LeidenResult) -> Self {
        Self { result }
    }

    pub fn stats(&self) -> LeidenStats {
        LeidenStats {
            node_count: self.result.node_count,
            community_count: self.result.community_count,
            modularity: self.result.modularity,
            levels: self.result.levels,
            converged: self.result.converged,
            execution_time_ms: self.result.execution_time.as_millis() as u64,
        }
    }

    pub fn execution_time_ms(&self) -> u64 {
        self.result.execution_time.as_millis() as u64
    }
}

define_algorithm_spec! {
    name: "leiden",
    output_type: LeidenResult,
    projection_hint: Dense,
    modes: [Stream, Stats, MutateNodeProperty, WriteNodeProperty],

    execute: |_self, graph_store, config, _context| {
        let parsed: LeidenConfig = serde_json::from_value(config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        parsed
            .validate_for_node_count(graph_store.node_count())
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;

        let start = std::time::Instant::now();
        let storage = LeidenStorageRuntime::new(graph_store)?;
        let node_count = storage.node_count();
        let mut progress = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume(
                "leiden".to_string(),
                node_count.saturating_add(parsed.max_iterations),
            ),
            parsed.concurrency,
        );
        let termination_flag = TerminationFlag::default();
        let mut computation = LeidenComputationRuntime::new();

        let result = storage.compute_leiden(
            &mut computation,
            &parsed,
            &mut progress,
            &termination_flag,
        )?;

        Ok(LeidenResult {
            execution_time: start.elapsed(),
            ..result
        })
    }
}
