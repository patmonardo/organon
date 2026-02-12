//! Vertical aggregation helpers.

use polars::prelude::{col as pl_col, cols as pl_cols, Expr};

fn select_expr_from_names(names: &[&str]) -> Expr {
    if names.is_empty() {
        pl_col("*")
    } else {
        pl_cols(names.iter().copied()).as_expr()
    }
}

pub fn all(names: &[&str], ignore_nulls: bool) -> Expr {
    select_expr_from_names(names).all(ignore_nulls)
}

pub fn any(names: &[&str], ignore_nulls: bool) -> Expr {
    select_expr_from_names(names).any(ignore_nulls)
}

pub fn max(names: &[&str]) -> Expr {
    select_expr_from_names(names).max()
}

pub fn min(names: &[&str]) -> Expr {
    select_expr_from_names(names).min()
}

pub fn sum(names: &[&str]) -> Expr {
    select_expr_from_names(names).sum()
}

pub fn mean(names: &[&str]) -> Expr {
    select_expr_from_names(names).mean()
}

pub fn cum_sum(names: &[&str], reverse: bool) -> Expr {
    select_expr_from_names(names).cum_sum(reverse)
}
