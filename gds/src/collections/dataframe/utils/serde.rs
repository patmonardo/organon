//! Serialization helpers lifted from the Python utilities.

use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::string::FromUtf8Error;

use polars::prelude::{PolarsError, PolarsResult};

/// Supported formats for serialized output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SerializationFormat {
    Binary,
    Json,
}

/// Result returned when no file target is provided.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SerializationOutput {
    Binary(Vec<u8>),
    Json(String),
}

/// Serialize a Polars-backed object by supplying a serializer closure.
///
/// If `target` is `None`, the function buffers the serialized bytes and
/// returns them as either `Binary` or `Json`. If `target` points to a path,
/// the closure writes directly to the opened file and nothing is returned.
pub fn serialize_polars_object(
    serializer: impl FnOnce(&mut dyn Write) -> PolarsResult<()>,
    target: Option<&Path>,
    format: SerializationFormat,
) -> PolarsResult<Option<SerializationOutput>> {
    match target {
        Some(path) => {
            let mut file = File::create(path).map_err(map_io_error)?;
            serializer(&mut file)?;
            Ok(None)
        }
        None => {
            let mut buffer = Vec::new();
            serializer(&mut buffer)?;
            match format {
                SerializationFormat::Binary => Ok(Some(SerializationOutput::Binary(buffer))),
                SerializationFormat::Json => {
                    let text = String::from_utf8(buffer).map_err(map_utf8_error)?;
                    Ok(Some(SerializationOutput::Json(text)))
                }
            }
        }
    }
}

fn map_io_error(err: io::Error) -> PolarsError {
    err.into()
}

fn map_utf8_error(err: FromUtf8Error) -> PolarsError {
    PolarsError::ComputeError(format!("invalid utf8: {err}").into())
}
