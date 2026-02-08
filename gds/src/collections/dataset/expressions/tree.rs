//! Tree expressions for dataset-level DSL.
//!
//! This module defines the Tree expression AST used by the Tree namespace.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TreeExpr {
    Node {
        label: TreeLabel,
        children: Vec<TreeExpr>,
    },
    Leaf(TreeLeafExpr),
    Pos(TreePos),
    Span(TreeSpan),
    Transform {
        input: Box<TreeExpr>,
        op: TreeOp,
    },
}

pub type TreeLabel = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TreeLeafExpr {
    Text(String),
    TokenIndex(usize),
    Number(i64),
    Bool(bool),
    BytesRange { start: usize, end: usize },
    Empty,
}

impl TreeLeafExpr {
    pub fn text(value: impl Into<String>) -> Self {
        TreeLeafExpr::Text(value.into())
    }

    pub fn token_index(index: usize) -> Self {
        TreeLeafExpr::TokenIndex(index)
    }

    pub fn bytes_range(start: usize, end: usize) -> Self {
        TreeLeafExpr::BytesRange { start, end }
    }
}

impl From<String> for TreeLeafExpr {
    fn from(value: String) -> Self {
        TreeLeafExpr::Text(value)
    }
}

impl From<&str> for TreeLeafExpr {
    fn from(value: &str) -> Self {
        TreeLeafExpr::Text(value.to_string())
    }
}

impl From<usize> for TreeLeafExpr {
    fn from(value: usize) -> Self {
        TreeLeafExpr::TokenIndex(value)
    }
}

impl From<i64> for TreeLeafExpr {
    fn from(value: i64) -> Self {
        TreeLeafExpr::Number(value)
    }
}

impl From<bool> for TreeLeafExpr {
    fn from(value: bool) -> Self {
        TreeLeafExpr::Bool(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TreePos {
    path: Vec<usize>,
}

impl TreePos {
    pub fn new(path: impl Into<Vec<usize>>) -> Self {
        Self { path: path.into() }
    }

    pub fn path(&self) -> &[usize] {
        &self.path
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TreeSpan {
    start: usize,
    end: usize,
}

impl TreeSpan {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TreeOp {
    Normalize,
    CollapseUnary,
    ChomskyNormalForm,
}
