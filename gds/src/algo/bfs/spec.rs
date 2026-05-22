//! **BFS Algorithm Specification**
//!
//! **Translation Source**: `org.neo4j.gds.traversal.BFS`
//!
//! This module defines the BFS algorithm specification, configuration, and result types.

use super::BfsComputationRuntime;
use super::BfsStorageRuntime;
use crate::algo::algorithms::pathfinding::{
    PathFindingResult, PathFindingResultBuilder, PathResult,
};
use crate::algo::algorithms::{ExecutionMetadata, ResultBuilder};
use crate::config::validation::ConfigError;
use crate::core::utils::progress::TaskProgressTracker;
use crate::core::utils::progress::Tasks;
use crate::define_algorithm_spec;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::NodeId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// BFS algorithm configuration
///
/// Translation of: `BFSConfig.java` (lines 32-75)
/// This defines the parameters for BFS traversal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BfsConfig {
    /// Source node for BFS traversal
    #[serde(default, alias = "sourceNode")]
    pub source_node: NodeId,
    /// Target nodes to find (empty means find all reachable)
    #[serde(default, alias = "targetNodes")]
    pub target_nodes: Vec<NodeId>,
    /// Maximum depth to traverse (None means unlimited)
    #[serde(default, alias = "maxDepth")]
    pub max_depth: Option<u32>,
    /// Whether to track paths during traversal
    #[serde(default, alias = "trackPaths")]
    pub track_paths: bool,
    /// Concurrency level for parallel processing
    #[serde(default = "BfsConfig::default_concurrency")]
    pub concurrency: usize,
    /// Delta parameter for chunking (default 64)
    #[serde(default = "BfsConfig::default_delta")]
    pub delta: usize,
}

impl Default for BfsConfig {
    fn default() -> Self {
        Self {
            source_node: 0,
            target_nodes: Vec::new(),
            max_depth: None,
            track_paths: false,
            concurrency: Self::default_concurrency(),
            delta: Self::default_delta(),
        }
    }
}

impl BfsConfig {
    fn default_concurrency() -> usize {
        1
    }

    fn default_delta() -> usize {
        64
    }

    /// Validate configuration parameters
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.source_node < 0 {
            return Err(ConfigError::FieldValidation {
                field: "source_node".to_string(),
                message: "must be >= 0".to_string(),
            });
        }

        if self.target_nodes.iter().any(|&node_id| node_id < 0) {
            return Err(ConfigError::FieldValidation {
                field: "target_nodes".to_string(),
                message: "all target nodes must be >= 0".to_string(),
            });
        }

        if self.concurrency == 0 {
            return Err(ConfigError::FieldValidation {
                field: "concurrency".to_string(),
                message: "must be > 0".to_string(),
            });
        }

        if let Some(depth) = self.max_depth {
            if depth == 0 {
                return Err(ConfigError::FieldValidation {
                    field: "max_depth".to_string(),
                    message: "must be > 0 or None".to_string(),
                });
            }
        }
        Ok(())
    }
}

// Implement ValidatedConfig for BFS algorithm config by delegating to its validate
impl crate::config::ValidatedConfig for BfsConfig {
    fn validate(&self) -> Result<(), crate::config::validation::ConfigError> {
        self.validate().map_err(
            |e| crate::config::validation::ConfigError::InvalidParameter {
                parameter: "BfsConfig".to_string(),
                reason: e.to_string(),
            },
        )
    }
}

/// BFS algorithm result
///
/// Translation of: `BFSResult.java` (lines 76-120)
/// This contains the results of BFS traversal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BfsResult {
    /// Visited nodes in traversal order
    pub visited_nodes: Vec<NodeId>,
    /// Aggregated traversal depth for each visited node, aligned with visited_nodes.
    #[serde(default)]
    pub visited_depths: Vec<f64>,
    /// Computation time in milliseconds
    pub computation_time_ms: u64,
}

/// Individual path result from BFS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BfsPathResult {
    /// Source node
    pub source_node: NodeId,
    /// Target node
    pub target_node: NodeId,
    /// Path as sequence of node IDs
    pub node_ids: Vec<NodeId>,
    /// Path length (number of edges)
    pub path_length: u32,
}

/// Statistics about BFS computation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BfsStats {
    /// Number of nodes visited during traversal
    pub nodes_visited: u64,
    /// Maximum depth reached during traversal
    pub max_depth_reached: u64,
    /// Total computation time in milliseconds
    pub execution_time_ms: u64,
    /// Number of target nodes found (if any specified)
    pub targets_found: u64,
    /// Whether all targets were reached
    pub all_targets_reached: bool,
    /// Average branching factor (neighbors per node)
    pub avg_branching_factor: f64,
}

