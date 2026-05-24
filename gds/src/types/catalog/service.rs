use std::collections::HashMap;
use std::sync::Arc;

use crate::types::graph_store::DefaultGraphStore;

#[derive(Debug, thiserror::Error)]
pub enum CatalogError {
    #[error("Graph not found: {0}")]
    NotFound(String),
    #[error("Graph already exists: {0}")]
    AlreadyExists(String),
    #[error("Graph is currently shared and cannot be mutated in place: {0}")]
    GraphInUse(String),
}

#[derive(Clone, Debug)]
pub struct ListEntry {
    pub name: String,
    pub node_count: u64,
    pub relationship_count: u64,
    pub degree_distribution: Option<HashMap<u32, u64>>, // simple histogram
}

#[derive(Clone, Debug)]
pub struct Dropped {
    pub name: String,
    pub node_count: u64,
    pub relationship_count: u64,
}

#[derive(Clone, Debug)]
pub struct GraphMemoryUsage {
    pub bytes: u64,
    pub nodes: u64,
    pub relationships: u64,
}

/// GraphCatalog - minimal, types-only registry of named graph stores
pub trait GraphCatalog: Send + Sync {
    /// Insert or replace a graph store under a name
    fn set(&self, name: &str, store: Arc<DefaultGraphStore>);

    /// Get a graph store by name
    fn get(&self, name: &str) -> Option<Arc<DefaultGraphStore>>;

    /// Mutate a graph store in place when the catalog owns it uniquely.
    fn with_store_mut(
        &self,
        name: &str,
        mutator: &mut dyn FnMut(&mut DefaultGraphStore),
    ) -> Result<(), CatalogError>;

    /// Drop graphs by names
    fn drop(&self, names: &[&str], fail_if_missing: bool) -> Result<Vec<Dropped>, CatalogError>;

    /// List graphs; optional name filter; optional degree histogram
    fn list(&self, filter: Option<&str>, include_degree_dist: bool) -> Vec<ListEntry>;

    /// Report rough memory usage
    fn size_of(&self, name: &str) -> Result<GraphMemoryUsage, CatalogError>;
}
