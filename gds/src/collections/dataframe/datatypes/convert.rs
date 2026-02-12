//! Rust-native conversion helpers inspired by polars.datatypes.convert.

use std::collections::HashSet;

use arrow::datatypes::{ArrowDataType, Field as ArrowField, TimeUnit as ArrowTimeUnit};
use polars::prelude::{AnyValue, DataType, PlSmallStr, TimeUnit, TimeZone};

use super::parse::parse_into_dtype;
use super::{EnumType, GDSDataType};

/// Indicate whether the given dtype is a Polars dtype.
pub fn is_polars_dtype(dtype: &GDSDataType, include_unknown: bool) -> bool {
    if include_unknown {
        true
    } else {
        !matches!(dtype, DataType::Unknown(_))
    }
}

/// Return a set of unique dtypes found in one or more (potentially compound) dtypes.
pub fn unpack_dtypes<I>(dtypes: I, include_compound: bool) -> HashSet<GDSDataType>
where
    I: IntoIterator<Item = GDSDataType>,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RustType {
    Bool,
    Date,
    DateTime,
    Decimal,
    Duration,
    Float32,
    Float64,
    Int64,
    Int128,
    UInt64,
    UInt128,
    String,
    Bytes,
    Vec,
    Struct,
    Time,
    Null,
    Object,
}

/// Convert a Polars dtype to a Rust-primitive-oriented type (best-effort).
pub fn dtype_to_rust_type(dtype: &GDSDataType) -> Option<RustType> {
    match dtype {
        DataType::Array(_, _) | DataType::List(_) => Some(RustType::Vec),
        DataType::Binary | DataType::BinaryOffset => Some(RustType::Bytes),
        DataType::Boolean => Some(RustType::Bool),
        DataType::Date => Some(RustType::Date),
        DataType::Datetime(_, _) => Some(RustType::DateTime),
        DataType::Decimal(_, _) => Some(RustType::Decimal),
        DataType::Duration(_) => Some(RustType::Duration),
        DataType::Float32 => Some(RustType::Float32),
        DataType::Float64 => Some(RustType::Float64),
        DataType::Int8 | DataType::Int16 | DataType::Int32 | DataType::Int64 => {
            Some(RustType::Int64)
        }
        DataType::Int128 => Some(RustType::Int128),
        DataType::Null => Some(RustType::Null),
        DataType::Object(_) => Some(RustType::Object),
        DataType::String => Some(RustType::String),
        DataType::Struct(_) => Some(RustType::Struct),
        DataType::Time => Some(RustType::Time),
        DataType::UInt8 | DataType::UInt16 | DataType::UInt32 | DataType::UInt64 => {
            Some(RustType::UInt64)
        }
        DataType::UInt128 => Some(RustType::UInt128),
        DataType::Enum(_, _) | DataType::Categorical(_, _) => Some(RustType::String),
        DataType::Unknown(_) => None,
    }
}

/// Convert a Rust type to an Arrow dtype (best-effort).
pub fn rust_type_to_arrow_type(rust_type: RustType) -> ArrowDataType {
    match rust_type {
        RustType::Bool => ArrowDataType::Boolean,
        RustType::Date => ArrowDataType::Date32,
        RustType::DateTime => ArrowDataType::Timestamp(ArrowTimeUnit::Microsecond, None),
        RustType::Decimal => ArrowDataType::Decimal(38, 0),
        RustType::Duration => ArrowDataType::Duration(ArrowTimeUnit::Microsecond),
        RustType::Float32 => ArrowDataType::Float32,
        RustType::Float64 => ArrowDataType::Float64,
        RustType::Int64 => ArrowDataType::Int64,
        RustType::Int128 => ArrowDataType::Int128,
        RustType::UInt64 => ArrowDataType::UInt64,
        RustType::UInt128 => ArrowDataType::UInt128,
        RustType::String => ArrowDataType::LargeUtf8,
        RustType::Bytes => ArrowDataType::Binary,
        RustType::Vec => ArrowDataType::List(Box::new(ArrowField::new(
            PlSmallStr::from("item"),
            ArrowDataType::Null,
            true,
        ))),
        RustType::Struct => ArrowDataType::Struct(Vec::new()),
        RustType::Time => ArrowDataType::Time64(ArrowTimeUnit::Microsecond),
        RustType::Null => ArrowDataType::Null,
        RustType::Object => ArrowDataType::Null,
    }
}

