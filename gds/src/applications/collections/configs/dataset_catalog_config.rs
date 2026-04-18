//! Serde-facing request models for Dataset catalog operations.
//!
//! These are the TS-JSON wire structs for the first slice of `facade =
//! "collections"` ops: listing, registering table paths, ingesting tables,
//! and loading cataloged datasets by stable name.
//!
//! The wire schema stays small and dataset-centered; heavier Dataset toolchain
//! types are kept internal and lowered from these configs in service layers.

use serde::{Deserialize, Serialize};

/// Request for `datasetListCatalog`.
///
/// All list queries are scoped by the caller's `databaseId` from the shared
/// facade envelope; this struct exists mainly to carry optional filters as the
/// op grows.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatasetListCatalogConfig {
    /// Optional substring match on the catalog entry `name`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_filter: Option<String>,
}

/// Request for `datasetRegisterTablePath`.
///
/// Registers an existing table file under a stable catalog name without
/// copying data. Format is detected from the path when omitted.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatasetRegisterTablePathConfig {
    pub dataset_name: String,
    pub data_path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

/// Request for `datasetIngestTable`.
///
/// Batch 2 scaffold: the table payload wire format is deliberately left as an
/// opaque inline reference. Actual table materialization is lowered in the
/// service layer in a later batch.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatasetIngestTableConfig {
    pub dataset_name: String,
    pub format: String,
    /// Opaque inline table reference. The exact payload shape is reserved for
    /// the service-layer lowering (Batch 3+).
    #[serde(default)]
    pub table: serde_json::Value,
}

/// Request for `datasetLoad`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatasetLoadConfig {
    pub dataset_name: String,
}

/// Request for `datasetSchema`.
///
/// Returns the column schema of a cataloged dataset without materializing its
/// rows. When the catalog entry already carries a persisted schema it is
/// returned directly; otherwise the dataset is lazily loaded to derive one.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatasetSchemaConfig {
    pub dataset_name: String,
}

/// Request for `datasetPreview`.
///
/// Returns the top `limit` rows of a cataloged dataset as a JSON array of
/// row objects. `limit` defaults to 10 and is capped at 1000 to keep the
/// response envelope small.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatasetPreviewConfig {
    pub dataset_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
}

/// Request for `datasetRemove`.
///
/// Unregisters a catalog entry by name. The underlying data file on disk is
/// left untouched; this op only removes the manifest record so higher layers
/// stop seeing the entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatasetRemoveConfig {
    pub dataset_name: String,
}
