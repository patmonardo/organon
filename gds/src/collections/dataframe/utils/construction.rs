//! Construction helpers for Polars-backed DataFrames (py-polars inspired).

use std::collections::{BTreeSet, HashMap};

use polars::error::PolarsError;
use polars::frame::row::Row;
use polars::prelude::{
    AnyValue, Column, DataFrame, DataType, Field, PolarsResult, Schema, SchemaExt, Series,
};

use crate::collections::dataframe::collection::PolarsDataFrameCollection;

pub type SchemaDefinition = Schema;

/// Orientation for input data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataOrientation {
    Row,
    Column,
    Infer,
}

/// Construction options mirroring py-polars behavior (subset).
#[derive(Debug, Clone)]
pub struct ConstructionOptions {
    pub schema: Option<SchemaDefinition>,
    pub schema_overrides: Option<SchemaDefinition>,
    pub strict: bool,
    pub orient: DataOrientation,
    pub nan_to_null: bool,
}

impl Default for ConstructionOptions {
    fn default() -> Self {
        Self {
            schema: None,
            schema_overrides: None,
            strict: true,
            orient: DataOrientation::Infer,
            nan_to_null: false,
        }
    }
}

pub fn dataframe_from_columns(
    data: &HashMap<String, Vec<AnyValue<'static>>>,
    options: ConstructionOptions,
) -> Result<PolarsDataFrameCollection, PolarsError> {
    let order = resolve_column_order(data.keys(), options.schema.as_ref());
    let mut columns = Vec::with_capacity(order.len());
    for name in order {
        let values = data.get(&name).cloned().unwrap_or_default();
        let values = normalize_values(values, options.nan_to_null);
        let series = Series::from_any_values(name.as_str().into(), &values, options.strict)?;
        columns.push(Column::from(series));
    }
    let df = DataFrame::new(columns)?;
    finalize_dataframe(df, options)
}

pub fn dataframe_from_rows(
    rows: &[Vec<AnyValue<'static>>],
    options: ConstructionOptions,
) -> Result<PolarsDataFrameCollection, PolarsError> {
    let mut row_values = Vec::with_capacity(rows.len());
    for row in rows {
        let normalized = normalize_values(row.clone(), options.nan_to_null);
        row_values.push(Row::new(normalized));
    }
    let df = if let Some(schema) = options.schema.as_ref() {
        DataFrame::from_rows_and_schema(&row_values, schema)?
    } else {
        DataFrame::from_rows(&row_values)?
    };
    finalize_dataframe(df, options)
}

pub fn dataframe_from_records(
    records: &[HashMap<String, AnyValue<'static>>],
    options: ConstructionOptions,
) -> Result<PolarsDataFrameCollection, PolarsError> {
    let mut key_set = BTreeSet::new();
    if let Some(schema) = options.schema.as_ref() {
        for name in schema.iter_names() {
            key_set.insert(name.to_string());
        }
    } else {
        for record in records {
            for key in record.keys() {
                key_set.insert(key.to_string());
            }
        }
    }
    let mut data: HashMap<String, Vec<AnyValue<'static>>> = HashMap::new();
    for key in key_set.iter() {
        data.insert(key.clone(), Vec::with_capacity(records.len()));
    }
    for record in records {
        for key in key_set.iter() {
            let value = record.get(key).cloned().unwrap_or(AnyValue::Null);
            data.get_mut(key).expect("column exists").push(value);
        }
    }
    dataframe_from_columns(&data, options)
}

pub fn dataframe_from_series(
    series: Vec<Series>,
    options: ConstructionOptions,
) -> Result<PolarsDataFrameCollection, PolarsError> {
    let columns: Vec<Column> = series.into_iter().map(Column::from).collect();
    dataframe_from_columns_vec(columns, options)
}

pub fn dataframe_from_columns_vec(
    columns: Vec<Column>,
    options: ConstructionOptions,
) -> Result<PolarsDataFrameCollection, PolarsError> {
    let df = DataFrame::new(columns)?;
    finalize_dataframe(df, options)
}

pub fn schema_from_pairs(pairs: &[(String, DataType)]) -> Schema {
    Schema::from_iter(
        pairs
            .iter()
            .map(|(name, dtype)| Field::new(name.into(), dtype.clone())),
    )
}

fn normalize_values(values: Vec<AnyValue<'static>>, nan_to_null: bool) -> Vec<AnyValue<'static>> {
    if !nan_to_null {
        return values;
    }
    values
        .into_iter()
        .map(|value| match value {
            AnyValue::Float32(val) if val.is_nan() => AnyValue::Null,
            AnyValue::Float64(val) if val.is_nan() => AnyValue::Null,
            _ => value,
        })
        .collect()
}

fn resolve_column_order(
    names: impl IntoIterator<Item = impl AsRef<str>>,
    schema: Option<&Schema>,
) -> Vec<String> {
    if let Some(schema) = schema {
        return schema.iter_names().map(|name| name.to_string()).collect();
    }
    let mut order: Vec<String> = names
        .into_iter()
        .map(|name| name.as_ref().to_string())
        .collect();
    order.sort();
    order
}

fn finalize_dataframe(
    df: DataFrame,
    options: ConstructionOptions,
) -> Result<PolarsDataFrameCollection, PolarsError> {
    let mut df = df;
    if let Some(schema) = options.schema.as_ref() {
        df = apply_schema(df, schema, options.strict)?;
    }
    if let Some(overrides) = options.schema_overrides.as_ref() {
        df = apply_schema_overrides(df, overrides, options.strict)?;
    }
    Ok(PolarsDataFrameCollection::new(df))
}

fn apply_schema(df: DataFrame, schema: &Schema, _strict: bool) -> PolarsResult<DataFrame> {
    let height = df.height();
    let mut columns = Vec::with_capacity(schema.len());
    for field in schema.iter_fields() {
        let name = field.name();
        let dtype = field.dtype();
        let series = match df.column(name) {
            Ok(column) => {
                let series = column
                    .as_series()
                    .ok_or_else(|| PolarsError::ComputeError("column is not a series".into()))?;
                if series.dtype() == dtype {
                    series.clone()
                } else {
                    series.cast(dtype)?
                }
            }
            Err(_) => Series::full_null(name.clone(), height, dtype),
        };
        columns.push(Column::from(series));
    }
    DataFrame::new(columns)
}

fn apply_schema_overrides(
    mut df: DataFrame,
    overrides: &Schema,
    _strict: bool,
) -> PolarsResult<DataFrame> {
    for field in overrides.iter_fields() {
        let name = field.name();
        let dtype = field.dtype();
        let column = df.column(name)?;
        let series = column
            .as_series()
            .ok_or_else(|| PolarsError::ComputeError("column is not a series".into()))?;
        let casted = if series.dtype() == dtype {
            series.clone()
        } else {
            series.cast(dtype)?
        };
        df.replace(name.as_str(), casted)?;
    }
    Ok(df)
}
