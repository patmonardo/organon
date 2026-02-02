//! List namespace for expressions (py-polars inspired).

use std::sync::Arc;

use polars::prelude::{lit, Expr, Literal, PlSmallStr, SortOptions};
use polars::series::ops::NullBehavior;

#[derive(Debug, Clone)]
pub struct ExprList {
    expr: Expr,
}

impl ExprList {
    pub fn new(expr: Expr) -> Self {
        Self { expr }
    }

    pub fn expr(&self) -> &Expr {
        &self.expr
    }

    pub fn into_expr(self) -> Expr {
        self.expr
    }

    pub fn all(self) -> Expr {
        self.expr.list().all()
    }

    pub fn any(self) -> Expr {
        self.expr.list().any()
    }

    pub fn len(self) -> Expr {
        self.expr.list().len()
    }

    pub fn drop_nulls(self) -> Expr {
        self.expr.list().drop_nulls()
    }

    pub fn sample_n(
        self,
        n: i64,
        with_replacement: bool,
        shuffle: bool,
        seed: Option<u64>,
    ) -> Expr {
        self.expr
            .list()
            .sample_n(lit(n), with_replacement, shuffle, seed)
    }

    pub fn sample_n_expr(
        self,
        n: Expr,
        with_replacement: bool,
        shuffle: bool,
        seed: Option<u64>,
    ) -> Expr {
        self.expr
            .list()
            .sample_n(n, with_replacement, shuffle, seed)
    }

    pub fn sample_fraction(
        self,
        fraction: f64,
        with_replacement: bool,
        shuffle: bool,
        seed: Option<u64>,
    ) -> Expr {
        self.expr
            .list()
            .sample_fraction(lit(fraction), with_replacement, shuffle, seed)
    }

    pub fn sample_fraction_expr(
        self,
        fraction: Expr,
        with_replacement: bool,
        shuffle: bool,
        seed: Option<u64>,
    ) -> Expr {
        self.expr
            .list()
            .sample_fraction(fraction, with_replacement, shuffle, seed)
    }

    pub fn sum(self) -> Expr {
        self.expr.list().sum()
    }

    pub fn max(self) -> Expr {
        self.expr.list().max()
    }

    pub fn min(self) -> Expr {
        self.expr.list().min()
    }

    pub fn mean(self) -> Expr {
        self.expr.list().mean()
    }

    pub fn median(self) -> Expr {
        self.expr.list().median()
    }

    pub fn std(self, ddof: u8) -> Expr {
        self.expr.list().std(ddof)
    }

    pub fn var(self, ddof: u8) -> Expr {
        self.expr.list().var(ddof)
    }

    pub fn sort(self, descending: bool, nulls_last: bool, multithreaded: bool) -> Expr {
        self.expr.list().sort(SortOptions {
            descending,
            nulls_last,
            multithreaded,
            ..SortOptions::default()
        })
    }

    pub fn reverse(self) -> Expr {
        self.expr.list().reverse()
    }

    pub fn unique(self, maintain_order: bool) -> Expr {
        if maintain_order {
            self.expr.list().unique_stable()
        } else {
            self.expr.list().unique()
        }
    }

    pub fn n_unique(self) -> Expr {
        self.expr.list().n_unique()
    }

    pub fn get(self, index: i64, null_on_oob: bool) -> Expr {
        self.expr.list().get(lit(index), null_on_oob)
    }

    pub fn get_expr(self, index: Expr, null_on_oob: bool) -> Expr {
        self.expr.list().get(index, null_on_oob)
    }

    pub fn gather<I: Literal>(self, indices: I, null_on_oob: bool) -> Expr {
        self.expr.list().gather(lit(indices), null_on_oob)
    }

    pub fn gather_expr(self, indices: Expr, null_on_oob: bool) -> Expr {
        self.expr.list().gather(indices, null_on_oob)
    }

    pub fn gather_every(self, n: i64, offset: i64) -> Expr {
        self.expr.list().gather_every(lit(n), lit(offset))
    }

