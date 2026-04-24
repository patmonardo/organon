//! TGrep search over dataset trees — port of NLTK `nltk/tgrep.py`.
//!
//! TGrep2 is a small declarative language for matching subtrees of a
//! parsed corpus by structural relations: dominance (`<`, `<<`),
//! sisterhood (`$`), linear precedence (`.`, `..`), nth-child (`<N`),
//! etc. NLTK's `tgrep` builds these as composable lambda predicates;
//! this module ports the **kernel-applicable** core of that idea on
//! top of the existing dataset [`TreeValue`] type.
//!
//! ## Scope
//!
//! Translated:
//! - All tree-walking helpers from the NLTK module: [`ancestors`],
//!   [`unique_ancestors`], [`descendants`], [`leftmost_descendants`],
//!   [`rightmost_descendants`], [`unique_descendants`], [`before`],
//!   [`after`], [`immediately_before`], [`immediately_after`].
//! - Node-literal value extraction: [`node_literal_value`].
//! - The full TGrep2 relation operator table as composable Rust
//!   combinators ([`parent_of`], [`child_of`], [`dominates`],
//!   [`dominated_by`], [`precedes`], [`follows`], [`sister_of`], …).
//! - Logical combinators [`and_`], [`or_`], [`not_`].
//! - Drivers [`tgrep_positions`] and [`tgrep_nodes`].
//!
//! Deferred (will be fleshed out as the dataset DSL grows):
//! - The pyparsing-based TGrep2 **string compiler** (macro defs, `=`
//!   labels, `i@…` case-insensitive matchers, parenthetical grouping,
//!   `'`-prefix print directive). The Rust API here gives the full
//!   matching machinery; a string-front-end can be layered on top
//!   when the dataset namespace gains its own parser surface.
//!
//! ## Trees
//!
//! Trees are [`TreeValue`]s. A *node* is referenced by a path: the
//! sequence of child indices from the root. `[]` is the root, `[0,2]`
//! is the third child of the first child, and so on. This matches
//! NLTK's `treeposition` tuples and lets us recover parent /
//! sibling / root information without a `ParentedTree`.
//!
//! Predicates are `Arc<dyn Fn(MatchCtx) -> bool + Send + Sync>`
//! ([`Predicate`]); a [`MatchCtx`] bundles the root tree with the
//! path of the currently-considered node.

use std::sync::Arc;

use regex::Regex;

use crate::collections::dataset::expressions::tree::TreePos;
use crate::collections::dataset::tree::{TreeLeafValue, TreeNode, TreeValue};

// ===========================================================================
// Tree positions and lookup
// ===========================================================================

/// Path from the root of a tree to a node. The root is `[]`.
pub type TreePath = Vec<usize>;

/// Pair of root and currently-considered node path.
#[derive(Debug, Clone, Copy)]
pub struct MatchCtx<'a> {
    pub root: &'a TreeValue,
    pub path: &'a [usize],
}

impl<'a> MatchCtx<'a> {
    pub fn new(root: &'a TreeValue, path: &'a [usize]) -> Self {
        Self { root, path }
    }

    pub fn node(&self) -> Option<&'a TreeValue> {
        node_at(self.root, self.path)
    }
}

/// Resolve a path to the corresponding [`TreeValue`].
pub fn node_at<'a>(root: &'a TreeValue, path: &[usize]) -> Option<&'a TreeValue> {
    let mut cur = root;
    for &idx in path {
        match cur {
            TreeValue::Node(n) => {
                cur = n.children().get(idx)?;
            }
            TreeValue::Leaf(_) => return None,
        }
    }
    Some(cur)
}

/// Return the parent path of `path`, or `None` if `path` is the root.
pub fn parent_path(path: &[usize]) -> Option<TreePath> {
    if path.is_empty() {
        None
    } else {
        Some(path[..path.len() - 1].to_vec())
    }
}

/// `true` iff `value` is a non-leaf node.
pub fn is_tree(value: &TreeValue) -> bool {
    value.is_node()
}

/// Borrow the inner [`TreeNode`] if `value` is a non-leaf node.
pub fn as_node(value: &TreeValue) -> Option<&TreeNode> {
    value.as_node()
}

/// All preorder tree positions of `root`, beginning with `[]`.
///
/// Thin wrapper over [`TreeValue::treepositions_preorder`] returning
/// `Vec<Vec<usize>>` instead of [`TreePos`] so callers can slice.
pub fn treepositions(root: &TreeValue) -> Vec<TreePath> {
    root.treepositions_preorder()
        .into_iter()
        .map(|p: TreePos| p.path().to_vec())
        .collect()
}

