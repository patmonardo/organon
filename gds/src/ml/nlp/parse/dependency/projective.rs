use std::collections::HashSet;

use crate::collections::dataset::dependency::DependencyGraph;

use super::common::graph_from_arcs;
use super::grammar::DependencyGrammar;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct DependencySpan {
    start_index: usize,
    end_index: usize,
    head_index: usize,
    arcs: Vec<isize>,
}

#[derive(Debug, Clone)]
pub struct ProjectiveDependencyParser {
    grammar: DependencyGrammar,
}

impl ProjectiveDependencyParser {
    pub fn new(grammar: DependencyGrammar) -> Self {
        Self { grammar }
    }

    pub fn parse(&self, tokens: &[&str]) -> Vec<DependencyGraph> {
        let n = tokens.len();
        if n == 0 {
            return Vec::new();
        }

        let mut chart = vec![vec![HashSet::<DependencySpan>::new(); n + 1]; n + 1];
        for (i, row) in chart.iter_mut().enumerate().take(n + 1) {
            for (j, cell) in row.iter_mut().enumerate().take(n + 1) {
                if i == j + 1 {
                    cell.insert(DependencySpan {
                        start_index: i - 1,
                        end_index: i,
                        head_index: i - 1,
                        arcs: vec![-1],
                    });
                }
            }
        }

        for i in 1..=n {
            for j in (0..=(i.saturating_sub(2))).rev() {
                for k in (j + 1..=i.saturating_sub(1)).rev() {
                    let left_entries = chart[k][j].iter().cloned().collect::<Vec<_>>();
                    let right_entries = chart[i][k].iter().cloned().collect::<Vec<_>>();
                    for span1 in &left_entries {
                        for span2 in &right_entries {
                            for new_span in self.concatenate(tokens, span1, span2) {
                                chart[i][j].insert(new_span);
                            }
                        }
                    }
                }
            }
        }

        let mut parses = Vec::new();
        for span in &chart[n][0] {
            if let Some(graph) = graph_from_arcs(tokens, &span.arcs) {
                parses.push(graph);
            }
        }
        parses
    }

    fn concatenate(
        &self,
        tokens: &[&str],
        span1: &DependencySpan,
        span2: &DependencySpan,
    ) -> Vec<DependencySpan> {
        let mut spans = Vec::new();
        if span1.start_index == span2.start_index {
            return spans;
        }

        let (left, right) = if span1.start_index > span2.start_index {
            (span2, span1)
        } else {
            (span1, span2)
        };

        let mut new_arcs = left.arcs.clone();
        new_arcs.extend_from_slice(&right.arcs);

        if self
            .grammar
            .contains(tokens[left.head_index], tokens[right.head_index])
        {
            let mut arcs = new_arcs.clone();
            arcs[right.head_index - left.start_index] = left.head_index as isize;
            spans.push(DependencySpan {
                start_index: left.start_index,
                end_index: right.end_index,
                head_index: left.head_index,
                arcs,
            });
        }

        if self
            .grammar
            .contains(tokens[right.head_index], tokens[left.head_index])
        {
            let mut arcs = new_arcs;
            arcs[left.head_index - left.start_index] = right.head_index as isize;
            spans.push(DependencySpan {
                start_index: left.start_index,
                end_index: right.end_index,
                head_index: right.head_index,
                arcs,
            });
        }

        spans
    }
}
