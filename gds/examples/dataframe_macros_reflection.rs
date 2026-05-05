//! DataFrame Macros Reflection walkthrough.
//!
//! Run with:
//!   cargo run -p gds --example dataframe_macros_reflection

use std::fs;
use std::path::{Path, PathBuf};

use gds::collections::dataframe::GDSDataFrame;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== DataFrame Macros Reflection ==");

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    stage(
        0,
        "Macro Table",
        "Macros expose the ergonomic construction surface for DataFrame work.",
    );
    let students = gds::tbl_def!(
        (id: i64 => [1, 2, 3, 4, 5]),
        (score: f64 => [62.0, 74.5, 91.0, 88.0, 97.0]),
        (cohort: ["A", "A", "B", "B", "A"]),
    )?;
    let source_path = persist_frame(&students, &fixture_root, "00-source")?;
    println!("shape: {:?}", students.shape());
    println!("persisted: {}", fixture_path(&source_path));
    println!();

    stage(
        1,
        "Macro Mutate",
        "Mutate macro applies multiple expression declarations in one step.",
    );
    let enriched = gds::mutate!(
        students,
        score_x2 = (score * 2.0),
        passed = (score >= 70.0),
        honors = (score >= 90.0),
    )?;
    let enriched_path = persist_frame(&enriched, &fixture_root, "01-enriched")?;
    println!("columns: {:?}", enriched.column_names());
    println!("persisted: {}", fixture_path(&enriched_path));
    println!();

    stage(
        2,
        "Macro Where and Arrange",
        "Where and arrange macros express judgment and ordering concisely.",
    );
    let shortlisted = gds::where_!(enriched, passed && (score_x2 > 150.0))?;
    let ordered = gds::arrange!(shortlisted, [score_x2], desc)?;
    let ordered_path = persist_frame(&ordered, &fixture_root, "02-ordered")?;
    println!("ordered shape: {:?}", ordered.shape());
    println!("persisted: {}", fixture_path(&ordered_path));
    println!();

    stage(
        3,
        "Macro Summarize",
        "Summarize macro reflects selected columns as a compact inspection surface.",
    );
    let summary = gds::summarize!(ordered, id, cohort, score, score_x2, passed, honors)?;
    let summary_path = persist_frame(&summary, &fixture_root, "03-summary")?;
    println!("summary columns: {:?}", summary.column_names());
    println!("persisted: {}", fixture_path(&summary_path));

    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(&source_path, &enriched_path, &ordered_path, &summary_path),
    )?;
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/collections/dataframe/dataframe_macros_reflection")
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
    format!("fixtures/collections/dataframe/dataframe_macros_reflection/{file_name}")
}

fn manifest(
    source_path: &Path,
    enriched_path: &Path,
    ordered_path: &Path,
    summary_path: &Path,
) -> String {
    format!(
        "DataFrame Macros Reflection Fixture\n\n\
         Namespace: dataframe::macros\n\n\
         00 Source\n\
         artifact: {}\n\
         meaning: macro-built source frame.\n\n\
         01 Enriched\n\
         artifact: {}\n\
         meaning: mutate macro introduces derived terms.\n\n\
         02 Ordered\n\
         artifact: {}\n\
         meaning: where and arrange macros enforce judgment and order.\n\n\
         03 Summary\n\
         artifact: {}\n\
         meaning: summarize macro emits compact reflective surface.\n",
        fixture_path(source_path),
        fixture_path(enriched_path),
        fixture_path(ordered_path),
        fixture_path(summary_path),
    )
}

fn stage(number: u8, name: &str, doctrine: &str) {
    println!("-- Stage {number}: {name} --");
    println!("doctrine: {doctrine}");
}
