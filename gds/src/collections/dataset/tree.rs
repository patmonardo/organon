//! Dataset Tree DSL entry point.
//!
//! This module re-exports the Tree namespace, expressions, and functionals.
//! It also provides thin helpers used by tree macros.

use std::collections::BTreeMap;

use crate::collections::dataset::expressions::tree as tree_expr;
use crate::collections::dataset::featstruct::FeatStruct;
use crate::collections::dataset::functions::tree::{format, inspect, pretty, transform};
use crate::collections::dataset::namespaces::tree::TreeNs;
use crate::collections::dataset::tag::Tag;
use crate::collections::dataset::token::TokenSpan;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TreeId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeNode {
    label: tree_expr::TreeLabel,
    children: Vec<TreeValue>,
    span: Option<tree_expr::TreeSpan>,
    id: Option<TreeId>,
    attributes: Option<FeatStruct>,
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

#[derive(Debug, Clone, PartialEq)]
pub struct ProbabilisticTree {
    tree: TreeValue,
    prob: f64,
}

impl ProbabilisticTree {
    pub fn new(tree: TreeValue, prob: f64) -> Self {
        Self { tree, prob }
    }

    pub fn tree(&self) -> &TreeValue {
        &self.tree
    }

    pub fn into_tree(self) -> TreeValue {
        self.tree
    }

    pub fn prob(&self) -> f64 {
        self.prob
    }

    pub fn with_prob(mut self, prob: f64) -> Self {
        self.prob = prob;
        self
    }

    pub fn copy(&self) -> Self {
        Self {
            tree: self.tree.clone(),
            prob: self.prob,
        }
    }

    pub fn convert(tree: &TreeValue, prob: f64) -> Self {
        Self {
            tree: tree.clone(),
            prob,
        }
    }

    pub fn format_bracketed(&self) -> String {
        self.tree.format_bracketed()
    }

    pub fn format_pretty(&self, indent: usize) -> String {
        self.tree.format_pretty(indent)
    }

    pub fn pretty_print(&self, options: pretty::PrettyOptions) -> String {
        pretty::pretty_print(&self.tree, options)
    }
}

impl PartialOrd for ProbabilisticTree {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let a = self.format_bracketed();
        let b = other.format_bracketed();
        match a.partial_cmp(&b) {
            Some(std::cmp::Ordering::Equal) => Some(self.prob.total_cmp(&other.prob)),
            other => other,
        }
    }
}

impl ProbabilisticTree {
    pub fn compare(&self, other: &Self) -> std::cmp::Ordering {
        let a = self.format_bracketed();
        let b = other.format_bracketed();
        match a.cmp(&b) {
            std::cmp::Ordering::Equal => self.prob.total_cmp(&other.prob),
            other => other,
        }
    }
}

impl std::fmt::Display for ProbabilisticTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (p={:.6})", self.format_bracketed(), self.prob)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ParentedIndex(pub usize);

#[derive(Debug, Clone)]
pub struct ParentedTree {
    nodes: Vec<ParentedNode>,
    root: ParentedIndex,
}

#[derive(Debug, Clone)]
pub struct ParentedNode {
    value: ParentedValue,
    parent: Option<ParentedIndex>,
}

#[derive(Debug, Clone)]
pub enum ParentedValue {
    Node {
        label: tree_expr::TreeLabel,
        children: Vec<ParentedIndex>,
    },
    Leaf(TreeLeafValue),
}

impl ParentedTree {
    pub fn from_tree(tree: &TreeValue) -> Self {
        let mut nodes = Vec::new();
        let root = build_parented(tree, None, &mut nodes);
        Self { nodes, root }
    }

    pub fn root(&self) -> ParentedIndex {
        self.root
    }

    pub fn get(&self, index: ParentedIndex) -> Option<&ParentedNode> {
        self.nodes.get(index.0)
    }

    pub fn parent(&self, index: ParentedIndex) -> Option<ParentedIndex> {
        self.get(index).and_then(|node| node.parent)
    }

    pub fn parent_index(&self, index: ParentedIndex) -> Option<usize> {
        let parent = self.parent(index)?;
        let parent_node = self.get(parent)?;
        match &parent_node.value {
            ParentedValue::Node { children, .. } => {
                children.iter().position(|child| *child == index)
            }
            ParentedValue::Leaf(_) => None,
        }
    }

