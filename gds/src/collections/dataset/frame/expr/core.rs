//! Dataset DSL — `Expr` namespace.
//!
//! Part of the dataset-side 2×2 matrix that mirrors the DataFrame DSL shell:
//!
//! |        | lazy        | eager        |
//! |--------|-------------|--------------|
//! | scalar | `Expr`      | `Series`     |
//! | frame  | `LazyFrame` | `DataFrame`  |
//!
//! This module provides [`DatasetExprNameSpace`], the lazy-scalar entry point,
//! plus the [`ExprDatasetExt`] trait that attaches `.ds()` onto Polars `Expr`.
//!
//! Most dataset logic flows through this surface because it composes cleanly
//! into [`crate::collections::dataset::frame::lazy::DatasetLazyFrameNameSpace`]
//! pipelines. Concrete sub-namespaces (`text`, `token`, `parse`, `tag`,
//! `stem`) are thin facades over the lower-level expression builders in
//! [`crate::collections::dataset::expressions`].

use polars::prelude::{col, Expr};

use crate::collections::dataset::dsl::expressions::binary::binary_contains_expr_from;
use crate::collections::dataset::dsl::expressions::binary::binary_contains_literal_from;
use crate::collections::dataset::dsl::expressions::binary::binary_decode_from;
use crate::collections::dataset::dsl::expressions::binary::binary_encode_from;
use crate::collections::dataset::dsl::expressions::binary::binary_ends_with_expr_from;
use crate::collections::dataset::dsl::expressions::binary::binary_ends_with_literal_from;
use crate::collections::dataset::dsl::expressions::binary::binary_size_bytes_from;
use crate::collections::dataset::dsl::expressions::binary::binary_size_from;
use crate::collections::dataset::dsl::expressions::binary::binary_starts_with_expr_from;
use crate::collections::dataset::dsl::expressions::binary::binary_starts_with_literal_from;
use crate::collections::dataset::dsl::expressions::binary::BinaryEncoding;
use crate::collections::dataset::dsl::expressions::binary::BinarySizeUnit;
use crate::collections::dataset::dsl::expressions::parse::parse_field_expr_from;
use crate::collections::dataset::dsl::expressions::parse::PARSE_END_FIELD;
use crate::collections::dataset::dsl::expressions::parse::PARSE_KIND_FIELD;
use crate::collections::dataset::dsl::expressions::parse::PARSE_SCORE_FIELD;
use crate::collections::dataset::dsl::expressions::parse::PARSE_START_FIELD;
use crate::collections::dataset::dsl::expressions::parse::PARSE_TREE_FIELD;
use crate::collections::dataset::dsl::expressions::stem::stem_field_expr_from;
use crate::collections::dataset::dsl::expressions::stem::STEM_END_FIELD;
use crate::collections::dataset::dsl::expressions::stem::STEM_KIND_FIELD;
use crate::collections::dataset::dsl::expressions::stem::STEM_START_FIELD;
use crate::collections::dataset::dsl::expressions::stem::STEM_TEXT_FIELD;
use crate::collections::dataset::dsl::expressions::tag::tag_field_expr_from;
use crate::collections::dataset::dsl::expressions::tag::TAG_END_FIELD;
use crate::collections::dataset::dsl::expressions::tag::TAG_START_FIELD;
use crate::collections::dataset::dsl::expressions::tag::TAG_TAG_FIELD;
use crate::collections::dataset::dsl::expressions::tag::TAG_TEXT_FIELD;
use crate::collections::dataset::dsl::expressions::text::lowercase_expr_from;
use crate::collections::dataset::dsl::expressions::text::token_count_expr_from;
use crate::collections::dataset::dsl::expressions::text::tokenize_expr_from;
use crate::collections::dataset::dsl::expressions::token::token_field_expr_from;
use crate::collections::dataset::dsl::expressions::token::TOKEN_END_FIELD;
use crate::collections::dataset::dsl::expressions::token::TOKEN_KIND_FIELD;
use crate::collections::dataset::dsl::expressions::token::TOKEN_START_FIELD;
use crate::collections::dataset::dsl::expressions::token::TOKEN_TEXT_FIELD;

#[derive(Debug, Clone)]
pub struct DatasetExprNameSpace {
    expr: Expr,
}

impl DatasetExprNameSpace {
    pub fn new(expr: Expr) -> Self {
        Self { expr }
    }

    pub fn col(name: &str) -> Self {
        Self::new(col(name))
    }

    pub fn expr(&self) -> Expr {
        self.expr.clone()
    }

    pub fn text(&self) -> DatasetExprText {
        DatasetExprText::new(self.expr.clone())
    }

    pub fn token(&self) -> DatasetExprToken {
        DatasetExprToken::new(self.expr.clone())
    }

    pub fn parse(&self) -> DatasetExprParse {
        DatasetExprParse::new(self.expr.clone())
    }

    pub fn tag(&self) -> DatasetExprTag {
        DatasetExprTag::new(self.expr.clone())
    }

    pub fn stem(&self) -> DatasetExprStem {
        DatasetExprStem::new(self.expr.clone())
    }

    pub fn bin(&self) -> DatasetExprBinary {
        DatasetExprBinary::new(self.expr.clone())
    }
}

pub trait ExprDatasetExt {
    fn ds(self) -> DatasetExprNameSpace;
}

impl ExprDatasetExt for Expr {
    fn ds(self) -> DatasetExprNameSpace {
        DatasetExprNameSpace::new(self)
    }
}