/// Return a stable string name for FFI use.
pub fn dtype_to_ffiname(dtype: &GDSDataType) -> Option<&'static str> {
    let name = match dtype {
        DataType::Int8 => "i8",
        DataType::Int16 => "i16",
        DataType::Int32 => "i32",
        DataType::Int64 => "i64",
        DataType::Int128 => "i128",
        DataType::UInt8 => "u8",
        DataType::UInt16 => "u16",
        DataType::UInt32 => "u32",
        DataType::UInt64 => "u64",
        DataType::UInt128 => "u128",
        DataType::Float32 => "f32",
        DataType::Float64 => "f64",
        DataType::Boolean => "bool",
        DataType::String => "str",
        DataType::Binary | DataType::BinaryOffset => "binary",
        DataType::Date => "date",
        DataType::Time => "time",
        DataType::Datetime(_, _) => "datetime",
        DataType::Duration(_) => "duration",
        DataType::List(_) => "list",
        DataType::Struct(_) => "struct",
        DataType::Object(_) => "object",
        DataType::Categorical(_, _) => "categorical",
        DataType::Enum(_, _) => "enum",
        DataType::Decimal(_, _) => "decimal",
        DataType::Null => "null",
        DataType::Unknown(_) => "unknown",
        _ => return None,
    };
    Some(name)
}

