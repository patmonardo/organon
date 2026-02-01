//! Table builders for Polars-backed DataFrames.

use polars::prelude::{Column, DataFrame, NamedFrom, Series};

use crate::collections::dataframe::collection::PolarsDataFrameCollection;
use crate::collections::dataframe::column::{
    column_binary, column_binary_opt, column_bool, column_bool_opt, column_bytes, column_bytes_opt,
    column_f32, column_f32_opt, column_f64, column_f64_opt, column_i128, column_i128_opt,
    column_i16, column_i16_opt, column_i32, column_i32_opt, column_i64, column_i64_opt, column_i8,
    column_i8_opt, column_str, column_str_opt, column_string, column_string_opt, column_u16,
    column_u16_opt, column_u32, column_u32_opt, column_u64, column_u64_opt, column_u8,
    column_u8_opt,
};

/// Builder for creating a DataFrame-backed table without exposing Polars types.
#[derive(Debug, Default)]
pub struct TableBuilder {
    columns: Vec<Column>,
}

impl TableBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_i64_column(mut self, name: &str, values: &[i64]) -> Self {
        self.columns.push(column_i64(name, values));
        self
    }

    pub fn with_i64_opt_column(mut self, name: &str, values: &[Option<i64>]) -> Self {
        self.columns.push(column_i64_opt(name, values));
        self
    }

    pub fn with_i128_column(mut self, name: &str, values: &[i128]) -> Self {
        self.columns.push(column_i128(name, values));
        self
    }

    pub fn with_i128_opt_column(mut self, name: &str, values: &[Option<i128>]) -> Self {
        self.columns.push(column_i128_opt(name, values));
        self
    }

    pub fn with_i32_column(mut self, name: &str, values: &[i32]) -> Self {
        self.columns.push(column_i32(name, values));
        self
    }

    pub fn with_i32_opt_column(mut self, name: &str, values: &[Option<i32>]) -> Self {
        self.columns.push(column_i32_opt(name, values));
        self
    }

    pub fn with_i16_column(mut self, name: &str, values: &[i16]) -> Self {
        self.columns.push(column_i16(name, values));
        self
    }

    pub fn with_i16_opt_column(mut self, name: &str, values: &[Option<i16>]) -> Self {
        self.columns.push(column_i16_opt(name, values));
        self
    }

    pub fn with_i8_column(mut self, name: &str, values: &[i8]) -> Self {
        self.columns.push(column_i8(name, values));
        self
    }

    pub fn with_i8_opt_column(mut self, name: &str, values: &[Option<i8>]) -> Self {
        self.columns.push(column_i8_opt(name, values));
        self
    }

    pub fn with_u64_column(mut self, name: &str, values: &[u64]) -> Self {
        self.columns.push(column_u64(name, values));
        self
    }

    pub fn with_u64_opt_column(mut self, name: &str, values: &[Option<u64>]) -> Self {
        self.columns.push(column_u64_opt(name, values));
        self
    }

    pub fn with_u32_column(mut self, name: &str, values: &[u32]) -> Self {
        self.columns.push(column_u32(name, values));
        self
    }

    pub fn with_u32_opt_column(mut self, name: &str, values: &[Option<u32>]) -> Self {
        self.columns.push(column_u32_opt(name, values));
        self
    }

    pub fn with_u16_column(mut self, name: &str, values: &[u16]) -> Self {
        self.columns.push(column_u16(name, values));
        self
    }

    pub fn with_u16_opt_column(mut self, name: &str, values: &[Option<u16>]) -> Self {
        self.columns.push(column_u16_opt(name, values));
        self
    }

    pub fn with_u8_column(mut self, name: &str, values: &[u8]) -> Self {
        self.columns.push(column_u8(name, values));
        self
    }

    pub fn with_u8_opt_column(mut self, name: &str, values: &[Option<u8>]) -> Self {
        self.columns.push(column_u8_opt(name, values));
        self
    }

    pub fn with_f64_column(mut self, name: &str, values: &[f64]) -> Self {
        self.columns.push(column_f64(name, values));
        self
    }

    pub fn with_f64_opt_column(mut self, name: &str, values: &[Option<f64>]) -> Self {
        self.columns.push(column_f64_opt(name, values));
        self
    }

    pub fn with_f32_column(mut self, name: &str, values: &[f32]) -> Self {
        self.columns.push(column_f32(name, values));
        self
    }

    pub fn with_f32_opt_column(mut self, name: &str, values: &[Option<f32>]) -> Self {
        self.columns.push(column_f32_opt(name, values));
        self
    }

    pub fn with_bool_column(mut self, name: &str, values: &[bool]) -> Self {
        self.columns.push(column_bool(name, values));
        self
    }

    pub fn with_bool_opt_column(mut self, name: &str, values: &[Option<bool>]) -> Self {
        self.columns.push(column_bool_opt(name, values));
        self
    }

    pub fn with_string_column(mut self, name: &str, values: &[String]) -> Self {
        self.columns.push(column_string(name, values));
        self
    }

    pub fn with_string_opt_column(mut self, name: &str, values: &[Option<String>]) -> Self {
        self.columns.push(column_string_opt(name, values));
        self
    }

    pub fn with_str_column(mut self, name: &str, values: &[&str]) -> Self {
        self.columns.push(column_str(name, values));
        self
    }

    pub fn with_str_opt_column(mut self, name: &str, values: &[Option<&str>]) -> Self {
        self.columns.push(column_str_opt(name, values));
        self
    }

    pub fn with_binary_column(mut self, name: &str, values: &[&[u8]]) -> Self {
        self.columns.push(column_binary(name, values));
        self
    }

    pub fn with_binary_opt_column(mut self, name: &str, values: &[Option<&[u8]>]) -> Self {
        self.columns.push(column_binary_opt(name, values));
        self
    }

    pub fn with_bytes_column(mut self, name: &str, values: &[Vec<u8>]) -> Self {
        self.columns.push(column_bytes(name, values));
        self
    }

    pub fn with_bytes_opt_column(mut self, name: &str, values: &[Option<Vec<u8>>]) -> Self {
        self.columns.push(column_bytes_opt(name, values));
        self
    }

    pub fn build(self) -> Result<PolarsDataFrameCollection, polars::error::PolarsError> {
        let df = DataFrame::new(self.columns)?;
        Ok(PolarsDataFrameCollection::from(df))
    }
}

/// Multiply an f64 column by a scalar factor.
pub fn scale_f64_column(
    table: &mut PolarsDataFrameCollection,
    column_name: &str,
    factor: f64,
) -> Result<(), polars::error::PolarsError> {
    let df = table.dataframe_mut();
    let column = df.column(column_name)?;
    let series = column.as_materialized_series();
    let scaled = series
        .f64()?
        .into_iter()
        .map(|value| value.map(|v| v * factor))
        .collect::<Vec<Option<f64>>>();
    let mut new_series = Series::new(column_name.into(), scaled);
    new_series.rename(column_name.into());
    df.replace(column_name, new_series)?;
    Ok(())
}
