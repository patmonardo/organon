//! Rust-native constructor helpers inspired by polars.datatypes.constructor.

use polars::prelude::{AnyValue, DataType, PolarsResult, Series};

/// Construct a `Series` from `AnyValue` inputs, then cast to the target dtype if needed.
pub fn series_from_any_values(
    name: &str,
    values: &[AnyValue<'static>],
    dtype: &DataType,
) -> PolarsResult<Series> {
    let series = Series::from_any_values(name.into(), values, true)?;
    if series.dtype() == dtype {
        Ok(series)
    } else {
        series.cast(dtype)
    }
}

/// Return a constructor closure for the given dtype.
///
/// This mirrors the Python mapping without NumPy-specific constructors.
pub fn polars_type_to_constructor<'a>(
    dtype: &'a DataType,
) -> impl Fn(&str, &[AnyValue<'static>]) -> PolarsResult<Series> + 'a {
    move |name, values| series_from_any_values(name, values, dtype)
}
