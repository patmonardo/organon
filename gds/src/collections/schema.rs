//! Schema utilities aligned with py-polars schema helpers.

use std::collections::HashSet;
use std::sync::Arc;

use arrow::datatypes::ArrowSchema;
use polars::prelude::{Column, DataFrame, DataType, PolarsError, Schema, SchemaRef};

use crate::collections::dataframe::GDSDataFrame;

/// Convert an Arrow schema to a Polars schema.
pub fn arrow_schema_to_polars_schema(schema: &ArrowSchema) -> SchemaRef {
    let mut polars_schema = Schema::with_capacity(schema.len());
    for (name, field) in schema.clone().into_iter() {
        let dtype = DataType::from_arrow_field(&field);
        polars_schema.insert(name, dtype);
    }
    Arc::new(polars_schema)
}

/// Get column names from a Polars schema.
pub fn schema_names(schema: &Schema) -> Vec<String> {
    schema
        .clone()
        .into_iter()
        .map(|(name, _)| name.to_string())
        .collect()
}

/// Get column data types from a Polars schema.
pub fn schema_dtypes(schema: &Schema) -> Vec<DataType> {
    schema.clone().into_iter().map(|(_, dtype)| dtype).collect()
}

/// Create an empty DataFrame collection from a Polars schema.
pub fn schema_to_dataframe(schema: &Schema) -> GDSDataFrame {
    GDSDataFrame::empty_with_schema(schema)
}

/// Get a schema reference from a Polars DataFrame.
pub fn schema_from_dataframe(df: &DataFrame) -> SchemaRef {
    df.schema().clone()
}

/// Merge two schemas, preserving the order of the base schema and appending new fields.
pub fn merge_schema(
    base: &Schema,
    overlay: &Schema,
    prefer_overlay: bool,
) -> Result<Schema, PolarsError> {
    merge_schema_with_options(
        base,
        overlay,
        MergeSchemaOptions {
            prefer_overlay,
            overlay_order: false,
            strict_types: false,
        },
    )
}

#[derive(Clone, Copy, Debug)]
pub struct MergeSchemaOptions {
    /// If true, overlay dtypes replace base dtypes when fields overlap.
    pub prefer_overlay: bool,
    /// If true, output schema order follows the overlay schema first.
    pub overlay_order: bool,
    /// If true, overlapping fields must have identical dtypes.
    pub strict_types: bool,
}

/// Merge two schemas with explicit ordering and strictness controls.
pub fn merge_schema_with_options(
    base: &Schema,
    overlay: &Schema,
    options: MergeSchemaOptions,
) -> Result<Schema, PolarsError> {
    if options.overlay_order {
        merge_schema_overlay_order(base, overlay, options)
    } else {
        merge_schema_base_order(base, overlay, options)
    }
}

fn merge_schema_base_order(
    base: &Schema,
    overlay: &Schema,
    options: MergeSchemaOptions,
) -> Result<Schema, PolarsError> {
    let mut merged = Schema::with_capacity(base.len() + overlay.len());
    for (name, dtype) in base.clone().into_iter() {
        if let Some(overlay_dtype) = overlay.get(name.as_str()) {
            if options.strict_types && overlay_dtype != &dtype {
                return Err(PolarsError::ComputeError(
                    format!(
                        "Schema merge type mismatch for '{name}': base {:?}, overlay {:?}",
                        dtype, overlay_dtype
                    )
                    .into(),
                ));
            }
            if options.prefer_overlay {
                merged.insert(name, overlay_dtype.clone());
                continue;
            }
        }
        merged.insert(name, dtype);
    }

    for (name, dtype) in overlay.clone().into_iter() {
        if !merged.contains(name.as_str()) {
            merged.insert(name, dtype);
        }
    }

    Ok(merged)
}

fn merge_schema_overlay_order(
    base: &Schema,
    overlay: &Schema,
    options: MergeSchemaOptions,
) -> Result<Schema, PolarsError> {
    let mut merged = Schema::with_capacity(base.len() + overlay.len());
    for (name, dtype) in overlay.clone().into_iter() {
        if let Some(base_dtype) = base.get(name.as_str()) {
            if options.strict_types && base_dtype != &dtype {
                return Err(PolarsError::ComputeError(
                    format!(
                        "Schema merge type mismatch for '{name}': base {:?}, overlay {:?}",
                        base_dtype, dtype
                    )
                    .into(),
                ));
            }
            if !options.prefer_overlay {
                merged.insert(name, base_dtype.clone());
                continue;
            }
        }
        merged.insert(name, dtype);
    }

    for (name, dtype) in base.clone().into_iter() {
        if !merged.contains(name.as_str()) {
            merged.insert(name, dtype);
        }
    }

    Ok(merged)
}

/// Align a DataFrame to a schema by casting, adding missing columns, and ordering fields.
pub fn align_dataframe_to_schema(
    df: &DataFrame,
    schema: &Schema,
    allow_cast: bool,
    drop_extra: bool,
) -> Result<DataFrame, PolarsError> {
    let mut columns = Vec::with_capacity(schema.len());
    let height = df.height();
    for (name, dtype) in schema.clone().into_iter() {
        match df.column(name.as_str()) {
            Ok(column) => {
                let next = if allow_cast && column.dtype() != &dtype {
                    column.cast(&dtype)?
                } else if !allow_cast && column.dtype() != &dtype {
                    return Err(PolarsError::ComputeError(
                        format!(
                            "Column '{name}' has dtype {:?}, expected {:?}",
                            column.dtype(),
                            dtype
                        )
                        .into(),
                    ));
                } else {
                    column.clone()
                };
                columns.push(next);
            }
            Err(_) => {
                columns.push(Column::full_null(name.clone(), height, &dtype));
            }
        }
    }

    if !drop_extra {
        let mut known = HashSet::with_capacity(schema.len());
        for (name, _) in schema.clone().into_iter() {
            known.insert(name);
        }
        for column in df.get_columns() {
            if !known.contains(column.name()) {
                columns.push(column.clone());
            }
        }
    }

    DataFrame::new(columns)
}
