//! Rust translation scaffold for polars.datatypes._parse.

use super::GDSDataType;
use polars::prelude::{DataType, DataTypeExpr, TimeUnit, UnknownKind};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum DTypeInput<'a> {
    Polars(GDSDataType),
    Name(&'a str),
    DataTypeExpr(DataTypeExpr),
}

impl<'a> From<GDSDataType> for DTypeInput<'a> {
    fn from(dtype: GDSDataType) -> Self {
        Self::Polars(dtype)
    }
}

impl<'a> From<&'a GDSDataType> for DTypeInput<'a> {
    fn from(dtype: &'a GDSDataType) -> Self {
        Self::Polars(dtype.clone())
    }
}

impl<'a> From<&'a str> for DTypeInput<'a> {
    fn from(name: &'a str) -> Self {
        Self::Name(name)
    }
}

impl<'a> From<DataTypeExpr> for DTypeInput<'a> {
    fn from(expr: DataTypeExpr) -> Self {
        Self::DataTypeExpr(expr)
    }
}

impl<'a> From<&'a DataTypeExpr> for DTypeInput<'a> {
    fn from(expr: &'a DataTypeExpr) -> Self {
        Self::DataTypeExpr(expr.clone())
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

    fn invalid_type(input: &str) -> Self {
        Self::InvalidInput(format!("cannot parse input {input} into Polars data type"))
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
) -> Result<DataTypeExpr, DTypeParseError> {
    match input.into() {
        DTypeInput::DataTypeExpr(expr) => Ok(expr),
        other => parse_into_dtype_from_input(other).map(DataTypeExpr::Literal),
    }
}

pub fn parse_py_type_into_dtype<'a>(
    input: impl Into<DTypeInput<'a>>,
) -> Result<GDSDataType, DTypeParseError> {
    match input.into() {
        DTypeInput::Polars(dtype) => Ok(dtype),
        DTypeInput::Name(name) => parse_name_to_dtype(name),
        DTypeInput::DataTypeExpr(_) => Err(DTypeParseError::invalid_type("DataTypeExpr")),
    }
}

pub fn parse_into_dtype<'a>(
    input: impl Into<DTypeInput<'a>>,
) -> Result<GDSDataType, DTypeParseError> {
    parse_into_dtype_from_input(input.into())
}

pub fn try_parse_into_dtype<'a>(input: impl Into<DTypeInput<'a>>) -> Option<GDSDataType> {
    parse_into_dtype(input).ok()
}

fn parse_name_to_dtype(raw: &str) -> Result<GDSDataType, DTypeParseError> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err(DTypeParseError::invalid(raw));
    }

    if let Some(result) = parse_generic(trimmed) {
        return result;
    }

    let formatted = strip_optional(trimmed);

    if let Some(dtype) = parse_py_type_name(formatted) {
        return Ok(dtype);
    }

    let norm = formatted.to_ascii_lowercase();
    match norm.as_str() {
        "int8" | "i8" => Ok(DataType::Int8),
        "int16" | "i16" => Ok(DataType::Int16),
        "int32" | "i32" => Ok(DataType::Int32),
        "int" | "int64" | "i64" => Ok(DataType::Int64),
        "int128" | "i128" => Ok(DataType::Int128),
        "uint8" | "u8" => Ok(DataType::UInt8),
        "uint16" | "u16" => Ok(DataType::UInt16),
        "uint32" | "u32" => Ok(DataType::UInt32),
        "uint64" | "u64" => Ok(DataType::UInt64),
        "float32" | "f32" => Ok(DataType::Float32),
        "float" | "float64" | "f64" => Ok(DataType::Float64),
        "bool" | "boolean" => Ok(DataType::Boolean),
        "str" | "string" | "utf8" => Ok(DataType::String),
        "binary" | "bytes" => Ok(DataType::Binary),
        "binary_offset" | "binaryoffset" => Ok(DataType::BinaryOffset),
        "date" => Ok(DataType::Date),
        "datetime" => Ok(DataType::Datetime(TimeUnit::Microseconds, None)),
        "time" => Ok(DataType::Time),
        "duration" => Ok(DataType::Duration(TimeUnit::Microseconds)),
        "null" | "none" | "nonetype" => Ok(DataType::Null),
        "list" | "tuple" => Ok(DataType::List(Box::new(DataType::Null))),
        "unknown" => Ok(DataType::Unknown(UnknownKind::Any)),
        "decimal" => Ok(DataType::Decimal(38, 0)),
        "object" => Ok(DataType::Object("object")),
        _ => Err(DTypeParseError::invalid(raw)),
    }
}

fn parse_generic(raw: &str) -> Option<Result<GDSDataType, DTypeParseError>> {
    let trimmed = raw.trim();

    if let Some(rest) = trimmed.strip_prefix("datetime[") {
        let unit = rest.strip_suffix(']')?;
        return Some(parse_time_unit(unit).map(|unit| DataType::Datetime(unit, None)));
    }

    if let Some(rest) = trimmed.strip_prefix("duration[") {
        let unit = rest.strip_suffix(']')?;
        return Some(parse_time_unit(unit).map(DataType::Duration));
    }

    if let Some(rest) = trimmed.strip_prefix("list[") {
        let inner = rest.strip_suffix(']')?;
        return Some(parse_name_to_dtype(inner).map(|dtype| DataType::List(Box::new(dtype))));
    }

    if let Some(rest) = trimmed.strip_prefix("tuple[") {
        let inner = rest.strip_suffix(']')?;
        return Some(parse_name_to_dtype(inner).map(|dtype| DataType::List(Box::new(dtype))));
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

fn parse_into_dtype_from_input(input: DTypeInput<'_>) -> Result<GDSDataType, DTypeParseError> {
    match input {
        DTypeInput::Polars(dtype) => Ok(dtype),
        DTypeInput::Name(name) => parse_name_to_dtype(name),
        DTypeInput::DataTypeExpr(_) => Err(DTypeParseError::invalid_type("DataTypeExpr")),
    }
}

fn strip_optional(raw: &str) -> &str {
    let mut formatted = raw.trim();
    if let Some(rest) = formatted.strip_prefix("None |") {
        formatted = rest.trim();
    }
    if let Some(rest) = formatted.strip_suffix("| None") {
        formatted = rest.trim();
    }
    formatted
}

fn parse_py_type_name(raw: &str) -> Option<GDSDataType> {
    match raw {
        "Decimal" => Some(DataType::Decimal(38, 0)),
        "NoneType" => Some(DataType::Null),
        "bool" => Some(DataType::Boolean),
        "bytes" => Some(DataType::Binary),
        "date" => Some(DataType::Date),
        "datetime" => Some(DataType::Datetime(TimeUnit::Microseconds, None)),
        "float" => Some(DataType::Float64),
        "int" => Some(DataType::Int64),
        "list" => Some(DataType::List(Box::new(DataType::Null))),
        "object" => Some(DataType::Object("object")),
        "str" => Some(DataType::String),
        "time" => Some(DataType::Time),
        "timedelta" => Some(DataType::Duration(TimeUnit::Microseconds)),
        "tuple" => Some(DataType::List(Box::new(DataType::Null))),
        "Unknown" => Some(DataType::Unknown(UnknownKind::Any)),
        _ => None,
    }
}
