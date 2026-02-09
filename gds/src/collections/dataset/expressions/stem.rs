//! Stem expression helpers.
//!
//! These helpers assume a stem-struct layout with the following fields:
//! - text: String
//! - start: UInt64
//! - end: UInt64
//! - kind: String

use polars::prelude::{col, Expr};

pub const STEM_TEXT_FIELD: &str = "text";
pub const STEM_START_FIELD: &str = "start";
pub const STEM_END_FIELD: &str = "end";
pub const STEM_KIND_FIELD: &str = "kind";

pub fn stem_field_expr(column: &str, field: &str) -> Expr {
    col(column).struct_().field_by_name(field)
}

pub fn stem_text_expr(column: &str) -> Expr {
    stem_field_expr(column, STEM_TEXT_FIELD)
}

pub fn stem_start_expr(column: &str) -> Expr {
    stem_field_expr(column, STEM_START_FIELD)
}

pub fn stem_end_expr(column: &str) -> Expr {
    stem_field_expr(column, STEM_END_FIELD)
}

pub fn stem_kind_expr(column: &str) -> Expr {
    stem_field_expr(column, STEM_KIND_FIELD)
}
