//! Parquet IO for Polars-backed tables.

use std::fs::File;
use std::path::{Path, PathBuf};

use arrow::datatypes::ArrowSchema;
use polars::prelude::PlSmallStr;
use polars::prelude::{
    col, LazyFrame, ParquetWriter, PlPath, PolarsError, ScanArgsParquet, SchemaRef,
};
use polars_io::parquet::read::ParallelStrategy;
use polars_io::{HiveOptions, RowIndex};
use polars_parquet::arrow::read::schema::{
    infer_schema, infer_schema_with_options, SchemaInferenceOptions,
};
use polars_parquet::arrow::read::{read_metadata as read_parquet_metadata, FileMetadata};
use polars_parquet::parquet::compression::Compression;
use polars_parquet::parquet::schema::types::PhysicalType;
use polars_parquet::parquet::statistics::Statistics;

use crate::collections::dataframe::collection::PolarsDataFrameCollection;
use crate::collections::io::partition::{partition_dataframe_for_write, PartitionByConfig};
use crate::collections::schema::arrow_schema_to_polars_schema;

/// Read a Parquet file into a Polars-backed table.
pub fn read_table(path: &Path) -> Result<PolarsDataFrameCollection, PolarsError> {
    read_table_with_options(path, ParquetScanConfig::default())
}

/// Read Parquet file metadata.
pub fn read_metadata(path: &Path) -> Result<FileMetadata, PolarsError> {
    let mut file = File::open(path)?;
    read_parquet_metadata(&mut file)
}

/// Read Parquet key-value metadata pairs.
pub fn read_key_value_metadata(path: &Path) -> Result<Vec<(String, Option<String>)>, PolarsError> {
    let metadata = read_metadata(path)?;
    let mut pairs = Vec::new();
    if let Some(entries) = metadata.key_value_metadata() {
        for entry in entries {
            pairs.push((entry.key.clone(), entry.value.clone()));
        }
    }
    Ok(pairs)
}

/// Infer the Arrow schema from Parquet metadata.
pub fn read_arrow_schema(path: &Path) -> Result<ArrowSchema, PolarsError> {
    let metadata = read_metadata(path)?;
    infer_schema(&metadata)
}

/// Infer the Arrow schema from Parquet metadata with options.
pub fn read_arrow_schema_with_options(
    path: &Path,
    options: &Option<SchemaInferenceOptions>,
) -> Result<ArrowSchema, PolarsError> {
    let metadata = read_metadata(path)?;
    infer_schema_with_options(&metadata, options)
}

/// Infer a Polars schema from Parquet metadata.
pub fn read_polars_schema(path: &Path) -> Result<SchemaRef, PolarsError> {
    let arrow_schema = read_arrow_schema(path)?;
    Ok(arrow_schema_to_polars_schema(&arrow_schema))
}

/// Infer a Polars schema from Parquet metadata with options.
pub fn read_polars_schema_with_options(
    path: &Path,
    options: &Option<SchemaInferenceOptions>,
) -> Result<SchemaRef, PolarsError> {
    let arrow_schema = read_arrow_schema_with_options(path, options)?;
    Ok(arrow_schema_to_polars_schema(&arrow_schema))
}

#[derive(Clone, Debug)]
pub struct ParquetMetadataSummary {
    pub created_by: Option<String>,
    pub num_rows: usize,
    pub num_row_groups: usize,
    pub max_row_group_height: usize,
    pub schema: ArrowSchema,
}

#[derive(Clone, Debug)]
pub struct ParquetMetadataDetails {
    pub summary: ParquetMetadataSummary,
    pub key_value_metadata: Vec<(String, Option<String>)>,
    pub row_groups: Vec<ParquetRowGroupMetadataSummary>,
}

#[derive(Clone, Debug)]
pub struct ParquetRowGroupMetadataSummary {
    pub num_rows: usize,
    pub total_byte_size: usize,
    pub compressed_size: usize,
    pub full_byte_range: std::ops::Range<u64>,
    pub sorting_columns: Option<Vec<String>>,
    pub columns: Vec<ParquetColumnChunkMetadataSummary>,
}

