use crate::collections::dataset::dependency::DependencyGraph;

use super::common::{enumerate_assignments, graph_from_arcs, has_cycle, roots_count};
use super::grammar::DependencyGrammar;

#[derive(Debug, Clone)]
pub struct NonprojectiveDependencyParser {
    grammar: DependencyGrammar,
}

impl NonprojectiveDependencyParser {
    pub fn new(grammar: DependencyGrammar) -> Self {
        Self { grammar }
    }

    pub fn parse(&self, tokens: &[&str]) -> Vec<DependencyGraph> {
        if tokens.is_empty() {
            return Vec::new();
        }

        let head_options = (0..tokens.len())
            .map(|dep_idx| {
                let mut heads = (0..tokens.len())
                    .filter(|head_idx| {
                        *head_idx != dep_idx
                            && self.grammar.contains(tokens[*head_idx], tokens[dep_idx])
                    })
                    .map(|head_idx| head_idx as isize)
                    .collect::<Vec<_>>();

                if heads.is_empty() {
                    heads.push(-1);
                }
                heads
            })
            .collect::<Vec<_>>();

        let mut assignments = Vec::new();
        let mut current = vec![-1; tokens.len()];
        enumerate_assignments(&head_options, 0, &mut current, &mut assignments);

        let mut parses = Vec::new();
        for arcs in assignments {
            if roots_count(&arcs) != 1 {
                continue;
            }
            if has_cycle(&arcs) {
                continue;
            }
            if let Some(graph) = graph_from_arcs(tokens, &arcs) {
                parses.push(graph);
            }
        }
        parses
    }
}
