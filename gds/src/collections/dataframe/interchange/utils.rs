//! Utilities for dataframe interchange.

use arrow::datatypes::{
    ArrowDataType, Field, IntegerType, TimeUnit as ArrowTimeUnit, UnionMode, UnionType,
};
use polars::prelude::{DataType, PlSmallStr, TimeUnit};

use crate::collections::dataframe::interchange::protocol::{
    Dtype, DtypeKind, Endianness, InterchangeError,
};

const NE: Endianness = Endianness::Native;

pub fn polars_dtype_to_dtype(dtype: &DataType) -> Result<Dtype, InterchangeError> {
    use DataType::*;

    let dtype = match dtype {
        Int8 => Dtype {
            kind: DtypeKind::Int,
            bit_width: 8,
            format: "c".to_string(),
            endianness: NE,
            children: Vec::new(),
            child_names: Vec::new(),
        },
        Int16 => Dtype {
            kind: DtypeKind::Int,
            bit_width: 16,
            format: "s".to_string(),
            endianness: NE,
            children: Vec::new(),
            child_names: Vec::new(),
        },
        Int32 => Dtype {
            kind: DtypeKind::Int,
            bit_width: 32,
            format: "i".to_string(),
            endianness: NE,
            children: Vec::new(),
            child_names: Vec::new(),
        },
        Int64 => Dtype {
            kind: DtypeKind::Int,
            bit_width: 64,
            format: "l".to_string(),
            endianness: NE,
            children: Vec::new(),
            child_names: Vec::new(),
        },
        UInt8 => Dtype {
            kind: DtypeKind::UInt,
            bit_width: 8,
            format: "C".to_string(),
            endianness: NE,
            children: Vec::new(),
            child_names: Vec::new(),
        },
        UInt16 => Dtype {
            kind: DtypeKind::UInt,
            bit_width: 16,
            format: "S".to_string(),
            endianness: NE,
            children: Vec::new(),
            child_names: Vec::new(),
        },
        UInt32 => Dtype {
            kind: DtypeKind::UInt,
            bit_width: 32,
            format: "I".to_string(),
            endianness: NE,
            children: Vec::new(),
            child_names: Vec::new(),
        },
        UInt64 => Dtype {
            kind: DtypeKind::UInt,
            bit_width: 64,
            format: "L".to_string(),
            endianness: NE,
            children: Vec::new(),
            child_names: Vec::new(),
        },
        Float32 => Dtype {
            kind: DtypeKind::Float,
            bit_width: 32,
            format: "f".to_string(),
            endianness: NE,
            children: Vec::new(),
            child_names: Vec::new(),
        },
        Float64 => Dtype {
            kind: DtypeKind::Float,
            bit_width: 64,
            format: "g".to_string(),
            endianness: NE,
            children: Vec::new(),
            child_names: Vec::new(),
        },
        Boolean => Dtype {
            kind: DtypeKind::Bool,
            bit_width: 1,
            format: "b".to_string(),
            endianness: NE,
            children: Vec::new(),
            child_names: Vec::new(),
        },
        Null => Dtype {
            kind: DtypeKind::Null,
            bit_width: 0,
            format: "null".to_string(),
            endianness: Endianness::NotApplicable,
            children: Vec::new(),
            child_names: Vec::new(),
        },
        String => Dtype {
            kind: DtypeKind::String,
            bit_width: 8,
            format: "U".to_string(),
            endianness: NE,
            children: Vec::new(),
            child_names: Vec::new(),
        },
        Date => Dtype {
            kind: DtypeKind::Datetime,
            bit_width: 32,
            format: "tdD".to_string(),
            endianness: NE,
            children: Vec::new(),
            child_names: Vec::new(),
        },
        Time => Dtype {
            kind: DtypeKind::Datetime,
            bit_width: 64,
            format: "ttu".to_string(),
            endianness: NE,
            children: Vec::new(),
            child_names: Vec::new(),
        },
        Datetime(time_unit, time_zone) => Dtype {
            kind: DtypeKind::Datetime,
            bit_width: 64,
            format: datetime_format(*time_unit, time_zone.as_ref().map(|tz| tz.as_str())),
            endianness: NE,
            children: Vec::new(),
            child_names: Vec::new(),
        },
        Duration(time_unit) => Dtype {
            kind: DtypeKind::Datetime,
            bit_width: 64,
            format: duration_format(*time_unit),
            endianness: NE,
            children: Vec::new(),
            child_names: Vec::new(),
        },
        Categorical(_, _) | Enum(_, _) => Dtype {
            kind: DtypeKind::Categorical,
            bit_width: 32,
            format: "I".to_string(),
            endianness: NE,
            children: Vec::new(),
            child_names: Vec::new(),
        },
        #[cfg(feature = "dtype-decimal")]
        Decimal(precision, scale) => Dtype {
            kind: DtypeKind::Decimal,
            bit_width: 128,
            format: format!("decimal:{precision}:{scale}"),
            endianness: NE,
            children: Vec::new(),
            child_names: Vec::new(),
        },
        Binary | BinaryOffset => Dtype {
            kind: DtypeKind::Binary,
            bit_width: 8,
            format: "B".to_string(),
            endianness: NE,
            children: Vec::new(),
            child_names: Vec::new(),
        },
        #[cfg(feature = "object")]
        Object(_) => Dtype {
            kind: DtypeKind::Object,
            bit_width: 0,
            format: "object".to_string(),
            endianness: Endianness::NotApplicable,
            children: Vec::new(),
            child_names: Vec::new(),
        },
        List(inner) => Dtype {
            kind: DtypeKind::List,
            bit_width: 64,
            format: "list".to_string(),
            endianness: NE,
            children: vec![polars_dtype_to_dtype(inner.as_ref())?],
            child_names: vec!["item".to_string()],
        },
        #[cfg(feature = "dtype-array")]
        Array(inner, size) => Dtype {
            kind: DtypeKind::Array,
            bit_width: 64,
            format: format!("array:{size}"),
            endianness: NE,
            children: vec![polars_dtype_to_dtype(inner.as_ref())?],
            child_names: vec!["item".to_string()],
        },
        Struct(fields) => Dtype {
            kind: DtypeKind::Struct,
            bit_width: 0,
            format: "struct".to_string(),
            endianness: Endianness::NotApplicable,
            children: fields
                .iter()
                .map(|field| polars_dtype_to_dtype(field.dtype()))
                .collect::<Result<Vec<_>, _>>()?,
            child_names: fields
                .iter()
                .map(|field| field.name().to_string())
                .collect(),
        },
        _ => return Err(InterchangeError::UnsupportedDataType(format!("{dtype:?}"))),
    };

    Ok(dtype)
}

