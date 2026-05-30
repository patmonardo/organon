//! Expr namespace for dataset-level DSL authoring.

use crate::collections::dataframe::GDSExpr as Expr;

use crate::collections::dataframe::col;
use crate::collections::dataset::dsl::functions::expr as expr_fn;
use crate::collections::dataset::dsl::functions::expr::BinaryEncoding;
use crate::collections::dataset::dsl::functions::expr::BinarySizeUnit;

#[derive(Debug, Clone, Default)]
pub struct ExprNs;

impl ExprNs {
    pub fn col(name: impl AsRef<str>) -> Expr {
        col(name.as_ref())
    }

    pub fn text_tokenize_ws(expr: Expr) -> Expr {
        expr_fn::expr_text_tokenize_ws_from(expr)
    }

    pub fn text_token_count_ws(expr: Expr) -> Expr {
        expr_fn::expr_text_token_count_ws_from(expr)
    }

    pub fn text_lowercase(expr: Expr) -> Expr {
        expr_fn::expr_text_lowercase_from(expr)
    }

    pub fn token_field(expr: Expr, field: &str) -> Expr {
        expr_fn::expr_token_field_from(expr, field)
    }

    pub fn parse_field(expr: Expr, field: &str) -> Expr {
        expr_fn::expr_parse_field_from(expr, field)
    }

    pub fn tag_field(expr: Expr, field: &str) -> Expr {
        expr_fn::expr_tag_field_from(expr, field)
    }

    pub fn stem_field(expr: Expr, field: &str) -> Expr {
        expr_fn::expr_stem_field_from(expr, field)
    }

    pub fn bin_contains(expr: Expr, literal: &[u8]) -> Expr {
        expr_fn::expr_bin_contains(expr, literal)
    }

    pub fn bin_contains_expr(expr: Expr, literal: Expr) -> Expr {
        expr_fn::expr_bin_contains_expr(expr, literal)
    }

    pub fn bin_starts_with(expr: Expr, prefix: &[u8]) -> Expr {
        expr_fn::expr_bin_starts_with(expr, prefix)
    }

    pub fn bin_starts_with_expr(expr: Expr, prefix: Expr) -> Expr {
        expr_fn::expr_bin_starts_with_expr(expr, prefix)
    }

    pub fn bin_ends_with(expr: Expr, suffix: &[u8]) -> Expr {
        expr_fn::expr_bin_ends_with(expr, suffix)
    }

    pub fn bin_ends_with_expr(expr: Expr, suffix: Expr) -> Expr {
        expr_fn::expr_bin_ends_with_expr(expr, suffix)
    }

    pub fn bin_encode(expr: Expr, encoding: BinaryEncoding) -> Expr {
        expr_fn::expr_bin_encode(expr, encoding)
    }

    pub fn bin_decode(expr: Expr, encoding: BinaryEncoding, strict: bool) -> Expr {
        expr_fn::expr_bin_decode(expr, encoding, strict)
    }

    pub fn bin_size_bytes(expr: Expr) -> Expr {
        expr_fn::expr_bin_size_bytes(expr)
    }

    pub fn bin_size(expr: Expr, unit: BinarySizeUnit) -> Expr {
        expr_fn::expr_bin_size(expr, unit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr_ns_builds_text_exprs() {
        let expr = ExprNs::text_lowercase(ExprNs::col("text")).alias("text_lower");
        assert!(!format!("{expr:?}").is_empty());
    }
}
