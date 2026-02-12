//! Repeat helpers.

use polars::prelude::{lit, repeat as pl_repeat, DataType, Expr};

pub fn repeat<E: Into<Expr>, N: Into<Expr>>(value: E, n: N) -> Expr {
    pl_repeat(value, n.into())
}

pub fn ones<N: Into<Expr>>(n: N, dtype: Option<DataType>) -> Expr {
    let expr = repeat(lit(1i64), n);
    match dtype {
        Some(dtype) => expr.cast(dtype),
        None => expr.cast(DataType::Float64),
    }
    .alias("ones")
}

pub fn zeros<N: Into<Expr>>(n: N, dtype: Option<DataType>) -> Expr {
    let expr = repeat(lit(0i64), n);
    match dtype {
        Some(dtype) => expr.cast(dtype),
        None => expr.cast(DataType::Float64),
    }
    .alias("zeros")
}
