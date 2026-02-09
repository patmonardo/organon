//! Demonstration of the Dataset Tree DSL.

use gds::collections::dataset as ds;
use gds::collections::dataset::prelude::*;

pub fn run() {
    println!("== Dataset Tree DSL Demo ==");

    // Build a tree expression with macros.
    let expr = gds::tree!("S" => [
        gds::tree!("NP" => [gds::tleaf!("Mary")]),
        gds::tree!("VP" => [gds::tleaf!("walks")]),
    ]);
    println!("\n[expr] {expr:?}");

    // Build a concrete tree value.
    let value = ds::TreeValue::node(
        "S",
        vec![
            ds::TreeValue::node("NP", vec![ds::TreeValue::leaf("Mary")]),
            ds::TreeValue::node("VP", vec![ds::TreeValue::leaf("walks")]),
        ],
    );
    println!("[value] {}", value.format_bracketed());

    // Parse from bracketed string.
    let parsed = parse_bracketed("(S (NP Mary) (VP walks))").unwrap();
    println!("[parsed] {}", parsed.format_bracketed());

    // Tree positions.
    let root = gds::troot!();
    println!("[positions.root-expr] {root:?}");
    let preorder = value.treepositions_preorder();
    println!("[positions.preorder] {}", fmt_positions(&preorder));
    let leaf_pos = value.leaf_treeposition(0);
    println!("[positions.leaf-0] {}", fmt_option_pos(leaf_pos.as_ref()));

    // Pretty print.
    let pretty = value.pretty_print(PrettyOptions::default());
    println!("\n[pretty]\n{pretty}");

    // Transform expression (example).
    let _normalized = gds::ttransform!(expr.clone(), ds::TreeOp::Normalize);

    // Parented tree traversal.
    let parented = ParentedTree::from_tree(&value);
    let root_idx = parented.root();
    println!("\n[parented.root] {:?}", root_idx);

    // Probabilistic tree.
    let prob = ProbabilisticTree::new(value.clone(), 0.42);
    println!("[prob] {prob}");
}

fn fmt_positions(positions: &[ds::TreePos]) -> String {
    let rendered: Vec<String> = positions.iter().map(fmt_pos).collect();
    format!("[{}]", rendered.join(", "))
}

fn fmt_option_pos(pos: Option<&ds::TreePos>) -> String {
    pos.map(fmt_pos).unwrap_or_else(|| "(none)".to_string())
}

fn fmt_pos(pos: &ds::TreePos) -> String {
    if pos.path().is_empty() {
        return "[]".to_string();
    }
    let parts: Vec<String> = pos.path().iter().map(|idx| idx.to_string()).collect();
    format!("[{}]", parts.join(", "))
}

fn main() {
    run();
}
