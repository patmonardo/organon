//! Column interchange wrapper for Polars Series.

use arrow::array::{
    Array, ArrayRef, BinaryArray, BooleanArray, FixedSizeBinaryArray, FixedSizeListArray,
    ListArray, MapArray, PrimitiveArray, StructArray, UnionArray, Utf8Array,
};
use arrow::datatypes::{ArrowDataType, PhysicalType, PrimitiveType};
use arrow::offset::Offset;
use polars::prelude::{CompatLevel, DataType, PlSmallStr, Series};
use std::mem::size_of;

use crate::collections::dataframe::interchange::buffer::PolarsInterchangeBuffer;
use crate::collections::dataframe::interchange::protocol::{
    Buffer, CategoricalDescription, Column, ColumnBuffers, ColumnNullType, CopyNotAllowedError,
    InterchangeError, MetadataMap,
};
use crate::collections::dataframe::interchange::utils::{
    arrow_datatype_to_dtype, polars_dtype_to_data_buffer_dtype, polars_dtype_to_dtype,
};

#[derive(Clone)]
pub struct PolarsInterchangeColumn {
    column: Series,
    allow_copy: bool,
}

impl PolarsInterchangeColumn {
    pub fn new(column: Series, allow_copy: bool) -> Self {
        Self { column, allow_copy }
    }

    pub fn series(&self) -> &Series {
        &self.column
    }

    fn ensure_contiguous(&self) -> Result<Series, InterchangeError> {
        if self.column.n_chunks() > 1 {
            if !self.allow_copy {
                return Err(CopyNotAllowedError(
                    "non-contiguous column must be rechunked".to_string(),
                )
                .into());
            }
            Ok(self.column.rechunk())
        } else {
            Ok(self.column.clone())
        }
    }