#[derive(Clone, Debug)]
pub struct ParquetColumnChunkMetadataSummary {
    pub path_in_schema: Vec<String>,
    pub physical_type: PhysicalType,
    pub compression: Compression,
    pub encodings: Vec<String>,
    pub file_path: Option<String>,
    pub file_offset: i64,
    pub num_values: i64,
    pub compressed_size: i64,
    pub uncompressed_size: i64,
    pub data_page_offset: i64,
    pub index_page_offset: Option<i64>,
    pub dictionary_page_offset: Option<i64>,
    pub byte_range: std::ops::Range<u64>,
    pub statistics: Option<ParquetColumnStatisticsSummary>,
}

#[derive(Clone, Debug)]
pub struct ParquetColumnChunkMetadataFlat {
    pub row_group_index: usize,
    pub column_index: usize,
    pub column: ParquetColumnChunkMetadataSummary,
}

#[derive(Clone, Debug)]
pub struct ParquetColumnStatisticsSummary {
    pub physical_type: PhysicalType,
    pub null_count: Option<i64>,
    pub distinct_count: Option<i64>,
    pub min: Option<String>,
    pub max: Option<String>,
}

#[derive(Clone, Debug)]
pub struct ParquetSchemaFieldMetadata {
    pub name: String,
    pub metadata: Vec<(String, String)>,
}

/// Read a high-level summary of Parquet metadata, including Arrow schema inference.
pub fn read_metadata_summary(path: &Path) -> Result<ParquetMetadataSummary, PolarsError> {
    let metadata = read_metadata(path)?;
    let schema = infer_schema(&metadata)?;
    Ok(ParquetMetadataSummary {
        created_by: metadata.created_by.clone(),
        num_rows: metadata.num_rows,
        num_row_groups: metadata.row_groups.len(),
        max_row_group_height: metadata.max_row_group_height,
        schema,
    })
}

/// Read a full metadata bundle, including key-value pairs and row groups.
pub fn read_metadata_details(path: &Path) -> Result<ParquetMetadataDetails, PolarsError> {
    Ok(ParquetMetadataDetails {
        summary: read_metadata_summary(path)?,
        key_value_metadata: read_key_value_metadata(path)?,
        row_groups: read_row_group_metadata(path)?,
    })
}

/// Read flattened column chunk metadata across all row groups.
pub fn read_column_chunk_metadata(
    path: &Path,
) -> Result<Vec<ParquetColumnChunkMetadataFlat>, PolarsError> {
    let metadata = read_metadata(path)?;
    let mut flat = Vec::new();
    for (row_group_index, row_group) in metadata.row_groups.iter().enumerate() {
        for (column_index, column) in row_group.parquet_columns().iter().enumerate() {
            let summary = summarize_column_chunk(column)?;
            flat.push(ParquetColumnChunkMetadataFlat {
                row_group_index,
                column_index,
                column: summary,
            });
        }
    }
    Ok(flat)
}

/// Read row group metadata summaries, including column chunk details and statistics.
pub fn read_row_group_metadata(
    path: &Path,
) -> Result<Vec<ParquetRowGroupMetadataSummary>, PolarsError> {
    let metadata = read_metadata(path)?;
    metadata
        .row_groups
        .iter()
        .map(|row_group| {
            let columns = row_group
                .parquet_columns()
                .iter()
                .map(|column| summarize_column_chunk(column))
                .collect::<Result<Vec<_>, PolarsError>>()?;
            Ok(ParquetRowGroupMetadataSummary {
                num_rows: row_group.num_rows(),
                total_byte_size: row_group.total_byte_size(),
                compressed_size: row_group.compressed_size(),
                full_byte_range: row_group.full_byte_range(),
                sorting_columns: row_group
                    .sorting_columns()
                    .map(|columns| columns.iter().map(|column| format!("{column:?}")).collect()),
                columns,
            })
        })
        .collect()
}

/// Read schema field metadata from the Arrow schema.
pub fn read_schema_field_metadata(
    path: &Path,
) -> Result<Vec<ParquetSchemaFieldMetadata>, PolarsError> {
    let schema = read_arrow_schema(path)?;
    Ok(schema_field_metadata(&schema))
}

