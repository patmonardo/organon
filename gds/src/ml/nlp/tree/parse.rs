//! Parsing for tree strings.
//!
//! Provides parity with `nltk.tree.Tree.fromstring()`.

use crate::collections::dataset::tree::{TreeLeafValue, TreeValue};

pub trait ParseTreeExt {
    /// Parse a bracketed tree string (e.g. Penn Treebank format) into a `TreeValue`.
    ///
    /// Matches `nltk.tree.Tree.fromstring`.
    fn fromstring(s: &str) -> Result<TreeValue, String>;
}

impl ParseTreeExt for TreeValue {
    fn fromstring(s: &str) -> Result<TreeValue, String> {
        let (tree, rest) = parse_node(s.trim())?;
        if !rest.trim().is_empty() {
            return Err(format!("Trailing characters after tree: '{}'", rest));
        }
        Ok(tree)
    }
}

fn parse_node(mut s: &str) -> Result<(TreeValue, &str), String> {
    s = s.trim_start();
    if !s.starts_with('(') {
        // It's a leaf
        return parse_leaf(s);
    }

    // Skip the '('
    s = &s[1..].trim_start();

    // Extract label
    let label_end = s
        .find(|c: char| c.is_whitespace() || c == '(' || c == ')')
        .unwrap_or(s.len());
    if label_end == 0 {
        return Err("Expected label after '('".to_string());
    }
    let label = &s[..label_end];
    s = &s[label_end..];

    let mut children = Vec::new();

    loop {
        s = s.trim_start();
        if s.is_empty() {
            return Err("Unexpected end of string, missing ')'".to_string());
        }
        if s.starts_with(')') {
            s = &s[1..];
            break;
        }

        let (child, next_s) = parse_node(s)?;
        children.push(child);
        s = next_s;
    }

    Ok((TreeValue::node(label.to_string(), children), s))
}

fn parse_leaf(s: &str) -> Result<(TreeValue, &str), String> {
    // A leaf is everything up to the next whitespace or ')'
    let end = s
        .find(|c: char| c.is_whitespace() || c == ')')
        .unwrap_or(s.len());
    if end == 0 {
        return Err("Expected leaf value".to_string());
    }
    let leaf_text = &s[..end];
    let rest = &s[end..];

    // We treat it as Text for now to match NLTK's default behavior
    Ok((
        TreeValue::leaf(TreeLeafValue::text(leaf_text.to_string())),
        rest,
    ))
}
