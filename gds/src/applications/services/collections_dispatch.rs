//! TS-JSON dispatcher for the Dataset-first Collections facade.
//!
//! Batch 3: the first vertical slice is wired in.
//! - Catalog: `datasetListCatalog`, `datasetRegisterTablePath`, `datasetLoad`.
//! - Compilation: `datasetCompileGdslSource`, `datasetMaterializeCompilation`.
//!
//! Remaining ops (`datasetIngestTable`, `datasetCompilePipeline`,
//! `datasetEvalFeature`, `datasetFeatureAttentionReport`,
//! `datasetCapabilities`) are recognized but return `UNSUPPORTED_OP` until
//! later batches.
//!
//! Kept intentionally thin, matching the graph-store-catalog dispatcher shape.

use serde_json::{json, Value};

use super::collections_context::parse_collections_context;
use crate::applications::collections::applications::{
    catalog_applications, compilation_applications, feature_applications,
    introspection_applications,
};

fn err(op: &str, code: &str, message: &str) -> Value {
    json!({
        "ok": false,
        "op": op,
        "error": { "code": code, "message": message }
    })
}

/// Route a `facade = "collections"` TS-JSON request by `op` name.
pub fn handle_collections(request: &Value) -> Value {
    let ctx = match parse_collections_context(request) {
        Ok(v) => v,
        Err(e) => return e,
    };

    match ctx.op.as_str() {
        // Catalog ops (Batch 3 implemented)
        "datasetListCatalog" => catalog_applications::handle_list_catalog(&ctx, request),
        "datasetRegisterTablePath" => {
            catalog_applications::handle_register_table_path(&ctx, request)
        }
        "datasetLoad" => catalog_applications::handle_load(&ctx, request),

        // Inline ingest (Batch 5 implemented)
        "datasetIngestTable" => catalog_applications::handle_ingest_table(&ctx, request),

        // Inspection + lifecycle ops (Batch 6 implemented)
        "datasetSchema" => catalog_applications::handle_schema(&ctx, request),
        "datasetPreview" => catalog_applications::handle_preview(&ctx, request),
        "datasetRemove" => catalog_applications::handle_remove(&ctx, request),

        // Compilation ops (Batch 3 implemented)
        "datasetCompileGdslSource" => {
            compilation_applications::handle_compile_gdsl_source(&ctx, request)
        }
        "datasetMaterializeCompilation" => {
            compilation_applications::handle_materialize_compilation(&ctx, request)
        }

        // Evaluation + reporting ops (Batch 4 implemented)
        "datasetEvalFeature" => feature_applications::handle_eval_feature(&ctx, request),
        "datasetFeatureAttentionReport" => {
            feature_applications::handle_feature_attention_report(&ctx, request)
        }

        // Introspection ops (Batch 4 implemented)
        "datasetCapabilities" => introspection_applications::handle_capabilities(&ctx, request),

        // Scaffolded but deferred
        "datasetCompilePipeline" => err(
            &ctx.op,
            "UNSUPPORTED_OP",
            "Collections op is scaffolded but not yet implemented.",
        ),

        _ => err(&ctx.op, "UNSUPPORTED_OP", "Unsupported operation."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::applications::collections::applications::{
        catalog_applications, compilation_applications, feature_applications,
        introspection_applications,
    };
    use crate::applications::collections::loaders::PerUserDbDatasetCatalogService;
    use crate::applications::services::collections_context::CollectionsContext;
    use std::sync::Arc;

    fn tempdir() -> std::path::PathBuf {
        let base = std::env::temp_dir().join(format!(
            "gds-collections-test-{}-{}",
            std::process::id(),
            uuid_like()
        ));
        std::fs::create_dir_all(&base).unwrap();
        base
    }

    fn uuid_like() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        format!("{nanos}")
    }

    fn ctx_for(op: &str, svc: Arc<PerUserDbDatasetCatalogService>) -> CollectionsContext {
        CollectionsContext::for_test(op, "alice", "db1", svc)
    }

    #[test]
    fn unsupported_facade_op_returns_unsupported_op() {
        let request = serde_json::json!({
            "facade": "collections",
            "op": "datasetCompilePipeline",
            "user": { "username": "alice", "isAdmin": false },
            "databaseId": "db1"
        });
        let response = handle_collections(&request);
        assert_eq!(response.get("ok").and_then(|v| v.as_bool()), Some(false));
        assert_eq!(
            response
                .get("error")
                .and_then(|e| e.get("code"))
                .and_then(|v| v.as_str()),
            Some("UNSUPPORTED_OP")
        );
    }

    #[test]
    fn missing_envelope_fields_yield_invalid_request() {
        let request = serde_json::json!({
            "facade": "collections",
            "op": "datasetListCatalog"
        });
        let response = handle_collections(&request);
        assert_eq!(
            response
                .get("error")
                .and_then(|e| e.get("code"))
                .and_then(|v| v.as_str()),
            Some("INVALID_REQUEST")
        );
    }

    #[test]
    fn list_catalog_is_empty_for_fresh_root() {
        let svc = Arc::new(PerUserDbDatasetCatalogService::with_base_root(tempdir()));
        let ctx = ctx_for("datasetListCatalog", svc);
        let request = serde_json::json!({ "op": "datasetListCatalog" });
        let response = catalog_applications::handle_list_catalog(&ctx, &request);
        assert_eq!(response.get("ok").and_then(|v| v.as_bool()), Some(true));
        let entries = response
            .get("data")
            .and_then(|v| v.get("entries"))
            .and_then(|v| v.as_array())
            .unwrap();
        assert!(entries.is_empty());
    }

    #[test]
    fn register_table_path_then_list_and_load_roundtrip() {
        let root = tempdir();

        // Seed a tiny parquet file so datasetLoad has something to read.
        use polars::prelude::{DataFrame, NamedFrom, ParquetWriter, Series};
        let mut df = DataFrame::new_infer_height(vec![
            Series::new("id".into(), &[1_i64, 2, 3]).into(),
            Series::new("name".into(), &["a", "b", "c"]).into(),
        ])
        .unwrap();
        let data_path = root.join("alice/db1/widgets.parquet");
        std::fs::create_dir_all(data_path.parent().unwrap()).unwrap();
        let file = std::fs::File::create(&data_path).unwrap();
        ParquetWriter::new(file).finish(&mut df).unwrap();

        let svc = Arc::new(PerUserDbDatasetCatalogService::with_base_root(root));

        // register
        {
            let ctx = ctx_for("datasetRegisterTablePath", svc.clone());
            let request = serde_json::json!({
                "op": "datasetRegisterTablePath",
                "datasetName": "widgets",
                "dataPath": "widgets.parquet",
                "format": "parquet"
            });
            let response = catalog_applications::handle_register_table_path(&ctx, &request);
            assert_eq!(
                response.get("ok").and_then(|v| v.as_bool()),
                Some(true),
                "register response: {response}"
            );
        }

        // list
        {
            let ctx = ctx_for("datasetListCatalog", svc.clone());
            let request = serde_json::json!({ "op": "datasetListCatalog" });
            let response = catalog_applications::handle_list_catalog(&ctx, &request);
            let entries = response
                .get("data")
                .and_then(|v| v.get("entries"))
                .and_then(|v| v.as_array())
                .unwrap();
            assert_eq!(entries.len(), 1);
            assert_eq!(
                entries[0].get("name").and_then(|v| v.as_str()),
                Some("widgets")
            );
            assert_eq!(
                entries[0].get("columnCount").and_then(|v| v.as_u64()),
                Some(2)
            );
        }

        // schema should resolve from the catalog snapshot, not via a dataset fallback.
        {
            let ctx = ctx_for("datasetSchema", svc.clone());
            let request = serde_json::json!({
                "op": "datasetSchema",
                "datasetName": "widgets"
            });
            let response = catalog_applications::handle_schema(&ctx, &request);
            assert_eq!(
                response.get("ok").and_then(|v| v.as_bool()),
                Some(true),
                "schema response: {response}"
            );
            let data = response.get("data").unwrap();
            assert_eq!(data.get("source").and_then(|v| v.as_str()), Some("catalog"));
        }

        // load
        {
            let ctx = ctx_for("datasetLoad", svc);
            let request = serde_json::json!({
                "op": "datasetLoad",
                "datasetName": "widgets"
            });
            let response = catalog_applications::handle_load(&ctx, &request);
            assert_eq!(
                response.get("ok").and_then(|v| v.as_bool()),
                Some(true),
                "load response: {response}"
            );
            let data = response.get("data").unwrap();
            assert_eq!(data.get("rowCount").and_then(|v| v.as_u64()), Some(3));
            assert_eq!(data.get("columnCount").and_then(|v| v.as_u64()), Some(2));
        }
    }

    #[test]
    fn compile_then_materialize_roundtrip() {
        let svc = Arc::new(PerUserDbDatasetCatalogService::with_base_root(tempdir()));

        let gdsl_source = "module demo\nfeature hello : signal\n";

        // compile
        {
            let ctx = ctx_for("datasetCompileGdslSource", svc.clone());
            let request = serde_json::json!({
                "op": "datasetCompileGdslSource",
                "compilationName": "demo",
                "gdslSource": gdsl_source
            });
            let response = compilation_applications::handle_compile_gdsl_source(&ctx, &request);
            assert_eq!(
                response.get("ok").and_then(|v| v.as_bool()),
                Some(true),
                "compile response: {response}"
            );
            let node_count = response
                .get("data")
                .and_then(|v| v.get("nodeCount"))
                .and_then(|v| v.as_u64())
                .unwrap();
            assert!(node_count >= 1);
        }

        // materialize
        {
            let ctx = ctx_for("datasetMaterializeCompilation", svc);
            let request = serde_json::json!({
                "op": "datasetMaterializeCompilation",
                "baseName": "demo-base",
                "compilationName": "demo"
            });
            let response = compilation_applications::handle_materialize_compilation(&ctx, &request);
            assert_eq!(
                response.get("ok").and_then(|v| v.as_bool()),
                Some(true),
                "materialize response: {response}"
            );
            let data = response.get("data").unwrap();
            assert_eq!(
                data.get("artifactsDataset").and_then(|v| v.as_str()),
                Some("demo-base.artifacts")
            );
        }
    }

    /// Shared setup: build an isolated service with a cataloged `widgets`
    /// parquet so feature/capability ops can run end-to-end.
    fn register_widgets() -> (Arc<PerUserDbDatasetCatalogService>, std::path::PathBuf) {
        let root = tempdir();

        use polars::prelude::{DataFrame, NamedFrom, ParquetWriter, Series};
        let mut df = DataFrame::new_infer_height(vec![
            Series::new("id".into(), &[1_i64, 2, 3, 4, 5]).into(),
            Series::new("name".into(), &["a", "b", "c", "d", "e"]).into(),
        ])
        .unwrap();
        let data_path = root.join("alice/db1/widgets.parquet");
        std::fs::create_dir_all(data_path.parent().unwrap()).unwrap();
        let file = std::fs::File::create(&data_path).unwrap();
        ParquetWriter::new(file).finish(&mut df).unwrap();

        let svc = Arc::new(PerUserDbDatasetCatalogService::with_base_root(root.clone()));

        let ctx = ctx_for("datasetRegisterTablePath", svc.clone());
        let request = serde_json::json!({
            "op": "datasetRegisterTablePath",
            "datasetName": "widgets",
            "dataPath": "widgets.parquet",
            "format": "parquet"
        });
        let response = catalog_applications::handle_register_table_path(&ctx, &request);
        assert_eq!(response.get("ok").and_then(|v| v.as_bool()), Some(true));

        (svc, root)
    }

    #[test]
    fn eval_feature_identity_returns_summary() {
        let (svc, _root) = register_widgets();

        let ctx = ctx_for("datasetEvalFeature", svc);
        let request = serde_json::json!({
            "op": "datasetEvalFeature",
            "datasetName": "widgets",
            "featureSpec": { "kind": "identity" },
            "evalMode": "fit",
            "runId": "run-1"
        });
        let response = feature_applications::handle_eval_feature(&ctx, &request);
        assert_eq!(
            response.get("ok").and_then(|v| v.as_bool()),
            Some(true),
            "eval response: {response}"
        );
        let data = response.get("data").unwrap();
        assert_eq!(data.get("evalMode").and_then(|v| v.as_str()), Some("fit"));
        assert_eq!(data.get("runId").and_then(|v| v.as_str()), Some("run-1"));
        let produced = data.get("producedDataset").unwrap();
        assert_eq!(produced.get("rowCount").and_then(|v| v.as_u64()), Some(5));
        assert_eq!(
            produced.get("columnCount").and_then(|v| v.as_u64()),
            Some(2)
        );
    }

    #[test]
    fn eval_feature_preview_caps_rows() {
        let (svc, _root) = register_widgets();

        let ctx = ctx_for("datasetEvalFeature", svc);
        let request = serde_json::json!({
            "op": "datasetEvalFeature",
            "datasetName": "widgets",
            "featureSpec": { "kind": "identity" },
            "evalMode": "preview",
            "previewRows": 2
        });
        let response = feature_applications::handle_eval_feature(&ctx, &request);
        let data = response.get("data").unwrap();
        assert_eq!(
            data.get("producedDataset")
                .and_then(|v| v.get("rowCount"))
                .and_then(|v| v.as_u64()),
            Some(2)
        );
    }

    #[test]
    fn eval_feature_rejects_unknown_kind() {
        let (svc, _root) = register_widgets();

        let ctx = ctx_for("datasetEvalFeature", svc);
        let request = serde_json::json!({
            "op": "datasetEvalFeature",
            "datasetName": "widgets",
            "featureSpec": { "kind": "bogus" }
        });
        let response = feature_applications::handle_eval_feature(&ctx, &request);
        assert_eq!(
            response
                .get("error")
                .and_then(|e| e.get("code"))
                .and_then(|v| v.as_str()),
            Some("INVALID_REQUEST")
        );
    }

    #[test]
    fn feature_attention_report_identity_returns_report() {
        let (svc, _root) = register_widgets();

        let ctx = ctx_for("datasetFeatureAttentionReport", svc);
        let request = serde_json::json!({
            "op": "datasetFeatureAttentionReport",
            "datasetName": "widgets",
            "featureSpec": { "kind": "identity" },
            "evalMode": "preview"
        });
        let response = feature_applications::handle_feature_attention_report(&ctx, &request);
        assert_eq!(
            response.get("ok").and_then(|v| v.as_bool()),
            Some(true),
            "attention response: {response}"
        );
        let report = response
            .get("data")
            .and_then(|v| v.get("attentionReport"))
            .unwrap();
        assert!(report.get("source").is_some());
        assert!(report.get("steps").is_some());
    }

    #[test]
    fn capabilities_reports_backend_and_format() {
        let (svc, _root) = register_widgets();

        let ctx = ctx_for("datasetCapabilities", svc);
        let request = serde_json::json!({
            "op": "datasetCapabilities",
            "datasetName": "widgets"
        });
        let response = introspection_applications::handle_capabilities(&ctx, &request);
        assert_eq!(
            response.get("ok").and_then(|v| v.as_bool()),
            Some(true),
            "capabilities response: {response}"
        );
        let data = response.get("data").unwrap();
        assert_eq!(
            data.get("datasetName").and_then(|v| v.as_str()),
            Some("widgets")
        );
        assert_eq!(data.get("format").and_then(|v| v.as_str()), Some("Parquet"));
        assert_eq!(data.get("backend").and_then(|v| v.as_str()), Some("Arrow"));
    }

    #[test]
    fn capabilities_not_found_for_missing_dataset() {
        let (svc, _root) = register_widgets();

        let ctx = ctx_for("datasetCapabilities", svc);
        let request = serde_json::json!({
            "op": "datasetCapabilities",
            "datasetName": "missing"
        });
        let response = introspection_applications::handle_capabilities(&ctx, &request);
        assert_eq!(
            response
                .get("error")
                .and_then(|e| e.get("code"))
                .and_then(|v| v.as_str()),
            Some("NOT_FOUND")
        );
    }

    #[test]
    fn ingest_table_then_load_roundtrip() {
        let svc = Arc::new(PerUserDbDatasetCatalogService::with_base_root(tempdir()));

        let ctx = ctx_for("datasetIngestTable", svc.clone());
        let request = serde_json::json!({
            "op": "datasetIngestTable",
            "datasetName": "inline",
            "format": "parquet",
            "table": [
                { "id": 1, "name": "a" },
                { "id": 2, "name": "b" },
                { "id": 3, "name": "c" }
            ]
        });
        let response = catalog_applications::handle_ingest_table(&ctx, &request);
        assert_eq!(
            response.get("ok").and_then(|v| v.as_bool()),
            Some(true),
            "ingest response: {response}"
        );

        // Load it back and verify row/column shape.
        let ctx = ctx_for("datasetLoad", svc.clone());
        let request = serde_json::json!({
            "op": "datasetLoad",
            "datasetName": "inline"
        });
        let response = catalog_applications::handle_load(&ctx, &request);
        let data = response.get("data").unwrap();
        assert_eq!(data.get("rowCount").and_then(|v| v.as_u64()), Some(3));
        assert_eq!(data.get("columnCount").and_then(|v| v.as_u64()), Some(2));

        // Evaluate an identity feature over the freshly ingested dataset.
        let ctx = ctx_for("datasetEvalFeature", svc);
        let request = serde_json::json!({
            "op": "datasetEvalFeature",
            "datasetName": "inline",
            "featureSpec": { "kind": "identity" },
            "evalMode": "fit"
        });
        let response = feature_applications::handle_eval_feature(&ctx, &request);
        assert_eq!(
            response.get("ok").and_then(|v| v.as_bool()),
            Some(true),
            "eval response: {response}"
        );
        let produced = response
            .get("data")
            .and_then(|v| v.get("producedDataset"))
            .unwrap();
        assert_eq!(produced.get("rowCount").and_then(|v| v.as_u64()), Some(3));
    }

    #[test]
    fn ingest_table_rejects_empty_payload() {
        let svc = Arc::new(PerUserDbDatasetCatalogService::with_base_root(tempdir()));

        let ctx = ctx_for("datasetIngestTable", svc);
        let request = serde_json::json!({
            "op": "datasetIngestTable",
            "datasetName": "inline",
            "format": "parquet",
            "table": []
        });
        let response = catalog_applications::handle_ingest_table(&ctx, &request);
        assert_eq!(
            response
                .get("error")
                .and_then(|e| e.get("code"))
                .and_then(|v| v.as_str()),
            Some("INVALID_REQUEST")
        );
    }

    /// Shared helper for Batch 6 ops: ingest a small inline table so there's a
    /// cataloged dataset to inspect/remove.
    fn ingest_inline(svc: &Arc<PerUserDbDatasetCatalogService>, name: &str) {
        let ctx = ctx_for("datasetIngestTable", svc.clone());
        let request = serde_json::json!({
            "op": "datasetIngestTable",
            "datasetName": name,
            "format": "parquet",
            "table": [
                { "id": 1, "name": "a" },
                { "id": 2, "name": "b" },
                { "id": 3, "name": "c" }
            ]
        });
        let response = catalog_applications::handle_ingest_table(&ctx, &request);
        assert_eq!(
            response.get("ok").and_then(|v| v.as_bool()),
            Some(true),
            "ingest response: {response}"
        );
    }

    #[test]
    fn schema_returns_fields_from_catalog() {
        let svc = Arc::new(PerUserDbDatasetCatalogService::with_base_root(tempdir()));
        ingest_inline(&svc, "widgets");

        let ctx = ctx_for("datasetSchema", svc);
        let request = serde_json::json!({
            "op": "datasetSchema",
            "datasetName": "widgets"
        });
        let response = catalog_applications::handle_schema(&ctx, &request);
        assert_eq!(
            response.get("ok").and_then(|v| v.as_bool()),
            Some(true),
            "schema response: {response}"
        );
        let data = response.get("data").unwrap();
        assert_eq!(data.get("source").and_then(|v| v.as_str()), Some("catalog"));
        let fields = data.get("fields").and_then(|v| v.as_array()).unwrap();
        assert_eq!(fields.len(), 2);
        let names: Vec<&str> = fields
            .iter()
            .filter_map(|f| f.get("name").and_then(|v| v.as_str()))
            .collect();
        assert!(names.contains(&"id"));
        assert!(names.contains(&"name"));
    }

    #[test]
    fn schema_not_found_for_missing_dataset() {
        let svc = Arc::new(PerUserDbDatasetCatalogService::with_base_root(tempdir()));

        let ctx = ctx_for("datasetSchema", svc);
        let request = serde_json::json!({
            "op": "datasetSchema",
            "datasetName": "ghost"
        });
        let response = catalog_applications::handle_schema(&ctx, &request);
        assert_eq!(
            response
                .get("error")
                .and_then(|e| e.get("code"))
                .and_then(|v| v.as_str()),
            Some("NOT_FOUND")
        );
    }

    #[test]
    fn preview_returns_rows_capped_by_limit() {
        let svc = Arc::new(PerUserDbDatasetCatalogService::with_base_root(tempdir()));
        ingest_inline(&svc, "widgets");

        let ctx = ctx_for("datasetPreview", svc);
        let request = serde_json::json!({
            "op": "datasetPreview",
            "datasetName": "widgets",
            "limit": 2
        });
        let response = catalog_applications::handle_preview(&ctx, &request);
        assert_eq!(
            response.get("ok").and_then(|v| v.as_bool()),
            Some(true),
            "preview response: {response}"
        );
        let data = response.get("data").unwrap();
        assert_eq!(data.get("rowCount").and_then(|v| v.as_u64()), Some(3));
        assert_eq!(data.get("columnCount").and_then(|v| v.as_u64()), Some(2));
        assert_eq!(data.get("limit").and_then(|v| v.as_u64()), Some(2));
        let rows = data.get("rows").and_then(|v| v.as_array()).unwrap();
        assert_eq!(rows.len(), 2);
        // First row should carry both columns.
        let first = rows[0].as_object().unwrap();
        assert!(first.contains_key("id"));
        assert!(first.contains_key("name"));
    }

    #[test]
    fn remove_then_list_no_longer_sees_entry() {
        let svc = Arc::new(PerUserDbDatasetCatalogService::with_base_root(tempdir()));
        ingest_inline(&svc, "widgets");

        // remove
        let ctx = ctx_for("datasetRemove", svc.clone());
        let request = serde_json::json!({
            "op": "datasetRemove",
            "datasetName": "widgets"
        });
        let response = catalog_applications::handle_remove(&ctx, &request);
        assert_eq!(
            response.get("ok").and_then(|v| v.as_bool()),
            Some(true),
            "remove response: {response}"
        );
        assert_eq!(
            response
                .get("data")
                .and_then(|v| v.get("removed"))
                .and_then(|v| v.as_bool()),
            Some(true)
        );

        // list should now be empty
        let ctx = ctx_for("datasetListCatalog", svc);
        let request = serde_json::json!({ "op": "datasetListCatalog" });
        let response = catalog_applications::handle_list_catalog(&ctx, &request);
        let entries = response
            .get("data")
            .and_then(|v| v.get("entries"))
            .and_then(|v| v.as_array())
            .unwrap();
        assert!(entries.is_empty(), "entries should be empty: {entries:?}");
    }

    #[test]
    fn remove_missing_dataset_returns_not_found() {
        let svc = Arc::new(PerUserDbDatasetCatalogService::with_base_root(tempdir()));

        let ctx = ctx_for("datasetRemove", svc);
        let request = serde_json::json!({
            "op": "datasetRemove",
            "datasetName": "ghost"
        });
        let response = catalog_applications::handle_remove(&ctx, &request);
        assert_eq!(
            response
                .get("error")
                .and_then(|e| e.get("code"))
                .and_then(|v| v.as_str()),
            Some("NOT_FOUND")
        );
    }
}