    fn data_buffers(
        &self,
        array: &(dyn Array + 'static),
        data_dtype: &DataType,
    ) -> Result<ColumnBuffers, InterchangeError> {
        let data_dtype = polars_dtype_to_dtype(data_dtype)?;
        let validity = array
            .validity()
            .cloned()
            .map(|bitmap| -> Result<_, InterchangeError> {
                let buffer = PolarsInterchangeBuffer::from_bitmap(bitmap)?;
                Ok((
                    Box::new(buffer) as Box<dyn Buffer>,
                    polars_dtype_to_dtype(&DataType::Boolean)?,
                ))
            })
            .transpose()?;

        match array.dtype().to_physical_type() {
            PhysicalType::Primitive(primitive) => {
                let (buffer, dtype) = primitive_data_buffers(array, primitive, data_dtype)?;
                Ok(ColumnBuffers {
                    data: Some((buffer, dtype)),
                    validity,
                    offsets: None,
                    children: Vec::new(),
                })
            }
            PhysicalType::Boolean => {
                let array = array
                    .as_any()
                    .downcast_ref::<BooleanArray>()
                    .ok_or_else(|| {
                        InterchangeError::NotImplemented("boolean downcast".to_string())
                    })?;
                let buffer = PolarsInterchangeBuffer::from_bitmap(array.values().clone())?;
                Ok(ColumnBuffers {
                    data: Some((Box::new(buffer) as Box<dyn Buffer>, data_dtype)),
                    validity,
                    offsets: None,
                    children: Vec::new(),
                })
            }
            PhysicalType::Utf8 => {
                let (data, offsets) = utf8_buffers(array)?;
                Ok(ColumnBuffers {
                    data: Some((data, data_dtype)),
                    validity,
                    offsets: Some(offsets),
                    children: Vec::new(),
                })
            }
            PhysicalType::Binary => {
                let (data, offsets) = binary_buffers(array)?;
                Ok(ColumnBuffers {
                    data: Some((data, data_dtype)),
                    validity,
                    offsets: Some(offsets),
                    children: Vec::new(),
                })
            }
            PhysicalType::List => list_buffers(array, validity, self.allow_copy),
            PhysicalType::LargeList => list_buffers(array, validity, self.allow_copy),
            PhysicalType::FixedSizeList => {
                fixed_size_list_buffers(array, validity, self.allow_copy)
            }
            PhysicalType::FixedSizeBinary => fixed_size_binary_buffers(array, validity),
            PhysicalType::Struct => struct_buffers(array, validity, self.allow_copy),
            PhysicalType::Map => map_buffers(array, validity, self.allow_copy),
            PhysicalType::Union => union_buffers(array, validity, self.allow_copy),
            PhysicalType::Null => Ok(ColumnBuffers {
                data: None,
                validity: None,
                offsets: None,
                children: Vec::new(),
            }),
            _ => Err(InterchangeError::NotImplemented(format!(
                "unsupported physical type {:?}",
                array.dtype().to_physical_type()
            ))),
        }
    }

    fn bool_offset(array: &(dyn Array + 'static)) -> Result<usize, InterchangeError> {
        if let Some(bool_array) = array.as_any().downcast_ref::<BooleanArray>() {
            let (_bytes, offset_bits, _length_bits) = bool_array.values().as_slice();
            Ok(offset_bits)
        } else {
            Ok(0)
        }
    }
}

impl Column for PolarsInterchangeColumn {
    fn size(&self) -> usize {
        self.column.len()
    }

    fn offset(&self) -> usize {
        if let Ok(series) = self.ensure_contiguous() {
            let array = series.to_arrow(0, CompatLevel::newest());
            let array_ref = array.as_ref() as &(dyn Array + 'static);
            Self::bool_offset(array_ref).unwrap_or(0)
        } else {
            0
        }
    }

    fn dtype(
        &self,
    ) -> Result<crate::collections::dataframe::interchange::protocol::Dtype, InterchangeError> {
        let series = self.ensure_contiguous()?;
        let array = series.to_arrow(0, CompatLevel::newest());
        let logical = array.dtype().to_logical_type();

        if matches!(logical, ArrowDataType::Union(_) | ArrowDataType::Map(_, _)) {
            return arrow_datatype_to_dtype(&logical);
        }

        if matches!(self.column.dtype(), DataType::Object(_)) {
            let array = array
                .as_any()
                .downcast_ref::<FixedSizeBinaryArray>()
                .ok_or_else(|| {
                    InterchangeError::NotImplemented("object to fixed-size binary".to_string())
                })?;
            let mut dtype = polars_dtype_to_dtype(self.column.dtype())?;
            dtype.format = format!("fixed_binary:{}", array.size());
            Ok(dtype)
        } else {
            polars_dtype_to_dtype(self.column.dtype())
        }
    }

    fn describe_categorical(&self) -> Result<CategoricalDescription, InterchangeError> {
        let dtype = self.column.dtype();
        if !matches!(dtype, DataType::Categorical(_, _) | DataType::Enum(_, _)) {
            return Err(InterchangeError::InvalidArgument(
                "describe_categorical only applies to categorical columns".to_string(),
            ));
        }
        let mapping = dtype
            .cat_mapping()
            .map_err(|err| InterchangeError::InvalidArgument(err.to_string()))?;
        let categories_array = mapping.to_arrow(false);
        let categories_series =
            series_from_array(PlSmallStr::from_static("categories"), categories_array)?;
        let is_ordered = matches!(dtype, DataType::Enum(_, _));
        Ok(CategoricalDescription {
            is_ordered,
            is_dictionary: true,
            categories: Box::new(PolarsInterchangeColumn::new(
                categories_series,
                self.allow_copy,
            )),
        })
    }

    fn describe_null(&self) -> Result<(ColumnNullType, Option<i64>), InterchangeError> {
        let nulls = self.column.null_count();
        if nulls == 0 {
            Ok((ColumnNullType::NonNullable, None))
        } else {
            Ok((ColumnNullType::UseBitmask, Some(0)))
        }
    }

    fn null_count(&self) -> Result<Option<usize>, InterchangeError> {
        Ok(Some(self.column.null_count()))
    }

    fn metadata(&self) -> Result<MetadataMap, InterchangeError> {
        Ok(MetadataMap::new())
    }

    fn num_chunks(&self) -> Result<usize, InterchangeError> {
        Ok(self.column.n_chunks())
    }

    fn get_chunks(
        &self,
        n_chunks: Option<usize>,
    ) -> Result<Vec<Box<dyn Column>>, InterchangeError> {
        let total_n_chunks = self.column.n_chunks();
        let mut chunks = Vec::new();

        if total_n_chunks == 0 {
            return Ok(chunks);
        }

        let chunk_sizes: Vec<usize> = self
            .column
            .chunks()
            .iter()
            .map(|chunk| chunk.len())
            .collect();

        if n_chunks.is_none() || n_chunks == Some(total_n_chunks) {
            let mut offset = 0usize;
            for size in chunk_sizes {
                let chunk = self.column.slice(offset as i64, size);
                chunks.push(
                    Box::new(PolarsInterchangeColumn::new(chunk, self.allow_copy))
                        as Box<dyn Column>,
                );
                offset += size;
            }
            return Ok(chunks);
        }

        let n_chunks = n_chunks.unwrap_or(total_n_chunks);
        if n_chunks == 0 || n_chunks % total_n_chunks != 0 {
            return Err(InterchangeError::InvalidArgument(format!(
                "n_chunks must be a multiple of the number of chunks ({total_n_chunks})"
            )));
        }

        let subchunks_per_chunk = n_chunks / total_n_chunks;
        let mut offset = 0usize;
        for size in chunk_sizes {
            let mut step = size / subchunks_per_chunk;
            if size % subchunks_per_chunk != 0 {
                step += 1;
            }
            for start in (0..step * subchunks_per_chunk).step_by(step) {
                let end = (start + step).min(size);
                if start >= end {
                    continue;
                }
                let chunk = self.column.slice((offset + start) as i64, end - start);
                chunks.push(
                    Box::new(PolarsInterchangeColumn::new(chunk, self.allow_copy))
                        as Box<dyn Column>,
                );
            }
            offset += size;
        }

        Ok(chunks)
    }

    fn get_buffers(&self) -> Result<ColumnBuffers, InterchangeError> {
        let series = self.ensure_contiguous()?;
        let array = series.to_arrow(0, CompatLevel::newest());
        let data_dtype = match series.dtype() {
            DataType::List(_) | DataType::Struct(_) => series.dtype().clone(),
            #[cfg(feature = "dtype-array")]
            DataType::Array(_, _) => series.dtype().clone(),
            _ if matches!(array.dtype().to_physical_type(), PhysicalType::Union) => DataType::Int8,
            _ => polars_dtype_to_data_buffer_dtype(series.dtype())?,
        };
        let array_ref = array.as_ref() as &(dyn Array + 'static);
        self.data_buffers(array_ref, &data_dtype)
    }
}

fn primitive_data_buffers(
    array: &dyn Array,
    primitive: PrimitiveType,
    data_dtype: crate::collections::dataframe::interchange::protocol::Dtype,
) -> Result<
    (
        Box<dyn Buffer>,
        crate::collections::dataframe::interchange::protocol::Dtype,
    ),
    InterchangeError,
> {
    let buffer = match primitive {
        PrimitiveType::Int8 => {
            let array = array
                .as_any()
                .downcast_ref::<PrimitiveArray<i8>>()
                .ok_or_else(|| InterchangeError::NotImplemented("int8 downcast".to_string()))?;
            PolarsInterchangeBuffer::from_buffer(array.values().clone())?
        }
        PrimitiveType::Int16 => {
            let array = array
                .as_any()
                .downcast_ref::<PrimitiveArray<i16>>()
                .ok_or_else(|| InterchangeError::NotImplemented("int16 downcast".to_string()))?;
            PolarsInterchangeBuffer::from_buffer(array.values().clone())?
        }
        PrimitiveType::Int32 => {
            let array = array
                .as_any()
                .downcast_ref::<PrimitiveArray<i32>>()
                .ok_or_else(|| InterchangeError::NotImplemented("int32 downcast".to_string()))?;
            PolarsInterchangeBuffer::from_buffer(array.values().clone())?
        }
        PrimitiveType::Int64 => {
            let array = array
                .as_any()
                .downcast_ref::<PrimitiveArray<i64>>()
                .ok_or_else(|| InterchangeError::NotImplemented("int64 downcast".to_string()))?;
            PolarsInterchangeBuffer::from_buffer(array.values().clone())?
        }
        PrimitiveType::UInt8 => {
            let array = array
                .as_any()
                .downcast_ref::<PrimitiveArray<u8>>()
                .ok_or_else(|| InterchangeError::NotImplemented("uint8 downcast".to_string()))?;
            PolarsInterchangeBuffer::from_buffer(array.values().clone())?
        }
        PrimitiveType::Int128 => {
            let array = array
                .as_any()
                .downcast_ref::<PrimitiveArray<i128>>()
                .ok_or_else(|| InterchangeError::NotImplemented("int128 downcast".to_string()))?;
            PolarsInterchangeBuffer::from_buffer(array.values().clone())?
        }
        PrimitiveType::UInt16 => {
            let array = array
                .as_any()
                .downcast_ref::<PrimitiveArray<u16>>()
                .ok_or_else(|| InterchangeError::NotImplemented("uint16 downcast".to_string()))?;
            PolarsInterchangeBuffer::from_buffer(array.values().clone())?
        }
        PrimitiveType::UInt32 => {
            let array = array
                .as_any()
                .downcast_ref::<PrimitiveArray<u32>>()
                .ok_or_else(|| InterchangeError::NotImplemented("uint32 downcast".to_string()))?;
            PolarsInterchangeBuffer::from_buffer(array.values().clone())?
        }
        PrimitiveType::UInt64 => {
            let array = array
                .as_any()
                .downcast_ref::<PrimitiveArray<u64>>()
                .ok_or_else(|| InterchangeError::NotImplemented("uint64 downcast".to_string()))?;
            PolarsInterchangeBuffer::from_buffer(array.values().clone())?
        }
        PrimitiveType::Float32 => {
            let array = array
                .as_any()
                .downcast_ref::<PrimitiveArray<f32>>()
                .ok_or_else(|| InterchangeError::NotImplemented("float32 downcast".to_string()))?;
            PolarsInterchangeBuffer::from_buffer(array.values().clone())?
        }
        PrimitiveType::Float64 => {
            let array = array
                .as_any()
                .downcast_ref::<PrimitiveArray<f64>>()
                .ok_or_else(|| InterchangeError::NotImplemented("float64 downcast".to_string()))?;
            PolarsInterchangeBuffer::from_buffer(array.values().clone())?
        }
        _ => {
            return Err(InterchangeError::NotImplemented(format!(
                "primitive type {primitive:?}"
            )))
        }
    };

    Ok((Box::new(buffer) as Box<dyn Buffer>, data_dtype))
}

fn utf8_buffers(
    array: &dyn Array,
) -> Result<
    (
        Box<dyn Buffer>,
        (
            Box<dyn Buffer>,
            crate::collections::dataframe::interchange::protocol::Dtype,
        ),
    ),
    InterchangeError,
> {
    if let Some(utf8) = array.as_any().downcast_ref::<Utf8Array<i32>>() {
        return utf8_buffers_impl(utf8);
    }
    if let Some(utf8) = array.as_any().downcast_ref::<Utf8Array<i64>>() {
        return utf8_buffers_impl(utf8);
    }
    Err(InterchangeError::NotImplemented(
        "utf8 downcast".to_string(),
    ))
}

fn utf8_buffers_impl<O: Offset>(
    array: &Utf8Array<O>,
) -> Result<
    (
        Box<dyn Buffer>,
        (
            Box<dyn Buffer>,
            crate::collections::dataframe::interchange::protocol::Dtype,
        ),
    ),
    InterchangeError,
> {
    let data_buffer = PolarsInterchangeBuffer::from_buffer(array.values().clone())?;
    let offsets_buffer = PolarsInterchangeBuffer::from_offsets(array.offsets().clone())?;
    let offsets_dtype = offsets_dtype::<O>();
    Ok((
        Box::new(data_buffer) as Box<dyn Buffer>,
        (Box::new(offsets_buffer) as Box<dyn Buffer>, offsets_dtype),
    ))
}

fn binary_buffers(
    array: &dyn Array,
) -> Result<
    (
        Box<dyn Buffer>,
        (
            Box<dyn Buffer>,
            crate::collections::dataframe::interchange::protocol::Dtype,
        ),
    ),
    InterchangeError,
> {
    if let Some(binary) = array.as_any().downcast_ref::<BinaryArray<i32>>() {
        return binary_buffers_impl(binary);
    }
    if let Some(binary) = array.as_any().downcast_ref::<BinaryArray<i64>>() {
        return binary_buffers_impl(binary);
    }
    Err(InterchangeError::NotImplemented(
        "binary downcast".to_string(),
    ))
}

fn binary_buffers_impl<O: Offset>(
    array: &BinaryArray<O>,
) -> Result<
    (
        Box<dyn Buffer>,
        (
            Box<dyn Buffer>,
            crate::collections::dataframe::interchange::protocol::Dtype,
        ),
    ),
    InterchangeError,
> {
    let data_buffer = PolarsInterchangeBuffer::from_buffer(array.values().clone())?;
    let offsets_buffer = PolarsInterchangeBuffer::from_offsets(array.offsets().clone())?;
    let offsets_dtype = offsets_dtype::<O>();
    Ok((
        Box::new(data_buffer) as Box<dyn Buffer>,
        (Box::new(offsets_buffer) as Box<dyn Buffer>, offsets_dtype),
    ))
}

fn fixed_size_binary_buffers(
    array: &(dyn Array + 'static),
    validity: Option<(
        Box<dyn Buffer>,
        crate::collections::dataframe::interchange::protocol::Dtype,
    )>,
) -> Result<ColumnBuffers, InterchangeError> {
    let array = array
        .as_any()
        .downcast_ref::<FixedSizeBinaryArray>()
        .ok_or_else(|| {
            InterchangeError::NotImplemented("fixed size binary downcast".to_string())
        })?;
    let data_buffer = PolarsInterchangeBuffer::from_buffer(array.values().clone())?;
    let data_dtype = crate::collections::dataframe::interchange::protocol::Dtype {
        kind: crate::collections::dataframe::interchange::protocol::DtypeKind::UInt,
        bit_width: 8,
        format: "C".to_string(),
        endianness: crate::collections::dataframe::interchange::protocol::Endianness::Native,
        children: Vec::new(),
        child_names: Vec::new(),
    };
    Ok(ColumnBuffers {
        data: Some((Box::new(data_buffer) as Box<dyn Buffer>, data_dtype)),
        validity,
        offsets: None,
        children: Vec::new(),
    })
}

fn list_buffers(
    array: &(dyn Array + 'static),
    validity: Option<(
        Box<dyn Buffer>,
        crate::collections::dataframe::interchange::protocol::Dtype,
    )>,
    allow_copy: bool,
) -> Result<ColumnBuffers, InterchangeError> {
    if let Some(list) = array.as_any().downcast_ref::<ListArray<i32>>() {
        return list_buffers_impl(list, validity, allow_copy);
    }
    if let Some(list) = array.as_any().downcast_ref::<ListArray<i64>>() {
        return list_buffers_impl(list, validity, allow_copy);
    }
    Err(InterchangeError::NotImplemented(
        "list downcast".to_string(),
    ))
}

fn list_buffers_impl<O: Offset>(
    array: &ListArray<O>,
    validity: Option<(
        Box<dyn Buffer>,
        crate::collections::dataframe::interchange::protocol::Dtype,
    )>,
    allow_copy: bool,
) -> Result<ColumnBuffers, InterchangeError> {
    let offsets_buffer = PolarsInterchangeBuffer::from_offsets(array.offsets().clone())?;
    let offsets_dtype = offsets_dtype::<O>();
    let child_series = series_from_array(PlSmallStr::from_static("item"), array.values().clone())?;
    Ok(ColumnBuffers {
        data: None,
        validity,
        offsets: Some((Box::new(offsets_buffer) as Box<dyn Buffer>, offsets_dtype)),
        children: vec![Box::new(PolarsInterchangeColumn::new(
            child_series,
            allow_copy,
        ))],
    })
}

fn offsets_dtype<O: Offset>() -> crate::collections::dataframe::interchange::protocol::Dtype {
    let bit_width = (size_of::<O>() * 8) as u8;
    let format = if bit_width == 32 { "i" } else { "l" };
    crate::collections::dataframe::interchange::protocol::Dtype {
        kind: crate::collections::dataframe::interchange::protocol::DtypeKind::Int,
        bit_width,
        format: format.to_string(),
        endianness: crate::collections::dataframe::interchange::protocol::Endianness::Native,
        children: Vec::new(),
        child_names: Vec::new(),
    }
}

fn fixed_size_list_buffers(
    array: &(dyn Array + 'static),
    validity: Option<(
        Box<dyn Buffer>,
        crate::collections::dataframe::interchange::protocol::Dtype,
    )>,
    allow_copy: bool,
) -> Result<ColumnBuffers, InterchangeError> {
    let list = array
        .as_any()
        .downcast_ref::<FixedSizeListArray>()
        .ok_or_else(|| InterchangeError::NotImplemented("fixed size list downcast".to_string()))?;
    let child_series = series_from_array(PlSmallStr::from_static("item"), list.values().clone())?;
    Ok(ColumnBuffers {
        data: None,
        validity,
        offsets: None,
        children: vec![Box::new(PolarsInterchangeColumn::new(
            child_series,
            allow_copy,
        ))],
    })
}

fn struct_buffers(
    array: &(dyn Array + 'static),
    validity: Option<(
        Box<dyn Buffer>,
        crate::collections::dataframe::interchange::protocol::Dtype,
    )>,
    allow_copy: bool,
) -> Result<ColumnBuffers, InterchangeError> {
    let struct_array = array
        .as_any()
        .downcast_ref::<StructArray>()
        .ok_or_else(|| InterchangeError::NotImplemented("struct downcast".to_string()))?;
    let mut children = Vec::with_capacity(struct_array.values().len());
    for (idx, child) in struct_array.values().iter().enumerate() {
        let name = struct_array
            .fields()
            .get(idx)
            .map(|field| field.name.as_str())
            .unwrap_or("field");
        let series = series_from_array(PlSmallStr::from(name), child.clone())?;
        children
            .push(Box::new(PolarsInterchangeColumn::new(series, allow_copy)) as Box<dyn Column>);
    }
    Ok(ColumnBuffers {
        data: None,
        validity,
        offsets: None,
        children,
    })
}

fn map_buffers(
    array: &(dyn Array + 'static),
    validity: Option<(
        Box<dyn Buffer>,
        crate::collections::dataframe::interchange::protocol::Dtype,
    )>,
    allow_copy: bool,
) -> Result<ColumnBuffers, InterchangeError> {
    let map = array
        .as_any()
        .downcast_ref::<MapArray>()
        .ok_or_else(|| InterchangeError::NotImplemented("map downcast".to_string()))?;
    let offsets_buffer = PolarsInterchangeBuffer::from_offsets(map.offsets().clone())?;
    let offsets_dtype = crate::collections::dataframe::interchange::protocol::Dtype {
        kind: crate::collections::dataframe::interchange::protocol::DtypeKind::Int,
        bit_width: 32,
        format: "i".to_string(),
        endianness: crate::collections::dataframe::interchange::protocol::Endianness::Native,
        children: Vec::new(),
        child_names: Vec::new(),
    };

    let entries = map.field();
    let struct_array = entries
        .as_any()
        .downcast_ref::<StructArray>()
        .ok_or_else(|| {
            InterchangeError::NotImplemented("map entries struct downcast".to_string())
        })?;

    let mut children = Vec::with_capacity(struct_array.values().len());
    for (idx, child) in struct_array.values().iter().enumerate() {
        let name = struct_array
            .fields()
            .get(idx)
            .map(|field| field.name.as_str())
            .unwrap_or("field");
        let series = series_from_array(PlSmallStr::from(name), child.clone())?;
        children
            .push(Box::new(PolarsInterchangeColumn::new(series, allow_copy)) as Box<dyn Column>);
    }

    Ok(ColumnBuffers {
        data: None,
        validity,
        offsets: Some((Box::new(offsets_buffer) as Box<dyn Buffer>, offsets_dtype)),
        children,
    })
}

fn union_buffers(
    array: &(dyn Array + 'static),
    validity: Option<(
        Box<dyn Buffer>,
        crate::collections::dataframe::interchange::protocol::Dtype,
    )>,
    allow_copy: bool,
) -> Result<ColumnBuffers, InterchangeError> {
    let union = array
        .as_any()
        .downcast_ref::<UnionArray>()
        .ok_or_else(|| InterchangeError::NotImplemented("union downcast".to_string()))?;
    let data_buffer = PolarsInterchangeBuffer::from_buffer(union.types().clone())?;
    let data_dtype = crate::collections::dataframe::interchange::protocol::Dtype {
        kind: crate::collections::dataframe::interchange::protocol::DtypeKind::Int,
        bit_width: 8,
        format: "c".to_string(),
        endianness: crate::collections::dataframe::interchange::protocol::Endianness::Native,
        children: Vec::new(),
        child_names: Vec::new(),
    };

    let offsets = if let Some(offsets) = union.offsets() {
        let offsets_buffer = PolarsInterchangeBuffer::from_buffer(offsets.clone())?;
        let offsets_dtype = crate::collections::dataframe::interchange::protocol::Dtype {
            kind: crate::collections::dataframe::interchange::protocol::DtypeKind::Int,
            bit_width: 32,
            format: "i".to_string(),
            endianness: crate::collections::dataframe::interchange::protocol::Endianness::Native,
            children: Vec::new(),
            child_names: Vec::new(),
        };
        Some((Box::new(offsets_buffer) as Box<dyn Buffer>, offsets_dtype))
    } else {
        None
    };

    let mut names = Vec::new();
    if let ArrowDataType::Union(union_type) = array.dtype().to_logical_type() {
        names = union_type
            .fields
            .iter()
            .map(|field| field.name.to_string())
            .collect();
    }

    let mut children = Vec::with_capacity(union.fields().len());
    for (idx, child) in union.fields().iter().enumerate() {
        let name = names
            .get(idx)
            .cloned()
            .unwrap_or_else(|| format!("field_{idx}"));
        let series = series_from_array(PlSmallStr::from(name), child.clone())?;
        children
            .push(Box::new(PolarsInterchangeColumn::new(series, allow_copy)) as Box<dyn Column>);
    }

    Ok(ColumnBuffers {
        data: Some((Box::new(data_buffer) as Box<dyn Buffer>, data_dtype)),
        validity,
        offsets,
        children,
    })
}

fn series_from_array(name: PlSmallStr, array: ArrayRef) -> Result<Series, InterchangeError> {
    Series::from_arrow(name, array).map_err(InterchangeError::Polars)
}
