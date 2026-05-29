//! Dataset Namespace String fixture.
//!
//! Exercises string operations through GDSSeries.str() to expose the string
//! namespace surface used by dataset/dataframe string-typed columns.
//!
//! Run with:
//!   cargo run -p gds --example dataset_namespace_string

use std::fs;
use std::path::{Path, PathBuf};

use gds::collections::dataframe::GDSSeries;

const TOKENS: &[&str] = &[
    "pre-processing",
    "tokenization",
    "post-lemmatization",
    "stemming",
    "part-of-speech",
    "named-entity",
    "dependency-parse",
    "coreference",
];
const LITERAL_MATCH: bool = true;
const STRICT_MODE: bool = false;
const PREFIX_CAPTURE_GROUP: usize = 1;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Dataset Namespace String ==");

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    let series = GDSSeries::from("token", TOKENS);
    let str_ns = series.str();

    println!("source series (token): {:?}", series);
    let src_path = fixture_root.join("00-source.txt");
    fs::write(&src_path, format!("{:?}\n", series))?;
    println!("persisted: {}", fixture_path(&src_path));
    println!();

    println!("-- Predicates --");
    println!("best practice: bind namespace once, reuse it for related operations");

    let starts_pre = str_ns.starts_with("pre")?;
    let ends_tion = str_ns.ends_with("tion")?;
    let has_dash = str_ns.contains("-", LITERAL_MATCH, STRICT_MODE)?;

    let pred_path = fixture_root.join("01-predicates.txt");
    report_and_persist(
        &pred_path,
        &[
            ("starts_with('pre')", format!("{:?}", starts_pre)),
            ("ends_with('tion')", format!("{:?}", ends_tion)),
            ("contains('-')", format!("{:?}", has_dash)),
        ],
    )?;

    println!("-- Find + Extract --");

    let find_dash = str_ns.find("-", LITERAL_MATCH, STRICT_MODE)?;
    let prefix = str_ns.extract(r"^([a-z]+)", PREFIX_CAPTURE_GROUP)?;

    let extract_path = fixture_root.join("02-extract.txt");
    report_and_persist(
        &extract_path,
        &[
            ("find('-')", format!("{:?}", find_dash)),
            ("extract(^([a-z]+), 1)", format!("{:?}", prefix)),
        ],
    )?;

    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(&src_path, &pred_path, &extract_path),
    )?;
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/collections/dataset/dataset_namespace_string")
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/dataset/dataset_namespace_string/{file_name}")
}

fn report_and_persist(
    path: &Path,
    entries: &[(&str, String)],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut out = String::new();
    for (name, value) in entries {
        println!("{name}: {value}");
        out.push_str(name);
        out.push('\n');
        out.push_str(value);
        out.push_str("\n\n");
    }
    fs::write(path, out)?;
    println!("persisted: {}", fixture_path(path));
    println!();
    Ok(())
}

fn manifest(src: &Path, pred: &Path, ext: &Path) -> String {
    format!(
        "Dataset Namespace String Fixture\n\n\
         Namespace: dataframe::namespaces::string (via GDSSeries.str)\n\n\
         00 Source\n\
         artifact: {}\n\
         meaning: string token corpus for namespace demonstration.\n\n\
         01 Predicates\n\
         artifact: {}\n\
         meaning: starts_with / ends_with / contains boolean outputs.\n\n\
         02 Find Extract\n\
         artifact: {}\n\
         meaning: find offset and regex capture extraction outputs.\n",
        fixture_path(src),
        fixture_path(pred),
        fixture_path(ext),
    )
}
