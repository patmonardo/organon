use std::collections::HashMap;

use crate::collections::dataset::tree::TreeValue;

use super::cfg::{Cfg, GrammarSymbol, ParseError, Production};

#[derive(Debug, Clone)]
enum ParseNode {
    Nonterminal {
        label: String,
        children: Vec<ParseNode>,
    },
    Terminal(String),
}

impl ParseNode {
    fn nonterminal(label: impl Into<String>, children: Vec<ParseNode>) -> Self {
        Self::Nonterminal {
            label: label.into(),
            children,
        }
    }

    fn terminal(token: impl Into<String>) -> Self {
        Self::Terminal(token.into())
    }

    fn as_nonterminal_label(&self) -> Option<&str> {
        match self {
            ParseNode::Nonterminal { label, .. } => Some(label),
            ParseNode::Terminal(_) => None,
        }
    }

    fn is_unexpanded_nonterminal(&self) -> bool {
        matches!(self, ParseNode::Nonterminal { children, .. } if children.is_empty())
    }

    fn as_terminal(&self) -> Option<&str> {
        match self {
            ParseNode::Terminal(token) => Some(token),
            ParseNode::Nonterminal { .. } => None,
        }
    }

    fn to_tree_value(&self) -> TreeValue {
        match self {
            ParseNode::Terminal(token) => TreeValue::leaf(token.clone()),
            ParseNode::Nonterminal { label, children } => {
                let child_values = children.iter().map(ParseNode::to_tree_value).collect();
                TreeValue::node(label.clone(), child_values)
            }
        }
    }

