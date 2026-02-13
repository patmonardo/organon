//! Table builders for Polars-backed DataFrames.

use polars::prelude::{Column, DataFrame, NamedFrom, Series};

use crate::collections::dataframe::column::{
    column_binary, column_binary_opt, column_bool, column_bool_opt, column_bytes, column_bytes_opt,
    column_f32, column_f32_opt, column_f64, column_f64_opt, column_i128, column_i128_opt,
    column_i16, column_i16_opt, column_i32, column_i32_opt, column_i64, column_i64_opt, column_i8,
    column_i8_opt, column_str, column_str_opt, column_string, column_string_opt, column_u16,
    column_u16_opt, column_u32, column_u32_opt, column_u64, column_u64_opt, column_u8,
    column_u8_opt,
};
use crate::collections::dataframe::GDSDataFrame;

/// Builder for creating a DataFrame-backed table without exposing Polars types.
#[derive(Debug, Default)]
pub struct TableBuilder {
    columns: Vec<Column>,
}

macro_rules! table_builder_with_column {
    ($( $method:ident => $column_fn:ident : $ty:ty ),+ $(,)?) => {
        $(
            pub fn $method(mut self, name: &str, values: $ty) -> Self {
                self.columns.push($column_fn(name, values));
                self
            }
        )+
    };
}

impl TableBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    table_builder_with_column!(
        with_i64_column => column_i64: &[i64],
        with_i64_opt_column => column_i64_opt: &[Option<i64>],
        with_i128_column => column_i128: &[i128],
        with_i128_opt_column => column_i128_opt: &[Option<i128>],
        with_i32_column => column_i32: &[i32],
        with_i32_opt_column => column_i32_opt: &[Option<i32>],
        with_i16_column => column_i16: &[i16],
        with_i16_opt_column => column_i16_opt: &[Option<i16>],
        with_i8_column => column_i8: &[i8],
        with_i8_opt_column => column_i8_opt: &[Option<i8>],
        with_u64_column => column_u64: &[u64],
        with_u64_opt_column => column_u64_opt: &[Option<u64>],
        with_u32_column => column_u32: &[u32],
        with_u32_opt_column => column_u32_opt: &[Option<u32>],
        with_u16_column => column_u16: &[u16],
        with_u16_opt_column => column_u16_opt: &[Option<u16>],
        with_u8_column => column_u8: &[u8],
        with_u8_opt_column => column_u8_opt: &[Option<u8>],
        with_f64_column => column_f64: &[f64],
        with_f64_opt_column => column_f64_opt: &[Option<f64>],
        with_f32_column => column_f32: &[f32],
        with_f32_opt_column => column_f32_opt: &[Option<f32>],
        with_bool_column => column_bool: &[bool],
        with_bool_opt_column => column_bool_opt: &[Option<bool>],
        with_string_column => column_string: &[String],
        with_string_opt_column => column_string_opt: &[Option<String>],
        with_str_column => column_str: &[&str],
        with_str_opt_column => column_str_opt: &[Option<&str>],
        with_binary_column => column_binary: &[&[u8]],
        with_binary_opt_column => column_binary_opt: &[Option<&[u8]>],
        with_bytes_column => column_bytes: &[Vec<u8>],
        with_bytes_opt_column => column_bytes_opt: &[Option<Vec<u8>>],
    );

    pub fn build(self) -> Result<GDSDataFrame, polars::error::PolarsError> {
        let df = DataFrame::new(self.columns)?;
        Ok(GDSDataFrame::from(df))
    }
}

/// Multiply an f64 column by a scalar factor.
pub fn scale_f64_column(
    table: &mut GDSDataFrame,
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
