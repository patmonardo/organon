use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use super::concept::{PlanPrincipleReport, PlanSynthesisReport};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanStepReport {
    pub index: usize,
    pub op: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanAttentionReport {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    pub mode: String,
    pub source: JsonValue,
    pub steps: Vec<PlanStepReport>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planned_columns: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_columns: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_hint: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_hint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synthesis: Option<PlanSynthesisReport>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principle: Option<PlanPrincipleReport>,
}
