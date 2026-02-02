//! Literal helpers.

use polars::prelude::{lit as pl_lit, Expr, Literal};

pub fn lit<T: Literal>(value: T) -> Expr {
    pl_lit(value)
}
