//! Serde-facing response models for Dataset catalog operations.

use serde::{Deserialize, Serialize};

/// Summary of a single catalog entry as visible on the TS-JSON boundary.
///
/// This is intentionally a projection of the kernel `CollectionsCatalogDiskEntry`:
/// it preserves the stable catalog name, data-path, and high-level schema
/// shape without exposing internal backend/extension vocabulary directly.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogEntrySummary {
    pub name: String,
    pub data_path: String,
    pub format: String,
    pub backend: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column_count: Option<usize>,
}

/// Response payload for `datasetListCatalog`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogListResult {
    pub entries: Vec<CatalogEntrySummary>,
}

/// Lightweight dataset summary returned by catalog ops that materialize a
/// `Dataset` (register / ingest / load).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatasetSummary {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub row_count: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column_count: Option<usize>,
}

/// A single column description returned by `datasetSchema`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaFieldSummary {
    pub name: String,
    pub dtype: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nullable: Option<bool>,
}

/// Response payload for `datasetSchema`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaSummary {
    pub dataset_name: String,
    pub fields: Vec<SchemaFieldSummary>,
    /// Source of the schema: `"catalog"` (from the persisted manifest) or
    /// `"dataset"` (derived by loading the table).
    pub source: String,
}

/// Response payload for `datasetPreview`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewSummary {
    pub dataset_name: String,
    pub row_count: u64,
    pub column_count: usize,
    pub limit: usize,
    pub rows: serde_json::Value,
}

/// Response payload for `datasetRemove`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveSummary {
    pub dataset_name: String,
    pub removed: bool,
}
