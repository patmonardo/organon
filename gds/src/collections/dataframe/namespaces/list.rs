//! List namespace for SeriesModel (py-polars inspired).

use polars::prelude::{Expr, Literal, PolarsResult, Series};
use polars::series::ops::NullBehavior;

use crate::collections::dataframe::expr::SeriesExprList;
use crate::collections::dataframe::expressions::list::ExprList;

#[derive(Debug, Clone)]
pub struct ListNameSpace {
    series: Series,
}

impl ListNameSpace {
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
        F: FnOnce(ExprList) -> Expr,
    {
        SeriesExprList::new(self.series.clone()).apply(f)
    }

    pub fn all(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.all())
    }

    pub fn any(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.any())
    }

    pub fn len(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.len())
    }

    pub fn drop_nulls(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.drop_nulls())
    }

    pub fn sample_n(
        &self,
        n: i64,
        with_replacement: bool,
        shuffle: bool,
        seed: Option<u64>,
    ) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.sample_n(n, with_replacement, shuffle, seed))
    }

    pub fn sample_n_expr(
        &self,
        n: Expr,
        with_replacement: bool,
        shuffle: bool,
        seed: Option<u64>,
    ) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.sample_n_expr(n, with_replacement, shuffle, seed))
    }

    pub fn sample_fraction(
        &self,
        fraction: f64,
        with_replacement: bool,
        shuffle: bool,
        seed: Option<u64>,
    ) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.sample_fraction(fraction, with_replacement, shuffle, seed))
    }

    pub fn sample_fraction_expr(
        &self,
        fraction: Expr,
        with_replacement: bool,
        shuffle: bool,
        seed: Option<u64>,
    ) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.sample_fraction_expr(fraction, with_replacement, shuffle, seed))
    }

    pub fn sum(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.sum())
    }

    pub fn max(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.max())
    }

    pub fn min(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.min())
    }

    pub fn mean(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.mean())
    }

    pub fn median(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.median())
    }

    pub fn std(&self, ddof: u8) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.std(ddof))
    }

    pub fn var(&self, ddof: u8) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.var(ddof))
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

    pub fn unique(&self, maintain_order: bool) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.unique(maintain_order))
    }

    pub fn n_unique(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.n_unique())
    }

    pub fn get(&self, index: i64, null_on_oob: bool) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.get(index, null_on_oob))
    }

    pub fn get_expr(&self, index: Expr, null_on_oob: bool) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.get_expr(index, null_on_oob))
    }

    pub fn gather<I: Literal>(&self, indices: I, null_on_oob: bool) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.gather(indices, null_on_oob))
    }

    pub fn gather_expr(&self, indices: Expr, null_on_oob: bool) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.gather_expr(indices, null_on_oob))
    }

    pub fn gather_every(&self, n: i64, offset: i64) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.gather_every(n, offset))
    }

    pub fn gather_every_expr(&self, n: Expr, offset: Expr) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.gather_every_expr(n, offset))
    }

    pub fn first(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.first())
    }

    pub fn last(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.last())
    }

    pub fn join(&self, separator: &str, ignore_nulls: bool) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.join(separator, ignore_nulls))
    }

    pub fn join_expr(&self, separator: Expr, ignore_nulls: bool) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.join_expr(separator, ignore_nulls))
    }

    pub fn arg_min(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.arg_min())
    }

    pub fn arg_max(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.arg_max())
    }

    pub fn diff(&self, n: i64, null_behavior: NullBehavior) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.diff(n, null_behavior))
    }

    pub fn shift(&self, periods: i64) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.shift(periods))
    }

    pub fn shift_expr(&self, periods: Expr) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.shift_expr(periods))
    }

    pub fn slice(&self, offset: i64, length: i64) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.slice(offset, length))
    }

    pub fn slice_expr(&self, offset: Expr, length: Expr) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.slice_expr(offset, length))
    }

    pub fn head(&self, n: i64) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.head(n))
    }

    pub fn head_expr(&self, n: Expr) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.head_expr(n))
    }

    pub fn tail(&self, n: i64) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.tail(n))
    }

    pub fn tail_expr(&self, n: Expr) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.tail_expr(n))
    }

    pub fn to_array(&self, width: usize) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.to_array(width))
    }

    pub fn to_struct<I, S>(&self, names: I) -> PolarsResult<Series>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.apply_expr(|expr| expr.to_struct(names))
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

    pub fn set_union<I: Literal>(&self, other: I) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.set_union(other))
    }

    pub fn set_union_expr(&self, other: Expr) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.set_union_expr(other))
    }

    pub fn set_difference<I: Literal>(&self, other: I) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.set_difference(other))
    }

    pub fn set_difference_expr(&self, other: Expr) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.set_difference_expr(other))
    }

    pub fn set_intersection<I: Literal>(&self, other: I) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.set_intersection(other))
    }

    pub fn set_intersection_expr(&self, other: Expr) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.set_intersection_expr(other))
    }

    pub fn set_symmetric_difference<I: Literal>(&self, other: I) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.set_symmetric_difference(other))
    }

    pub fn set_symmetric_difference_expr(&self, other: Expr) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.set_symmetric_difference_expr(other))
    }

    pub fn eval(&self, expr: Expr) -> PolarsResult<Series> {
        self.apply_expr(|list| list.eval(expr))
    }
}
