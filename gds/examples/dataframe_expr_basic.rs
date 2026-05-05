//! DataFrame Expr Basic walkthrough.
//!
//! Run with:
//!   cargo run -p gds --example dataframe_expr_basic

use std::fs;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== DataFrame Expr Basic ==");

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    stage(
        0,
        "Source Frame",
        "Expr begins from a concrete frame of named columns.",
    );
    let table = gds::tbl_def!(
        (id: i64 => [1, 2, 3, 4, 5]),
        (score: f64 => [10.0, 25.0, 40.0, 15.0, 30.0]),
    )?;
    let source_path = persist_frame(&table, &fixture_root, "00-source")?;
    println!("shape: {:?}", table.shape());
    println!("persisted: {}", fixture_path(&source_path));
    println!();

    stage(
        1,
        "Predicate Expr",
        "Expr can be built once and reused for filtering and derivation.",
    );
    let high_score = gds::expr!(score > 20.0);
    let filtered = table.filter_expr(high_score.clone())?;
    let filtered_path = persist_frame(&filtered, &fixture_root, "01-filtered")?;
    println!("filtered shape: {:?}", filtered.shape());
    println!("persisted: {}", fixture_path(&filtered_path));
    println!();

    stage(
        2,
        "Derived Expr",
        "Expr produces a deterministic derived column.",
    );
    let tagged = table.with_columns(&[high_score.alias("is_high")])?;
    let tagged_path = persist_frame(&tagged, &fixture_root, "02-tagged")?;
    println!("tagged columns: {:?}", tagged.column_names());
    println!("persisted: {}", fixture_path(&tagged_path));
    println!();

    stage(
        3,
        "Aggregate Expr",
        "Expr drives aggregate judgment over the frame.",
    );
    let grouped = gds::group_by!(tagged, [is_high], [gds::agg!(score.mean => "mean_score")])?;
    let grouped_path = persist_frame(&grouped, &fixture_root, "03-grouped")?;
    println!("grouped shape: {:?}", grouped.shape());
    println!("persisted: {}", fixture_path(&grouped_path));

    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(&source_path, &filtered_path, &tagged_path, &grouped_path),
    )?;
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/collections/dataframe/dataframe_expr_basic")
}

fn persist_frame(
    frame: &gds::collections::dataframe::GDSDataFrame,
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
    format!("fixtures/collections/dataframe/dataframe_expr_basic/{file_name}")
}

fn manifest(
    source_path: &Path,
    filtered_path: &Path,
    tagged_path: &Path,
    grouped_path: &Path,
) -> String {
    format!(
        "DataFrame Expr Basic Fixture\n\n\
         Namespace: dataframe::expr\n\n\
         00 Source\n\
         artifact: {}\n\
         meaning: source frame for expression evaluation.\n\n\
         01 Filtered\n\
         artifact: {}\n\
         meaning: predicate expression reused as filter.\n\n\
         02 Tagged\n\
         artifact: {}\n\
         meaning: predicate expression materialized as a derived column.\n\n\
         03 Grouped\n\
         artifact: {}\n\
         meaning: expression-driven aggregate judgment by derived identity.\n",
        fixture_path(source_path),
        fixture_path(filtered_path),
        fixture_path(tagged_path),
        fixture_path(grouped_path),
    )
}

fn stage(number: u8, name: &str, doctrine: &str) {
    println!("-- Stage {number}: {name} --");
    println!("doctrine: {doctrine}");
}
