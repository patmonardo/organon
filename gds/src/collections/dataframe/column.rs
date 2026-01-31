//! Column builders for DataFrame-backed tables.

use polars::prelude::Column;

use crate::collections::dataframe::series::{
    series_bool, series_f32, series_f64, series_i16, series_i32, series_i64, series_i8, series_str,
    series_string, series_u16, series_u32, series_u64, series_u8,
};

pub fn column_i64(name: &str, values: &[i64]) -> Column {
    Column::from(series_i64(name, values))
}

pub fn column_i32(name: &str, values: &[i32]) -> Column {
    Column::from(series_i32(name, values))
}

pub fn column_i16(name: &str, values: &[i16]) -> Column {
    Column::from(series_i16(name, values))
}

pub fn column_i8(name: &str, values: &[i8]) -> Column {
    Column::from(series_i8(name, values))
}

pub fn column_u64(name: &str, values: &[u64]) -> Column {
    Column::from(series_u64(name, values))
}

pub fn column_u32(name: &str, values: &[u32]) -> Column {
    Column::from(series_u32(name, values))
}

pub fn column_u16(name: &str, values: &[u16]) -> Column {
    Column::from(series_u16(name, values))
}

pub fn column_u8(name: &str, values: &[u8]) -> Column {
    Column::from(series_u8(name, values))
}

pub fn column_f64(name: &str, values: &[f64]) -> Column {
    Column::from(series_f64(name, values))
}

pub fn column_f32(name: &str, values: &[f32]) -> Column {
    Column::from(series_f32(name, values))
}

pub fn column_bool(name: &str, values: &[bool]) -> Column {
    Column::from(series_bool(name, values))
}

pub fn column_str(name: &str, values: &[&str]) -> Column {
    Column::from(series_str(name, values))
}

pub fn column_string(name: &str, values: &[String]) -> Column {
    Column::from(series_string(name, values))
}
