//! DataFrame String Namespace walkthrough.
//!
//! Exercises StringNameSpace (GDSSeries.str()): starts_with, ends_with,
//! contains, find, extract, replace, count_matches, and len_chars.
//!
//! Run with:
//!   cargo run -p gds --example dataframe_namespace_string

use std::fs;
use std::path::{Path, PathBuf};

use gds::collections::dataframe::GDSSeries;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== DataFrame String Namespace ==");

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    // ------------------------------------------------------------------ Stage 0
    stage(
        0,
        "Source",
        "A mixed token corpus as the string series under test.",
    );

    let tokens: Vec<&str> = vec![
        "pre-processing",
        "tokenization",
        "post-lemmatization",
        "stemming",
        "part-of-speech",
        "named-entity",
        "dependency-parse",
        "coreference",
    ];
    let series = GDSSeries::from("token", &tokens);
    let table = gds::tbl_def!(
        (token: [
            "pre-processing",
            "tokenization",
            "post-lemmatization",
            "stemming",
            "part-of-speech",
            "named-entity",
            "dependency-parse",
            "coreference"
        ]),
    )?;

    println!("tokens: {}", tokens.len());
    println!("{}", table.fmt_table());
    let src_path = persist_csv(&table, &fixture_root, "00-source")?;
    println!("persisted: {}", fixture_path(&src_path));
    println!();

    // ------------------------------------------------------------------ Stage 1
    stage(
        1,
        "starts_with / ends_with",
        "Boolean series flagging tokens by prefix or suffix.",
    );

    let str_ns = series.str();
    let has_pre = str_ns.starts_with("pre")?;
    let has_tion = str_ns.ends_with("tion")?;

    println!("starts_with('pre'): {:?}", bool_vec(&has_pre));
    println!("ends_with('tion'):  {:?}", bool_vec(&has_tion));

    let sw_table = gds::tbl_def!(
        (token: [
            "pre-processing",
            "tokenization",
            "post-lemmatization",
            "stemming",
            "part-of-speech",
            "named-entity",
            "dependency-parse",
            "coreference"
        ]),
    )?;
    let sw_with_flags = sw_table.with_columns(&[
        gds::col!("token")
            .str()
            .starts_with(gds::lit!("pre"))
            .alias("starts_pre"),
        gds::col!("token")
            .str()
            .ends_with(gds::lit!("tion"))
            .alias("ends_tion"),
    ])?;
    println!("{}", sw_with_flags.fmt_table());
    let sw_path = persist_csv(&sw_with_flags, &fixture_root, "01-sw-ew")?;
    println!("persisted: {}", fixture_path(&sw_path));
    println!();

    // ------------------------------------------------------------------ Stage 2
    stage(
        2,
        "contains",
        "Substring containment check — literal and regex modes.",
    );

    let has_dash = str_ns.contains("-", true, false)?; // literal '-'
    let has_parse = str_ns.contains("parse", true, false)?;
    println!("contains('-'):     {:?}", bool_vec(&has_dash));
    println!("contains('parse'): {:?}", bool_vec(&has_parse));

    let cnt_table = sw_table.with_columns(&[
        gds::col!("token")
            .str()
            .contains(gds::lit!("-"), true)
            .alias("has_dash"),
        gds::col!("token")
            .str()
            .contains(gds::lit!("parse"), true)
            .alias("has_parse"),
    ])?;
    println!("{}", cnt_table.fmt_table());
    let cnt_path = persist_csv(&cnt_table, &fixture_root, "02-contains")?;
    println!("persisted: {}", fixture_path(&cnt_path));
    println!();

    // ------------------------------------------------------------------ Stage 3
    stage(
        3,
        "find",
        "Returns the byte offset of the first match, or null if absent.",
    );

    let find_dash = str_ns.find("-", true, false)?;
    println!("find('-') offsets: {:?}", opt_i32_vec(&find_dash));

    let find_table = sw_table.with_columns(&[gds::col!("token")
        .str()
        .find(gds::lit!("-"), true)
        .alias("dash_offset")])?;
    println!("{}", find_table.fmt_table());
    let find_path = persist_csv(&find_table, &fixture_root, "03-find")?;
    println!("persisted: {}", fixture_path(&find_path));
    println!();

    // ------------------------------------------------------------------ Stage 4
    stage(
        4,
        "extract",
        "Regex capture: pull the first component before a dash.",
    );

    // Capture the first word before a dash (group 1)
    let prefix = str_ns.extract(r"^([a-z]+)", 1)?;
    println!("extract('^([a-z]+)', 1): {:?}", str_opt_vec(&prefix));

    let ext_table = sw_table.with_columns(&[gds::col!("token")
        .str()
        .extract(gds::lit!(r"^([a-z]+)"), 1)
        .alias("prefix")])?;
    println!("{}", ext_table.fmt_table());
    let ext_path = persist_csv(&ext_table, &fixture_root, "04-extract")?;
    println!("persisted: {}", fixture_path(&ext_path));
    println!();

    // ------------------------------------------------------------------ README
    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(&src_path, &sw_path, &cnt_path, &find_path, &ext_path),
    )?;
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

// ── display helpers ───────────────────────────────────────────────────────────

fn bool_vec(series: &polars::prelude::Series) -> Vec<Option<bool>> {
    series.bool().unwrap().into_iter().collect()
}

fn opt_i32_vec(series: &polars::prelude::Series) -> Vec<Option<i32>> {
    series
        .cast(&polars::prelude::DataType::Int32)
        .unwrap()
        .i32()
        .unwrap()
        .into_iter()
        .collect()
}

fn str_opt_vec(series: &polars::prelude::Series) -> Vec<Option<String>> {
    series
        .str()
        .unwrap()
        .into_iter()
        .map(|v| v.map(|s| s.to_string()))
        .collect()
}

// ── fixture helpers ───────────────────────────────────────────────────────────

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/collections/dataframe/dataframe_namespace_string")
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/dataframe/dataframe_namespace_string/{file_name}")
}

fn path_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn persist_csv(
    frame: &gds::collections::dataframe::GDSDataFrame,
    root: &Path,
    stem: &str,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let path = root.join(format!("{stem}.csv"));
    frame.write_csv(&path_string(&path))?;
    Ok(path)
}

fn stage(n: u32, name: &str, doctrine: &str) {
    println!("── Stage {n}: {name} ──────────────────────────────────────────");
    println!("   {doctrine}");
    println!();
}

fn manifest(src: &Path, sw: &Path, cnt: &Path, find: &Path, ext: &Path) -> String {
    format!(
        "DataFrame String Namespace Fixture\n\n\
         Namespace: dataframe::namespaces::string (GDSSeries.str())\n\n\
         00 Source\n\
         artifact: {}\n\
         meaning: 8-token NLP corpus string series.\n\n\
         01 starts_with / ends_with\n\
         artifact: {}\n\
         meaning: Boolean columns for prefix/suffix membership.\n\n\
         02 contains\n\
         artifact: {}\n\
         meaning: Literal substring containment in two columns.\n\n\
         03 find\n\
         artifact: {}\n\
         meaning: Byte offset of first match, null when absent.\n\n\
         04 extract\n\
         artifact: {}\n\
         meaning: Regex capture group 1: alphabetic prefix before dash.\n",
        fixture_path(src),
        fixture_path(sw),
        fixture_path(cnt),
        fixture_path(find),
        fixture_path(ext),
    )
}