fn datetime_format(time_unit: TimeUnit, time_zone: Option<&str>) -> String {
    let unit = time_unit_code(time_unit);
    let tz = time_zone.unwrap_or("");
    format!("ts{unit}:{tz}")
}

fn duration_format(time_unit: TimeUnit) -> String {
    let unit = time_unit_code(time_unit);
    format!("tD{unit}")
}

fn time_unit_code(time_unit: TimeUnit) -> &'static str {
    match time_unit {
        TimeUnit::Nanoseconds => "n",
        TimeUnit::Microseconds => "u",
        TimeUnit::Milliseconds => "m",
    }
}

pub fn get_buffer_length_in_elements(
    buffer_size: usize,
    dtype: &Dtype,
) -> Result<usize, InterchangeError> {
    let bits_per_element = dtype.bit_width;
    let bytes_per_element = bits_per_element / 8;
    if bits_per_element % 8 != 0 {
        return Err(InterchangeError::InvalidArgument(format!(
            "cannot get buffer length for dtype {dtype:?}"
        )));
    }
    Ok(buffer_size / bytes_per_element as usize)
}

pub fn polars_dtype_to_data_buffer_dtype(dtype: &DataType) -> Result<DataType, InterchangeError> {
    use DataType::*;

    let out = if dtype.is_integer() || dtype.is_float() || matches!(dtype, Boolean) {
        dtype.clone()
    } else if matches!(dtype, Null) {
        DataType::Null
    } else if dtype.is_decimal() {
        DataType::Int128
    } else if dtype.is_temporal() {
        if matches!(dtype, Date) {
            Int32
        } else {
            Int64
        }
    } else if matches!(dtype, String | Binary | BinaryOffset) {
        UInt8
    } else if matches!(dtype, Object(_)) {
        UInt8
    } else if matches!(dtype, Categorical(_, _) | Enum(_, _)) {
        UInt32
    } else {
        return Err(InterchangeError::UnsupportedDataType(format!("{dtype:?}")));
    };

    Ok(out)
}

