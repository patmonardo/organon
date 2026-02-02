//! Horizontal aggregation helpers.

use polars::lazy::dsl::{
    max_horizontal as pl_max_horizontal, mean_horizontal as pl_mean_horizontal,
    min_horizontal as pl_min_horizontal, sum_horizontal as pl_sum_horizontal,
};
use polars::prelude::{
    all_horizontal as pl_all_horizontal, any_horizontal as pl_any_horizontal, Expr, PolarsResult,
};

pub fn all_horizontal(exprs: &[Expr]) -> PolarsResult<Expr> {
    pl_all_horizontal(exprs)
}

pub fn any_horizontal(exprs: &[Expr]) -> PolarsResult<Expr> {
    pl_any_horizontal(exprs)
}

pub fn max_horizontal(exprs: &[Expr]) -> PolarsResult<Expr> {
    pl_max_horizontal(exprs)
}

pub fn min_horizontal(exprs: &[Expr]) -> PolarsResult<Expr> {
    pl_min_horizontal(exprs)
}

pub fn sum_horizontal(exprs: &[Expr], ignore_nulls: bool) -> PolarsResult<Expr> {
    pl_sum_horizontal(exprs, ignore_nulls)
}

pub fn mean_horizontal(exprs: &[Expr], ignore_nulls: bool) -> PolarsResult<Expr> {
    pl_mean_horizontal(exprs, ignore_nulls)
}
