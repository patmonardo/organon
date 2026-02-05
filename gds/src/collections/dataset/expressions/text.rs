//! Text expressions for dataset-level DSL.
//!
//! Small collection of expression builders for text-specific transforms that
//! compile down to `polars::Expr` where possible (optimizable and lazy).

use polars::prelude::{col, lit, Expr};

/// Tokenize a UTF-8 text column by whitespace, returning an `Expr` that outputs
/// a list/array of tokens for each row (Polars list dtype).
pub fn tokenize_expr(column: &str) -> Expr {
    col(column).str().split(lit(" "))
}

/// Token count (number of tokens) for a text column.
pub fn token_count_expr(column: &str) -> Expr {
    tokenize_expr(column).arr().len()
}

/// Lowercase a text column (returns `Expr`).
pub fn lowercase_expr(column: &str) -> Expr {
    col(column).str().to_lowercase()
}