pub fn dtype_to_arrow_datatype(dtype: &Dtype) -> Result<ArrowDataType, InterchangeError> {
    match dtype.kind {
        DtypeKind::Int => match dtype.bit_width {
            8 => Ok(ArrowDataType::Int8),
            16 => Ok(ArrowDataType::Int16),
            32 => Ok(ArrowDataType::Int32),
            64 => Ok(ArrowDataType::Int64),
            _ => Err(InterchangeError::UnsupportedDataType(format!("{dtype:?}"))),
        },
        DtypeKind::UInt => match dtype.bit_width {
            8 => Ok(ArrowDataType::UInt8),
            16 => Ok(ArrowDataType::UInt16),
            32 => Ok(ArrowDataType::UInt32),
            64 => Ok(ArrowDataType::UInt64),
            _ => Err(InterchangeError::UnsupportedDataType(format!("{dtype:?}"))),
        },
        DtypeKind::Float => match dtype.bit_width {
            16 => Ok(ArrowDataType::Float16),
            32 => Ok(ArrowDataType::Float32),
            64 => Ok(ArrowDataType::Float64),
            _ => Err(InterchangeError::UnsupportedDataType(format!("{dtype:?}"))),
        },
        DtypeKind::Bool => Ok(ArrowDataType::Boolean),
        DtypeKind::String => Ok(ArrowDataType::Utf8),
        DtypeKind::Datetime => parse_temporal_format(dtype),
        DtypeKind::Categorical => Ok(ArrowDataType::Dictionary(
            IntegerType::UInt32,
            Box::new(ArrowDataType::Utf8),
            false,
        )),
        DtypeKind::Object => {
            let size = parse_fixed_binary_size(&dtype.format)?;
            Ok(ArrowDataType::Extension(Box::new(
                arrow::datatypes::ExtensionType {
                    name: PlSmallStr::from_static("polars_object"),
                    inner: ArrowDataType::FixedSizeBinary(size),
                    metadata: None,
                },
            )))
        }
        DtypeKind::FixedSizeBinary => {
            let size = parse_fixed_binary_size(&dtype.format)?;
            Ok(ArrowDataType::FixedSizeBinary(size))
        }
        DtypeKind::Decimal => {
            let (precision, scale) = parse_decimal_format(&dtype.format)?;
            Ok(ArrowDataType::Decimal(precision, scale))
        }
        DtypeKind::Null => Ok(ArrowDataType::Null),
        DtypeKind::Binary => Ok(ArrowDataType::LargeBinary),
        DtypeKind::List => {
            let child = dtype.children.first().ok_or_else(|| {
                InterchangeError::InvalidArgument("list dtype missing child".to_string())
            })?;
            let child_dt = dtype_to_arrow_datatype(child)?;
            Ok(ArrowDataType::List(Box::new(Field::new(
                PlSmallStr::from_static("item"),
                child_dt,
                true,
            ))))
        }
        DtypeKind::Array => {
            let child = dtype.children.first().ok_or_else(|| {
                InterchangeError::InvalidArgument("array dtype missing child".to_string())
            })?;
            let size = parse_array_size(&dtype.format)?;
            let child_dt = dtype_to_arrow_datatype(child)?;
            Ok(ArrowDataType::FixedSizeList(
                Box::new(Field::new(PlSmallStr::from_static("item"), child_dt, true)),
                size,
            ))
        }
        DtypeKind::Struct => {
            let mut fields = Vec::with_capacity(dtype.children.len());
            for (idx, child) in dtype.children.iter().enumerate() {
                let name = dtype
                    .child_names
                    .get(idx)
                    .cloned()
                    .unwrap_or_else(|| format!("field_{idx}"));
                let child_dt = dtype_to_arrow_datatype(child)?;
                fields.push(Field::new(PlSmallStr::from(name), child_dt, true));
            }
            Ok(ArrowDataType::Struct(fields))
        }
        DtypeKind::Map => {
            if dtype.children.len() < 2 {
                return Err(InterchangeError::InvalidArgument(
                    "map dtype requires key/value children".to_string(),
                ));
            }
            let key_name = dtype
                .child_names
                .get(0)
                .cloned()
                .unwrap_or_else(|| "key".to_string());
            let value_name = dtype
                .child_names
                .get(1)
                .cloned()
                .unwrap_or_else(|| "value".to_string());
            let key_dt = dtype_to_arrow_datatype(&dtype.children[0])?;
            let value_dt = dtype_to_arrow_datatype(&dtype.children[1])?;
            let entries = ArrowDataType::Struct(vec![
                Field::new(PlSmallStr::from(key_name), key_dt, true),
                Field::new(PlSmallStr::from(value_name), value_dt, true),
            ]);
            Ok(ArrowDataType::Map(
                Box::new(Field::new(
                    PlSmallStr::from_static("entries"),
                    entries,
                    false,
                )),
                false,
            ))
        }
        DtypeKind::Union => {
            let mode = parse_union_mode(&dtype.format)?;
            let mut fields = Vec::with_capacity(dtype.children.len());
            for (idx, child) in dtype.children.iter().enumerate() {
                let name = dtype
                    .child_names
                    .get(idx)
                    .cloned()
                    .unwrap_or_else(|| format!("field_{idx}"));
                let child_dt = dtype_to_arrow_datatype(child)?;
                fields.push(Field::new(PlSmallStr::from(name), child_dt, true));
            }
            Ok(ArrowDataType::Union(Box::new(UnionType {
                fields,
                ids: None,
                mode,
            })))
        }
    }
}

