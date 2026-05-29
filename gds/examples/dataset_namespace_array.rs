//! Dataset Namespace Array fixture.
//!
//! Exercises array operations through GDSSeries.arr() to expose the array
//! namespace surface used by dataset/dataframe array-typed columns.
//!
//! Run with:
//!   cargo run -p gds --example dataset_namespace_array

use std::fs;
use std::path::{Path, PathBuf};

use gds::collections::dataframe::GDSSeries;

const ARRAY_VALUES: &[&[i64]] = &[&[1, 2, 3], &[4, 5, 6], &[2, 2, 2], &[9, 8, 7]];
const ARRAY_WIDTH: usize = 3;
const POSITION_INDEX: i64 = 1;
const NULL_ON_OOB: bool = true;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Dataset Namespace Array ==");

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    let values: Vec<Vec<i64>> = ARRAY_VALUES.iter().map(|row| row.to_vec()).collect();
    let list_series = GDSSeries::from_list_i64("vals", &values);
    let series = GDSSeries::new(list_series.list().to_array(ARRAY_WIDTH)?);
    let arr_ns = series.arr();

    println!("source series: {:?}", series);
    let src_path = fixture_root.join("00-source.txt");
    fs::write(&src_path, format!("{:?}\n", series))?;
    println!("persisted: {}", fixture_path(&src_path));
    println!();

    println!("-- Aggregates --");
    println!("best practice: convert to fixed-width array once, then reuse namespace");

    let lens = arr_ns.len()?;
    let sums = arr_ns.sum()?;
    let maxs = arr_ns.max()?;

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

    let first = arr_ns.first()?;
    let last = arr_ns.last()?;
    let second = arr_ns.get(POSITION_INDEX, NULL_ON_OOB)?;

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

    let unique = arr_ns.unique(true)?;
    let n_unique = arr_ns.n_unique()?;
    let reversed = arr_ns.reverse()?;

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
        .join("fixtures/collections/dataset/dataset_namespace_array")
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/dataset/dataset_namespace_array/{file_name}")
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
        "Dataset Namespace Array Fixture\n\n\
         Namespace: dataframe::namespaces::array (via GDSSeries.arr)\n\n\
         00 Source\n\
         artifact: {}\n\
         meaning: fixed-width numeric sequences for array namespace demonstration.\n\n\
         01 Aggregates\n\
         artifact: {}\n\
         meaning: per-row len, sum, and max over array elements.\n\n\
         02 Positional\n\
         artifact: {}\n\
         meaning: first, last, and get(1) extraction from arrays.\n\n\
         03 Normalize\n\
         artifact: {}\n\
         meaning: unique, n_unique, and reverse transforms.\n",
        fixture_path(src),
        fixture_path(agg),
        fixture_path(pos),
        fixture_path(norm),
    )
}
