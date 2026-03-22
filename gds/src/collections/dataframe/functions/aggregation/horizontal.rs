//! Horizontal aggregation helpers.

use polars::error::PolarsError;
use polars::prelude::{
    all_horizontal as pl_all_horizontal, any_horizontal as pl_any_horizontal, Expr, PolarsResult,
};
use polars_plan::dsl::as_struct;
use polars_plan::dsl::function_expr::FunctionExpr;

fn ensure_non_empty(exprs: &[Expr]) -> PolarsResult<()> {
    if exprs.is_empty() {
        return Err(PolarsError::ComputeError(
            "cannot return empty fold because the number of output rows is unknown".into(),
        ));
    }

    Ok(())
}

pub fn all_horizontal(exprs: &[Expr]) -> PolarsResult<Expr> {
    pl_all_horizontal(exprs)
}

pub fn any_horizontal(exprs: &[Expr]) -> PolarsResult<Expr> {
    pl_any_horizontal(exprs)
}

pub fn max_horizontal(exprs: &[Expr]) -> PolarsResult<Expr> {
    ensure_non_empty(exprs)?;
    Ok(Expr::Function {
        input: exprs.to_vec(),
        function: FunctionExpr::MaxHorizontal,
    })
}

pub fn min_horizontal(exprs: &[Expr]) -> PolarsResult<Expr> {
    ensure_non_empty(exprs)?;
    Ok(Expr::Function {
        input: exprs.to_vec(),
        function: FunctionExpr::MinHorizontal,
    })
}

pub fn sum_horizontal(exprs: &[Expr], ignore_nulls: bool) -> PolarsResult<Expr> {
    ensure_non_empty(exprs)?;
    Ok(Expr::Function {
        input: exprs.to_vec(),
        function: FunctionExpr::SumHorizontal { ignore_nulls },
    })
}

pub fn mean_horizontal(exprs: &[Expr], ignore_nulls: bool) -> PolarsResult<Expr> {
    ensure_non_empty(exprs)?;
    Ok(Expr::Function {
        input: exprs.to_vec(),
        function: FunctionExpr::MeanHorizontal { ignore_nulls },
    })
}

pub fn cum_sum_horizontal(exprs: &[Expr]) -> PolarsResult<Expr> {
    let mut cumulated = Vec::with_capacity(exprs.len());
    for index in 0..exprs.len() {
        cumulated.push(sum_horizontal(&exprs[..=index], true)?);
    }
    Ok(as_struct(cumulated).alias("cum_sum"))
}
