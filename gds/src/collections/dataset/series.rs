//! Dataset DSL — `Series` namespace.
//!
//! Part of the dataset-side 2×2 matrix that mirrors the DataFrame DSL shell:
//!
//! |        | lazy        | eager        |
//! |--------|-------------|--------------|
//! | scalar | `Expr`      | `Series`     |
//! | frame  | `LazyFrame` | `DataFrame`  |
//!
//! This file provides [`DatasetSeriesNameSpace`], the eager-scalar entry
//! point, plus the [`SeriesDatasetExt`] trait that attaches `.ds()` onto
//! `GDSSeries`.
//!
//! Practically, dataset logic is *mostly* expressed via `Expr` (lazy-friendly),
//! but this small eager bridge is useful for tests and one-off eager
//! transforms. It evaluates a [`DatasetExprNameSpace`]-built expression
//! against the underlying Polars `Series`.

use polars::prelude::{Expr, PolarsResult, Series};

use crate::collections::dataframe::{series_expr, GDSSeries};
use crate::collections::dataset::expr::DatasetExprNameSpace;

#[derive(Debug, Clone)]
pub struct DatasetSeriesNameSpace {
    series: GDSSeries,
}

impl DatasetSeriesNameSpace {
    pub fn new(series: GDSSeries) -> Self {
        Self { series }
    }

    pub fn series(&self) -> &GDSSeries {
        &self.series
    }

    pub fn into_series(self) -> GDSSeries {
        self.series
    }

    /// Evaluate a dataset expression against this Series.
    pub fn eval_expr(&self, expr: Expr) -> PolarsResult<Series> {
        series_expr(self.series.series().clone()).eval_expr(expr)
    }

    /// Build an expression rooted at this Series' column name and evaluate it.
    pub fn apply<F>(&self, f: F) -> PolarsResult<Series>
    where
        F: FnOnce(DatasetExprNameSpace) -> Expr,
    {
        let root = DatasetExprNameSpace::col(self.series.name());
        let expr = f(root);
        self.eval_expr(expr)
    }
}

pub trait SeriesDatasetExt {
    fn ds(self) -> DatasetSeriesNameSpace;
}

impl SeriesDatasetExt for GDSSeries {
    fn ds(self) -> DatasetSeriesNameSpace {
        DatasetSeriesNameSpace::new(self)
    }
}