// ===========================================================================
// NLTK node-literal value
// ===========================================================================

/// Stringified value used by tgrep node-literal matching.
///
/// Mirrors NLTK `_tgrep_node_literal_value`: the label for a tree
/// node, otherwise a string form of the leaf.
pub fn node_literal_value(node: &TreeValue) -> String {
    match node {
        TreeValue::Node(n) => n.label().to_string(),
        TreeValue::Leaf(leaf) => leaf_to_string(leaf),
    }
}

fn leaf_to_string(leaf: &TreeLeafValue) -> String {
    match leaf {
        TreeLeafValue::Text(s) => s.clone(),
        TreeLeafValue::TokenIndex(i) => i.to_string(),
        TreeLeafValue::Number(n) => n.to_string(),
        TreeLeafValue::Bool(b) => b.to_string(),
        TreeLeafValue::BytesRange { start, end } => format!("{}..{}", start, end),
        TreeLeafValue::Empty => String::new(),
    }
}

// ===========================================================================
// Tree-walk helpers (paths only — equivalent to NLTK's helper functions)
// ===========================================================================

/// Paths of all proper ancestors of `path`, nearest first.
///
/// Mirrors NLTK `ancestors`. For the root, returns `[]`.
pub fn ancestors(path: &[usize]) -> Vec<TreePath> {
    let mut out = Vec::with_capacity(path.len());
    for i in (0..path.len()).rev() {
        out.push(path[..i].to_vec());
    }
    out
}

/// Ancestors traversed only while each ancestor has a single child.
///
/// Mirrors NLTK `unique_ancestors`.
pub fn unique_ancestors(root: &TreeValue, path: &[usize]) -> Vec<TreePath> {
    let mut out = Vec::new();
    let mut depth = path.len();
    while depth > 0 {
        let parent = path[..depth - 1].to_vec();
        let len = node_at(root, &parent)
            .and_then(as_node)
            .map(|n| n.children().len())
            .unwrap_or(0);
        if len != 1 {
            out.push(parent);
            return out;
        }
        out.push(parent);
        depth -= 1;
    }
    out
}

/// All proper descendants of the node at `path` (any depth).
pub fn descendants(root: &TreeValue, path: &[usize]) -> Vec<TreePath> {
    let Some(node) = node_at(root, path) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    walk_descendants(node, path.to_vec(), &mut out);
    out
}

fn walk_descendants(node: &TreeValue, path: TreePath, out: &mut Vec<TreePath>) {
    if let TreeValue::Node(n) = node {
        for (i, child) in n.children().iter().enumerate() {
            let mut child_path = path.clone();
            child_path.push(i);
            out.push(child_path.clone());
            walk_descendants(child, child_path, out);
        }
    }
}

/// Descendants reachable only via `[0, 0, …]` paths.
///
/// Mirrors NLTK `_leftmost_descendants`.
pub fn leftmost_descendants(root: &TreeValue, path: &[usize]) -> Vec<TreePath> {
    descendants(root, path)
        .into_iter()
        .filter(|p| p[path.len()..].iter().all(|&y| y == 0))
        .collect()
}

/// Descendants reachable via the rightmost child at every step.
///
/// Mirrors NLTK `_rightmost_descendants`.
pub fn rightmost_descendants(root: &TreeValue, path: &[usize]) -> Vec<TreePath> {
    let mut out = Vec::new();
    let mut cur_path = path.to_vec();
    loop {
        let Some(node) = node_at(root, &cur_path) else {
            break;
        };
        let Some(n) = node.as_node() else { break };
        if n.children().is_empty() {
            break;
        }
        cur_path.push(n.children().len() - 1);
        out.push(cur_path.clone());
    }
    out
}

/// Descendants traversed only while each parent has a single child.
///
/// Mirrors NLTK `_unique_descendants`.
pub fn unique_descendants(root: &TreeValue, path: &[usize]) -> Vec<TreePath> {
    let mut out = Vec::new();
    let mut cur_path = path.to_vec();
    loop {
        let Some(node) = node_at(root, &cur_path) else {
            break;
        };
        let Some(n) = node.as_node() else { break };
        if n.children().len() != 1 {
            break;
        }
        cur_path.push(0);
        out.push(cur_path.clone());
    }
    out
}

/// Paths of all nodes that linearly precede `path`.
///
/// Mirrors NLTK `_before`: a node `q` precedes `p` iff
/// `q[:len(p)] < p[:len(q)]` lexicographically.
pub fn before(root: &TreeValue, path: &[usize]) -> Vec<TreePath> {
    treepositions(root)
        .into_iter()
        .filter(|q| {
            let plen = path.len();
            let qlen = q.len();
            q[..plen.min(qlen)] < path[..qlen.min(plen)]
        })
        .collect()
}

