//! Dataset-level expression facade.
//!
//! This module provides dataset-flavored `Expr` namespaces.
//!
//! Matrix alignment:
//! - `Expr` is to `LazyFrame` as `Series` is to `DataFrame`.
//! - This file provides the dataset-side `Expr` half of the `Expr`↔`LazyFrame` pair.

use polars::prelude::{col, Expr};

use crate::collections::dataset::expressions::parse::{
    parse_field_expr_from, PARSE_END_FIELD, PARSE_KIND_FIELD, PARSE_SCORE_FIELD, PARSE_START_FIELD,
    PARSE_TREE_FIELD,
};
use crate::collections::dataset::expressions::stem::{
    stem_field_expr_from, STEM_END_FIELD, STEM_KIND_FIELD, STEM_START_FIELD, STEM_TEXT_FIELD,
};
use crate::collections::dataset::expressions::tag::{
    tag_field_expr_from, TAG_END_FIELD, TAG_START_FIELD, TAG_TAG_FIELD, TAG_TEXT_FIELD,
};
use crate::collections::dataset::expressions::text::{
    lowercase_expr_from, token_count_expr_from, tokenize_expr_from,
};
use crate::collections::dataset::expressions::token::{
    token_field_expr_from, TOKEN_END_FIELD, TOKEN_KIND_FIELD, TOKEN_START_FIELD, TOKEN_TEXT_FIELD,
};

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
