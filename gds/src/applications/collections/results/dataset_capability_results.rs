//! Serde-facing response models for Dataset backend / capability introspection.
//!
//! The wire schema distinguishes three taxonomies agreed in the Collections
//! plan:
//! - storage **backend** (Vec / Huge / Arrow / ...),
//! - execution **capabilities** (SIMD / GPU / parallel / ...),
//! - storage **modifiers** (compression / caching / encryption / ...).
//!
//! Batch 4 exposes the backend and format directly from the catalog entry
//! and surfaces compression as the first storage modifier. Execution
//! capabilities are reported as an empty list until a capability-detection
//! layer is wired in.

use serde::{Deserialize, Serialize};

/// Response payload for `datasetCapabilities`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilitySummary {
    pub dataset_name: String,
    pub backend: String,
    pub format: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub execution_capabilities: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub storage_modifiers: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extensions: Vec<String>,
}
