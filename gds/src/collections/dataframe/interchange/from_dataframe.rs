//! Build Polars DataFrames from the interchange protocol.

use std::mem::size_of;

use arrow::array::{
    ArrayRef, BinaryArray, BooleanArray, DictionaryArray, FixedSizeBinaryArray, FixedSizeListArray,
    ListArray, MapArray, NullArray, PrimitiveArray, StructArray, UnionArray, Utf8Array,
};
use arrow::bitmap::Bitmap;
use arrow::buffer::Buffer as ArrowBuffer;
use arrow::datatypes::{ArrowDataType, Field};
use arrow::offset::OffsetsBuffer;
use polars::prelude::{DataFrame, PlSmallStr, Series};

use crate::collections::dataframe::interchange::protocol::{
    Column, ColumnBuffers, CopyNotAllowedError, DataFrame as InterchangeDataFrame, Dtype,
    DtypeKind, InterchangeError,
};
use crate::collections::dataframe::interchange::utils::dtype_to_arrow_datatype;

pub fn from_interchange(
    df: &dyn InterchangeDataFrame,
    allow_copy: bool,
) -> Result<DataFrame, InterchangeError> {
    let chunks = df.get_chunks(None)?;
    let mut out = Vec::new();

    if chunks.is_empty() {
        let chunk = df_to_polars(df, allow_copy)?;
        out.push(chunk);
    } else {
        for chunk in chunks {
            out.push(df_to_polars(chunk.as_ref(), allow_copy)?);
        }
    }

    let mut iter = out.into_iter();
    let mut acc = iter
        .next()
        .ok_or_else(|| InterchangeError::InvalidArgument("no chunks".to_string()))?;
    for df in iter {
        acc.vstack_mut(&df).map_err(InterchangeError::Polars)?;
    }
    Ok(acc)
}

fn df_to_polars(
    df: &dyn InterchangeDataFrame,
    allow_copy: bool,
) -> Result<DataFrame, InterchangeError> {
    let names = df.column_names()?;
    let mut columns = Vec::with_capacity(names.len());

    for (idx, name) in names.iter().enumerate() {
        let column = df.get_column(idx)?;
        let mut series = column_to_series(column.as_ref(), allow_copy)?;
        series.rename(PlSmallStr::from(name.as_str()));
        columns.push(series);
    }

    let columns = columns.into_iter().map(Into::into).collect();
    DataFrame::new(columns).map_err(InterchangeError::Polars)
}

fn column_to_series(column: &dyn Column, allow_copy: bool) -> Result<Series, InterchangeError> {
    let array = column_to_array(column, allow_copy)?;
    Series::from_arrow(PlSmallStr::from_static(""), array).map_err(InterchangeError::Polars)
}

fn column_to_array(column: &dyn Column, allow_copy: bool) -> Result<ArrayRef, InterchangeError> {
    let dtype = column.dtype()?;
    let buffers = column.get_buffers()?;
    let len = column.size();
    let offset = column.offset();

    match dtype.kind {
        DtypeKind::Bool => boolean_array(len, offset, buffers, allow_copy),
        DtypeKind::Int
        | DtypeKind::UInt
        | DtypeKind::Float
        | DtypeKind::Datetime
        | DtypeKind::Decimal => primitive_array(len, offset, dtype, buffers, allow_copy),
        DtypeKind::String => utf8_array(len, offset, buffers, allow_copy),
        DtypeKind::Binary => binary_array(len, offset, buffers, allow_copy),
        DtypeKind::FixedSizeBinary => {
            fixed_size_binary_array(len, offset, dtype, buffers, allow_copy)
        }
        DtypeKind::Null => Ok(NullArray::new(ArrowDataType::Null, len).boxed()),
        DtypeKind::Object => Err(InterchangeError::NotImplemented(
            "object import is not supported".to_string(),
        )),
        DtypeKind::Categorical => {
            categorical_array(len, offset, dtype, column, buffers, allow_copy)
        }
        DtypeKind::List => list_array(len, buffers, allow_copy),
        DtypeKind::Array => array_array(len, dtype, buffers, allow_copy),
        DtypeKind::Struct => struct_array(len, dtype, buffers, allow_copy),
        DtypeKind::Map => map_array(len, dtype, buffers, allow_copy),
        DtypeKind::Union => union_array(len, dtype, buffers, allow_copy),
    }
}

