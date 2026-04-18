//! Serde-facing request models for Dataset feature evaluation and reporting.
//!
//! Batch 4 keeps `FeatureSpec` narrow but extensible: the only supported
//! `kind` today is `"identity"` (read the cataloged dataset unmodified).
//! Compositional specs (filter / select / item / split / dataop) will be
//! lowered through this schema in later batches as the wire surface
//! stabilizes.

use serde::{Deserialize, Serialize};

/// Declarative feature description. `kind = "identity"` is the only variant
/// recognized in Batch 4; unknown kinds yield `INVALID_REQUEST`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatureSpec {
    pub kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Request for `datasetEvalFeature`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatasetEvalFeatureConfig {
    /// Name of a cataloged dataset to bind as the evaluation source.
    pub dataset_name: String,
    /// Inline feature description. Only `kind = "identity"` is supported
    /// in Batch 4.
    pub feature_spec: FeatureSpec,
    /// Evaluation mode. Accepted values: `"preview"` (default) and `"fit"`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eval_mode: Option<String>,
    /// Optional preview row cap (only meaningful when `evalMode = "preview"`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview_rows: Option<usize>,
    /// Optional stable identifier for the run, echoed in responses.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
}

/// Request for `datasetFeatureAttentionReport`.
///
/// Shares the same `featureSpec` grammar as `datasetEvalFeature`, but runs
/// `Feature::attention_report` without materializing a new dataset.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatasetFeatureAttentionReportConfig {
    pub dataset_name: String,
    pub feature_spec: FeatureSpec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eval_mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview_rows: Option<usize>,
}

/// Request for `datasetCapabilities`.
///
/// Returns the backend / format / storage-modifier envelope for a named
/// catalog entry. No feature evaluation happens.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatasetCapabilitiesConfig {
    pub dataset_name: String,
}