/// Paths of all nodes that linearly follow `path`.
pub fn after(root: &TreeValue, path: &[usize]) -> Vec<TreePath> {
    treepositions(root)
        .into_iter()
        .filter(|q| {
            let plen = path.len();
            let qlen = q.len();
            q[..plen.min(qlen)] > path[..qlen.min(plen)]
        })
        .collect()
}

/// Paths of nodes immediately preceding `path` (last terminal of one
/// touches first terminal of the other).
///
/// Mirrors NLTK `_immediately_before`.
pub fn immediately_before(root: &TreeValue, path: &[usize]) -> Vec<TreePath> {
    if path.is_empty() {
        return Vec::new();
    }
    let mut idx = path.len() as isize - 1;
    while idx >= 0 && path[idx as usize] == 0 {
        idx -= 1;
    }
    if idx < 0 {
        return Vec::new();
    }
    let mut new_path: TreePath = path[..=idx as usize].to_vec();
    let last = new_path.last_mut().unwrap();
    *last -= 1;
    let mut out = vec![new_path.clone()];
    out.extend(rightmost_descendants(root, &new_path));
    out
}

/// Paths of nodes immediately following `path`.
///
/// Mirrors NLTK `_immediately_after`.
pub fn immediately_after(root: &TreeValue, path: &[usize]) -> Vec<TreePath> {
    if path.is_empty() {
        return Vec::new();
    }
    // Walk upwards while we are the last child at every level.
    let mut idx = path.len() as isize - 1;
    while idx >= 0 {
        let parent_path = &path[..idx as usize];
        let parent_len = node_at(root, parent_path)
            .and_then(as_node)
            .map(|n| n.children().len())
            .unwrap_or(0);
        if path[idx as usize] != parent_len.saturating_sub(1) {
            break;
        }
        idx -= 1;
    }
    if idx < 0 {
        return Vec::new();
    }
    let mut new_path: TreePath = path[..=idx as usize].to_vec();
    let last = new_path.last_mut().unwrap();
    *last += 1;
    let mut out = vec![new_path.clone()];
    out.extend(leftmost_descendants(root, &new_path));
    out
}

// ===========================================================================
// Predicate type and basic node matchers
// ===========================================================================

/// A predicate over a tree node identified by its [`MatchCtx`].
pub type Predicate = Arc<dyn for<'a> Fn(MatchCtx<'a>) -> bool + Send + Sync>;

/// Build a `Predicate` from a closure.
pub fn pred<F>(f: F) -> Predicate
where
    F: for<'a> Fn(MatchCtx<'a>) -> bool + Send + Sync + 'static,
{
    Arc::new(f)
}

/// Predicate matching any tree position.
///
/// Mirrors the TGrep2 wildcards `*` and `__`.
pub fn any_node() -> Predicate {
    pred(|_| true)
}

/// Predicate matching nodes whose literal value equals `literal`
/// (NLTK plain node-literal matcher).
pub fn label_eq(literal: impl Into<String>) -> Predicate {
    let literal = literal.into();
    pred(move |ctx| {
        ctx.node()
            .map(|n| node_literal_value(n) == literal)
            .unwrap_or(false)
    })
}

/// Predicate matching nodes whose literal value matches `regex`
/// (NLTK `/.../` form). The regex is searched anywhere in the
/// stringified value, matching NLTK's `re.search` semantics.
pub fn label_matches(regex: Regex) -> Predicate {
    let r = Arc::new(regex);
    pred(move |ctx| {
        ctx.node()
            .map(|n| r.is_match(&node_literal_value(n)))
            .unwrap_or(false)
    })
}

/// Disjunction of node-literal matchers (NLTK `A|B|C` form).
pub fn label_in<I, S>(literals: I) -> Predicate
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let lits: Vec<String> = literals.into_iter().map(Into::into).collect();
    pred(move |ctx| {
        ctx.node()
            .map(|n| {
                let v = node_literal_value(n);
                lits.iter().any(|l| *l == v)
            })
            .unwrap_or(false)
    })
}

// ===========================================================================
// Logical combinators
// ===========================================================================

/// Conjunction. NLTK `&` join.
pub fn and_(parts: Vec<Predicate>) -> Predicate {
    pred(move |ctx| parts.iter().all(|p| p(ctx)))
}

/// Disjunction. NLTK `|` join.
pub fn or_(parts: Vec<Predicate>) -> Predicate {
    pred(move |ctx| parts.iter().any(|p| p(ctx)))
}

