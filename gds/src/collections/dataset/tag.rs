//! Tag DSL core types.

use polars::prelude::{lit, Expr};

use crate::collections::dataframe::record;
use crate::collections::dataset::token::TokenSpan;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tag {
    text: String,
    tag: String,
    span: TokenSpan,
}

impl Tag {
    pub fn new(text: impl Into<String>, tag: impl Into<String>, span: TokenSpan) -> Self {
        Self {
            text: text.into(),
            tag: tag.into(),
            span,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn tag(&self) -> &str {
        &self.tag
    }

    pub fn span(&self) -> TokenSpan {
        self.span
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = tag.into();
        self
    }

    pub fn with_span(mut self, span: TokenSpan) -> Self {
        self.span = span;
        self
    }

    pub fn to_struct_expr(&self) -> Expr {
        record(vec![
            lit(self.text.clone()).alias("text"),
            lit(self.tag.clone()).alias("tag"),
            lit(self.span.start() as u64).alias("start"),
            lit(self.span.end() as u64).alias("end"),
        ])
    }
}
