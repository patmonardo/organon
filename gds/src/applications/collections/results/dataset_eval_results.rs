//! Serde-facing response models for Dataset feature evaluation and reporting.

use serde::{Deserialize, Serialize};

use super::DatasetSummary;

/// Response payload for `datasetEvalFeature`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvalSummary {
    /// Name of the source dataset bound into the plan env.
    pub dataset_name: String,
    /// Lightweight summary of the dataset produced by the evaluation.
    pub produced_dataset: DatasetSummary,
    /// Evaluation mode that was actually applied (`"preview"` or `"fit"`).
    pub eval_mode: String,
    /// Echoed run identifier, if the caller supplied one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
}

/// Response payload for `datasetFeatureAttentionReport`.
///
/// The `attention_report` field is a direct projection of the kernel's
/// `PlanAttentionReport`. It is serialized as opaque JSON here because the
/// kernel type is already the stable public shape.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttentionReportSummary {
    pub dataset_name: String,
    pub eval_mode: String,
    pub attention_report: serde_json::Value,
}
