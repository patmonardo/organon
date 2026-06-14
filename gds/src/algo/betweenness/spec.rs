//! Betweenness Centrality specification

use crate::algo::betweenness::BetweennessCentralityComputationRuntime;
use crate::task::concurrency::{Concurrency, TerminationFlag};
use crate::config::config_trait::ValidatedConfig;
use crate::config::validation::ConfigError;
use crate::task::progress::LeafTask;
use crate::task::progress::ProgressTracker;
use crate::task::progress::TaskProgressTracker;
use crate::task::progress::Tasks;
use crate::define_algorithm_spec;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::Orientation;
use std::sync::Arc;
use std::time::{Duration, Instant};

use super::BetweennessCentralityStorageRuntime;

/// Configuration for betweenness centrality.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BetweennessCentralityConfig {
    /// Traversal direction: "outgoing", "incoming", or "both".
    #[serde(default = "default_direction")]
    pub direction: String,

    /// Requested parallelism.
    #[serde(default = "default_concurrency")]
    pub concurrency: usize,

    /// Optional relationship weight property.
    #[serde(default, alias = "relationshipWeightProperty")]
    pub relationship_weight_property: Option<String>,

    /// Sampling strategy: "all" or "random_degree".
    #[serde(default = "default_sampling_strategy", alias = "samplingStrategy")]
    pub sampling_strategy: String,

    /// Optional number of sources to process (<= node_count).
    #[serde(default, alias = "samplingSize")]
    pub sampling_size: Option<usize>,

    /// RNG seed for sampling.
    #[serde(default = "default_random_seed", alias = "randomSeed")]
    pub random_seed: u64,
}

fn default_direction() -> String {
    "both".to_string()
}

fn default_concurrency() -> usize {
    4
}

fn default_sampling_strategy() -> String {
    "all".to_string()
}

fn default_random_seed() -> u64 {
    42
}

impl Default for BetweennessCentralityConfig {
    fn default() -> Self {
        Self {
            direction: default_direction(),
            concurrency: default_concurrency(),
            relationship_weight_property: None,
            sampling_strategy: default_sampling_strategy(),
            sampling_size: None,
            random_seed: default_random_seed(),
        }
    }
}

impl crate::config::ValidatedConfig for BetweennessCentralityConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        crate::config::validate_positive(self.concurrency as f64, "concurrency")?;
        if let Some(size) = self.sampling_size {
            crate::config::validate_positive(size as f64, "samplingSize")?;
        }
        if let Some(property) = &self.relationship_weight_property {
            if property.trim().is_empty() {
                return Err(ConfigError::InvalidParameter {
                    parameter: "relationshipWeightProperty".to_string(),
                    reason: "relationshipWeightProperty must be non-empty".to_string(),
                });
            }
        }
        if parse_betweenness_orientation(&self.direction).is_err() {
            return Err(ConfigError::InvalidParameter {
                parameter: "direction".to_string(),
                reason: "direction must be one of outgoing, incoming, or both".to_string(),
            });
        }
        if parse_betweenness_sampling_strategy(&self.sampling_strategy).is_err() {
            return Err(ConfigError::InvalidParameter {
                parameter: "samplingStrategy".to_string(),
                reason: "samplingStrategy must be one of all or random_degree".to_string(),
            });
        }
        Ok(())
    }
}

/// Executor-facing result type.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BetweennessCentralityResult {
    pub centralities: Vec<f64>,
    pub node_count: usize,
    pub execution_time: Duration,
}

/// Statistics about betweenness centrality in the graph.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BetweennessCentralityStats {
    /// Number of nodes scored
    pub node_count: usize,
    /// Minimum betweenness score
    pub min: f64,
    /// Maximum betweenness score
    pub max: f64,
    /// Average betweenness
    pub mean: f64,
    /// Standard deviation
    pub stddev: f64,
    /// Median (50th percentile)
    pub p50: f64,
    /// 90th percentile
    pub p90: f64,
    /// 99th percentile
    pub p99: f64,
    /// Number of "bridge" nodes (high betweenness > mean + stddev)
    pub bridge_nodes: u64,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

/// Summary of a mutate operation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BetweennessCentralityMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BetweennessCentralityWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for betweenness centrality: summary + updated store
#[derive(Debug, Clone)]
pub struct BetweennessCentralityMutateResult {
    pub summary: BetweennessCentralityMutationSummary,
    pub updated_store: Arc<crate::types::prelude::DefaultGraphStore>,
}

/// Betweenness result builder (facade adapter).
pub struct BetweennessCentralityResultBuilder {
    result: BetweennessCentralityResult,
}

impl BetweennessCentralityResultBuilder {
    pub fn new(result: BetweennessCentralityResult) -> Self {
        Self { result }
    }

