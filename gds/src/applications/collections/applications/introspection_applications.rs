//! Introspection handlers for the Dataset-first Collections facade.
//!
//! Surfaces the backend / format / modifier envelope attached to a named
//! catalog entry as a stable wire summary. Execution-capability detection
//! (SIMD / GPU / parallel compute) is deferred until the relevant
//! compute-kernel layer exposes a classification API.

use serde_json::{json, Value};

use crate::applications::collections::configs::DatasetCapabilitiesConfig;
use crate::applications::collections::results::CapabilitySummary;
use crate::applications::services::collections_context::CollectionsContext;
use crate::collections::catalog::{CatalogError, CollectionsCatalogDiskEntry};

fn ok(op: &str, data: Value) -> Value {
    json!({ "ok": true, "op": op, "data": data })
}

fn err(op: &str, code: &str, message: impl Into<String>) -> Value {
    json!({
        "ok": false,
        "op": op,
        "error": { "code": code, "message": message.into() }
    })
}

fn parse_config<T: serde::de::DeserializeOwned>(op: &str, request: &Value) -> Result<T, Value> {
    serde_json::from_value::<T>(request.clone())
        .map_err(|e| err(op, "INVALID_REQUEST", e.to_string()))
}

fn catalog_error(op: &str, e: CatalogError) -> Value {
    let code = match &e {
        CatalogError::NotFound(_) => "NOT_FOUND",
        _ => "CATALOG_ERROR",
    };
    err(op, code, e.to_string())
}

fn entry_to_summary(dataset_name: &str, entry: &CollectionsCatalogDiskEntry) -> CapabilitySummary {
    let mut storage_modifiers = Vec::new();
    if let Some(compression) = entry.io_policy.compression.as_deref() {
        if !compression.trim().is_empty() {
            storage_modifiers.push(format!("compression:{compression}"));
        }
    }
    if let Some(batch_size) = entry.io_policy.batch_size {
        storage_modifiers.push(format!("batchSize:{batch_size}"));
    }

    CapabilitySummary {
        dataset_name: dataset_name.to_string(),
        backend: format!("{:?}", entry.backend),
        format: format!("{:?}", entry.io_policy.format),
        execution_capabilities: Vec::new(),
        storage_modifiers,
        extensions: entry.extensions.clone(),
    }
}

/// `datasetCapabilities` handler.
pub(crate) fn handle_capabilities(ctx: &CollectionsContext, request: &Value) -> Value {
    let op = &ctx.op;
    let cfg: DatasetCapabilitiesConfig = match parse_config(op, request) {
        Ok(v) => v,
        Err(e) => return e,
    };

    if cfg.dataset_name.trim().is_empty() {
        return err(op, "INVALID_REQUEST", "datasetName missing or empty");
    }

    let catalog_handle = match ctx.dataset_catalog() {
        Ok(v) => v,
        Err(e) => return catalog_error(op, e),
    };
    let catalog = catalog_handle
        .read()
        .expect("dataset catalog read lock poisoned");

    let Some(entry) = catalog.catalog().get(&cfg.dataset_name) else {
        return err(
            op,
            "NOT_FOUND",
            format!("Catalog entry '{}' not found", cfg.dataset_name),
        );
    };

    let summary = entry_to_summary(&cfg.dataset_name, entry);
    match serde_json::to_value(summary) {
        Ok(data) => ok(op, data),
        Err(e) => err(op, "SERIALIZATION_ERROR", e.to_string()),
    }
}