    fn freeze_key(&self) -> String {
        match self {
            ParseNode::Terminal(token) => token.clone(),
            ParseNode::Nonterminal { label, children } => {
                if children.is_empty() {
                    format!("({label})")
                } else {
                    let children = children
                        .iter()
                        .map(ParseNode::freeze_key)
                        .collect::<Vec<_>>()
                        .join(" ");
                    format!("({label} {children})")
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct RecursiveDescentParser {
    grammar: Cfg,
    trace: u8,
}

impl RecursiveDescentParser {
    pub fn new(grammar: Cfg) -> Self {
        Self { grammar, trace: 0 }
    }

    pub fn grammar(&self) -> &Cfg {
        &self.grammar
    }

    pub fn trace(&mut self, trace: u8) {
        self.trace = trace;
    }

    pub fn parse_tokens(
        &self,
        tokens: &[crate::collections::dataset::token::Token],
    ) -> Result<Vec<TreeValue>, ParseError> {
        self.parse_token_texts(tokens.iter().map(|token| token.text().to_string()))
    }

    pub fn parse_token_texts<I, S>(&self, tokens: I) -> Result<Vec<TreeValue>, ParseError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let tokens = tokens.into_iter().map(Into::into).collect::<Vec<_>>();
        self.grammar
            .check_coverage_texts(tokens.iter().map(String::as_str))?;

        let initial_tree = ParseNode::nonterminal(self.grammar.start(), Vec::new());
        let frontier = vec![Vec::new()];
        let mut out = Vec::new();
        self.parse_state(tokens, initial_tree, frontier, &mut out);
        Ok(out)
    }

    fn parse_state(
        &self,
        remaining_text: Vec<String>,
        tree: ParseNode,
        frontier: Vec<Vec<usize>>,
        out: &mut Vec<TreeValue>,
    ) {
        if remaining_text.is_empty() && frontier.is_empty() {
            out.push(tree.to_tree_value());
            return;
        }

        if frontier.is_empty() {
            return;
        }

        let position = &frontier[0];
        let Some(node) = get_node(&tree, position) else {
            return;
        };

        if node.is_unexpanded_nonterminal() {
            self.expand_state(remaining_text, tree, frontier, None, out);
            return;
        }

        if let Some(terminal) = node.as_terminal() {
            if let Some(next_token) = remaining_text.first() {
                if terminal == next_token {
                    let mut next_remaining = remaining_text.clone();
                    next_remaining.remove(0);
                    self.parse_state(next_remaining, tree, frontier[1..].to_vec(), out);
                }
            }
        }
    }

    fn expand_state(
        &self,
        remaining_text: Vec<String>,
        tree: ParseNode,
        frontier: Vec<Vec<usize>>,
        production: Option<&Production>,
        out: &mut Vec<TreeValue>,
    ) {
        if frontier.is_empty() {
            return;
        }
        let current = &frontier[0];
        let Some(label) = get_node(&tree, current).and_then(ParseNode::as_nonterminal_label) else {
            return;
        };

        let productions = match production {
            Some(production) => vec![production.clone()],
            None => self.grammar.productions().to_vec(),
        };

        for production in productions {
            if production.lhs() != label {
                continue;
            }

            let subtree = self.production_to_tree(&production);
            let mut new_tree = tree.clone();
            replace_node(&mut new_tree, current, subtree);

            let mut new_frontier = (0..production.rhs().len())
                .map(|index| {
                    let mut child_pos = current.clone();
                    child_pos.push(index);
                    child_pos
                })
                .collect::<Vec<_>>();
            new_frontier.extend(frontier[1..].iter().cloned());

            self.parse_state(remaining_text.clone(), new_tree, new_frontier, out);
        }
    }

    fn production_to_tree(&self, production: &Production) -> ParseNode {
        let mut children = Vec::with_capacity(production.rhs().len());
        for rhs in production.rhs() {
            match rhs {
                GrammarSymbol::Nonterminal(label) => {
                    children.push(ParseNode::nonterminal(label.clone(), Vec::new()));
                }
                GrammarSymbol::Terminal(token) => {
                    children.push(ParseNode::terminal(token.clone()));
                }
            }
        }
        ParseNode::nonterminal(production.lhs().to_string(), children)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecursiveStepOutcome {
    Matched(String),
    Expanded(Production),
    Backtracked,
    NoOp,
}

#[derive(Debug, Clone)]
pub struct SteppingRecursiveDescentParser {
    parser: RecursiveDescentParser,
    rtext: Vec<String>,
    tree: ParseNode,
    frontier: Vec<Vec<usize>>,
    tried_e: HashMap<String, Vec<Production>>,
    tried_m: HashMap<String, Vec<String>>,
    history: Vec<(Vec<String>, ParseNode, Vec<Vec<usize>>)>,
    parses: Vec<TreeValue>,
}

impl SteppingRecursiveDescentParser {
    pub fn new(grammar: Cfg) -> Self {
        let start = grammar.start().to_string();
        Self {
            parser: RecursiveDescentParser::new(grammar),
            rtext: Vec::new(),
            tree: ParseNode::nonterminal(start, Vec::new()),
            frontier: vec![Vec::new()],
            tried_e: HashMap::new(),
            tried_m: HashMap::new(),
            history: Vec::new(),
            parses: Vec::new(),
        }
    }

    pub fn initialize_texts<'a>(
        &mut self,
        tokens: impl IntoIterator<Item = &'a str>,
    ) -> Result<(), ParseError> {
        let texts = tokens
            .into_iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>();
        self.parser
            .grammar()
            .check_coverage_texts(texts.iter().map(String::as_str))?;

        self.rtext = texts;
        self.tree = ParseNode::nonterminal(self.parser.grammar().start(), Vec::new());
        self.frontier = vec![Vec::new()];
        self.tried_e.clear();
        self.tried_m.clear();
        self.history.clear();
        self.parses.clear();
        Ok(())
    }

    pub fn parse_token_texts<I, S>(&mut self, tokens: I) -> Result<Vec<TreeValue>, ParseError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let texts = tokens.into_iter().map(Into::into).collect::<Vec<_>>();
        self.initialize_texts(texts.iter().map(String::as_str))?;
        while self.step() != RecursiveStepOutcome::NoOp {}
        Ok(self.parses.clone())
    }

    pub fn step(&mut self) -> RecursiveStepOutcome {
        if self.untried_match() {
            if let Some(token) = self.match_token() {
                return RecursiveStepOutcome::Matched(token);
            }
        }

        if let Some(production) = self.expand(None) {
            return RecursiveStepOutcome::Expanded(production);
        }

        if self.backtrack() {
            return RecursiveStepOutcome::Backtracked;
        }

        RecursiveStepOutcome::NoOp
    }

    pub fn expand(&mut self, production: Option<&Production>) -> Option<Production> {
        if self.frontier.is_empty() {
            return None;
        }
        let Some(node) = get_node(&self.tree, &self.frontier[0]) else {
            return None;
        };
        if !node.is_unexpanded_nonterminal() {
            return None;
        }

        let productions = match production {
            Some(production) => vec![production.clone()],
            None => self.untried_expandable_productions(),
        };

        for prod in productions {
            let key = self.tree.freeze_key();
            self.tried_e.entry(key).or_default().push(prod.clone());
            if self.apply_expand(&prod) {
                return Some(prod);
            }
        }

        None
    }

    pub fn match_token(&mut self) -> Option<String> {
        if self.frontier.is_empty() || self.rtext.is_empty() {
            return None;
        }
        let token = self.rtext[0].clone();
        let key = self.tree.freeze_key();
        self.tried_m.entry(key).or_default().push(token.clone());

        let Some(frontier_node) = get_node(&self.tree, &self.frontier[0]) else {
            return None;
        };
        if frontier_node.as_terminal() != Some(token.as_str()) {
            return None;
        }

        self.history
            .push((self.rtext.clone(), self.tree.clone(), self.frontier.clone()));
        self.rtext.remove(0);
        self.frontier.remove(0);
        self.capture_parse_if_complete();
        Some(token)
    }

    pub fn backtrack(&mut self) -> bool {
        let Some((rtext, tree, frontier)) = self.history.pop() else {
            return false;
        };
        self.rtext = rtext;
        self.tree = tree;
        self.frontier = frontier;
        true
    }

    pub fn currently_complete(&self) -> bool {
        self.frontier.is_empty() && self.rtext.is_empty()
    }

    pub fn parses(&self) -> impl Iterator<Item = &TreeValue> {
        self.parses.iter()
    }

    pub fn expandable_productions(&self) -> Vec<Production> {
        if self.frontier.is_empty() {
            return Vec::new();
        }
        let Some(frontier_child) = get_node(&self.tree, &self.frontier[0]) else {
            return Vec::new();
        };
        let Some(label) = frontier_child.as_nonterminal_label() else {
            return Vec::new();
        };
        self.parser
            .grammar()
            .productions()
            .iter()
            .filter(|production| production.lhs() == label)
            .cloned()
            .collect()
    }

    pub fn untried_expandable_productions(&self) -> Vec<Production> {
        let tried = self
            .tried_e
            .get(&self.tree.freeze_key())
            .cloned()
            .unwrap_or_default();
        self.expandable_productions()
            .into_iter()
            .filter(|production| !tried.contains(production))
            .collect()
    }

    pub fn untried_match(&self) -> bool {
        if self.frontier.is_empty() || self.rtext.is_empty() {
            return false;
        }
        let tried = self
            .tried_m
            .get(&self.tree.freeze_key())
            .cloned()
            .unwrap_or_default();
        !tried.contains(&self.rtext[0])
    }

    pub fn remaining_len(&self) -> usize {
        self.rtext.len()
    }

    fn apply_expand(&mut self, production: &Production) -> bool {
        let Some(position) = self.frontier.first().cloned() else {
            return false;
        };
        let Some(label) = get_node(&self.tree, &position).and_then(ParseNode::as_nonterminal_label)
        else {
            return false;
        };
        if label != production.lhs() {
            return false;
        }

        self.history
            .push((self.rtext.clone(), self.tree.clone(), self.frontier.clone()));
        let subtree = self.parser.production_to_tree(production);
        replace_node(&mut self.tree, &position, subtree);

        let mut new_frontier = (0..production.rhs().len())
            .map(|index| {
                let mut path = position.clone();
                path.push(index);
                path
            })
            .collect::<Vec<_>>();
        new_frontier.extend(self.frontier[1..].iter().cloned());
        self.frontier = new_frontier;
        self.capture_parse_if_complete();
        true
    }

    fn capture_parse_if_complete(&mut self) {
        if self.currently_complete() {
            self.parses.push(self.tree.to_tree_value());
        }
    }
}

fn get_node<'a>(tree: &'a ParseNode, position: &[usize]) -> Option<&'a ParseNode> {
    let mut current = tree;
    for index in position {
        match current {
            ParseNode::Nonterminal { children, .. } => {
                current = children.get(*index)?;
            }
            ParseNode::Terminal(_) => return None,
        }
    }
    Some(current)
}

fn replace_node(tree: &mut ParseNode, position: &[usize], replacement: ParseNode) {
    if position.is_empty() {
        *tree = replacement;
        return;
    }

    let mut current = tree;
    for index in &position[..position.len() - 1] {
        current = match current {
            ParseNode::Nonterminal { children, .. } => &mut children[*index],
            ParseNode::Terminal(_) => return,
        };
    }

    if let ParseNode::Nonterminal { children, .. } = current {
        let last = position[position.len() - 1];
        if last < children.len() {
            children[last] = replacement;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grammar() -> Cfg {
        Cfg::new(
            "S",
            vec![
                Production::new("NP", vec![GrammarSymbol::terminal("I")]),
                Production::new("NP", vec![GrammarSymbol::terminal("her")]),
                Production::new("V", vec![GrammarSymbol::terminal("saw")]),
                Production::new(
                    "VP",
                    vec![
                        GrammarSymbol::nonterminal("V"),
                        GrammarSymbol::nonterminal("NP"),
                    ],
                ),
                Production::new(
                    "S",
                    vec![
                        GrammarSymbol::nonterminal("NP"),
                        GrammarSymbol::nonterminal("VP"),
                    ],
                ),
            ],
        )
        .expect("valid grammar")
    }

    #[test]
    fn recursive_descent_parses_sentence() {
        let parser = RecursiveDescentParser::new(grammar());
        let parses = parser
            .parse_token_texts(["I", "saw", "her"])
            .expect("parse should succeed");
        assert_eq!(parses.len(), 1);
        assert_eq!(
            parses[0].format_bracketed(),
            "(S (NP I) (VP (V saw) (NP her)))"
        );
    }

    #[test]
    fn stepping_recursive_descent_finds_parse() {
        let mut parser = SteppingRecursiveDescentParser::new(grammar());
        let parses = parser
            .parse_token_texts(["I", "saw", "her"])
            .expect("parse should succeed");
        assert!(!parses.is_empty());
    }
}
