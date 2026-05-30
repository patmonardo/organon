//! Dataset DSL — `LazyFrame` namespace.
//!
//! Part of the dataset-side 2×2 matrix that mirrors the DataFrame DSL shell:
//!
//! |        | lazy        | eager        |
//! |--------|-------------|--------------|
//! | scalar | `Expr`      | `Series`     |
//! | frame  | `LazyFrame` | `DataFrame`  |
//!
//! This file provides [`DatasetLazyFrameNs`], the lazy-frame entry
//! point, plus the [`LazyFrameDatasetExt`] trait that attaches `.ds()` and
//! `.dataset()` onto `GDSLazyFrame`.
//!
//! From here you reach the dataset-flavored sub-namespaces
//! [`FeatureLazyFrameNs`] and [`TreeLazyFrameNs`], which are
//! the lazy-frame side of feature/tree pipelines.

use crate::collections::dataframe::GDSLazyFrame;
use crate::collections::dataset::feature::Feature;
use crate::collections::dataset::plan::Plan;

#[derive(Clone)]
pub struct DatasetLazyFrameNs {
    lf: GDSLazyFrame,
}

impl DatasetLazyFrameNs {
    pub fn new(lf: GDSLazyFrame) -> Self {
        Self { lf }
    }

    pub fn storage_runtime(&self) -> DatasetLazyFrameStorageRuntime {
        DatasetLazyFrameStorageRuntime::new(self.lf.clone())
    }

    pub fn lazyframe(&self) -> &GDSLazyFrame {
        &self.lf
    }

    pub fn into_lazyframe(self) -> GDSLazyFrame {
        self.lf
    }

    pub fn feature(&self) -> FeatureLazyFrameNs {
        FeatureLazyFrameNs::new(self.lf.clone())
    }

    pub fn tree(&self) -> TreeLazyFrameNs {
        TreeLazyFrameNs::new(self.lf.clone())
    }

    /// Apply a deferred Dataset plan to this lazy frame.
    pub fn apply_plan(&self, plan: &Plan) -> Self {
        self.storage_runtime()
            .compute_plan(DatasetLazyFrameComputationRuntime::new(plan))
    }

    /// Alias for `apply_plan` to support fluent pipeline style.
    pub fn with_plan(&self, plan: &Plan) -> Self {
        self.apply_plan(plan)
    }
}

#[derive(Clone)]
pub struct DatasetLazyFrameStorageRuntime {
    lf: GDSLazyFrame,
}

impl DatasetLazyFrameStorageRuntime {
    pub fn new(lf: GDSLazyFrame) -> Self {
        Self { lf }
    }

    pub fn lazyframe(&self) -> &GDSLazyFrame {
        &self.lf
    }

    pub fn into_lazyframe(self) -> GDSLazyFrame {
        self.lf
    }

    pub fn compute_plan(
        &self,
        computation: DatasetLazyFrameComputationRuntime<'_>,
    ) -> DatasetLazyFrameNs {
        DatasetLazyFrameNs::new(computation.apply_to(self.lf.clone()))
    }
}

pub struct DatasetLazyFrameComputationRuntime<'a> {
    plan: &'a Plan,
}

impl<'a> DatasetLazyFrameComputationRuntime<'a> {
    pub fn new(plan: &'a Plan) -> Self {
        Self { plan }
    }

    pub fn apply_to(&self, lf: GDSLazyFrame) -> GDSLazyFrame {
        let lf = self.plan.apply_to_lazyframe(lf.into_lazyframe());
        GDSLazyFrame::new(lf)
    }
}

#[derive(Clone)]
pub struct FeatureLazyFrameNs {
    lf: GDSLazyFrame,
}

impl FeatureLazyFrameNs {
    pub fn new(lf: GDSLazyFrame) -> Self {
        Self { lf }
    }

    pub fn lazyframe(&self) -> &GDSLazyFrame {
        &self.lf
    }

    pub fn into_lazyframe(self) -> GDSLazyFrame {
        self.lf
    }

    pub fn apply(&self, feature: &Feature) -> GDSLazyFrame {
        let lf = feature.apply_to_lazyframe(self.lf.clone().into_lazyframe());
        GDSLazyFrame::new(lf)
    }
}

#[derive(Clone)]
pub struct TreeLazyFrameNs {
    lf: GDSLazyFrame,
}

impl TreeLazyFrameNs {
    pub fn new(lf: GDSLazyFrame) -> Self {
        Self { lf }
    }

    pub fn lazyframe(&self) -> &GDSLazyFrame {
        &self.lf
    }

    pub fn into_lazyframe(self) -> GDSLazyFrame {
        self.lf
    }
}

pub trait LazyFrameDatasetExt {
    fn ds(self) -> DatasetLazyFrameNs;
    fn dataset(self) -> DatasetLazyFrameNs;
}

impl LazyFrameDatasetExt for GDSLazyFrame {
    fn ds(self) -> DatasetLazyFrameNs {
        DatasetLazyFrameNs::new(self)
    }