fn boolean_array(
    len: usize,
    offset: usize,
    buffers: ColumnBuffers,
    allow_copy: bool,
) -> Result<ArrayRef, InterchangeError> {
    let (data, _) = buffers
        .data
        .ok_or_else(|| InterchangeError::InvalidArgument("missing data buffer".to_string()))?;
    let mut values = bitmap_from_buffer(data.as_ref(), len, allow_copy)?;
    if offset > 0 {
        values = values.sliced(offset, len);
    }
    let validity = bitmap_from_validity(buffers.validity, len, allow_copy)?;
    Ok(BooleanArray::new(ArrowDataType::Boolean, values, validity).boxed())
}

fn primitive_array(
    len: usize,
    offset: usize,
    dtype: Dtype,
    buffers: ColumnBuffers,
    allow_copy: bool,
) -> Result<ArrayRef, InterchangeError> {
    let (data, _) = buffers
        .data
        .ok_or_else(|| InterchangeError::InvalidArgument("missing data buffer".to_string()))?;
    let validity = bitmap_from_validity(buffers.validity, len, allow_copy)?;
    let arrow_dtype = dtype_to_arrow_datatype(&dtype)?;

    macro_rules! primitive {
        ($t:ty) => {{
            let mut values = copy_buffer::<$t>(data.as_ref(), len, allow_copy)?;
            if offset > 0 {
                values.drain(0..offset);
            }
            let buffer: ArrowBuffer<$t> = values.into();
            PrimitiveArray::<$t>::try_new(arrow_dtype.clone(), buffer, validity)
                .map(|arr| arr.boxed())
                .map_err(InterchangeError::Polars)
        }};
    }

    match (dtype.kind, dtype.bit_width) {
        (DtypeKind::Int, 8) => primitive!(i8),
        (DtypeKind::Int, 16) => primitive!(i16),
        (DtypeKind::Int, 32) => primitive!(i32),
        (DtypeKind::Int, 64) => primitive!(i64),
        (DtypeKind::UInt, 8) => primitive!(u8),
        (DtypeKind::UInt, 16) => primitive!(u16),
        (DtypeKind::UInt, 32) => primitive!(u32),
        (DtypeKind::UInt, 64) => primitive!(u64),
        (DtypeKind::Float, 32) => primitive!(f32),
        (DtypeKind::Float, 64) => primitive!(f64),
        (DtypeKind::Decimal, 128) => primitive!(i128),
        _ => Err(InterchangeError::UnsupportedDataType(format!("{dtype:?}"))),
    }
}

fn utf8_array(
    len: usize,
    _offset: usize,
    buffers: ColumnBuffers,
    allow_copy: bool,
) -> Result<ArrayRef, InterchangeError> {
    let (data, _) = buffers
        .data
        .ok_or_else(|| InterchangeError::InvalidArgument("missing data buffer".to_string()))?;
    let (offsets, offsets_dtype) = buffers
        .offsets
        .ok_or_else(|| InterchangeError::InvalidArgument("missing offsets buffer".to_string()))?;
    let validity = bitmap_from_validity(buffers.validity, len, allow_copy)?;
    let data_bytes = copy_bytes(data.as_ref(), allow_copy)?;

    match offsets_dtype.bit_width {
        32 => {
            let offsets = offsets_from_buffer::<i32>(offsets.as_ref(), allow_copy)?;
            let offsets =
                OffsetsBuffer::<i32>::try_from(offsets).map_err(InterchangeError::Polars)?;
            let values: ArrowBuffer<u8> = data_bytes.into();
            Utf8Array::<i32>::try_new(ArrowDataType::Utf8, offsets, values, validity)
                .map(|arr| arr.boxed())
                .map_err(InterchangeError::Polars)
        }
        _ => {
            let offsets = offsets_from_buffer::<i64>(offsets.as_ref(), allow_copy)?;
            let offsets =
                OffsetsBuffer::<i64>::try_from(offsets).map_err(InterchangeError::Polars)?;
            let values: ArrowBuffer<u8> = data_bytes.into();
            Utf8Array::<i64>::try_new(ArrowDataType::LargeUtf8, offsets, values, validity)
                .map(|arr| arr.boxed())
                .map_err(InterchangeError::Polars)
        }
    }
}

