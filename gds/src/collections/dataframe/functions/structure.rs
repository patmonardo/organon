//! Struct construction helpers.

use polars::lazy::dsl::as_struct as pl_as_struct;
use polars::prelude::Expr;

pub fn structure(exprs: Vec<Expr>) -> Expr {
    pl_as_struct(exprs)
}

pub fn record(exprs: Vec<Expr>) -> Expr {
    structure(exprs)
}
