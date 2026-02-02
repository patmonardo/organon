//! Conditional expression helpers.

use polars::prelude::{when as pl_when, Expr, When};

pub fn when(predicate: Expr) -> When {
    pl_when(predicate)
}

pub fn expr_when(predicate: Expr, then_expr: Expr, otherwise_expr: Expr) -> Expr {
    pl_when(predicate).then(then_expr).otherwise(otherwise_expr)
}
