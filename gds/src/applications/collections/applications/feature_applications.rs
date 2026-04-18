//! Feature evaluation and attention-report handlers for the Dataset-first
//! Collections facade.
//!
//! Batch 4 exposes two evaluation ops over cataloged datasets. The feature
//! grammar recognized on the wire is intentionally narrow (`kind = "identity"`)
//! so that a real end-to-end eval path exists before richer feature specs
//! are committed to the stable TS-JSON schema.

use serde_json::{json, Value};

use crate::applications::collections::configs::{
    DatasetEvalFeatureConfig, DatasetFeatureAttentionReportConfig, FeatureSpec,
};
use crate::applications::collections::results::{
    AttentionReportSummary, DatasetSummary, EvalSummary,
};
use crate::applications::services::collections_context::CollectionsContext;
use crate::collections::catalog::CatalogError;
use crate::collections::dataset::feature::Feature;
use crate::collections::dataset::plan::{EvalMode, Plan, PlanEnv, Source};

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

fn parse_eval_mode(op: &str, raw: Option<&str>) -> Result<EvalMode, Value> {
    let Some(value) = raw else {
        return Ok(EvalMode::Preview);
    };
    match value.trim().to_ascii_lowercase().as_str() {
        "" | "preview" => Ok(EvalMode::Preview),
        "fit" => Ok(EvalMode::Fit),
        other => Err(err(
            op,
            "INVALID_REQUEST",
            format!("Unsupported evalMode '{other}'"),
        )),
    }
}

/// Build a `Feature` from the narrow Batch 4 `FeatureSpec` grammar.
///
/// The bound dataset variable name is embedded in the plan so the evaluator
/// can look it up through `PlanEnv::bind_dataset` later.
fn feature_from_spec(op: &str, spec: &FeatureSpec, dataset_var: &str) -> Result<Feature, Value> {
    match spec.kind.as_str() {
        "identity" => {
            let mut plan = Plan::new(Source::Var(dataset_var.to_string()));
            if let Some(name) = &spec.name {
                plan = plan.named(name.clone());
            }
            Ok(Feature::new(plan))
        }
        other => Err(err(
            op,
            "INVALID_REQUEST",
            format!(
                "Unsupported featureSpec.kind '{other}'; only 'identity' is supported in this release"
            ),
        )),
    }
}

fn load_bound_env(
    op: &str,
    ctx: &CollectionsContext,
    dataset_name: &str,
    preview_rows: Option<usize>,
) -> Result<PlanEnv, Value> {
    if dataset_name.trim().is_empty() {
        return Err(err(op, "INVALID_REQUEST", "datasetName missing or empty"));
    }

    let catalog_handle = ctx.dataset_catalog().map_err(|e| catalog_error(op, e))?;
    let catalog = catalog_handle
        .read()
        .expect("dataset catalog read lock poisoned");

    let dataset = catalog
        .load_table(dataset_name)
        .map_err(|e| catalog_error(op, e))?;

    let mut env = PlanEnv::new().bind_dataset(dataset_name.to_string(), dataset);
    if let Some(n) = preview_rows {
        env = env.with_preview_rows(n);
    }
    Ok(env)
}

fn eval_mode_label(mode: EvalMode) -> &'static str {
    match mode {
        EvalMode::Preview => "preview",
        EvalMode::Fit => "fit",
    }
}

/// `datasetEvalFeature` handler.
pub(crate) fn handle_eval_feature(ctx: &CollectionsContext, request: &Value) -> Value {
    let op = &ctx.op;
    let cfg: DatasetEvalFeatureConfig = match parse_config(op, request) {
        Ok(v) => v,
        Err(e) => return e,
    };

    let mode = match parse_eval_mode(op, cfg.eval_mode.as_deref()) {
        Ok(v) => v,
        Err(e) => return e,
    };

    let env = match load_bound_env(op, ctx, &cfg.dataset_name, cfg.preview_rows) {
        Ok(v) => v,
        Err(e) => return e,
    };

    let feature = match feature_from_spec(op, &cfg.feature_spec, &cfg.dataset_name) {
        Ok(v) => v,
        Err(e) => return e,
    };

    let produced = match feature.eval_dataset(&env, mode) {
        Ok(d) => d,
        Err(e) => return err(op, "EVAL_ERROR", e.to_string()),
    };

    let summary = EvalSummary {
        dataset_name: cfg.dataset_name,
        produced_dataset: DatasetSummary {
            name: produced
                .name()
                .map(str::to_string)
                .unwrap_or_else(|| "<unnamed>".to_string()),
            row_count: Some(produced.row_count() as u64),
            column_count: Some(produced.column_count()),
        },
        eval_mode: eval_mode_label(mode).to_string(),
        run_id: cfg.run_id,
    };

    match serde_json::to_value(summary) {
        Ok(data) => ok(op, data),
        Err(e) => err(op, "SERIALIZATION_ERROR", e.to_string()),
    }
}

/// `datasetFeatureAttentionReport` handler.
pub(crate) fn handle_feature_attention_report(ctx: &CollectionsContext, request: &Value) -> Value {
    let op = &ctx.op;
    let cfg: DatasetFeatureAttentionReportConfig = match parse_config(op, request) {
        Ok(v) => v,
        Err(e) => return e,
    };

    let mode = match parse_eval_mode(op, cfg.eval_mode.as_deref()) {
        Ok(v) => v,
        Err(e) => return e,
    };

    let env = match load_bound_env(op, ctx, &cfg.dataset_name, cfg.preview_rows) {
        Ok(v) => v,
        Err(e) => return e,
    };

    let feature = match feature_from_spec(op, &cfg.feature_spec, &cfg.dataset_name) {
        Ok(v) => v,
        Err(e) => return e,
    };

    let report = feature.attention_report(Some(&env), mode);
    let report_value = match serde_json::to_value(report) {
        Ok(v) => v,
        Err(e) => return err(op, "SERIALIZATION_ERROR", e.to_string()),
    };

    let summary = AttentionReportSummary {
        dataset_name: cfg.dataset_name,
        eval_mode: eval_mode_label(mode).to_string(),
        attention_report: report_value,
    };

    match serde_json::to_value(summary) {
        Ok(data) => ok(op, data),
        Err(e) => err(op, "SERIALIZATION_ERROR", e.to_string()),
    }
}
