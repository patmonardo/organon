//! Dataset Namespace List fixture.
//!
//! Exercises list operations through GDSSeries.list() to expose the list
//! namespace surface used by dataset/dataframe list-typed columns.
//!
//! Run with:
//!   cargo run -p gds --example dataset_namespace_list

use std::fs;
use std::path::{Path, PathBuf};

use gds::collections::dataframe::GDSSeries;

const LIST_VALUES: &[&[i64]] = &[&[1, 2, 3], &[4, 5], &[6], &[2, 2, 2], &[9, 8, 7, 6]];
const POSITION_INDEX: i64 = 1;
const NULL_ON_OOB: bool = true;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Dataset Namespace List ==");

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    let values: Vec<Vec<i64>> = LIST_VALUES.iter().map(|row| row.to_vec()).collect();
    let series = GDSSeries::from_list_i64("span", &values);
    let list_ns = series.list();

    println!("source series: {:?}", series);
    let src_path = fixture_root.join("00-source.txt");
    fs::write(&src_path, format!("{:?}\n", series))?;
    println!("persisted: {}", fixture_path(&src_path));
    println!();

    println!("-- Aggregates --");
    println!("best practice: group related namespace calls by operation family");

    let lens = list_ns.len()?;
    let sums = list_ns.sum()?;
    let maxs = list_ns.max()?;

    let agg_path = fixture_root.join("01-aggregates.txt");
    report_and_persist(
        &agg_path,
        &[
            ("len", format!("{:?}", lens)),
            ("sum", format!("{:?}", sums)),
            ("max", format!("{:?}", maxs)),
        ],
    )?;

    println!("-- Positional --");

    let first = list_ns.first()?;
    let last = list_ns.last()?;
    let second = list_ns.get(POSITION_INDEX, NULL_ON_OOB)?;

    let pos_path = fixture_root.join("02-positional.txt");
    report_and_persist(
        &pos_path,
        &[
            ("first", format!("{:?}", first)),
            ("last", format!("{:?}", last)),
            ("get(POSITION_INDEX)", format!("{:?}", second)),
        ],
    )?;

    println!("-- Normalize --");

    let unique = list_ns.unique(true)?;
    let n_unique = list_ns.n_unique()?;
    let reversed = list_ns.reverse()?;

    let norm_path = fixture_root.join("03-normalize.txt");
    report_and_persist(
        &norm_path,
        &[
            ("unique", format!("{:?}", unique)),
            ("n_unique", format!("{:?}", n_unique)),
            ("reverse", format!("{:?}", reversed)),
        ],
    )?;

    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(&src_path, &agg_path, &pos_path, &norm_path),
    )?;
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/collections/dataset/dataset_namespace_list")
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/dataset/dataset_namespace_list/{file_name}")
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

fn manifest(src: &Path, agg: &Path, pos: &Path, norm: &Path) -> String {
    format!(
        "Dataset Namespace List Fixture\n\n\
         Namespace: dataframe::namespaces::list (via GDSSeries.list)\n\n\
         00 Source\n\
         artifact: {}\n\
         meaning: list-typed integer spans for namespace demonstration.\n\n\
         01 Aggregates\n\
         artifact: {}\n\
         meaning: per-row len, sum, and max over list elements.\n\n\
         02 Positional\n\
         artifact: {}\n\
         meaning: first, last, and get(1) extraction from list rows.\n\n\
         03 Normalize\n\
         artifact: {}\n\
         meaning: unique, n_unique, and reverse list transforms.\n",
        fixture_path(src),
        fixture_path(agg),
        fixture_path(pos),
        fixture_path(norm),
    )
}
