//! Tag expression helpers.
//!
//! These helpers assume a tag-struct layout with the following fields:
//! - text: String
//! - tag: String
//! - start: UInt64
//! - end: UInt64

use polars::prelude::{col, Expr};

pub const TAG_TEXT_FIELD: &str = "text";
pub const TAG_TAG_FIELD: &str = "tag";
pub const TAG_START_FIELD: &str = "start";
pub const TAG_END_FIELD: &str = "end";

pub fn tag_field_expr(column: &str, field: &str) -> Expr {
    col(column).struct_().field_by_name(field)
}

pub fn tag_text_expr(column: &str) -> Expr {
    tag_field_expr(column, TAG_TEXT_FIELD)
}

pub fn tag_tag_expr(column: &str) -> Expr {
    tag_field_expr(column, TAG_TAG_FIELD)
}

pub fn tag_start_expr(column: &str) -> Expr {
    tag_field_expr(column, TAG_START_FIELD)
}

pub fn tag_end_expr(column: &str) -> Expr {
    tag_field_expr(column, TAG_END_FIELD)
}
