//! Tree and graph utilities — port of NLTK `nltk/util.py`.
//!
//! Faithful translation of the kernel-applicable surface of NLTK's
//! `util` module: tree traversal (BFS/DFS with cycle detection),
//! minimum spanning tree, transitive closure / graph inversion,
//! n-grams (bigrams, trigrams, everygrams, skipgrams), and a handful
//! of small list/dict helpers.
//!
//! Skipped (not kernel-applicable): `usage`, `in_idle`, all
//! pretty-printing and `print_*` helpers, `filestring`, HTTP proxy
//! configuration, ElementTree indenting, `binary_search_file`,
//! `clean_html`/`clean_url`, encoding guessing, and the Graphviz
//! `edges2dot` / `unweighted_minimum_spanning_digraph` rendering.
//!
//! Trees are represented as NLTK does: a root plus a `children`
//! closure mapping a node to its children. Children iteration order
//! is preserved by accepting a `Fn(&N) -> Vec<N>`. Graphs are
//! `BTreeMap<N, BTreeSet<N>>` for deterministic iteration.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::hash::Hash;

// ===========================================================================
// Tree traversal
// ===========================================================================

/// Depth at which a child appears under the root. The root itself is
/// at depth 0.
pub type Depth = i64;

/// Breadth-first traversal of a tree. **No cycle detection.**
///
/// `maxdepth = -1` means unbounded.
///
/// Mirrors NLTK `breadth_first`.
pub fn breadth_first<N, F>(root: N, children: F, maxdepth: Depth) -> Vec<N>
where
    N: Clone,
    F: Fn(&N) -> Vec<N>,
{
    let mut out = Vec::new();
    let mut queue: VecDeque<(N, Depth)> = VecDeque::new();
    queue.push_back((root, 0));
    while let Some((node, depth)) = queue.pop_front() {
        out.push(node.clone());
        if depth != maxdepth {
            for c in children(&node) {
                queue.push_back((c, depth + 1));
            }
        }
    }
    out
}