    pub fn stats(&self) -> BetweennessCentralityStats {
        let scores = &self.result.centralities;
        if scores.is_empty() {
            return BetweennessCentralityStats {
                node_count: self.result.node_count,
                min: 0.0,
                max: 0.0,
                mean: 0.0,
                stddev: 0.0,
                p50: 0.0,
                p90: 0.0,
                p99: 0.0,
                bridge_nodes: 0,
                execution_time_ms: self.result.execution_time.as_millis() as u64,
            };
        }

        let mut sorted = scores.clone();
        sorted.sort_by(|a, b| a.total_cmp(b));
        let min = *sorted.first().unwrap_or(&0.0);
        let max = *sorted.last().unwrap_or(&0.0);
        let mean = scores.iter().sum::<f64>() / scores.len() as f64;
        let var = scores
            .iter()
            .map(|x| {
                let d = x - mean;
                d * d
            })
            .sum::<f64>()
            / scores.len() as f64;
        let stddev = var.sqrt();

        let percentile = |p: f64| -> f64 {
            let idx =
                ((p.clamp(0.0, 100.0) / 100.0) * (sorted.len() as f64 - 1.0)).round() as usize;
            sorted[idx]
        };

        let threshold = mean + stddev;
        let bridge_nodes = scores.iter().filter(|v| **v > threshold).count() as u64;

        BetweennessCentralityStats {
            node_count: self.result.node_count,
            min,
            max,
            mean,
            stddev,
            p50: percentile(50.0),
            p90: percentile(90.0),
            p99: percentile(99.0),
            bridge_nodes,
            execution_time_ms: self.result.execution_time.as_millis() as u64,
        }
    }

    pub fn execution_time_ms(&self) -> u64 {
        self.result.execution_time.as_millis() as u64
    }
}

pub fn parse_betweenness_orientation(direction: &str) -> Result<Orientation, AlgorithmError> {
    match direction.trim().to_lowercase().as_str() {
        "outgoing" | "natural" => Ok(Orientation::Natural),
        "incoming" | "reverse" => Ok(Orientation::Reverse),
        "both" | "undirected" => Ok(Orientation::Undirected),
        other => Err(AlgorithmError::Execution(format!(
            "Invalid Betweenness direction '{other}'. Use 'outgoing', 'incoming', or 'both'"
        ))),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BetweennessSamplingStrategy {
    All,
    RandomDegree,
}

pub fn parse_betweenness_sampling_strategy(
    strategy: &str,
) -> Result<BetweennessSamplingStrategy, AlgorithmError> {
    match strategy
        .trim()
        .to_lowercase()
        .replace('_', "")
        .replace('-', "")
        .as_str()
    {
        "all" => Ok(BetweennessSamplingStrategy::All),
        "randomdegree" => Ok(BetweennessSamplingStrategy::RandomDegree),
        other => Err(AlgorithmError::Execution(format!(
            "Invalid Betweenness sampling strategy '{other}'. Use 'all' or 'random_degree'"
        ))),
    }
}

pub fn betweenness_progress_task(source_count: usize) -> LeafTask {
    Tasks::leaf_with_volume("BetweennessCentrality".to_string(), source_count)
}

define_algorithm_spec! {
    name: "betweenness",
    output_type: BetweennessCentralityResult,
    projection_hint: Dense,
    modes: [Stream, Stats],

    execute: |_self, graph_store, config, _context| {
        let parsed: BetweennessCentralityConfig = serde_json::from_value(config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {}", e)))?;
        parsed
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;

        let start = Instant::now();

        let orientation = parse_betweenness_orientation(&parsed.direction)?;
        let storage = BetweennessCentralityStorageRuntime::new(
            graph_store,
            orientation,
            parsed.relationship_weight_property.as_deref(),
        )?;

        let node_count = storage.node_count();
        let sources = storage.select_sources(
            &parsed.sampling_strategy,
            parsed.sampling_size,
            parsed.random_seed,
        )?;
        let divisor = if orientation == Orientation::Undirected { 2.0 } else { 1.0 };
        let concurrency = Concurrency::of(parsed.concurrency.max(1));

        let mut computation = BetweennessCentralityComputationRuntime::new(node_count);

        let mut tracker = TaskProgressTracker::with_concurrency(
            betweenness_progress_task(sources.len()),
            concurrency.value(),
        );
        tracker.begin_subtask_with_volume(sources.len());

        let termination = TerminationFlag::running_true();
        let on_done = {
            let tracker = tracker.clone();
            Arc::new(move || {
                let mut progress = tracker.clone();
                progress.log_progress(1);
            })
        };

        let result = storage
            .compute_betweenness(
                &mut computation,
                &sources,
                divisor,
                concurrency,
                &termination,
                on_done,
            )
            .map_err(|e| AlgorithmError::Execution(format!("terminated: {e}")))?;

        tracker.end_subtask();

        Ok(BetweennessCentralityResult {
            centralities: result.centralities,
            node_count,
            execution_time: start.elapsed(),
        })
    }
}

// Re-exported and aliased from `betweenness/mod.rs`.
