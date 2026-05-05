//! DataFrame Series Concept walkthrough.
//!
//! Run with:
//!   cargo run -p gds --example dataframe_series_concept

use std::fs;
use std::path::{Path, PathBuf};

use gds::collections::dataframe::{series, GDSDataFrame};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== DataFrame Series Concept ==");

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    stage(
        0,
        "Series Construction",
        "Series are named and typed concept surfaces in a DataFrame body.",
    );
    let values = series("values", &[-3i64, -1, 2, 4]);
    let weights = series("weights", &[0.5f64, 1.0, 1.5, 2.0]);
    let mask = series("mask", &[true, false, true, true]);

    let df = GDSDataFrame::from_series(vec![values, weights, mask])?;
    let source_path = persist_frame(&df, &fixture_root, "00-source")?;
    println!("shape: {:?}", df.shape());
    println!("columns: {:?}", df.column_names());
    println!("persisted: {}", fixture_path(&source_path));
    println!();

    stage(
        1,
        "Concept Operations",
        "Deterministic operations derive from the series identity and type.",
    );
    let abs_values = df.select(&[gds::col!(values).abs().alias("abs_values")])?;
    let stats = df.select(&[
        gds::col!(values).sum().alias("sum_values"),
        gds::col!(weights).mean().alias("mean_weights"),
    ])?;
    let filtered = gds::where_!(df, mask)?.select_columns(&gds::selector![values])?;
    let sorted = gds::arrange!(df, [values])?.select_columns(&gds::selector![values])?;
    let head = sorted.head(2);

    let abs_path = persist_frame(&abs_values, &fixture_root, "01-abs-values")?;
    let stats_path = persist_frame(&stats, &fixture_root, "02-stats")?;
    let filtered_path = persist_frame(&filtered, &fixture_root, "03-filtered")?;
    let sorted_path = persist_frame(&sorted, &fixture_root, "04-sorted")?;
    let head_path = persist_frame(&head, &fixture_root, "05-head")?;
    println!("persisted: {}", fixture_path(&head_path));
    println!();

    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(
            &source_path,
            &abs_path,
            &stats_path,
            &filtered_path,
            &sorted_path,
            &head_path,
        ),
    )?;
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/collections/dataframe/dataframe_series_concept")
}

fn persist_frame(
    frame: &GDSDataFrame,
    root: &Path,
    file_stem: &str,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let path = root.join(format!("{file_stem}.csv"));
    frame.write_csv(&path_string(&path))?;
    Ok(path)
}

fn path_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/dataframe/dataframe_series_concept/{file_name}")
}

fn manifest(
    source_path: &Path,
    abs_path: &Path,
    stats_path: &Path,
    filtered_path: &Path,
    sorted_path: &Path,
    head_path: &Path,
) -> String {
    format!(
        "DataFrame Series Concept Fixture\n\n\
         Namespace: dataframe::series\n\n\
         00 Source\n\
         artifact: {}\n\
         meaning: named typed series assembled into DataFrame body.\n\n\
         01 Absolute\n\
         artifact: {}\n\
         meaning: concept operation on values series.\n\n\
         02 Stats\n\
         artifact: {}\n\
         meaning: sum and mean aggregate measures over series identities.\n\n\
         03 Filtered\n\
         artifact: {}\n\
         meaning: mask-constrained membership of values.\n\n\
         04-05 Ordering\n\
         sorted: {}\n\
         head: {}\n\
         meaning: ordered concept surface and leading members.\n",
        fixture_path(source_path),
        fixture_path(abs_path),
        fixture_path(stats_path),
        fixture_path(filtered_path),
        fixture_path(sorted_path),
        fixture_path(head_path),
    )
}

fn stage(number: u8, name: &str, doctrine: &str) {
    println!("-- Stage {number}: {name} --");
    println!("doctrine: {doctrine}");
}