/// Convert a Polars dtype to an Arrow dtype (best-effort).
pub fn dtype_to_arrow_type(dtype: &GDSDataType) -> ArrowDataType {
    match dtype {
        DataType::Int8 => ArrowDataType::Int8,
        DataType::Int16 => ArrowDataType::Int16,
        DataType::Int32 => ArrowDataType::Int32,
        DataType::Int64 => ArrowDataType::Int64,
        DataType::Int128 => ArrowDataType::Int128,
        DataType::UInt8 => ArrowDataType::UInt8,
        DataType::UInt16 => ArrowDataType::UInt16,
        DataType::UInt32 => ArrowDataType::UInt32,
        DataType::UInt64 => ArrowDataType::UInt64,
        DataType::UInt128 => ArrowDataType::UInt128,
        DataType::Float32 => ArrowDataType::Float32,
        DataType::Float64 => ArrowDataType::Float64,
        DataType::Boolean => ArrowDataType::Boolean,
        DataType::String => ArrowDataType::Utf8,
        DataType::Binary | DataType::BinaryOffset => ArrowDataType::Binary,
        DataType::Date => ArrowDataType::Date32,
        DataType::Time => ArrowDataType::Time64(ArrowTimeUnit::Microsecond),
        DataType::Datetime(unit, _tz) => ArrowDataType::Timestamp(to_arrow_time_unit(*unit), None),
        DataType::Duration(unit) => ArrowDataType::Duration(to_arrow_time_unit(*unit)),
        DataType::List(inner) => ArrowDataType::List(Box::new(ArrowField::new(
            PlSmallStr::from("item"),
            dtype_to_arrow_type(inner),
            true,
        ))),
        DataType::Array(inner, width) => ArrowDataType::FixedSizeList(
            Box::new(ArrowField::new(
                PlSmallStr::from("item"),
                dtype_to_arrow_type(inner),
                true,
            )),
            *width,
        ),
        DataType::Struct(fields) => ArrowDataType::Struct(
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
        DataType::Decimal(precision, scale) => ArrowDataType::Decimal(*precision, *scale as usize),
        DataType::Null | DataType::Unknown(_) => ArrowDataType::Null,
        _ => ArrowDataType::Null,
    }
}

/// Map a Polars short repr (eg: "i64", "list[str]") back into a dtype.
pub fn dtype_short_repr_to_dtype(dtype_string: Option<&str>) -> Option<GDSDataType> {
    let raw = dtype_string?.trim();
    if raw.is_empty() {
        return None;
    }
    let (base, subtype) = split_short_repr(raw)?;
    let dtype_base = parse_into_dtype(base).ok()?;
    if let Some(subtype) = subtype {
        return apply_short_repr_subtype(dtype_base.clone(), subtype).or(Some(dtype_base));
    }
    Some(dtype_base)
}

/// Best-effort value casting helper (seed pass).
pub fn maybe_cast(value: AnyValue<'static>, _dtype: &GDSDataType) -> AnyValue<'static> {
    value
}

fn to_arrow_time_unit(unit: TimeUnit) -> ArrowTimeUnit {
    match unit {
        TimeUnit::Nanoseconds => ArrowTimeUnit::Nanosecond,
        TimeUnit::Microseconds => ArrowTimeUnit::Microsecond,
        TimeUnit::Milliseconds => ArrowTimeUnit::Millisecond,
    }
}

fn split_short_repr(raw: &str) -> Option<(&str, Option<&str>)> {
    let trimmed = raw.trim();
    if let Some(start) = trimmed.find('[') {
        if !trimmed.ends_with(']') {
            return None;
        }
        let base = trimmed[..start].trim();
        let subtype = trimmed[start + 1..trimmed.len() - 1].trim();
        return Some((base, Some(subtype)));
    }
    Some((trimmed, None))
}

fn apply_short_repr_subtype(dtype_base: GDSDataType, subtype: &str) -> Option<GDSDataType> {
    match dtype_base {
        DataType::Decimal(_, _) => {
            let scale = subtype.trim().parse::<usize>().ok()?;
            Some(DataType::Decimal(38, scale))
        }
        DataType::Datetime(_, _) => parse_datetime_subtype(subtype),
        DataType::Duration(_) => parse_duration_subtype(subtype),
        DataType::List(_) => parse_into_dtype(subtype)
            .ok()
            .map(|inner| DataType::List(Box::new(inner))),
        DataType::Array(_, _) => parse_array_subtype(subtype),
        DataType::Categorical(_, _) => parse_categorical_subtype(subtype),
        DataType::Enum(_, _) => parse_enum_subtype(subtype),
        _ => None,
    }
}

fn parse_datetime_subtype(subtype: &str) -> Option<GDSDataType> {
    let parts = split_subtype_parts(subtype);
    let unit = parse_time_unit(parts.first()?.as_str())?;
    let tz = parts
        .get(1)
        .and_then(|raw| normalize_subtype(raw))
        .and_then(|raw| TimeZone::opt_try_new(Some(raw)).ok().flatten());
    Some(DataType::Datetime(unit, tz))
}

fn parse_duration_subtype(subtype: &str) -> Option<GDSDataType> {
    let parts = split_subtype_parts(subtype);
    let unit = parse_time_unit(parts.first()?.as_str())?;
    Some(DataType::Duration(unit))
}

fn parse_array_subtype(subtype: &str) -> Option<GDSDataType> {
    let parts = split_subtype_parts(subtype);
    if parts.len() < 2 {
        return None;
    }
    let inner = parse_into_dtype(parts[0].as_str()).ok()?;
    let width = parts[1].trim().parse::<usize>().ok()?;
    Some(DataType::Array(Box::new(inner), width))
}

fn parse_enum_subtype(subtype: &str) -> Option<GDSDataType> {
    let categories = parse_enum_categories(subtype);
    EnumType::new(categories).to_dtype().ok()
}

fn parse_categorical_subtype(subtype: &str) -> Option<GDSDataType> {
    let formatted = format!("categorical[{subtype}]");
    parse_into_dtype(formatted.as_str()).ok()
}

fn parse_enum_categories(raw: &str) -> Vec<String> {
    raw.split(',')
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .map(|value| value.trim_matches('"').trim_matches('\''))
        .filter(|value| !value.is_empty())
        .map(String::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_enum_short_repr() {
        let dtype = dtype_short_repr_to_dtype(Some("enum[alpha,beta]")).expect("enum dtype");
        assert!(matches!(dtype, DataType::Enum(_, _)));
    }

    #[test]
    fn parses_categorical_short_repr() {
        let dtype = dtype_short_repr_to_dtype(Some("categorical[team, ns, u32]"))
            .expect("categorical dtype");
        assert!(matches!(dtype, DataType::Categorical(_, _)));
    }
}

fn split_subtype_parts(subtype: &str) -> Vec<String> {
    subtype
        .replace("\u{03bc}s", "us")
        .split(',')
        .map(|part| part.trim())
        .filter(|part| !part.is_empty())
        .map(|part| part.to_string())
        .collect()
}

fn normalize_subtype(raw: &str) -> Option<String> {
    let trimmed = raw.trim().trim_matches('"').trim_matches('\'');
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn parse_time_unit(raw: &str) -> Option<TimeUnit> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "ns" | "nanoseconds" => Some(TimeUnit::Nanoseconds),
        "us" | "microseconds" => Some(TimeUnit::Microseconds),
        "ms" | "milliseconds" => Some(TimeUnit::Milliseconds),
        _ => None,
    }
}
