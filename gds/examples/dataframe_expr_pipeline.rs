//! DataFrame Expr Pipeline walkthrough.
//!
//! Run with:
//!   cargo run -p gds --example dataframe_expr_pipeline

use std::fs;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== DataFrame Expr Pipeline ==");

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    stage(
        0,
        "Input",
        "Expr pipeline starts from determinate numeric series.",
    );
    let df = gds::tbl_def!(
        (a: i64 => [1, 2, 3, 4]),
        (b: i64 => [10, 20, 30, 40]),
    )?;
    let source_path = persist_frame(&df, &fixture_root, "00-source")?;
    println!("shape: {:?}", df.shape());
    println!("persisted: {}", fixture_path(&source_path));
    println!();

    stage(
        1,
        "Mutate",
        "Expr composition builds new determinations over the source.",
    );
    let mutated = gds::mutate!(
        df,
        sum_ab = { gds::col!(a) + gds::col!(b) },
        prod_ab = { gds::col!(a) * gds::col!(b) },
    )?;
    let mutated_path = persist_frame(&mutated, &fixture_root, "01-mutated")?;
    println!("columns: {:?}", mutated.column_names());
    println!("persisted: {}", fixture_path(&mutated_path));
    println!();

    stage(
        2,
        "Filter + Project",
        "Pipeline narrows by judgment and projects selected outputs.",
    );
    let filtered = mutated.filter(gds::col!(sum_ab).gt(gds::lit!(25)))?;
    let projected = filtered.select(&[
        gds::col!(a),
        gds::col!(b),
        gds::col!(sum_ab),
        gds::col!(prod_ab),
    ])?;
    let projected_path = persist_frame(&projected, &fixture_root, "02-projected")?;
    println!("projected shape: {:?}", projected.shape());
    println!("persisted: {}", fixture_path(&projected_path));

    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(&source_path, &mutated_path, &projected_path),
    )?;
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/collections/dataframe/dataframe_expr_pipeline")
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
    format!("fixtures/collections/dataframe/dataframe_expr_pipeline/{file_name}")
}

fn manifest(source_path: &Path, mutated_path: &Path, projected_path: &Path) -> String {
    format!(
        "DataFrame Expr Pipeline Fixture\n\n\
         Namespace: dataframe::expr\n\n\
         00 Source\n\
         artifact: {}\n\
         meaning: base frame for expression composition.\n\n\
         01 Mutated\n\
         artifact: {}\n\
         meaning: composed expressions create derived determinations.\n\n\
         02 Projected\n\
         artifact: {}\n\
         meaning: filtered and projected expression pipeline result.\n",
        fixture_path(source_path),
        fixture_path(mutated_path),
        fixture_path(projected_path),
    )
}

fn stage(number: u8, name: &str, doctrine: &str) {
    println!("-- Stage {number}: {name} --");
    println!("doctrine: {doctrine}");
}