/// Negation. NLTK `!` prefix.
pub fn not_(inner: Predicate) -> Predicate {
    pred(move |ctx| !inner(ctx))
}

// ===========================================================================
// Relation combinators (TGrep2 operator table)
// ===========================================================================

fn ctx_for<'a>(root: &'a TreeValue, path: &'a [usize]) -> MatchCtx<'a> {
    MatchCtx { root, path }
}

/// `A < B`: A is the parent of B.
pub fn parent_of(child: Predicate) -> Predicate {
    pred(move |ctx| {
        let Some(node) = ctx.node().and_then(as_node) else {
            return false;
        };
        for i in 0..node.children().len() {
            let mut cp: TreePath = ctx.path.to_vec();
            cp.push(i);
            if child(ctx_for(ctx.root, &cp)) {
                return true;
            }
        }
        false
    })
}

/// `A > B`: A is the child of B.
pub fn child_of(parent: Predicate) -> Predicate {
    pred(move |ctx| match parent_path(ctx.path) {
        Some(p) => parent(ctx_for(ctx.root, &p)),
        None => false,
    })
}

/// `A <N B`: B is the (1-indexed) Nth child of A.
pub fn nth_child(n: usize, child: Predicate) -> Predicate {
    pred(move |ctx| {
        let Some(node) = ctx.node().and_then(as_node) else {
            return false;
        };
        if n == 0 || n > node.children().len() {
            return false;
        }
        let mut cp: TreePath = ctx.path.to_vec();
        cp.push(n - 1);
        child(ctx_for(ctx.root, &cp))
    })
}

/// `A >N B`: A is the Nth child of B (1-indexed).
pub fn is_nth_child_of(n: usize, parent: Predicate) -> Predicate {
    pred(move |ctx| {
        let Some(parent_p) = parent_path(ctx.path) else {
            return false;
        };
        if ctx.path.last().copied() != Some(n.saturating_sub(1)) {
            return false;
        }
        parent(ctx_for(ctx.root, &parent_p))
    })
}

/// `A <-N B`: B is the Nth-to-last child of A (1-indexed; `<-1` is
/// the last child).
pub fn nth_last_child(n: usize, child: Predicate) -> Predicate {
    pred(move |ctx| {
        let Some(node) = ctx.node().and_then(as_node) else {
            return false;
        };
        let len = node.children().len();
        if n == 0 || n > len {
            return false;
        }
        let mut cp: TreePath = ctx.path.to_vec();
        cp.push(len - n);
        child(ctx_for(ctx.root, &cp))
    })
}

/// `A >-N B`: A is the Nth-to-last child of B.
pub fn is_nth_last_child_of(n: usize, parent: Predicate) -> Predicate {
    pred(move |ctx| {
        let Some(parent_p) = parent_path(ctx.path) else {
            return false;
        };
        let plen = node_at(ctx.root, &parent_p)
            .and_then(as_node)
            .map(|n| n.children().len())
            .unwrap_or(0);
        if n == 0 || n > plen {
            return false;
        }
        if ctx.path.last().copied() != Some(plen - n) {
            return false;
        }
        parent(ctx_for(ctx.root, &parent_p))
    })
}

/// `A <: B`: B is the only child of A.
pub fn only_child(child: Predicate) -> Predicate {
    pred(move |ctx| {
        let Some(node) = ctx.node().and_then(as_node) else {
            return false;
        };
        if node.children().len() != 1 {
            return false;
        }
        let mut cp: TreePath = ctx.path.to_vec();
        cp.push(0);
        child(ctx_for(ctx.root, &cp))
    })
}

/// `A >: B`: A is the only child of B.
pub fn is_only_child_of(parent: Predicate) -> Predicate {
    pred(move |ctx| {
        let Some(parent_p) = parent_path(ctx.path) else {
            return false;
        };
        let len = node_at(ctx.root, &parent_p)
            .and_then(as_node)
            .map(|n| n.children().len())
            .unwrap_or(0);
        if len != 1 {
            return false;
        }
        parent(ctx_for(ctx.root, &parent_p))
    })
}

/// `A << B`: A dominates B (A is an ancestor of B).
pub fn dominates(descendant: Predicate) -> Predicate {
    pred(move |ctx| {
        for d in descendants(ctx.root, ctx.path) {
            if descendant(ctx_for(ctx.root, &d)) {
                return true;
            }
        }
        false
    })
}

/// `A >> B`: A is dominated by B (B is an ancestor of A).
pub fn dominated_by(ancestor: Predicate) -> Predicate {
    pred(move |ctx| {
        for a in ancestors(ctx.path) {
            if ancestor(ctx_for(ctx.root, &a)) {
                return true;
            }
        }
        false
    })
}

