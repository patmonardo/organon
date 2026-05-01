//! Dependency graph core types.

use std::collections::HashSet;

use polars::prelude::{lit, Expr, NamedFrom, PlSmallStr, Series};

use crate::collections::dataframe::record;
use crate::collections::dataframe::{GDSDataFrame, GDSFrameError};
use crate::collections::dataset::token::{Token, TokenSpan};
use crate::collections::dataset::tree::TreeValue;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DependencyNode {
    index: usize,
    text: String,
    span: TokenSpan,
}

impl DependencyNode {
    pub fn new(index: usize, text: impl Into<String>, span: TokenSpan) -> Self {
        Self {
            index,
            text: text.into(),
            span,
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn span(&self) -> TokenSpan {
        self.span
    }

    pub fn to_struct_expr(&self) -> Expr {
        record(vec![
            lit(self.index as u64).alias("index"),
            lit(self.text.clone()).alias("text"),
            lit(self.span.start() as u64).alias("start"),
            lit(self.span.end() as u64).alias("end"),
        ])
    }
}

impl From<(usize, &Token)> for DependencyNode {
    fn from((index, token): (usize, &Token)) -> Self {
        Self::new(index, token.text().to_string(), token.span())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DependencyEdge {
    head: usize,
    dep: usize,
    label: String,
}

impl DependencyEdge {
    pub fn new(head: usize, dep: usize, label: impl Into<String>) -> Self {
        Self {
            head,
            dep,
            label: label.into(),
        }
    }

    pub fn head(&self) -> usize {
        self.head
    }

    pub fn dep(&self) -> usize {
        self.dep
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn to_struct_expr(&self) -> Expr {
        record(vec![
            lit(self.head as u64).alias("head"),
            lit(self.dep as u64).alias("dep"),
            lit(self.label.clone()).alias("label"),
        ])
    }
}

#[derive(Debug, Clone)]
pub struct DependencyGraph {
    nodes: Vec<DependencyNode>,
    edges: Vec<DependencyEdge>,
    root: Option<usize>,
}

impl DependencyGraph {
    pub fn new(
        nodes: Vec<DependencyNode>,
        edges: Vec<DependencyEdge>,
        root: Option<usize>,
    ) -> Self {
        Self { nodes, edges, root }
    }

    pub fn from_tokens(tokens: &[Token], edges: Vec<DependencyEdge>, root: Option<usize>) -> Self {
        let nodes = tokens
            .iter()
            .enumerate()
            .map(DependencyNode::from)
            .collect();
        Self::new(nodes, edges, root)
    }

    pub fn nodes(&self) -> &[DependencyNode] {
        &self.nodes
    }

    pub fn edges(&self) -> &[DependencyEdge] {
        &self.edges
    }

    pub fn root(&self) -> Option<usize> {
        self.root
    }

    pub fn set_root(&mut self, root: Option<usize>) {
        self.root = root;
    }

    pub fn node(&self, index: usize) -> Option<&DependencyNode> {
        self.nodes.get(index)
    }

    pub fn add_edge(&mut self, edge: DependencyEdge) {
        self.edges.push(edge);
    }

    pub fn head_of(&self, dep: usize) -> Option<usize> {
        if self.root == Some(dep) {
            return None;
        }
        self.edges
            .iter()
            .find(|edge| edge.dep == dep)
            .map(|edge| edge.head)
    }

    pub fn relation_of(&self, dep: usize) -> Option<&str> {
        if self.root == Some(dep) {
            return Some("ROOT");
        }
        self.edges
            .iter()
            .find(|edge| edge.dep == dep)
            .map(|edge| edge.label.as_str())
    }

    pub fn edges_from(&self, head: usize) -> Vec<&DependencyEdge> {
        self.edges.iter().filter(|e| e.head == head).collect()
    }

    pub fn edges_to(&self, dep: usize) -> Vec<&DependencyEdge> {
        self.edges.iter().filter(|e| e.dep == dep).collect()
    }

    pub fn children_of(&self, head: usize) -> Vec<usize> {
        let mut children = self
            .edges
            .iter()
            .filter(|edge| edge.head == head)
            .map(|edge| edge.dep)
            .collect::<Vec<_>>();
        children.sort_unstable();
        children
    }

    pub fn left_children(&self, node_index: usize) -> usize {
        self.children_of(node_index)
            .into_iter()
            .filter(|child| *child < node_index)
            .count()
    }

    pub fn right_children(&self, node_index: usize) -> usize {
        self.children_of(node_index)
            .into_iter()
            .filter(|child| *child > node_index)
            .count()
    }

    pub fn triples(&self) -> Vec<((String, usize), String, (String, usize))> {
        let mut out = Vec::new();
        for edge in &self.edges {
            let Some(head) = self.nodes.get(edge.head) else {
                continue;
            };
            let Some(dep) = self.nodes.get(edge.dep) else {
                continue;
            };
            out.push((
                (head.text.clone(), head.index),
                edge.label.clone(),
                (dep.text.clone(), dep.index),
            ));
        }
        out
    }

    pub fn to_dependency_tree(&self) -> Option<TreeValue> {
        let root = self.root?;
        self.node(root)?;
        Some(self.tree_from(root))
    }

    pub fn to_dot(&self) -> String {
        let mut output = String::new();
        output.push_str("digraph G{\n");
        output.push_str("edge [dir=forward]\n");
        output.push_str("node [shape=plaintext]\n\n");

        output.push_str("0 [label=\"0 (TOP)\"]\n");
        if let Some(root) = self.root {
            output.push_str(&format!("0 -> {} [label=\"ROOT\"]\n", root + 1));
        }

        for node in &self.nodes {
            output.push_str(&format!(
                "{} [label=\"{} ({})\"]\n",
                node.index + 1,
                node.index + 1,
                node.text
            ));
        }

        for edge in &self.edges {
            output.push_str(&format!(
                "{} -> {} [label=\"{}\"]\n",
                edge.head + 1,
                edge.dep + 1,
                edge.label
            ));
        }

        output.push('}');
        output
    }

    pub fn to_conll_4(&self) -> String {
        let mut lines = Vec::with_capacity(self.nodes.len());
        for node in &self.nodes {
            let head = self.head_of(node.index).map(|value| value + 1).unwrap_or(0);
            let rel = self.relation_of(node.index).unwrap_or("dep");
            lines.push(format!("{}\t_\t{}\t{}", node.text, head, rel));
        }
        lines.join("\n")
    }

    pub fn contains_cycle(&self) -> bool {
        let mut visited = HashSet::new();
        let mut stack = HashSet::new();

        for node in &self.nodes {
            if self.detect_cycle(node.index, &mut visited, &mut stack) {
                return true;
            }
        }
        false
    }

    pub fn to_node_frame(&self) -> Result<GDSDataFrame, GDSFrameError> {
        let indices: Vec<u64> = self.nodes.iter().map(|n| n.index as u64).collect();
        let texts: Vec<String> = self.nodes.iter().map(|n| n.text.clone()).collect();
        let starts: Vec<u64> = self.nodes.iter().map(|n| n.span.start() as u64).collect();
        let ends: Vec<u64> = self.nodes.iter().map(|n| n.span.end() as u64).collect();
        let df = GDSDataFrame::from_series(vec![
            Series::new(PlSmallStr::from_static("index"), indices),
            Series::new(PlSmallStr::from_static("text"), texts),
            Series::new(PlSmallStr::from_static("start"), starts),
            Series::new(PlSmallStr::from_static("end"), ends),
        ])?;
        Ok(df)
    }

    pub fn to_edge_frame(&self) -> Result<GDSDataFrame, GDSFrameError> {
        let heads: Vec<u64> = self.edges.iter().map(|e| e.head as u64).collect();
        let deps: Vec<u64> = self.edges.iter().map(|e| e.dep as u64).collect();
        let labels: Vec<String> = self.edges.iter().map(|e| e.label.clone()).collect();
        let df = GDSDataFrame::from_series(vec![
            Series::new(PlSmallStr::from_static("head"), heads),
            Series::new(PlSmallStr::from_static("dep"), deps),
            Series::new(PlSmallStr::from_static("label"), labels),
        ])?;
        Ok(df)
    }

    fn tree_from(&self, node_index: usize) -> TreeValue {
        let node = &self.nodes[node_index];
        let children = self
            .children_of(node_index)
            .into_iter()
            .map(|child_index| self.tree_from(child_index))
            .collect::<Vec<_>>();
        if children.is_empty() {
            TreeValue::leaf(node.text.clone())
        } else {
            TreeValue::node(node.text.clone(), children)
        }
    }

    fn detect_cycle(
        &self,
        node_index: usize,
        visited: &mut HashSet<usize>,
        stack: &mut HashSet<usize>,
    ) -> bool {
        if stack.contains(&node_index) {
            return true;
        }
        if visited.contains(&node_index) {
            return false;
        }

        visited.insert(node_index);
        stack.insert(node_index);

        for child in self.children_of(node_index) {
            if self.detect_cycle(child, visited, stack) {
                return true;
            }
        }

        stack.remove(&node_index);
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::dataset::token::{TokenKind, TokenSpan};

    #[test]
    fn dependency_graph_from_tokens() {
        let tokens = vec![
            Token::new("I", TokenSpan::new(0, 1), TokenKind::Word),
            Token::new("run", TokenSpan::new(2, 5), TokenKind::Word),
        ];
        let edges = vec![DependencyEdge::new(1, 0, "nsubj")];
        let graph = DependencyGraph::from_tokens(&tokens, edges, Some(1));
        assert_eq!(graph.nodes().len(), 2);
        assert_eq!(graph.edges().len(), 1);
        assert_eq!(graph.root(), Some(1));
    }

    #[test]
    fn dependency_graph_dot_and_conll() {
        let tokens = vec![
            Token::new("John", TokenSpan::new(0, 4), TokenKind::Word),
            Token::new("loves", TokenSpan::new(5, 10), TokenKind::Word),
            Token::new("Mary", TokenSpan::new(11, 15), TokenKind::Word),
        ];
        let edges = vec![
            DependencyEdge::new(1, 0, "nsubj"),
            DependencyEdge::new(1, 2, "obj"),
        ];
        let graph = DependencyGraph::from_tokens(&tokens, edges, Some(1));

        let dot = graph.to_dot();
        assert!(dot.contains("0 -> 2 [label=\"ROOT\"]"));

        let conll = graph.to_conll_4();
        assert!(conll.contains("John\t_\t2\tnsubj"));
        assert!(conll.contains("loves\t_\t0\tROOT"));
    }

    #[test]
    fn dependency_tree_and_cycle_check() {
        let tokens = vec![
            Token::new("I", TokenSpan::new(0, 1), TokenKind::Word),
            Token::new("run", TokenSpan::new(2, 5), TokenKind::Word),
        ];
        let edges = vec![DependencyEdge::new(1, 0, "nsubj")];
        let graph = DependencyGraph::from_tokens(&tokens, edges, Some(1));

        let tree = graph.to_dependency_tree().expect("tree");
        assert_eq!(tree.format_bracketed(), "(run I)");
        assert!(!graph.contains_cycle());
    }
}
