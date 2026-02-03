//! Rust-native conversion helpers inspired by polars.datatypes.convert.

use std::collections::HashSet;

use arrow::datatypes::{ArrowDataType, Field as ArrowField, TimeUnit as ArrowTimeUnit};
use polars::prelude::{AnyValue, DataType, PlSmallStr, TimeUnit};

use super::parse::parse_into_dtype;
use super::PolarsDataType;

/// Indicate whether the given dtype is a Polars dtype.
pub fn is_polars_dtype(dtype: &PolarsDataType, include_unknown: bool) -> bool {
    if include_unknown {
        true
    } else {
        !matches!(dtype, DataType::Unknown(_))
    }
}

/// Return a set of unique dtypes found in one or more (potentially compound) dtypes.
pub fn unpack_dtypes<I>(dtypes: I, include_compound: bool) -> HashSet<PolarsDataType>
where
    I: IntoIterator<Item = PolarsDataType>,
{
    let mut unpacked = HashSet::new();
    for dtype in dtypes {
        match dtype {
            DataType::List(inner) => {
                if include_compound {
                    unpacked.insert(DataType::List(inner.clone()));
                }
                unpacked.extend(unpack_dtypes([*inner], include_compound));
            }
            DataType::Array(inner, width) => {
                if include_compound {
                    unpacked.insert(DataType::Array(inner.clone(), width));
                }
                unpacked.extend(unpack_dtypes([*inner], include_compound));
            }
            DataType::Struct(fields) => {
                if include_compound {
                    unpacked.insert(DataType::Struct(fields.clone()));
                }
                let inner_dtypes = fields.into_iter().map(|field| field.dtype().clone());
                unpacked.extend(unpack_dtypes(inner_dtypes, include_compound));
            }
            DataType::Unknown(_) => {
                if include_compound {
                    unpacked.insert(dtype);
                }
            }
            other => {
                unpacked.insert(other);
            }
        }
    }
    unpacked
}

/// Return a stable string name for FFI use.
pub fn dtype_to_ffiname(dtype: &PolarsDataType) -> Option<&'static str> {
    use DataType::*;

    let name = match dtype {
        Int8 => "i8",
        Int16 => "i16",
        Int32 => "i32",
        Int64 => "i64",
        Int128 => "i128",
        UInt8 => "u8",
        UInt16 => "u16",
        UInt32 => "u32",
        UInt64 => "u64",
        UInt128 => "u128",
        Float32 => "f32",
        Float64 => "f64",
        Boolean => "bool",
        String => "str",
        Binary | BinaryOffset => "binary",
        Date => "date",
        Time => "time",
        Datetime(_, _) => "datetime",
        Duration(_) => "duration",
        List(_) => "list",
        Struct(_) => "struct",
        Object(_) => "object",
        Categorical(_, _) => "categorical",
        Enum(_, _) => "enum",
        Decimal(_, _) => "decimal",
        Null => "null",
        Unknown(_) => "unknown",
        _ => return None,
    };
    Some(name)
}

/// Convert a Polars dtype to an Arrow dtype (best-effort).
pub fn dtype_to_arrow_type(dtype: &PolarsDataType) -> ArrowDataType {
    use DataType::*;

    match dtype {
        Int8 => ArrowDataType::Int8,
        Int16 => ArrowDataType::Int16,
        Int32 => ArrowDataType::Int32,
        Int64 => ArrowDataType::Int64,
        Int128 => ArrowDataType::Int128,
        UInt8 => ArrowDataType::UInt8,
        UInt16 => ArrowDataType::UInt16,
        UInt32 => ArrowDataType::UInt32,
        UInt64 => ArrowDataType::UInt64,
        UInt128 => ArrowDataType::UInt128,
        Float32 => ArrowDataType::Float32,
        Float64 => ArrowDataType::Float64,
        Boolean => ArrowDataType::Boolean,
        String => ArrowDataType::Utf8,
        Binary | BinaryOffset => ArrowDataType::Binary,
        Date => ArrowDataType::Date32,
        Time => ArrowDataType::Time64(ArrowTimeUnit::Microsecond),
        Datetime(unit, _tz) => ArrowDataType::Timestamp(to_arrow_time_unit(*unit), None),
        Duration(unit) => ArrowDataType::Duration(to_arrow_time_unit(*unit)),
        List(inner) => ArrowDataType::List(Box::new(ArrowField::new(
            PlSmallStr::from("item"),
            dtype_to_arrow_type(inner),
            true,
        ))),
        Array(inner, width) => ArrowDataType::FixedSizeList(
            Box::new(ArrowField::new(
                PlSmallStr::from("item"),
                dtype_to_arrow_type(inner),
                true,
            )),
            *width,
        ),
        Struct(fields) => ArrowDataType::Struct(
            fields
                .iter()
                .map(|field| {
                    ArrowField::new(
                        field.name().clone(),
                        dtype_to_arrow_type(field.dtype()),
                        true,
                    )
                })
                .collect(),
        ),
        Decimal(precision, scale) => ArrowDataType::Decimal(*precision, *scale as usize),
        Null | Unknown(_) => ArrowDataType::Null,
        _ => ArrowDataType::Null,
    }
}

/// Map a Polars short repr (eg: "i64", "list[str]") back into a dtype.
pub fn dtype_short_repr_to_dtype(dtype_string: Option<&str>) -> Option<PolarsDataType> {
    let raw = dtype_string?.trim();
    if raw.is_empty() {
        return None;
    }

    if let Ok(dtype) = parse_into_dtype(raw) {
        return Some(dtype);
    }

    None
}

/// Best-effort value casting helper (seed pass).
pub fn maybe_cast(value: AnyValue<'static>, _dtype: &PolarsDataType) -> AnyValue<'static> {
    value
}

fn to_arrow_time_unit(unit: TimeUnit) -> ArrowTimeUnit {
    match unit {
        TimeUnit::Nanoseconds => ArrowTimeUnit::Nanosecond,
        TimeUnit::Microseconds => ArrowTimeUnit::Microsecond,
        TimeUnit::Milliseconds => ArrowTimeUnit::Millisecond,
    }
}
