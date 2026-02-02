//! IPC (Arrow) IO for Polars-backed tables.

use std::fs::File;
use std::path::Path;

use polars::prelude::PlSmallStr;
use polars::prelude::{
    col, IdxSize, IpcWriter, LazyFrame, PlPath, PolarsError, SchemaRef, SerWriter,
};
use polars_io::ipc::IpcScanOptions;
use polars_io::{HiveOptions, RowIndex};
use polars_plan::prelude::UnifiedScanArgs;

use crate::collections::dataframe::collection::PolarsDataFrameCollection;
use crate::collections::io::partition::{partition_dataframe_for_write, PartitionByConfig};

/// Read an IPC file into a Polars-backed table.
pub fn read_table(path: &Path) -> Result<PolarsDataFrameCollection, PolarsError> {
    read_table_with_options(path, IpcScanConfig::default())
}

#[derive(Clone)]
pub struct IpcScanConfig {
    pub ipc_options: IpcScanOptions,
    pub n_rows: Option<usize>,
    pub rechunk: bool,
    pub cache: bool,
    pub include_file_paths: Option<String>,
    pub hive_enabled: Option<bool>,
    pub hive_start_idx: usize,
    pub hive_schema: Option<SchemaRef>,
    pub hive_try_parse_dates: bool,
    pub row_index_name: Option<String>,
    pub row_index_offset: usize,
    pub columns: Option<Vec<String>>,
}

impl Default for IpcScanConfig {
    fn default() -> Self {
        Self {
            ipc_options: IpcScanOptions::default(),
            n_rows: None,
            rechunk: false,
            cache: true,
            include_file_paths: None,
            hive_enabled: None,
            hive_start_idx: 0,
            hive_schema: None,
            hive_try_parse_dates: true,
            row_index_name: None,
            row_index_offset: 0,
            columns: None,
        }
    }
}

/// Read an IPC file into a Polars-backed table with scan options.
pub fn read_table_with_options(
    path: &Path,
    config: IpcScanConfig,
) -> Result<PolarsDataFrameCollection, PolarsError> {
    let path = PlPath::new(path.to_string_lossy().as_ref());
    let df = scan_table_with_options(path, config)?.collect()?;
    Ok(PolarsDataFrameCollection::from(df))
}

/// Write a Polars-backed table to IPC.
pub fn write_table(path: &Path, table: &PolarsDataFrameCollection) -> Result<(), PolarsError> {
    let mut df = table.dataframe().clone();
    let mut file = File::create(path)?;
    IpcWriter::new(&mut file).finish(&mut df)?;
    Ok(())
}

/// Write a Polars-backed table to partitioned IPC files.
pub fn write_table_partitioned(
    table: &PolarsDataFrameCollection,
    config: PartitionByConfig,
) -> Result<Vec<std::path::PathBuf>, PolarsError> {
    let chunks = partition_dataframe_for_write(table.dataframe(), &config, "ipc")?;
    let mut written = Vec::with_capacity(chunks.len());
    for chunk in chunks {
        let mut frame = chunk.frame;
        let mut file = File::create(&chunk.path)?;
        IpcWriter::new(&mut file).finish(&mut frame)?;
        written.push(chunk.path);
    }
    Ok(written)
}

/// Scan an IPC file into a LazyFrame.
pub fn scan_table(path: PlPath) -> Result<LazyFrame, PolarsError> {
    scan_table_with_options(path, IpcScanConfig::default())
}

/// Scan an IPC file into a LazyFrame with scan options.
pub fn scan_table_with_options(
    path: PlPath,
    config: IpcScanConfig,
) -> Result<LazyFrame, PolarsError> {
    let mut unified_scan_args = UnifiedScanArgs::default();
    unified_scan_args.rechunk = config.rechunk;
    unified_scan_args.cache = config.cache;
    unified_scan_args.include_file_paths = config
        .include_file_paths
        .as_ref()
        .map(|name| PlSmallStr::from(name.as_str()));
    if config.hive_enabled.is_some()
        || config.hive_start_idx != 0
        || config.hive_schema.is_some()
        || !config.hive_try_parse_dates
    {
        unified_scan_args.hive_options = HiveOptions {
            enabled: config.hive_enabled,
            hive_start_idx: config.hive_start_idx,
            schema: config.hive_schema,
            try_parse_dates: config.hive_try_parse_dates,
        };
    }
    if let Some(name) = config.row_index_name.as_ref() {
        unified_scan_args.row_index = Some(RowIndex {
            name: name.as_str().into(),
            offset: config.row_index_offset as _,
        });
    }
    let mut lf = LazyFrame::scan_ipc(path, config.ipc_options, unified_scan_args)?;
    if let Some(n_rows) = config.n_rows {
        lf = lf.slice(0, n_rows as IdxSize);
    }
    if let Some(columns) = config.columns {
        let exprs = columns.iter().map(|name| col(name)).collect::<Vec<_>>();
        lf = lf.select(exprs);
    }
    Ok(lf)
}
