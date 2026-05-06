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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Dataset Namespace Array ==");

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    // ------------------------------------------------------------------ Stage 0
    stage(
        0,
        "Source Array Series",
        "Construct a fixed-width numeric sequence surface for array namespace ops.",
    );

    let values: Vec<Vec<i64>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![2, 2, 2], vec![9, 8, 7]];
    let list_series = GDSSeries::from_list_i64("vals", &values);
    let series = GDSSeries::new(list_series.list().to_array(3)?);
    let arr_ns = series.arr();

    println!("source series: {:?}", series);
    let src_path = fixture_root.join("00-source.txt");
    fs::write(&src_path, format!("{:?}\n", series))?;
    println!("persisted: {}", fixture_path(&src_path));
    println!();

    // ------------------------------------------------------------------ Stage 1
    stage(
        1,
        "len / sum / max",
        "Per-row array aggregate operations over sequence elements.",
    );

    let lens = arr_ns.len()?;
    let sums = arr_ns.sum()?;
    let maxs = arr_ns.max()?;

    println!("len : {:?}", lens);
    println!("sum : {:?}", sums);
    println!("max : {:?}", maxs);

    let agg_path = fixture_root.join("01-aggregates.txt");
    fs::write(
        &agg_path,
        format!("len\n{:?}\n\nsum\n{:?}\n\nmax\n{:?}\n", lens, sums, maxs),
    )?;
    println!("persisted: {}", fixture_path(&agg_path));
    println!();

    // ------------------------------------------------------------------ Stage 2
    stage(
        2,
        "first / last / get",
        "Positional array extraction from each row-array.",
    );

    let first = arr_ns.first()?;
    let last = arr_ns.last()?;
    let second = arr_ns.get(1, true)?;

    println!("first : {:?}", first);
    println!("last  : {:?}", last);
    println!("get(1): {:?}", second);

    let pos_path = fixture_root.join("02-positional.txt");
    fs::write(
        &pos_path,
        format!(
            "first\n{:?}\n\nlast\n{:?}\n\nget(1)\n{:?}\n",
            first, last, second
        ),
    )?;
    println!("persisted: {}", fixture_path(&pos_path));
    println!();

    // ------------------------------------------------------------------ Stage 3
    stage(
        3,
        "unique / n_unique / reverse",
        "Array normalization operations inside each row-array.",
    );

    let unique = arr_ns.unique(true)?;
    let n_unique = arr_ns.n_unique()?;
    let reversed = arr_ns.reverse()?;

    println!("unique  : {:?}", unique);
    println!("n_unique: {:?}", n_unique);
    println!("reverse : {:?}", reversed);

    let norm_path = fixture_root.join("03-normalize.txt");
    fs::write(
        &norm_path,
        format!(
            "unique\n{:?}\n\nn_unique\n{:?}\n\nreverse\n{:?}\n",
            unique, n_unique, reversed
        ),
    )?;
    println!("persisted: {}", fixture_path(&norm_path));
    println!();

    // ------------------------------------------------------------------ README
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

fn stage(n: u32, name: &str, doctrine: &str) {
    println!("── Stage {n}: {name} ──────────────────────────────────────────");
    println!("   {doctrine}");
    println!();
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
