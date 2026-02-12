//! Array namespace for expressions (py-polars inspired).

use polars::prelude::{lit, Expr, Literal, PolarsResult, SortOptions};

#[derive(Debug, Clone)]
pub struct ExprArray {
    expr: Expr,
}

impl ExprArray {
    pub fn new(expr: Expr) -> Self {
        Self { expr }
    }

    pub fn expr(&self) -> &Expr {
        &self.expr
    }

    pub fn into_expr(self) -> Expr {
        self.expr
    }

    pub fn len(self) -> Expr {
        self.expr.arr().len()
    }

    pub fn min(self) -> Expr {
        self.expr.arr().min()
    }

    pub fn max(self) -> Expr {
        self.expr.arr().max()
    }

    pub fn sum(self) -> Expr {
        self.expr.arr().sum()
    }

    pub fn mean(self) -> Expr {
        self.expr.arr().mean()
    }

    pub fn std(self, ddof: u8) -> Expr {
        self.expr.arr().std(ddof)
    }

    pub fn var(self, ddof: u8) -> Expr {
        self.expr.arr().var(ddof)
    }

    pub fn median(self) -> Expr {
        self.expr.arr().median()
    }

    pub fn unique(self, maintain_order: bool) -> Expr {
        if maintain_order {
            self.expr.arr().unique_stable()
        } else {
            self.expr.arr().unique()
        }
    }

    pub fn n_unique(self) -> Expr {
        self.expr.arr().n_unique()
    }

    pub fn to_list(self) -> Expr {
        self.expr.arr().to_list()
    }

    pub fn any(self) -> Expr {
        self.expr.arr().any()
    }

    pub fn all(self) -> Expr {
        self.expr.arr().all()
    }

    pub fn sort(self, descending: bool, nulls_last: bool) -> Expr {
        self.expr.arr().sort(SortOptions {
            descending,
            nulls_last,
            ..SortOptions::default()
        })
    }

    pub fn reverse(self) -> Expr {
        self.expr.arr().reverse()
    }

    pub fn arg_min(self) -> Expr {
        self.expr.arr().arg_min()
    }

    pub fn arg_max(self) -> Expr {
        self.expr.arr().arg_max()
    }

    pub fn get(self, index: i64, null_on_oob: bool) -> Expr {
        self.expr.arr().get(lit(index), null_on_oob)
    }

    pub fn get_expr(self, index: Expr, null_on_oob: bool) -> Expr {
        self.expr.arr().get(index, null_on_oob)
    }

    pub fn first(self) -> Expr {
        self.get(0, true)
    }

    pub fn last(self) -> Expr {
        self.get(-1, true)
    }

    pub fn join(self, separator: &str, ignore_nulls: bool) -> Expr {
        self.expr.arr().join(lit(separator), ignore_nulls)
    }

    pub fn join_expr(self, separator: Expr, ignore_nulls: bool) -> Expr {
        self.expr.arr().join(separator, ignore_nulls)
    }

    pub fn explode(self) -> Expr {
        self.expr.arr().explode()
    }

    pub fn contains<I: Literal>(self, item: I, nulls_equal: bool) -> Expr {
        self.expr.arr().contains(lit(item), nulls_equal)
    }

    pub fn contains_expr(self, item: Expr, nulls_equal: bool) -> Expr {
        self.expr.arr().contains(item, nulls_equal)
    }

    pub fn count_matches<I: Literal>(self, item: I) -> Expr {
        self.expr.arr().count_matches(lit(item))
    }

    pub fn count_matches_expr(self, item: Expr) -> Expr {
        self.expr.arr().count_matches(item)
    }

    pub fn to_struct(self) -> Expr {
        self.expr.arr().to_struct(None)
    }

    pub fn slice(self, offset: i64, length: i64, as_array: bool) -> PolarsResult<Expr> {
        self.expr.arr().slice(lit(offset), lit(length), as_array)
    }

    pub fn slice_expr(self, offset: Expr, length: Expr, as_array: bool) -> PolarsResult<Expr> {
        self.expr.arr().slice(offset, length, as_array)
    }

    pub fn head(self, n: i64, as_array: bool) -> PolarsResult<Expr> {
        self.expr.arr().head(lit(n), as_array)
    }

    pub fn head_expr(self, n: Expr, as_array: bool) -> PolarsResult<Expr> {
        self.expr.arr().head(n, as_array)
    }

    pub fn tail(self, n: i64, as_array: bool) -> PolarsResult<Expr> {
        self.expr.arr().tail(lit(n), as_array)
    }

    pub fn tail_expr(self, n: Expr, as_array: bool) -> PolarsResult<Expr> {
        self.expr.arr().tail(n, as_array)
    }

    pub fn shift(self, n: i64) -> Expr {
        self.expr.arr().shift(lit(n))
    }

    pub fn shift_expr(self, n: Expr) -> Expr {
        self.expr.arr().shift(n)
    }

    pub fn eval(self, expr: Expr, as_list: bool) -> Expr {
        self.expr.arr().eval(expr, as_list)
    }

    pub fn agg(self, expr: Expr) -> Expr {
        self.expr.arr().agg(expr)
    }
}
