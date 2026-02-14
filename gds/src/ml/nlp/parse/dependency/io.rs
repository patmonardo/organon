use crate::collections::dataset::dependency::{DependencyEdge, DependencyGraph};
use crate::collections::dataset::token::{Token, TokenKind, TokenSpan};
use crate::ml::nlp::parse::cfg::ParseError;

pub fn parse_malt_tab(input: &str) -> Result<DependencyGraph, ParseError> {
    let mut words = Vec::new();
    let mut spans = Vec::new();
    let mut heads = Vec::new();
    let mut rels = Vec::new();

    let mut cursor = 0usize;
    for line in input.lines().map(str::trim).filter(|line| !line.is_empty()) {
        let columns = line.split_whitespace().collect::<Vec<_>>();
        if columns.len() < 3 {
            return Err(ParseError::InvalidDependencyRow(line.to_string()));
        }

        let word = columns[0].to_string();
        let head = columns[2]
            .parse::<isize>()
            .map_err(|_| ParseError::InvalidDependencyRow(line.to_string()))?;
        let rel = columns.get(3).copied().unwrap_or("dep").to_string();

        let start = cursor;
        let end = start + word.len();
        cursor = end + 1;

        words.push(word);
        spans.push(TokenSpan::new(start, end));
        heads.push(head);
        rels.push(rel);
    }

    let tokens = words
        .into_iter()
        .zip(spans)
        .map(|(text, span)| Token::new(text, span, TokenKind::Word))
        .collect::<Vec<_>>();

    let mut root = None;
    let mut edges = Vec::new();
    for (index, (head, rel)) in heads.into_iter().zip(rels.into_iter()).enumerate() {
        if head <= 0 {
            root = Some(index);
            continue;
        }
        let head_index = (head as usize).saturating_sub(1);
        if head_index >= tokens.len() {
            return Err(ParseError::InvalidHeadIndex { index, head });
        }
        edges.push(DependencyEdge::new(head_index, index, rel));
    }

    Ok(DependencyGraph::from_tokens(&tokens, edges, root))
}

pub fn to_dot(graph: &DependencyGraph) -> String {
    graph.to_dot()
}
