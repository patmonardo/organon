//! Binary expression helpers for Dataset DSL.
//!
//! This module is part of the Dataset expressions layer and mirrors the
//! binary Expr surface used by DataFrame/Polars.

use polars::prelude::Expr;

pub use crate::collections::dataframe::expressions::binary::BinaryEncoding;
pub use crate::collections::dataframe::expressions::binary::BinaryEndianness;
pub use crate::collections::dataframe::expressions::binary::BinarySizeUnit;

pub fn binary_contains_expr_from(expr: Expr, literal: Expr) -> Expr {
    crate::collections::dataframe::expressions::binary::ExprBinary::new(expr).contains_expr(literal)
}

pub fn binary_contains_literal_from(expr: Expr, literal: &[u8]) -> Expr {
    crate::collections::dataframe::expressions::binary::ExprBinary::new(expr).contains(literal)
}

pub fn binary_starts_with_expr_from(expr: Expr, prefix: Expr) -> Expr {
    crate::collections::dataframe::expressions::binary::ExprBinary::new(expr)
        .starts_with_expr(prefix)
}

pub fn binary_starts_with_literal_from(expr: Expr, prefix: &[u8]) -> Expr {
    crate::collections::dataframe::expressions::binary::ExprBinary::new(expr).starts_with(prefix)
}

pub fn binary_ends_with_expr_from(expr: Expr, suffix: Expr) -> Expr {
    crate::collections::dataframe::expressions::binary::ExprBinary::new(expr).ends_with_expr(suffix)
}

pub fn binary_ends_with_literal_from(expr: Expr, suffix: &[u8]) -> Expr {
    crate::collections::dataframe::expressions::binary::ExprBinary::new(expr).ends_with(suffix)
}

pub fn binary_encode_from(expr: Expr, encoding: BinaryEncoding) -> Expr {
    crate::collections::dataframe::expressions::binary::ExprBinary::new(expr).encode(encoding)
}

pub fn binary_decode_from(expr: Expr, encoding: BinaryEncoding, strict: bool) -> Expr {
    crate::collections::dataframe::expressions::binary::ExprBinary::new(expr)
        .decode(encoding, strict)
}

pub fn binary_size_bytes_from(expr: Expr) -> Expr {
    crate::collections::dataframe::expressions::binary::ExprBinary::new(expr).size_bytes()
}

pub fn binary_size_from(expr: Expr, unit: BinarySizeUnit) -> Expr {
    crate::collections::dataframe::expressions::binary::ExprBinary::new(expr).size(unit)
}
