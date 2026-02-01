//! JSON (NDJSON) IO for Polars-backed tables.

use std::fs::File;
use std::path::Path;

use std::num::NonZeroUsize;

use polars::prelude::{
    col, LazyFileListReader, LazyFrame, PlPath, PlSmallStr, PolarsError, Schema, SchemaRef,
};
use polars_io::json::{JsonFormat, JsonReader, JsonWriter};
use polars_io::prelude::{SerReader, SerWriter};
use polars_io::RowIndex;
use polars_lazy::prelude::LazyJsonLineReader;

use crate::collections::dataframe::collection::PolarsDataFrameCollection;

pub struct JsonWriteConfig {
    pub format: JsonFormat,
}

impl Default for JsonWriteConfig {
    fn default() -> Self {
        Self {
            format: JsonFormat::JsonLines,
        }
    }
}

pub struct JsonReadConfig {
    pub format: JsonFormat,
    pub schema: Option<SchemaRef>,
    pub schema_overwrite: Option<Schema>,
}

impl Default for JsonReadConfig {
    fn default() -> Self {
        Self {
            format: JsonFormat::JsonLines,
            schema: None,
            schema_overwrite: None,
        }
    }
}

/// Read a JSON file into a Polars-backed table.
pub fn read_table(
    path: &Path,
    config: JsonReadConfig,
) -> Result<PolarsDataFrameCollection, PolarsError> {
    let file = File::open(path)?;
    let mut reader = JsonReader::new(file).with_json_format(config.format);
    if let Some(schema) = config.schema {
        reader = reader.with_schema(schema);
    }
    if let Some(schema_overwrite) = config.schema_overwrite.as_ref() {
        reader = reader.with_schema_overwrite(schema_overwrite);
    }
    let df = reader.finish()?;
    Ok(PolarsDataFrameCollection::from(df))
}

/// Write a Polars-backed table to JSON.
pub fn write_table(
    path: &Path,
    table: &PolarsDataFrameCollection,
    config: JsonWriteConfig,
) -> Result<(), PolarsError> {
    let mut df = table.dataframe().clone();
    let file = File::create(path)?;
    JsonWriter::new(file)
        .with_json_format(config.format)
        .finish(&mut df)?;
    Ok(())
}

/// Scan a JSON Lines file into a LazyFrame.
pub fn scan_table(path: PlPath) -> Result<LazyFrame, PolarsError> {
    scan_table_with_options(path, JsonScanConfig::default())
}

#[derive(Debug, Clone)]
pub struct JsonScanConfig {
    pub n_rows: Option<usize>,
    pub ignore_errors: bool,
    pub infer_schema_length: Option<usize>,
    pub batch_size: Option<usize>,
    pub low_memory: bool,
    pub include_file_paths: Option<String>,
    pub schema: Option<SchemaRef>,
    pub schema_overwrite: Option<SchemaRef>,
    pub row_index_name: Option<String>,
    pub row_index_offset: usize,
    pub columns: Option<Vec<String>>,
}

impl Default for JsonScanConfig {
    fn default() -> Self {
        Self {
            n_rows: None,
            ignore_errors: false,
            infer_schema_length: None,
            batch_size: None,
            low_memory: false,
            include_file_paths: None,
            schema: None,
            schema_overwrite: None,
            row_index_name: None,
            row_index_offset: 0,
            columns: None,
        }
    }
}

/// Scan a JSON Lines file into a LazyFrame with scan options.
pub fn scan_table_with_options(
    path: PlPath,
    config: JsonScanConfig,
) -> Result<LazyFrame, PolarsError> {
    let infer_schema_length = config.infer_schema_length.and_then(NonZeroUsize::new);
    let row_index = config.row_index_name.as_ref().map(|name| RowIndex {
        name: name.as_str().into(),
        offset: config.row_index_offset as _,
    });
    let batch_size = config.batch_size.and_then(NonZeroUsize::new);
    let mut lf = LazyJsonLineReader::new(path)
        .with_n_rows(config.n_rows)
        .with_ignore_errors(config.ignore_errors)
        .with_infer_schema_length(infer_schema_length)
        .with_batch_size(batch_size)
        .low_memory(config.low_memory)
        .with_include_file_paths(
            config
                .include_file_paths
                .as_ref()
                .map(|name| PlSmallStr::from(name.as_str())),
        )
        .with_schema(config.schema)
        .with_schema_overwrite(config.schema_overwrite)
        .with_row_index(row_index)
        .finish()?;
    if let Some(columns) = config.columns {
        let exprs = columns.iter().map(|name| col(name)).collect::<Vec<_>>();
        lf = lf.select(exprs);
    }
    Ok(lf)
}
