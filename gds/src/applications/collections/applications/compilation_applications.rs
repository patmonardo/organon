//! Compilation application handlers for the Dataset-first Collections facade.
//!
//! Implements the first compile/materialize slice: lower GDSL source into a
//! `DatasetCompilation`, cache it per-(user, database) under a stable
//! compilation name, and materialize a named compilation into the three
//! artifact/relations/properties Datasets via
//! `DatasetCompilation::materialize_artifact_datasets`.

use serde_json::{json, Value};

use crate::applications::collections::configs::{
    DatasetCompileGdslSourceConfig, DatasetMaterializeCompilationConfig,
};
use crate::applications::collections::results::{CompilationSummary, MaterializationSummary};
use crate::applications::services::collections_context::CollectionsContext;
use crate::collections::dataset::compile::DatasetCompilation;
use crate::collections::dataset::toolchain::DatasetToolChain;

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

fn compilation_summary(name: &str, compilation: &DatasetCompilation) -> CompilationSummary {
    let entrypoints: Vec<String> = compilation.entrypoints.iter().cloned().collect();
    CompilationSummary {
        compilation_name: name.to_string(),
        node_count: compilation.nodes.len(),
        entrypoint_count: entrypoints.len(),
        entrypoints,
    }
}

/// `datasetCompileGdslSource` handler.
pub(crate) fn handle_compile_gdsl_source(ctx: &CollectionsContext, request: &Value) -> Value {
    let op = &ctx.op;
    let cfg: DatasetCompileGdslSourceConfig = match parse_config(op, request) {
        Ok(v) => v,
        Err(e) => return e,
    };

    if cfg.compilation_name.trim().is_empty() {
        return err(op, "INVALID_REQUEST", "compilationName missing or empty");
    }
    if cfg.gdsl_source.trim().is_empty() {
        return err(op, "INVALID_REQUEST", "gdslSource missing or empty");
    }

    let compilation = match DatasetToolChain::image_from_gdsl_source(&cfg.gdsl_source) {
        Ok(c) => c,
        Err(e) => return err(op, "COMPILATION_ERROR", e.to_string()),
    };

    if let Err(e) = compilation.validate() {
        return err(op, "COMPILATION_ERROR", e);
    }

    let summary = compilation_summary(&cfg.compilation_name, &compilation);

    // Store under the request-supplied name for a later materialize call.
    let store = ctx.compilation_store();
    {
        let mut guard = store.write().expect("compilation store poisoned");
        guard.insert(cfg.compilation_name.clone(), compilation);
    }

    match serde_json::to_value(summary) {
        Ok(data) => ok(op, data),
        Err(e) => err(op, "SERIALIZATION_ERROR", e.to_string()),
    }
}

/// `datasetMaterializeCompilation` handler.
///
/// Batch 3: materialization is resolved from the per-(user, database)
/// compilation cache by `compilationName`. Inline compilation payloads on the
/// wire are deferred until the kernel `DatasetCompilation` gains a stable
/// serde-facing representation.
pub(crate) fn handle_materialize_compilation(ctx: &CollectionsContext, request: &Value) -> Value {
    let op = &ctx.op;
    let cfg: DatasetMaterializeCompilationConfig = match parse_config(op, request) {
        Ok(v) => v,
        Err(e) => return e,
    };

    if cfg.base_name.trim().is_empty() {
        return err(op, "INVALID_REQUEST", "baseName missing or empty");
    }
    if cfg.compilation.is_some() {
        return err(
            op,
            "UNSUPPORTED_OP",
            "Inline `compilation` payloads are not supported yet; supply `compilationName` \
             after a prior datasetCompileGdslSource call.",
        );
    }
    let Some(name) = cfg.compilation_name.as_deref() else {
        return err(op, "INVALID_REQUEST", "compilationName missing or empty");
    };
    if name.trim().is_empty() {
        return err(op, "INVALID_REQUEST", "compilationName missing or empty");
    }

    let store = ctx.compilation_store();
    let compilation = {
        let guard = store.read().expect("compilation store poisoned");
        match guard.get(name) {
            Some(c) => c.clone(),
            None => {
                return err(
                    op,
                    "NOT_FOUND",
                    format!("Compilation '{name}' not found in this session"),
                )
            }
        }
    };

    let artifacts = match compilation.materialize_artifact_datasets(&cfg.base_name) {
        Ok(a) => a,
        Err(e) => return err(op, "COMPILATION_ERROR", e.to_string()),
    };

    let summary = MaterializationSummary {
        base_name: cfg.base_name.clone(),
        artifacts_dataset: artifacts.artifacts.name().map(str::to_string),
        relations_dataset: artifacts.relations.name().map(str::to_string),
        properties_dataset: artifacts.properties.name().map(str::to_string),
        artifact_row_count: Some(artifacts.artifacts.row_count() as u64),
        relation_row_count: Some(artifacts.relations.row_count() as u64),
        property_row_count: Some(artifacts.properties.row_count() as u64),
    };

    match serde_json::to_value(summary) {
        Ok(data) => ok(op, data),
        Err(e) => err(op, "SERIALIZATION_ERROR", e.to_string()),
    }
}
