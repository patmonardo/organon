//! Process-wide singleton for the Dataset catalog service behind the
//! Collections facade. Mirrors the `TSJSON_CATALOG_SERVICE` singleton used
//! by the graph-store-catalog facade.

use once_cell::sync::Lazy;
use std::sync::Arc;

use super::dataset_catalog_loader::PerUserDbDatasetCatalogService;

/// Shared, lazily-initialized dataset catalog service used by the Collections
/// TS-JSON dispatcher. Tests and callers that need isolation should construct
/// a `PerUserDbDatasetCatalogService::with_base_root(..)` explicitly instead
/// of relying on this singleton.
pub static TSJSON_DATASET_CATALOG_SERVICE: Lazy<Arc<PerUserDbDatasetCatalogService>> =
    Lazy::new(|| Arc::new(PerUserDbDatasetCatalogService::new()));