    pub fn left_sibling(&self, index: ParentedIndex) -> Option<ParentedIndex> {
        let parent = self.parent(index)?;
        let parent_node = self.get(parent)?;
        if let ParentedValue::Node { children, .. } = &parent_node.value {
            let pos = children.iter().position(|child| *child == index)?;
            if pos > 0 {
                return Some(children[pos - 1]);
            }
        }
        None
    }

    pub fn right_sibling(&self, index: ParentedIndex) -> Option<ParentedIndex> {
        let parent = self.parent(index)?;
        let parent_node = self.get(parent)?;
        if let ParentedValue::Node { children, .. } = &parent_node.value {
            let pos = children.iter().position(|child| *child == index)?;
            if pos + 1 < children.len() {
                return Some(children[pos + 1]);
            }
        }
        None
    }

    pub fn treeposition(&self, index: ParentedIndex) -> Vec<usize> {
        let mut path = Vec::new();
        let mut current = index;
        while let Some(parent) = self.parent(current) {
            if let Some(pos) = self.parent_index(current) {
                path.push(pos);
            }
            current = parent;
        }
        path.reverse();
        path
    }
}

impl ParentedNode {
    pub fn value(&self) -> &ParentedValue {
        &self.value
    }

    pub fn parent(&self) -> Option<ParentedIndex> {
        self.parent
    }

    pub fn children(&self) -> Option<&[ParentedIndex]> {
        match &self.value {
            ParentedValue::Node { children, .. } => Some(children),
            ParentedValue::Leaf(_) => None,
        }
    }

    pub fn label(&self) -> Option<&str> {
        match &self.value {
            ParentedValue::Node { label, .. } => Some(label),
            ParentedValue::Leaf(_) => None,
        }
    }
}

