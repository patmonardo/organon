//! Harmonic Centrality Algorithm Specification
//!
//! Java parity reference: `org.neo4j.gds.harmonic.HarmonicCentrality`.
//!
//! Semantics:
//! - For each node $v$, compute $\sum_{u \ne v} 1 / d(v,u)$ where unreachable pairs contribute 0.
//! - Uses ANP MSBFS batching; accumulates into the reached node per BFS depth.
//! - Normalizes by `(nodeCount - 1)`.

use crate::collections::backends::vec::VecDouble;
use crate::concurrency::{Concurrency, TerminationFlag};
use crate::config::validation::ConfigError;
use crate::core::utils::progress::{LeafTask, ProgressTracker, TaskProgressTracker, Tasks};
use crate::core::LogLevel;
use crate::define_algorithm_spec;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::NodeLabel;
use crate::projection::Orientation;
use crate::types::properties::node::{DefaultDoubleNodePropertyValues, NodePropertyValues};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::{Duration, Instant};

use super::HarmonicComputationRuntime;
use super::HarmonicStorageRuntime;

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum HarmonicDirection {
    Incoming,
    Outgoing,
    Both,
    #[serde(other)]
    Invalid,
}

impl Default for HarmonicDirection {
    fn default() -> Self {
        Self::Both
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HarmonicConfig {
    #[serde(default)]
    pub direction: HarmonicDirection,
    #[serde(default = "default_concurrency")]
    pub concurrency: usize,
}

fn default_concurrency() -> usize {
    4
}

impl Default for HarmonicConfig {
    fn default() -> Self {
        Self {
            direction: HarmonicDirection::default(),
            concurrency: default_concurrency(),
        }
    }
}

impl HarmonicConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.concurrency == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "concurrency".to_string(),
                reason: "concurrency must be positive".to_string(),
            });
        }
        if self.direction == HarmonicDirection::Invalid {
            return Err(ConfigError::InvalidParameter {
                parameter: "direction".to_string(),
                reason: "direction must be one of outgoing, incoming, or both".to_string(),
            });
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for HarmonicConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        HarmonicConfig::validate(self)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HarmonicResult {
    pub centralities: Vec<f64>,
    pub node_count: usize,
    pub execution_time: Duration,
}

/// Statistics about harmonic centrality.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HarmonicCentralityStats {
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub stddev: f64,
    pub p50: f64,
    pub p90: f64,
    pub p99: f64,
    pub isolated_nodes: u64,
    pub execution_time_ms: u64,
}

/// Summary of a mutate operation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HarmonicCentralityMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HarmonicCentralityWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for harmonic centrality: summary + updated store
#[derive(Debug, Clone)]
pub struct HarmonicCentralityMutateResult {
    pub summary: HarmonicCentralityMutationSummary,
    pub updated_store: Arc<crate::types::prelude::DefaultGraphStore>,
}

/// Harmonic result builder (facade adapter).
pub struct HarmonicResultBuilder {
    result: HarmonicResult,
}

impl HarmonicResultBuilder {
    pub fn new(result: HarmonicResult) -> Self {
        Self { result }
    }

    pub fn stats(&self) -> HarmonicCentralityStats {
        let scores = &self.result.centralities;
        if scores.is_empty() {
            return HarmonicCentralityStats {
                min: 0.0,
                max: 0.0,
                mean: 0.0,
                stddev: 0.0,
                p50: 0.0,
                p90: 0.0,
                p99: 0.0,
                isolated_nodes: 0,
                execution_time_ms: self.result.execution_time.as_millis() as u64,
            };
        }

        let isolated_nodes = scores.iter().filter(|v| **v == 0.0).count() as u64;
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

        HarmonicCentralityStats {
            min,
            max,
            mean,
            stddev,
            p50: percentile(50.0),
            p90: percentile(90.0),
            p99: percentile(99.0),
            isolated_nodes,
            execution_time_ms: self.result.execution_time.as_millis() as u64,
        }
    }

    pub fn execution_time_ms(&self) -> u64 {
        self.result.execution_time.as_millis() as u64
    }
}

