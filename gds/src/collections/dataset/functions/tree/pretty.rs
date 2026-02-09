//! Tree pretty-printer utilities.

use crate::collections::dataset::tree::{TreeLeafValue, TreeValue};

#[derive(Debug, Clone, Copy)]
pub struct PrettyOptions {
    pub indent: usize,
    pub unicode: bool,
}

impl Default for PrettyOptions {
    fn default() -> Self {
        Self {
            indent: 2,
            unicode: false,
        }
    }
}

pub fn pretty_print(tree: &TreeValue, options: PrettyOptions) -> String {
    let mut lines = Vec::new();
    lines.push(node_label(tree));
    if let TreeValue::Node(node) = tree {
        let total = node.children().len();
        for (idx, child) in node.children().iter().enumerate() {
            let is_last = idx + 1 == total;
            build_lines(child, "", is_last, options, &mut lines);
        }
    }
    lines.join("\n")
}

fn build_lines(
    tree: &TreeValue,
    prefix: &str,
    is_last: bool,
    options: PrettyOptions,
    lines: &mut Vec<String>,
) {
    let (branch, next_prefix) = branch_tokens(is_last, options);
    let label = node_label(tree);
    lines.push(format!("{prefix}{branch} {label}"));
    if let TreeValue::Node(node) = tree {
        let total = node.children().len();
        let new_prefix = format!("{prefix}{next_prefix}");
        for (idx, child) in node.children().iter().enumerate() {
            let child_last = idx + 1 == total;
            build_lines(child, &new_prefix, child_last, options, lines);
        }
    }
}

fn branch_tokens(is_last: bool, options: PrettyOptions) -> (&'static str, String) {
    if options.unicode {
        if is_last {
            ("└──", "    ".to_string())
        } else {
            ("├──", "│   ".to_string())
        }
    } else if is_last {
        ("`--", "    ".to_string())
    } else {
        ("+--", "|   ".to_string())
    }
}

fn node_label(tree: &TreeValue) -> String {
    match tree {
        TreeValue::Node(node) => node.label().to_string(),
        TreeValue::Leaf(value) => leaf_label(value),
    }
}

fn leaf_label(value: &TreeLeafValue) -> String {
    match value {
        TreeLeafValue::Text(value) => value.clone(),
        TreeLeafValue::TokenIndex(index) => index.to_string(),
        TreeLeafValue::Number(value) => value.to_string(),
        TreeLeafValue::Bool(value) => value.to_string(),
        TreeLeafValue::BytesRange { start, end } => format!("{start}..{end}"),
        TreeLeafValue::Empty => "_".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::dataset::tree::TreeValue;

    #[test]
    fn pretty_print_includes_root_label() {
        let tree = TreeValue::node("S", vec![TreeValue::leaf("x")]);
        let out = pretty_print(&tree, PrettyOptions::default());
        assert!(out.lines().next().unwrap_or("").contains("S"));
    }
}
