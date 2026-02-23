//! Advanced Tree Transformations
//!
//! Provides parity for operations like `un_chomsky_normal_form`,
//! and Markov smoothing / Parent annotation.

use crate::collections::dataset::tree::TreeValue;

pub trait TransformTreeExt {
    /// Undo Chomsky Normal Form (CNF) binarization.
    ///
    /// Restores a tree that was binarized (e.g. `A|<B-C>`) back into a flat `A -> B C`.
    fn un_chomsky_normal_form(
        &self,
        expand_unary: bool,
        child_char: &str,
        parent_char: &str,
        unary_char: &str,
    ) -> TreeValue;

    /// Annotate node labels with their parent labels (e.g., `NP^S`).
    fn parent_annotate(&self, parent_char: &str) -> TreeValue;
}

impl TransformTreeExt for TreeValue {
    fn un_chomsky_normal_form(
        &self,
        expand_unary: bool,
        child_char: &str,
        parent_char: &str,
        unary_char: &str,
    ) -> TreeValue {
        // Simple recursive implementation mimicking NLTK
        // NLTK primarily looks for child_char in labels to flatten them.
        un_cnf(self, expand_unary, child_char, parent_char, unary_char)
    }

    fn parent_annotate(&self, parent_char: &str) -> TreeValue {
        annotate_parents(self, "ROOT", parent_char)
    }
}

fn un_cnf(
    tree: &TreeValue,
    expand_unary: bool,
    child_char: &str,
    parent_char: &str,
    unary_char: &str,
) -> TreeValue {
    if let TreeValue::Node(node) = tree {
        let mut new_children = Vec::new();
        for child in node.children() {
            let un_child = un_cnf(child, expand_unary, child_char, parent_char, unary_char);

            // If the child is a node and has child_char in its label, we hoist its children up.
            if let TreeValue::Node(un_c_node) = &un_child {
                if un_c_node.label().contains(child_char) {
                    new_children.extend(un_c_node.children().iter().cloned());
                    continue;
                }
            }
            new_children.push(un_child);
        }

        let mut new_label = node.label().to_string();

        // Remove parent annotation
        if let Some(idx) = new_label.find(parent_char) {
            new_label.truncate(idx);
        }

        // Un-collapse unary nodes if requested
        if expand_unary && new_label.contains(unary_char) {
            let parts: Vec<&str> = new_label.split(unary_char).collect();
            // This reconstitutes unary chains A+B+C -> A -> B -> C
            let mut current = TreeValue::node(parts.last().unwrap().to_string(), new_children);
            for part in parts.iter().rev().skip(1) {
                current = TreeValue::node(part.to_string(), vec![current]);
            }
            return current;
        }

        TreeValue::node(new_label, new_children)
    } else {
        tree.clone()
    }
}

fn annotate_parents(tree: &TreeValue, parent_label: &str, parent_char: &str) -> TreeValue {
    if let TreeValue::Node(node) = tree {
        let label = node.label();
        let new_label = format!("{}{}{}", label, parent_char, parent_label);
        let annotated_children: Vec<TreeValue> = node
            .children()
            .iter()
            .map(|c| annotate_parents(c, label, parent_char))
            .collect();
        TreeValue::node(new_label, annotated_children)
    } else {
        tree.clone()
    }
}
