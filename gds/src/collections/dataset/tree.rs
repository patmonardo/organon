//! Dataset Tree DSL entry point.
//!
//! This module re-exports the Tree namespace, expressions, and functionals.
//! It also provides thin helpers used by tree macros.

use std::collections::BTreeMap;

use crate::collections::dataset::expressions::tree as tree_expr;
use crate::collections::dataset::functions::tree::transform;
use crate::collections::dataset::namespaces::tree::TreeNs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TreeId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeNode {
    label: tree_expr::TreeLabel,
    children: Vec<TreeValue>,
    span: Option<tree_expr::TreeSpan>,
    id: Option<TreeId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TreeValue {
    Node(TreeNode),
    Leaf(TreeLeafValue),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TreeLeafValue {
    Text(String),
    TokenIndex(usize),
    Number(i64),
    Bool(bool),
    BytesRange { start: usize, end: usize },
    Empty,
}

impl TreeNode {
    pub fn new(label: impl Into<tree_expr::TreeLabel>, children: Vec<TreeValue>) -> Self {
        Self {
            label: label.into(),
            children,
            span: None,
            id: None,
        }
    }

    pub fn with_span(mut self, span: tree_expr::TreeSpan) -> Self {
        self.span = Some(span);
        self
    }

    pub fn with_id(mut self, id: TreeId) -> Self {
        self.id = Some(id);
        self
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn children(&self) -> &[TreeValue] {
        &self.children
    }

    pub fn span(&self) -> Option<&tree_expr::TreeSpan> {
        self.span.as_ref()
    }

    pub fn id(&self) -> Option<TreeId> {
        self.id
    }

    pub fn with_children(mut self, children: Vec<TreeValue>) -> Self {
        self.children = children;
        self
    }
}

impl TreeValue {
    pub fn node(label: impl Into<tree_expr::TreeLabel>, children: Vec<TreeValue>) -> Self {
        TreeValue::Node(TreeNode::new(label, children))
    }

    pub fn leaf(value: impl Into<TreeLeafValue>) -> Self {
        TreeValue::Leaf(value.into())
    }

    pub fn is_node(&self) -> bool {
        matches!(self, TreeValue::Node(_))
    }

    pub fn is_leaf(&self) -> bool {
        matches!(self, TreeValue::Leaf(_))
    }

    pub fn as_node(&self) -> Option<&TreeNode> {
        if let TreeValue::Node(node) = self {
            Some(node)
        } else {
            None
        }
    }

    pub fn as_leaf(&self) -> Option<&TreeLeafValue> {
        if let TreeValue::Leaf(value) = self {
            Some(value)
        } else {
            None
        }
    }
}

impl TreeLeafValue {
    pub fn text(value: impl Into<String>) -> Self {
        TreeLeafValue::Text(value.into())
    }

    pub fn token_index(index: usize) -> Self {
        TreeLeafValue::TokenIndex(index)
    }

    pub fn bytes_range(start: usize, end: usize) -> Self {
        TreeLeafValue::BytesRange { start, end }
    }
}

impl From<String> for TreeLeafValue {
    fn from(value: String) -> Self {
        TreeLeafValue::Text(value)
    }
}

impl From<&str> for TreeLeafValue {
    fn from(value: &str) -> Self {
        TreeLeafValue::Text(value.to_string())
    }
}

impl From<usize> for TreeLeafValue {
    fn from(value: usize) -> Self {
        TreeLeafValue::TokenIndex(value)
    }
}

impl From<i64> for TreeLeafValue {
    fn from(value: i64) -> Self {
        TreeLeafValue::Number(value)
    }
}

impl From<bool> for TreeLeafValue {
    fn from(value: bool) -> Self {
        TreeLeafValue::Bool(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TreeIndex(pub usize);

#[derive(Debug, Clone, Default)]
pub struct TreeCollection {
    trees: Vec<TreeValue>,
    ids: Vec<TreeId>,
    id_index: BTreeMap<TreeId, TreeIndex>,
}

impl TreeCollection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_values(values: Vec<TreeValue>) -> Self {
        let mut out = Self::default();
        for value in values {
            out.push(value, None);
        }
        out
    }

    pub fn push(&mut self, tree: TreeValue, id: Option<TreeId>) -> TreeIndex {
        let index = TreeIndex(self.trees.len());
        if let Some(id) = id {
            self.ids.push(id);
            self.id_index.insert(id, index);
        } else {
            self.ids.push(TreeId(self.trees.len() as u64));
        }
        self.trees.push(tree);
        index
    }

    pub fn len(&self) -> usize {
        self.trees.len()
    }

    pub fn is_empty(&self) -> bool {
        self.trees.is_empty()
    }

    pub fn get(&self, index: TreeIndex) -> Option<&TreeValue> {
        self.trees.get(index.0)
    }

    pub fn get_by_id(&self, id: TreeId) -> Option<&TreeValue> {
        self.id_index.get(&id).and_then(|idx| self.get(*idx))
    }

    pub fn iter(&self) -> impl Iterator<Item = &TreeValue> {
        self.trees.iter()
    }

    pub fn map(&self, f: impl Fn(&TreeValue) -> TreeValue) -> Self {
        let values = self.trees.iter().map(f).collect::<Vec<TreeValue>>();
        Self::from_values(values)
    }

    pub fn normalize(&self) -> Self {
        self.map(transform::normalize)
    }

    pub fn collapse_unary(&self) -> Self {
        self.map(transform::collapse_unary)
    }

    pub fn chomsky_normal_form(&self) -> Self {
        self.map(transform::chomsky_normal_form)
    }

    pub fn normalize_with(&self, options: &transform::NormalizeOptions) -> Self {
        self.map(|tree| transform::normalize_with(tree, options))
    }

    pub fn collapse_unary_with(&self, options: &transform::CollapseUnaryOptions) -> Self {
        self.map(|tree| transform::collapse_unary_with(tree, options))
    }

    pub fn chomsky_normal_form_with(&self, options: &transform::CnfOptions) -> Self {
        self.map(|tree| transform::chomsky_normal_form_with(tree, options))
    }

    pub fn series(&self) -> TreeSeriesNameSpace {
        TreeSeriesNameSpace::new(self.clone())
    }

    pub fn into_series(self) -> TreeSeries {
        TreeSeries::new(self)
    }
}

#[derive(Debug, Clone)]
pub struct TreeSeries {
    collection: TreeCollection,
}

impl TreeSeries {
    pub fn new(collection: TreeCollection) -> Self {
        Self { collection }
    }

    pub fn collection(&self) -> &TreeCollection {
        &self.collection
    }

    pub fn into_collection(self) -> TreeCollection {
        self.collection
    }

    pub fn tree(&self) -> TreeSeriesNameSpace {
        TreeSeriesNameSpace::new(self.collection.clone())
    }
}

#[derive(Debug, Clone)]
pub struct TreeSeriesNameSpace {
    collection: TreeCollection,
}

impl TreeSeriesNameSpace {
    pub fn new(collection: TreeCollection) -> Self {
        Self { collection }
    }

    pub fn normalize(&self) -> TreeSeries {
        TreeSeries::new(self.collection.normalize())
    }

    pub fn collapse_unary(&self) -> TreeSeries {
        TreeSeries::new(self.collection.collapse_unary())
    }

    pub fn chomsky_normal_form(&self) -> TreeSeries {
        TreeSeries::new(self.collection.chomsky_normal_form())
    }

    pub fn normalize_with(&self, options: &transform::NormalizeOptions) -> TreeSeries {
        TreeSeries::new(self.collection.normalize_with(options))
    }

    pub fn collapse_unary_with(&self, options: &transform::CollapseUnaryOptions) -> TreeSeries {
        TreeSeries::new(self.collection.collapse_unary_with(options))
    }

    pub fn chomsky_normal_form_with(&self, options: &transform::CnfOptions) -> TreeSeries {
        TreeSeries::new(self.collection.chomsky_normal_form_with(options))
    }
}

pub fn node_expr(
    label: impl Into<tree_expr::TreeLabel>,
    children: Vec<tree_expr::TreeExpr>,
) -> tree_expr::TreeExpr {
    TreeNs::node(label, children)
}

pub fn leaf_expr(value: impl Into<tree_expr::TreeLeafExpr>) -> tree_expr::TreeExpr {
    TreeNs::leaf(value)
}

pub fn pos_expr(path: impl Into<Vec<usize>>) -> tree_expr::TreeExpr {
    TreeNs::pos(path)
}

pub fn span_expr(start: usize, end: usize) -> tree_expr::TreeExpr {
    TreeNs::span(start, end)
}

pub fn transform_expr(input: tree_expr::TreeExpr, op: tree_expr::TreeOp) -> tree_expr::TreeExpr {
    TreeNs::transform(input, op)
}

pub use crate::collections::dataset::expressions::tree::{
    TreeExpr, TreeLabel, TreeLeafExpr, TreeOp, TreePos, TreeSpan,
};
pub use crate::collections::dataset::functions::tree as tree_fn;
pub use crate::collections::dataset::namespaces::tree::TreeNs as TreeNamespace;