fn list_array(
    len: usize,
    buffers: ColumnBuffers,
    allow_copy: bool,
) -> Result<ArrayRef, InterchangeError> {
    let child = buffers
        .children
        .into_iter()
        .next()
        .ok_or_else(|| InterchangeError::InvalidArgument("missing list child".to_string()))?;
    let child_array = column_to_array(child.as_ref(), allow_copy)?;
    let validity = bitmap_from_validity(buffers.validity, len, allow_copy)?;

    let (offsets, offsets_dtype) = match buffers.offsets {
        Some(offsets) => offsets,
        None => {
            let size = if len == 0 { 0 } else { child_array.len() / len };
            let field = Field::new(
                PlSmallStr::from_static("item"),
                child_array.dtype().clone(),
                true,
            );
            let dtype = ArrowDataType::FixedSizeList(Box::new(field), size);
            return FixedSizeListArray::try_new(dtype, len, child_array, validity)
                .map(|arr| arr.boxed())
                .map_err(InterchangeError::Polars);
        }
    };

    let field = Field::new(
        PlSmallStr::from_static("item"),
        child_array.dtype().clone(),
        true,
    );
    let dtype = match offsets_dtype.bit_width {
        32 => ArrowDataType::List(Box::new(field)),
        _ => ArrowDataType::LargeList(Box::new(field)),
    };

    match offsets_dtype.bit_width {
        32 => {
            let offsets = offsets_from_buffer::<i32>(offsets.as_ref(), allow_copy)?;
            let offsets =
                OffsetsBuffer::<i32>::try_from(offsets).map_err(InterchangeError::Polars)?;
            ListArray::<i32>::try_new(dtype, offsets, child_array, validity)
                .map(|arr| arr.boxed())
                .map_err(InterchangeError::Polars)
        }
        _ => {
            let offsets = offsets_from_buffer::<i64>(offsets.as_ref(), allow_copy)?;
            let offsets =
                OffsetsBuffer::<i64>::try_from(offsets).map_err(InterchangeError::Polars)?;
            ListArray::<i64>::try_new(dtype, offsets, child_array, validity)
                .map(|arr| arr.boxed())
                .map_err(InterchangeError::Polars)
        }
    }
}

