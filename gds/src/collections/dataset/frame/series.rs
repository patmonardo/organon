//! Dataset DSL — `Series` namespace.
//!
//! Part of the dataset-side 2×2 matrix that mirrors the DataFrame DSL shell:
//!
//! |        | lazy        | eager        |
//! |--------|-------------|--------------|
//! | scalar | `Expr`      | `Series`     |
//! | frame  | `LazyFrame` | `DataFrame`  |
//!
//! This module provides [`DatasetSeriesNs`], the eager-scalar entry
//! point, plus the [`SeriesDatasetExt`] trait that attaches `.ds()` and
//! `.dataset()` onto `GDSSeries`.
//!
//! Practically, dataset logic is *mostly* expressed via `Expr` (lazy-friendly),
//! but this small eager bridge is useful for tests and one-off eager
//! transforms. It evaluates a [`DatasetExprNs`]-built expression
//! against the underlying Polars `Series`.

use crate::collections::dataframe::GDSExpr as Expr;
use polars::prelude::{PolarsResult, Series};

use crate::collections::dataframe::{col, series_expr, GDSSeries};
use crate::collections::dataset::frame::expr::{DatasetExprNs, ExprDatasetExt};

#[derive(Debug, Clone)]
pub struct DatasetSeriesNs {
    series: GDSSeries,
}

impl DatasetSeriesNs {
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
        F: FnOnce(DatasetExprNs) -> Expr,
    {
        let root = col(self.series.name()).ds();
        let expr = f(root);
        self.eval_expr(expr)
    }
}

pub trait SeriesDatasetExt {
    fn ds(self) -> DatasetSeriesNs;
    fn dataset(self) -> DatasetSeriesNs;
}

impl SeriesDatasetExt for GDSSeries {
    fn ds(self) -> DatasetSeriesNs {
        DatasetSeriesNs::new(self)
    }

    fn dataset(self) -> DatasetSeriesNs {
        DatasetSeriesNs::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_series_extension_alias_matches_short_form() {
        let short = GDSSeries::from("x", [1i64, 2, 3])
            .ds()
            .series()
            .name()
            .to_string();
        let explicit = GDSSeries::from("x", [1i64, 2, 3])
            .dataset()
            .series()
            .name()
            .to_string();

        assert_eq!(short, explicit);
    }
}
