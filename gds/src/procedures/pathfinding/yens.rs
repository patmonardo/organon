//! Yen's K-Shortest Paths Facade
//!
//! Single-pair K-shortest paths using Yen's algorithm.
//!
//! This facade runs the translated Yen's runtime against a live `DefaultGraphStore`.

use crate::algo::algorithms::pathfinding::{PathFindingResult, PathResult};
use crate::algo::algorithms::Result;
use crate::algo::yens::{
    YensComputationRuntime, YensConfig, YensMutateResult, YensMutationSummary, YensResult,
    YensResultBuilder, YensStats, YensStorageRuntime, YensWriteSummary,
};
use crate::core::utils::progress::{
    EmptyTaskRegistryFactory, TaskProgressTracker, TaskRegistryFactory, Tasks,
};
use crate::mem::MemoryRange;
use crate::projection::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::NodeId;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

// Additional import for error handling
use crate::projection::eval::algorithm::AlgorithmError;

/// Yen's algorithm facade - fluent configuration
pub struct YensFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: YensConfig,
    concurrency: usize,
    task_registry_factory: Option<Box<dyn TaskRegistryFactory>>,
    user_log_registry_factory: Option<Box<dyn TaskRegistryFactory>>, // Placeholder for now
}

/// Backwards-compatible alias (builder-style naming).
pub type YensBuilder = YensFacade;

