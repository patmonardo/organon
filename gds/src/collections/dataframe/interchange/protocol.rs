//! DataFrame interchange protocol definitions.

use std::collections::HashMap;

use polars::error::PolarsError;
use serde_json::Value;
use thiserror::Error;

pub type MetadataMap = HashMap<String, Value>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum DlpackDeviceType {
    CPU = 1,
    CUDA = 2,
    CPUPinned = 3,
    OpenCL = 4,
    Vulkan = 7,
    Metal = 8,
    Vpi = 9,
    Rocm = 10,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum DtypeKind {
    Int = 0,
    UInt = 1,
    Float = 2,
    Bool = 20,
    String = 21,
    Datetime = 22,
    Categorical = 23,
    List = 24,
    Struct = 25,
    Binary = 26,
    Array = 27,
    Decimal = 28,
    Object = 29,
    FixedSizeBinary = 30,
    Null = 31,
    Map = 32,
    Union = 33,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dtype {
    pub kind: DtypeKind,
    pub bit_width: u8,
    pub format: String,
    pub endianness: Endianness,
    pub children: Vec<Dtype>,
    pub child_names: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ColumnNullType {
    NonNullable = 0,
    UseNan = 1,
    UseSentinel = 2,
    UseBitmask = 3,
    UseBytemask = 4,
}

pub struct ColumnBuffers {
    pub data: Option<(Box<dyn Buffer>, Dtype)>,
    pub validity: Option<(Box<dyn Buffer>, Dtype)>,
    pub offsets: Option<(Box<dyn Buffer>, Dtype)>,
    pub children: Vec<Box<dyn Column>>,
}

pub struct CategoricalDescription {
    pub is_ordered: bool,
    pub is_dictionary: bool,
    pub categories: Box<dyn Column>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endianness {
    Little,
    Big,
    Native,
    NotApplicable,
}

#[derive(Debug, Error)]
#[error("copy not allowed: {0}")]
pub struct CopyNotAllowedError(pub String);

#[derive(Debug, Error)]
pub enum InterchangeError {
    #[error("copy not allowed: {0}")]
    CopyNotAllowed(String),
    #[error("unsupported data type: {0}")]
    UnsupportedDataType(String),
    #[error("invalid argument: {0}")]
    InvalidArgument(String),
    #[error("not implemented: {0}")]
    NotImplemented(String),
    #[error("polars error: {0}")]
    Polars(#[from] PolarsError),
}

impl From<CopyNotAllowedError> for InterchangeError {
    fn from(err: CopyNotAllowedError) -> Self {
        InterchangeError::CopyNotAllowed(err.0)
    }
}

pub trait Buffer: Send + Sync {
    fn bufsize(&self) -> usize;
    fn ptr(&self) -> *const u8;
    fn dlpack_device(&self) -> (DlpackDeviceType, Option<i32>);
}

pub trait Column: Send + Sync {
    fn size(&self) -> usize;
    fn offset(&self) -> usize;
    fn dtype(&self) -> Result<Dtype, InterchangeError>;
    fn describe_categorical(&self) -> Result<CategoricalDescription, InterchangeError>;
    fn describe_null(&self) -> Result<(ColumnNullType, Option<i64>), InterchangeError>;
    fn null_count(&self) -> Result<Option<usize>, InterchangeError>;
    fn metadata(&self) -> Result<MetadataMap, InterchangeError>;
    fn num_chunks(&self) -> Result<usize, InterchangeError>;
    fn get_chunks(&self, n_chunks: Option<usize>)
        -> Result<Vec<Box<dyn Column>>, InterchangeError>;
    fn get_buffers(&self) -> Result<ColumnBuffers, InterchangeError>;
}

pub trait DataFrame: Send + Sync {
    fn version(&self) -> u8;
    fn metadata(&self) -> Result<MetadataMap, InterchangeError>;
    fn num_columns(&self) -> Result<usize, InterchangeError>;
    fn num_rows(&self) -> Result<Option<usize>, InterchangeError>;
    fn num_chunks(&self) -> Result<usize, InterchangeError>;
    fn column_names(&self) -> Result<Vec<String>, InterchangeError>;
    fn get_column(&self, index: usize) -> Result<Box<dyn Column>, InterchangeError>;
    fn get_column_by_name(&self, name: &str) -> Result<Box<dyn Column>, InterchangeError>;
    fn get_columns(&self) -> Result<Vec<Box<dyn Column>>, InterchangeError>;
    fn select_columns(&self, indices: &[usize]) -> Result<Box<dyn DataFrame>, InterchangeError>;
    fn select_columns_by_name(
        &self,
        names: &[&str],
    ) -> Result<Box<dyn DataFrame>, InterchangeError>;
    fn get_chunks(
        &self,
        n_chunks: Option<usize>,
    ) -> Result<Vec<Box<dyn DataFrame>>, InterchangeError>;
}

pub trait SupportsInterchange {
    fn dataframe_interchange(
        &self,
        nan_as_null: bool,
        allow_copy: bool,
    ) -> Result<Box<dyn DataFrame>, InterchangeError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompatLevel {
    version: u8,
}

impl CompatLevel {
    pub fn newest() -> Self {
        Self { version: 1 }
    }

    pub fn oldest() -> Self {
        Self { version: 0 }
    }

    pub fn version(&self) -> u8 {
        self.version
    }
}