/// Summary of a mutate operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BfsMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BfsWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for BFS: summary + updated store
#[derive(Debug, Clone)]
pub struct BfsMutateResult {
    pub summary: BfsMutationSummary,
    pub updated_store: std::sync::Arc<crate::types::prelude::DefaultGraphStore>,
}

fn checked_u64(value: NodeId) -> u64 {
    u64::try_from(value).unwrap_or(0)
}

fn spec_path_to_core(source: NodeId, target: NodeId, depth: f64) -> PathResult {
    let target_u64 = checked_u64(target);
    PathResult {
        source: checked_u64(source),
        target: target_u64,
        path: vec![target_u64],
        cost: depth,
    }
}

fn result_paths(result: &BfsResult, source_node: NodeId) -> Vec<PathResult> {
    result
        .visited_nodes
        .iter()
        .copied()
        .enumerate()
        .map(|(index, node_id)| {
            let depth = result
                .visited_depths
                .get(index)
                .copied()
                .unwrap_or(index as f64);
            spec_path_to_core(source_node, node_id, depth)
        })
        .collect()
}

/// BFS result builder (pathfinding-family adapter).
pub struct BfsResultBuilder {
    result: BfsResult,
    execution_time: Duration,
    source_node: NodeId,
    target_nodes: Vec<NodeId>,
}

impl BfsResultBuilder {
    pub fn new(
        result: BfsResult,
        execution_time: Duration,
        source_node: NodeId,
        target_nodes: Vec<NodeId>,
    ) -> Self {
        Self {
            result,
            execution_time,
            source_node,
            target_nodes,
        }
    }

    pub fn result(
        result: BfsResult,
        execution_time: Duration,
        source_node: NodeId,
        target_nodes: Vec<NodeId>,
    ) -> Result<PathFindingResult, AlgorithmError> {
        Self::new(result, execution_time, source_node, target_nodes).build_pathfinding_result()
    }

    pub fn build_pathfinding_result(self) -> Result<PathFindingResult, AlgorithmError> {
        let paths = result_paths(&self.result, self.source_node);

        let mut additional = HashMap::new();
        additional.insert(
            "computation_time_ms".to_string(),
            self.result.computation_time_ms.to_string(),
        );
        additional.insert("targets".to_string(), self.target_nodes.len().to_string());

        let metadata = ExecutionMetadata {
            execution_time: self.execution_time,
            iterations: None,
            converged: None,
            additional,
        };

        PathFindingResultBuilder::new()
            .with_paths(paths)
            .with_metadata(metadata)
            .build()
            .map_err(|e| AlgorithmError::Execution(e.to_string()))
    }

    pub fn paths(&self) -> Vec<PathResult> {
        result_paths(&self.result, self.source_node)
    }

    pub fn stats(&self) -> BfsStats {
        let nodes_visited = self.result.visited_nodes.len() as u64;
        let targets = self.target_nodes.len() as u64;
        let targets_found = if self.target_nodes.is_empty() {
            0
        } else {
            self.target_nodes
                .iter()
                .filter(|target| self.result.visited_nodes.contains(target))
                .count() as u64
        };
        let all_targets_reached = targets > 0 && targets_found == targets;
        let max_depth_reached = self
            .result
            .visited_depths
            .iter()
            .copied()
            .fold(0.0_f64, f64::max)
            .floor() as u64;

        BfsStats {
            nodes_visited,
            max_depth_reached,
            execution_time_ms: self.execution_time.as_millis() as u64,
            targets_found,
            all_targets_reached,
            avg_branching_factor: 0.0,
        }
    }

    pub fn mutation_summary(&self, property_name: &str, nodes_updated: u64) -> BfsMutationSummary {
        BfsMutationSummary {
            nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: self.execution_time.as_millis() as u64,
        }
    }

    pub fn write_summary(&self, property_name: &str, nodes_written: u64) -> BfsWriteSummary {
        BfsWriteSummary {
            nodes_written,
            property_name: property_name.to_string(),
            execution_time_ms: self.execution_time.as_millis() as u64,
        }
    }

    pub fn execution_time_ms(&self) -> u64 {
        self.execution_time.as_millis() as u64
    }
}

