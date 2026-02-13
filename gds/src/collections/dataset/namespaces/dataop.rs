//! DataOp namespace for dataset-level SDSL pipeline authoring.

use crate::collections::dataset::expressions::dataop::{
    DataFrameLoweringArtifact, DatasetAspectArtifact, DatasetDataOpExpr,
};
use polars::prelude::Expr;

#[derive(Debug, Clone, Default)]
pub struct DataOpNs;

impl DataOpNs {
    pub fn input(name: impl Into<String>) -> DatasetDataOpExpr {
        DatasetDataOpExpr::input(name)
    }

    pub fn encode(name: impl Into<String>) -> DatasetDataOpExpr {
        DatasetDataOpExpr::encode(name)
    }

    pub fn transform(name: impl Into<String>) -> DatasetDataOpExpr {
        DatasetDataOpExpr::transform(name)
    }

    pub fn decode(name: impl Into<String>) -> DatasetDataOpExpr {
        DatasetDataOpExpr::decode(name)
    }

    pub fn output(name: impl Into<String>) -> DatasetDataOpExpr {
        DatasetDataOpExpr::output(name)
    }

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

    pub fn lower_expr(dataop: &DatasetDataOpExpr, input_expr: Expr) -> Expr {
        dataop.as_dataframe_expr(input_expr)
    }

    pub fn lower_column(dataop: &DatasetDataOpExpr, input_column: &str) -> Expr {
        dataop.as_dataframe_expr_for_column(input_column)
    }

    pub fn aspect_artifact(dataop: &DatasetDataOpExpr) -> DatasetAspectArtifact {
        dataop.as_dataset_aspect_artifact()
    }

    pub fn dataframe_artifact(
        dataop: &DatasetDataOpExpr,
        input_column: &str,
    ) -> DataFrameLoweringArtifact {
        dataop.lower_to_dataframe_artifact(input_column)
    }
}
