//! K-Means Facade
//!
//! Clusters nodes based on an array-valued node property (feature vector).
//!
//! Parameters (Neo4j GDS aligned):
//! - `k`
//! - `max_iterations`
//! - `delta_threshold`
//! - `number_of_restarts`
//! - `compute_silhouette`
//! - `concurrency`: parallel worker count (KMeans++ sampling + assignment + silhouette all run in parallel).
//! - `node_property`
//! - `sampler_type` (UNIFORM, KMEANSPP)
//! - `seed_centroids`
//! - `random_seed`

use crate::algo::algorithms::Result;
use crate::algo::algorithms::{ConfigValidator, WriteResult};
pub use crate::algo::kmeans::KMeansSamplerType;
use crate::algo::kmeans::{
    kmeans_progress_task, KMeansComputationRuntime, KMeansConfig, KMeansMutateResult,
    KMeansMutationSummary, KMeansResult, KMeansResultBuilder, KMeansStats, KMeansStorageRuntime,
};
use crate::collections::backends::vec::VecLong;
use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::TaskRegistry;
use crate::mem::{Estimate, MemoryRange};
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use crate::types::properties::node::DefaultLongNodePropertyValues;
use crate::types::properties::node::NodePropertyValues;
use crate::types::schema::NodeLabel;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize)]
pub struct KMeansRow {
    pub node_id: u64,
    pub community_id: u64,
    pub distance_from_center: f64,
}

#[derive(Clone)]
pub struct KMeansFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: KMeansConfig,
    task_registry: Option<TaskRegistry>,
}

