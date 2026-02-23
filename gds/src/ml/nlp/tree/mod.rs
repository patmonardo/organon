//! NLP Tree Module.
//!
//! This module provides NLP-focused extensions and wrappers over the dataset's tree primitives.
//! It aims to offer an API similar to Python's NLTK `nltk.tree` module while using the
//! fast data representations from `dataset::tree::TreeValue`.

pub mod grammar;
pub mod parse;
pub mod query;
pub mod transform;

pub use grammar::{GrammarTreeExt, Nonterminal, Production, ProductionRhs};
pub use parse::ParseTreeExt;
pub use query::QueryTreeExt;
pub use transform::TransformTreeExt;

use crate::collections::dataset::tree::{TreeLeafValue, TreeValue};

/// The main extension trait that provides NLTK-like capabilities to the base `TreeValue`.
pub trait NlpTree {
    /// Return the node label of the tree, or `None` if this is a leaf.
    fn label(&self) -> Option<&str>;

    /// Return a single flat list of all leaves in strings/tokens.
    fn text_leaves(&self) -> Vec<String>;
}

impl NlpTree for TreeValue {
    fn label(&self) -> Option<&str> {
        self.as_node().map(|n| n.label())
    }

    fn text_leaves(&self) -> Vec<String> {
        let mut leaves = Vec::new();
        collect_text_leaves(self, &mut leaves);
        leaves
    }
}

fn collect_text_leaves(tree: &TreeValue, leaves: &mut Vec<String>) {
    match tree {
        TreeValue::Leaf(leaf) => match leaf {
            TreeLeafValue::Text(s) => leaves.push(s.clone()),
            TreeLeafValue::TokenIndex(idx) => leaves.push(format!("TokenIndex({})", idx)),
            TreeLeafValue::Number(n) => leaves.push(n.to_string()),
            TreeLeafValue::Bool(b) => leaves.push(b.to_string()),
            TreeLeafValue::BytesRange { start, end } => {
                leaves.push(format!("BytesRange({},{})", start, end))
            }
            TreeLeafValue::Empty => {}
        },
        TreeValue::Node(node) => {
            for child in node.children() {
                collect_text_leaves(child, leaves);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fromstring_and_productions() {
        let s = "(S (NP (D the) (N dog)) (VP (V chased) (NP (D the) (N cat))))";
        let tree = TreeValue::fromstring(s).expect("Failed to parse tree");

        assert_eq!(tree.label(), Some("S"));
        assert_eq!(
            tree.text_leaves(),
            vec!["the", "dog", "chased", "the", "cat"]
        );

        let prods = tree.productions();
        assert_eq!(prods.len(), 9);
        assert_eq!(prods[0].lhs.symbol(), "S");
    }

    #[test]
    fn test_parent_annotate() {
        let s = "(S (NP (D the) (N dog)))";
        let tree = TreeValue::fromstring(s).unwrap();
        let annotated = tree.parent_annotate("^");

        assert_eq!(annotated.label(), Some("S^ROOT"));

        if let TreeValue::Node(n) = annotated {
            let np = &n.children()[0];
            assert_eq!(np.as_node().unwrap().label(), "NP^S");
        }
    }
}
