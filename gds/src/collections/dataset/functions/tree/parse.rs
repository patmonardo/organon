//! Bracketed tree parsing helpers.

use crate::collections::dataset::tree::{TreeLeafValue, TreeValue};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TreeParseError {
    UnexpectedEof,
    UnexpectedToken(String),
    MissingLabel,
    UnbalancedParens,
}

impl std::fmt::Display for TreeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TreeParseError::UnexpectedEof => write!(f, "unexpected end of input"),
            TreeParseError::UnexpectedToken(token) => write!(f, "unexpected token: {token}"),
            TreeParseError::MissingLabel => write!(f, "missing node label"),
            TreeParseError::UnbalancedParens => write!(f, "unbalanced parentheses"),
        }
    }
}

impl std::error::Error for TreeParseError {}

pub fn parse_bracketed(input: &str) -> Result<TreeValue, TreeParseError> {
    let tokens = tokenize(input)?;
    let mut parser = TokenParser::new(tokens);
    let tree = parse_tree(&mut parser)?;
    if parser.has_more() {
        let token = parser.next_token().unwrap_or_default();
        return Err(TreeParseError::UnexpectedToken(token));
    }
    Ok(tree)
}

fn parse_tree(parser: &mut TokenParser) -> Result<TreeValue, TreeParseError> {
    let token = parser.next_token().ok_or(TreeParseError::UnexpectedEof)?;
    if token == "(" {
        let label = parser.next_token().ok_or(TreeParseError::MissingLabel)?;
        if label == ")" {
            return Err(TreeParseError::MissingLabel);
        }
        let mut children = Vec::new();
        loop {
            let next = parser.peek_token().ok_or(TreeParseError::UnexpectedEof)?;
            if next == ")" {
                parser.next_token();
                break;
            }
            let child = parse_tree(parser)?;
            children.push(child);
        }
        Ok(TreeValue::node(label, children))
    } else if token == ")" {
        Err(TreeParseError::UnbalancedParens)
    } else {
        Ok(TreeValue::leaf(TreeLeafValue::Text(token)))
    }
}

struct TokenParser {
    tokens: Vec<String>,
    index: usize,
}

impl TokenParser {
    fn new(tokens: Vec<String>) -> Self {
        Self { tokens, index: 0 }
    }

    fn next_token(&mut self) -> Option<String> {
        if self.index >= self.tokens.len() {
            return None;
        }
        let token = self.tokens[self.index].clone();
        self.index += 1;
        Some(token)
    }

    fn peek_token(&self) -> Option<&str> {
        self.tokens.get(self.index).map(|t| t.as_str())
    }

    fn has_more(&self) -> bool {
        self.index < self.tokens.len()
    }
}

fn tokenize(input: &str) -> Result<Vec<String>, TreeParseError> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.peek().copied() {
        if ch.is_whitespace() {
            chars.next();
            continue;
        }
        if ch == '(' || ch == ')' {
            tokens.push(ch.to_string());
            chars.next();
            continue;
        }
        if ch == '"' {
            chars.next();
            let mut buffer = String::new();
            while let Some(next) = chars.next() {
                match next {
                    '\\' => {
                        if let Some(escaped) = chars.next() {
                            buffer.push(escaped);
                        }
                    }
                    '"' => break,
                    _ => buffer.push(next),
                }
            }
            tokens.push(buffer);
            continue;
        }

        let mut buffer = String::new();
        while let Some(next) = chars.peek().copied() {
            if next.is_whitespace() || next == '(' || next == ')' {
                break;
            }
            buffer.push(next);
            chars.next();
        }
        if buffer.is_empty() {
            return Err(TreeParseError::UnexpectedEof);
        }
        tokens.push(buffer);
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_bracketed_tree() {
        let tree = parse_bracketed("(S (NP Mary) (VP walks))").unwrap();
        match tree {
            TreeValue::Node(node) => {
                assert_eq!(node.label(), "S");
                assert_eq!(node.children().len(), 2);
            }
            _ => panic!("expected node"),
        }
    }
}