// Generate the algorithm specification using focused macros
define_algorithm_spec! {
    name: "bfs",
    output_type: BfsResult,
    projection_hint: Dense,
    modes: [Stream, WriteNodeProperty],
    execute: |_self, graph_store, config_input, _context| {
        // Parse and validate configuration
        let parsed_config: BfsConfig = serde_json::from_value(config_input.clone())
            .map_err(|e| AlgorithmError::InvalidGraph(format!("Failed to parse config: {}", e)))?;

        parsed_config.validate()
            .map_err(|e| AlgorithmError::InvalidGraph(format!("Config validation failed: {}", e)))?;

        // Create storage and computation runtimes
        let storage = BfsStorageRuntime::new(
            parsed_config.source_node,
            parsed_config.target_nodes.clone(),
            parsed_config.max_depth,
            parsed_config.track_paths,
        )
        .with_delta(parsed_config.delta);

        // Execute BFS algorithm with a filtered/oriented view (defaults: all types, NATURAL)
        let rel_types: std::collections::HashSet<RelationshipType> = std::collections::HashSet::new();
        let graph_view = graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Natural)
            .map_err(|e| AlgorithmError::InvalidGraph(format!("Failed to obtain graph view: {}", e)))?;

        let node_count = graph_view.node_count() as usize;

        let mut computation = BfsComputationRuntime::new(
            parsed_config.source_node,
            parsed_config.track_paths,
            parsed_config.concurrency,
            node_count,
        );

        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf("BFS".to_string()),
            parsed_config.concurrency,
        );

        let result =
            storage.compute_bfs(&mut computation, Some(graph_view.as_ref()), &mut progress_tracker)?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::eval::algorithm::AlgorithmSpec;

    #[test]
    fn test_bfs_result() {
        let result = BfsResult {
            computation_time_ms: 5,
            visited_nodes: vec![0, 1, 2],
            visited_depths: vec![0.0, 1.0, 1.0],
        };

        assert_eq!(result.visited_nodes.len(), 3);
        assert_eq!(result.visited_nodes[0], 0);
    }

    #[test]
    fn test_bfs_path_result() {
        let path = BfsPathResult {
            source_node: 0,
            target_node: 3,
            node_ids: vec![0, 1, 2, 3],
            path_length: 3,
        };

        assert_eq!(path.source_node, 0);
        assert_eq!(path.target_node, 3);
        assert_eq!(path.node_ids.len(), 4);
        assert_eq!(path.path_length, 3);
    }

    #[test]
    fn test_bfs_config_default() {
        let config = BfsConfig::default();
        assert_eq!(config.source_node, 0);
        assert!(config.target_nodes.is_empty());
        assert!(config.max_depth.is_none());
        assert!(!config.track_paths);
        assert_eq!(config.concurrency, 1);
    }

    #[test]
    fn test_bfs_config_validation() {
        let mut config = BfsConfig::default();
        assert!(config.validate().is_ok());

        config.concurrency = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_bfs_algorithm_spec_contract() {
        let spec = BFSAlgorithmSpec::new("test_graph".to_string());
        assert_eq!(spec.name(), "bfs");
        assert_eq!(spec.graph_name(), "test_graph");
    }

    #[test]
    fn test_bfs_execution_modes() {
        let spec = BFSAlgorithmSpec::new("test_graph".to_string());
        // Test that the algorithm can be created
        assert_eq!(spec.graph_name(), "test_graph");
    }

    #[test]
    fn test_bfs_config_validation_integration() {
        // Macro validation_config does not validate custom fields; use BfsConfig::validate()
        let config = BfsConfig {
            concurrency: 4,
            ..Default::default()
        };
        assert!(config.validate().is_ok());

        let invalid_config = BfsConfig {
            concurrency: 0,
            ..Default::default()
        };
        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_bfs_config_accepts_java_aliases() {
        let config: BfsConfig = serde_json::from_value(serde_json::json!({
            "sourceNode": 0,
            "targetNodes": [2, 3],
            "maxDepth": 4,
            "trackPaths": true,
            "concurrency": 2,
            "delta": 64
        }))
        .unwrap();

        assert_eq!(config.source_node, 0);
        assert_eq!(config.target_nodes, vec![2, 3]);
        assert_eq!(config.max_depth, Some(4));
        assert!(config.track_paths);
    }

    #[test]
    fn test_bfs_paths_use_depth_as_cost() {
        let result = BfsResult {
            computation_time_ms: 5,
            visited_nodes: vec![0, 1, 3],
            visited_depths: vec![0.0, 1.0, 2.0],
        };

        let paths = BfsResultBuilder::new(result, Duration::from_millis(5), 0, vec![3]).paths();

        assert_eq!(paths[0].cost, 0.0);
        assert_eq!(paths[1].cost, 1.0);
        assert_eq!(paths[2].cost, 2.0);
    }

    #[test]
    fn test_bfs_focused_macro_integration() {
        let spec = BFSAlgorithmSpec::new("test_graph".to_string());
        assert_eq!(spec.name(), "bfs");
        assert_eq!(spec.graph_name(), "test_graph");

        // Test that the algorithm can be created
        assert_eq!(spec.graph_name(), "test_graph");
    }
}