fn map_array(
    len: usize,
    dtype: Dtype,
    buffers: ColumnBuffers,
    allow_copy: bool,
) -> Result<ArrayRef, InterchangeError> {
    if buffers.children.len() < 2 {
        return Err(InterchangeError::InvalidArgument(
            "map requires key/value children".to_string(),
        ));
    }
    let (offsets, offsets_dtype) = buffers
        .offsets
        .ok_or_else(|| InterchangeError::InvalidArgument("missing offsets buffer".to_string()))?;
    let validity = bitmap_from_validity(buffers.validity, len, allow_copy)?;

    let mut fields = Vec::with_capacity(2);
    let mut values = Vec::with_capacity(2);
    for (idx, child) in buffers.children.iter().take(2).enumerate() {
        let name = dtype.child_names.get(idx).cloned().unwrap_or_else(|| {
            if idx == 0 {
                "key".to_string()
            } else {
                "value".to_string()
            }
        });
        let child_array = column_to_array(child.as_ref(), allow_copy)?;
        fields.push(Field::new(
            PlSmallStr::from(name),
            child_array.dtype().clone(),
            true,
        ));
        values.push(child_array);
    }

    let entries_len = values.first().map(|array| array.len()).unwrap_or(0);
    if values.iter().any(|array| array.len() != entries_len) {
        return Err(InterchangeError::InvalidArgument(
            "map entry arrays must have equal length".to_string(),
        ));
    }

    let entries_dtype = ArrowDataType::Struct(fields.clone());
    let entries_array = StructArray::try_new(entries_dtype.clone(), entries_len, values, None)
        .map_err(InterchangeError::Polars)?;
    let entries_field = Field::new(PlSmallStr::from_static("entries"), entries_dtype, false);

    let offsets = match offsets_dtype.bit_width {
        32 => {
            let offsets = offsets_from_buffer::<i32>(offsets.as_ref(), allow_copy)?;
            OffsetsBuffer::<i32>::try_from(offsets).map_err(InterchangeError::Polars)?
        }
        _ => {
            let offsets = offsets_from_buffer::<i64>(offsets.as_ref(), allow_copy)?;
            let offsets = offsets
                .into_iter()
                .map(|value| {
                    i32::try_from(value).map_err(|_| {
                        InterchangeError::InvalidArgument(
                            "map offsets exceed i32 range".to_string(),
                        )
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            OffsetsBuffer::<i32>::try_from(offsets).map_err(InterchangeError::Polars)?
        }
    };

    let dtype = ArrowDataType::Map(Box::new(entries_field), false);
    MapArray::try_new(dtype, offsets, Box::new(entries_array), validity)
        .map(|arr| arr.boxed())
        .map_err(InterchangeError::Polars)
}

fn union_array(
    len: usize,
    dtype: Dtype,
    buffers: ColumnBuffers,
    allow_copy: bool,
) -> Result<ArrayRef, InterchangeError> {
    let (types, types_dtype) = buffers
        .data
        .ok_or_else(|| InterchangeError::InvalidArgument("missing type id buffer".to_string()))?;
    let type_ids: Vec<i8> = match (types_dtype.kind, types_dtype.bit_width) {
        (DtypeKind::Int, 8) => copy_buffer::<i8>(types.as_ref(), len, allow_copy)?,
        (DtypeKind::UInt, 8) => copy_buffer::<u8>(types.as_ref(), len, allow_copy)?
            .into_iter()
            .map(|value| {
                i8::try_from(value).map_err(|_| {
                    InterchangeError::InvalidArgument("union type id exceeds i8 range".to_string())
                })
            })
            .collect::<Result<Vec<_>, _>>()?,
        _ => {
            return Err(InterchangeError::InvalidArgument(
                "union type ids must be 8-bit integers".to_string(),
            ))
        }
    };

    let arrow_dtype = dtype_to_arrow_datatype(&dtype)?;
    let union_mode = match &arrow_dtype {
        ArrowDataType::Union(union) => union.mode,
        _ => {
            return Err(InterchangeError::InvalidArgument(
                "union dtype did not resolve to Arrow union".to_string(),
            ))
        }
    };

    let offsets = match union_mode {
        arrow::datatypes::UnionMode::Dense => {
            let (offsets, offsets_dtype) = buffers.offsets.ok_or_else(|| {
                InterchangeError::InvalidArgument("dense union missing offsets buffer".to_string())
            })?;
            let offsets = match offsets_dtype.bit_width {
                32 => offsets_from_buffer::<i32>(offsets.as_ref(), allow_copy)?,
                _ => offsets_from_buffer::<i64>(offsets.as_ref(), allow_copy)?
                    .into_iter()
                    .map(|value| {
                        i32::try_from(value).map_err(|_| {
                            InterchangeError::InvalidArgument(
                                "union offsets exceed i32 range".to_string(),
                            )
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            };
            Some(offsets.into())
        }
        arrow::datatypes::UnionMode::Sparse => None,
    };

    let mut children = Vec::with_capacity(buffers.children.len());
    for child in buffers.children.iter() {
        children.push(column_to_array(child.as_ref(), allow_copy)?);
    }

    let types_buffer: ArrowBuffer<i8> = type_ids.into();
    UnionArray::try_new(arrow_dtype, types_buffer, children, offsets)
        .map(|arr| arr.boxed())
        .map_err(InterchangeError::Polars)
}

fn array_array(
    len: usize,
    dtype: Dtype,
    buffers: ColumnBuffers,
    allow_copy: bool,
) -> Result<ArrayRef, InterchangeError> {
    let child = buffers
        .children
        .into_iter()
        .next()
        .ok_or_else(|| InterchangeError::InvalidArgument("missing array child".to_string()))?;
    let child_array = column_to_array(child.as_ref(), allow_copy)?;
    let validity = bitmap_from_validity(buffers.validity, len, allow_copy)?;
    let arrow_dtype = dtype_to_arrow_datatype(&dtype)?;
    FixedSizeListArray::try_new(arrow_dtype, len, child_array, validity)
        .map(|arr| arr.boxed())
        .map_err(InterchangeError::Polars)
}

fn binary_array(
    len: usize,
    _offset: usize,
    buffers: ColumnBuffers,
    allow_copy: bool,
) -> Result<ArrayRef, InterchangeError> {
    let (data, _) = buffers
        .data
        .ok_or_else(|| InterchangeError::InvalidArgument("missing data buffer".to_string()))?;
    let (offsets, offsets_dtype) = buffers
        .offsets
        .ok_or_else(|| InterchangeError::InvalidArgument("missing offsets buffer".to_string()))?;
    let validity = bitmap_from_validity(buffers.validity, len, allow_copy)?;
    let data_bytes = copy_bytes(data.as_ref(), allow_copy)?;

    match offsets_dtype.bit_width {
        32 => {
            let offsets = offsets_from_buffer::<i32>(offsets.as_ref(), allow_copy)?;
            let offsets =
                OffsetsBuffer::<i32>::try_from(offsets).map_err(InterchangeError::Polars)?;
            let values: ArrowBuffer<u8> = data_bytes.into();
            BinaryArray::<i32>::try_new(ArrowDataType::Binary, offsets, values, validity)
                .map(|arr| arr.boxed())
                .map_err(InterchangeError::Polars)
        }
        _ => {
            let offsets = offsets_from_buffer::<i64>(offsets.as_ref(), allow_copy)?;
            let offsets =
                OffsetsBuffer::<i64>::try_from(offsets).map_err(InterchangeError::Polars)?;
            let values: ArrowBuffer<u8> = data_bytes.into();
            BinaryArray::<i64>::try_new(ArrowDataType::LargeBinary, offsets, values, validity)
                .map(|arr| arr.boxed())
                .map_err(InterchangeError::Polars)
        }
    }
}

fn fixed_size_binary_array(
    len: usize,
    offset: usize,
    dtype: Dtype,
    buffers: ColumnBuffers,
    allow_copy: bool,
) -> Result<ArrayRef, InterchangeError> {
    let (data, _) = buffers
        .data
        .ok_or_else(|| InterchangeError::InvalidArgument("missing data buffer".to_string()))?;
    let validity = bitmap_from_validity(buffers.validity, len, allow_copy)?;
    let size = parse_fixed_binary_size(&dtype.format)?;
    let mut values = copy_bytes(data.as_ref(), allow_copy)?;
    if offset > 0 {
        let byte_offset = offset * size;
        if byte_offset <= values.len() {
            values.drain(0..byte_offset);
        }
    }
    let buffer: ArrowBuffer<u8> = values.into();
    FixedSizeBinaryArray::try_new(ArrowDataType::FixedSizeBinary(size), buffer, validity)
        .map(|arr| arr.boxed())
        .map_err(InterchangeError::Polars)
}

fn struct_array(
    len: usize,
    dtype: Dtype,
    buffers: ColumnBuffers,
    allow_copy: bool,
) -> Result<ArrayRef, InterchangeError> {
    let mut fields = Vec::with_capacity(dtype.children.len());
    let mut values = Vec::with_capacity(dtype.children.len());

    for (idx, child) in buffers.children.iter().enumerate() {
        let name = dtype
            .child_names
            .get(idx)
            .cloned()
            .unwrap_or_else(|| format!("field_{idx}"));
        let child_array = column_to_array(child.as_ref(), allow_copy)?;
        fields.push(Field::new(
            PlSmallStr::from(name),
            child_array.dtype().clone(),
            true,
        ));
        values.push(child_array);
    }

    let validity = bitmap_from_validity(buffers.validity, len, allow_copy)?;
    StructArray::try_new(ArrowDataType::Struct(fields), len, values, validity)
        .map(|arr| arr.boxed())
        .map_err(InterchangeError::Polars)
}

fn categorical_array(
    len: usize,
    offset: usize,
    dtype: Dtype,
    column: &dyn Column,
    buffers: ColumnBuffers,
    allow_copy: bool,
) -> Result<ArrayRef, InterchangeError> {
    let categories = column.describe_categorical()?;
    let categories_array = column_to_array(categories.categories.as_ref(), allow_copy)?;
    let arrow_dtype = dtype_to_arrow_datatype(&dtype)?;

    let (data, _) = buffers
        .data
        .ok_or_else(|| InterchangeError::InvalidArgument("missing data buffer".to_string()))?;
    let validity = bitmap_from_validity(buffers.validity, len, allow_copy)?;

    let mut keys = copy_buffer::<u32>(data.as_ref(), len, allow_copy)?;
    if offset > 0 {
        keys.drain(0..offset);
    }
    let key_buffer: ArrowBuffer<u32> = keys.into();
    let keys = PrimitiveArray::<u32>::try_new(ArrowDataType::UInt32, key_buffer, validity)
        .map_err(InterchangeError::Polars)?;

    DictionaryArray::<u32>::try_new(arrow_dtype, keys, categories_array)
        .map(|arr| arr.boxed())
        .map_err(InterchangeError::Polars)
}

fn bitmap_from_validity(
    validity: Option<(
        Box<dyn crate::collections::dataframe::interchange::protocol::Buffer>,
        Dtype,
    )>,
    len: usize,
    allow_copy: bool,
) -> Result<Option<Bitmap>, InterchangeError> {
    match validity {
        None => Ok(None),
        Some((buffer, _dtype)) => Ok(Some(bitmap_from_buffer(buffer.as_ref(), len, allow_copy)?)),
    }
}

fn bitmap_from_buffer(
    buffer: &dyn crate::collections::dataframe::interchange::protocol::Buffer,
    len: usize,
    allow_copy: bool,
) -> Result<Bitmap, InterchangeError> {
    let bytes = copy_bytes(buffer, allow_copy)?;
    Ok(Bitmap::from_u8_vec(bytes, len))
}

fn offsets_from_buffer<T: Copy>(
    buffer: &dyn crate::collections::dataframe::interchange::protocol::Buffer,
    allow_copy: bool,
) -> Result<Vec<T>, InterchangeError> {
    let len = buffer.bufsize() / size_of::<T>();
    copy_buffer(buffer, len, allow_copy)
}

fn copy_bytes(
    buffer: &dyn crate::collections::dataframe::interchange::protocol::Buffer,
    allow_copy: bool,
) -> Result<Vec<u8>, InterchangeError> {
    if !allow_copy {
        return Err(InterchangeError::CopyNotAllowed(
            "copy required to import buffer".to_string(),
        ));
    }
    unsafe { Ok(std::slice::from_raw_parts(buffer.ptr(), buffer.bufsize()).to_vec()) }
}

fn copy_buffer<T: Copy>(
    buffer: &dyn crate::collections::dataframe::interchange::protocol::Buffer,
    len: usize,
    allow_copy: bool,
) -> Result<Vec<T>, InterchangeError> {
    if !allow_copy {
        return Err(CopyNotAllowedError("copy required to import buffer".to_string()).into());
    }
    let mut values = Vec::with_capacity(len);
    unsafe {
        values.set_len(len);
        std::ptr::copy_nonoverlapping(buffer.ptr() as *const T, values.as_mut_ptr(), len);
    }
    Ok(values)
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
