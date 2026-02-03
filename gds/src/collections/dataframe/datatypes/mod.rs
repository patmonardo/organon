//! DataType helpers (py-polars style naming).

use polars::prelude::DataType;

pub mod classes;
pub mod constants;
pub mod constructor;
pub mod convert;
pub mod extension;
pub mod group;
pub mod parse;
pub mod utils;

pub type PolarsDataType = DataType;
