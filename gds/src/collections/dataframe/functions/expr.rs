//! Expression helpers.

use polars::prelude::{DataType, Expr};

pub fn expr_eq(lhs: Expr, rhs: Expr) -> Expr {
    lhs.eq(rhs)
}

pub fn expr_neq(lhs: Expr, rhs: Expr) -> Expr {
    lhs.neq(rhs)
}

pub fn expr_gt(lhs: Expr, rhs: Expr) -> Expr {
    lhs.gt(rhs)
}

pub fn expr_gte(lhs: Expr, rhs: Expr) -> Expr {
    lhs.gt_eq(rhs)
}

pub fn expr_lt(lhs: Expr, rhs: Expr) -> Expr {
    lhs.lt(rhs)
}

pub fn expr_lte(lhs: Expr, rhs: Expr) -> Expr {
    lhs.lt_eq(rhs)
}

pub fn expr_and(lhs: Expr, rhs: Expr) -> Expr {
    lhs.and(rhs)
}

pub fn expr_or(lhs: Expr, rhs: Expr) -> Expr {
    lhs.or(rhs)
}

pub fn expr_not(expr: Expr) -> Expr {
    expr.not()
}

pub fn expr_alias(expr: Expr, name: &str) -> Expr {
    expr.alias(name)
}

pub fn expr_is_null(expr: Expr) -> Expr {
    expr.is_null()
}

pub fn expr_is_not_null(expr: Expr) -> Expr {
    expr.is_not_null()
}

pub fn expr_fill_null(expr: Expr, value: Expr) -> Expr {
    expr.fill_null(value)
}

pub fn expr_cast(expr: Expr, dtype: DataType) -> Expr {
    expr.cast(dtype)
}

pub fn expr_sum(expr: Expr) -> Expr {
    expr.sum()
}

pub fn expr_mean(expr: Expr) -> Expr {
    expr.mean()
}

pub fn expr_min(expr: Expr) -> Expr {
    expr.min()
}

pub fn expr_max(expr: Expr) -> Expr {
    expr.max()
}

pub fn expr_count(expr: Expr) -> Expr {
    expr.count()
}