impl KMeansFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: KMeansConfig {
                concurrency: 4,
                ..KMeansConfig::default()
            },
            task_registry: None,
        }
    }

    /// Create a facade using the spec.rs config model.
    pub fn from_spec_config(
        graph_store: Arc<DefaultGraphStore>,
        config: KMeansConfig,
    ) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;

        Ok(Self {
            graph_store,
            config,
            task_registry: None,
        })
    }

    /// Parse JSON into spec.rs config and return a configured facade.
    pub fn from_spec_json(
        graph_store: Arc<DefaultGraphStore>,
        raw_config: &serde_json::Value,
    ) -> Result<Self> {
        let parsed: KMeansConfig = serde_json::from_value(raw_config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        Self::from_spec_config(graph_store, parsed)
    }

    /// Apply a spec.rs config onto an existing facade.
    pub fn with_spec_config(mut self, config: KMeansConfig) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;
        self.config = config;
        Ok(self)
    }

    pub fn k(mut self, k: usize) -> Self {
        self.config.k = k;
        self
    }

    pub fn max_iterations(mut self, max_iterations: u32) -> Self {
        self.config.max_iterations = max_iterations;
        self
    }

    pub fn delta_threshold(mut self, delta_threshold: f64) -> Self {
        self.config.delta_threshold = delta_threshold;
        self
    }

    pub fn number_of_restarts(mut self, number_of_restarts: u32) -> Self {
        self.config.number_of_restarts = number_of_restarts;
        self
    }

    pub fn compute_silhouette(mut self, enabled: bool) -> Self {
        self.config.compute_silhouette = enabled;
        self
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.config.concurrency = concurrency;
        self
    }

    pub fn node_property(mut self, node_property: &str) -> Self {
        self.config.node_property = node_property.to_string();
        self
    }

    pub fn sampler_type(mut self, sampler_type: KMeansSamplerType) -> Self {
        self.config.sampler_type = sampler_type;
        self
    }

    pub fn seed_centroids(mut self, seed_centroids: Vec<Vec<f64>>) -> Self {
        self.config.seed_centroids = seed_centroids;
        self
    }

    pub fn random_seed(mut self, seed: u64) -> Self {
        self.config.random_seed = Some(seed);
        self
    }

    pub fn task_registry(mut self, task_registry: TaskRegistry) -> Self {
        self.task_registry = Some(task_registry);
        self
    }

    fn validate(&self) -> Result<()> {
        self.config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))
    }

    fn compute(&self) -> Result<KMeansResult> {
        self.validate()?;
        let start = Instant::now();

        let config = self.config.clone();
        let node_count = self.graph_store.node_count();

        let progress_task = kmeans_progress_task(
            node_count,
            config.number_of_restarts,
            config.compute_silhouette,
        );
        let mut progress_tracker = super::progress_tracker_for_task(
            progress_task,
            config.concurrency,
            self.task_registry.as_ref(),
        );

        let termination_flag = TerminationFlag::running_true();
        let storage = KMeansStorageRuntime::new();
        let mut runtime = KMeansComputationRuntime::new();

        let result = storage.compute_kmeans(
            &mut runtime,
            self.graph_store.as_ref(),
            &config,
            &mut progress_tracker,
            &termination_flag,
        )?;

        Ok(KMeansResult {
            execution_time: start.elapsed(),
            ..result
        })
    }

    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = KMeansRow>>> {
        let result = self.compute()?;
        let iter = result
            .communities
            .into_iter()
            .zip(result.distance_from_center)
            .enumerate()
            .map(
                |(node_id, (community_id, distance_from_center))| KMeansRow {
                    node_id: node_id as u64,
                    community_id,
                    distance_from_center,
                },
            );
        Ok(Box::new(iter))
    }

    pub fn stats(&self) -> Result<KMeansStats> {
        let result = self.compute()?;
        Ok(KMeansResultBuilder::new(result).stats())
    }

    /// Mutate mode: writes community assignments back to the graph store.
    pub fn mutate(self, property_name: &str) -> Result<KMeansMutateResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        let result = self.compute()?;

        let node_count = self.graph_store.node_count();
        let nodes_updated = node_count as u64;

        let longs: Vec<i64> = result.communities.into_iter().map(|c| c as i64).collect();
        let backend = VecLong::from(longs);
        let values = DefaultLongNodePropertyValues::from_collection(backend, node_count);
        let values: Arc<dyn NodePropertyValues> = Arc::new(values);

        let mut new_store = self.graph_store.as_ref().clone();
        let labels_set: HashSet<NodeLabel> = new_store.node_labels();
        new_store
            .add_node_property(labels_set, property_name.to_string(), values)
            .map_err(|e| {
                AlgorithmError::Execution(format!("KMeans mutate failed to add property: {e}"))
            })?;

        let summary = KMeansMutationSummary {
            nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: result.execution_time.as_millis() as u64,
        };

        Ok(KMeansMutateResult {
            summary,
            updated_store: Arc::new(new_store),
        })
    }

    /// Write mode: writes community assignments to a new graph.
    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        let res = self.mutate(property_name)?;
        Ok(WriteResult::new(
            res.summary.nodes_updated,
            property_name.to_string(),
            std::time::Duration::from_millis(res.summary.execution_time_ms),
        ))
    }

    /// Estimate memory usage.
    pub fn estimate_memory(&self) -> Result<MemoryRange> {
        self.validate()?;

        let node_count = self.graph_store.node_count();
        let k = self.config.k.max(1);
        let dimensions = 128;

        let best_communities = Estimate::size_of_int_array(node_count);
        let best_centroids = Estimate::size_of_object_array(k)
            .saturating_add(k.saturating_mul(Estimate::size_of_double_array(dimensions)));
        let nodes_in_cluster = Estimate::size_of_long_array(k);
        let distance_from_centroid = Estimate::size_of_double_array(node_count);
        let cluster_manager = nodes_in_cluster.saturating_add(best_centroids);
        let task_memory = self.config.concurrency.max(1).saturating_mul(
            Estimate::size_of_long_array(k)
                .saturating_add(k.saturating_mul(Estimate::size_of_double_array(dimensions))),
        );
        let silhouette = if self.config.compute_silhouette {
            Estimate::size_of_double_array(node_count)
        } else {
            0
        };
        let seeded_centroids = if self.config.seed_centroids.is_empty() {
            0
        } else {
            Estimate::size_of_object_array(self.config.seed_centroids.len()).saturating_add(
                self.config
                    .seed_centroids
                    .iter()
                    .map(|centroid| Estimate::size_of_double_array(centroid.len()))
                    .sum::<usize>(),
            )
        };
        let materialized_points = Estimate::size_of_object_array(node_count)
            .saturating_add(node_count.saturating_mul(Estimate::size_of_double_array(dimensions)));

        let total = best_communities
            .saturating_add(best_centroids)
            .saturating_add(nodes_in_cluster)
            .saturating_add(distance_from_centroid)
            .saturating_add(cluster_manager)
            .saturating_add(task_memory)
            .saturating_add(silhouette)
            .saturating_add(seeded_centroids)
            .saturating_add(materialized_points);

        Ok(MemoryRange::of_range(
            total,
            total.saturating_add(task_memory),
        ))
    }

    pub fn run(&self) -> Result<KMeansResult> {
        let result = self.compute()?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::backends::vec::VecDoubleArray;
    use crate::config::GraphStoreConfig;
    use crate::procedures::GraphFacade;
    use crate::types::graph::SimpleIdMap;
    use crate::types::graph_store::{
        Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName,
    };
    use crate::types::properties::node::DefaultDoubleArrayNodePropertyValues;
    use crate::types::schema::MutableGraphSchema;
    use std::collections::HashMap;

    fn store_from_points(points: Vec<Vec<f64>>) -> DefaultGraphStore {
        let node_count = points.len();
        let schema = MutableGraphSchema::empty().build();
        let original_ids: Vec<i64> = (0..node_count as i64).collect();
        let id_map = SimpleIdMap::from_original_ids(original_ids);

        let mut store = DefaultGraphStore::new(
            GraphStoreConfig::default(),
            GraphName::new("g"),
            DatabaseInfo::new(
                DatabaseId::new("db"),
                DatabaseLocation::remote("localhost", 7687, None, None),
            ),
            schema,
            Capabilities::default(),
            id_map,
            HashMap::new(),
        );

        let dense = points.into_iter().map(Some).collect::<Vec<_>>();
        let values = DefaultDoubleArrayNodePropertyValues::<VecDoubleArray>::from_collection(
            VecDoubleArray::from(dense),
            node_count,
        );
        let values: Arc<dyn NodePropertyValues> = Arc::new(values);
        store
            .add_node_property(store.node_labels(), "p".to_string(), values)
            .unwrap();
        store
    }

    #[test]
    fn facade_modes_on_small_points() {
        let store = store_from_points(vec![
            vec![0.0, 0.0],
            vec![0.1, 0.0],
            vec![10.0, 10.0],
            vec![10.1, 10.0],
        ]);
        let graph = GraphFacade::new(Arc::new(store));

        let rows: Vec<_> = graph
            .kmeans()
            .k(2)
            .node_property("p")
            .max_iterations(10)
            .number_of_restarts(2)
            .random_seed(42)
            .stream()
            .unwrap()
            .collect();
        assert_eq!(rows.len(), 4);

        let stats = graph
            .kmeans()
            .k(2)
            .node_property("p")
            .max_iterations(10)
            .stats()
            .unwrap();
        assert_eq!(stats.k, 2);

        let memory = graph
            .kmeans()
            .k(2)
            .node_property("p")
            .estimate_memory()
            .unwrap();
        assert!(memory.max() >= memory.min());

        let mutation = graph
            .kmeans()
            .k(2)
            .node_property("p")
            .max_iterations(10)
            .mutate("community")
            .unwrap();
        assert_eq!(mutation.summary.nodes_updated, 4);
        assert!(mutation.updated_store.has_node_property("community"));
    }
}
