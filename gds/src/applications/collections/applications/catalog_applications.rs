//! Catalog application handlers for the Dataset-first Collections facade.
//!
//! Thin shims that parse a TS-JSON request into a typed config, delegate to
//! the shared `DatasetCatalog` handle resolved by the dispatcher context, and
//! wrap the typed result into the standard ok/op/data envelope.

use serde_json::{json, Value};

use crate::applications::collections::configs::{
    DatasetIngestTableConfig, DatasetListCatalogConfig, DatasetLoadConfig, DatasetPreviewConfig,
    DatasetRegisterTablePathConfig, DatasetRemoveConfig, DatasetSchemaConfig,
};
use crate::applications::collections::results::{
    CatalogEntrySummary, CatalogListResult, DatasetSummary, PreviewSummary, RemoveSummary,
    SchemaFieldSummary, SchemaSummary,
};
use crate::applications::services::collections_context::CollectionsContext;
use crate::collections::catalog::{CatalogError, CollectionsIoFormat};
use crate::collections::dataframe::GDSDataFrame;
use polars_io::json::{JsonFormat, JsonReader, JsonWriter};
use polars_io::prelude::{SerReader, SerWriter};

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
        CatalogError::AlreadyExists(_) => "CATALOG_ERROR",
        _ => "CATALOG_ERROR",
    };
    err(op, code, e.to_string())
}

fn parse_io_format(op: &str, raw: Option<&str>) -> Result<Option<CollectionsIoFormat>, Value> {
    let Some(value) = raw else { return Ok(None) };
    let normalized = value.trim().to_ascii_lowercase();
    let format = match normalized.as_str() {
        "" | "auto" => CollectionsIoFormat::Auto,
        "parquet" => CollectionsIoFormat::Parquet,
        "ipc" | "arrow" | "arrow-ipc" | "arrowipc" => CollectionsIoFormat::ArrowIpc,
        "csv" => CollectionsIoFormat::Csv,
        "json" => CollectionsIoFormat::Json,
        "database" | "db" => CollectionsIoFormat::Database,
        other => {
            return Err(err(
                op,
                "INVALID_REQUEST",
                format!("Unsupported format '{other}'"),
            ));
        }
    };
    Ok(Some(format))
}

fn entry_to_summary(
    entry: &crate::collections::catalog::CollectionsCatalogDiskEntry,
) -> CatalogEntrySummary {
    CatalogEntrySummary {
        name: entry.name.clone(),
        data_path: entry.data_path.clone(),
        format: format!("{:?}", entry.io_policy.format),
        backend: format!("{:?}", entry.backend),
        column_count: entry.schema.as_ref().map(|s| s.fields.len()),
    }
}

/// `datasetListCatalog` handler.
pub(crate) fn handle_list_catalog(ctx: &CollectionsContext, request: &Value) -> Value {
    let op = &ctx.op;
    let cfg: DatasetListCatalogConfig = match parse_config(op, request) {
        Ok(v) => v,
        Err(e) => return e,
    };

    let catalog_handle = match ctx.dataset_catalog() {
        Ok(v) => v,
        Err(e) => return catalog_error(op, e),
    };
    let catalog = catalog_handle
        .read()
        .expect("dataset catalog read lock poisoned");

    let filter = cfg.name_filter.as_deref().map(str::to_ascii_lowercase);

    let entries = catalog
        .catalog()
        .list()
        .iter()
        .filter(|entry| match &filter {
            Some(substr) => entry.name.to_ascii_lowercase().contains(substr),
            None => true,
        })
        .map(entry_to_summary)
        .collect::<Vec<_>>();

    let result = CatalogListResult { entries };
    match serde_json::to_value(result) {
        Ok(data) => ok(op, data),
        Err(e) => err(op, "SERIALIZATION_ERROR", e.to_string()),
    }
}

/// `datasetRegisterTablePath` handler.
pub(crate) fn handle_register_table_path(ctx: &CollectionsContext, request: &Value) -> Value {
    let op = &ctx.op;
    let cfg: DatasetRegisterTablePathConfig = match parse_config(op, request) {
        Ok(v) => v,
        Err(e) => return e,
    };

    if cfg.dataset_name.trim().is_empty() {
        return err(op, "INVALID_REQUEST", "datasetName missing or empty");
    }
    if cfg.data_path.trim().is_empty() {
        return err(op, "INVALID_REQUEST", "dataPath missing or empty");
    }

    let format = match parse_io_format(op, cfg.format.as_deref()) {
        Ok(v) => v,
        Err(e) => return e,
    };

    let catalog_handle = match ctx.dataset_catalog() {
        Ok(v) => v,
        Err(e) => return catalog_error(op, e),
    };
    let mut catalog = catalog_handle
        .write()
        .expect("dataset catalog write lock poisoned");

    let entry = match catalog.register_table_path(&cfg.dataset_name, &cfg.data_path, format, None) {
        Ok(e) => e,
        Err(e) => return catalog_error(op, e),
    };
    if let Err(e) = catalog.save() {
        return catalog_error(op, e);
    }

    let summary = entry_to_summary(&entry);
    match serde_json::to_value(summary) {
        Ok(data) => ok(op, data),
        Err(e) => err(op, "SERIALIZATION_ERROR", e.to_string()),
    }
}