    pub fn gather_every_expr(self, n: Expr, offset: Expr) -> Expr {
        self.expr.list().gather_every(n, offset)
    }

    pub fn first(self) -> Expr {
        self.expr.list().first()
    }

    pub fn last(self) -> Expr {
        self.expr.list().last()
    }

    pub fn join(self, separator: &str, ignore_nulls: bool) -> Expr {
        self.expr.list().join(lit(separator), ignore_nulls)
    }

    pub fn join_expr(self, separator: Expr, ignore_nulls: bool) -> Expr {
        self.expr.list().join(separator, ignore_nulls)
    }

    pub fn arg_min(self) -> Expr {
        self.expr.list().arg_min()
    }

    pub fn arg_max(self) -> Expr {
        self.expr.list().arg_max()
    }

    pub fn diff(self, n: i64, null_behavior: NullBehavior) -> Expr {
        self.expr.list().diff(n, null_behavior)
    }

    pub fn shift(self, periods: i64) -> Expr {
        self.expr.list().shift(lit(periods))
    }

    pub fn shift_expr(self, periods: Expr) -> Expr {
        self.expr.list().shift(periods)
    }

    pub fn slice(self, offset: i64, length: i64) -> Expr {
        self.expr.list().slice(lit(offset), lit(length))
    }

    pub fn slice_expr(self, offset: Expr, length: Expr) -> Expr {
        self.expr.list().slice(offset, length)
    }

    pub fn head(self, n: i64) -> Expr {
        self.expr.list().head(lit(n))
    }

    pub fn head_expr(self, n: Expr) -> Expr {
        self.expr.list().head(n)
    }

    pub fn tail(self, n: i64) -> Expr {
        self.expr.list().tail(lit(n))
    }

    pub fn tail_expr(self, n: Expr) -> Expr {
        self.expr.list().tail(n)
    }

    pub fn to_array(self, width: usize) -> Expr {
        self.expr.list().to_array(width)
    }

    pub fn to_struct<I, S>(self, names: I) -> Expr
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let fields = names
            .into_iter()
            .map(|name| PlSmallStr::from_str(name.as_ref()))
            .collect::<Vec<_>>();
        self.expr.list().to_struct(Arc::from(fields))
    }

    pub fn contains<I: Literal>(self, item: I, nulls_equal: bool) -> Expr {
        self.expr.list().contains(lit(item), nulls_equal)
    }

    pub fn contains_expr(self, item: Expr, nulls_equal: bool) -> Expr {
        self.expr.list().contains(item, nulls_equal)
    }

    pub fn count_matches<I: Literal>(self, item: I) -> Expr {
        self.expr.list().count_matches(lit(item))
    }

    pub fn count_matches_expr(self, item: Expr) -> Expr {
        self.expr.list().count_matches(item)
    }

    pub fn set_union<I: Literal>(self, other: I) -> Expr {
        self.expr.list().union(lit(other))
    }

    pub fn set_union_expr(self, other: Expr) -> Expr {
        self.expr.list().union(other)
    }

    pub fn set_difference<I: Literal>(self, other: I) -> Expr {
        self.expr.list().set_difference(lit(other))
    }

    pub fn set_difference_expr(self, other: Expr) -> Expr {
        self.expr.list().set_difference(other)
    }

    pub fn set_intersection<I: Literal>(self, other: I) -> Expr {
        self.expr.list().set_intersection(lit(other))
    }

    pub fn set_intersection_expr(self, other: Expr) -> Expr {
        self.expr.list().set_intersection(other)
    }

    pub fn set_symmetric_difference<I: Literal>(self, other: I) -> Expr {
        self.expr.list().set_symmetric_difference(lit(other))
    }

    pub fn set_symmetric_difference_expr(self, other: Expr) -> Expr {
        self.expr.list().set_symmetric_difference(other)
    }

    pub fn eval(self, expr: Expr) -> Expr {
        self.expr.list().eval(expr)
    }
}
