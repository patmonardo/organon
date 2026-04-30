//! Loader that resolves a disk-backed `DatasetCatalog` per (user, databaseId).
//!
//! Catalog roots are computed as `<base>/<username>/<database>` where `<base>`
//! is taken from the `GDS_COLLECTIONS_ROOT` environment variable and defaults
//! to `$TMPDIR/gds-collections-root`. The default intentionally keeps the
//! scaffold self-contained; production callers are expected to set
//! `GDS_COLLECTIONS_ROOT` explicitly.
//!
//! Handles are cached in-process under `Arc<RwLock<DatasetCatalog>>` so that
//! concurrent ops against the same (user, database) see the same manifest
//! state without re-reading from disk on every call.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use crate::collections::catalog::CatalogError;
use crate::collections::dataset::catalog::DatasetCatalog;
use crate::collections::dataset::compile::DatasetCompilation;
use crate::core::User;
use crate::types::graph_store::DatabaseId;

use super::super::services::dataset_catalog_service::DatasetCatalogService;

/// Environment variable controlling the base root for all dataset catalogs.
pub const GDS_COLLECTIONS_ROOT_ENV: &str = "GDS_COLLECTIONS_ROOT";

type CatalogRegistry = Arc<RwLock<HashMap<(String, String), Arc<RwLock<DatasetCatalog>>>>>;

/// In-memory store for named `DatasetCompilation` values, scoped per
/// (user, database). Used as a short-lived workspace for compile -> materialize
/// flows that span two TS-JSON requests.
pub type CompilationStore = Arc<RwLock<HashMap<String, DatasetCompilation>>>;

type CompilationRegistry = Arc<RwLock<HashMap<(String, String), CompilationStore>>>;

/// In-process, disk-backed implementation of [`DatasetCatalogService`] keyed
/// by (username, databaseName).
#[derive(Clone, Default)]
pub struct PerUserDbDatasetCatalogService {
    base_root: Option<PathBuf>,
    catalogs: CatalogRegistry,
    compilations: CompilationRegistry,
}

impl PerUserDbDatasetCatalogService {
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct a service pinned to an explicit base directory. Useful for
    /// tests; production callers typically rely on `GDS_COLLECTIONS_ROOT`.
    pub fn with_base_root(base_root: PathBuf) -> Self {
        Self {
            base_root: Some(base_root),
            catalogs: CatalogRegistry::default(),
            compilations: CompilationRegistry::default(),
        }
    }

    fn resolve_base_root(&self) -> PathBuf {
        if let Some(root) = &self.base_root {
            return root.clone();
        }
        if let Ok(raw) = std::env::var(GDS_COLLECTIONS_ROOT_ENV) {
            let trimmed = raw.trim();
            if !trimmed.is_empty() {
                return PathBuf::from(trimmed);
            }
        }
        std::env::temp_dir().join("gds-collections-root")
    }

    fn catalog_root(&self, username: &str, database_id: &DatabaseId) -> PathBuf {
        self.resolve_base_root()
            .join(sanitize_segment(username))
            .join(sanitize_segment(database_id.database_name()))
    }

    /// Resolve (or lazily create) the per-(user, database) compilation store.
    pub fn compilation_store(&self, user: &dyn User, database_id: &DatabaseId) -> CompilationStore {
        let key = (
            user.username().to_string(),
            database_id.database_name().to_string(),
        );

        if let Ok(map) = self.compilations.read() {
            if let Some(existing) = map.get(&key) {
                return existing.clone();
            }
        }

        let mut map = self
            .compilations
            .write()
            .expect("dataset compilations poisoned");
        map.entry(key)
            .or_insert_with(|| Arc::new(RwLock::new(HashMap::new())))
            .clone()
    }
}

impl DatasetCatalogService for PerUserDbDatasetCatalogService {
    fn dataset_catalog(
        &self,
        user: &dyn User,
        database_id: &DatabaseId,
    ) -> Result<Arc<RwLock<DatasetCatalog>>, CatalogError> {
        let key = (
            user.username().to_string(),
            database_id.database_name().to_string(),
        );

        if let Ok(map) = self.catalogs.read() {
            if let Some(existing) = map.get(&key) {
                return Ok(existing.clone());
            }
        }

        let root = self.catalog_root(user.username(), database_id);
        let loaded = DatasetCatalog::load(root)?;
        let handle = Arc::new(RwLock::new(loaded));

        let mut map = self.catalogs.write().expect("dataset catalogs poisoned");
        Ok(map.entry(key).or_insert(handle).clone())
    }
}

fn sanitize_segment(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect()
}
