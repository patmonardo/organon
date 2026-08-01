use std::sync::Arc;

use crate::applications::graph_store_catalog::configs::SamplingConfig;
use crate::applications::graph_store_catalog::loaders::GraphStoreCatalogService;
use crate::applications::graph_store_catalog::results::SamplingResult;
use crate::applications::services::logging::Log;
use crate::core::User;
use crate::types::graph::IdMap as _;
use crate::types::graph::MappedNodeId;
use crate::types::graph::OriginalNodeId;
use crate::types::graph_store::GraphStore as _;
use crate::types::graph_store::{DatabaseId, GraphName};
use rand::seq::SliceRandom;
use rand::SeedableRng;

/// GraphSamplingApplication
///
/// Java parity: sampling produces a new sampled graph and stores it in the catalog.
/// Rust pass-1: uses `DefaultGraphStore::commit_induced_subgraph_by_original_node_ids`.
#[derive(Clone)]
pub struct GraphSamplingApplication {
    #[allow(dead_code)]
    log: Log,
    graph_store_catalog_service: Arc<dyn GraphStoreCatalogService>,
}

impl GraphSamplingApplication {
    pub fn new(log: Log, graph_store_catalog_service: Arc<dyn GraphStoreCatalogService>) -> Self {
        Self {
            log,
            graph_store_catalog_service,
        }
    }

    pub fn compute(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
        origin_graph_name: &str,
        cfg: &SamplingConfig,
    ) -> Result<SamplingResult, String> {
        let origin = self.graph_store_catalog_service.get_graph_store(
            user,
            database_id,
            origin_graph_name,
        )?;

        let origin_nodes = origin.node_count() as u64;
        let origin_relationships = origin.relationship_count() as u64;

        if origin_nodes == 0 {
            return Err("Cannot sample from an empty graph".to_string());
        }

        let mut sample_size: usize = if let Some(n) = cfg.sample_node_count {
            if n == 0 {
                return Err("sampling_config.sampleNodeCount must be > 0".to_string());
            }
            n
        } else if let Some(r) = cfg.sample_ratio {
            if !(0.0 < r && r <= 1.0) {
                return Err("sampling_config.sampleRatio must be in (0.0, 1.0]".to_string());
            }
            (((origin_nodes as f64) * r).ceil() as usize).max(1)
        } else {
            // Default: half the nodes, but at least 1.
            ((origin_nodes as usize) / 2).max(1)
        };

        sample_size = sample_size.min(origin_nodes as usize);

        let sampled_graph_name = cfg
            .sampled_graph_name
            .as_deref()
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("{}_sampled", origin_graph_name));

        // Select nodes (original ids) deterministically.
        let graph = origin.graph();
        let mut original_ids = Vec::with_capacity(graph.node_count());
        for mapped_index in 0..graph.node_count() {
            let mapped_id = MappedNodeId::try_from(mapped_index)
                .map_err(|_| "Graph node count exceeds mapped ID space".to_string())?;
            let original_id = graph.to_original_node_id(mapped_id).ok_or_else(|| {
                format!("No original node ID for mapped node {mapped_id}")
            })?;
            original_ids.push(original_id);
        }

        let seed = cfg.seed.unwrap_or(0);
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        original_ids.shuffle(&mut rng);

        let selected: Vec<OriginalNodeId> = original_ids.into_iter().take(sample_size).collect();
        let induced = origin
            .commit_induced_subgraph_by_original_node_ids(
                GraphName::new(sampled_graph_name.clone()),
                &selected,
            )
            .map_err(|e| format!("Failed to induce sampled subgraph: {e}"))?;

        let sampled_nodes = induced.store.node_count() as u64;
        let sampled_relationships = induced.store.relationship_count() as u64;

        self.graph_store_catalog_service
            .graph_catalog(user, database_id)
            .set(sampled_graph_name.as_str(), Arc::new(induced.store));

        Ok(SamplingResult::new(
            sampled_graph_name,
            origin_nodes,
            sampled_nodes,
            origin_relationships,
            sampled_relationships,
        ))
    }
}
