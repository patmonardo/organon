//! Token DSL core types.

use polars::prelude::{lit, Expr};

use crate::collections::dataframe::record;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TokenSpan {
    start: usize,
    end: usize,
}

impl TokenSpan {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }

    pub fn is_empty(&self) -> bool {
        self.start >= self.end
    }

    pub fn contains(&self, offset: usize) -> bool {
        offset >= self.start && offset < self.end
    }

    pub fn shift(&self, delta: isize) -> Self {
        let start = shift_offset(self.start, delta);
        let end = shift_offset(self.end, delta);
        Self { start, end }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Word,
    Punct,
    Number,
    Symbol,
    Whitespace,
    Unknown,
}

impl TokenKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            TokenKind::Word => "word",
            TokenKind::Punct => "punct",
            TokenKind::Number => "number",
            TokenKind::Symbol => "symbol",
            TokenKind::Whitespace => "whitespace",
            TokenKind::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
    text: String,
    span: TokenSpan,
    kind: TokenKind,
}

impl Token {
    pub fn new(text: impl Into<String>, span: TokenSpan, kind: TokenKind) -> Self {
        Self {
            text: text.into(),
            span,
            kind,
        }
    }

    pub fn word(text: impl Into<String>, span: TokenSpan) -> Self {
        Self::new(text, span, TokenKind::Word)
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn span(&self) -> TokenSpan {
        self.span
    }

    pub fn kind(&self) -> &TokenKind {
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

    pub fn with_kind(mut self, kind: TokenKind) -> Self {
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

fn shift_offset(offset: usize, delta: isize) -> usize {
    if delta >= 0 {
        offset.saturating_add(delta as usize)
    } else {
        offset.saturating_sub(delta.unsigned_abs())
    }
}
