//! Text expressions for dataset-level DSL.
//!
//! Small collection of expression builders for text-specific transforms that
//! compile down to `polars::Expr` where possible (optimizable and lazy).

use crate::collections::dataframe::{col, lit, GDSExpr};

/// Tokenize a UTF-8 text column by whitespace, returning an `Expr` that outputs
/// a list/array of tokens for each row (Polars list dtype).
pub fn tokenize_expr(column: &str) -> GDSExpr {
    col(column).str().split(lit(" "))
}

/// Tokenize a UTF-8 text expression by whitespace.
pub fn tokenize_expr_from(expr: GDSExpr) -> GDSExpr {
    expr.str().split(lit(" "))
}

/// Token count (number of tokens) for a text column.
pub fn token_count_expr(column: &str) -> GDSExpr {
    tokenize_expr(column).list().len()
}

/// Token count (number of tokens) for a text expression.
pub fn token_count_expr_from(expr: GDSExpr) -> GDSExpr {
    tokenize_expr_from(expr).list().len()
}

/// Lowercase a text column (returns `Expr`).
pub fn lowercase_expr(column: &str) -> GDSExpr {
    col(column).str().to_lowercase()
}

/// Lowercase a text expression (returns `Expr`).
pub fn lowercase_expr_from(expr: GDSExpr) -> GDSExpr {
    expr.str().to_lowercase()
}
