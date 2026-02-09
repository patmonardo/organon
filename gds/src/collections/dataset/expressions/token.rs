//! Token expression helpers.
//!
//! These helpers assume a token-struct layout with the following fields:
//! - text: String
//! - start: UInt64
//! - end: UInt64
//! - kind: String

use polars::prelude::{col, Expr};

pub const TOKEN_TEXT_FIELD: &str = "text";
pub const TOKEN_START_FIELD: &str = "start";
pub const TOKEN_END_FIELD: &str = "end";
pub const TOKEN_KIND_FIELD: &str = "kind";

pub fn token_field_expr(column: &str, field: &str) -> Expr {
	col(column).struct_().field_by_name(field)
}

pub fn token_text_expr(column: &str) -> Expr {
	token_field_expr(column, TOKEN_TEXT_FIELD)
}

pub fn token_start_expr(column: &str) -> Expr {
	token_field_expr(column, TOKEN_START_FIELD)
}

pub fn token_end_expr(column: &str) -> Expr {
	token_field_expr(column, TOKEN_END_FIELD)
}

pub fn token_kind_expr(column: &str) -> Expr {
	token_field_expr(column, TOKEN_KIND_FIELD)
}
