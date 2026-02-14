use std::collections::{HashMap, HashSet};

use super::common::strip_quotes;

#[derive(Debug, Clone, Default)]
pub struct DependencyGrammar {
    rules: HashMap<String, HashSet<String>>,
}

impl DependencyGrammar {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    pub fn add_rule(&mut self, head: impl Into<String>, dependent: impl Into<String>) {
        self.rules
            .entry(head.into())
            .or_default()
            .insert(dependent.into());
    }

    pub fn contains(&self, head: &str, dependent: &str) -> bool {
        self.rules
            .get(head)
            .map(|deps| deps.contains(dependent))
            .unwrap_or(false)
    }

    pub fn from_string(input: &str) -> Self {
        let mut grammar = Self::new();
        for raw_line in input.lines() {
            let line = raw_line.trim();
            if line.is_empty() {
                continue;
            }
            let Some((lhs, rhs)) = line.split_once("->") else {
                continue;
            };
            let head = strip_quotes(lhs.trim());
            if head.is_empty() {
                continue;
            }
            for dependent in rhs.split('|') {
                let dep = strip_quotes(dependent.trim());
                if !dep.is_empty() {
                    grammar.add_rule(head.to_string(), dep.to_string());
                }
            }
        }
        grammar
    }
}