#[derive(Debug, Clone)]
pub struct DatasetExprText {
    expr: Expr,
}

impl DatasetExprText {
    pub fn new(expr: Expr) -> Self {
        Self { expr }
    }

    /// Tokenize a UTF-8 text expression by whitespace.
    pub fn tokenize_ws(&self) -> Expr {
        tokenize_expr_from(self.expr.clone())
    }

    /// Token count (number of whitespace tokens).
    pub fn token_count_ws(&self) -> Expr {
        token_count_expr_from(self.expr.clone())
    }

    /// Lowercase a UTF-8 text expression.
    pub fn lowercase(&self) -> Expr {
        lowercase_expr_from(self.expr.clone())
    }
}

#[derive(Debug, Clone)]
pub struct DatasetExprToken {
    expr: Expr,
}

impl DatasetExprToken {
    pub fn new(expr: Expr) -> Self {
        Self { expr }
    }

    pub fn field(&self, field: &str) -> Expr {
        token_field_expr_from(self.expr.clone(), field)
    }

    pub fn text(&self) -> Expr {
        self.field(TOKEN_TEXT_FIELD)
    }

    pub fn start(&self) -> Expr {
        self.field(TOKEN_START_FIELD)
    }

    pub fn end(&self) -> Expr {
        self.field(TOKEN_END_FIELD)
    }

    pub fn kind(&self) -> Expr {
        self.field(TOKEN_KIND_FIELD)
    }
}

#[derive(Debug, Clone)]
pub struct DatasetExprParse {
    expr: Expr,
}

impl DatasetExprParse {
    pub fn new(expr: Expr) -> Self {
        Self { expr }
    }

    pub fn field(&self, field: &str) -> Expr {
        parse_field_expr_from(self.expr.clone(), field)
    }

    pub fn tree(&self) -> Expr {
        self.field(PARSE_TREE_FIELD)
    }

    pub fn kind(&self) -> Expr {
        self.field(PARSE_KIND_FIELD)
    }

    pub fn start(&self) -> Expr {
        self.field(PARSE_START_FIELD)
    }

    pub fn end(&self) -> Expr {
        self.field(PARSE_END_FIELD)
    }

    pub fn score(&self) -> Expr {
        self.field(PARSE_SCORE_FIELD)
    }
}

#[derive(Debug, Clone)]
pub struct DatasetExprTag {
    expr: Expr,
}

impl DatasetExprTag {
    pub fn new(expr: Expr) -> Self {
        Self { expr }
    }

    pub fn field(&self, field: &str) -> Expr {
        tag_field_expr_from(self.expr.clone(), field)
    }

    pub fn text(&self) -> Expr {
        self.field(TAG_TEXT_FIELD)
    }

    pub fn tag(&self) -> Expr {
        self.field(TAG_TAG_FIELD)
    }

    pub fn start(&self) -> Expr {
        self.field(TAG_START_FIELD)
    }

    pub fn end(&self) -> Expr {
        self.field(TAG_END_FIELD)
    }
}

#[derive(Debug, Clone)]
pub struct DatasetExprStem {
    expr: Expr,
}

impl DatasetExprStem {
    pub fn new(expr: Expr) -> Self {
        Self { expr }
    }

    pub fn field(&self, field: &str) -> Expr {
        stem_field_expr_from(self.expr.clone(), field)
    }

    pub fn text(&self) -> Expr {
        self.field(STEM_TEXT_FIELD)
    }

    pub fn start(&self) -> Expr {
        self.field(STEM_START_FIELD)
    }

    pub fn end(&self) -> Expr {
        self.field(STEM_END_FIELD)
    }

    pub fn kind(&self) -> Expr {
        self.field(STEM_KIND_FIELD)
    }
}

#[derive(Debug, Clone)]
pub struct DatasetExprBinary {
    expr: Expr,
}

impl DatasetExprBinary {
    pub fn new(expr: Expr) -> Self {
        Self { expr }
    }

    pub fn contains(&self, literal: &[u8]) -> Expr {
        binary_contains_literal_from(self.expr.clone(), literal)
    }

    pub fn contains_expr(&self, literal: Expr) -> Expr {
        binary_contains_expr_from(self.expr.clone(), literal)
    }

    pub fn starts_with(&self, prefix: &[u8]) -> Expr {
        binary_starts_with_literal_from(self.expr.clone(), prefix)
    }

    pub fn starts_with_expr(&self, prefix: Expr) -> Expr {
        binary_starts_with_expr_from(self.expr.clone(), prefix)
    }

    pub fn ends_with(&self, suffix: &[u8]) -> Expr {
        binary_ends_with_literal_from(self.expr.clone(), suffix)
    }

    pub fn ends_with_expr(&self, suffix: Expr) -> Expr {
        binary_ends_with_expr_from(self.expr.clone(), suffix)
    }

    pub fn encode(&self, encoding: BinaryEncoding) -> Expr {
        binary_encode_from(self.expr.clone(), encoding)
    }

    pub fn decode(&self, encoding: BinaryEncoding, strict: bool) -> Expr {
        binary_decode_from(self.expr.clone(), encoding, strict)
    }

    pub fn size_bytes(&self) -> Expr {
        binary_size_bytes_from(self.expr.clone())
    }

    pub fn size(&self, unit: BinarySizeUnit) -> Expr {
        binary_size_from(self.expr.clone(), unit)
    }
}
