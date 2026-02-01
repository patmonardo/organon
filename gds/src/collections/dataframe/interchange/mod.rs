//! DataFrame interchange subsystem.

pub mod buffer;
pub mod column;
pub mod dataframe;
pub mod from_dataframe;
pub mod protocol;
pub mod utils;

pub use buffer::PolarsInterchangeBuffer;
pub use column::PolarsInterchangeColumn;
pub use dataframe::PolarsInterchangeDataFrame;
pub use from_dataframe::from_interchange;
pub use protocol::{
    Buffer, CategoricalDescription, Column, ColumnBuffers, ColumnNullType, CompatLevel,
    CopyNotAllowedError, DataFrame, DlpackDeviceType, Dtype, DtypeKind, Endianness,
    InterchangeError, MetadataMap, SupportsInterchange,
};
pub use utils::{
    dtype_to_arrow_datatype, get_buffer_length_in_elements, polars_dtype_to_data_buffer_dtype,
    polars_dtype_to_dtype,
};
