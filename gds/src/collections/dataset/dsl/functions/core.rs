//! Core Dataset DSL functions.
//!
//! These are the atomic constructors: they build the underlying semantic
//! objects directly and stay close to the data-model surface.

use serde_json::Value as JsonValue;

use crate::collections::dataset::lab::protocol::dataop::DatasetDataOpExpr;
use crate::collections::dataset::lab::protocol::io::DatasetIoExpr;
use crate::collections::dataset::lab::protocol::metadata::DatasetMetadataExpr;
use crate::collections::dataset::lab::protocol::projection::DatasetProjectionExpr;
use crate::collections::dataset::lab::protocol::registry::DatasetRegistryExpr;
use crate::collections::dataset::lab::protocol::reporting::DatasetReportExpr;

/// Generic Dataset data-op constructors.
pub fn dataop_input(name: impl Into<String>) -> DatasetDataOpExpr {
    DatasetDataOpExpr::input(name)
}

pub fn dataop_encode(name: impl Into<String>) -> DatasetDataOpExpr {
    DatasetDataOpExpr::encode(name)
}

pub fn dataop_transform(name: impl Into<String>) -> DatasetDataOpExpr {
    DatasetDataOpExpr::transform(name)
}

pub fn dataop_decode(name: impl Into<String>) -> DatasetDataOpExpr {
    DatasetDataOpExpr::decode(name)
}

pub fn dataop_output(name: impl Into<String>) -> DatasetDataOpExpr {
    DatasetDataOpExpr::output(name)
}

/// Text-domain Dataset data-op constructors.
pub fn text_input(name: impl Into<String>) -> DatasetDataOpExpr {
    DatasetDataOpExpr::text_input(name)
}

pub fn text_encode(name: impl Into<String>) -> DatasetDataOpExpr {
    DatasetDataOpExpr::text_encode(name)
}

pub fn text_transform(name: impl Into<String>) -> DatasetDataOpExpr {
    DatasetDataOpExpr::text_transform(name)
}

pub fn text_decode(name: impl Into<String>) -> DatasetDataOpExpr {
    DatasetDataOpExpr::text_decode(name)
}

pub fn text_output(name: impl Into<String>) -> DatasetDataOpExpr {
    DatasetDataOpExpr::text_output(name)
}

/// Declarative Dataset source descriptors.
pub fn io_path(path: impl Into<String>) -> DatasetIoExpr {
    DatasetIoExpr::from_path(path)
}

pub fn io_url(url: impl Into<String>) -> DatasetIoExpr {
    DatasetIoExpr::from_url(url)
}

pub fn registry(name: impl Into<String>) -> DatasetRegistryExpr {
    DatasetRegistryExpr::new(name)
}

pub fn registry_versioned(
    name: impl Into<String>,
    version: impl Into<String>,
) -> DatasetRegistryExpr {
    DatasetRegistryExpr::versioned(name, version)
}

pub fn metadata(key: impl Into<String>, value: impl Into<JsonValue>) -> DatasetMetadataExpr {
    DatasetMetadataExpr::new(key, value)
}

pub fn project_text<I, S>(columns: I) -> DatasetProjectionExpr
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    DatasetProjectionExpr::text(collect_columns(columns))
}

pub fn project_corpus<I, S>(columns: I) -> DatasetProjectionExpr
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    DatasetProjectionExpr::corpus(collect_columns(columns))
}

pub fn project_graph<I, S>(columns: I) -> DatasetProjectionExpr
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    DatasetProjectionExpr::graph(collect_columns(columns))
}

pub fn report_summary() -> DatasetReportExpr {
    DatasetReportExpr::summary()
}

pub fn report_profile() -> DatasetReportExpr {
    DatasetReportExpr::profile()
}

fn collect_columns<I, S>(columns: I) -> Vec<String>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    columns.into_iter().map(Into::into).collect()
}