/// `datasetLoad` handler.
pub(crate) fn handle_load(ctx: &CollectionsContext, request: &Value) -> Value {
    let op = &ctx.op;
    let cfg: DatasetLoadConfig = match parse_config(op, request) {
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

    let dataset = match catalog.load_table(&cfg.dataset_name) {
        Ok(d) => d,
        Err(e) => return catalog_error(op, e),
    };

    let summary = DatasetSummary {
        name: cfg.dataset_name,
        row_count: Some(dataset.row_count() as u64),
        column_count: Some(dataset.column_count()),
    };
    match serde_json::to_value(summary) {
        Ok(data) => ok(op, data),
        Err(e) => err(op, "SERIALIZATION_ERROR", e.to_string()),
    }
}

/// Materialize an inline JSON `table` payload (array of row objects) into a
/// Polars-backed `GDSDataFrame` using the shared JSON reader.
fn materialize_inline_table(op: &str, table: &Value) -> Result<GDSDataFrame, Value> {
    if table.is_null() {
        return Err(err(op, "INVALID_REQUEST", "table payload missing"));
    }
    let rows = table.as_array().ok_or_else(|| {
        err(
            op,
            "INVALID_REQUEST",
            "table must be a JSON array of row objects",
        )
    })?;
    if rows.is_empty() {
        return Err(err(op, "INVALID_REQUEST", "table payload is empty"));
    }
    for (idx, row) in rows.iter().enumerate() {
        if !row.is_object() {
            return Err(err(
                op,
                "INVALID_REQUEST",
                format!("table[{idx}] must be a JSON object"),
            ));
        }
    }

    let bytes =
        serde_json::to_vec(table).map_err(|e| err(op, "SERIALIZATION_ERROR", e.to_string()))?;
    let cursor = std::io::Cursor::new(bytes);
    let df = JsonReader::new(cursor)
        .with_json_format(JsonFormat::Json)
        .finish()
        .map_err(|e| err(op, "INVALID_REQUEST", format!("failed to parse table: {e}")))?;
    Ok(GDSDataFrame::from(df))
}

/// `datasetIngestTable` handler.
///
/// Accepts an inline JSON array of row objects under `table`, materializes it
/// into a `GDSDataFrame`, and persists it under the catalog root using the
/// kernel's `DatasetCatalog::ingest_table`. Returns the resulting catalog
/// entry summary.
pub(crate) fn handle_ingest_table(ctx: &CollectionsContext, request: &Value) -> Value {
    let op = &ctx.op;
    let cfg: DatasetIngestTableConfig = match parse_config(op, request) {
        Ok(v) => v,
        Err(e) => return e,
    };

    if cfg.dataset_name.trim().is_empty() {
        return err(op, "INVALID_REQUEST", "datasetName missing or empty");
    }

    let format = match parse_io_format(op, Some(cfg.format.as_str())) {
        Ok(Some(f)) => match f {
            CollectionsIoFormat::Auto => {
                return err(
                    op,
                    "INVALID_REQUEST",
                    "format must be an explicit backend (parquet, ipc, csv, json); 'auto' is not supported for ingest",
                );
            }
            other => other,
        },
        Ok(None) => {
            return err(op, "INVALID_REQUEST", "format missing or empty");
        }
        Err(e) => return e,
    };

    let table = match materialize_inline_table(op, &cfg.table) {
        Ok(t) => t,
        Err(e) => return e,
    };

    let catalog_handle = match ctx.dataset_catalog() {
        Ok(v) => v,
        Err(e) => return catalog_error(op, e),
    };
    let mut catalog = catalog_handle
        .write()
        .expect("dataset catalog write lock poisoned");

    let entry = match catalog.ingest_table(&cfg.dataset_name, &table, format) {
        Ok(e) => e,
        Err(e) => return catalog_error(op, e),
    };
    if let Err(e) = catalog.save() {
        return catalog_error(op, e);
    }

    let summary = entry_to_summary(&entry);
    match serde_json::to_value(summary) {
        Ok(data) => ok(op, data),
        Err(e) => err(op, "SERIALIZATION_ERROR", e.to_string()),
    }
}

/// `datasetSchema` handler.
///
/// Prefers the catalog manifest's persisted schema (no I/O) and falls back to
/// lazily loading the table when the manifest lacks a recorded schema.
pub(crate) fn handle_schema(ctx: &CollectionsContext, request: &Value) -> Value {
    let op = &ctx.op;
    let cfg: DatasetSchemaConfig = match parse_config(op, request) {
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

    let resolved = match catalog.resolve_table_schema(&cfg.dataset_name) {
        Ok(v) => v,
        Err(e) => return catalog_error(op, e),
    };

    let fields = resolved
        .schema
        .fields
        .iter()
        .map(|f| SchemaFieldSummary {
            name: f.name.clone(),
            dtype: format!("{:?}", f.value_type),
            nullable: Some(f.nullable),
        })
        .collect::<Vec<_>>();
    let source = resolved.source.as_str().to_string();

    let summary = SchemaSummary {
        dataset_name: cfg.dataset_name,
        fields,
        source,
    };
    match serde_json::to_value(summary) {
        Ok(data) => ok(op, data),
        Err(e) => err(op, "SERIALIZATION_ERROR", e.to_string()),
    }
}

/// `datasetPreview` handler.
///
/// Returns the top `limit` rows of a cataloged dataset as a JSON array of row
/// objects. Uses the polars `JsonWriter` in array mode, then round-trips the
/// bytes through `serde_json` so the rows appear as native JSON inside the
/// response envelope.
pub(crate) fn handle_preview(ctx: &CollectionsContext, request: &Value) -> Value {
    let op = &ctx.op;
    let cfg: DatasetPreviewConfig = match parse_config(op, request) {
        Ok(v) => v,
        Err(e) => return e,
    };

    if cfg.dataset_name.trim().is_empty() {
        return err(op, "INVALID_REQUEST", "datasetName missing or empty");
    }

    const DEFAULT_LIMIT: usize = 10;
    const MAX_LIMIT: usize = 1000;
    let limit = cfg.limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT);

    let catalog_handle = match ctx.dataset_catalog() {
        Ok(v) => v,
        Err(e) => return catalog_error(op, e),
    };
    let catalog = catalog_handle
        .read()
        .expect("dataset catalog read lock poisoned");

    let dataset = match catalog.load_table(&cfg.dataset_name) {
        Ok(d) => d,
        Err(e) => return catalog_error(op, e),
    };

    let total_rows = dataset.row_count();
    let total_cols = dataset.column_count();
    let head = dataset.head(limit);

    let mut buf: Vec<u8> = Vec::new();
    let mut df = head.table().dataframe().clone();
    if let Err(e) = JsonWriter::new(&mut buf)
        .with_json_format(JsonFormat::Json)
        .finish(&mut df)
    {
        return err(op, "SERIALIZATION_ERROR", e.to_string());
    }
    let rows: Value = match serde_json::from_slice(&buf) {
        Ok(v) => v,
        Err(e) => return err(op, "SERIALIZATION_ERROR", e.to_string()),
    };

    let summary = PreviewSummary {
        dataset_name: cfg.dataset_name,
        row_count: total_rows as u64,
        column_count: total_cols,
        limit,
        rows,
    };
    match serde_json::to_value(summary) {
        Ok(data) => ok(op, data),
        Err(e) => err(op, "SERIALIZATION_ERROR", e.to_string()),
    }
}

/// `datasetRemove` handler.
///
/// Unregisters a catalog entry by name. Returns `{ removed: true }` when the
/// entry existed; returns `NOT_FOUND` otherwise. The underlying data file is
/// intentionally left on disk — this op is a manifest-level unregister only.
pub(crate) fn handle_remove(ctx: &CollectionsContext, request: &Value) -> Value {
    let op = &ctx.op;
    let cfg: DatasetRemoveConfig = match parse_config(op, request) {
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
    let mut catalog = catalog_handle
        .write()
        .expect("dataset catalog write lock poisoned");

    if let Err(e) = catalog.remove_entry(&cfg.dataset_name) {
        return catalog_error(op, e);
    }
    if let Err(e) = catalog.save() {
        return catalog_error(op, e);
    }

    let summary = RemoveSummary {
        dataset_name: cfg.dataset_name,
        removed: true,
    };
    match serde_json::to_value(summary) {
        Ok(data) => ok(op, data),
        Err(e) => err(op, "SERIALIZATION_ERROR", e.to_string()),
    }
}