fn summarize_column_chunk(
    column: &polars_parquet::parquet::metadata::ColumnChunkMetadata,
) -> Result<ParquetColumnChunkMetadataSummary, PolarsError> {
    let path_in_schema = column
        .descriptor()
        .path_in_schema
        .iter()
        .map(|part| part.to_string())
        .collect::<Vec<_>>();
    let statistics = match column.statistics() {
        Some(Ok(stats)) => Some(summarize_statistics(stats)),
        Some(Err(err)) => {
            return Err(PolarsError::ComputeError(
                format!("Failed to decode parquet statistics: {err}").into(),
            ));
        }
        None => None,
    };
    Ok(ParquetColumnChunkMetadataSummary {
        path_in_schema,
        physical_type: column.physical_type(),
        compression: column.compression(),
        encodings: column
            .column_encoding()
            .iter()
            .map(|encoding| format!("{encoding:?}"))
            .collect(),
        file_path: column.file_path().clone(),
        file_offset: column.file_offset(),
        num_values: column.num_values(),
        compressed_size: column.compressed_size(),
        uncompressed_size: column.uncompressed_size(),
        data_page_offset: column.data_page_offset(),
        index_page_offset: column.index_page_offset(),
        dictionary_page_offset: column.dictionary_page_offset(),
        byte_range: column.byte_range(),
        statistics,
    })
}

fn summarize_statistics(stats: Statistics) -> ParquetColumnStatisticsSummary {
    match stats {
        Statistics::Binary(s) => ParquetColumnStatisticsSummary {
            physical_type: s.primitive_type.physical_type,
            null_count: s.null_count,
            distinct_count: s.distinct_count,
            min: s.min_value.map(|value| format!("{value:?}")),
            max: s.max_value.map(|value| format!("{value:?}")),
        },
        Statistics::Boolean(s) => ParquetColumnStatisticsSummary {
            physical_type: PhysicalType::Boolean,
            null_count: s.null_count,
            distinct_count: s.distinct_count,
            min: s.min_value.map(|value| format!("{value}")),
            max: s.max_value.map(|value| format!("{value}")),
        },
        Statistics::FixedLen(s) => ParquetColumnStatisticsSummary {
            physical_type: s.primitive_type.physical_type,
            null_count: s.null_count,
            distinct_count: s.distinct_count,
            min: s.min_value.map(|value| format!("{value:?}")),
            max: s.max_value.map(|value| format!("{value:?}")),
        },
        Statistics::Int32(s) => ParquetColumnStatisticsSummary {
            physical_type: s.primitive_type.physical_type,
            null_count: s.null_count,
            distinct_count: s.distinct_count,
            min: s.min_value.map(|value| format!("{value}")),
            max: s.max_value.map(|value| format!("{value}")),
        },
        Statistics::Int64(s) => ParquetColumnStatisticsSummary {
            physical_type: s.primitive_type.physical_type,
            null_count: s.null_count,
            distinct_count: s.distinct_count,
            min: s.min_value.map(|value| format!("{value}")),
            max: s.max_value.map(|value| format!("{value}")),
        },
        Statistics::Int96(s) => ParquetColumnStatisticsSummary {
            physical_type: s.primitive_type.physical_type,
            null_count: s.null_count,
            distinct_count: s.distinct_count,
            min: s.min_value.map(|value| format!("{value:?}")),
            max: s.max_value.map(|value| format!("{value:?}")),
        },
        Statistics::Float(s) => ParquetColumnStatisticsSummary {
            physical_type: s.primitive_type.physical_type,
            null_count: s.null_count,
            distinct_count: s.distinct_count,
            min: s.min_value.map(|value| format!("{value}")),
            max: s.max_value.map(|value| format!("{value}")),
        },
        Statistics::Double(s) => ParquetColumnStatisticsSummary {
            physical_type: s.primitive_type.physical_type,
            null_count: s.null_count,
            distinct_count: s.distinct_count,
            min: s.min_value.map(|value| format!("{value}")),
            max: s.max_value.map(|value| format!("{value}")),
        },
    }
}