pub fn arrow_datatype_to_dtype(dtype: &ArrowDataType) -> Result<Dtype, InterchangeError> {
    let dtype = match dtype.to_logical_type() {
        ArrowDataType::Int8 => Dtype {
            kind: DtypeKind::Int,
            bit_width: 8,
            format: "c".to_string(),
            endianness: NE,
            children: Vec::new(),
            child_names: Vec::new(),
        },
        ArrowDataType::Int16 => Dtype {
            kind: DtypeKind::Int,
            bit_width: 16,
            format: "s".to_string(),
            endianness: NE,
            children: Vec::new(),
            child_names: Vec::new(),
        },
        ArrowDataType::Int32 => Dtype {
            kind: DtypeKind::Int,
            bit_width: 32,
            format: "i".to_string(),
            endianness: NE,
            children: Vec::new(),
            child_names: Vec::new(),
        },
        ArrowDataType::Int64 => Dtype {
            kind: DtypeKind::Int,
            bit_width: 64,
            format: "l".to_string(),
            endianness: NE,
            children: Vec::new(),
            child_names: Vec::new(),
        },
        ArrowDataType::UInt8 => Dtype {
            kind: DtypeKind::UInt,
            bit_width: 8,
            format: "C".to_string(),
            endianness: NE,
            children: Vec::new(),
            child_names: Vec::new(),
        },
        ArrowDataType::UInt16 => Dtype {
            kind: DtypeKind::UInt,
            bit_width: 16,
            format: "S".to_string(),
            endianness: NE,
            children: Vec::new(),
            child_names: Vec::new(),
        },
        ArrowDataType::UInt32 => Dtype {
            kind: DtypeKind::UInt,
            bit_width: 32,
            format: "I".to_string(),
            endianness: NE,
            children: Vec::new(),
            child_names: Vec::new(),
        },
        ArrowDataType::UInt64 => Dtype {
            kind: DtypeKind::UInt,
            bit_width: 64,
            format: "L".to_string(),
            endianness: NE,
            children: Vec::new(),
            child_names: Vec::new(),
        },
        ArrowDataType::Float16 => Dtype {
            kind: DtypeKind::Float,
            bit_width: 16,
            format: "e".to_string(),
            endianness: NE,
            children: Vec::new(),
            child_names: Vec::new(),
        },
        ArrowDataType::Float32 => Dtype {
            kind: DtypeKind::Float,
            bit_width: 32,
            format: "f".to_string(),
            endianness: NE,
            children: Vec::new(),
            child_names: Vec::new(),
        },
        ArrowDataType::Float64 => Dtype {
            kind: DtypeKind::Float,
            bit_width: 64,
            format: "g".to_string(),
            endianness: NE,
            children: Vec::new(),
            child_names: Vec::new(),
        },
        ArrowDataType::Boolean => Dtype {
            kind: DtypeKind::Bool,
            bit_width: 1,
            format: "b".to_string(),
            endianness: NE,
            children: Vec::new(),
            child_names: Vec::new(),
        },
        ArrowDataType::Null => Dtype {
            kind: DtypeKind::Null,
            bit_width: 0,
            format: "null".to_string(),
            endianness: Endianness::NotApplicable,
            children: Vec::new(),
            child_names: Vec::new(),
        },
        ArrowDataType::Utf8 | ArrowDataType::LargeUtf8 | ArrowDataType::Utf8View => Dtype {
            kind: DtypeKind::String,
            bit_width: 8,
            format: "U".to_string(),
            endianness: NE,
            children: Vec::new(),
            child_names: Vec::new(),
        },
        ArrowDataType::Binary | ArrowDataType::LargeBinary | ArrowDataType::BinaryView => Dtype {
            kind: DtypeKind::Binary,
            bit_width: 8,
            format: "B".to_string(),
            endianness: NE,
            children: Vec::new(),
            child_names: Vec::new(),
        },
        ArrowDataType::FixedSizeBinary(size) => Dtype {
            kind: DtypeKind::FixedSizeBinary,
            bit_width: 8,
            format: format!("fixed_binary:{size}"),
            endianness: NE,
            children: Vec::new(),
            child_names: Vec::new(),
        },
        ArrowDataType::Date32 => Dtype {
            kind: DtypeKind::Datetime,
            bit_width: 32,
            format: "tdD".to_string(),
            endianness: NE,
            children: Vec::new(),
            child_names: Vec::new(),
        },
        ArrowDataType::Time64(unit) => Dtype {
            kind: DtypeKind::Datetime,
            bit_width: 64,
            format: format!("tt{}", arrow_time_unit_code(*unit)),
            endianness: NE,
            children: Vec::new(),
            child_names: Vec::new(),
        },
        ArrowDataType::Timestamp(unit, tz) => Dtype {
            kind: DtypeKind::Datetime,
            bit_width: 64,
            format: datetime_format_from_arrow(*unit, tz.as_deref()),
            endianness: NE,
            children: Vec::new(),
            child_names: Vec::new(),
        },
        ArrowDataType::Duration(unit) => Dtype {
            kind: DtypeKind::Datetime,
            bit_width: 64,
            format: duration_format_from_arrow(*unit),
            endianness: NE,
            children: Vec::new(),
            child_names: Vec::new(),
        },
        ArrowDataType::Decimal(precision, scale) => Dtype {
            kind: DtypeKind::Decimal,
            bit_width: 128,
            format: format!("decimal:{precision}:{scale}"),
            endianness: NE,
            children: Vec::new(),
            child_names: Vec::new(),
        },
        ArrowDataType::List(field) | ArrowDataType::LargeList(field) => Dtype {
            kind: DtypeKind::List,
            bit_width: 64,
            format: "list".to_string(),
            endianness: NE,
            children: vec![arrow_datatype_to_dtype(field.dtype())?],
            child_names: vec!["item".to_string()],
        },
        ArrowDataType::FixedSizeList(field, size) => Dtype {
            kind: DtypeKind::Array,
            bit_width: 64,
            format: format!("array:{size}"),
            endianness: NE,
            children: vec![arrow_datatype_to_dtype(field.dtype())?],
            child_names: vec!["item".to_string()],
        },
        ArrowDataType::Struct(fields) => {
            let mut children = Vec::with_capacity(fields.len());
            let mut child_names = Vec::with_capacity(fields.len());
            for field in fields.iter() {
                children.push(arrow_datatype_to_dtype(field.dtype())?);
                child_names.push(field.name.to_string());
            }
            Dtype {
                kind: DtypeKind::Struct,
                bit_width: 0,
                format: "struct".to_string(),
                endianness: Endianness::NotApplicable,
                children,
                child_names,
            }
        }
        ArrowDataType::Map(field, _is_sorted) => {
            let entry = field.dtype();
            let (key, value) = if let ArrowDataType::Struct(entries) = entry.to_logical_type() {
                let key = entries
                    .get(0)
                    .ok_or_else(|| {
                        InterchangeError::InvalidArgument(
                            "map entries missing key field".to_string(),
                        )
                    })?
                    .dtype();
                let value = entries
                    .get(1)
                    .ok_or_else(|| {
                        InterchangeError::InvalidArgument(
                            "map entries missing value field".to_string(),
                        )
                    })?
                    .dtype();
                (key, value)
            } else {
                return Err(InterchangeError::InvalidArgument(
                    "map entries must be struct".to_string(),
                ));
            };
            Dtype {
                kind: DtypeKind::Map,
                bit_width: 0,
                format: "map".to_string(),
                endianness: Endianness::NotApplicable,
                children: vec![
                    arrow_datatype_to_dtype(key)?,
                    arrow_datatype_to_dtype(value)?,
                ],
                child_names: vec!["key".to_string(), "value".to_string()],
            }
        }
        ArrowDataType::Union(union) => {
            let mut children = Vec::with_capacity(union.fields.len());
            let mut child_names = Vec::with_capacity(union.fields.len());
            for field in union.fields.iter() {
                children.push(arrow_datatype_to_dtype(field.dtype())?);
                child_names.push(field.name.to_string());
            }
            Dtype {
                kind: DtypeKind::Union,
                bit_width: 8,
                format: format!(
                    "union:{}",
                    if union.mode.is_sparse() {
                        "sparse"
                    } else {
                        "dense"
                    }
                ),
                endianness: Endianness::NotApplicable,
                children,
                child_names,
            }
        }
        ArrowDataType::Dictionary(key, value, _ordered) => {
            let is_utf8 = matches!(
                value.as_ref(),
                ArrowDataType::Utf8 | ArrowDataType::LargeUtf8
            );
            let is_u32 = matches!(key, IntegerType::UInt32);
            if is_utf8 && is_u32 {
                Dtype {
                    kind: DtypeKind::Categorical,
                    bit_width: 32,
                    format: "I".to_string(),
                    endianness: NE,
                    children: Vec::new(),
                    child_names: Vec::new(),
                }
            } else {
                return Err(InterchangeError::UnsupportedDataType(format!(
                    "dictionary {key:?}->{value:?}"
                )));
            }
        }
        ArrowDataType::Extension(ext) if ext.name.as_str() == "polars_object" => {
            if let ArrowDataType::FixedSizeBinary(size) = ext.inner {
                Dtype {
                    kind: DtypeKind::Object,
                    bit_width: 0,
                    format: format!("object:{size}"),
                    endianness: Endianness::NotApplicable,
                    children: Vec::new(),
                    child_names: Vec::new(),
                }
            } else {
                return Err(InterchangeError::UnsupportedDataType(
                    "polars_object extension inner dtype".to_string(),
                ));
            }
        }
        other => {
            return Err(InterchangeError::UnsupportedDataType(format!(
                "arrow dtype {other:?}"
            )))
        }
    };

    Ok(dtype)
}

