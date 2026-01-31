//! Column builders for DataFrame-backed tables.

use polars::prelude::Column;

use crate::collections::dataframe::series::{
    series_binary, series_binary_opt, series_bool, series_bool_opt, series_bytes, series_bytes_opt,
    series_f32, series_f32_opt, series_f64, series_f64_opt, series_i128, series_i128_opt,
    series_i16, series_i16_opt, series_i32, series_i32_opt, series_i64, series_i64_opt, series_i8,
    series_i8_opt, series_str, series_str_opt, series_string, series_string_opt, series_u16,
    series_u16_opt, series_u32, series_u32_opt, series_u64, series_u64_opt, series_u8,
    series_u8_opt,
};

pub fn column_i64(name: &str, values: &[i64]) -> Column {
    Column::from(series_i64(name, values))
}

pub fn column_i64_opt(name: &str, values: &[Option<i64>]) -> Column {
    Column::from(series_i64_opt(name, values))
}

pub fn column_i128(name: &str, values: &[i128]) -> Column {
    Column::from(series_i128(name, values))
}

pub fn column_i128_opt(name: &str, values: &[Option<i128>]) -> Column {
    Column::from(series_i128_opt(name, values))
}

pub fn column_i32(name: &str, values: &[i32]) -> Column {
    Column::from(series_i32(name, values))
}

pub fn column_i32_opt(name: &str, values: &[Option<i32>]) -> Column {
    Column::from(series_i32_opt(name, values))
}

pub fn column_i16(name: &str, values: &[i16]) -> Column {
    Column::from(series_i16(name, values))
}

pub fn column_i16_opt(name: &str, values: &[Option<i16>]) -> Column {
    Column::from(series_i16_opt(name, values))
}

pub fn column_i8(name: &str, values: &[i8]) -> Column {
    Column::from(series_i8(name, values))
}

pub fn column_i8_opt(name: &str, values: &[Option<i8>]) -> Column {
    Column::from(series_i8_opt(name, values))
}

pub fn column_u64(name: &str, values: &[u64]) -> Column {
    Column::from(series_u64(name, values))
}

pub fn column_u64_opt(name: &str, values: &[Option<u64>]) -> Column {
    Column::from(series_u64_opt(name, values))
}

pub fn column_u32(name: &str, values: &[u32]) -> Column {
    Column::from(series_u32(name, values))
}

pub fn column_u32_opt(name: &str, values: &[Option<u32>]) -> Column {
    Column::from(series_u32_opt(name, values))
}

pub fn column_u16(name: &str, values: &[u16]) -> Column {
    Column::from(series_u16(name, values))
}

pub fn column_u16_opt(name: &str, values: &[Option<u16>]) -> Column {
    Column::from(series_u16_opt(name, values))
}

pub fn column_u8(name: &str, values: &[u8]) -> Column {
    Column::from(series_u8(name, values))
}

pub fn column_u8_opt(name: &str, values: &[Option<u8>]) -> Column {
    Column::from(series_u8_opt(name, values))
}

pub fn column_f64(name: &str, values: &[f64]) -> Column {
    Column::from(series_f64(name, values))
}

pub fn column_f64_opt(name: &str, values: &[Option<f64>]) -> Column {
    Column::from(series_f64_opt(name, values))
}

pub fn column_f32(name: &str, values: &[f32]) -> Column {
    Column::from(series_f32(name, values))
}

pub fn column_f32_opt(name: &str, values: &[Option<f32>]) -> Column {
    Column::from(series_f32_opt(name, values))
}

pub fn column_bool(name: &str, values: &[bool]) -> Column {
    Column::from(series_bool(name, values))
}

pub fn column_bool_opt(name: &str, values: &[Option<bool>]) -> Column {
    Column::from(series_bool_opt(name, values))
}

pub fn column_str(name: &str, values: &[&str]) -> Column {
    Column::from(series_str(name, values))
}

pub fn column_str_opt(name: &str, values: &[Option<&str>]) -> Column {
    Column::from(series_str_opt(name, values))
}

pub fn column_string(name: &str, values: &[String]) -> Column {
    Column::from(series_string(name, values))
}

pub fn column_string_opt(name: &str, values: &[Option<String>]) -> Column {
    Column::from(series_string_opt(name, values))
}

pub fn column_binary(name: &str, values: &[&[u8]]) -> Column {
    Column::from(series_binary(name, values))
}

pub fn column_binary_opt(name: &str, values: &[Option<&[u8]>]) -> Column {
    Column::from(series_binary_opt(name, values))
}

pub fn column_bytes(name: &str, values: &[Vec<u8>]) -> Column {
    Column::from(series_bytes(name, values))
}

pub fn column_bytes_opt(name: &str, values: &[Option<Vec<u8>>]) -> Column {
    Column::from(series_bytes_opt(name, values))
}
