//! PageRank algorithm specification (executor integration)

use crate::collections::backends::vec::VecDouble;
use crate::concurrency::Concurrency;
use crate::config::validation::ConfigError;
use crate::core::utils::progress::{LeafTask, ProgressTracker, TaskProgressTracker, Tasks};
use crate::define_algorithm_spec;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::NodeLabel;
use crate::projection::Orientation;
use crate::types::properties::node::{DefaultDoubleNodePropertyValues, NodePropertyValues};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::{Duration, Instant};

use super::PageRankComputationRuntime;
use super::PageRankStorageRuntime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PageRankVariant {
    PageRank,
    ArticleRank,
    Eigenvector,
}

impl Default for PageRankVariant {
    fn default() -> Self {
        Self::PageRank
    }
}

impl PageRankVariant {
    pub fn task_name(self) -> &'static str {
        match self {
            Self::PageRank => "PageRank",
            Self::ArticleRank => "ArticleRank",
            Self::Eigenvector => "EigenVector",
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PageRankConfig {
    #[serde(default = "default_direction")]
    pub direction: String,

    #[serde(default, alias = "pageRankVariant")]
    pub variant: PageRankVariant,

    #[serde(default = "default_concurrency")]
    pub concurrency: usize,

    #[serde(default = "default_max_iterations", alias = "maxIterations")]
    pub max_iterations: usize,

    #[serde(default = "default_tolerance")]
    pub tolerance: f64,

    #[serde(default = "default_damping_factor", alias = "dampingFactor")]
    pub damping_factor: f64,

    #[serde(default, alias = "sourceNodes")]
    pub source_nodes: Option<Vec<u64>>,

    #[serde(default, alias = "relationshipWeightProperty")]
    pub relationship_weight_property: Option<String>,
}

fn default_direction() -> String {
    "outgoing".to_string()
}

fn default_concurrency() -> usize {
    4
}

fn default_max_iterations() -> usize {
    20
}

fn default_tolerance() -> f64 {
    1e-4
}

fn default_damping_factor() -> f64 {
    0.85
}

impl Default for PageRankConfig {
    fn default() -> Self {
        Self {
            direction: default_direction(),
            variant: PageRankVariant::default(),
            concurrency: default_concurrency(),
            max_iterations: default_max_iterations(),
            tolerance: default_tolerance(),
            damping_factor: default_damping_factor(),
            source_nodes: None,
            relationship_weight_property: None,
        }
    }
}

impl PageRankConfig {
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
        if self.damping_factor <= 0.0 || self.damping_factor >= 1.0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "dampingFactor".to_string(),
                reason: "dampingFactor must be between 0 and 1".to_string(),
            });
        }
        if parse_pagerank_orientation(&self.direction).is_err() {
            return Err(ConfigError::InvalidParameter {
                parameter: "direction".to_string(),
                reason: "direction must be one of outgoing, incoming, or both".to_string(),
            });
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for PageRankConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        PageRankConfig::validate(self)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PageRankResult {
    pub scores: Vec<f64>,
    pub ran_iterations: usize,
    pub did_converge: bool,
    pub node_count: usize,
    pub execution_time: Duration,
}

/// Statistics about PageRank computation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PageRankStats {
    pub node_count: usize,
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub stddev: f64,
    pub p50: f64,
    pub p90: f64,
    pub p99: f64,
    pub iterations_ran: usize,
    pub converged: bool,
    pub execution_time_ms: u64,
}

/// Summary of a mutate operation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PageRankMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PageRankWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for PageRank: summary + updated store
#[derive(Debug, Clone)]
pub struct PageRankMutateResult {
    pub summary: PageRankMutationSummary,
    pub updated_store: Arc<crate::types::prelude::DefaultGraphStore>,
}

/// PageRank result builder (facade adapter).
pub struct PageRankResultBuilder {
    result: PageRankResult,
}

impl PageRankResultBuilder {
    pub fn new(result: PageRankResult) -> Self {
        Self { result }
    }

    pub fn stats(&self) -> PageRankStats {
        let scores = &self.result.scores;
        if scores.is_empty() {
            return PageRankStats {
                min: 0.0,
                max: 0.0,
                mean: 0.0,
                stddev: 0.0,
                p50: 0.0,
                p90: 0.0,
                p99: 0.0,
                node_count: self.result.node_count,
                iterations_ran: self.result.ran_iterations,
                converged: self.result.did_converge,
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

        PageRankStats {
            min,
            max,
            mean,
            stddev,
            p50: percentile(50.0),
            p90: percentile(90.0),
            p99: percentile(99.0),
            node_count: self.result.node_count,
            iterations_ran: self.result.ran_iterations,
            converged: self.result.did_converge,
            execution_time_ms: self.result.execution_time.as_millis() as u64,
        }
    }

    pub fn execution_time_ms(&self) -> u64 {
        self.result.execution_time.as_millis() as u64
    }
}

pub fn parse_pagerank_orientation(direction: &str) -> Result<Orientation, AlgorithmError> {
    match direction.trim().to_lowercase().as_str() {
        "outgoing" | "natural" => Ok(Orientation::Natural),
        "incoming" | "reverse" => Ok(Orientation::Reverse),
        "both" | "undirected" => Ok(Orientation::Undirected),
        other => Err(AlgorithmError::Execution(format!(
            "Invalid PageRank direction '{other}'. Use 'outgoing', 'incoming', or 'both'"
        ))),
    }
}

pub fn pagerank_progress_task(max_iterations: usize) -> LeafTask {
    Tasks::leaf_with_volume("PageRank".to_string(), max_iterations)
}

define_algorithm_spec! {
    name: "pagerank",
    output_type: PageRankResult,
    projection_hint: VertexCentric,
    modes: [Stream, Stats, MutateNodeProperty],

    execute: |_self, graph_store, config, _context| {
        let parsed: PageRankConfig = serde_json::from_value(config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        parsed
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;

        let start = Instant::now();

        let storage = PageRankStorageRuntime::new(
            graph_store,
            parse_pagerank_orientation(&parsed.direction)?,
            parsed.relationship_weight_property.as_deref(),
        )?;

        let sources = parsed.source_nodes.clone().map(|v| v.into_iter().collect());

        let computation = PageRankComputationRuntime::new(
            parsed.max_iterations,
            parsed.damping_factor,
            parsed.tolerance,
            sources,
            parsed.variant,
        );
        let concurrency = Concurrency::of(parsed.concurrency.max(1));

        // Lightweight progress hook (executor can override/ignore).
        let mut progress = TaskProgressTracker::with_concurrency(
            pagerank_progress_task(parsed.max_iterations),
            concurrency.value(),
        );
        progress.begin_subtask_with_volume(parsed.max_iterations);

        let run = storage.run(&computation, concurrency, &mut progress);

        progress.end_subtask();

        Ok(PageRankResult {
            scores: run.scores,
            ran_iterations: run.ran_iterations,
            did_converge: run.did_converge,
            node_count: graph_store.node_count(),
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
        if result.scores.len() != node_count {
            return Err(AlgorithmError::Execution(format!(
                "pagerank returned {} scores for {} nodes",
                result.scores.len(),
                node_count
            )));
        }

        let backend = VecDouble::from(result.scores.clone());
        let values = DefaultDoubleNodePropertyValues::from_collection(backend, node_count);
        let values: Arc<dyn NodePropertyValues> = Arc::new(values);

        graph_store
            .add_node_property(labels, mutate_property.to_string(), values)
            .map_err(|e| AlgorithmError::Execution(e.to_string()))?;

        Ok(node_count)
    }
}