fn parse_array_size(format: &str) -> Result<usize, InterchangeError> {
    let size = format.strip_prefix("array:").ok_or_else(|| {
        InterchangeError::InvalidArgument("array format missing size".to_string())
    })?;
    size.parse::<usize>().map_err(|_| {
        InterchangeError::InvalidArgument(format!("invalid array size in format: {format}"))
    })
}

fn parse_decimal_format(format: &str) -> Result<(usize, usize), InterchangeError> {
    let rest = format.strip_prefix("decimal:").ok_or_else(|| {
        InterchangeError::InvalidArgument("decimal format missing prefix".to_string())
    })?;
    let (precision, scale) = rest.split_once(':').ok_or_else(|| {
        InterchangeError::InvalidArgument("decimal format missing scale".to_string())
    })?;
    let precision = precision.parse::<usize>().map_err(|_| {
        InterchangeError::InvalidArgument(format!("invalid decimal precision in format: {format}"))
    })?;
    let scale = scale.parse::<usize>().map_err(|_| {
        InterchangeError::InvalidArgument(format!("invalid decimal scale in format: {format}"))
    })?;
    Ok((precision, scale))
}

fn parse_fixed_binary_size(format: &str) -> Result<usize, InterchangeError> {
    let rest = format
        .strip_prefix("fixed_binary:")
        .or_else(|| format.strip_prefix("object:"))
        .ok_or_else(|| {
            InterchangeError::InvalidArgument("fixed binary format missing size".to_string())
        })?;
    rest.parse::<usize>().map_err(|_| {
        InterchangeError::InvalidArgument(format!("invalid fixed binary size in format: {format}"))
    })
}