/// `A <<, B`: B is a left-most descendant of A.
pub fn dominates_leftmost(descendant: Predicate) -> Predicate {
    pred(move |ctx| {
        for d in leftmost_descendants(ctx.root, ctx.path) {
            if descendant(ctx_for(ctx.root, &d)) {
                return true;
            }
        }
        false
    })
}

/// `A <<' B`: B is a right-most descendant of A.
pub fn dominates_rightmost(descendant: Predicate) -> Predicate {
    pred(move |ctx| {
        for d in rightmost_descendants(ctx.root, ctx.path) {
            if descendant(ctx_for(ctx.root, &d)) {
                return true;
            }
        }
        false
    })
}

/// `A <<: B`: there is a single path of descent from A and B is on it.
pub fn dominates_unique(descendant: Predicate) -> Predicate {
    pred(move |ctx| {
        for d in unique_descendants(ctx.root, ctx.path) {
            if descendant(ctx_for(ctx.root, &d)) {
                return true;
            }
        }
        false
    })
}

/// `A >>: B`: there is a single path of descent from B and A is on it.
pub fn dominated_by_unique(ancestor: Predicate) -> Predicate {
    pred(move |ctx| {
        for a in unique_ancestors(ctx.root, ctx.path) {
            if ancestor(ctx_for(ctx.root, &a)) {
                return true;
            }
        }
        false
    })
}

/// `A . B`: A immediately precedes B.
pub fn precedes_immediately(other: Predicate) -> Predicate {
    pred(move |ctx| {
        for q in immediately_after(ctx.root, ctx.path) {
            if other(ctx_for(ctx.root, &q)) {
                return true;
            }
        }
        false
    })
}

/// `A , B`: A immediately follows B.
pub fn follows_immediately(other: Predicate) -> Predicate {
    pred(move |ctx| {
        for q in immediately_before(ctx.root, ctx.path) {
            if other(ctx_for(ctx.root, &q)) {
                return true;
            }
        }
        false
    })
}

/// `A .. B`: A precedes B.
pub fn precedes(other: Predicate) -> Predicate {
    pred(move |ctx| {
        for q in after(ctx.root, ctx.path) {
            if other(ctx_for(ctx.root, &q)) {
                return true;
            }
        }
        false
    })
}

/// `A ,, B`: A follows B.
pub fn follows(other: Predicate) -> Predicate {
    pred(move |ctx| {
        for q in before(ctx.root, ctx.path) {
            if other(ctx_for(ctx.root, &q)) {
                return true;
            }
        }
        false
    })
}

/// `A $ B`: A is a sister of B (same parent, distinct child).
pub fn sister_of(other: Predicate) -> Predicate {
    pred(move |ctx| {
        let Some(parent_p) = parent_path(ctx.path) else {
            return false;
        };
        let Some(parent_node) = node_at(ctx.root, &parent_p).and_then(as_node) else {
            return false;
        };
        let my_idx = ctx.path.last().copied().unwrap_or(0);
        for i in 0..parent_node.children().len() {
            if i == my_idx {
                continue;
            }
            let mut sp: TreePath = parent_p.clone();
            sp.push(i);
            if other(ctx_for(ctx.root, &sp)) {
                return true;
            }
        }
        false
    })
}

/// `A $. B`: A is the immediate left sister of B (right sibling).
pub fn right_sibling_is(other: Predicate) -> Predicate {
    pred(move |ctx| {
        let Some(parent_p) = parent_path(ctx.path) else {
            return false;
        };
        let len = node_at(ctx.root, &parent_p)
            .and_then(as_node)
            .map(|n| n.children().len())
            .unwrap_or(0);
        let my_idx = ctx.path.last().copied().unwrap_or(0);
        if my_idx + 1 >= len {
            return false;
        }
        let mut sp: TreePath = parent_p.clone();
        sp.push(my_idx + 1);
        other(ctx_for(ctx.root, &sp))
    })
}

/// `A $, B`: A is the immediate right sister of B (left sibling).
pub fn left_sibling_is(other: Predicate) -> Predicate {
    pred(move |ctx| {
        let Some(parent_p) = parent_path(ctx.path) else {
            return false;
        };
        let my_idx = ctx.path.last().copied().unwrap_or(0);
        if my_idx == 0 {
            return false;
        }
        let mut sp: TreePath = parent_p.clone();
        sp.push(my_idx - 1);
        other(ctx_for(ctx.root, &sp))
    })
}

