//! Rust translation scaffold for polars.datatypes.parse.

use super::{
    binary, binary_offset, boolean, date, datetime, duration, float32, float64, int128, int16,
    int32, int64, int8, list, null, string, time, uint16, uint32, uint64, uint8, PolarsDataType,
};
use polars::prelude::{TimeUnit, UnknownKind};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DTypeInput<'a> {
    Polars(PolarsDataType),
    Name(&'a str),
}

impl<'a> From<PolarsDataType> for DTypeInput<'a> {
    fn from(dtype: PolarsDataType) -> Self {
        Self::Polars(dtype)
    }
}

impl<'a> From<&'a PolarsDataType> for DTypeInput<'a> {
    fn from(dtype: &'a PolarsDataType) -> Self {
        Self::Polars(dtype.clone())
    }
}

impl<'a> From<&'a str> for DTypeInput<'a> {
    fn from(name: &'a str) -> Self {
        Self::Name(name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DTypeParseError {
    Unsupported(String),
    InvalidInput(String),
}

impl DTypeParseError {
    fn invalid(input: &str) -> Self {
        Self::InvalidInput(format!(
            "cannot parse input '{input}' into Polars data type"
        ))
    }
}

impl fmt::Display for DTypeParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DTypeParseError::Unsupported(msg) | DTypeParseError::InvalidInput(msg) => {
                write!(f, "{msg}")
            }
        }
    }
}

impl std::error::Error for DTypeParseError {}

pub fn parse_into_datatype_expr<'a>(
    input: impl Into<DTypeInput<'a>>,
) -> Result<PolarsDataType, DTypeParseError> {
    parse_into_dtype(input)
}

pub fn parse_py_type_into_dtype<'a>(
    input: impl Into<DTypeInput<'a>>,
) -> Result<PolarsDataType, DTypeParseError> {
    parse_into_dtype(input)
}

pub fn parse_into_dtype<'a>(
    input: impl Into<DTypeInput<'a>>,
) -> Result<PolarsDataType, DTypeParseError> {
    match input.into() {
        DTypeInput::Polars(dtype) => Ok(dtype),
        DTypeInput::Name(name) => parse_name_to_dtype(name),
    }
}

pub fn try_parse_into_dtype<'a>(input: impl Into<DTypeInput<'a>>) -> Option<PolarsDataType> {
    parse_into_dtype(input).ok()
}

fn parse_name_to_dtype(raw: &str) -> Result<PolarsDataType, DTypeParseError> {
    if let Some(result) = parse_generic(raw) {
        return result;
    }

    let norm = raw.trim().to_ascii_lowercase();
    match norm.as_str() {
        "int8" | "i8" => Ok(int8()),
        "int16" | "i16" => Ok(int16()),
        "int32" | "i32" => Ok(int32()),
        "int" | "int64" | "i64" => Ok(int64()),
        "int128" | "i128" => Ok(int128()),
        "uint8" | "u8" => Ok(uint8()),
        "uint16" | "u16" => Ok(uint16()),
        "uint32" | "u32" => Ok(uint32()),
        "uint64" | "u64" => Ok(uint64()),
        "float32" | "f32" => Ok(float32()),
        "float" | "float64" | "f64" => Ok(float64()),
        "bool" | "boolean" => Ok(boolean()),
        "str" | "string" | "utf8" => Ok(string()),
        "binary" | "bytes" => Ok(binary()),
        "binary_offset" | "binaryoffset" => Ok(binary_offset()),
        "date" => Ok(date()),
        "datetime" => Ok(datetime(TimeUnit::Microseconds)),
        "time" => Ok(time()),
        "duration" => Ok(duration(TimeUnit::Microseconds)),
        "null" | "none" => Ok(null()),
        "list" | "tuple" => Ok(list(null())),
        "unknown" => Ok(PolarsDataType::Unknown(UnknownKind::Any)),
        _ => Err(DTypeParseError::invalid(raw)),
    }
}

fn parse_generic(raw: &str) -> Option<Result<PolarsDataType, DTypeParseError>> {
    let trimmed = raw.trim();

    if let Some(rest) = trimmed.strip_prefix("datetime[") {
        let unit = rest.strip_suffix(']')?;
        return Some(parse_time_unit(unit).map(|unit| datetime(unit)));
    }

    if let Some(rest) = trimmed.strip_prefix("duration[") {
        let unit = rest.strip_suffix(']')?;
        return Some(parse_time_unit(unit).map(|unit| duration(unit)));
    }

    if let Some(rest) = trimmed.strip_prefix("list[") {
        let inner = rest.strip_suffix(']')?;
        return Some(parse_name_to_dtype(inner).map(list));
    }

    if let Some(rest) = trimmed.strip_prefix("tuple[") {
        let inner = rest.strip_suffix(']')?;
        return Some(parse_name_to_dtype(inner).map(list));
    }

    None
}

fn parse_time_unit(raw: &str) -> Result<TimeUnit, DTypeParseError> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "ns" | "nanoseconds" => Ok(TimeUnit::Nanoseconds),
        "us" | "microseconds" => Ok(TimeUnit::Microseconds),
        "ms" | "milliseconds" => Ok(TimeUnit::Milliseconds),
        other => Err(DTypeParseError::Unsupported(format!(
            "unsupported time unit '{other}'"
        ))),
    }
}