impl YensFacade {
    /// Create a new Yen's builder bound to a live graph store.
    ///
    /// Defaults:
    /// - source/target: None (must be set)
    /// - k: 3
    /// - weight_property: "weight"
    /// - relationship_types: all types
    /// - direction: "outgoing"
    /// - track_relationships: false
    /// - concurrency: 1
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: YensConfig::default(),
            concurrency: 1,
            task_registry_factory: None,
            user_log_registry_factory: None,
        }
    }

    /// Create a facade using the spec.rs config model.
    pub fn from_spec_config(
        graph_store: Arc<DefaultGraphStore>,
        config: YensConfig,
    ) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;

        Ok(Self {
            graph_store,
            concurrency: config.concurrency,
            config,
            task_registry_factory: None,
            user_log_registry_factory: None,
        })
    }

    /// Parse JSON into spec.rs config and return a configured facade.
    pub fn from_spec_json(
        graph_store: Arc<DefaultGraphStore>,
        raw_config: &serde_json::Value,
    ) -> Result<Self> {
        let parsed: YensConfig = serde_json::from_value(raw_config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        Self::from_spec_config(graph_store, parsed)
    }

    /// Apply a spec.rs config onto an existing facade.
    pub fn with_spec_config(mut self, config: YensConfig) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;
        self.concurrency = config.concurrency;
        self.config = config;
        Ok(self)
    }

    pub fn source(mut self, source: u64) -> Self {
        self.config.source_node = i64::try_from(source).unwrap_or(-1);
        self
    }

    pub fn source_node(mut self, source: NodeId) -> Self {
        self.config.source_node = source;
        self
    }

    pub fn target(mut self, target: u64) -> Self {
        self.config.target_node = i64::try_from(target).unwrap_or(-1);
        self
    }

    pub fn target_node(mut self, target: NodeId) -> Self {
        self.config.target_node = target;
        self
    }

    pub fn k(mut self, k: usize) -> Self {
        self.config.k = k;
        self
    }

    pub fn weight_property(mut self, property: &str) -> Self {
        self.config.weight_property = property.to_string();
        self
    }

    pub fn relationship_types(mut self, relationship_types: Vec<String>) -> Self {
        self.config.relationship_types = relationship_types;
        self
    }

    pub fn direction(mut self, direction: &str) -> Self {
        self.config.direction = direction.to_string();
        self
    }

    pub fn track_relationships(mut self, enabled: bool) -> Self {
        self.config.track_relationships = enabled;
        self
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.config.concurrency = concurrency;
        self.concurrency = concurrency;
        self
    }

    /// Set task registry factory for progress tracking
    pub fn task_registry_factory(mut self, factory: Box<dyn TaskRegistryFactory>) -> Self {
        self.task_registry_factory = Some(factory);
        self
    }

    /// Set user log registry factory for progress tracking
    pub fn user_log_registry_factory(mut self, factory: Box<dyn TaskRegistryFactory>) -> Self {
        self.user_log_registry_factory = Some(factory);
        self
    }

    fn compute(self) -> Result<PathFindingResult> {
        self.config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;

        // Set up progress tracking placeholders for API consistency with other facades.
        let _task_registry_factory = self
            .task_registry_factory
            .unwrap_or_else(|| Box::new(EmptyTaskRegistryFactory));
        let _user_log_registry_factory = self
            .user_log_registry_factory
            .unwrap_or_else(|| Box::new(EmptyTaskRegistryFactory));

        let source_node = self.config.source_node;
        let target_node = self.config.target_node;

        let rel_types: HashSet<RelationshipType> = if self.config.relationship_types.is_empty() {
            self.graph_store.relationship_types()
        } else {
            RelationshipType::list_of(self.config.relationship_types.clone())
                .into_iter()
                .collect()
        };

        let (orientation, direction_byte) =
            match self.config.direction.to_ascii_lowercase().as_str() {
                "incoming" => (Orientation::Reverse, 1u8),
                _ => (Orientation::Natural, 0u8),
            };

        let selectors: HashMap<RelationshipType, String> = rel_types
            .iter()
            .map(|t| (t.clone(), self.config.weight_property.clone()))
            .collect();

        let graph_view = self
            .graph_store
            .get_graph_with_types_selectors_and_orientation(&rel_types, &selectors, orientation)
            .map_err(|e| AlgorithmError::Graph(e.to_string()))?;

        let storage = YensStorageRuntime::new(
            source_node,
            target_node,
            self.config.k,
            self.config.track_relationships,
            self.config.concurrency,
        );

        let mut computation = YensComputationRuntime::new(
            source_node,
            target_node,
            self.config.k,
            self.config.track_relationships,
            self.config.concurrency,
        );

        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("yens".to_string(), self.config.k),
            self.config.concurrency,
        );

        let start = std::time::Instant::now();
        let result: YensResult = storage.compute_yens(
            &mut computation,
            Some(graph_view.as_ref()),
            direction_byte,
            &mut progress_tracker,
        )?;
        YensResultBuilder::result(
            result,
            start.elapsed(),
            self.config.k,
            self.config.track_relationships,
        )
    }

    pub fn stream(self) -> Result<Box<dyn Iterator<Item = PathResult>>> {
        let result = self.compute()?;
        Ok(Box::new(result.paths.into_iter()))
    }

    pub fn stats(self) -> Result<YensStats> {
        let result = self.compute()?;
        let computation_time_ms = result
            .metadata
            .additional
            .get("computation_time_ms")
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);
        let spur_searches = result
            .metadata
            .additional
            .get("spur_searches")
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);
        let candidates_generated = result
            .metadata
            .additional
            .get("candidates_generated")
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);

        Ok(YensStats {
            paths_found: result.paths.len() as u64,
            computation_time_ms,
            execution_time_ms: result.metadata.execution_time.as_millis() as u64,
            spur_searches,
            candidates_generated,
        })
    }

    pub fn mutate(self, property_name: &str) -> Result<YensMutateResult> {
        if property_name.is_empty() {
            return Err(AlgorithmError::Execution(
                "property_name cannot be empty".to_string(),
            ));
        }
        let graph_store = Arc::clone(&self.graph_store);
        let result = self.compute()?;
        let paths = result.paths;

        let updated_store = crate::algo::algorithms::pathfinding::build_path_relationship_store(
            graph_store.as_ref(),
            property_name,
            &paths,
        )?;

        let summary = YensMutationSummary {
            nodes_updated: paths.len() as u64,
            property_name: property_name.to_string(),
            execution_time_ms: result.metadata.execution_time.as_millis() as u64,
        };

        Ok(YensMutateResult {
            summary,
            updated_store,
        })
    }

    pub fn write(self, property_name: &str) -> Result<YensWriteSummary> {
        if property_name.is_empty() {
            return Err(AlgorithmError::Execution(
                "property_name cannot be empty".to_string(),
            ));
        }
        let res = self.mutate(property_name)?;
        Ok(YensWriteSummary {
            nodes_written: res.summary.nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: res.summary.execution_time_ms,
        })
    }

    /// Estimate memory requirements for Yen's K-shortest paths execution
    ///
    /// Returns a memory range estimate based on path storage, priority queues, and graph overhead.
    pub fn estimate_memory(&self) -> MemoryRange {
        let node_count = self.graph_store.node_count();
        let k = self.config.k;
        let concurrency = self.config.concurrency.max(1);
        let relationship_count = self.graph_store.relationship_count();

        let node_id_bytes = std::mem::size_of::<NodeId>();
        let cost_bytes = std::mem::size_of::<f64>();
        let path_entry_bytes = node_id_bytes.saturating_add(cost_bytes).saturating_add(
            if self.config.track_relationships {
                node_id_bytes
            } else {
                0
            },
        );

        let accepted_paths_memory = k
            .saturating_mul(node_count)
            .saturating_mul(path_entry_bytes);

        let candidate_paths_memory = k
            .saturating_mul(k.max(concurrency))
            .saturating_mul(node_count)
            .saturating_mul(path_entry_bytes);

        let dijkstra_worker_memory = node_count.saturating_mul(
            cost_bytes
                .saturating_add(node_id_bytes)
                .saturating_add(1)
                .saturating_add(if self.config.track_relationships {
                    node_id_bytes
                } else {
                    0
                }),
        );
        let relationship_filter_memory = concurrency
            .saturating_mul(k.max(1))
            .saturating_mul(node_id_bytes);
        let graph_cursor_memory =
            relationship_count.saturating_mul(node_id_bytes.saturating_add(cost_bytes));

        let min_memory = accepted_paths_memory
            .saturating_add(dijkstra_worker_memory)
            .saturating_add(relationship_filter_memory);
        let max_memory = min_memory
            .saturating_add(candidate_paths_memory)
            .saturating_add(dijkstra_worker_memory.saturating_mul(concurrency.saturating_sub(1)))
            .saturating_add(graph_cursor_memory);

        MemoryRange::of_range(min_memory, max_memory.max(min_memory))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::procedures::GraphFacade;
    use crate::types::random::{RandomGraphConfig, RandomRelationshipConfig};
    use serde_json::json;

    fn random_store(seed: u64) -> Arc<DefaultGraphStore> {
        let config = RandomGraphConfig {
            seed: Some(seed),
            node_count: 8,
            relationships: vec![RandomRelationshipConfig::new("REL", 0.7)],
            ..RandomGraphConfig::default()
        };

        Arc::new(DefaultGraphStore::random(&config).unwrap())
    }

    #[test]
    fn test_builder_validation() {
        let store = random_store(99);

        assert!(YensBuilder::new(Arc::clone(&store))
            .source_node(-1)
            .config
            .validate()
            .is_err());
        assert!(YensBuilder::new(Arc::clone(&store))
            .target_node(-1)
            .config
            .validate()
            .is_err());
        assert!(YensBuilder::new(Arc::clone(&store))
            .source(0)
            .target(1)
            .k(0)
            .config
            .validate()
            .is_err());
    }

    #[test]
    fn test_from_spec_json_accepts_java_aliases() {
        let store = random_store(102);
        let facade = YensFacade::from_spec_json(
            Arc::clone(&store),
            &json!({
                "sourceNode": 0,
                "targetNode": 3,
                "k": 5,
                "relationshipWeightProperty": "travelTime",
                "relationshipTypes": ["REL"],
                "trackRelationships": true,
                "concurrency": 4
            }),
        )
        .unwrap();

        assert_eq!(facade.config.source_node, 0);
        assert_eq!(facade.config.target_node, 3);
        assert_eq!(facade.config.k, 5);
        assert_eq!(facade.config.weight_property, "travelTime");
        assert_eq!(facade.config.relationship_types, vec!["REL"]);
        assert!(facade.config.track_relationships);
        assert_eq!(facade.config.concurrency, 4);
        assert_eq!(facade.concurrency, 4);
    }

    #[test]
    fn test_with_spec_config_syncs_facade_state() {
        let store = random_store(103);
        let config = YensConfig {
            source_node: 0,
            target_node: 3,
            k: 2,
            weight_property: "cost".to_string(),
            relationship_types: vec!["REL".to_string()],
            direction: "outgoing".to_string(),
            track_relationships: true,
            concurrency: 3,
        };

        let facade = YensFacade::new(store).with_spec_config(config).unwrap();

        assert_eq!(facade.config.weight_property, "cost");
        assert_eq!(facade.config.relationship_types, vec!["REL"]);
        assert!(facade.config.track_relationships);
        assert_eq!(facade.config.concurrency, 3);
        assert_eq!(facade.concurrency, 3);
    }

    #[test]
    fn test_builder_methods_sync_spec_config() {
        let store = random_store(104);
        let facade = YensFacade::new(store)
            .source(1)
            .target(4)
            .k(6)
            .weight_property("latency")
            .relationship_types(vec!["REL".to_string()])
            .track_relationships(true)
            .concurrency(2);

        assert_eq!(facade.config.source_node, 1);
        assert_eq!(facade.config.target_node, 4);
        assert_eq!(facade.config.k, 6);
        assert_eq!(facade.config.weight_property, "latency");
        assert_eq!(facade.config.relationship_types, vec!["REL"]);
        assert!(facade.config.track_relationships);
        assert_eq!(facade.config.concurrency, 2);
        assert_eq!(facade.concurrency, 2);
    }

    #[test]
    fn test_estimate_memory_scales_with_k_and_concurrency() {
        let store = random_store(105);
        let small = YensFacade::new(Arc::clone(&store))
            .source(0)
            .target(3)
            .k(1)
            .concurrency(1)
            .estimate_memory();
        let larger = YensFacade::new(store)
            .source(0)
            .target(3)
            .k(5)
            .track_relationships(true)
            .concurrency(4)
            .estimate_memory();

        assert!(larger.min() > small.min());
        assert!(larger.max() > small.max());
        assert!(larger.max() >= larger.min());
    }

    #[test]
    fn test_stream_smoke() {
        let config = RandomGraphConfig {
            seed: Some(101),
            node_count: 10,
            relationships: vec![RandomRelationshipConfig::new("REL", 0.8)],
            ..RandomGraphConfig::default()
        };

        let store = Arc::new(DefaultGraphStore::random(&config).unwrap());
        let graph = GraphFacade::new(store);

        let _rows: Vec<_> = graph
            .yens()
            .source(0)
            .target(3)
            .k(3)
            .relationship_types(vec!["REL".to_string()])
            .weight_property("weight")
            .stream()
            .unwrap()
            .collect();
    }
}