/// `A $.. B`: A is a sister of and precedes B.
pub fn sister_precedes(other: Predicate) -> Predicate {
    pred(move |ctx| {
        let Some(parent_p) = parent_path(ctx.path) else {
            return false;
        };
        let Some(parent_node) = node_at(ctx.root, &parent_p).and_then(as_node) else {
            return false;
        };
        let my_idx = ctx.path.last().copied().unwrap_or(0);
        for i in (my_idx + 1)..parent_node.children().len() {
            let mut sp: TreePath = parent_p.clone();
            sp.push(i);
            if other(ctx_for(ctx.root, &sp)) {
                return true;
            }
        }
        false
    })
}

/// `A $,, B`: A is a sister of and follows B.
pub fn sister_follows(other: Predicate) -> Predicate {
    pred(move |ctx| {
        let Some(parent_p) = parent_path(ctx.path) else {
            return false;
        };
        let my_idx = ctx.path.last().copied().unwrap_or(0);
        for i in 0..my_idx {
            let mut sp: TreePath = parent_p.clone();
            sp.push(i);
            if other(ctx_for(ctx.root, &sp)) {
                return true;
            }
        }
        false
    })
}

// ===========================================================================
// Drivers
// ===========================================================================

/// All paths in `tree` matching `predicate`, in preorder.
///
/// Mirrors NLTK `tgrep_positions(pattern, [tree])` for a single tree.
/// When `search_leaves` is false, leaf positions are skipped.
pub fn tgrep_positions(
    predicate: &Predicate,
    tree: &TreeValue,
    search_leaves: bool,
) -> Vec<TreePath> {
    let mut out = Vec::new();
    for path in treepositions(tree) {
        let node = match node_at(tree, &path) {
            Some(n) => n,
            None => continue,
        };
        if !search_leaves && node.is_leaf() {
            continue;
        }
        if predicate(MatchCtx::new(tree, &path)) {
            out.push(path);
        }
    }
    out
}

