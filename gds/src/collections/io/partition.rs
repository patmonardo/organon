//! Partitioned write helpers for IO modules.

use std::fs;
use std::path::PathBuf;

use polars::prelude::{AnyValue, DataFrame, PolarsError};

#[derive(Clone, Debug)]
pub struct PartitionByConfig {
    pub base_path: PathBuf,
    pub key: Vec<String>,
    pub include_key: bool,
    pub max_rows_per_file: Option<usize>,
    pub approximate_bytes_per_file: Option<u64>,
}

#[derive(Clone, Debug)]
pub struct PartitionChunk {
    pub path: PathBuf,
    pub frame: DataFrame,
}

/// Build partitioned file chunks for writing.
pub fn partition_dataframe_for_write(
    df: &DataFrame,
    config: &PartitionByConfig,
    extension: &str,
) -> Result<Vec<PartitionChunk>, PolarsError> {
    if config.key.is_empty() {
        return Err(PolarsError::ComputeError(
            "PartitionByConfig.key must contain at least one column".into(),
        ));
    }

    let key_columns = config.key.clone();
    let partitions = df.partition_by(key_columns.as_slice(), true)?;

    let mut chunks = Vec::new();
    for (partition_index, partition) in partitions.into_iter().enumerate() {
        let mut dir = config.base_path.clone();
        for key in &config.key {
            let value = partition
                .column(key)?
                .get(0)
                .map_err(|err| PolarsError::ComputeError(format!("{err}").into()))?;
            dir.push(format!(
                "{}={}",
                key,
                sanitize_path_component(&any_value_to_string(value))
            ));
        }
        fs::create_dir_all(&dir)?;

        let partition_df = if config.include_key {
            partition
        } else {
            partition.drop_many(config.key.iter().map(|name| name.as_str()))
        };

        let total_rows = partition_df.height();
        let chunk_size = chunk_size_for_partition(&partition_df, config, total_rows);
        let mut offset = 0usize;
        let mut chunk_index = 0usize;
        while offset < total_rows {
            let len = (total_rows - offset).min(chunk_size);
            let chunk = partition_df.slice(offset as i64, len);
            let file_name = format!(
                "part-{:05}-{:05}.{}",
                partition_index, chunk_index, extension
            );
            let path = dir.join(file_name);
            chunks.push(PartitionChunk { path, frame: chunk });
            offset += len;
            chunk_index += 1;
        }
    }

    Ok(chunks)
}

fn chunk_size_for_partition(
    partition_df: &DataFrame,
    config: &PartitionByConfig,
    total_rows: usize,
) -> usize {
    let base = config.max_rows_per_file.unwrap_or(total_rows.max(1));
    let Some(target_bytes) = config.approximate_bytes_per_file else {
        return base;
    };
    if total_rows == 0 {
        return base.max(1);
    }

    let estimated_size = partition_df.estimated_size() as u64;
    let bytes_per_row = (estimated_size / total_rows as u64).max(1);
    let rows_by_bytes = (target_bytes / bytes_per_row).max(1) as usize;

    match config.max_rows_per_file {
        Some(max_rows) => max_rows.min(rows_by_bytes),
        None => rows_by_bytes,
    }
}

fn any_value_to_string(value: AnyValue) -> String {
    match value {
        AnyValue::Null => "null".to_string(),
        AnyValue::String(value) => value.to_string(),
        AnyValue::StringOwned(value) => value.to_string(),
        AnyValue::Binary(value) => format!("{value:?}"),
        AnyValue::BinaryOwned(value) => format!("{value:?}"),
        value => format!("{value}"),
    }
}

fn sanitize_path_component(value: &str) -> String {
    value.replace('/', "_").replace('\\', "_").replace(':', "_")
}
