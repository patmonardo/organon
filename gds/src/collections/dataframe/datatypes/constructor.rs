//! Rust-native constructor helpers inspired by polars.datatypes.constructor.

use polars::prelude::{AnyValue, DataType, PolarsResult, Series};

use super::{Categorical, EnumType};

pub type SeriesConstructor<'a> =
    Box<dyn Fn(&str, &[AnyValue<'static>]) -> PolarsResult<Series> + 'a>;

fn boxed_constructor<'a>(dtype: &'a DataType) -> SeriesConstructor<'a> {
    Box::new(move |name, values| series_from_any_values(name, values, dtype))
}

fn boxed_constructor_owned(dtype: DataType) -> SeriesConstructor<'static> {
    Box::new(move |name, values| series_from_any_values(name, values, &dtype))
}

/// Construct a `Series` from `AnyValue` inputs, then cast to the target dtype if needed.
pub fn series_from_any_values(
    name: &str,
    values: &[AnyValue<'static>],
    dtype: &DataType,
) -> PolarsResult<Series> {
    if values.is_empty() {
        return Ok(Series::new_empty(name.into(), dtype));
    }
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
pub fn polars_type_to_constructor<'a>(dtype: &'a DataType) -> SeriesConstructor<'a> {
    match dtype {
        DataType::Boolean
        | DataType::UInt8
        | DataType::UInt16
        | DataType::UInt32
        | DataType::UInt64
        | DataType::UInt128
        | DataType::Int8
        | DataType::Int16
        | DataType::Int32
        | DataType::Int64
        | DataType::Int128
        | DataType::Float32
        | DataType::Float64
        | DataType::Decimal(_, _)
        | DataType::String
        | DataType::Binary
        | DataType::BinaryOffset
        | DataType::Date
        | DataType::Datetime(_, _)
        | DataType::Duration(_)
        | DataType::Time
        | DataType::List(_)
        | DataType::Array(_, _)
        | DataType::Struct(_)
        | DataType::Categorical(_, _)
        | DataType::Enum(_, _)
        | DataType::Object(_)
        | DataType::Null
        | DataType::Unknown(_) => boxed_constructor(dtype),
    }
}

/// Return a constructor closure for a categorical helper.
pub fn categorical_to_constructor(
    categorical: &Categorical,
) -> PolarsResult<SeriesConstructor<'static>> {
    let dtype = categorical.to_dtype()?;
    Ok(boxed_constructor_owned(dtype))
}

/// Return a constructor closure for an enum helper.
pub fn enum_to_constructor(enum_type: &EnumType) -> PolarsResult<SeriesConstructor<'static>> {
    let dtype = enum_type.to_dtype()?;
    Ok(boxed_constructor_owned(dtype))
}

#[cfg(test)]
mod tests {
    use super::*;
    use polars::prelude::AnyValue;

    #[test]
    fn builds_categorical_series() {
        let ctor =
            categorical_to_constructor(&Categorical::new(None)).expect("categorical constructor");
        let values = vec![
            AnyValue::StringOwned("alpha".into()),
            AnyValue::StringOwned("beta".into()),
        ];
        let series = ctor("cat", &values).expect("categorical series");
        assert!(matches!(series.dtype(), DataType::Categorical(_, _)));
    }

    #[test]
    fn builds_enum_series() {
        let enum_type = EnumType::new(["alpha", "beta"]);
        let ctor = enum_to_constructor(&enum_type).expect("enum constructor");
        let values = vec![
            AnyValue::StringOwned("alpha".into()),
            AnyValue::StringOwned("beta".into()),
        ];
        let series = ctor("enm", &values).expect("enum series");
        assert!(matches!(series.dtype(), DataType::Enum(_, _)));
    }
}
