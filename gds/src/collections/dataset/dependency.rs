//! Dependency graph core types.

use polars::prelude::{NamedFrom, PlSmallStr, Series, lit, Expr};

use crate::collections::dataframe::{GDSDataFrame, GDSFrameError};
use crate::collections::dataframe::record;
use crate::collections::dataset::token::{Token, TokenSpan};

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

    pub fn from_tokens(
        tokens: &[Token],
        edges: Vec<DependencyEdge>,
        root: Option<usize>,
    ) -> Self {
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

    pub fn add_edge(&mut self, edge: DependencyEdge) {
        self.edges.push(edge);
    }

    pub fn edges_from(&self, head: usize) -> Vec<&DependencyEdge> {
        self.edges.iter().filter(|e| e.head == head).collect()
    }

    pub fn edges_to(&self, dep: usize) -> Vec<&DependencyEdge> {
        self.edges.iter().filter(|e| e.dep == dep).collect()
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
}
