//! Subtree Querying / Filtering
//!
//! Provides parity with `nltk.tree.Tree.subtrees()`.

use crate::collections::dataset::tree::TreeValue;

pub trait QueryTreeExt {
    /// Return a sequence of all subtrees of this tree.
    fn subtrees<F>(&self, filter: F) -> Vec<&TreeValue>
    where
        F: FnMut(&TreeValue) -> bool;
}

impl QueryTreeExt for TreeValue {
    fn subtrees<F>(&self, mut filter: F) -> Vec<&TreeValue>
    where
        F: FnMut(&TreeValue) -> bool,
    {
        let mut results = Vec::new();
        collect_subtrees(self, &mut filter, &mut results);
        results
    }
}

fn collect_subtrees<'a, F>(tree: &'a TreeValue, filter: &mut F, results: &mut Vec<&'a TreeValue>)
where
    F: FnMut(&TreeValue) -> bool,
{
    if filter(tree) {
        results.push(tree);
    }
    if let TreeValue::Node(node) = tree {
        for child in node.children() {
            collect_subtrees(child, filter, results);
        }
    }
}