fn schema_field_metadata(schema: &ArrowSchema) -> Vec<ParquetSchemaFieldMetadata> {
    schema
        .clone()
        .into_iter()
        .map(|(name, field)| {
            let metadata = field
                .metadata
                .as_ref()
                .map(|metadata| {
                    metadata
                        .iter()
                        .map(|(key, value)| (key.to_string(), value.to_string()))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            ParquetSchemaFieldMetadata {
                name: name.to_string(),
                metadata,
            }
        })
        .collect()
}

#[derive(Clone)]
pub struct ParquetScanConfig {
    pub scan_args: ScanArgsParquet,
    pub schema: Option<SchemaRef>,
    pub n_rows: Option<usize>,
    pub parallel: ParallelStrategy,
    pub use_statistics: bool,
    pub low_memory: bool,
    pub rechunk: bool,
    pub cache: bool,
    pub glob: bool,
    pub include_file_paths: Option<String>,
    pub hive_enabled: Option<bool>,
    pub hive_start_idx: usize,
    pub hive_schema: Option<SchemaRef>,
    pub hive_try_parse_dates: bool,
    pub allow_missing_columns: bool,
    pub row_index_name: Option<String>,
    pub row_index_offset: usize,
    pub columns: Option<Vec<String>>,
}

impl Default for ParquetScanConfig {
    fn default() -> Self {
        Self {
            scan_args: ScanArgsParquet::default(),
            schema: None,
            n_rows: None,
            parallel: ParallelStrategy::Auto,
            use_statistics: true,
            low_memory: false,
            rechunk: false,
            cache: true,
            glob: true,
            include_file_paths: None,
            hive_enabled: None,
            hive_start_idx: 0,
            hive_schema: None,
            hive_try_parse_dates: true,
            allow_missing_columns: false,
            row_index_name: None,
            row_index_offset: 0,
            columns: None,
        }
    }
}

/// Read a Parquet file into a Polars-backed table with scan options.
pub fn read_table_with_options(
    path: &Path,
    config: ParquetScanConfig,
) -> Result<PolarsDataFrameCollection, PolarsError> {
    let path = PlPath::new(path.to_string_lossy().as_ref());
    let df = scan_table_with_options(path, config)?.collect()?;
    Ok(PolarsDataFrameCollection::from(df))
}

/// Write a Polars-backed table to Parquet.
pub fn write_table(path: &Path, table: &PolarsDataFrameCollection) -> Result<(), PolarsError> {
    let mut df = table.dataframe().clone();
    let mut file = File::create(path)?;
    ParquetWriter::new(&mut file).finish(&mut df)?;
    Ok(())
}

/// Write a Polars-backed table to partitioned Parquet files.
pub fn write_table_partitioned(
    table: &PolarsDataFrameCollection,
    config: PartitionByConfig,
) -> Result<Vec<PathBuf>, PolarsError> {
    let chunks = partition_dataframe_for_write(table.dataframe(), &config, "parquet")?;
    let mut written = Vec::with_capacity(chunks.len());
    for chunk in chunks {
        let mut frame = chunk.frame;
        let mut file = File::create(&chunk.path)?;
        ParquetWriter::new(&mut file).finish(&mut frame)?;
        written.push(chunk.path);
    }
    Ok(written)
}

/// Scan a Parquet file into a LazyFrame.
pub fn scan_table(path: PlPath) -> Result<LazyFrame, PolarsError> {
    scan_table_with_options(path, ParquetScanConfig::default())
}

/// Scan a Parquet file into a LazyFrame with scan options.
pub fn scan_table_with_options(
    path: PlPath,
    config: ParquetScanConfig,
) -> Result<LazyFrame, PolarsError> {
    let mut scan_args = config.scan_args;
    if let Some(schema) = config.schema {
        scan_args.schema = Some(schema);
    }
    scan_args.n_rows = config.n_rows;
    scan_args.parallel = config.parallel;
    scan_args.use_statistics = config.use_statistics;
    scan_args.low_memory = config.low_memory;
    scan_args.rechunk = config.rechunk;
    scan_args.cache = config.cache;
    scan_args.glob = config.glob;
    scan_args.include_file_paths = config
        .include_file_paths
        .as_ref()
        .map(|name| PlSmallStr::from(name.as_str()));
    if config.hive_enabled.is_some()
        || config.hive_start_idx != 0
        || config.hive_schema.is_some()
        || !config.hive_try_parse_dates
    {
        scan_args.hive_options = HiveOptions {
            enabled: config.hive_enabled,
            hive_start_idx: config.hive_start_idx,
            schema: config.hive_schema,
            try_parse_dates: config.hive_try_parse_dates,
        };
    }
    scan_args.allow_missing_columns = config.allow_missing_columns;
    if let Some(name) = config.row_index_name.as_ref() {
        scan_args.row_index = Some(RowIndex {
            name: name.as_str().into(),
            offset: config.row_index_offset as _,
        });
    }
    let mut lf = LazyFrame::scan_parquet(path, scan_args)?;
    if let Some(columns) = config.columns {
        let exprs = columns.iter().map(|name| col(name)).collect::<Vec<_>>();
        lf = lf.select(exprs);
    }
    Ok(lf)
}
