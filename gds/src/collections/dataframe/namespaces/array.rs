//! Array namespace for SeriesModel (py-polars inspired).

use polars::prelude::{Expr, Literal, PolarsResult, Series};

use crate::collections::dataframe::expr::SeriesExprArray;
use crate::collections::dataframe::expressions::array::ExprArray;

#[derive(Debug, Clone)]
pub struct ArrayNameSpace {
    series: Series,
}

impl ArrayNameSpace {
    pub fn new(series: Series) -> Self {
        Self { series }
    }

    pub fn series(&self) -> &Series {
        &self.series
    }

    pub fn into_series(self) -> Series {
        self.series
    }

    fn apply_expr<F>(&self, f: F) -> PolarsResult<Series>
    where
        F: FnOnce(ExprArray) -> Expr,
    {
        SeriesExprArray::new(self.series.clone()).apply(f)
    }

    fn apply_expr_result<F>(&self, f: F) -> PolarsResult<Series>
    where
        F: FnOnce(ExprArray) -> PolarsResult<Expr>,
    {
        SeriesExprArray::new(self.series.clone()).apply_result(f)
    }

    pub fn len(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.len())
    }

    pub fn min(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.min())
    }

    pub fn max(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.max())
    }

    pub fn sum(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.sum())
    }

    pub fn mean(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.mean())
    }

    pub fn std(&self, ddof: u8) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.std(ddof))
    }

    pub fn var(&self, ddof: u8) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.var(ddof))
    }

    pub fn median(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.median())
    }

    pub fn unique(&self, maintain_order: bool) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.unique(maintain_order))
    }

    pub fn n_unique(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.n_unique())
    }

    pub fn to_list(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.to_list())
    }

    pub fn any(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.any())
    }

    pub fn all(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.all())
    }

    pub fn sort(
        &self,
        descending: bool,
        nulls_last: bool,
        multithreaded: bool,
    ) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.sort(descending, nulls_last, multithreaded))
    }

    pub fn reverse(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.reverse())
    }

    pub fn arg_min(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.arg_min())
    }

    pub fn arg_max(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.arg_max())
    }

    pub fn get(&self, index: i64, null_on_oob: bool) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.get(index, null_on_oob))
    }

    pub fn get_expr(&self, index: Expr, null_on_oob: bool) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.get_expr(index, null_on_oob))
    }

    pub fn join(&self, separator: &str, ignore_nulls: bool) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.join(separator, ignore_nulls))
    }

    pub fn join_expr(&self, separator: Expr, ignore_nulls: bool) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.join_expr(separator, ignore_nulls))
    }

    pub fn explode(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.explode())
    }

    pub fn contains<I: Literal>(&self, item: I, nulls_equal: bool) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.contains(item, nulls_equal))
    }

    pub fn contains_expr(&self, item: Expr, nulls_equal: bool) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.contains_expr(item, nulls_equal))
    }

    pub fn count_matches<I: Literal>(&self, item: I) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.count_matches(item))
    }

    pub fn count_matches_expr(&self, item: Expr) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.count_matches_expr(item))
    }

    pub fn to_struct(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.to_struct())
    }

    pub fn slice(&self, offset: i64, length: i64, as_array: bool) -> PolarsResult<Series> {
        self.apply_expr_result(|expr| expr.slice(offset, length, as_array))
    }

    pub fn slice_expr(&self, offset: Expr, length: Expr, as_array: bool) -> PolarsResult<Series> {
        self.apply_expr_result(|expr| expr.slice_expr(offset, length, as_array))
    }

    pub fn head(&self, n: i64, as_array: bool) -> PolarsResult<Series> {
        self.apply_expr_result(|expr| expr.head(n, as_array))
    }

    pub fn head_expr(&self, n: Expr, as_array: bool) -> PolarsResult<Series> {
        self.apply_expr_result(|expr| expr.head_expr(n, as_array))
    }

    pub fn tail(&self, n: i64, as_array: bool) -> PolarsResult<Series> {
        self.apply_expr_result(|expr| expr.tail(n, as_array))
    }

    pub fn tail_expr(&self, n: Expr, as_array: bool) -> PolarsResult<Series> {
        self.apply_expr_result(|expr| expr.tail_expr(n, as_array))
    }

    pub fn shift(&self, n: i64) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.shift(n))
    }

    pub fn shift_expr(&self, n: Expr) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.shift_expr(n))
    }
}
