//! Error and warning types (py-polars inspired).

use thiserror::Error;

use polars::error::PolarsError as PolarsCoreError;
use std::io;

use crate::collections::catalog::types::CatalogError;
use crate::collections::extensions::chunking::ChunkingError;
use crate::collections::extensions::framing::FramingError;
use crate::collections::extensions::streaming::StreamingError;
use crate::collections::plugins::graphframe::GraphFramePluginError;
use crate::types::random::RandomGraphError;

macro_rules! simple_error {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[derive(Debug, Clone, Error, PartialEq, Eq)]
        #[error("{message}")]
        pub struct $name {
            message: String,
        }

        impl $name {
            pub fn new(message: impl Into<String>) -> Self {
                Self {
                    message: message.into(),
                }
            }

            pub fn message(&self) -> &str {
                &self.message
            }
        }
    };
}

macro_rules! simple_warning {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[derive(Debug, Clone, Error, PartialEq, Eq)]
        #[error("{message}")]
        pub struct $name {
            message: String,
        }

        impl $name {
            pub fn new(message: impl Into<String>) -> Self {
                Self {
                    message: message.into(),
                }
            }

            pub fn message(&self) -> &str {
                &self.message
            }
        }
    };
}

simple_error!(PolarsError, "Base error for Polars-like failures.");
simple_error!(ColumnNotFoundError, "Column name not found.");
simple_error!(ComputeError, "Underlying computation failed.");
simple_error!(DuplicateError, "Duplicate column or entry name.");
simple_error!(
    InvalidOperationError,
    "Operation is invalid for the given data."
);
simple_error!(NoDataError, "Operation cannot be performed on empty data.");
simple_error!(OutOfBoundsError, "Index out of bounds.");
simple_error!(PanicException, "Unexpected panic in the execution engine.");
simple_error!(SchemaError, "Schema mismatch or invalid schema operation.");
simple_error!(SchemaFieldNotFoundError, "Schema field not found.");
simple_error!(ShapeError, "Incompatible shapes for operation.");
simple_error!(SQLInterfaceError, "SQL interface error.");
simple_error!(SQLSyntaxError, "SQL syntax error.");
simple_error!(StringCacheMismatchError, "String cache mismatch.");
simple_error!(StructFieldNotFoundError, "Struct field not found.");
simple_error!(RowsError, "Unexpected number of rows returned.");
simple_error!(
    NoRowsReturnedError,
    "No rows returned when at least one expected."
);
simple_error!(
    TooManyRowsReturnedError,
    "More rows returned than expected."
);
simple_error!(ModuleUpgradeRequiredError, "Module upgrade required.");
simple_error!(
    ParameterCollisionError,
    "Same parameter supplied multiple times."
);
simple_error!(
    UnsuitableSQLError,
    "SQL query unsuitable for the operation."
);

simple_warning!(PolarsWarning, "Base warning type.");
simple_warning!(PerformanceWarning, "Performance-related warning.");
simple_warning!(
    CategoricalRemappingWarning,
    "Categorical remapping warning."
);
simple_warning!(
    MapWithoutReturnDtypeWarning,
    "Map called without return dtype."
);
simple_warning!(ChronoFormatWarning, "Suspicious chrono format string.");
simple_warning!(
    CustomUFuncWarning,
    "Custom ufunc behavior differs from expectation."
);
simple_warning!(
    DataOrientationWarning,
    "Row/column orientation inferred from input."
);
simple_warning!(
    PolarsInefficientMapWarning,
    "Potentially slow map operation."
);
simple_warning!(UnstableWarning, "Unstable functionality used.");

/// Unified error wrapper for GDS/Polars-facing APIs and examples.
#[derive(Debug, Error)]
pub enum GDSPolarsError {
    #[error(transparent)]
    Polars(#[from] PolarsCoreError),
    #[error(transparent)]
    Streaming(#[from] StreamingError),
    #[error(transparent)]
    Catalog(#[from] CatalogError),
    #[error(transparent)]
    Framing(#[from] FramingError),
    #[error(transparent)]
    Chunking(#[from] ChunkingError),
    #[error(transparent)]
    GraphFrame(#[from] GraphFramePluginError),
    #[error(transparent)]
    RandomGraph(#[from] RandomGraphError),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Regex(#[from] regex::Error),
    #[error("{0}")]
    Message(String),
}

impl From<&str> for GDSPolarsError {
    fn from(value: &str) -> Self {
        Self::Message(value.to_string())
    }
}

impl From<String> for GDSPolarsError {
    fn from(value: String) -> Self {
        Self::Message(value)
    }
}
