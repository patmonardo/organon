//! Tree inspection helpers.

use crate::collections::dataset::expressions::tree::TreePos;
use crate::collections::dataset::tree::{TreeLeafValue, TreeValue};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TreeTraversal {
    PreOrder,
    PostOrder,
    BothOrder,
    Leaves,
}

pub fn leaves(tree: &TreeValue) -> Vec<TreeLeafValue> {
    let mut out = Vec::new();
    collect_leaves(tree, &mut out);
    out
}

fn collect_leaves(tree: &TreeValue, out: &mut Vec<TreeLeafValue>) {
    match tree {
        TreeValue::Leaf(value) => out.push(value.clone()),
        TreeValue::Node(node) => {
            for child in node.children() {
                collect_leaves(child, out);
            }
        }
    }
}

pub fn height(tree: &TreeValue) -> usize {
    match tree {
        TreeValue::Leaf(_) => 1,
        TreeValue::Node(node) => {
            let mut max_child = 0;
            for child in node.children() {
                max_child = max_child.max(height(child));
            }
            max_child + 1
        }
    }
}

pub fn treepositions(tree: &TreeValue, order: TreeTraversal) -> Vec<TreePos> {
    let mut out = Vec::new();
    match order {
        TreeTraversal::PreOrder => treepositions_pre(tree, &mut Vec::new(), &mut out),
        TreeTraversal::PostOrder => treepositions_post(tree, &mut Vec::new(), &mut out),
        TreeTraversal::BothOrder => treepositions_both(tree, &mut Vec::new(), &mut out),
        TreeTraversal::Leaves => treepositions_leaves(tree, &mut Vec::new(), &mut out),
    }
    out
}

fn treepositions_pre(tree: &TreeValue, path: &mut Vec<usize>, out: &mut Vec<TreePos>) {
    out.push(TreePos::new(path.clone()));
    if let TreeValue::Node(node) = tree {
        for (idx, child) in node.children().iter().enumerate() {
            path.push(idx);
            treepositions_pre(child, path, out);
            path.pop();
        }
    }
}

fn treepositions_post(tree: &TreeValue, path: &mut Vec<usize>, out: &mut Vec<TreePos>) {
    if let TreeValue::Node(node) = tree {
        for (idx, child) in node.children().iter().enumerate() {
            path.push(idx);
            treepositions_post(child, path, out);
            path.pop();
        }
    }
    out.push(TreePos::new(path.clone()));
}

fn treepositions_both(tree: &TreeValue, path: &mut Vec<usize>, out: &mut Vec<TreePos>) {
    out.push(TreePos::new(path.clone()));
    if let TreeValue::Node(node) = tree {
        for (idx, child) in node.children().iter().enumerate() {
            path.push(idx);
            treepositions_both(child, path, out);
            path.pop();
        }
    }
    out.push(TreePos::new(path.clone()));
}

fn treepositions_leaves(tree: &TreeValue, path: &mut Vec<usize>, out: &mut Vec<TreePos>) {
    match tree {
        TreeValue::Leaf(_) => out.push(TreePos::new(path.clone())),
        TreeValue::Node(node) => {
            for (idx, child) in node.children().iter().enumerate() {
                path.push(idx);
                treepositions_leaves(child, path, out);
                path.pop();
            }
        }
    }
}

pub fn leaf_treeposition(tree: &TreeValue, index: usize) -> Option<TreePos> {
    let positions = treepositions(tree, TreeTraversal::Leaves);
    positions.get(index).cloned()
}

pub fn treeposition_spanning_leaves(tree: &TreeValue, start: usize, end: usize) -> Option<TreePos> {
    if start >= end {
        return None;
    }
    let positions = treepositions(tree, TreeTraversal::Leaves);
    if end > positions.len() {
        return None;
    }
    let a = positions.get(start)?;
    let b = positions.get(end - 1)?;
    let mut prefix = Vec::new();
    for (left, right) in a.path().iter().zip(b.path().iter()) {
        if left != right {
            break;
        }
        prefix.push(*left);
    }
    Some(TreePos::new(prefix))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::dataset::tree::TreeValue;

    #[test]
    fn treepositions_preorder_includes_root() {
        let tree = TreeValue::node("S", vec![TreeValue::leaf("x")]);
        let positions = treepositions(&tree, TreeTraversal::PreOrder);
        assert!(positions[0].path().is_empty());
    }

    #[test]
    fn leaf_treeposition_returns_leaf_path() {
        let tree = TreeValue::node("S", vec![TreeValue::leaf("x")]);
        let pos = leaf_treeposition(&tree, 0).unwrap();
        assert_eq!(pos.path(), &[0]);
    }
}