fn build_parented(
    tree: &TreeValue,
    parent: Option<ParentedIndex>,
    nodes: &mut Vec<ParentedNode>,
) -> ParentedIndex {
    let index = ParentedIndex(nodes.len());
    let value = match tree {
        TreeValue::Leaf(value) => ParentedValue::Leaf(value.clone()),
        TreeValue::Node(node) => {
            let mut children = Vec::new();
            for child in node.children() {
                let child_index = build_parented(child, Some(index), nodes);
                children.push(child_index);
            }
            ParentedValue::Node {
                label: node.label().to_string(),
                children,
            }
        }
    };
    nodes.push(ParentedNode { value, parent });
    index
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MultiParentedIndex(pub usize);

#[derive(Debug, Clone)]
pub struct MultiParentedTree {
    nodes: Vec<MultiParentedNode>,
    root: MultiParentedIndex,
}

#[derive(Debug, Clone)]
pub struct MultiParentedNode {
    value: MultiParentedValue,
    parents: Vec<MultiParentedIndex>,
}

#[derive(Debug, Clone)]
pub enum MultiParentedValue {
    Node {
        label: tree_expr::TreeLabel,
        children: Vec<MultiParentedIndex>,
    },
    Leaf(TreeLeafValue),
}

impl MultiParentedTree {
    pub fn from_tree(tree: &TreeValue) -> Self {
        let mut nodes = Vec::new();
        let root = build_multi_parented(tree, None, &mut nodes);
        Self { nodes, root }
    }

    pub fn root(&self) -> MultiParentedIndex {
        self.root
    }

    pub fn get(&self, index: MultiParentedIndex) -> Option<&MultiParentedNode> {
        self.nodes.get(index.0)
    }

    pub fn parents(&self, index: MultiParentedIndex) -> &[MultiParentedIndex] {
        self.get(index)
            .map(|node| node.parents.as_slice())
            .unwrap_or(&[])
    }

    pub fn parent_indices(
        &self,
        index: MultiParentedIndex,
        parent: MultiParentedIndex,
    ) -> Vec<usize> {
        let mut out = Vec::new();
        let parent_node = match self.get(parent) {
            Some(node) => node,
            None => return out,
        };
        if let MultiParentedValue::Node { children, .. } = &parent_node.value {
            for (pos, child) in children.iter().enumerate() {
                if *child == index {
                    out.push(pos);
                }
            }
        }
        out
    }

    pub fn roots(&self) -> Vec<MultiParentedIndex> {
        self.nodes
            .iter()
            .enumerate()
            .filter_map(|(idx, node)| {
                if node.parents.is_empty() {
                    Some(MultiParentedIndex(idx))
                } else {
                    None
                }
            })
            .collect()
    }
}

impl MultiParentedNode {
    pub fn value(&self) -> &MultiParentedValue {
        &self.value
    }

    pub fn parents(&self) -> &[MultiParentedIndex] {
        &self.parents
    }

    pub fn children(&self) -> Option<&[MultiParentedIndex]> {
        match &self.value {
            MultiParentedValue::Node { children, .. } => Some(children),
            MultiParentedValue::Leaf(_) => None,
        }
    }

    pub fn label(&self) -> Option<&str> {
        match &self.value {
            MultiParentedValue::Node { label, .. } => Some(label),
            MultiParentedValue::Leaf(_) => None,
        }
    }
}

fn build_multi_parented(
    tree: &TreeValue,
    parent: Option<MultiParentedIndex>,
    nodes: &mut Vec<MultiParentedNode>,
) -> MultiParentedIndex {
    let index = MultiParentedIndex(nodes.len());
    let value = match tree {
        TreeValue::Leaf(value) => MultiParentedValue::Leaf(value.clone()),
        TreeValue::Node(node) => {
            let mut children = Vec::new();
            for child in node.children() {
                let child_index = build_multi_parented(child, Some(index), nodes);
                children.push(child_index);
            }
            MultiParentedValue::Node {
                label: node.label().to_string(),
                children,
            }
        }
    };
    let mut parents = Vec::new();
    if let Some(parent) = parent {
        parents.push(parent);
    }
    nodes.push(MultiParentedNode { value, parents });
    index
}

impl TreeNode {
    pub fn new(label: impl Into<tree_expr::TreeLabel>, children: Vec<TreeValue>) -> Self {
        Self {
            label: label.into(),
            children,
            span: None,
            id: None,
            attributes: None,
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

    pub fn attributes(&self) -> Option<&FeatStruct> {
        self.attributes.as_ref()
    }

    pub fn with_children(mut self, children: Vec<TreeValue>) -> Self {
        self.children = children;
        self
    }

    pub fn with_attributes(mut self, attributes: FeatStruct) -> Self {
        self.attributes = Some(attributes);
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

    pub fn from_tags(root_label: impl Into<tree_expr::TreeLabel>, tags: &[Tag]) -> Self {
        let children = tags
            .iter()
            .map(|tag| {
                TreeValue::Node(
                    TreeNode::new(
                        tag.tag(),
                        vec![TreeValue::leaf(TreeLeafValue::text(tag.text()))],
                    )
                    .with_span(tree_expr::TreeSpan::new(
                        tag.span().start(),
                        tag.span().end(),
                    )),
                )
            })
            .collect();
        TreeValue::node(root_label, children)
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

    pub fn leaves(&self) -> Vec<TreeLeafValue> {
        inspect::leaves(self)
    }

    pub fn height(&self) -> usize {
        inspect::height(self)
    }

    pub fn treepositions_preorder(&self) -> Vec<tree_expr::TreePos> {
        inspect::treepositions(self, inspect::TreeTraversal::PreOrder)
    }

    pub fn treepositions_postorder(&self) -> Vec<tree_expr::TreePos> {
        inspect::treepositions(self, inspect::TreeTraversal::PostOrder)
    }

    pub fn treepositions_bothorder(&self) -> Vec<tree_expr::TreePos> {
        inspect::treepositions(self, inspect::TreeTraversal::BothOrder)
    }

    pub fn treepositions_leaves(&self) -> Vec<tree_expr::TreePos> {
        inspect::treepositions(self, inspect::TreeTraversal::Leaves)
    }

    pub fn leaf_treeposition(&self, index: usize) -> Option<tree_expr::TreePos> {
        inspect::leaf_treeposition(self, index)
    }

    pub fn treeposition_spanning_leaves(
        &self,
        start: usize,
        end: usize,
    ) -> Option<tree_expr::TreePos> {
        inspect::treeposition_spanning_leaves(self, start, end)
    }

    pub fn format_bracketed(&self) -> String {
        format::format_bracketed(self)
    }

    pub fn format_pretty(&self, indent: usize) -> String {
        format::format_pretty(self, indent)
    }

    pub fn pretty_print(&self, options: pretty::PrettyOptions) -> String {
        pretty::pretty_print(self, options)
    }

    pub fn align_tags_by_index(&self, tags: &[Tag]) -> Vec<Option<Tag>> {
        self.leaves()
            .into_iter()
            .map(|leaf| match leaf {
                TreeLeafValue::TokenIndex(index) => tags.get(index).cloned(),
                _ => None,
            })
            .collect()
    }

    pub fn align_tags_by_span(&self, tags: &[Tag]) -> Vec<Option<Tag>> {
        self.leaves()
            .into_iter()
            .map(|leaf| match leaf {
                TreeLeafValue::TokenIndex(index) => tags.get(index).cloned(),
                TreeLeafValue::BytesRange { start, end } => tags
                    .iter()
                    .find(|tag| tag.span().start() == start && tag.span().end() == end)
                    .cloned(),
                _ => None,
            })
            .collect()
    }

    pub fn to_preterminal_tags(&self) -> Vec<Tag> {
        let mut out = Vec::new();
        collect_preterminal_tags(self, &mut out);
        out
    }
}

fn collect_preterminal_tags(tree: &TreeValue, out: &mut Vec<Tag>) {
    if let TreeValue::Node(node) = tree {
        if node.children().len() == 1 {
            let child = &node.children()[0];
            if let Some(text) = leaf_text(child) {
                let span = node
                    .span()
                    .map(|span| TokenSpan::new(span.start(), span.end()))
                    .or_else(|| leaf_span(child).map(|(start, end)| TokenSpan::new(start, end)))
                    .unwrap_or_else(|| TokenSpan::new(0, 0));
                out.push(Tag::new(text, node.label(), span));
                return;
            }
        }
        for child in node.children() {
            collect_preterminal_tags(child, out);
        }
    }
}

fn leaf_text(tree: &TreeValue) -> Option<&str> {
    match tree {
        TreeValue::Leaf(TreeLeafValue::Text(value)) => Some(value.as_str()),
        _ => None,
    }
}

fn leaf_span(tree: &TreeValue) -> Option<(usize, usize)> {
    match tree {
        TreeValue::Leaf(TreeLeafValue::BytesRange { start, end }) => Some((*start, *end)),
        _ => None,
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

pub fn encode_expr(input: tree_expr::TreeExpr, name: impl Into<String>) -> tree_expr::TreeExpr {
    TreeNs::encode(input, name)
}

pub fn decode_expr(input: tree_expr::TreeExpr, name: impl Into<String>) -> tree_expr::TreeExpr {
    TreeNs::decode(input, name)
}

pub fn transform_named_expr(
    input: tree_expr::TreeExpr,
    name: impl Into<String>,
) -> tree_expr::TreeExpr {
    TreeNs::transform_named(input, name)
}

pub use crate::collections::dataset::expressions::tree::{
    TreeExpr, TreeLabel, TreeLeafExpr, TreeOp, TreePos, TreeSpan,
};
pub use crate::collections::dataset::functions::tree as tree_fn;
pub use crate::collections::dataset::functions::tree::format::{format_bracketed, format_pretty};
pub use crate::collections::dataset::functions::tree::inspect::TreeTraversal;
pub use crate::collections::dataset::functions::tree::parse::{parse_bracketed, TreeParseError};
pub use crate::collections::dataset::functions::tree::pretty::{pretty_print, PrettyOptions};
pub use crate::collections::dataset::namespaces::tree::TreeNs as TreeNamespace;

#[cfg(test)]
mod tests {
    use super::{Tag, TokenSpan, TreeValue};

    #[test]
    fn from_tags_builds_preterminal_tree() {
        let tags = vec![
            Tag::new("Mary", "NNP", TokenSpan::new(0, 4)),
            Tag::new("walks", "VBZ", TokenSpan::new(5, 10)),
        ];

        let tree = TreeValue::from_tags("S", &tags);
        assert_eq!(tree.format_bracketed(), "(S (NNP Mary) (VBZ walks))");
    }

    #[test]
    fn preterminal_tags_roundtrip_from_tag_tree() {
        let tags = vec![
            Tag::new("Mary", "NNP", TokenSpan::new(0, 4)),
            Tag::new("walks", "VBZ", TokenSpan::new(5, 10)),
        ];

        let tree = TreeValue::from_tags("S", &tags);
        let recovered = tree.to_preterminal_tags();

        assert_eq!(recovered, tags);
    }
}
