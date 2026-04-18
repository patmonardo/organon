//! Serde-facing response models for Dataset compilation and materialization.

use serde::{Deserialize, Serialize};

/// Response payload for `datasetCompilePipeline` and `datasetCompileGdslSource`.
///
/// A stable projection of `DatasetCompilation`: node / entrypoint counts plus
/// the stored compilation name. Raw IR nodes are not part of the wire schema.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompilationSummary {
    pub compilation_name: String,
    pub node_count: usize,
    pub entrypoint_count: usize,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entrypoints: Vec<String>,
}

/// Response payload for `datasetMaterializeCompilation`.
///
/// Reports the base artifact name and lightweight shapes of the three
/// artifact datasets (artifacts, relations, properties) produced by
/// `DatasetCompilation::materialize_artifact_datasets`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterializationSummary {
    pub base_name: String,
    pub artifacts_dataset: Option<String>,
    pub relations_dataset: Option<String>,
    pub properties_dataset: Option<String>,
    pub artifact_row_count: Option<u64>,
    pub relation_row_count: Option<u64>,
    pub property_row_count: Option<u64>,
}
