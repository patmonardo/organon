//! Serde-facing request models for Dataset compilation and materialization.
//!
//! These configs wrap the SDSL / pipeline / GDSL compilation entrypoints from
//! `collections::dataset::toolchain` and `collections::dataset::compile`.
//! Heavy kernel-side types (SdslSpecification, DatasetPipeline, ProgramFeatures,
//! DatasetCompilation) are intentionally **not** exposed directly on the wire;
//! service-layer code is responsible for lowering these configs into them.

use serde::{Deserialize, Serialize};

/// Request for `datasetCompilePipeline`.
///
/// Accepts a pipeline/specification request model and produces a
/// `DatasetCompilation`. The exact schema of `specification`, `pipeline`, and
/// `program_features` is left opaque at Batch 2; each is a JSON envelope that
/// the service layer lowers into the kernel structs.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatasetCompilePipelineConfig {
    pub compilation_name: String,
    #[serde(default)]
    pub specification: serde_json::Value,
    #[serde(default)]
    pub pipeline: serde_json::Value,
    #[serde(default)]
    pub program_features: serde_json::Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ontology_manifest: Option<serde_json::Value>,
}

/// Request for `datasetCompileGdslSource`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatasetCompileGdslSourceConfig {
    pub compilation_name: String,
    pub gdsl_source: String,
    #[serde(default)]
    pub specification: serde_json::Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ontology_manifest: Option<serde_json::Value>,
}

/// Request for `datasetMaterializeCompilation`.
///
/// Either `compilation_name` references a previously produced compilation,
/// or `compilation` supplies a fresh one inline.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatasetMaterializeCompilationConfig {
    pub base_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compilation_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compilation: Option<serde_json::Value>,
}
