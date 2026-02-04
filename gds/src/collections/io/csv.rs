//! CSV IO for Polars-backed tables.

use std::fs::File;
use std::path::Path;

use polars::prelude::{
    col, CsvWriter, LazyCsvReader, LazyFileListReader, LazyFrame, PlPath, PlSmallStr, PolarsError,
    SchemaRef, SerWriter,
};
use polars_io::csv::read::{CsvEncoding, NullValues};
use polars_io::RowIndex;

use crate::collections::dataframe::GDSDataFrame;
use crate::collections::io::partition::{partition_dataframe_for_write, PartitionByConfig};

#[derive(Debug, Clone, Copy)]
pub struct CsvWriteConfig {
    pub include_header: bool,
}

impl Default for CsvWriteConfig {
    fn default() -> Self {
        Self {
            include_header: true,
        }
    }
}

/// Read a CSV file into a Polars-backed table.
pub fn read_table(path: &Path) -> Result<GDSDataFrame, PolarsError> {
    read_table_with_options(path, CsvScanConfig::default())
}

#[derive(Debug, Clone)]
pub struct CsvScanConfig {
    pub has_header: bool,
    pub separator: Option<u8>,
    pub quote_char: Option<u8>,
    pub disable_quotes: bool,
    pub comment_prefix: Option<String>,
    pub n_rows: Option<usize>,
    pub skip_rows: usize,
    pub skip_lines: usize,
    pub skip_rows_after_header: usize,
    pub infer_schema_length: Option<usize>,
    pub ignore_errors: bool,
    pub schema: Option<SchemaRef>,
    pub schema_overwrite: Option<SchemaRef>,
    pub null_values: Option<NullValues>,
    pub missing_is_null: bool,
    pub encoding: Option<CsvEncoding>,
    pub try_parse_dates: bool,
    pub decimal_comma: bool,
    pub eol_char: Option<u8>,
    pub truncate_ragged_lines: bool,
    pub low_memory: bool,
    pub cache: bool,
    pub include_file_paths: Option<String>,
    pub row_index_name: Option<String>,
    pub row_index_offset: usize,
    pub columns: Option<Vec<String>>,
}

impl Default for CsvScanConfig {
    fn default() -> Self {
        Self {
            has_header: true,
            separator: None,
            quote_char: None,
            disable_quotes: false,
            comment_prefix: None,
            n_rows: None,
            skip_rows: 0,
            skip_lines: 0,
            skip_rows_after_header: 0,
            infer_schema_length: None,
            ignore_errors: false,
            schema: None,
            schema_overwrite: None,
            null_values: None,
            missing_is_null: false,
            encoding: None,
            try_parse_dates: false,
            decimal_comma: false,
            eol_char: None,
            truncate_ragged_lines: false,
            low_memory: false,
            cache: true,
            include_file_paths: None,
            row_index_name: None,
            row_index_offset: 0,
            columns: None,
        }
    }
}

/// Read a CSV file into a Polars-backed table with scan options.
pub fn read_table_with_options(
    path: &Path,
    config: CsvScanConfig,
) -> Result<GDSDataFrame, PolarsError> {
    let path = PlPath::new(path.to_string_lossy().as_ref());
    let df = scan_table_with_options(path, config)?.collect()?;
    Ok(GDSDataFrame::from(df))
}

/// Write a Polars-backed table to CSV.
pub fn write_table(
    path: &Path,
    table: &GDSDataFrame,
    config: CsvWriteConfig,
) -> Result<(), PolarsError> {
    let mut df = table.dataframe().clone();
    let mut file = File::create(path)?;
    CsvWriter::new(&mut file)
        .include_header(config.include_header)
        .finish(&mut df)?;
    Ok(())
}

/// Write a Polars-backed table to partitioned CSV files.
pub fn write_table_partitioned(
    table: &GDSDataFrame,
    config: PartitionByConfig,
    write_config: CsvWriteConfig,
) -> Result<Vec<std::path::PathBuf>, PolarsError> {
    let chunks = partition_dataframe_for_write(table.dataframe(), &config, "csv")?;
    let mut written = Vec::with_capacity(chunks.len());
    for chunk in chunks {
        let mut frame = chunk.frame;
        let mut file = File::create(&chunk.path)?;
        CsvWriter::new(&mut file)
            .include_header(write_config.include_header)
            .finish(&mut frame)?;
        written.push(chunk.path);
    }
    Ok(written)
}

/// Scan a CSV file into a LazyFrame.
pub fn scan_table(path: PlPath) -> Result<LazyFrame, PolarsError> {
    scan_table_with_options(path, CsvScanConfig::default())
}

/// Scan a CSV file into a LazyFrame with scan options.
pub fn scan_table_with_options(
    path: PlPath,
    config: CsvScanConfig,
) -> Result<LazyFrame, PolarsError> {
    let row_index = config.row_index_name.as_ref().map(|name| RowIndex {
        name: name.as_str().into(),
        offset: config.row_index_offset as _,
    });
    let mut reader = LazyCsvReader::new(path)
        .with_has_header(config.has_header)
        .with_skip_rows_after_header(config.skip_rows_after_header)
        .with_skip_rows(config.skip_rows)
        .with_skip_lines(config.skip_lines)
        .with_n_rows(config.n_rows)
        .with_infer_schema_length(config.infer_schema_length)
        .with_ignore_errors(config.ignore_errors)
        .with_schema(config.schema)
        .with_dtype_overwrite(config.schema_overwrite)
        .with_null_values(config.null_values.clone())
        .with_missing_is_null(config.missing_is_null)
        .with_low_memory(config.low_memory)
        .with_cache(config.cache)
        .with_include_file_paths(
            config
                .include_file_paths
                .as_ref()
                .map(|name| PlSmallStr::from(name.as_str())),
        )
        .with_row_index(row_index);
    if let Some(separator) = config.separator {
        reader = reader.with_separator(separator);
    }
    if config.disable_quotes {
        reader = reader.with_quote_char(None);
    } else if config.quote_char.is_some() {
        reader = reader.with_quote_char(config.quote_char);
    }
    if let Some(comment_prefix) = config.comment_prefix.as_ref() {
        reader = reader.with_comment_prefix(Some(PlSmallStr::from(comment_prefix.as_str())));
    }
    if let Some(encoding) = config.encoding {
        reader = reader.with_encoding(encoding);
    }
    if config.try_parse_dates {
        reader = reader.with_try_parse_dates(true);
    }
    if config.decimal_comma {
        reader = reader.with_decimal_comma(true);
    }
    if let Some(eol_char) = config.eol_char {
        reader = reader.with_eol_char(eol_char);
    }
    if config.truncate_ragged_lines {
        reader = reader.with_truncate_ragged_lines(true);
    }
    let mut lf = reader.finish()?;
    if let Some(columns) = config.columns {
        let exprs = columns.iter().map(|name| col(name)).collect::<Vec<_>>();
        lf = lf.select(exprs);
    }
    Ok(lf)
}
