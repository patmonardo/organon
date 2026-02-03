//! Other construction helpers (seed pass).
//!
//! Mirrors py-polars: polars/_utils/construction/other.py

use polars::error::PolarsError;
use polars::prelude::Series;

/// Convert a pandas Series to Arrow (placeholder).
#[allow(dead_code)]
pub fn pandas_series_to_arrow() -> Result<Series, PolarsError> {
    todo!("seed pass: pandas_series_to_arrow")
}

/// Coerce Arrow arrays for Polars compatibility (placeholder).
#[allow(dead_code)]
pub fn coerce_arrow() -> Result<(), PolarsError> {
    todo!("seed pass: coerce_arrow")
}