/// All matching nodes in `tree`, in preorder, returned as cloned values.
pub fn tgrep_nodes(predicate: &Predicate, tree: &TreeValue, search_leaves: bool) -> Vec<TreeValue> {
    tgrep_positions(predicate, tree, search_leaves)
        .into_iter()
        .filter_map(|p| node_at(tree, &p).cloned())
        .collect()
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn leaf(s: &str) -> TreeValue {
        TreeValue::leaf(TreeLeafValue::Text(s.to_string()))
    }

    fn node(label: &str, children: Vec<TreeValue>) -> TreeValue {
        TreeValue::node(label, children)
    }

    /// (S (NP (DT the) (JJ big) (NN dog)) (VP bit) (NP (DT a) (NN cat)))
    fn doc_tree() -> TreeValue {
        node(
            "S",
            vec![
                node(
                    "NP",
                    vec![
                        node("DT", vec![leaf("the")]),
                        node("JJ", vec![leaf("big")]),
                        node("NN", vec![leaf("dog")]),
                    ],
                ),
                node("VP", vec![leaf("bit")]),
                node(
                    "NP",
                    vec![node("DT", vec![leaf("a")]), node("NN", vec![leaf("cat")])],
                ),
            ],
        )
    }

    fn labels_at(tree: &TreeValue, paths: &[TreePath]) -> Vec<String> {
        paths
            .iter()
            .map(|p| node_literal_value(node_at(tree, p).unwrap()))
            .collect()
    }

    // ---- Positional helpers ----

    #[test]
    fn ancestors_of_deep_path() {
        // path of "the": (0, 0, 0)
        let anc = ancestors(&[0, 0, 0]);
        assert_eq!(anc, vec![vec![0, 0], vec![0], vec![]]);
    }

    #[test]
    fn descendants_of_subtree() {
        let t = doc_tree();
        let mut out: Vec<String> = descendants(&t, &[0])
            .iter()
            .map(|p| node_literal_value(node_at(&t, p).unwrap()))
            .collect();
        out.sort();
        let mut expected = vec!["DT", "JJ", "NN", "the", "big", "dog"];
        expected.sort();
        assert_eq!(out, expected);
    }

    #[test]
    fn rightmost_descendants_chain() {
        let t = doc_tree();
        let r = rightmost_descendants(&t, &[]);
        // S -> NP(2) -> NN -> "cat"
        assert_eq!(r, vec![vec![2], vec![2, 1], vec![2, 1, 0]]);
    }

    #[test]
    fn unique_descendants_only_one_child() {
        let t = doc_tree();
        // VP at (1) has a single Leaf child, so unique_descendants from
        // [1] yields one step then stops at the leaf.
        let u = unique_descendants(&t, &[1]);
        assert_eq!(u, vec![vec![1, 0]]);
    }

    #[test]
    fn before_after_root_relations() {
        let t = doc_tree();
        // "bit" at (1, 0) — preceded by everything in NP[0], followed
        // by everything in NP[2].
        let bf: Vec<String> = before(&t, &[1, 0])
            .iter()
            .map(|p| node_literal_value(node_at(&t, p).unwrap()))
            .collect();
        assert!(bf.contains(&"the".to_string()));
        assert!(bf.contains(&"NP".to_string()));
        let af: Vec<String> = after(&t, &[1, 0])
            .iter()
            .map(|p| node_literal_value(node_at(&t, p).unwrap()))
            .collect();
        assert!(af.contains(&"cat".to_string()));
    }

    #[test]
    fn immediately_after_steps_right() {
        let t = doc_tree();
        // "dog" at (0, 2, 0). Its immediately-following nodes start at
        // VP at (1), then walk leftmost descendants -> "bit".
        let after = immediately_after(&t, &[0, 2, 0]);
        let labels: Vec<String> = after
            .iter()
            .map(|p| node_literal_value(node_at(&t, p).unwrap()))
            .collect();
        assert_eq!(labels[0], "VP");
        assert!(labels.contains(&"bit".to_string()));
    }

    #[test]
    fn immediately_before_steps_left() {
        let t = doc_tree();
        // "a" at (2, 0, 0). Immediately preceding: VP at (1), then its
        // rightmost descendants -> "bit".
        let before = immediately_before(&t, &[2, 0, 0]);
        let labels: Vec<String> = before
            .iter()
            .map(|p| node_literal_value(node_at(&t, p).unwrap()))
            .collect();
        assert_eq!(labels[0], "VP");
        assert!(labels.contains(&"bit".to_string()));
    }

    // ---- Matchers and drivers (NLTK doctest parity) ----

    #[test]
    fn nn_doctest() {
        let t = doc_tree();
        let p = label_eq("NN");
        let positions = tgrep_positions(&p, &t, true);
        assert_eq!(positions, vec![vec![0, 2], vec![2, 1]]);
        let nodes = tgrep_nodes(&p, &t, true);
        assert_eq!(nodes.len(), 2);
        assert_eq!(nodes[0].as_node().unwrap().label(), "NN");
    }

    #[test]
    fn dt_doctest() {
        let t = doc_tree();
        let p = label_eq("DT");
        let positions = tgrep_positions(&p, &t, true);
        assert_eq!(positions, vec![vec![0, 0], vec![2, 0]]);
    }

    #[test]
    fn dt_sister_jj_doctest() {
        // `DT $ JJ` — a DT that has a sister JJ.
        let t = doc_tree();
        let p = and_(vec![label_eq("DT"), sister_of(label_eq("JJ"))]);
        let positions = tgrep_positions(&p, &t, true);
        assert_eq!(positions, vec![vec![0, 0]]);
    }

    // ---- Relation parity ----

    #[test]
    fn parent_of_picks_dominator() {
        let t = doc_tree();
        // NP < JJ — first NP only
        let p = and_(vec![label_eq("NP"), parent_of(label_eq("JJ"))]);
        let positions = tgrep_positions(&p, &t, true);
        assert_eq!(positions, vec![vec![0]]);
    }

    #[test]
    fn dominates_finds_deep_descendants() {
        let t = doc_tree();
        // S << "cat"
        let p = and_(vec![label_eq("S"), dominates(label_eq("cat"))]);
        let positions = tgrep_positions(&p, &t, true);
        let expected: Vec<TreePath> = vec![vec![]];
        assert_eq!(positions, expected);
    }

    #[test]
    fn dominated_by_finds_ancestor() {
        let t = doc_tree();
        // NN >> NP — every NN whose ancestor includes NP
        let p = and_(vec![label_eq("NN"), dominated_by(label_eq("NP"))]);
        let positions = tgrep_positions(&p, &t, true);
        assert_eq!(positions, vec![vec![0, 2], vec![2, 1]]);
    }

    #[test]
    fn nth_child_first() {
        let t = doc_tree();
        // S <1 NP — S whose first child is NP (true)
        let p = and_(vec![label_eq("S"), nth_child(1, label_eq("NP"))]);
        let positions = tgrep_positions(&p, &t, true);
        let expected: Vec<TreePath> = vec![vec![]];
        assert_eq!(positions, expected);
    }

    #[test]
    fn nth_last_child_last() {
        let t = doc_tree();
        // S <-1 NP — S whose last child is NP (true; index 2)
        let p = and_(vec![label_eq("S"), nth_last_child(1, label_eq("NP"))]);
        let positions = tgrep_positions(&p, &t, true);
        let expected: Vec<TreePath> = vec![vec![]];
        assert_eq!(positions, expected);
    }

    #[test]
    fn only_child_check() {
        let t = doc_tree();
        // VP at (1) has only "bit" as a leaf child → VP <: "bit"
        let p = and_(vec![label_eq("VP"), only_child(label_eq("bit"))]);
        let positions = tgrep_positions(&p, &t, true);
        assert_eq!(positions, vec![vec![1]]);
    }

    #[test]
    fn precedes_immediately_link() {
        let t = doc_tree();
        // (0, 1, 0) "big" . "dog"
        let p = and_(vec![label_eq("big"), precedes_immediately(label_eq("dog"))]);
        let positions = tgrep_positions(&p, &t, true);
        assert_eq!(positions, vec![vec![0, 1, 0]]);
    }

    #[test]
    fn follows_immediately_link() {
        let t = doc_tree();
        // "dog" , "big"
        let p = and_(vec![label_eq("dog"), follows_immediately(label_eq("big"))]);
        let positions = tgrep_positions(&p, &t, true);
        assert_eq!(positions, vec![vec![0, 2, 0]]);
    }

    #[test]
    fn precedes_link_finds_following_subtree() {
        let t = doc_tree();
        // NP .. NP — first NP precedes second NP
        let p = and_(vec![label_eq("NP"), precedes(label_eq("NP"))]);
        let positions = tgrep_positions(&p, &t, true);
        assert_eq!(positions, vec![vec![0]]);
    }

    #[test]
    fn follows_link_finds_preceding_subtree() {
        let t = doc_tree();
        // second NP ,, first NP
        let p = and_(vec![label_eq("NP"), follows(label_eq("NP"))]);
        let positions = tgrep_positions(&p, &t, true);
        assert_eq!(positions, vec![vec![2]]);
    }

    #[test]
    fn right_left_sibling_links() {
        let t = doc_tree();
        // DT $. JJ — DT whose right sibling is JJ
        let p = and_(vec![label_eq("DT"), right_sibling_is(label_eq("JJ"))]);
        assert_eq!(tgrep_positions(&p, &t, true), vec![vec![0, 0]]);
        // JJ $, DT — JJ whose left sibling is DT
        let p = and_(vec![label_eq("JJ"), left_sibling_is(label_eq("DT"))]);
        assert_eq!(tgrep_positions(&p, &t, true), vec![vec![0, 1]]);
    }

    #[test]
    fn sister_precedes_and_follows() {
        let t = doc_tree();
        // DT $.. NN — DT with a following sister NN (first NP only)
        let p = and_(vec![label_eq("DT"), sister_precedes(label_eq("NN"))]);
        assert_eq!(tgrep_positions(&p, &t, true), vec![vec![0, 0], vec![2, 0]]);
        // NN $,, DT — NN with a preceding sister DT
        let p = and_(vec![label_eq("NN"), sister_follows(label_eq("DT"))]);
        assert_eq!(tgrep_positions(&p, &t, true), vec![vec![0, 2], vec![2, 1]]);
    }

    // ---- Logical combinators ----

    #[test]
    fn or_and_not_combine() {
        let t = doc_tree();
        // (NN | DT) & !> NP — neither NN nor DT exists outside NP, so empty.
        let p = and_(vec![
            or_(vec![label_eq("NN"), label_eq("DT")]),
            not_(child_of(label_eq("NP"))),
        ]);
        assert!(tgrep_positions(&p, &t, true).is_empty());
        // NN | DT alone returns four positions.
        let p = or_(vec![label_eq("NN"), label_eq("DT")]);
        let positions = tgrep_positions(&p, &t, true);
        assert_eq!(labels_at(&t, &positions), vec!["DT", "NN", "DT", "NN"]);
    }

    // ---- Regex matcher ----

    #[test]
    fn label_matches_with_regex() {
        let t = doc_tree();
        let p = label_matches(Regex::new("^N").unwrap()); // NP and NN
        let positions = tgrep_positions(&p, &t, false);
        let labels = labels_at(&t, &positions);
        assert_eq!(labels, vec!["NP", "NN", "NP", "NN"]);
    }

    #[test]
    fn search_leaves_flag_excludes_strings() {
        let t = doc_tree();
        let p = any_node();
        let with_leaves = tgrep_positions(&p, &t, true).len();
        let no_leaves = tgrep_positions(&p, &t, false).len();
        assert!(with_leaves > no_leaves);
    }
}
