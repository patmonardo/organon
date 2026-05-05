//! NLTK Tree Parity Demonstration Script
//!
//! This module demonstrates how the `ml/nlp/tree` facade wraps around and extends
//! the high-performance `dataset` primitives to provide a rich, linguistics-first API.

use gds::ml::nlp::tree::{GrammarTreeExt, NlpTree, ParseTreeExt, QueryTreeExt, TransformTreeExt};

pub fn run_nlp_tree_demo() {
    println!("=== NLTK Tree Parity Demonstration ===\n");

    // 1. Penn Treebank Parsing
    // We parse a complex Penn Treebank string directly into our Dataset primitives.
    let s = "(S (NP (D the) (N dog)) (VP (V chased) (NP (D the) (N cat))))";
    println!("1. Parsing Penn Treebank String:");
    println!("  Input: {}", s);
    let tree = gds::collections::dataset::tree::TreeValue::fromstring(s)
        .expect("Failed to parse tree string");

    // We can use NlpTree traits to inspect it easily
    println!("  -> Root Label: {:?}", tree.label());
    println!("  -> Text Leaves: {:?}", tree.text_leaves());
    println!();

    // 2. Grammar Productions
    // A key feature of NLTK is extracting Context-Free Grammar (CFG) rules from trees.
    println!("2. Extracting Grammar Productions:");
    let productions = tree.productions();
    for prod in &productions {
        println!("  {}", prod);
    }
    println!("  -> Extracted {} total productions.\n", productions.len());

    // 3. Subtree Querying / Filtering
    // Like NLTK's `tree.subtrees()`, we can filter and iterate over specific nodes.
    println!("3. Querying Subtrees (Filtering for Noun Phrases 'NP'):");
    let np_subtrees = tree.subtrees(|t| t.label() == Some("NP"));
    for np in &np_subtrees {
        // format_bracketed is natively on the dataset primitive
        println!("  {}", np.format_bracketed());
    }
    println!("  -> Found {} subtrees matching 'NP'.\n", np_subtrees.len());

    // 4. Advanced NLTK Transforms
    // Parent Annotation: Widely used for PCFG parameter smoothing (e.g., Markovizing).
    println!("4. Parent Annotation:");
    let annotated = tree.parent_annotate("^");
    println!("  {}", annotated.format_bracketed());

    // Un-Chomsky Normal Form: Flattening un-natural binary branching back out.
    // For this, let's parse a CNF binarized string
    println!("\n  Un-Chomsky Normal Form (Flattening `A|<B-C>` back to `A -> B C`):");
    let cnf_str = "(S (NP the_dog) (S|<VP> (V chased) (NP the_cat)))";
    let cnf_tree = gds::collections::dataset::tree::TreeValue::fromstring(cnf_str)
        .expect("Failed CNF parsing");

    println!("  Before: {}", cnf_tree.format_bracketed());
    let flat_tree = cnf_tree.un_chomsky_normal_form(false, "|", "^", "+");
    println!("  After:  {}", flat_tree.format_bracketed());

    println!("\n=== Demonstration Complete ===");
}

fn main() {
    run_nlp_tree_demo();
}
