//! Miscellaneous expression helpers.

use polars::lazy::dsl::{
    arg_where as pl_arg_where, coalesce as pl_coalesce, concat_arr as pl_concat_arr,
    concat_list as pl_concat_list, concat_str as pl_concat_str, cov as pl_cov,
    cum_fold_exprs as pl_cum_fold_exprs, cum_reduce_exprs as pl_cum_reduce_exprs,
    fold_exprs as pl_fold_exprs, format_str as pl_format_str, pearson_corr as pl_pearson_corr,
    reduce_exprs as pl_reduce_exprs, DataTypeExpr,
};
use polars::prelude::{Expr, PlanCallback, PolarsResult, Series};

pub fn arg_where(condition: Expr) -> Expr {
    pl_arg_where(condition)
}

pub fn coalesce(exprs: &[Expr]) -> Expr {
    pl_coalesce(exprs)
}

pub fn corr(a: Expr, b: Expr) -> Expr {
    pl_pearson_corr(a, b)
}

pub fn cov(a: Expr, b: Expr, ddof: u8) -> Expr {
    pl_cov(a, b, ddof)
}

pub fn fold(
    acc: Expr,
    function: PlanCallback<(Series, Series), Series>,
    exprs: &[Expr],
    returns_scalar: bool,
    return_dtype: Option<DataTypeExpr>,
) -> Expr {
    pl_fold_exprs(acc, function, exprs, returns_scalar, return_dtype)
}

pub fn reduce(
    function: PlanCallback<(Series, Series), Series>,
    exprs: &[Expr],
    returns_scalar: bool,
    return_dtype: Option<DataTypeExpr>,
) -> Expr {
    pl_reduce_exprs(function, exprs, returns_scalar, return_dtype)
}

pub fn cum_fold(
    acc: Expr,
    function: PlanCallback<(Series, Series), Series>,
    exprs: &[Expr],
    returns_scalar: bool,
    return_dtype: Option<DataTypeExpr>,
    include_init: bool,
) -> Expr {
    pl_cum_fold_exprs(
        acc,
        function,
        exprs,
        returns_scalar,
        return_dtype,
        include_init,
    )
}

pub fn cum_reduce(
    function: PlanCallback<(Series, Series), Series>,
    exprs: &[Expr],
    returns_scalar: bool,
    return_dtype: Option<DataTypeExpr>,
) -> Expr {
    pl_cum_reduce_exprs(function, exprs, returns_scalar, return_dtype)
}

pub fn concat_list(exprs: &[Expr]) -> PolarsResult<Expr> {
    pl_concat_list(exprs)
}

pub fn concat_arr(exprs: &[Expr]) -> PolarsResult<Expr> {
    pl_concat_arr(exprs.to_vec())
}

pub fn concat_str(exprs: &[Expr], separator: &str, ignore_nulls: bool) -> Expr {
    pl_concat_str(exprs, separator, ignore_nulls)
}

pub fn format(format: &str, args: &[Expr]) -> PolarsResult<Expr> {
    pl_format_str(format, args)
}
