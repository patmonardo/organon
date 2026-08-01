//! GraphSAGE facade (builder API).

use crate::algo::algorithms::traits as facade;
use crate::algo::algorithms::ConfigValidator;
use crate::algo::embeddings::graphsage::spec::{GraphSageConfig, GraphSageResult};
use crate::algo::embeddings::graphsage::GraphSageStorageRuntime;
use crate::procedures::model_catalog::{shared_in_memory_model_catalog, ModelCatalogFacade};
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::Orientation;
use crate::task::concurrency::TerminationFlag;
use crate::task::progress::{TaskProgressTracker, Tasks};
use crate::types::DefaultGraphStore;
use crate::types::GraphStore;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphSageStats {
    #[serde(rename = "nodeCount")]
    pub node_count: u64,
    #[serde(rename = "embeddingDimension")]
    pub embedding_dimension: u64,
    #[serde(rename = "computeMillis")]
    pub compute_millis: u64,
    pub success: bool,
}

/// GraphSAGE builder for inference/embeddings generation.
#[derive(Clone)]
pub struct GraphSageBuilder {
    graph_store: Arc<DefaultGraphStore>,
    model_user: String,
    model_name: String,
    batch_size: usize,
    concurrency: usize,
    model_catalog: Arc<ModelCatalogFacade>,
}

impl GraphSageBuilder {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            model_user: "anonymous".to_string(),
            model_name: "".to_string(),
            batch_size: 100,
            concurrency: num_cpus::get().max(1),
            model_catalog: shared_in_memory_model_catalog(),
        }
    }

    pub fn model_user(mut self, user: impl Into<String>) -> Self {
        self.model_user = user.into();
        self
    }

    pub fn model_name(mut self, name: impl Into<String>) -> Self {
        self.model_name = name.into();
        self
    }

    pub fn batch_size(mut self, batch_size: usize) -> Self {
        self.batch_size = batch_size;
        self
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    pub fn model_catalog(mut self, model_catalog: Arc<ModelCatalogFacade>) -> Self {
        self.model_catalog = model_catalog;
        self
    }

    pub fn validate(&self) -> facade::Result<()> {
        if self.model_name.is_empty() {
            return Err(AlgorithmError::Execution(
                "modelName must be specified".to_string(),
            ));
        }

        ConfigValidator::in_range(self.batch_size as f64, 1.0, 1_000_000_000.0, "batch_size")?;
        ConfigValidator::in_range(self.concurrency as f64, 1.0, 1_000_000.0, "concurrency")?;
        Ok(())
    }

    pub fn run(self) -> facade::Result<GraphSageResult> {
        self.validate()?;

        // Directly call the storage runtime
        let storage = GraphSageStorageRuntime::new();
        let rel_types = std::collections::HashSet::new();
        let graph = self
            .graph_store
            .get_graph_with_types_selectors_and_orientation(
                &rel_types,
                &std::collections::HashMap::new(),
                Orientation::Natural,
            )
            .map_err(|error| AlgorithmError::Graph(error.to_string()))?;

        let graphsage_config = GraphSageConfig {
            model_user: self.model_user,
            model_name: self.model_name,
            batch_size: self.batch_size,
            concurrency: self.concurrency,
        };

        storage.compute(
            graph,
            &graphsage_config,
            self.model_catalog.as_ref(),
            TaskProgressTracker::new(Tasks::leaf_with_volume(
                "GraphSAGE".to_string(),
                self.graph_store.node_count(),
            )),
            TerminationFlag::running_true(),
        )
    }

    pub fn stats(self) -> facade::Result<GraphSageStats> {
        let start = Instant::now();
        let result = self.run()?;
        let compute_millis = start.elapsed().as_millis() as u64;

        Ok(GraphSageStats {
            node_count: result.node_count as u64,
            embedding_dimension: result.embedding_dimension as u64,
            compute_millis,
            success: true,
        })
    }
}