pub fn parse_harmonic_direction(direction: &str) -> Result<HarmonicDirection, AlgorithmError> {
    match direction.trim().to_lowercase().as_str() {
        "outgoing" | "natural" => Ok(HarmonicDirection::Outgoing),
        "incoming" | "reverse" => Ok(HarmonicDirection::Incoming),
        "both" | "undirected" => Ok(HarmonicDirection::Both),
        other => Err(AlgorithmError::Execution(format!(
            "Invalid Harmonic direction '{other}'. Use 'outgoing', 'incoming', or 'both'"
        ))),
    }
}

pub fn harmonic_orientation(direction: HarmonicDirection) -> Result<Orientation, AlgorithmError> {
    match direction {
        HarmonicDirection::Incoming => Ok(Orientation::Reverse),
        HarmonicDirection::Outgoing => Ok(Orientation::Natural),
        HarmonicDirection::Both => Ok(Orientation::Undirected),
        HarmonicDirection::Invalid => Err(AlgorithmError::Execution(
            "Invalid Harmonic direction. Use 'outgoing', 'incoming', or 'both'".to_string(),
        )),
    }
}

pub fn harmonic_progress_task(node_count: usize) -> LeafTask {
    Tasks::leaf_with_volume("HarmonicCentrality".to_string(), node_count)
}

define_algorithm_spec! {
    name: "harmonic",
    output_type: HarmonicResult,
    projection_hint: Dense,
    modes: [Stream, Stats, MutateNodeProperty],

    execute: |self, graph_store, config, context| {
        let parsed_config: HarmonicConfig = serde_json::from_value(config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {}", e)))?;
        parsed_config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {}", e)))?;

        let start = Instant::now();

        let concurrency = Concurrency::of(parsed_config.concurrency.max(1));
        let orientation = harmonic_orientation(parsed_config.direction)?;
        let node_count = graph_store.node_count();

        context.log(
            LogLevel::Info,
            &format!(
                "Computing harmonic centrality (direction={:?}, concurrency={}) on graph with {} nodes",
                parsed_config.direction,
                concurrency.value(),
                node_count
            ),
        );

        let storage = HarmonicStorageRuntime::with_orientation(graph_store, orientation)?;
        let computation = HarmonicComputationRuntime::new(storage.node_count());

        let mut tracker = TaskProgressTracker::with_concurrency(
            harmonic_progress_task(node_count),
            concurrency.value(),
        );
        tracker.begin_subtask_with_volume(node_count);

        let on_sources_done = {
            let tracker = tracker.clone();
            Arc::new(move |n: usize| {
                let mut progress = tracker.clone();
                progress.log_progress(n);
            })
        };

        let termination = TerminationFlag::running_true();
        let centralities = storage
            .compute_parallel(&computation, concurrency, &termination, on_sources_done)
            .map_err(|e| AlgorithmError::Execution(format!("Harmonic terminated: {}", e)))?;

        tracker.end_subtask();

        Ok(HarmonicResult {
            centralities,
            node_count: storage.node_count(),
            execution_time: start.elapsed(),
        })
    },

    mutate_node_property: |_self, graph_store, config, result| {
        let mutate_property = config
            .get("mutateProperty")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AlgorithmError::Execution("Missing mutateProperty".to_string()))?;

        let labels: HashSet<NodeLabel> = config
            .get("nodeLabels")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| NodeLabel::of(s.to_string())))
                    .collect()
            })
            .unwrap_or_else(|| graph_store.node_labels());

        let node_count = graph_store.node_count();
        if result.centralities.len() != node_count {
            return Err(AlgorithmError::Execution(format!(
                "harmonic returned {} scores for {} nodes",
                result.centralities.len(),
                node_count
            )));
        }

        let backend = VecDouble::from(result.centralities.clone());
        let values = DefaultDoubleNodePropertyValues::from_collection(backend, node_count);
        let values: Arc<dyn NodePropertyValues> = Arc::new(values);

        graph_store
            .add_node_property(labels, mutate_property.to_string(), values)
            .map_err(|e| AlgorithmError::Execution(e.to_string()))?;

        Ok(node_count)
    }
}
