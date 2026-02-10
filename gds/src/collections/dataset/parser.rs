//! Parser trait and default implementations.

use regex::Regex;
use serde_json::Value as JsonValue;

use crate::collections::dataset::featstruct::{FeatDict, FeatStruct, FeatValue};
use crate::collections::dataset::functions::tree::parse::parse_bracketed;
use crate::collections::dataset::parse::{Parse, ParseForest, ParseKind};
use crate::collections::dataset::token::{Token, TokenSpan};
use crate::collections::dataset::tree::{TreeNode, TreeValue};

/// Pluggable parser trait.
pub trait Parser {
    /// Parse tokens into a forest of parses.
    fn parse_tokens(&self, tokens: &[Token]) -> ParseForest;
}

/// Dependency parser placeholder: to be implemented in GraphFrame/ML/NLP.
#[derive(Debug, Clone, Copy, Default)]
pub struct DependencyParser;

/// Flat parser: wraps tokens under a single ROOT node.
#[derive(Debug, Clone, Copy, Default)]
pub struct FlatParser;

impl Parser for FlatParser {
    fn parse_tokens(&self, tokens: &[Token]) -> ParseForest {
        if tokens.is_empty() {
            return Vec::new();
        }

        let children: Vec<TreeValue> = tokens
            .iter()
            .map(|t| TreeValue::leaf(t.text().to_string()))
            .collect();
        let tree = TreeValue::node("ROOT", children);
        let mut parse = Parse::new(tree, ParseKind::Constituency);

        if let (Some(first), Some(last)) = (tokens.first(), tokens.last()) {
            let span = TokenSpan::new(first.span().start(), last.span().end());
            parse = parse.with_span(span);
        }

        vec![parse]
    }
}

/// Bracketed tree parser (constituency).
#[derive(Debug, Clone, Copy, Default)]
pub struct BracketedParser;

impl Parser for BracketedParser {
    fn parse_tokens(&self, tokens: &[Token]) -> ParseForest {
        if tokens.is_empty() {
            return Vec::new();
        }
        let input = tokens
            .iter()
            .map(|t| t.text())
            .collect::<Vec<_>>()
            .join(" ");
        let Ok(tree) = parse_bracketed(&input) else {
            return Vec::new();
        };
        let mut parse = Parse::new(tree, ParseKind::Constituency);
        if let (Some(first), Some(last)) = (tokens.first(), tokens.last()) {
            let span = TokenSpan::new(first.span().start(), last.span().end());
            parse = parse.with_span(span);
        }
        vec![parse]
    }
}

/// Simple XML/HTML-like markup parser that builds a tree from tag tokens.
#[derive(Debug, Clone)]
pub struct MarkupParser {
    root_label: String,
    strict: bool,
}

/// JSON parser that builds a tree from JSON values.
#[derive(Debug, Clone)]
pub struct JsonParser {
    root_label: String,
}

impl JsonParser {
    pub fn new(root_label: impl Into<String>) -> Self {
        Self {
            root_label: root_label.into(),
        }
    }
}

impl Default for JsonParser {
    fn default() -> Self {
        Self::new("JSON")
    }
}

impl Parser for JsonParser {
    fn parse_tokens(&self, tokens: &[Token]) -> ParseForest {
        if tokens.is_empty() {
            return Vec::new();
        }

        let mut input = String::new();
        for token in tokens {
            input.push_str(token.text());
        }

        let Ok(value) = serde_json::from_str::<JsonValue>(&input) else {
            return Vec::new();
        };

        let tree = TreeValue::node(self.root_label.clone(), vec![json_to_tree(&value)]);
        let mut parse = Parse::new(tree, ParseKind::Unknown);
        if let (Some(first), Some(last)) = (tokens.first(), tokens.last()) {
            let span = TokenSpan::new(first.span().start(), last.span().end());
            parse = parse.with_span(span);
        }
        vec![parse]
    }
}

