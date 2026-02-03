//! Schema helpers (py-polars inspired).

use std::collections::BTreeMap;

use polars::prelude::{
    DataFrame, DataType, IntoLazy, LazyFrame, PlSmallStr, Schema as PolarsSchema, SchemaExt,
};
use thiserror::Error;

use crate::collections::dataframe::datatypes::parse::{parse_into_dtype, DTypeParseError};

/// Supported dtype inputs for schema entries.
#[derive(Debug, Clone, PartialEq)]
pub enum SchemaDType {
    DataType(DataType),
    Name(String),
}

impl From<DataType> for SchemaDType {
    fn from(dtype: DataType) -> Self {
        Self::DataType(dtype)
    }
}

impl From<&DataType> for SchemaDType {
    fn from(dtype: &DataType) -> Self {
        Self::DataType(dtype.clone())
    }
}

impl From<&str> for SchemaDType {
    fn from(name: &str) -> Self {
        Self::Name(name.to_string())
    }
}

impl From<String> for SchemaDType {
    fn from(name: String) -> Self {
        Self::Name(name)
    }
}

#[derive(Debug, Error)]
pub enum SchemaError {
    #[error("schema contains duplicate name '{0}'")]
    DuplicateName(String),
    #[error("invalid dtype for '{name}': {source}")]
    InvalidDtype {
        name: String,
        #[source]
        source: DTypeParseError,
    },
}

/// Ordered mapping of column names to their data type.
#[derive(Debug, Clone, PartialEq)]
pub struct Schema {
    inner: PolarsSchema,
}

impl Default for Schema {
    fn default() -> Self {
        Self::new()
    }
}

impl Schema {
    /// Create an empty schema.
    pub fn new() -> Self {
        Self {
            inner: PolarsSchema::with_capacity(0),
        }
    }

    /// Create a schema from an existing Polars schema.
    pub fn from_polars(schema: PolarsSchema) -> Self {
        Self { inner: schema }
    }

    /// Create a schema from name/dtype pairs.
    pub fn from_pairs<I, N, D>(pairs: I, check_dtypes: bool) -> Result<Self, SchemaError>
    where
        I: IntoIterator<Item = (N, D)>,
        N: Into<PlSmallStr>,
        D: Into<SchemaDType>,
    {
        let mut schema = PolarsSchema::with_capacity(0);
        for (name, dtype) in pairs {
            let name = name.into();
            if schema.get(name.as_str()).is_some() {
                return Err(SchemaError::DuplicateName(name.to_string()));
            }
            let dtype = resolve_schema_dtype(name.as_str(), dtype.into(), check_dtypes)?;
            schema.insert(name, dtype);
        }
        Ok(Self { inner: schema })
    }

    /// Insert or update a dtype for a column name.
    pub fn insert<N, D>(&mut self, name: N, dtype: D, check_dtypes: bool) -> Result<(), SchemaError>
    where
        N: Into<PlSmallStr>,
        D: Into<SchemaDType>,
    {
        let name = name.into();
        let dtype = resolve_schema_dtype(name.as_str(), dtype.into(), check_dtypes)?;
        self.inner.insert(name, dtype);
        Ok(())
    }

    /// Return column names in schema order.
    pub fn names(&self) -> Vec<String> {
        self.inner
            .iter_names()
            .map(|name| name.to_string())
            .collect()
    }

    /// Return column dtypes in schema order.
    pub fn dtypes(&self) -> Vec<DataType> {
        self.inner
            .iter_fields()
            .map(|field| field.dtype().clone())
            .collect()
    }

    /// Number of schema entries.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// True if no fields are defined.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Create an empty DataFrame from this schema.
    pub fn to_frame(&self) -> DataFrame {
        DataFrame::empty_with_schema(&self.inner)
    }

    /// Create an empty LazyFrame from this schema.
    pub fn to_lazyframe(&self) -> LazyFrame {
        DataFrame::empty_with_schema(&self.inner).lazy()
    }

    /// Return a mapping of column names to coarse Python type names.
    pub fn to_python(&self) -> BTreeMap<String, &'static str> {
        self.inner
            .iter_fields()
            .map(|field| {
                let name = field.name().to_string();
                let py = dtype_to_python_name(field.dtype());
                (name, py)
            })
            .collect()
    }

    /// Borrow the underlying Polars schema.
    pub fn as_polars(&self) -> &PolarsSchema {
        &self.inner
    }

    /// Consume into the underlying Polars schema.
    pub fn into_polars(self) -> PolarsSchema {
        self.inner
    }
}

impl From<PolarsSchema> for Schema {
    fn from(schema: PolarsSchema) -> Self {
        Self::from_polars(schema)
    }
}

impl From<Schema> for PolarsSchema {
    fn from(schema: Schema) -> Self {
        schema.inner
    }
}

fn resolve_schema_dtype(
    name: &str,
    dtype: SchemaDType,
    _check_dtypes: bool,
) -> Result<DataType, SchemaError> {
    match dtype {
        SchemaDType::DataType(dtype) => Ok(dtype),
        SchemaDType::Name(name_or) => {
            parse_into_dtype(name_or.as_str()).map_err(|source| SchemaError::InvalidDtype {
                name: name.to_string(),
                source,
            })
        }
    }
}

fn dtype_to_python_name(dtype: &DataType) -> &'static str {
    match dtype {
        DataType::Int8
        | DataType::Int16
        | DataType::Int32
        | DataType::Int64
        | DataType::Int128
        | DataType::UInt8
        | DataType::UInt16
        | DataType::UInt32
        | DataType::UInt64
        | DataType::UInt128 => "int",
        DataType::Float32 | DataType::Float64 => "float",
        DataType::Boolean => "bool",
        DataType::String => "str",
        DataType::Binary | DataType::BinaryOffset => "bytes",
        DataType::Date | DataType::Datetime(_, _) | DataType::Duration(_) | DataType::Time => {
            "datetime"
        }
        DataType::List(_) | DataType::Array(_, _) => "list",
        DataType::Struct(_) => "dict",
        _ => "object",
    }
}
