//! DataType helpers (py-polars style naming).

use polars::prelude::{DataType, Field, TimeUnit};

pub mod classes;
pub mod constants;
pub mod constructor;
pub mod convert;
pub mod extension;
pub mod group;
pub mod parse;
pub mod utils;

pub use parse::{
    parse_into_datatype_expr, parse_into_dtype, parse_py_type_into_dtype, try_parse_into_dtype,
    DTypeInput, DTypeParseError,
};

pub type PolarsDataType = DataType;

pub fn int8() -> DataType {
    DataType::Int8
}

pub fn int16() -> DataType {
    DataType::Int16
}

pub fn int32() -> DataType {
    DataType::Int32
}

pub fn int64() -> DataType {
    DataType::Int64
}

pub fn int128() -> DataType {
    DataType::Int128
}

pub fn uint8() -> DataType {
    DataType::UInt8
}

pub fn uint16() -> DataType {
    DataType::UInt16
}

pub fn uint32() -> DataType {
    DataType::UInt32
}

pub fn uint64() -> DataType {
    DataType::UInt64
}

pub fn float32() -> DataType {
    DataType::Float32
}

pub fn float64() -> DataType {
    DataType::Float64
}

pub fn boolean() -> DataType {
    DataType::Boolean
}

pub fn string() -> DataType {
    DataType::String
}

pub fn utf8() -> DataType {
    DataType::String
}

pub fn binary() -> DataType {
    DataType::Binary
}

pub fn binary_offset() -> DataType {
    DataType::BinaryOffset
}

pub fn date() -> DataType {
    DataType::Date
}

pub fn datetime(time_unit: TimeUnit) -> DataType {
    DataType::Datetime(time_unit, None)
}

pub fn time() -> DataType {
    DataType::Time
}

pub fn duration(time_unit: TimeUnit) -> DataType {
    DataType::Duration(time_unit)
}

pub fn null() -> DataType {
    DataType::Null
}

pub fn list(inner: DataType) -> DataType {
    DataType::List(Box::new(inner))
}

#[cfg(feature = "dtype-array")]
pub fn array(inner: DataType, width: usize) -> DataType {
    DataType::Array(Box::new(inner), width)
}

#[cfg(feature = "dtype-decimal")]
pub fn decimal(precision: Option<usize>, scale: Option<usize>) -> DataType {
    DataType::Decimal(precision.unwrap_or(38), scale.unwrap_or(0))
}

#[cfg(feature = "object")]
pub fn object(name: &'static str) -> DataType {
    DataType::Object(name)
}

pub fn struct_(fields: Vec<Field>) -> DataType {
    DataType::Struct(fields)
}

pub fn record(fields: Vec<Field>) -> DataType {
    struct_(fields)
}

pub fn field(name: &str, dtype: DataType) -> Field {
    Field::new(name.into(), dtype)
}