impl MarkupParser {
    pub fn new(root_label: impl Into<String>) -> Self {
        Self {
            root_label: root_label.into(),
            strict: true,
        }
    }

    pub fn with_strict(mut self, strict: bool) -> Self {
        self.strict = strict;
        self
    }
}

impl Default for MarkupParser {
    fn default() -> Self {
        Self::new("DOC")
    }
}

impl Parser for MarkupParser {
    fn parse_tokens(&self, tokens: &[Token]) -> ParseForest {
        if tokens.is_empty() {
            return Vec::new();
        }

        let mut stack: Vec<(String, Vec<TreeValue>, Option<FeatStruct>)> =
            vec![(self.root_label.clone(), Vec::new(), None)];

        for token in tokens {
            let text = token.text();
            if let Some((tag, attrs)) = parse_open_tag(text) {
                stack.push((tag, Vec::new(), attrs));
                continue;
            }
            if let Some(tag) = parse_close_tag(text) {
                if stack.len() <= 1 {
                    if self.strict {
                        return Vec::new();
                    }
                    continue;
                }
                let (label, children, attrs) = stack.pop().unwrap();
                if self.strict && label != tag {
                    return Vec::new();
                }
                let mut node = TreeNode::new(label, children);
                if let Some(attrs) = attrs {
                    node = node.with_attributes(attrs);
                }
                if let Some(parent) = stack.last_mut() {
                    parent.1.push(TreeValue::Node(node));
                }
                continue;
            }
            if let Some((tag, attrs)) = parse_self_closing_tag(text) {
                let mut node = TreeNode::new(tag, Vec::new());
                if let Some(attrs) = attrs {
                    node = node.with_attributes(attrs);
                }
                if let Some(parent) = stack.last_mut() {
                    parent.1.push(TreeValue::Node(node));
                }
                continue;
            }

            if let Some(parent) = stack.last_mut() {
                parent.1.push(TreeValue::leaf(text.to_string()));
            }
        }

        if self.strict && stack.len() > 1 {
            return Vec::new();
        }

        while stack.len() > 1 {
            let (label, children, attrs) = stack.pop().unwrap();
            let mut node = TreeNode::new(label, children);
            if let Some(attrs) = attrs {
                node = node.with_attributes(attrs);
            }
            let node = TreeValue::Node(node);
            if let Some(parent) = stack.last_mut() {
                parent.1.push(node);
            }
        }

        let root_children = stack.pop().unwrap().1;
        let tree = TreeValue::node(self.root_label.clone(), root_children);
        let mut parse = Parse::new(tree, ParseKind::Constituency);
        if let (Some(first), Some(last)) = (tokens.first(), tokens.last()) {
            let span = TokenSpan::new(first.span().start(), last.span().end());
            parse = parse.with_span(span);
        }
        vec![parse]
    }
}

impl Parser for DependencyParser {
    fn parse_tokens(&self, _tokens: &[Token]) -> ParseForest {
        Vec::new()
    }
}

fn parse_open_tag(text: &str) -> Option<(String, Option<FeatStruct>)> {
    if text.starts_with('<')
        && text.ends_with('>')
        && !text.starts_with("</")
        && !text.ends_with("/>")
    {
        let inner = &text[1..text.len() - 1];
        let (name, attrs) = tag_name(inner)?;
        return Some((name, attrs));
    }
    None
}

fn parse_close_tag(text: &str) -> Option<String> {
    if text.starts_with("</") && text.ends_with('>') {
        let inner = &text[2..text.len() - 1];
        let (name, _) = tag_name(inner)?;
        return Some(name);
    }
    None
}

fn parse_self_closing_tag(text: &str) -> Option<(String, Option<FeatStruct>)> {
    if text.starts_with('<') && text.ends_with("/>") {
        let inner = &text[1..text.len() - 2];
        let (name, attrs) = tag_name(inner)?;
        return Some((name, attrs));
    }
    None
}

