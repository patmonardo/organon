//! DataType helpers (py-polars style naming).

pub use polars::prelude::DataType;

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
