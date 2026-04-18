//! Shared context for the Collections TS-JSON dispatcher.
//!
//! Parses the standard facade envelope (`op`, `user`, `databaseId`) and
//! carries a lazily-resolved dataset catalog handle. Kept internal to the
//! `services` module so that collections ops share one validation surface.

use std::sync::Arc;
use std::sync::RwLock;

use serde_json::{json, Value};

use crate::applications::collections::loaders::{
    CompilationStore, PerUserDbDatasetCatalogService, TSJSON_DATASET_CATALOG_SERVICE,
};
use crate::applications::collections::services::dataset_catalog_service::DatasetCatalogService;
use crate::collections::catalog::CatalogError;
use crate::collections::dataset::catalog::DatasetCatalog;
use crate::core::User;
use crate::types::graph_store::DatabaseId;

/// Minimal User implementation used by the Collections dispatcher. Mirrors
/// the `TsjsonUser` held by `tsjson_support` but is kept local so the
/// collections facade does not depend on the graph-store catalog envelope.
#[derive(Clone, Debug)]
pub(crate) struct CollectionsUser {
    username: String,
    roles: Vec<String>,
    permissions: Vec<String>,
}

impl CollectionsUser {
    fn new(username: String, is_admin: bool) -> Self {
        Self {
            username,
            roles: if is_admin {
                vec!["admin".to_string()]
            } else {
                Vec::new()
            },
            permissions: Vec::new(),
        }
    }
}

impl User for CollectionsUser {
    fn username(&self) -> &str {
        &self.username
    }
    fn roles(&self) -> &[String] {
        &self.roles
    }
    fn is_authenticated(&self) -> bool {
        true
    }
    fn permissions(&self) -> &[String] {
        &self.permissions
    }
}

/// Parsed envelope for a `facade = "collections"` request.
#[allow(dead_code)] // some fields used via accessor methods
pub(crate) struct CollectionsContext {
    pub(crate) op: String,
    pub(crate) user: CollectionsUser,
    pub(crate) db: DatabaseId,
    pub(crate) catalog_service: Arc<PerUserDbDatasetCatalogService>,
}

impl CollectionsContext {
    /// Resolve (or lazily load) the dataset catalog handle for this request.
    pub(crate) fn dataset_catalog(&self) -> Result<Arc<RwLock<DatasetCatalog>>, CatalogError> {
        self.catalog_service.dataset_catalog(&self.user, &self.db)
    }

    /// Resolve (or lazily create) the compilation workspace for this request.
    pub(crate) fn compilation_store(&self) -> CompilationStore {
        self.catalog_service.compilation_store(&self.user, &self.db)
    }

    /// Test-only constructor using an isolated catalog service.
    #[cfg(test)]
    pub(crate) fn for_test(
        op: impl Into<String>,
        username: &str,
        database_id: &str,
        catalog_service: Arc<PerUserDbDatasetCatalogService>,
    ) -> Self {
        Self {
            op: op.into(),
            user: CollectionsUser::new(username.to_string(), false),
            db: DatabaseId::new(database_id),
            catalog_service,
        }
    }
}

fn err(op: &str, code: &str, message: impl Into<String>) -> Value {
    json!({
        "ok": false,
        "op": op,
        "error": { "code": code, "message": message.into() }
    })
}

pub(crate) fn parse_collections_context(request: &Value) -> Result<CollectionsContext, Value> {
    let op = request
        .get("op")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_string();

    if op.is_empty() {
        return Err(err("", "INVALID_REQUEST", "Missing required field: op"));
    }

    let user_obj = request
        .get("user")
        .and_then(|v| v.as_object())
        .ok_or_else(|| err(&op, "INVALID_REQUEST", "Missing required field: user"))?;

    let username = user_obj
        .get("username")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| err(&op, "INVALID_REQUEST", "user.username missing or empty"))?
        .to_string();

    let is_admin = user_obj
        .get("isAdmin")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let user = CollectionsUser::new(username, is_admin);

    let database_id = request
        .get("databaseId")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| err(&op, "INVALID_REQUEST", "databaseId missing or empty"))?;
    let db = DatabaseId::new(database_id);

    Ok(CollectionsContext {
        op,
        user,
        db,
        catalog_service: TSJSON_DATASET_CATALOG_SERVICE.clone(),
    })
}
