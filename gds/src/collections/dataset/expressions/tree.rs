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

impl TreeExpr {
    pub fn node(label: impl Into<TreeLabel>, children: Vec<TreeExpr>) -> Self {
        TreeExpr::Node {
            label: label.into(),
            children,
        }
    }

    pub fn leaf(value: impl Into<TreeLeafExpr>) -> Self {
        TreeExpr::Leaf(value.into())
    }

    pub fn pos(path: impl Into<Vec<usize>>) -> Self {
        TreeExpr::Pos(TreePos::new(path))
    }

    pub fn span(start: usize, end: usize) -> Self {
        TreeExpr::Span(TreeSpan::new(start, end))
    }

    pub fn transform(input: TreeExpr, op: TreeOp) -> Self {
        TreeExpr::Transform {
            input: Box::new(input),
            op,
        }
    }

    pub fn is_node(&self) -> bool {
        matches!(self, TreeExpr::Node { .. })
    }

    pub fn is_leaf(&self) -> bool {
        matches!(self, TreeExpr::Leaf(_))
    }

    pub fn is_pos(&self) -> bool {
        matches!(self, TreeExpr::Pos(_))
    }

    pub fn is_span(&self) -> bool {
        matches!(self, TreeExpr::Span(_))
    }

    pub fn is_transform(&self) -> bool {
        matches!(self, TreeExpr::Transform { .. })
    }

    pub fn label(&self) -> Option<&TreeLabel> {
        match self {
            TreeExpr::Node { label, .. } => Some(label),
            _ => None,
        }
    }

    pub fn children(&self) -> Option<&[TreeExpr]> {
        match self {
            TreeExpr::Node { children, .. } => Some(children),
            _ => None,
        }
    }

    pub fn children_mut(&mut self) -> Option<&mut Vec<TreeExpr>> {
        match self {
            TreeExpr::Node { children, .. } => Some(children),
            _ => None,
        }
    }

    pub fn input(&self) -> Option<&TreeExpr> {
        match self {
            TreeExpr::Transform { input, .. } => Some(input),
            _ => None,
        }
    }

    pub fn op(&self) -> Option<&TreeOp> {
        match self {
            TreeExpr::Transform { op, .. } => Some(op),
            _ => None,
        }
    }

    pub fn visit<F>(&self, mut f: F)
    where
        F: FnMut(&TreeExpr),
    {
        self.visit_inner(&mut f);
    }

    pub fn fold<T, F>(&self, init: T, mut f: F) -> T
    where
        F: FnMut(T, &TreeExpr) -> T,
    {
        self.fold_inner(init, &mut f)
    }

    pub fn map<F>(self, mut f: F) -> TreeExpr
    where
        F: FnMut(TreeExpr) -> TreeExpr,
    {
        self.map_inner(&mut f)
    }

    fn visit_inner<F>(&self, f: &mut F)
    where
        F: FnMut(&TreeExpr),
    {
        f(self);
        match self {
            TreeExpr::Node { children, .. } => {
                for child in children {
                    child.visit_inner(f);
                }
            }
            TreeExpr::Transform { input, .. } => input.visit_inner(f),
            TreeExpr::Leaf(_) | TreeExpr::Pos(_) | TreeExpr::Span(_) => {}
        }
    }

    fn fold_inner<T, F>(&self, init: T, f: &mut F) -> T
    where
        F: FnMut(T, &TreeExpr) -> T,
    {
        let mut acc = f(init, self);
        match self {
            TreeExpr::Node { children, .. } => {
                for child in children {
                    acc = child.fold_inner(acc, f);
                }
            }
            TreeExpr::Transform { input, .. } => {
                acc = input.fold_inner(acc, f);
            }
            TreeExpr::Leaf(_) | TreeExpr::Pos(_) | TreeExpr::Span(_) => {}
        }
        acc
    }

    fn map_inner<F>(self, f: &mut F) -> TreeExpr
    where
        F: FnMut(TreeExpr) -> TreeExpr,
    {
        let mapped = match self {
            TreeExpr::Node { label, children } => {
                let children = children
                    .into_iter()
                    .map(|child| child.map_inner(f))
                    .collect();
                TreeExpr::Node { label, children }
            }
            TreeExpr::Transform { input, op } => TreeExpr::Transform {
                input: Box::new(input.map_inner(f)),
                op,
            },
            other => other,
        };
        f(mapped)
    }
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
