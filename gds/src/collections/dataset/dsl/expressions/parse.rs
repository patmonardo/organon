//! Parse expression helpers.
//!
//! These helpers assume a parse-struct layout with the following fields:
//! - tree: String
//! - kind: String
//! - start: UInt64 (optional span)
//! - end: UInt64 (optional span)
//! - score: Float64 (optional score)

use polars::prelude::{col, Expr};

pub const PARSE_TREE_FIELD: &str = "tree";
pub const PARSE_KIND_FIELD: &str = "kind";
pub const PARSE_START_FIELD: &str = "start";
pub const PARSE_END_FIELD: &str = "end";
pub const PARSE_SCORE_FIELD: &str = "score";

pub fn parse_field_expr(column: &str, field: &str) -> Expr {
    col(column).struct_().field_by_name(field)
}

pub fn parse_field_expr_from(expr: Expr, field: &str) -> Expr {
    expr.struct_().field_by_name(field)
}

pub fn parse_kind_expr(column: &str) -> Expr {
    parse_field_expr(column, PARSE_KIND_FIELD)
}

pub fn parse_tree_expr(column: &str) -> Expr {
    parse_field_expr(column, PARSE_TREE_FIELD)
}

pub fn parse_start_expr(column: &str) -> Expr {
    parse_field_expr(column, PARSE_START_FIELD)
}

pub fn parse_end_expr(column: &str) -> Expr {
    parse_field_expr(column, PARSE_END_FIELD)
}

pub fn parse_score_expr(column: &str) -> Expr {
    parse_field_expr(column, PARSE_SCORE_FIELD)
}
