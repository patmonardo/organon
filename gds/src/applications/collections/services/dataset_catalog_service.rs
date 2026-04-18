//! Shared dataset catalog service for the Collections facade.
//!
//! Mirrors the shape of [`GraphStoreCatalogService`] from `graph_store_catalog`
//! but resolves a **disk-backed** `DatasetCatalog` keyed by (username,
//! databaseId). The dataset catalog is persisted on disk, so the service
//! caches handles under a shared lock so multiple ops within the same
//! request/user/database see a consistent view.
//!
//! Not yet wired into the dispatcher; Batch 3 will use this to implement
//! the first vertical slice of catalog operations.

use crate::collections::catalog::CatalogError;
use crate::collections::dataset::catalog::DatasetCatalog;
use crate::core::User;
use crate::types::graph_store::DatabaseId;
use std::sync::{Arc, RwLock};

/// Service trait for resolving a per-(user, database) `DatasetCatalog` handle.
pub trait DatasetCatalogService: Send + Sync {
    /// Resolve (or lazily load) the `DatasetCatalog` for this user / database.
    fn dataset_catalog(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
    ) -> Result<Arc<RwLock<DatasetCatalog>>, CatalogError>;
}
