//! Parse DSL core types.

use polars::prelude::{lit, AnyValue, DataType, Expr, LiteralValue, Scalar};

use crate::collections::dataframe::record;
use crate::collections::dataset::token::TokenSpan;
use crate::collections::dataset::tree::TreeValue;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParseKind {
    Constituency,
    Dependency,
    Unknown,
}

impl ParseKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            ParseKind::Constituency => "constituency",
            ParseKind::Dependency => "dependency",
            ParseKind::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parse {
    tree: TreeValue,
    span: Option<TokenSpan>,
    kind: ParseKind,
    score: Option<f64>,
}

impl Parse {
    pub fn new(tree: TreeValue, kind: ParseKind) -> Self {
        Self {
            tree,
            span: None,
            kind,
            score: None,
        }
    }

    pub fn tree(&self) -> &TreeValue {
        &self.tree
    }

    pub fn span(&self) -> Option<TokenSpan> {
        self.span
    }

    pub fn kind(&self) -> &ParseKind {
        &self.kind
    }

    pub fn score(&self) -> Option<f64> {
        self.score
    }

    pub fn with_tree(mut self, tree: TreeValue) -> Self {
        self.tree = tree;
        self
    }

    pub fn with_span(mut self, span: TokenSpan) -> Self {
        self.span = Some(span);
        self
    }

    pub fn with_kind(mut self, kind: ParseKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn with_score(mut self, score: f64) -> Self {
        self.score = Some(score);
        self
    }

    pub fn to_struct_expr(&self) -> Expr {
        let start = self.span.map(|span| span.start() as u64);
        let end = self.span.map(|span| span.end() as u64);
        let start_expr = match start {
            Some(value) => lit(value),
            None => null_expr(),
        };
        let end_expr = match end {
            Some(value) => lit(value),
            None => null_expr(),
        };
        let score_expr = match self.score {
            Some(value) => lit(value),
            None => null_expr(),
        };
        record(vec![
            lit(self.tree.format_bracketed()).alias("tree"),
            lit(self.kind.as_str()).alias("kind"),
            start_expr.alias("start"),
            end_expr.alias("end"),
            score_expr.alias("score"),
        ])
    }
}

pub type ParseForest = Vec<Parse>;

fn null_expr() -> Expr {
    Expr::Literal(LiteralValue::Scalar(Scalar::new(
        DataType::Null,
        AnyValue::Null,
    )))
}
