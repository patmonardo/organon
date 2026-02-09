//! Tree formatting helpers.

use crate::collections::dataset::tree::{TreeLeafValue, TreeValue};

pub fn format_bracketed(tree: &TreeValue) -> String {
    match tree {
        TreeValue::Leaf(value) => format_leaf(value),
        TreeValue::Node(node) => {
            let mut out = String::new();
            out.push('(');
            out.push_str(&format_token(node.label()));
            for child in node.children() {
                out.push(' ');
                out.push_str(&format_bracketed(child));
            }
            out.push(')');
            out
        }
    }
}

pub fn format_pretty(tree: &TreeValue, indent: usize) -> String {
    format_pretty_inner(tree, 0, indent)
}

fn format_pretty_inner(tree: &TreeValue, depth: usize, indent: usize) -> String {
    match tree {
        TreeValue::Leaf(value) => format_leaf(value),
        TreeValue::Node(node) => {
            if node.children().is_empty() {
                return format!("({})", format_token(node.label()));
            }
            let pad = " ".repeat(depth * indent);
            let child_pad = " ".repeat((depth + 1) * indent);
            let mut out = String::new();
            out.push_str(&pad);
            out.push('(');
            out.push_str(&format_token(node.label()));
            let multiline = node.children().iter().any(|child| matches!(child, TreeValue::Node(_)));
            if multiline {
                for child in node.children() {
                    out.push('\n');
                    out.push_str(&child_pad);
                    out.push_str(&format_pretty_inner(child, depth + 1, indent));
                }
                out.push('\n');
                out.push_str(&pad);
                out.push(')');
            } else {
                for child in node.children() {
                    out.push(' ');
                    out.push_str(&format_pretty_inner(child, depth + 1, indent));
                }
                out.push(')');
            }
            out
        }
    }
}

fn format_leaf(value: &TreeLeafValue) -> String {
    match value {
        TreeLeafValue::Text(value) => format_token(value),
        TreeLeafValue::TokenIndex(index) => index.to_string(),
        TreeLeafValue::Number(value) => value.to_string(),
        TreeLeafValue::Bool(value) => value.to_string(),
        TreeLeafValue::BytesRange { start, end } => format!("{start}..{end}"),
        TreeLeafValue::Empty => "_".to_string(),
    }
}

fn format_token(value: &str) -> String {
    if value.is_empty() {
        return "\"\"".to_string();
    }
    if value.chars().any(|c| c.is_whitespace() || c == '(' || c == ')') {
        let escaped = value.replace('"', "\\\"");
        return format!("\"{escaped}\"");
    }
    value.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::dataset::tree::{TreeLeafValue, TreeValue};

    #[test]
    fn format_bracketed_round_trip() {
        let tree = TreeValue::node(
            "S",
            vec![
                TreeValue::node("NP", vec![TreeValue::leaf(TreeLeafValue::text("Mary"))]),
                TreeValue::node("VP", vec![TreeValue::leaf(TreeLeafValue::text("walks"))]),
            ],
        );
        let out = format_bracketed(&tree);
        assert!(out.contains("(S"));
    }
}
