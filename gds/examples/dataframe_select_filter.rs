//! DataFrame Select and Filter walkthrough.
//!
//! Run with:
//!   cargo run -p gds --example dataframe_select_filter

use std::fs;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== DataFrame Select and Filter ==");

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    stage(
        0,
        "Input",
        "Selectors and filters operate on the immediate DataFrame body.",
    );
    let table = gds::tbl_def!(
        (id: i64 => [1, 2, 3, 4, 5]),
        (score: f64 => [10.0, 25.0, 40.0, 15.0, 30.0]),
        (weight: f64 => [1.1, 0.8, 1.5, 1.0, 1.2]),
        (group: ["A", "B", "A", "B", "A"]),
    )?;
    let source_path = persist_frame(&table, &fixture_root, "00-source")?;
    println!("shape: {:?}", table.shape());
    println!("persisted: {}", fixture_path(&source_path));
    println!();

    stage(
        1,
        "Select",
        "Selectors delimit which columns remain visible.",
    );
    let selected = gds::select_columns!(table, [id, score])?;
    let selected_path = persist_frame(&selected, &fixture_root, "01-selected")?;
    println!("selected columns: {:?}", selected.column_names());
    println!("persisted: {}", fixture_path(&selected_path));
    println!();

    stage(
        2,
        "Filter",
        "Filter predicates determine row membership under a threshold.",
    );
    let filtered = gds::filter!(table, score > 20.0)?;
    let filtered_path = persist_frame(&filtered, &fixture_root, "02-filtered")?;
    println!("filtered shape: {:?}", filtered.shape());
    println!("persisted: {}", fixture_path(&filtered_path));
    println!();

    stage(
        3,
        "Projected Expr",
        "Select with expressions preserves addresses while introducing aliases.",
    );
    let projected = gds::select!(table, id, (score * 2.0) as "score_x2")?;
    let projected_path = persist_frame(&projected, &fixture_root, "03-projected")?;
    println!("projected columns: {:?}", projected.column_names());
    println!("persisted: {}", fixture_path(&projected_path));

    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(&source_path, &selected_path, &filtered_path, &projected_path),
    )?;
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/collections/dataframe/dataframe_select_filter")
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
    format!("fixtures/collections/dataframe/dataframe_select_filter/{file_name}")
}

fn manifest(
    source_path: &Path,
    selected_path: &Path,
    filtered_path: &Path,
    projected_path: &Path,
) -> String {
    format!(
        "DataFrame Select Filter Fixture\n\n\
         Namespace: dataframe::selectors\n\n\
         00 Source\n\
         artifact: {}\n\
         meaning: immediate frame before selector/filter operations.\n\n\
         01 Selected\n\
         artifact: {}\n\
         meaning: narrowed columns by selector grammar.\n\n\
         02 Filtered\n\
         artifact: {}\n\
         meaning: row membership constrained by predicate expression.\n\n\
         03 Projected\n\
         artifact: {}\n\
         meaning: expression projection with explicit aliasing.\n",
        fixture_path(source_path),
        fixture_path(selected_path),
        fixture_path(filtered_path),
        fixture_path(projected_path),
    )
}

fn stage(number: u8, name: &str, doctrine: &str) {
    println!("-- Stage {number}: {name} --");
    println!("doctrine: {doctrine}");
}
