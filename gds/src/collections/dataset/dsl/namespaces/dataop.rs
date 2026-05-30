//! DataOp namespace for dataset-level SDSL pipeline authoring.

use crate::collections::dataframe::GDSExpr as Expr;
use crate::collections::dataset::dsl::functions;
use crate::collections::dataset::lab::protocol::dataop::{
    DataFrameLoweringArtifact, DatasetAspectArtifact, DatasetDataOpExpr,
};

#[derive(Debug, Clone, Default)]
pub struct DataOpNs;

impl DataOpNs {
    pub fn input(name: impl Into<String>) -> DatasetDataOpExpr {
        functions::dataop_input(name)
    }

    pub fn encode(name: impl Into<String>) -> DatasetDataOpExpr {
        functions::dataop_encode(name)
    }

    pub fn transform(name: impl Into<String>) -> DatasetDataOpExpr {
        functions::dataop_transform(name)
    }

    pub fn decode(name: impl Into<String>) -> DatasetDataOpExpr {
        functions::dataop_decode(name)
    }

    pub fn output(name: impl Into<String>) -> DatasetDataOpExpr {
        functions::dataop_output(name)
    }

    pub fn text_input(name: impl Into<String>) -> DatasetDataOpExpr {
        functions::text_input(name)
    }

    pub fn text_encode(name: impl Into<String>) -> DatasetDataOpExpr {
        functions::text_encode(name)
    }

    pub fn text_transform(name: impl Into<String>) -> DatasetDataOpExpr {
        functions::text_transform(name)
    }

    pub fn text_decode(name: impl Into<String>) -> DatasetDataOpExpr {
        functions::text_decode(name)
    }

    pub fn text_output(name: impl Into<String>) -> DatasetDataOpExpr {
        functions::text_output(name)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::dataframe::col;

    #[test]
    fn test_dataop_ns_text_ops_and_artifacts_are_stable() {
        let op = DataOpNs::text_transform("tok");
        assert_eq!(op.stage(), "transform");
        assert_eq!(op.name(), "tok");
        assert_eq!(op.domain(), Some("text"));

        let lowered = DataOpNs::lower_expr(&op, col("text"));
        let dbg = format!("{lowered:?}");
        assert!(!dbg.is_empty());

        let aspect = DataOpNs::aspect_artifact(&op);
        assert_eq!(aspect.stage, "transform");
        assert_eq!(aspect.op_name, "tok");

        let artifact = DataOpNs::dataframe_artifact(&op, "text");
        assert_eq!(artifact.stage, "transform");
        assert_eq!(artifact.op_name, "tok");
        assert_eq!(artifact.input_column, "text");
        assert_eq!(artifact.domain.as_deref(), Some("text"));
    }
}
