//! Column selector helpers.

use polars::prelude::{col as pl_col, Expr};

pub fn col(name: &str) -> Expr {
    pl_col(name)
}

pub fn cols(names: &[&str]) -> Vec<Expr> {
    names.iter().map(|name| pl_col(*name)).collect()
}

pub fn expr_col(name: &str) -> Expr {
    pl_col(name)
}

pub fn expr_cols(names: &[&str]) -> Vec<Expr> {
    names.iter().map(|name| pl_col(*name)).collect()
}
