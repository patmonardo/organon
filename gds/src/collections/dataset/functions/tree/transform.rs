//! Tree transforms.
//!
//! These are pure transforms over concrete tree values.

use crate::collections::dataset::tree::{TreeLeafValue, TreeNode, TreeValue};

const DEFAULT_UNARY_JOIN: &str = "+";
const DEFAULT_CNF_CHILD_JOIN: &str = "-";
const DEFAULT_CNF_NODE_SEP: &str = "|";
const DEFAULT_CNF_BRACKET_L: &str = "<";
const DEFAULT_CNF_BRACKET_R: &str = ">";
const DEFAULT_CNF_LEAF_LABEL: &str = "LEAF";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizeOptions {
    pub preserve_empty_leaf: bool,
}

impl Default for NormalizeOptions {
    fn default() -> Self {
        Self {
            preserve_empty_leaf: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CollapseUnaryOptions {
    pub join_char: String,
    pub collapse_pos: bool,
    pub collapse_root: bool,
}

impl Default for CollapseUnaryOptions {
    fn default() -> Self {
        Self {
            join_char: DEFAULT_UNARY_JOIN.to_string(),
            collapse_pos: false,
            collapse_root: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CnfFactor {
    Right,
    Left,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CnfOptions {
    pub factor: CnfFactor,
    pub child_join: String,
    pub node_sep: String,
    pub bracket_l: String,
    pub bracket_r: String,
    pub leaf_label: String,
}

impl Default for CnfOptions {
    fn default() -> Self {
        Self {
            factor: CnfFactor::Right,
            child_join: DEFAULT_CNF_CHILD_JOIN.to_string(),
            node_sep: DEFAULT_CNF_NODE_SEP.to_string(),
            bracket_l: DEFAULT_CNF_BRACKET_L.to_string(),
            bracket_r: DEFAULT_CNF_BRACKET_R.to_string(),
            leaf_label: DEFAULT_CNF_LEAF_LABEL.to_string(),
        }
    }
}

pub fn normalize(tree: &TreeValue) -> TreeValue {
    normalize_with(tree, &NormalizeOptions::default())
}

pub fn normalize_with(tree: &TreeValue, options: &NormalizeOptions) -> TreeValue {
    match tree {
        TreeValue::Leaf(_) => tree.clone(),
        TreeValue::Node(node) => {
            let mut children: Vec<TreeValue> = node
                .children()
                .iter()
                .map(|child| normalize_with(child, options))
                .filter(|child| !is_empty_leaf(child))
                .collect();

            if options.preserve_empty_leaf
                && children.is_empty()
                && has_empty_leaf_child(node.children())
            {
                children.push(TreeValue::leaf(TreeLeafValue::Empty));
            }

            TreeValue::Node(copy_node_with_meta(
                node,
                node.label().to_string(),
                children,
            ))
        }
    }
}

pub fn collapse_unary(tree: &TreeValue) -> TreeValue {
    collapse_unary_with(tree, &CollapseUnaryOptions::default())
}

pub fn collapse_unary_with(tree: &TreeValue, options: &CollapseUnaryOptions) -> TreeValue {
    collapse_unary_with_root(tree, options, true)
}

pub fn chomsky_normal_form(tree: &TreeValue) -> TreeValue {
    chomsky_normal_form_with(tree, &CnfOptions::default())
}

pub fn chomsky_normal_form_with(tree: &TreeValue, options: &CnfOptions) -> TreeValue {
    match tree {
        TreeValue::Leaf(_) => tree.clone(),
        TreeValue::Node(node) => {
            let mut children: Vec<TreeValue> = node
                .children()
                .iter()
                .map(|child| chomsky_normal_form_with(child, options))
                .collect();
            if children.len() <= 2 {
                return TreeValue::Node(copy_node_with_meta(
                    node,
                    node.label().to_string(),
                    children,
                ));
            }

            let out = match options.factor {
                CnfFactor::Right => {
                    let first = children.remove(0);
                    let synthetic = cnf_right_factor(node.label(), children, options);
                    TreeNode::new(node.label().to_string(), vec![first, synthetic])
                }
                CnfFactor::Left => {
                    let last = children.pop().expect("children must be non-empty");
                    let synthetic = cnf_left_factor(node.label(), children, options);
                    TreeNode::new(node.label().to_string(), vec![synthetic, last])
                }
            };

            TreeValue::Node(merge_meta(node, out))
        }
    }
}

fn collapse_unary_with_root(
    tree: &TreeValue,
    options: &CollapseUnaryOptions,
    is_root: bool,
) -> TreeValue {
    match tree {
        TreeValue::Leaf(_) => tree.clone(),
        TreeValue::Node(node) => {
            let children: Vec<TreeValue> = node
                .children()
                .iter()
                .map(|child| collapse_unary_with_root(child, options, false))
                .collect();
            let candidate = TreeValue::Node(copy_node_with_meta(
                node,
                node.label().to_string(),
                children,
            ));
            collapse_unary_once(&candidate, options, is_root)
        }
    }
}

fn collapse_unary_once(
    tree: &TreeValue,
    options: &CollapseUnaryOptions,
    is_root: bool,
) -> TreeValue {
    if !options.collapse_root && is_root {
        return tree.clone();
    }

    if let TreeValue::Node(node) = tree {
        if node.children().len() == 1 {
            if let TreeValue::Node(child) = &node.children()[0] {
                if !options.collapse_pos && child.label() == "POS" && child.children().len() == 1 {
                    if let TreeValue::Leaf(_) = child.children()[0] {
                        return tree.clone();
                    }
                }

                let merged_label =
                    format!("{}{}{}", node.label(), options.join_char, child.label());
                let mut merged = TreeNode::new(merged_label, child.children().to_vec());
                merged = merge_meta(node, merged);
                merged = merge_meta(child, merged);
                return collapse_unary_with_root(&TreeValue::Node(merged), options, is_root);
            }
        }
    }

    tree.clone()
}

fn cnf_right_factor(base_label: &str, children: Vec<TreeValue>, options: &CnfOptions) -> TreeValue {
    if children.len() == 2 {
        let labels = children
            .iter()
            .map(|child| child_label(child, options))
            .collect::<Vec<String>>();
        let synthetic_label = cnf_label(base_label, &labels, options);
        return TreeValue::Node(TreeNode::new(synthetic_label, children));
    }

    let mut rest = children;
    let labels = rest
        .iter()
        .map(|child| child_label(child, options))
        .collect::<Vec<String>>();
    let synthetic_label = cnf_label(base_label, &labels, options);
    let first = rest.remove(0);
    let right = cnf_right_factor(base_label, rest, options);
    TreeValue::Node(TreeNode::new(synthetic_label, vec![first, right]))
}

fn cnf_left_factor(base_label: &str, children: Vec<TreeValue>, options: &CnfOptions) -> TreeValue {
    if children.len() == 2 {
        let labels = children
            .iter()
            .map(|child| child_label(child, options))
            .collect::<Vec<String>>();
        let synthetic_label = cnf_label(base_label, &labels, options);
        return TreeValue::Node(TreeNode::new(synthetic_label, children));
    }

    let mut rest = children;
    let last = rest.pop().expect("children must be non-empty");
    let labels = rest
        .iter()
        .map(|child| child_label(child, options))
        .collect::<Vec<String>>();
    let synthetic_label = cnf_label(base_label, &labels, options);
    let left = cnf_left_factor(base_label, rest, options);
    TreeValue::Node(TreeNode::new(synthetic_label, vec![left, last]))
}

fn cnf_label(base_label: &str, child_labels: &[String], options: &CnfOptions) -> String {
    format!(
        "{base}{sep}{l}{inner}{r}",
        base = base_label,
        sep = options.node_sep,
        l = options.bracket_l,
        inner = child_labels.join(&options.child_join),
        r = options.bracket_r,
    )
}

fn child_label(child: &TreeValue, options: &CnfOptions) -> String {
    match child {
        TreeValue::Node(node) => node.label().to_string(),
        TreeValue::Leaf(_) => options.leaf_label.clone(),
    }
}

fn has_empty_leaf_child(children: &[TreeValue]) -> bool {
    children.iter().any(is_empty_leaf)
}

fn is_empty_leaf(child: &TreeValue) -> bool {
    matches!(child, TreeValue::Leaf(TreeLeafValue::Empty))
}

fn copy_node_with_meta(node: &TreeNode, label: String, children: Vec<TreeValue>) -> TreeNode {
    merge_meta(node, TreeNode::new(label, children))
}

fn merge_meta(source: &TreeNode, mut target: TreeNode) -> TreeNode {
    if let Some(span) = source.span() {
        target = target.with_span(*span);
    }
    if let Some(id) = source.id() {
        target = target.with_id(id);
    }
    target
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::dataset::tree::TreeLeafValue;

    fn leaf_text(value: &str) -> TreeValue {
        TreeValue::leaf(TreeLeafValue::text(value))
    }

    fn empty_leaf() -> TreeValue {
        TreeValue::leaf(TreeLeafValue::Empty)
    }

    fn node(label: &str, children: Vec<TreeValue>) -> TreeValue {
        TreeValue::node(label.to_string(), children)
    }

    #[test]
    fn normalize_drops_empty_leaves() {
        let tree = node("S", vec![leaf_text("a"), empty_leaf(), leaf_text("b")]);
        let out = normalize(&tree);

        match out {
            TreeValue::Node(node) => {
                assert_eq!(node.label(), "S");
                assert_eq!(node.children().len(), 2);
            }
            _ => panic!("expected node"),
        }
    }

    #[test]
    fn normalize_preserves_single_empty_child() {
        let tree = node("S", vec![empty_leaf(), empty_leaf()]);
        let out = normalize(&tree);

        match out {
            TreeValue::Node(node) => {
                assert_eq!(node.children().len(), 1);
                assert!(matches!(
                    node.children()[0],
                    TreeValue::Leaf(TreeLeafValue::Empty)
                ));
            }
            _ => panic!("expected node"),
        }
    }

    #[test]
    fn collapse_unary_merges_labels() {
        let tree = node("A", vec![node("B", vec![node("C", vec![leaf_text("x")])])]);
        let out = collapse_unary(&tree);

        match out {
            TreeValue::Node(node) => {
                assert_eq!(node.label(), "A+B+C");
                assert_eq!(node.children().len(), 1);
            }
            _ => panic!("expected node"),
        }
    }

    #[test]
    fn collapse_unary_respects_pos_flag() {
        let tree = node("A", vec![node("POS", vec![leaf_text("x")])]);
        let mut options = CollapseUnaryOptions::default();
        options.collapse_pos = false;

        let out = collapse_unary_with(&tree, &options);
        match out {
            TreeValue::Node(node) => {
                assert_eq!(node.label(), "A");
                assert_eq!(node.children().len(), 1);
                assert_eq!(node.children()[0].as_node().unwrap().label(), "POS");
            }
            _ => panic!("expected node"),
        }
    }

    #[test]
    fn collapse_unary_respects_root_flag() {
        let tree = node("A", vec![node("B", vec![leaf_text("x")])]);
        let mut options = CollapseUnaryOptions::default();
        options.collapse_root = false;

        let out = collapse_unary_with(&tree, &options);
        match out {
            TreeValue::Node(node) => {
                assert_eq!(node.label(), "A");
                assert_eq!(node.children().len(), 1);
                assert_eq!(node.children()[0].as_node().unwrap().label(), "B");
            }
            _ => panic!("expected node"),
        }
    }

    #[test]
    fn chomsky_normal_form_binarizes() {
        let tree = node(
            "A",
            vec![node("B", vec![]), node("C", vec![]), node("D", vec![])],
        );
        let out = chomsky_normal_form(&tree);

        match out {
            TreeValue::Node(node) => {
                assert_eq!(node.label(), "A");
                assert_eq!(node.children().len(), 2);

                match &node.children()[1] {
                    TreeValue::Node(synthetic) => {
                        assert_eq!(synthetic.label(), "A|<C-D>");
                        assert_eq!(synthetic.children().len(), 2);
                    }
                    _ => panic!("expected synthetic node"),
                }
            }
            _ => panic!("expected node"),
        }
    }

    #[test]
    fn chomsky_normal_form_left_factor() {
        let tree = node(
            "A",
            vec![node("B", vec![]), node("C", vec![]), node("D", vec![])],
        );
        let mut options = CnfOptions::default();
        options.factor = CnfFactor::Left;

        let out = chomsky_normal_form_with(&tree, &options);
        match out {
            TreeValue::Node(node) => {
                assert_eq!(node.label(), "A");
                assert_eq!(node.children().len(), 2);

                match &node.children()[0] {
                    TreeValue::Node(synthetic) => {
                        assert_eq!(synthetic.label(), "A|<B-C>");
                        assert_eq!(synthetic.children().len(), 2);
                    }
                    _ => panic!("expected synthetic node"),
                }
            }
            _ => panic!("expected node"),
        }
    }
}