fn tag_name(inner: &str) -> Option<(String, Option<FeatStruct>)> {
    let name = inner.split_whitespace().next().unwrap_or("");
    if name.is_empty() {
        return None;
    }
    let attrs = parse_attributes(inner, name);
    Some((name.to_string(), attrs))
}

fn parse_attributes(inner: &str, name: &str) -> Option<FeatStruct> {
    let re = Regex::new(
        r#"(?x)
        (?P<key>[A-Za-z_:][A-Za-z0-9:._-]*)
        (?:\s*=\s*(?P<val>\"[^\"]*\"|'[^']*'|[^\s\"']+))?
    "#,
    )
    .ok()?;

    let mut attrs: FeatDict = FeatDict::new();
    for (idx, caps) in re.captures_iter(inner).enumerate() {
        let key = caps.name("key")?.as_str();
        if idx == 0 && key == name {
            continue;
        }
        let value = caps.name("val").map(|m| m.as_str());
        let feat_value = match value {
            Some(raw) => {
                let trimmed = raw.trim_matches('"').trim_matches('\'');
                FeatValue::Text(trimmed.to_string())
            }
            None => FeatValue::Bool(true),
        };
        attrs.insert(key.to_string(), feat_value);
    }

    if attrs.is_empty() {
        None
    } else {
        Some(FeatStruct::Dict(attrs))
    }
}

fn json_to_tree(value: &JsonValue) -> TreeValue {
    match value {
        JsonValue::Null => TreeValue::leaf("null"),
        JsonValue::Bool(val) => TreeValue::leaf(val.to_string()),
        JsonValue::Number(num) => TreeValue::leaf(num.to_string()),
        JsonValue::String(text) => TreeValue::leaf(text.clone()),
        JsonValue::Array(values) => {
            let children = values.iter().map(json_to_tree).collect();
            TreeValue::node("array", children)
        }
        JsonValue::Object(map) => {
            let mut children = Vec::with_capacity(map.len());
            for (key, value) in map {
                let child = json_to_tree(value);
                let node = TreeValue::node(key.clone(), vec![child]);
                children.push(node);
            }
            TreeValue::node("object", children)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::dataset::token::{TokenKind, TokenSpan};

    #[test]
    fn flat_parser_returns_single_root_parse() {
        let tokens = vec![
            Token::new("a", TokenSpan::new(0, 1), TokenKind::Word),
            Token::new("b", TokenSpan::new(2, 3), TokenKind::Word),
        ];
        let forest = FlatParser::default().parse_tokens(&tokens);
        assert_eq!(forest.len(), 1);
        assert_eq!(forest[0].kind(), &ParseKind::Constituency);
    }

    #[test]
    fn markup_parser_builds_tree() {
        let tokens = vec![
            Token::new("<p>", TokenSpan::new(0, 3), TokenKind::Word),
            Token::new("Hello", TokenSpan::new(3, 8), TokenKind::Word),
            Token::new("</p>", TokenSpan::new(8, 12), TokenKind::Word),
        ];
        let forest = MarkupParser::default().parse_tokens(&tokens);
        assert_eq!(forest.len(), 1);
        assert_eq!(forest[0].kind(), &ParseKind::Constituency);
    }

    #[test]
    fn bracketed_parser_parses_tree() {
        let tokens = vec![
            Token::new("(S", TokenSpan::new(0, 2), TokenKind::Word),
            Token::new("(NP", TokenSpan::new(3, 6), TokenKind::Word),
            Token::new("Mary)", TokenSpan::new(7, 12), TokenKind::Word),
            Token::new("(VP", TokenSpan::new(13, 16), TokenKind::Word),
            Token::new("walks))", TokenSpan::new(17, 24), TokenKind::Word),
        ];
        let forest = BracketedParser::default().parse_tokens(&tokens);
        assert_eq!(forest.len(), 1);
    }
}