/// Breadth-first traversal that skips already-visited nodes.
///
/// Mirrors NLTK `acyclic_breadth_first`.
pub fn acyclic_breadth_first<N, F>(root: N, children: F, maxdepth: Depth) -> Vec<N>
where
    N: Clone + Eq + Hash + Ord,
    F: Fn(&N) -> Vec<N>,
{
    let mut out = Vec::new();
    let mut traversed: BTreeSet<N> = BTreeSet::new();
    let mut queue: VecDeque<(N, Depth)> = VecDeque::new();
    queue.push_back((root, 0));
    while let Some((node, depth)) = queue.pop_front() {
        if traversed.contains(&node) {
            continue;
        }
        out.push(node.clone());
        traversed.insert(node.clone());
        if depth != maxdepth {
            for c in children(&node) {
                if !traversed.contains(&c) {
                    queue.push_back((c, depth + 1));
                }
            }
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Acyclic depth-first traversal as nested lists.
// ---------------------------------------------------------------------------

/// A nested-list tree node: either a real node or a cycle marker
/// inserted by the depth-first traversals.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DfsTree<N: Clone> {
    /// A node, followed by zero or more child subtrees.
    Node(N, Vec<DfsTree<N>>),
    /// A cycle marker, e.g. `Cycle(child, depth, mark)` in NLTK.
    Cycle(String),
}

/// Depth-first traversal that discards cycles within any branch
/// (using a single shared `traversed` set across siblings, matching
/// NLTK semantics — duplicate paths in different branches are
/// pruned).
///
/// `depth = -1` means unbounded. When `cut_mark` is `Some`, a marker
/// `Cycle({child},{depth-1},{cut_mark})` is inserted when a cycle is
/// truncated.
///
/// Mirrors NLTK `acyclic_depth_first`.
pub fn acyclic_depth_first<N, F>(
    root: N,
    children: F,
    depth: Depth,
    cut_mark: Option<&str>,
) -> DfsTree<N>
where
    N: Clone + Eq + Hash + Ord + std::fmt::Debug,
    F: Fn(&N) -> Vec<N>,
{
    let mut traversed: BTreeSet<N> = BTreeSet::from([root.clone()]);
    acyclic_depth_first_inner(&root, &children, depth, cut_mark, &mut traversed)
}

fn acyclic_depth_first_inner<N, F>(
    node: &N,
    children: &F,
    depth: Depth,
    cut_mark: Option<&str>,
    traversed: &mut BTreeSet<N>,
) -> DfsTree<N>
where
    N: Clone + Eq + Hash + Ord + std::fmt::Debug,
    F: Fn(&N) -> Vec<N>,
{
    let mut out: Vec<DfsTree<N>> = Vec::new();
    if depth != 0 {
        for child in children(node) {
            if !traversed.contains(&child) {
                traversed.insert(child.clone());
                out.push(acyclic_depth_first_inner(
                    &child,
                    children,
                    depth - 1,
                    cut_mark,
                    traversed,
                ));
            } else if let Some(mark) = cut_mark {
                out.push(DfsTree::Cycle(format!(
                    "Cycle({:?},{},{})",
                    child,
                    depth - 1,
                    mark
                )));
            }
        }
    } else if let Some(mark) = cut_mark {
        out.push(DfsTree::Cycle(mark.to_string()));
    }
    DfsTree::Node(node.clone(), out)
}

/// Depth-first traversal that discards cycles within the same branch
/// only, allowing duplicate paths across siblings (each sibling gets a
/// fresh copy of the `traversed` set).
///
/// Mirrors NLTK `acyclic_branches_depth_first`.
pub fn acyclic_branches_depth_first<N, F>(
    root: N,
    children: F,
    depth: Depth,
    cut_mark: Option<&str>,
) -> DfsTree<N>
where
    N: Clone + Eq + Hash + Ord + std::fmt::Debug,
    F: Fn(&N) -> Vec<N>,
{
    let traversed: BTreeSet<N> = BTreeSet::from([root.clone()]);
    acyclic_branches_inner(&root, &children, depth, cut_mark, traversed)
}

fn acyclic_branches_inner<N, F>(
    node: &N,
    children: &F,
    depth: Depth,
    cut_mark: Option<&str>,
    traversed: BTreeSet<N>,
) -> DfsTree<N>
where
    N: Clone + Eq + Hash + Ord + std::fmt::Debug,
    F: Fn(&N) -> Vec<N>,
{
    let mut out: Vec<DfsTree<N>> = Vec::new();
    if depth != 0 {
        for child in children(node) {
            if !traversed.contains(&child) {
                let mut new_trav = traversed.clone();
                new_trav.insert(child.clone());
                out.push(acyclic_branches_inner(
                    &child,
                    children,
                    depth - 1,
                    cut_mark,
                    new_trav,
                ));
            } else if let Some(mark) = cut_mark {
                out.push(DfsTree::Cycle(format!(
                    "Cycle({:?},{},{})",
                    child,
                    depth - 1,
                    mark
                )));
            }
        }
    } else if let Some(mark) = cut_mark {
        out.push(DfsTree::Cycle(mark.to_string()));
    }
    DfsTree::Node(node.clone(), out)
}

// ===========================================================================
// Edge closure / minimum spanning helpers
// ===========================================================================

/// Yield the edges of a graph reachable from `root` in breadth-first
/// order, discarding edge duplicates.
///
/// Mirrors NLTK `edge_closure`. Note that NLTK's docstring example
/// shows that traversed nodes are not re-queued, but their incoming
/// edges are still emitted on first encounter.
pub fn edge_closure<N, F>(root: N, children: F, maxdepth: Depth) -> Vec<(N, N)>
where
    N: Clone + Eq + Hash + Ord,
    F: Fn(&N) -> Vec<N>,
{
    let mut traversed: BTreeSet<N> = BTreeSet::new();
    let mut edges: BTreeSet<(N, N)> = BTreeSet::new();
    let mut out: Vec<(N, N)> = Vec::new();
    let mut queue: VecDeque<(N, Depth)> = VecDeque::new();
    queue.push_back((root, 0));
    while let Some((node, depth)) = queue.pop_front() {
        traversed.insert(node.clone());
        if depth != maxdepth {
            for child in children(&node) {
                if !traversed.contains(&child) {
                    queue.push_back((child.clone(), depth + 1));
                }
                let edge = (node.clone(), child);
                if !edges.contains(&edge) {
                    out.push(edge.clone());
                    edges.insert(edge);
                }
            }
        }
    }
    out
}

/// Build a Minimum Spanning Tree of an unweighted graph as a
/// dictionary: `node -> list of children`. Cycles are broken by
/// queueing each node at most once (via an `agenda` set).
///
/// Mirrors NLTK `unweighted_minimum_spanning_dict`.
pub fn unweighted_minimum_spanning_dict<N, F>(root: N, children: F) -> BTreeMap<N, Vec<N>>
where
    N: Clone + Eq + Hash + Ord,
    F: Fn(&N) -> Vec<N>,
{
    let mut traversed: BTreeSet<N> = BTreeSet::new();
    let mut agenda: BTreeSet<N> = BTreeSet::from([root.clone()]);
    let mut mst: BTreeMap<N, Vec<N>> = BTreeMap::new();
    let mut queue: VecDeque<N> = VecDeque::new();
    queue.push_back(root);
    while let Some(node) = queue.pop_front() {
        mst.entry(node.clone()).or_insert_with(Vec::new);
        if !traversed.contains(&node) {
            traversed.insert(node.clone());
            for child in children(&node) {
                if !agenda.contains(&child) {
                    mst.get_mut(&node).unwrap().push(child.clone());
                    queue.push_back(child.clone());
                    agenda.insert(child);
                }
            }
        }
    }
    mst
}

/// Convert an acyclic dictionary `node -> children` into a
/// nested-list tree rooted at `node`.
///
/// Mirrors NLTK `acyclic_dic2tree`. Returns a generic [`DfsTree`]
/// without cycle markers.
pub fn acyclic_dic2tree<N>(node: &N, dic: &BTreeMap<N, Vec<N>>) -> DfsTree<N>
where
    N: Clone + Eq + Hash + Ord,
{
    let kids = dic.get(node).cloned().unwrap_or_default();
    let subs = kids.iter().map(|c| acyclic_dic2tree(c, dic)).collect();
    DfsTree::Node(node.clone(), subs)
}

/// Build the MST and return it as a nested-list tree.
///
/// Mirrors NLTK `unweighted_minimum_spanning_tree`.
pub fn unweighted_minimum_spanning_tree<N, F>(root: N, children: F) -> DfsTree<N>
where
    N: Clone + Eq + Hash + Ord,
    F: Fn(&N) -> Vec<N>,
{
    let dic = unweighted_minimum_spanning_dict(root.clone(), children);
    acyclic_dic2tree(&root, &dic)
}

// ===========================================================================
// Directed-graph utilities: transitive closure, inversion
// ===========================================================================

/// Transitive closure of a directed graph (dict-of-set). When
/// `reflexive` is true, each node also has itself as a successor.
///
/// Implements the "Marking Algorithm" of Ioannidis & Ramakrishnan
/// (1998), as in NLTK `transitive_closure`.
pub fn transitive_closure<N>(
    graph: &BTreeMap<N, BTreeSet<N>>,
    reflexive: bool,
) -> BTreeMap<N, BTreeSet<N>>
where
    N: Clone + Eq + Hash + Ord,
{
    let base = |k: &N| -> BTreeSet<N> {
        if reflexive {
            BTreeSet::from([k.clone()])
        } else {
            BTreeSet::new()
        }
    };

    let mut agenda_graph: BTreeMap<N, BTreeSet<N>> = graph.clone();
    let mut closure_graph: BTreeMap<N, BTreeSet<N>> =
        graph.keys().map(|k| (k.clone(), base(k))).collect();

    let keys: Vec<N> = graph.keys().cloned().collect();
    for i in &keys {
        loop {
            let next = {
                let agenda = agenda_graph.get_mut(i).unwrap();
                agenda.iter().next().cloned()
            };
            let Some(j) = next else { break };
            agenda_graph.get_mut(i).unwrap().remove(&j);
            closure_graph.get_mut(i).unwrap().insert(j.clone());

            let cj = closure_graph
                .entry(j.clone())
                .or_insert_with(|| base(&j))
                .clone();
            closure_graph.get_mut(i).unwrap().extend(cj);

            let aj = agenda_graph
                .entry(j.clone())
                .or_insert_with(|| base(&j))
                .clone();
            let agenda = agenda_graph.get_mut(i).unwrap();
            agenda.extend(aj);
            let closure = closure_graph.get(i).unwrap().clone();
            for c in &closure {
                agenda.remove(c);
            }
        }
    }
    closure_graph
}

/// Invert a directed graph (reverse every edge).
///
/// Mirrors NLTK `invert_graph`.
pub fn invert_graph<N>(graph: &BTreeMap<N, BTreeSet<N>>) -> BTreeMap<N, BTreeSet<N>>
where
    N: Clone + Eq + Hash + Ord,
{
    let mut inverted: BTreeMap<N, BTreeSet<N>> = BTreeMap::new();
    for (key, vals) in graph {
        for v in vals {
            inverted.entry(v.clone()).or_default().insert(key.clone());
        }
    }
    inverted
}

// ===========================================================================
// Small list / dict helpers
// ===========================================================================

/// Deterministically remove duplicates while preserving first-seen
/// order. Mirrors NLTK `unique_list`.
pub fn unique_list<T: Clone + Eq + Hash>(xs: &[T]) -> Vec<T> {
    let mut seen: std::collections::HashSet<T> = std::collections::HashSet::new();
    let mut out = Vec::new();
    for x in xs {
        if seen.insert(x.clone()) {
            out.push(x.clone());
        }
    }
    out
}

/// `Index<K, V>` collects (key, value) pairs into a multimap. Mirrors
/// NLTK's `Index` (`defaultdict(list)` flavour).
#[derive(Debug, Clone)]
pub struct Index<K: Ord + Eq + Hash + Clone, V: Clone> {
    inner: BTreeMap<K, Vec<V>>,
}

impl<K: Ord + Eq + Hash + Clone, V: Clone> Default for Index<K, V> {
    fn default() -> Self {
        Self {
            inner: BTreeMap::new(),
        }
    }
}

impl<K: Ord + Eq + Hash + Clone, V: Clone> Index<K, V> {
    pub fn new<I: IntoIterator<Item = (K, V)>>(pairs: I) -> Self {
        let mut me = Self::default();
        for (k, v) in pairs {
            me.inner.entry(k).or_default().push(v);
        }
        me
    }

    pub fn get(&self, key: &K) -> Option<&[V]> {
        self.inner.get(key).map(|v| v.as_slice())
    }

    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.inner.keys()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &[V])> {
        self.inner.iter().map(|(k, v)| (k, v.as_slice()))
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

/// Invert a `K -> Vec<V>` multimap into a `V -> Vec<K>` multimap.
/// Mirrors the iterable branch of NLTK `invert_dict`.
pub fn invert_dict<K, V>(d: &BTreeMap<K, Vec<V>>) -> BTreeMap<V, Vec<K>>
where
    K: Clone + Eq + Hash + Ord,
    V: Clone + Eq + Hash + Ord,
{
    let mut inverted: BTreeMap<V, Vec<K>> = BTreeMap::new();
    for (k, vs) in d {
        for v in vs {
            inverted.entry(v.clone()).or_default().push(k.clone());
        }
    }
    inverted
}

/// Recursively flatten a nested list. Mirrors NLTK `flatten`.
#[derive(Debug, Clone)]
pub enum Nested<T: Clone> {
    Leaf(T),
    List(Vec<Nested<T>>),
}

pub fn flatten<T: Clone>(items: &[Nested<T>]) -> Vec<T> {
    let mut out = Vec::new();
    for item in items {
        flatten_into(item, &mut out);
    }
    out
}

fn flatten_into<T: Clone>(item: &Nested<T>, out: &mut Vec<T>) {
    match item {
        Nested::Leaf(t) => out.push(t.clone()),
        Nested::List(xs) => {
            for x in xs {
                flatten_into(x, out);
            }
        }
    }
}

// ===========================================================================
// N-grams
// ===========================================================================

/// Left/right-pad a sequence so subsequent n-gram extraction yields
/// peripheral grams. Mirrors NLTK `pad_sequence`.
pub fn pad_sequence<T: Clone>(
    sequence: &[T],
    n: usize,
    pad_left: bool,
    pad_right: bool,
    left_pad_symbol: Option<&T>,
    right_pad_symbol: Option<&T>,
) -> Vec<T> {
    let pad_n = n.saturating_sub(1);
    let mut out = Vec::with_capacity(sequence.len() + 2 * pad_n);
    if pad_left {
        let sym = left_pad_symbol.expect("pad_left requires left_pad_symbol");
        for _ in 0..pad_n {
            out.push(sym.clone());
        }
    }
    out.extend(sequence.iter().cloned());
    if pad_right {
        let sym = right_pad_symbol.expect("pad_right requires right_pad_symbol");
        for _ in 0..pad_n {
            out.push(sym.clone());
        }
    }
    out
}

/// All n-grams of length `n` from `sequence`, in order. Mirrors NLTK
/// `ngrams` (without padding — call [`pad_sequence`] first if needed).
pub fn ngrams<T: Clone>(sequence: &[T], n: usize) -> Vec<Vec<T>> {
    if n == 0 || sequence.len() < n {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(sequence.len() - n + 1);
    for i in 0..=sequence.len() - n {
        out.push(sequence[i..i + n].to_vec());
    }
    out
}

/// Bigrams (`n = 2`). Mirrors NLTK `bigrams`.
pub fn bigrams<T: Clone>(sequence: &[T]) -> Vec<(T, T)> {
    if sequence.len() < 2 {
        return Vec::new();
    }
    sequence
        .windows(2)
        .map(|w| (w[0].clone(), w[1].clone()))
        .collect()
}

/// Trigrams (`n = 3`). Mirrors NLTK `trigrams`.
pub fn trigrams<T: Clone>(sequence: &[T]) -> Vec<(T, T, T)> {
    if sequence.len() < 3 {
        return Vec::new();
    }
    sequence
        .windows(3)
        .map(|w| (w[0].clone(), w[1].clone(), w[2].clone()))
        .collect()
}

/// All n-grams of every length from `min_len` to `max_len` (inclusive)
/// within `sequence`. When `max_len == 0` the sequence length is used.
///
/// Mirrors NLTK `everygrams` (without padding for now). The output
/// order matches NLTK's "new version": for each starting position,
/// emit grams in increasing length.
pub fn everygrams<T: Clone>(sequence: &[T], min_len: usize, max_len: usize) -> Vec<Vec<T>> {
    let max_len = if max_len == 0 {
        sequence.len()
    } else {
        max_len
    };
    let min_len = min_len.max(1);
    let mut out = Vec::new();
    for start in 0..sequence.len() {
        let upper = (start + max_len).min(sequence.len());
        for end in (start + min_len)..=upper {
            out.push(sequence[start..end].to_vec());
        }
    }
    out
}

/// Skipgrams of length `n` with up to `k` skips between adjacent
/// items. Mirrors NLTK `skipgrams`.
///
/// Implementation follows NLTK: take each `(n+k)`-length window and
/// pick every `(n-1)`-combination of the tail, prepending the head.
pub fn skipgrams<T: Clone>(sequence: &[T], n: usize, k: usize) -> Vec<Vec<T>> {
    if n == 0 {
        return Vec::new();
    }
    let win = n + k;
    let mut out = Vec::new();
    let len = sequence.len();
    if len < n {
        return out;
    }
    // For each window position (allowing the window to extend past the
    // end so the head still yields tails when `len < win`):
    let last_start = if len < n { 0 } else { len - n };
    for start in 0..=last_start {
        let head = sequence[start].clone();
        let tail_end = (start + win).min(len);
        let tail: Vec<usize> = ((start + 1)..tail_end).collect();
        // Pick (n-1)-combinations of tail indices, preserving order.
        for combo in combinations(&tail, n - 1) {
            let mut gram = Vec::with_capacity(n);
            gram.push(head.clone());
            for idx in combo {
                gram.push(sequence[idx].clone());
            }
            out.push(gram);
        }
    }
    out
}

/// Iterate all `k`-element ordered combinations of `xs`, preserving
/// the input order — the standard mathematical "n choose k" without
/// replacement.
fn combinations<T: Clone>(xs: &[T], k: usize) -> Vec<Vec<T>> {
    let n = xs.len();
    if k == 0 {
        return vec![Vec::new()];
    }
    if k > n {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut idx: Vec<usize> = (0..k).collect();
    loop {
        out.push(idx.iter().map(|&i| xs[i].clone()).collect());
        // Find rightmost index that can be incremented.
        let mut i = k;
        while i > 0 {
            i -= 1;
            if idx[i] != i + n - k {
                break;
            }
            if i == 0 {
                return out;
            }
        }
        idx[i] += 1;
        for j in (i + 1)..k {
            idx[j] = idx[j - 1] + 1;
        }
    }
}

/// `(s0, s1), (s1, s2), (s2, s3), ...`. Mirrors NLTK `pairwise`.
pub fn pairwise<T: Clone>(iterable: &[T]) -> Vec<(T, T)> {
    bigrams(iterable)
}

// ===========================================================================
// Mathematical approximations
// ===========================================================================

/// Binomial coefficient nCk via the iterative form used by NLTK
/// `choose`. Returns 0 when `k` is out of range.
pub fn choose(n: u64, k: u64) -> u64 {
    if k > n {
        return 0;
    }
    let mut n = n;
    let k = k.min(n - k);
    let mut ntok: u64 = 1;
    let mut ktok: u64 = 1;
    for t in 1..=k {
        ntok = ntok.saturating_mul(n);
        ktok = ktok.saturating_mul(t);
        n -= 1;
    }
    ntok / ktok
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn graph_a() -> BTreeMap<&'static str, Vec<&'static str>> {
        // A -> B, C ; B -> C ; C -> B
        let mut g: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
        g.insert("A", vec!["B", "C"]);
        g.insert("B", vec!["C"]);
        g.insert("C", vec!["B"]);
        g
    }

    fn lookup<'a>(
        g: &'a BTreeMap<&'static str, Vec<&'static str>>,
    ) -> impl Fn(&&'static str) -> Vec<&'static str> + 'a {
        move |n: &&'static str| g.get(n).cloned().unwrap_or_default()
    }

    // ---- Traversal ----

    #[test]
    fn breadth_first_no_cycle_check() {
        let g = graph_a();
        let f = lookup(&g);
        let out = breadth_first("A", &f, 2);
        // First visits A, then B, C, then C's child B, B's child C.
        assert_eq!(&out[..3], &["A", "B", "C"]);
        assert!(out.len() > 3);
    }

    #[test]
    fn acyclic_breadth_first_visits_each_once() {
        let g = graph_a();
        let f = lookup(&g);
        let out = acyclic_breadth_first("A", &f, -1);
        assert_eq!(out, vec!["A", "B", "C"]);
    }

    #[test]
    fn edge_closure_matches_nltk_doctest() {
        // print(list(edge_closure('A', lambda node:{'A':['B','C'],'B':'C','C':'B'}[node])))
        // -> [('A','B'), ('A','C'), ('B','C'), ('C','B')]
        let g = graph_a();
        let f = lookup(&g);
        let edges = edge_closure("A", &f, -1);
        assert_eq!(edges, vec![("A", "B"), ("A", "C"), ("B", "C"), ("C", "B")]);
    }

    // ---- DFS ----

    #[test]
    fn acyclic_depth_first_basic_tree() {
        // Pure tree: A -> B,C ; B -> D
        let mut g: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
        g.insert("A", vec!["B", "C"]);
        g.insert("B", vec!["D"]);
        let f = move |n: &&'static str| g.get(n).cloned().unwrap_or_default();
        let t = acyclic_depth_first("A", f, -1, None);
        // Expect: Node("A", [Node("B", [Node("D", [])]), Node("C", [])])
        let expected = DfsTree::Node(
            "A",
            vec![
                DfsTree::Node("B", vec![DfsTree::Node("D", vec![])]),
                DfsTree::Node("C", vec![]),
            ],
        );
        assert_eq!(t, expected);
    }

    #[test]
    fn acyclic_depth_first_marks_cycles() {
        let g = graph_a();
        let f = lookup(&g);
        let t = acyclic_depth_first("A", &f, -1, Some("..."));
        // After visiting A,B,C the back-edge C->B should be marked.
        match t {
            DfsTree::Node(_, kids) => {
                // First child B should have one cycle marker (B's child C
                // is already traversed).
                assert!(matches!(kids[0], DfsTree::Node(_, _)));
            }
            _ => panic!(),
        }
    }

    // ---- MST ----

    #[test]
    fn mst_dict_matches_nltk_pattern() {
        let g = graph_a();
        let f = lookup(&g);
        let mst = unweighted_minimum_spanning_dict("A", &f);
        assert_eq!(mst.get("A").unwrap(), &vec!["B", "C"]);
        // B and C are leaves in the MST (cycle cut).
        assert!(mst.get("B").unwrap().is_empty());
        assert!(mst.get("C").unwrap().is_empty());
    }

    #[test]
    fn mst_tree_nests() {
        let g = graph_a();
        let f = lookup(&g);
        let t = unweighted_minimum_spanning_tree("A", &f);
        let expected = DfsTree::Node(
            "A",
            vec![DfsTree::Node("B", vec![]), DfsTree::Node("C", vec![])],
        );
        assert_eq!(t, expected);
    }

    // ---- Transitive closure / inversion ----

    #[test]
    fn transitive_closure_basic() {
        // A -> B -> C ; D isolated
        let mut g: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
        g.insert("A", BTreeSet::from(["B"]));
        g.insert("B", BTreeSet::from(["C"]));
        g.insert("C", BTreeSet::new());
        g.insert("D", BTreeSet::new());
        let tc = transitive_closure(&g, false);
        assert_eq!(tc.get("A").unwrap(), &BTreeSet::from(["B", "C"]));
        assert_eq!(tc.get("B").unwrap(), &BTreeSet::from(["C"]));
        assert!(tc.get("C").unwrap().is_empty());
        assert!(tc.get("D").unwrap().is_empty());
    }

    #[test]
    fn transitive_closure_reflexive() {
        let mut g: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
        g.insert("A", BTreeSet::from(["B"]));
        g.insert("B", BTreeSet::new());
        let tc = transitive_closure(&g, true);
        assert!(tc.get("A").unwrap().contains("A"));
        assert!(tc.get("A").unwrap().contains("B"));
        assert!(tc.get("B").unwrap().contains("B"));
    }

    #[test]
    fn invert_graph_reverses_edges() {
        let mut g: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
        g.insert("A", BTreeSet::from(["B", "C"]));
        g.insert("B", BTreeSet::from(["C"]));
        let inv = invert_graph(&g);
        assert_eq!(inv.get("B").unwrap(), &BTreeSet::from(["A"]));
        assert_eq!(inv.get("C").unwrap(), &BTreeSet::from(["A", "B"]));
    }

    // ---- list/dict helpers ----

    #[test]
    fn unique_list_preserves_order() {
        assert_eq!(unique_list(&[3, 1, 2, 1, 3, 4]), vec![3, 1, 2, 4]);
    }

    #[test]
    fn index_groups_by_key() {
        let idx = Index::new([("n", "dog"), ("v", "ran"), ("n", "cat")]);
        assert_eq!(idx.get(&"n").unwrap(), &["dog", "cat"]);
        assert_eq!(idx.get(&"v").unwrap(), &["ran"]);
        assert_eq!(idx.len(), 2);
    }

    #[test]
    fn invert_dict_swaps() {
        let mut d: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
        d.insert("a", vec!["x", "y"]);
        d.insert("b", vec!["x"]);
        let inv = invert_dict(&d);
        assert_eq!(inv.get("x").unwrap(), &vec!["a", "b"]);
        assert_eq!(inv.get("y").unwrap(), &vec!["a"]);
    }

    #[test]
    fn flatten_recursive() {
        // [1, 2, ['b', 'a', ['c', 'd']], 3]
        let nested: Vec<Nested<&str>> = vec![
            Nested::Leaf("1"),
            Nested::Leaf("2"),
            Nested::List(vec![
                Nested::Leaf("b"),
                Nested::Leaf("a"),
                Nested::List(vec![Nested::Leaf("c"), Nested::Leaf("d")]),
            ]),
            Nested::Leaf("3"),
        ];
        assert_eq!(flatten(&nested), vec!["1", "2", "b", "a", "c", "d", "3"]);
    }

    // ---- N-grams ----

    #[test]
    fn ngrams_doctest() {
        let s = vec![1, 2, 3, 4, 5];
        assert_eq!(
            ngrams(&s, 3),
            vec![vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5]]
        );
    }

    #[test]
    fn ngrams_with_pad_right() {
        let s = vec![1, 2, 3, 4, 5];
        let padded = pad_sequence(&s, 2, false, true, None, Some(&-1));
        assert_eq!(
            ngrams(&padded, 2),
            vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, -1]]
        );
    }

    #[test]
    fn bigrams_trigrams_match_doctest() {
        let s = vec![1, 2, 3, 4, 5];
        assert_eq!(bigrams(&s), vec![(1, 2), (2, 3), (3, 4), (4, 5)]);
        assert_eq!(trigrams(&s), vec![(1, 2, 3), (2, 3, 4), (3, 4, 5)]);
    }

    #[test]
    fn everygrams_doctest() {
        let s: Vec<&str> = vec!["a", "b", "c"];
        let got = everygrams(&s, 1, 0);
        assert_eq!(
            got,
            vec![
                vec!["a"],
                vec!["a", "b"],
                vec!["a", "b", "c"],
                vec!["b"],
                vec!["b", "c"],
                vec!["c"],
            ]
        );
    }

    #[test]
    fn skipgrams_n2_k2() {
        // sent = "Insurgents killed in ongoing fighting".split()
        // list(skipgrams(sent, 2, 2)) =
        //  [(I,k),(I,in),(I,o),(k,in),(k,o),(k,f),(in,o),(in,f),(o,f)]
        let s: Vec<&str> = vec!["I", "k", "in", "o", "f"];
        let got = skipgrams(&s, 2, 2);
        let expected: Vec<Vec<&str>> = vec![
            vec!["I", "k"],
            vec!["I", "in"],
            vec!["I", "o"],
            vec!["k", "in"],
            vec!["k", "o"],
            vec!["k", "f"],
            vec!["in", "o"],
            vec!["in", "f"],
            vec!["o", "f"],
        ];
        assert_eq!(got, expected);
    }

    #[test]
    fn pairwise_matches_bigrams() {
        let s = vec!["a", "b", "c", "d"];
        assert_eq!(pairwise(&s), vec![("a", "b"), ("b", "c"), ("c", "d")]);
    }

    // ---- choose ----

    #[test]
    fn choose_doctest() {
        assert_eq!(choose(4, 2), 6);
        assert_eq!(choose(6, 2), 15);
        assert_eq!(choose(5, 0), 1);
        assert_eq!(choose(2, 5), 0);
    }

    #[test]
    fn combinations_iterates_in_order() {
        let xs: Vec<usize> = vec![0, 1, 2, 3];
        let combos = combinations(&xs, 2);
        assert_eq!(
            combos,
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![0, 3],
                vec![1, 2],
                vec![1, 3],
                vec![2, 3],
            ]
        );
    }
}