fn parse_union_mode(format: &str) -> Result<UnionMode, InterchangeError> {
    let mode = format
        .strip_prefix("union:")
        .unwrap_or(format)
        .to_ascii_lowercase();
    match mode.as_str() {
        "dense" | "union" => Ok(UnionMode::Dense),
        "sparse" => Ok(UnionMode::Sparse),
        _ => Err(InterchangeError::InvalidArgument(format!(
            "invalid union mode in format: {format}"
        ))),
    }
}

fn datetime_format_from_arrow(unit: ArrowTimeUnit, time_zone: Option<&str>) -> String {
    let unit = arrow_time_unit_code(unit);
    let tz = time_zone.unwrap_or("");
    format!("ts{unit}:{tz}")
}

fn duration_format_from_arrow(unit: ArrowTimeUnit) -> String {
    let unit = arrow_time_unit_code(unit);
    format!("tD{unit}")
}

fn arrow_time_unit_code(unit: ArrowTimeUnit) -> &'static str {
    match unit {
        ArrowTimeUnit::Nanosecond => "n",
        ArrowTimeUnit::Microsecond => "u",
        ArrowTimeUnit::Millisecond => "m",
        ArrowTimeUnit::Second => "s",
    }
}

fn parse_temporal_format(dtype: &Dtype) -> Result<ArrowDataType, InterchangeError> {
    let format = dtype.format.as_str();
    if let Some(rest) = format.strip_prefix("ts") {
        let (unit_str, tz) = rest.split_once(':').unwrap_or((rest, ""));
        let unit = parse_time_unit(unit_str)?;
        let tz = if tz.is_empty() {
            None
        } else {
            Some(PlSmallStr::from(tz))
        };
        return Ok(ArrowDataType::Timestamp(unit, tz));
    }
    if format == "tdD" {
        return Ok(ArrowDataType::Date32);
    }
    if let Some(unit_str) = format.strip_prefix("tt") {
        let unit = parse_time_unit(unit_str)?;
        return Ok(ArrowDataType::Time64(unit));
    }
    if let Some(unit_str) = format.strip_prefix("tD") {
        let unit = parse_time_unit(unit_str)?;
        return Ok(ArrowDataType::Duration(unit));
    }
    Err(InterchangeError::UnsupportedDataType(format!("{dtype:?}")))
}

fn parse_time_unit(unit_str: &str) -> Result<ArrowTimeUnit, InterchangeError> {
    match unit_str {
        "n" => Ok(ArrowTimeUnit::Nanosecond),
        "u" => Ok(ArrowTimeUnit::Microsecond),
        "m" => Ok(ArrowTimeUnit::Millisecond),
        "s" => Ok(ArrowTimeUnit::Second),
        _ => Err(InterchangeError::UnsupportedDataType(format!(
            "unknown time unit {unit_str}"
        ))),
    }
}
