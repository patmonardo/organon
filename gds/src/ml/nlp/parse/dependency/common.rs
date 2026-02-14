use crate::collections::dataset::dependency::{DependencyEdge, DependencyGraph};
use crate::collections::dataset::token::{Token, TokenKind, TokenSpan};

pub(super) fn strip_quotes(value: &str) -> &str {
    value.trim_matches('"').trim_matches('\'')
}

pub(super) fn enumerate_assignments(
    options: &[Vec<isize>],
    index: usize,
    current: &mut [isize],
    out: &mut Vec<Vec<isize>>,
) {
    if index == options.len() {
        out.push(current.to_vec());
        return;
    }

    for head in &options[index] {
        current[index] = *head;
        enumerate_assignments(options, index + 1, current, out);
    }
}

pub(super) fn roots_count(arcs: &[isize]) -> usize {
    arcs.iter().filter(|head| **head < 0).count()
}

pub(super) fn has_cycle(arcs: &[isize]) -> bool {
    let n = arcs.len();
    let mut state = vec![0u8; n];

    for node in 0..n {
        if state[node] != 0 {
            continue;
        }
        let mut current = node;
        loop {
            if state[current] == 1 {
                return true;
            }
            if state[current] == 2 {
                break;
            }
            state[current] = 1;
            let head = arcs[current];
            if head < 0 {
                break;
            }
            current = head as usize;
            if current >= n {
                return true;
            }
        }

        let mut current = node;
        while state[current] == 1 {
            state[current] = 2;
            let head = arcs[current];
            if head < 0 {
                break;
            }
            current = head as usize;
            if current >= n {
                break;
            }
        }
    }

    false
}

pub(super) fn graph_from_arcs(tokens: &[&str], arcs: &[isize]) -> Option<DependencyGraph> {
    if tokens.len() != arcs.len() {
        return None;
    }

    let mut cursor = 0usize;
    let token_values = tokens
        .iter()
        .map(|token| {
            let start = cursor;
            let end = start + token.len();
            cursor = end + 1;
            Token::new(
                (*token).to_string(),
                TokenSpan::new(start, end),
                TokenKind::Word,
            )
        })
        .collect::<Vec<_>>();

    let mut root = None;
    let mut edges = Vec::new();
    for (dep_index, head) in arcs.iter().enumerate() {
        if *head < 0 {
            if root.is_some() {
                return None;
            }
            root = Some(dep_index);
            continue;
        }

        let head_index = *head as usize;
        if head_index >= arcs.len() {
            return None;
        }
        edges.push(DependencyEdge::new(head_index, dep_index, "dep"));
    }

    Some(DependencyGraph::from_tokens(&token_values, edges, root))
}
