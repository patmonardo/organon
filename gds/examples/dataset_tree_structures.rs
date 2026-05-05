//! Dataset Tree Structures fixture.
//!
//! Exercises the LM tree fold: TreeValue construction, parse_bracketed,
//! format_bracketed, height, leaves, TreeCollection, and ParentedTree.
//!
//! Run with:
//!   cargo run -p gds --example dataset_tree_structures

use std::fs;
use std::path::{Path, PathBuf};

use gds::collections::dataset::prelude::*;
use gds::collections::dataset::tree::{TreeLeafValue, TreeValue};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Dataset Tree Structures ==");

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    // ------------------------------------------------------------------ Stage 0
    stage(0, "Hand-Built Tree", "TreeValue::node / leaf construct a concrete syntax tree.");

    let tree = TreeValue::node(
        "S",
        vec![
            TreeValue::node(
                "NP",
                vec![
                    TreeValue::node("DT", vec![TreeValue::leaf(TreeLeafValue::text("The"))]),
                    TreeValue::node("NN", vec![TreeValue::leaf(TreeLeafValue::text("cat"))]),
                ],
            ),
            TreeValue::node(
                "VP",
                vec![
                    TreeValue::node("VBZ", vec![TreeValue::leaf(TreeLeafValue::text("sat"))]),
                    TreeValue::node(
                        "PP",
                        vec![
                            TreeValue::node(
                                "IN",
                                vec![TreeValue::leaf(TreeLeafValue::text("on"))],
                            ),
                            TreeValue::node(
                                "NP",
                                vec![
                                    TreeValue::node(
                                        "DT",
                                        vec![TreeValue::leaf(TreeLeafValue::text("the"))],
                                    ),
                                    TreeValue::node(
                                        "NN",
                                        vec![TreeValue::leaf(TreeLeafValue::text("mat"))],
                                    ),
                                ],
                            ),
                        ],
                    ),
                ],
            ),
        ],
    );

    let leaves = tree.leaves();
    let height = tree.height();
    let bracketed = tree.format_bracketed();
    let pretty = tree.format_pretty(2);

    println!("leaves: {}", leaves.len());
    println!("height: {}", height);
    println!("bracketed: {}", &bracketed[..bracketed.len().min(80)]);

    let built_path = fixture_root.join("00-built.txt");
    fs::write(
        &built_path,
        format!(
            "leaves: {}\nheight: {}\nbracketed: {}\npretty:\n{}\n",
            leaves.len(),
            height,
            bracketed,
            pretty,
        ),
    )?;
    println!("persisted: {}", fixture_path(&built_path));
    println!();

    // ------------------------------------------------------------------ Stage 1
    stage(
        1,
        "Parse Bracketed",
        "parse_bracketed round-trips a bracketed string to TreeValue.",
    );

    let input = "(S (NP (DT The) (NN dog)) (VP (VBZ runs) (ADVP (RB quickly))))";
    let parsed = parse_bracketed(input)?;
    let roundtrip = format_bracketed(&parsed);
    let parsed_leaves = parsed.leaves();
    let parsed_height = parsed.height();

    println!("input:     {}", input);
    println!("roundtrip: {}", roundtrip);
    println!("leaves: {}", parsed_leaves.len());
    println!("height: {}", parsed_height);

    let parse_path = fixture_root.join("01-parse.txt");
    fs::write(
        &parse_path,
        format!(
            "input: {}\nroundtrip: {}\nleaves: {}\nheight: {}\n",
            input,
            roundtrip,
            parsed_leaves.len(),
            parsed_height,
        ),
    )?;
    println!("persisted: {}", fixture_path(&parse_path));
    println!();

    // ------------------------------------------------------------------ Stage 2
    stage(
        2,
        "TreeCollection",
        "TreeCollection holds a set of trees with stable indices.",
    );

    let mut collection = TreeCollection::new();
    collection.push(tree.clone(), None);
    collection.push(parsed.clone(), None);
    let normalized = collection.normalize();
    let cnf = collection.chomsky_normal_form();

    println!("collection len: {}", collection.len());
    println!("normalized len: {}", normalized.len());
    println!("cnf len: {}", cnf.len());

    let collection_path = fixture_root.join("02-collection.txt");
    fs::write(
        &collection_path,
        format!(
            "original trees: {}\n\
             normalized:\n{}\n\
             chomsky normal form:\n{}\n",
            collection.len(),
            normalized
                .iter()
                .map(|t| t.format_bracketed())
                .collect::<Vec<_>>()
                .join("\n"),
            cnf.iter()
                .map(|t| t.format_bracketed())
                .collect::<Vec<_>>()
                .join("\n"),
        ),
    )?;
    println!("persisted: {}", fixture_path(&collection_path));
    println!();

    // ------------------------------------------------------------------ Stage 3
    stage(
        3,
        "Parented Tree",
        "ParentedTree provides parent-pointer navigation over a TreeValue.",
    );

    let parented = ParentedTree::from_tree(&tree);
    let root = parented.root();
    let root_label = parented
        .get(root)
        .and_then(|n| n.label())
        .unwrap_or("?");
    let child_count = parented
        .get(root)
        .and_then(|n| n.children())
        .map(|c| c.len())
        .unwrap_or(0);

    println!("root label:  {}", root_label);
    println!("root children: {}", child_count);

    let parented_path = fixture_root.join("03-parented.txt");
    fs::write(
        &parented_path,
        format!(
            "root: {}\nroot children: {}\n",
            root_label, child_count,
        ),
    )?;
    println!("persisted: {}", fixture_path(&parented_path));

    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(&built_path, &parse_path, &collection_path, &parented_path),
    )?;
    println!("\nmanifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/collections/dataset/dataset_tree_structures")
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/dataset/dataset_tree_structures/{file_name}")
}

fn manifest(
    built_path: &Path,
    parse_path: &Path,
    collection_path: &Path,
    parented_path: &Path,
) -> String {
    format!(
        "Dataset Tree Structures Fixture\n\n\
         Namespace: dataset::lm::tree (dataset::tree shim)\n\n\
         00 Hand-Built\n\
         artifact: {}\n\
         meaning: direct TreeValue::node/leaf construction, leaves, height, pretty.\n\n\
         01 Parse\n\
         artifact: {}\n\
         meaning: parse_bracketed + format_bracketed roundtrip.\n\n\
         02 Collection\n\
         artifact: {}\n\
         meaning: TreeCollection normalize + CNF transforms over multiple trees.\n\n\
         03 Parented\n\
         artifact: {}\n\
         meaning: ParentedTree parent-pointer navigation.\n",
        fixture_path(built_path),
        fixture_path(parse_path),
        fixture_path(collection_path),
        fixture_path(parented_path),
    )
}

fn stage(number: u8, name: &str, doctrine: &str) {
    println!("-- Stage {number}: {name} --");
    println!("doctrine: {doctrine}");
}
