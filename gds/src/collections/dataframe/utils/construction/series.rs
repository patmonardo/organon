//! Series construction utilities (seed pass).
//!
//! Mirrors py-polars: polars/_utils/construction/series.py

use polars::error::PolarsError;
use polars::prelude::Series;

/// Construct a Series from a sequence.
pub fn sequence_to_pyseries() -> Result<Series, PolarsError> {
    todo!("seed pass: sequence_to_pyseries")
}

/// Construct a Series from an iterable/generator.
pub fn iterable_to_pyseries() -> Result<Series, PolarsError> {
    todo!("seed pass: iterable_to_pyseries")
}

/// Construct a Series from a pandas Series/Index.
pub fn pandas_to_pyseries() -> Result<Series, PolarsError> {
    todo!("seed pass: pandas_to_pyseries")
}

/// Construct a Series from an Arrow array.
pub fn arrow_to_pyseries() -> Result<Series, PolarsError> {
    todo!("seed pass: arrow_to_pyseries")
}

/// Construct a Series from a numpy array.
pub fn numpy_to_pyseries() -> Result<Series, PolarsError> {
    todo!("seed pass: numpy_to_pyseries")
}

/// Construct a Series from an existing Series.
pub fn series_to_pyseries() -> Result<Series, PolarsError> {
    todo!("seed pass: series_to_pyseries")
}

/// Construct a Series from a DataFrame.
pub fn dataframe_to_pyseries() -> Result<Series, PolarsError> {
    todo!("seed pass: dataframe_to_pyseries")
}
