//! HITS algorithm specification

use crate::algo::hits::HitsComputationRuntime;
use crate::algo::hits::HitsStorageRuntime;
use crate::collections::backends::vec::VecDouble;
use crate::task::concurrency::Concurrency;
use crate::config::validation::ConfigError;
use crate::task::progress::{LeafTask, ProgressTracker, TaskProgressTracker, Tasks};
use crate::define_algorithm_spec;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::NodeLabel;
use crate::types::properties::node::{DefaultDoubleNodePropertyValues, NodePropertyValues};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HitsConfig {
    #[serde(default = "default_max_iterations", alias = "maxIterations")]
    pub max_iterations: usize,

    #[serde(default = "default_tolerance")]
    pub tolerance: f64,

    #[serde(default = "default_concurrency")]
    pub concurrency: usize,

    #[serde(default = "default_hub_property", alias = "hubProperty")]
    pub hub_property: String,

    #[serde(default = "default_auth_property", alias = "authProperty")]
    pub auth_property: String,
}

fn default_max_iterations() -> usize {
    20
}

fn default_tolerance() -> f64 {
    1e-4
}

fn default_concurrency() -> usize {
    4
}

fn default_hub_property() -> String {
    "hub".to_string()
}

fn default_auth_property() -> String {
    "authority".to_string()
}

impl Default for HitsConfig {
    fn default() -> Self {
        Self {
            max_iterations: default_max_iterations(),
            tolerance: default_tolerance(),
            concurrency: default_concurrency(),
            hub_property: default_hub_property(),
            auth_property: default_auth_property(),
        }
    }
}

impl HitsConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.concurrency == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "concurrency".to_string(),
                reason: "concurrency must be positive".to_string(),
            });
        }
        if self.max_iterations == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "maxIterations".to_string(),
                reason: "maxIterations must be positive".to_string(),
            });
        }
        if self.tolerance <= 0.0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "tolerance".to_string(),
                reason: "tolerance must be positive".to_string(),
            });
        }
        if self.hub_property.trim().is_empty() {
            return Err(ConfigError::InvalidParameter {
                parameter: "hubProperty".to_string(),
                reason: "hubProperty must be non-empty".to_string(),
            });
        }
        if self.auth_property.trim().is_empty() {
            return Err(ConfigError::InvalidParameter {
                parameter: "authProperty".to_string(),
                reason: "authProperty must be non-empty".to_string(),
            });
        }
        if self.hub_property == self.auth_property {
            return Err(ConfigError::InvalidParameter {
                parameter: "authProperty".to_string(),
                reason: "authProperty must differ from hubProperty".to_string(),
            });
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for HitsConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        HitsConfig::validate(self)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HitsResult {
    pub hub_scores: Vec<f64>,
    pub authority_scores: Vec<f64>,
    pub hub_property: String,
    pub auth_property: String,
    pub node_count: usize,
    pub iterations: usize,
    pub converged: bool,
    pub execution_time: Duration,
}

/// Statistics for HITS algorithm
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HitsCentralityStats {
    pub node_count: usize,
    pub hub_property: String,
    pub auth_property: String,
    pub iterations: usize,
    pub converged: bool,
    pub execution_time_ms: u64,
}

/// Summary of a mutate operation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HitsCentralityMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HitsCentralityWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for HITS centrality: summary + updated store
#[derive(Debug, Clone)]
pub struct HitsCentralityMutateResult {
    pub summary: HitsCentralityMutationSummary,
    pub updated_store: Arc<crate::types::prelude::DefaultGraphStore>,
}

/// HITS result builder (facade adapter).
pub struct HitsResultBuilder {
    result: HitsResult,
}

impl HitsResultBuilder {
    pub fn new(result: HitsResult) -> Self {
        Self { result }
    }

    pub fn stats(&self) -> HitsCentralityStats {
        HitsCentralityStats {
            node_count: self.result.node_count,
            hub_property: self.result.hub_property.clone(),
            auth_property: self.result.auth_property.clone(),
            iterations: self.result.iterations,
            converged: self.result.converged,
            execution_time_ms: self.result.execution_time.as_millis() as u64,
        }
    }

    pub fn execution_time_ms(&self) -> u64 {
        self.result.execution_time.as_millis() as u64
    }
}

pub fn hits_progress_task(_node_count: usize, max_iterations: usize) -> LeafTask {
    Tasks::leaf_with_volume("Hits".to_string(), max_iterations)
}

define_algorithm_spec! {
    name: "hits",
    output_type: HitsResult,
    projection_hint: Dense,
    modes: [Stream, Stats, MutateNodeProperty],

    execute: |_self, graph_store, config, _context| {
        let parsed: HitsConfig = serde_json::from_value(config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        parsed
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;

        let start = Instant::now();

        let storage = HitsStorageRuntime::with_default_projection(graph_store)?;
        let computation = HitsComputationRuntime::with_properties(
            parsed.tolerance,
            parsed.hub_property.clone(),
            parsed.auth_property.clone(),
        );
        let concurrency = Concurrency::of(parsed.concurrency.max(1));

        let mut tracker = TaskProgressTracker::with_concurrency(
            hits_progress_task(graph_store.node_count(), parsed.max_iterations),
            concurrency.value(),
        );

        tracker.begin_subtask_with_volume(parsed.max_iterations);

        let run = storage.run(&computation, parsed.max_iterations, concurrency, &mut tracker);

        tracker.end_subtask();

        Ok(HitsResult {
            hub_scores: run.hub_scores,
            authority_scores: run.authority_scores,
            hub_property: parsed.hub_property,
            auth_property: parsed.auth_property,
            node_count: graph_store.node_count(),
            iterations: run.iterations_ran,
            converged: run.did_converge,
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
        if result.hub_scores.len() != node_count {
            return Err(AlgorithmError::Execution(format!(
                "hits returned {} hub scores for {} nodes",
                result.hub_scores.len(),
                node_count
            )));
        }

        let backend = VecDouble::from(result.hub_scores.clone());
        let values = DefaultDoubleNodePropertyValues::from_collection(backend, node_count);
        let values: Arc<dyn NodePropertyValues> = Arc::new(values);

        graph_store
            .add_node_property(labels, mutate_property.to_string(), values)
            .map_err(|e| AlgorithmError::Execution(e.to_string()))?;

        Ok(node_count)
    }
}
