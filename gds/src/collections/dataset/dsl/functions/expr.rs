//! Dataset expression function helpers.
//!
//! These are compatibility helpers that return `Expr`.
//! The canonical protocol surface is the Dataset `expr` + `expressions`
//! layers; this module is intentionally downstream.

use crate::collections::dataframe::GDSExpr as Expr;

pub use crate::collections::dataset::dsl::expressions::binary::BinaryEncoding;
pub use crate::collections::dataset::dsl::expressions::binary::BinaryEndianness;
pub use crate::collections::dataset::dsl::expressions::binary::BinarySizeUnit;
pub use crate::collections::dataset::dsl::expressions::parse::PARSE_END_FIELD;
pub use crate::collections::dataset::dsl::expressions::parse::PARSE_KIND_FIELD;
pub use crate::collections::dataset::dsl::expressions::parse::PARSE_SCORE_FIELD;
pub use crate::collections::dataset::dsl::expressions::parse::PARSE_START_FIELD;
pub use crate::collections::dataset::dsl::expressions::parse::PARSE_TREE_FIELD;
pub use crate::collections::dataset::dsl::expressions::stem::STEM_END_FIELD;
pub use crate::collections::dataset::dsl::expressions::stem::STEM_KIND_FIELD;
pub use crate::collections::dataset::dsl::expressions::stem::STEM_START_FIELD;
pub use crate::collections::dataset::dsl::expressions::stem::STEM_TEXT_FIELD;
pub use crate::collections::dataset::dsl::expressions::tag::TAG_END_FIELD;
pub use crate::collections::dataset::dsl::expressions::tag::TAG_START_FIELD;
pub use crate::collections::dataset::dsl::expressions::tag::TAG_TAG_FIELD;
pub use crate::collections::dataset::dsl::expressions::tag::TAG_TEXT_FIELD;
pub use crate::collections::dataset::dsl::expressions::token::TOKEN_END_FIELD;
pub use crate::collections::dataset::dsl::expressions::token::TOKEN_KIND_FIELD;
pub use crate::collections::dataset::dsl::expressions::token::TOKEN_START_FIELD;
pub use crate::collections::dataset::dsl::expressions::token::TOKEN_TEXT_FIELD;

pub fn expr_text_tokenize_ws_from(expr: Expr) -> Expr {
    crate::collections::dataset::dsl::expressions::text::tokenize_expr_from(expr)
}

pub fn expr_text_token_count_ws_from(expr: Expr) -> Expr {
    crate::collections::dataset::dsl::expressions::text::token_count_expr_from(expr)
}

pub fn expr_text_lowercase_from(expr: Expr) -> Expr {
    crate::collections::dataset::dsl::expressions::text::lowercase_expr_from(expr)
}

pub fn expr_token_field_from(expr: Expr, field: &str) -> Expr {
    crate::collections::dataset::dsl::expressions::token::token_field_expr_from(expr, field)
}

pub fn expr_parse_field_from(expr: Expr, field: &str) -> Expr {
    crate::collections::dataset::dsl::expressions::parse::parse_field_expr_from(expr, field)
}

pub fn expr_tag_field_from(expr: Expr, field: &str) -> Expr {
    crate::collections::dataset::dsl::expressions::tag::tag_field_expr_from(expr, field)
}

pub fn expr_stem_field_from(expr: Expr, field: &str) -> Expr {
    crate::collections::dataset::dsl::expressions::stem::stem_field_expr_from(expr, field)
}

pub fn expr_bin_contains(expr: Expr, literal: &[u8]) -> Expr {
    crate::collections::dataset::dsl::expressions::binary::binary_contains_literal_from(
        expr, literal,
    )
}

pub fn expr_bin_contains_expr(expr: Expr, literal: Expr) -> Expr {
    crate::collections::dataset::dsl::expressions::binary::binary_contains_expr_from(expr, literal)
}

pub fn expr_bin_starts_with(expr: Expr, prefix: &[u8]) -> Expr {
    crate::collections::dataset::dsl::expressions::binary::binary_starts_with_literal_from(
        expr, prefix,
    )
}

pub fn expr_bin_starts_with_expr(expr: Expr, prefix: Expr) -> Expr {
    crate::collections::dataset::dsl::expressions::binary::binary_starts_with_expr_from(
        expr, prefix,
    )
}

pub fn expr_bin_ends_with(expr: Expr, suffix: &[u8]) -> Expr {
    crate::collections::dataset::dsl::expressions::binary::binary_ends_with_literal_from(
        expr, suffix,
    )
}

pub fn expr_bin_ends_with_expr(expr: Expr, suffix: Expr) -> Expr {
    crate::collections::dataset::dsl::expressions::binary::binary_ends_with_expr_from(expr, suffix)
}

pub fn expr_bin_encode(expr: Expr, encoding: BinaryEncoding) -> Expr {
    crate::collections::dataset::dsl::expressions::binary::binary_encode_from(expr, encoding)
}

pub fn expr_bin_decode(expr: Expr, encoding: BinaryEncoding, strict: bool) -> Expr {
    crate::collections::dataset::dsl::expressions::binary::binary_decode_from(
        expr, encoding, strict,
    )
}

pub fn expr_bin_size_bytes(expr: Expr) -> Expr {
    crate::collections::dataset::dsl::expressions::binary::binary_size_bytes_from(expr)
}

pub fn expr_bin_size(expr: Expr, unit: BinarySizeUnit) -> Expr {
    crate::collections::dataset::dsl::expressions::binary::binary_size_from(expr, unit)
}
