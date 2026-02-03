//! DataFrame construction utilities (seed pass).
//!
//! Mirrors py-polars: polars/_utils/construction/dataframe.py

use polars::error::PolarsError;
use polars::prelude::DataFrame;

/// Construct a DataFrame from a dict-like input.
///
/// Seed pass: placeholder for py-polars parity.
pub fn dict_to_pydf() -> Result<DataFrame, PolarsError> {
    todo!("seed pass: dict_to_pydf")
}

/// Construct a DataFrame from a sequence.
pub fn sequence_to_pydf() -> Result<DataFrame, PolarsError> {
    todo!("seed pass: sequence_to_pydf")
}

/// Construct a DataFrame from an iterable/generator.
pub fn iterable_to_pydf() -> Result<DataFrame, PolarsError> {
    todo!("seed pass: iterable_to_pydf")
}

/// Construct a DataFrame from a numpy array.
pub fn numpy_to_pydf() -> Result<DataFrame, PolarsError> {
    todo!("seed pass: numpy_to_pydf")
}

/// Construct a DataFrame from a pandas DataFrame.
pub fn pandas_to_pydf() -> Result<DataFrame, PolarsError> {
    todo!("seed pass: pandas_to_pydf")
}

/// Construct a DataFrame from an Arrow table/record batch.
pub fn arrow_to_pydf() -> Result<DataFrame, PolarsError> {
    todo!("seed pass: arrow_to_pydf")
}

/// Construct a DataFrame from a Series.
pub fn series_to_pydf() -> Result<DataFrame, PolarsError> {
    todo!("seed pass: series_to_pydf")
}

/// Construct a DataFrame from a DataFrame (schema override path).
pub fn dataframe_to_pydf() -> Result<DataFrame, PolarsError> {
    todo!("seed pass: dataframe_to_pydf")
}
