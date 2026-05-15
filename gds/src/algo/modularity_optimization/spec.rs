use crate::config::validation::ConfigError;
use crate::core::utils::progress::{Task, Tasks};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModularityOptimizationConfig {
    #[serde(default = "default_concurrency")]
    pub concurrency: usize,

    #[serde(default = "default_min_batch_size", rename = "minBatchSize")]
    pub min_batch_size: usize,

    #[serde(default = "default_max_iterations", rename = "maxIterations")]
    pub max_iterations: usize,

    #[serde(default = "default_tolerance")]
    pub tolerance: f64,

    /// Modularity resolution parameter. $\gamma = 1$ is classic modularity.
    #[serde(default = "default_gamma")]
    pub gamma: f64,

    #[serde(default, rename = "relationshipWeightProperty")]
    pub relationship_weight_property: Option<String>,
}

fn default_concurrency() -> usize {
    4
}

fn default_min_batch_size() -> usize {
    10_000
}

fn default_max_iterations() -> usize {
    20
}

fn default_tolerance() -> f64 {
    1e-6
}

fn default_gamma() -> f64 {
    1.0
}

impl Default for ModularityOptimizationConfig {
    fn default() -> Self {
        Self {
            concurrency: default_concurrency(),
            min_batch_size: default_min_batch_size(),
            max_iterations: default_max_iterations(),
            tolerance: default_tolerance(),
            gamma: default_gamma(),
            relationship_weight_property: None,
        }
    }
}

impl ModularityOptimizationConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
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
        if self.max_iterations == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "maxIterations".to_string(),
                reason: "max_iterations must be positive".to_string(),
            });
        }
        if self.tolerance < 0.0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "tolerance".to_string(),
                reason: "tolerance must be non-negative".to_string(),
            });
        }
        if self.gamma <= 0.0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "gamma".to_string(),
                reason: "gamma must be positive".to_string(),
            });
        }
        if matches!(&self.relationship_weight_property, Some(property) if property.trim().is_empty())
        {
            return Err(ConfigError::InvalidParameter {
                parameter: "relationshipWeightProperty".to_string(),
                reason: "relationship_weight_property cannot be empty".to_string(),
            });
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for ModularityOptimizationConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        ModularityOptimizationConfig::validate(self)
    }
}

pub fn modularity_optimization_progress_task(
    node_count: usize,
    relationship_count: usize,
    max_iterations: usize,
) -> Task {
    let supplier = Arc::new(move || {
        vec![Arc::new(
            Tasks::leaf_with_volume("optimizeForColor".to_string(), relationship_count)
                .base()
                .clone(),
        )]
    });

    Tasks::task(
        "ModularityOptimization".to_string(),
        vec![
            Arc::new(
                Tasks::leaf_with_volume("initialization".to_string(), node_count)
                    .base()
                    .clone(),
            ),
            Arc::new(
                Tasks::iterative_dynamic(
                    "compute modularity".to_string(),
                    supplier,
                    max_iterations,
                )
                .base()
                .clone(),
            ),
        ],
    )
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModularityOptimizationResult {
    pub communities: Vec<u64>,
    pub modularity: f64,
    pub ran_iterations: usize,
    pub did_converge: bool,
    pub node_count: usize,
    pub execution_time: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModularityOptimizationStats {
    pub modularity: f64,
    pub community_count: usize,
    pub ran_iterations: usize,
    pub did_converge: bool,
    pub execution_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModularityOptimizationMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

#[derive(Debug, Clone)]
pub struct ModularityOptimizationMutateResult {
    pub summary: ModularityOptimizationMutationSummary,
    pub updated_store: Arc<crate::types::prelude::DefaultGraphStore>,
}

pub struct ModularityOptimizationResultBuilder {
    result: ModularityOptimizationResult,
}

impl ModularityOptimizationResultBuilder {
    pub fn new(result: ModularityOptimizationResult) -> Self {
        Self { result }
    }

    pub fn stats(&self) -> ModularityOptimizationStats {
        let community_count = self
            .result
            .communities
            .iter()
            .copied()
            .collect::<std::collections::HashSet<u64>>()
            .len();

        ModularityOptimizationStats {
            modularity: self.result.modularity,
            community_count,
            ran_iterations: self.result.ran_iterations,
            did_converge: self.result.did_converge,
            execution_time_ms: self.result.execution_time.as_millis() as u64,
        }
    }
}
