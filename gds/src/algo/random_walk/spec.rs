//! RandomWalk Specification
//!
//! **Translation Source**: `org.neo4j.gds.traversal.RandomWalkBaseConfig`

use crate::algo::algorithms::pathfinding::PathResult;
use crate::config::validation::ConfigError;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomWalkConfig {
    /// Number of walks to perform per node
    #[serde(
        default = "RandomWalkConfig::default_walks_per_node",
        alias = "walksPerNode"
    )]
    pub walks_per_node: usize,
    /// Length of each walk (number of steps)
    #[serde(
        default = "RandomWalkConfig::default_walk_length",
        alias = "walkLength"
    )]
    pub walk_length: usize,
    /// Return factor for node2vec (probability to return to previous node)
    #[serde(
        default = "RandomWalkConfig::default_return_factor",
        alias = "returnFactor"
    )]
    pub return_factor: f64,
    /// In-out factor for node2vec (probability to explore vs exploit)
    #[serde(
        default = "RandomWalkConfig::default_in_out_factor",
        alias = "inOutFactor"
    )]
    pub in_out_factor: f64,
    /// Optional list of source nodes (if empty, walks from all nodes)
    #[serde(default, alias = "sourceNodes")]
    pub source_nodes: Vec<u64>,
    /// Random seed for reproducibility
    #[serde(default, alias = "randomSeed")]
    pub random_seed: Option<u64>,
    /// Concurrency level
    #[serde(default = "RandomWalkConfig::default_concurrency")]
    pub concurrency: usize,
}

impl Default for RandomWalkConfig {
    fn default() -> Self {
        Self {
            walks_per_node: Self::default_walks_per_node(),
            walk_length: Self::default_walk_length(),
            return_factor: Self::default_return_factor(),
            in_out_factor: Self::default_in_out_factor(),
            source_nodes: Vec::new(),
            random_seed: None,
            concurrency: Self::default_concurrency(),
        }
    }
}

impl RandomWalkConfig {
    fn default_walks_per_node() -> usize {
        10
    }

    fn default_walk_length() -> usize {
        80
    }

    fn default_return_factor() -> f64 {
        1.0
    }

    fn default_in_out_factor() -> f64 {
        1.0
    }

    fn default_concurrency() -> usize {
        4
    }

    /// Validate configuration parameters
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.concurrency == 0 {
            return Err(ConfigError::MustBePositive {
                name: "concurrency".to_string(),
                value: self.concurrency as f64,
            });
        }

        if self.walks_per_node == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "walks_per_node".to_string(),
                reason: "walks_per_node must be > 0".to_string(),
            });
        }

        if self.walk_length == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "walk_length".to_string(),
                reason: "walk_length must be > 0".to_string(),
            });
        }

        if !(0.0..=100.0).contains(&self.return_factor) {
            return Err(ConfigError::InvalidParameter {
                parameter: "return_factor".to_string(),
                reason: "return_factor must be in [0, 100]".to_string(),
            });
        }

        if !(0.0..=100.0).contains(&self.in_out_factor) {
            return Err(ConfigError::InvalidParameter {
                parameter: "in_out_factor".to_string(),
                reason: "in_out_factor must be in [0, 100]".to_string(),
            });
        }

        Ok(())
    }
}

impl crate::config::ValidatedConfig for RandomWalkConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        RandomWalkConfig::validate(self)
    }
}

/// Result of random walk computation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomWalkResult {
    /// Collection of walks (each walk is a sequence of node IDs)
    pub walks: Vec<Vec<u64>>,
}

/// Result row for random walk stream mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomWalkRow {
    /// The walk as a sequence of node IDs
    pub path: Vec<u64>,
}

/// Statistics for random walk computation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomWalkStats {
    pub walk_count: usize,
    pub execution_time_ms: u64,
}

/// Summary of a mutate operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomWalkMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomWalkWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for random walk: summary + updated store
#[derive(Debug, Clone)]
pub struct RandomWalkMutateResult {
    pub summary: RandomWalkMutationSummary,
    pub updated_store: Arc<crate::types::prelude::DefaultGraphStore>,
}

fn walks_to_paths(walks: &[Vec<u64>]) -> Vec<PathResult> {
    let mut paths: Vec<PathResult> = Vec::new();
    for walk in walks {
        if walk.len() < 2 {
            continue;
        }
        let source = walk[0];
        let target = *walk.last().unwrap_or(&source);
        let cost = (walk.len().saturating_sub(1)) as f64;
        paths.push(PathResult {
            source,
            target,
            path: walk.clone(),
            cost,
        });
    }
    paths
}

/// Random walk result builder (facade adapter).
pub struct RandomWalkResultBuilder {
    result: RandomWalkResult,
    execution_time: Duration,
}

impl RandomWalkResultBuilder {
    pub fn new(result: RandomWalkResult, execution_time: Duration) -> Self {
        Self {
            result,
            execution_time,
        }
    }

    pub fn rows(&self) -> Vec<RandomWalkRow> {
        self.result
            .walks
            .iter()
            .cloned()
            .map(|path| RandomWalkRow { path })
            .collect()
    }

    pub fn stats(&self) -> RandomWalkStats {
        RandomWalkStats {
            walk_count: self.result.walks.len(),
            execution_time_ms: self.execution_time.as_millis() as u64,
        }
    }

    pub fn paths(&self) -> Vec<PathResult> {
        walks_to_paths(&self.result.walks)
    }

    pub fn execution_time_ms(&self) -> u64 {
        self.execution_time.as_millis() as u64
    }
}

pub struct RandomWalkAlgorithmSpec {
    graph_name: String,
}

impl RandomWalkAlgorithmSpec {
    pub fn new(graph_name: String) -> Self {
        Self { graph_name }
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }
}
