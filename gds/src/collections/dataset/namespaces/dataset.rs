//! Dataset namespace for dataset-level expression builders.
//!
//! These helpers create declarative dataset expressions that can later be
//! interpreted by higher-level Dataset planners.

use crate::collections::dataset::expressions::dataop::DatasetDataOpExpr;
use crate::collections::dataset::expressions::io::DatasetIoExpr;
use crate::collections::dataset::expressions::metadata::DatasetMetadataExpr;
use crate::collections::dataset::expressions::projection::DatasetProjectionExpr;
use crate::collections::dataset::expressions::registry::DatasetRegistryExpr;
use crate::collections::dataset::expressions::reporting::DatasetReportExpr;
use crate::collections::dataset::expressions::text::{
    lowercase_expr, token_count_expr, tokenize_expr,
};
use polars::prelude::Expr;

#[derive(Debug, Clone, Default)]
pub struct DatasetNs;

impl DatasetNs {
    pub fn registry(name: impl Into<String>) -> DatasetRegistryExpr {
        DatasetRegistryExpr::new(name)
    }

    pub fn registry_versioned(
        name: impl Into<String>,
        version: impl Into<String>,
    ) -> DatasetRegistryExpr {
        DatasetRegistryExpr::versioned(name, version)
    }

    pub fn io_path(path: impl Into<String>) -> DatasetIoExpr {
        DatasetIoExpr::from_path(path)
    }

    pub fn io_url(url: impl Into<String>) -> DatasetIoExpr {
        DatasetIoExpr::from_url(url)
    }

    pub fn metadata(
        key: impl Into<String>,
        value: impl Into<serde_json::Value>,
    ) -> DatasetMetadataExpr {
        DatasetMetadataExpr::new(key, value)
    }

    pub fn project_text(columns: Vec<String>) -> DatasetProjectionExpr {
        DatasetProjectionExpr::text(columns)
    }

    pub fn project_corpus(columns: Vec<String>) -> DatasetProjectionExpr {
        DatasetProjectionExpr::corpus(columns)
    }

    pub fn project_graph(columns: Vec<String>) -> DatasetProjectionExpr {
        DatasetProjectionExpr::graph(columns)
    }

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

    pub fn dataop_text_input(name: impl Into<String>) -> DatasetDataOpExpr {
        DatasetDataOpExpr::text_input(name)
    }

    pub fn dataop_text_encode(name: impl Into<String>) -> DatasetDataOpExpr {
        DatasetDataOpExpr::text_encode(name)
    }

    pub fn dataop_text_transform(name: impl Into<String>) -> DatasetDataOpExpr {
        DatasetDataOpExpr::text_transform(name)
    }

    pub fn dataop_text_decode(name: impl Into<String>) -> DatasetDataOpExpr {
        DatasetDataOpExpr::text_decode(name)
    }

    pub fn dataop_text_output(name: impl Into<String>) -> DatasetDataOpExpr {
        DatasetDataOpExpr::text_output(name)
    }

    pub fn report_summary() -> DatasetReportExpr {
        DatasetReportExpr::summary()
    }

    pub fn report_profile() -> DatasetReportExpr {
        DatasetReportExpr::profile()
    }

    /// DataFrame-compat: emit a text tokenization expression.
    pub fn text_tokenize(column: &str) -> Expr {
        tokenize_expr(column)
    }

    /// DataFrame-compat: emit a text lowercase expression.
    pub fn text_lowercase(column: &str) -> Expr {
        lowercase_expr(column)
    }

    /// DataFrame-compat: emit a text token-count expression.
    pub fn text_token_count(column: &str) -> Expr {
        token_count_expr(column)
    }

    /// DataFrame-compat: lower a dataset data-op onto an existing expression.
    pub fn apply_dataop_to_expr(dataop: &DatasetDataOpExpr, expr: Expr) -> Expr {
        dataop.as_dataframe_expr(expr)
    }

    /// DataFrame-compat: lower a dataset data-op for a source column.
    pub fn apply_dataop_to_column(dataop: &DatasetDataOpExpr, column: &str) -> Expr {
        dataop.as_dataframe_expr_for_column(column)
    }
}
