//! Series builders and helpers.

use polars::prelude::{
    BinaryChunked, IntoSeries, NamedFrom, NewChunkedArray, Series, UInt16Chunked, UInt8Chunked,
};

pub fn series_i64(name: &str, values: &[i64]) -> Series {
    Series::new(name.into(), values)
}

pub fn series_i64_opt(name: &str, values: &[Option<i64>]) -> Series {
    Series::new(name.into(), values)
}

pub fn series_i128(name: &str, values: &[i128]) -> Series {
    Series::new(name.into(), values)
}

pub fn series_i128_opt(name: &str, values: &[Option<i128>]) -> Series {
    Series::new(name.into(), values)
}

pub fn series_i32(name: &str, values: &[i32]) -> Series {
    Series::new(name.into(), values)
}

pub fn series_i32_opt(name: &str, values: &[Option<i32>]) -> Series {
    Series::new(name.into(), values)
}

pub fn series_i16(name: &str, values: &[i16]) -> Series {
    Series::new(name.into(), values)
}

pub fn series_i16_opt(name: &str, values: &[Option<i16>]) -> Series {
    Series::new(name.into(), values)
}

pub fn series_i8(name: &str, values: &[i8]) -> Series {
    Series::new(name.into(), values)
}

pub fn series_i8_opt(name: &str, values: &[Option<i8>]) -> Series {
    Series::new(name.into(), values)
}

pub fn series_u64(name: &str, values: &[u64]) -> Series {
    Series::new(name.into(), values)
}

pub fn series_u64_opt(name: &str, values: &[Option<u64>]) -> Series {
    Series::new(name.into(), values)
}

pub fn series_u32(name: &str, values: &[u32]) -> Series {
    Series::new(name.into(), values)
}

pub fn series_u32_opt(name: &str, values: &[Option<u32>]) -> Series {
    Series::new(name.into(), values)
}

pub fn series_u16(name: &str, values: &[u16]) -> Series {
    UInt16Chunked::from_slice(name.into(), values).into_series()
}

pub fn series_u16_opt(name: &str, values: &[Option<u16>]) -> Series {
    let mut series = UInt16Chunked::from_iter(values.iter().copied()).into_series();
    series.rename(name.into());
    series
}

pub fn series_u8(name: &str, values: &[u8]) -> Series {
    UInt8Chunked::from_slice(name.into(), values).into_series()
}

pub fn series_u8_opt(name: &str, values: &[Option<u8>]) -> Series {
    let mut series = UInt8Chunked::from_iter(values.iter().copied()).into_series();
    series.rename(name.into());
    series
}

pub fn series_f64(name: &str, values: &[f64]) -> Series {
    Series::new(name.into(), values)
}

pub fn series_f64_opt(name: &str, values: &[Option<f64>]) -> Series {
    Series::new(name.into(), values)
}

pub fn series_f32(name: &str, values: &[f32]) -> Series {
    Series::new(name.into(), values)
}

pub fn series_f32_opt(name: &str, values: &[Option<f32>]) -> Series {
    Series::new(name.into(), values)
}

pub fn series_bool(name: &str, values: &[bool]) -> Series {
    Series::new(name.into(), values)
}

pub fn series_bool_opt(name: &str, values: &[Option<bool>]) -> Series {
    Series::new(name.into(), values)
}

pub fn series_str(name: &str, values: &[&str]) -> Series {
    Series::new(name.into(), values)
}

pub fn series_str_opt(name: &str, values: &[Option<&str>]) -> Series {
    Series::new(name.into(), values)
}

pub fn series_string(name: &str, values: &[String]) -> Series {
    Series::new(name.into(), values)
}

pub fn series_string_opt(name: &str, values: &[Option<String>]) -> Series {
    Series::new(name.into(), values)
}

pub fn series_binary(name: &str, values: &[&[u8]]) -> Series {
    BinaryChunked::from_slice(name.into(), values).into_series()
}

pub fn series_binary_opt(name: &str, values: &[Option<&[u8]>]) -> Series {
    let slices: Vec<Option<&[u8]>> = values.iter().copied().collect();
    Series::new(name.into(), slices)
}

pub fn series_bytes(name: &str, values: &[Vec<u8>]) -> Series {
    let slices: Vec<&[u8]> = values.iter().map(|value| value.as_slice()).collect();
    BinaryChunked::from_slice(name.into(), &slices).into_series()
}

pub fn series_bytes_opt(name: &str, values: &[Option<Vec<u8>>]) -> Series {
    let slices: Vec<Option<&[u8]>> = values
        .iter()
        .map(|value| value.as_ref().map(|bytes| bytes.as_slice()))
        .collect();
    Series::new(name.into(), slices)
}