    fn dataset(self) -> DatasetLazyFrameNs {
        DatasetLazyFrameNs::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::dataframe::{col, lit, GDSDataFrame, RowValue};
    use crate::collections::dataset::dsl::namespaces::dataop::DataOpNs;
    use crate::collections::dataset::dsl::namespaces::dataset::DatasetNs;
    use crate::collections::dataset::dsl::namespaces::expr::ExprNs;
    use crate::collections::dataset::dsl::namespaces::plan::PlanNs;
    use crate::collections::dataset::frame::expr::ExprDatasetExt;
    use crate::collections::dataset::lab::protocol::projection::DatasetProjectionKind;
    use crate::collections::dataset::plan::EvalMode;
    use crate::collections::dataset::DatasetSplit;
    use crate::tbl;
    use std::error::Error;

    #[test]
    fn test_apply_plan_and_with_plan_are_equivalent_for_empty_plan() -> Result<(), Box<dyn Error>> {
        let df = tbl!((x: i64 => [1, 2, 3]))?;
        let ns = DatasetLazyFrameNs::new(df.lazy());
        let plan = Plan::from_var("ds");

        let out_apply = GDSDataFrame::new(ns.apply_plan(&plan).into_lazyframe().collect()?);
        let out_with = GDSDataFrame::new(ns.with_plan(&plan).into_lazyframe().collect()?);

        assert_eq!(out_apply.shape(), out_with.shape());
        assert_eq!(out_apply.column_names(), out_with.column_names());
        Ok(())
    }

    #[test]
    fn test_lazyframe_extension_trait_enters_dataset_namespace() -> Result<(), Box<dyn Error>> {
        let lf = tbl!((x: i64 => [1, 2, 3]))?.lazy();
        let out = GDSDataFrame::new(lf.dataset().into_lazyframe().collect()?);
        assert_eq!(out.shape(), (3, 1));
        Ok(())
    }

    #[test]
    fn test_phase_b_integration_lazy_plan_namespace_flow() -> Result<(), Box<dyn Error>> {
        let df = tbl!((id: i64 => [1, 2]), (text: ["A B", "C D"]))?;

        let ns = DatasetLazyFrameNs::new(df.lazy());

        let text_encode = DataOpNs::text_encode("normalize");
        let text_norm_expr = DataOpNs::lower_column(&text_encode, "text").alias("text_norm");

        let plan = Plan::from_var("ds")
            .with_columns([text_norm_expr])
            .filter(col("id").gt(lit(1i64)))
            .select([col("id"), col("text_norm")]);

        let out = GDSDataFrame::new(ns.with_plan(&plan).into_lazyframe().collect()?);
        assert_eq!(out.shape(), (1, 2));

        let row0 = out.row(0)?;
        assert!(matches!(row0[0], RowValue::Int64(2) | RowValue::Int32(2)));
        assert!(match &row0[1] {
            RowValue::String(value) => *value == "c d",
            RowValue::StringOwned(value) => value.as_str() == "c d",
            _ => false,
        });

        let projection = DatasetNs::project_text(vec!["id".to_string(), "text_norm".to_string()]);
        assert!(matches!(
            projection.kind(),
            DatasetProjectionKind::TextToFrame
        ));
        assert_eq!(
            projection.columns(),
            ["id".to_string(), "text_norm".to_string()].as_slice()
        );
        Ok(())
    }

    #[test]
    fn test_plan_lazy_expr_spine_keeps_authoring_control_and_execution_separate(
    ) -> Result<(), Box<dyn Error>> {
        let df =
            tbl!((id: i64 => [1, 2, 3]), (text: ["Alpha Beta", "Gamma Delta", "Epsilon Zeta"]))?;

        let ns = DatasetLazyFrameNs::new(df.lazy());
        let text_lower = col("text").dataset().text().lowercase().alias("text_lower");
        let text_encode = DataOpNs::text_encode("normalize");

        let plan = Plan::from_var("ds")
            .named("lazy-expr-spine")
            .dataop(text_encode)
            .batch(64)
            .split(DatasetSplit::Train)
            .with_columns([text_lower])
            .filter(col("id").gt(lit(1i64)))
            .select([col("id"), col("text_lower")]);

        let report = plan.attention_report(None, EvalMode::Preview);
        assert_eq!(report.plan_name.as_deref(), Some("lazy-expr-spine"));
        assert_eq!(report.batch_hint, Some(64));
        assert_eq!(report.split_hint.as_deref(), Some("Train"));
        assert_eq!(
            report.planned_columns,
            Some(vec!["id".to_string(), "text_lower".to_string()])
        );

        let out = GDSDataFrame::new(ns.with_plan(&plan).into_lazyframe().collect()?);
        assert_eq!(out.shape(), (2, 2));
        assert_eq!(
            out.column_names(),
            vec!["id".to_string(), "text_lower".to_string()]
        );

        let ids_col = out.get_column("id")?;
        let ids = ids_col.i64()?.into_no_null_iter().collect::<Vec<_>>();
        assert_eq!(ids, vec![2, 3]);

        let text_lower_col = out.get_column("text_lower")?;
        let text_lower = text_lower_col
            .str()?
            .into_no_null_iter()
            .map(str::to_string)
            .collect::<Vec<_>>();
        assert_eq!(
            text_lower,
            vec!["gamma delta".to_string(), "epsilon zeta".to_string()]
        );
        Ok(())
    }

    #[test]
    fn test_expr_ns_plan_ns_flow_through_lazyframe_namespace() -> Result<(), Box<dyn Error>> {
        let df = tbl!((id: i64 => [1, 2]), (text: ["Alpha Beta", "Gamma Delta"]))?;

        let ns = DatasetLazyFrameNs::new(df.lazy());
        let text_lower = ExprNs::text_lowercase(ExprNs::col("text")).alias("text_lower");
        let token_count = ExprNs::text_token_count_ws(ExprNs::col("text")).alias("token_count");

        let plan = PlanNs::from_var("ds")
            .with_columns([text_lower, token_count])
            .select([
                ExprNs::col("id"),
                ExprNs::col("text_lower"),
                ExprNs::col("token_count"),
            ]);

        let out = GDSDataFrame::new(ns.with_plan(&plan).into_lazyframe().collect()?);

        assert_eq!(out.shape(), (2, 3));
        assert_eq!(
            out.column_names(),
            vec![
                "id".to_string(),
                "text_lower".to_string(),
                "token_count".to_string()
            ]
        );
        Ok(())
    }
}
