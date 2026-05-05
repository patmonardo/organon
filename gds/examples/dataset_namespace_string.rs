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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Dataset Namespace String ==");

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    // ------------------------------------------------------------------ Stage 0
    stage(
        0,
        "Source String Series",
        "Construct a string series with NLP-oriented tokens.",
    );

    let values: Vec<&str> = vec![
        "pre-processing",
        "tokenization",
        "post-lemmatization",
        "stemming",
        "part-of-speech",
        "named-entity",
        "dependency-parse",
        "coreference",
    ];
    let series = GDSSeries::from("token", &values);
    let str_ns = series.str();

    println!("source series: {:?}", series);
    let src_path = fixture_root.join("00-source.txt");
    fs::write(&src_path, format!("{:?}\n", series))?;
    println!("persisted: {}", fixture_path(&src_path));
    println!();

    // ------------------------------------------------------------------ Stage 1
    stage(
        1,
        "starts_with / ends_with / contains",
        "Core boolean string predicates over each token row.",
    );

    let starts_pre = str_ns.starts_with("pre")?;
    let ends_tion = str_ns.ends_with("tion")?;
    let has_dash = str_ns.contains("-", true, false)?;

    println!("starts_with('pre'): {:?}", starts_pre);
    println!("ends_with('tion') : {:?}", ends_tion);
    println!("contains('-')     : {:?}", has_dash);

    let pred_path = fixture_root.join("01-predicates.txt");
    fs::write(
        &pred_path,
        format!(
            "starts_with_pre\n{:?}\n\nends_with_tion\n{:?}\n\ncontains_dash\n{:?}\n",
            starts_pre, ends_tion, has_dash
        ),
    )?;
    println!("persisted: {}", fixture_path(&pred_path));
    println!();

    // ------------------------------------------------------------------ Stage 2
    stage(
        2,
        "find / extract",
        "Index and capture-based extraction from token strings.",
    );

    let find_dash = str_ns.find("-", true, false)?;
    let prefix = str_ns.extract(r"^([a-z]+)", 1)?;

    println!("find('-'): {:?}", find_dash);
    println!("extract(^([a-z]+),1): {:?}", prefix);

    let extract_path = fixture_root.join("02-extract.txt");
    fs::write(
        &extract_path,
        format!("find_dash\n{:?}\n\nprefix\n{:?}\n", find_dash, prefix),
    )?;
    println!("persisted: {}", fixture_path(&extract_path));
    println!();

    // ------------------------------------------------------------------ README
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

fn stage(n: u32, name: &str, doctrine: &str) {
    println!("── Stage {n}: {name} ──────────────────────────────────────────");
    println!("   {doctrine}");
    println!();
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
