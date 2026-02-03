//! Rust-native helpers inspired by polars.datatypes.classes.
//!
//! This seed pass exposes simple helpers around `polars::prelude::DataType`.

use polars::prelude::DataType;

use super::GDSDataType;

/// Base extension dtype description (seed pass).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BaseExtension {
    name: String,
    storage: GDSDataType,
    metadata: Option<String>,
}

impl BaseExtension {
    pub fn new(name: impl Into<String>, storage: GDSDataType, metadata: Option<String>) -> Self {
        Self {
            name: name.into(),
            storage,
            metadata,
        }
    }

    pub fn ext_name(&self) -> &str {
        &self.name
    }

    pub fn ext_storage(&self) -> &GDSDataType {
        &self.storage
    }

    pub fn ext_metadata(&self) -> Option<&str> {
        self.metadata.as_deref()
    }
}

/// Generic extension dtype (seed pass).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Extension(BaseExtension);

impl Extension {
    pub fn new(name: impl Into<String>, storage: GDSDataType, metadata: Option<String>) -> Self {
        Self(BaseExtension::new(name, storage, metadata))
    }

    pub fn base(&self) -> &BaseExtension {
        &self.0
    }
}

/// Return the base dtype. For the seed pass, this returns the input dtype.
pub fn base_type(dtype: &DataType) -> DataType {
    dtype.clone()
}

/// Check whether the dtype is numeric.
pub fn is_numeric(dtype: &DataType) -> bool {
    dtype.is_numeric()
}

/// Check whether the dtype is decimal.
pub fn is_decimal(dtype: &DataType) -> bool {
    dtype.is_decimal()
}

/// Check whether the dtype is integer.
pub fn is_integer(dtype: &DataType) -> bool {
    dtype.is_integer()
}

/// Check whether the dtype is signed integer.
pub fn is_signed_integer(dtype: &DataType) -> bool {
    dtype.is_signed_integer()
}

/// Check whether the dtype is unsigned integer.
pub fn is_unsigned_integer(dtype: &DataType) -> bool {
    dtype.is_unsigned_integer()
}

/// Check whether the dtype is float.
pub fn is_float(dtype: &DataType) -> bool {
    dtype.is_float()
}

/// Check whether the dtype is temporal.
pub fn is_temporal(dtype: &DataType) -> bool {
    dtype.is_temporal()
}

/// Check whether the dtype is nested.
pub fn is_nested(dtype: &DataType) -> bool {
    dtype.is_nested()
}

/// Check whether the dtype is object.
pub fn is_object(dtype: &DataType) -> bool {
    dtype.is_object()
}
