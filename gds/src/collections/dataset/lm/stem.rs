//! Stem DSL core types.

use polars::prelude::{lit, Expr};

use crate::collections::dataframe::record;
use crate::collections::dataset::token::TokenSpan;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StemKind {
    Stem,
    Lemma,
    Root,
    Unknown,
}

impl StemKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            StemKind::Stem => "stem",
            StemKind::Lemma => "lemma",
            StemKind::Root => "root",
            StemKind::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Stem {
    text: String,
    span: TokenSpan,
    kind: StemKind,
}

impl Stem {
    pub fn new(text: impl Into<String>, span: TokenSpan, kind: StemKind) -> Self {
        Self {
            text: text.into(),
            span,
            kind,
        }
    }

    pub fn stem(text: impl Into<String>, span: TokenSpan) -> Self {
        Self::new(text, span, StemKind::Stem)
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn span(&self) -> TokenSpan {
        self.span
    }

    pub fn kind(&self) -> &StemKind {
        &self.kind
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    pub fn with_span(mut self, span: TokenSpan) -> Self {
        self.span = span;
        self
    }

    pub fn with_kind(mut self, kind: StemKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn to_struct_expr(&self) -> Expr {
        record(vec![
            lit(self.text.clone()).alias("text"),
            lit(self.span.start() as u64).alias("start"),
            lit(self.span.end() as u64).alias("end"),
            lit(self.kind.as_str()).alias("kind"),
        ])
    }
}
