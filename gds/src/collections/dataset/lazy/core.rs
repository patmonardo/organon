//! Dataset DSL — `LazyFrame` namespace.
//!
//! Part of the dataset-side 2×2 matrix that mirrors the DataFrame DSL shell:
//!
//! |        | lazy        | eager        |
//! |--------|-------------|--------------|
//! | scalar | `Expr`      | `Series`     |
//! | frame  | `LazyFrame` | `DataFrame`  |
//!
//! This file provides [`DatasetLazyFrameNameSpace`], the lazy-frame entry
//! point, plus the [`LazyFrameDatasetExt`] trait that attaches `.ds()` onto
//! both `GDSLazyFrame` and Polars `LazyFrame`.
//!
//! From here you reach the dataset-flavored sub-namespaces
//! [`FeatureLazyFrameNameSpace`] and [`TreeLazyFrameNameSpace`], which are
//! the lazy-frame side of feature/tree pipelines.

use crate::collections::dataframe::GDSLazyFrame;
use crate::collections::dataset::feature::Feature;
use crate::collections::dataset::plan::Plan;

#[derive(Clone)]
pub struct DatasetLazyFrameNameSpace {
    lf: GDSLazyFrame,
}

impl DatasetLazyFrameNameSpace {
    pub fn new(lf: GDSLazyFrame) -> Self {
        Self { lf }
    }

    pub fn lazyframe(&self) -> &GDSLazyFrame {
        &self.lf
    }

    pub fn into_lazyframe(self) -> GDSLazyFrame {
        self.lf
    }

    pub fn feature(&self) -> FeatureLazyFrameNameSpace {
        FeatureLazyFrameNameSpace::new(self.lf.clone())
    }

    pub fn tree(&self) -> TreeLazyFrameNameSpace {
        TreeLazyFrameNameSpace::new(self.lf.clone())
    }

    /// Apply a deferred Dataset plan to this lazy frame.
    pub fn apply_plan(&self, plan: &Plan) -> Self {
        let lf = plan.apply_to_lazyframe(self.lf.clone().into_lazyframe());
        Self::new(GDSLazyFrame::new(lf))
    }

    /// Alias for `apply_plan` to support fluent pipeline style.
    pub fn with_plan(&self, plan: &Plan) -> Self {
        self.apply_plan(plan)
    }
}

#[derive(Clone)]
pub struct FeatureLazyFrameNameSpace {
    lf: GDSLazyFrame,
}

impl FeatureLazyFrameNameSpace {
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
pub struct TreeLazyFrameNameSpace {
    lf: GDSLazyFrame,
}

impl TreeLazyFrameNameSpace {
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
    fn ds(self) -> DatasetLazyFrameNameSpace;
}

impl LazyFrameDatasetExt for GDSLazyFrame {
    fn ds(self) -> DatasetLazyFrameNameSpace {
        DatasetLazyFrameNameSpace::new(self)
    }
}

impl LazyFrameDatasetExt for polars::prelude::LazyFrame {
    fn ds(self) -> DatasetLazyFrameNameSpace {
        DatasetLazyFrameNameSpace::new(GDSLazyFrame::new(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::dataset::namespaces::dataop::DataOpNs;
    use crate::collections::dataset::namespaces::dataset::DatasetNs;
    use crate::collections::dataset::protocol::projection::DatasetProjectionKind;
    use polars::df;
    use polars::prelude::{col, lit, IntoLazy};

    #[test]
    fn test_apply_plan_and_with_plan_are_equivalent_for_empty_plan() {
        let df = df!("x" => &[1i32, 2, 3]).unwrap();
        let ns = DatasetLazyFrameNameSpace::new(GDSLazyFrame::from_dataframe(df));
        let plan = Plan::from_var("ds");

        let out_apply = ns.apply_plan(&plan).into_lazyframe().collect().unwrap();
        let out_with = ns.with_plan(&plan).into_lazyframe().collect().unwrap();

        assert_eq!(out_apply.shape(), out_with.shape());
        assert_eq!(out_apply.get_column_names(), out_with.get_column_names());
    }

    #[test]
    fn test_lazyframe_extension_trait_enters_dataset_namespace() {
        let lf = df!("x" => &[1i32, 2, 3]).unwrap().lazy();
        let out = lf.ds().into_lazyframe().collect().unwrap();
        assert_eq!(out.shape(), (3, 1));
    }

    #[test]
    fn test_phase_b_integration_lazy_plan_namespace_flow() {
        let df = df!(
            "id" => &[1i32, 2],
            "text" => &["A B", "C D"]
        )
        .unwrap();

        let ns = DatasetLazyFrameNameSpace::new(GDSLazyFrame::from_dataframe(df));

        let text_encode = DataOpNs::text_encode("normalize");
        let text_norm_expr = DataOpNs::lower_column(&text_encode, "text").alias("text_norm");

        let plan = Plan::from_var("ds")
            .with_columns([text_norm_expr])
            .filter(col("id").gt(lit(1i32)))
            .select([col("id"), col("text_norm")]);

        let out = ns.with_plan(&plan).into_lazyframe().collect().unwrap();
        assert_eq!(out.shape(), (1, 2));

        let ids = out.column("id").unwrap().i32().unwrap();
        assert_eq!(ids.get(0), Some(2));

        let text_norm = out.column("text_norm").unwrap().str().unwrap();
        assert_eq!(text_norm.get(0), Some("c d"));

        let projection = DatasetNs::project_text(vec!["id".to_string(), "text_norm".to_string()]);
        assert!(matches!(
            projection.kind(),
            DatasetProjectionKind::TextToFrame
        ));
        assert_eq!(
            projection.columns(),
            ["id".to_string(), "text_norm".to_string()].as_slice()
        );
    }
}
