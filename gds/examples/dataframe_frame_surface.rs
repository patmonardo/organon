//! DataFrame Frame surface doctrinal fixture.
//!
//! Run with:
//!   cargo run -p gds --example dataframe_frame_surface

use std::fs;
use std::path::{Path, PathBuf};

use gds::collections::dataframe::{col, lit};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== DataFrame Frame Surface ==");
    println!("The Frame is the executable enclosure: shape, schema, rows, columns, and eager transforms.");
    println!("This example reads like RustScript over a Polars-shaped client surface.");
    println!();

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    stage(0, "Construct", "Frame begins as a shaped tabular body.");
    let frame = gds::tbl_def!(
        (row_id: i64 => [1, 2, 3, 4]),
        (subject: ["frame", "series", "expr", "lazy"]),
        (score: i64 => [10, 20, 30, 40]),
        (sphere: ["being", "being", "being", "deferred"]),
    )?;
    let source_path = persist_frame(&frame, &fixture_root, "00-frame")?;
    println!(
        "shape: {} rows x {} columns",
        frame.row_count(),
        frame.column_count()
    );
    println!("columns: {:?}", frame.column_names());
    println!("persisted: {}", fixture_path(&source_path));
    println!();

    stage(
        1,
        "Inspect",
        "A Frame must reveal schema and dtype before mediation.",
    );
    let schema_path = fixture_root.join("01-schema.txt");
    fs::write(
        &schema_path,
        format!(
            "columns: {:?}\ndtypes: {:?}\nshape: {:?}\nestimated_size: {}\nchunks: {}\n",
            frame.column_names(),
            frame.dtypes(),
            frame.shape(),
            frame.estimated_size(),
            frame.n_chunks(),
        ),
    )?;
    println!("dtypes: {:?}", frame.dtypes());
    println!("persisted: {}", fixture_path(&schema_path));
    println!();

    stage(
        2,
        "Transform",
        "Frame transformations are eager and still remain DataFrame work.",
    );
    let enriched = frame.with_columns(&[
        (col("score") * lit(2)).alias("double_score"),
        lit("frame-surface").alias("surface"),
    ])?;
    let selected = enriched.select(&[
        col("row_id"),
        col("subject"),
        col("double_score"),
        col("surface"),
    ])?;
    let filtered = selected.filter(col("double_score").gt(lit(30)))?;
    let renamed = filtered.rename_columns(&[("subject", "term")])?;
    let head = renamed.head(2);
    let enriched_path = persist_frame(&enriched, &fixture_root, "02-enriched")?;
    let selected_path = persist_frame(&selected, &fixture_root, "03-selected")?;
    let filtered_path = persist_frame(&filtered, &fixture_root, "04-filtered")?;
    let renamed_path = persist_frame(&renamed, &fixture_root, "05-renamed")?;
    let head_path = persist_frame(&head, &fixture_root, "06-head")?;
    println!("enriched columns: {:?}", enriched.column_names());
    println!("filtered shape: {:?}", filtered.shape());
    println!("renamed columns: {:?}", renamed.column_names());
    println!("persisted: {}", fixture_path(&head_path));
    println!();

    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(
            &source_path,
            &schema_path,
            &enriched_path,
            &selected_path,
            &filtered_path,
            &renamed_path,
            &head_path,
        ),
    )?;
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/collections/dataframe/dataframe_frame_surface")
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
    format!("fixtures/collections/dataframe/dataframe_frame_surface/{file_name}")
}

fn manifest(
    source_path: &Path,
    schema_path: &Path,
    enriched_path: &Path,
    selected_path: &Path,
    filtered_path: &Path,
    renamed_path: &Path,
    head_path: &Path,
) -> String {
    format!(
        "DataFrame Frame Surface Fixture\n\n\
         Namespace: dataframe::frame\n\n\
         00 Source Frame\n\
         artifact: {}\n\
         meaning: eager Frame as executable enclosure.\n\n\
         01 Schema\n\
         artifact: {}\n\
         meaning: names, dtypes, shape, memory, and chunk state.\n\n\
         02-06 Eager Transform Chain\n\
         enriched: {}\n\
         selected: {}\n\
         filtered: {}\n\
         renamed: {}\n\
         head: {}\n\
         meaning: Frame can transform eagerly without becoming Dataset mediation.\n",
        fixture_path(source_path),
        fixture_path(schema_path),
        fixture_path(enriched_path),
        fixture_path(selected_path),
        fixture_path(filtered_path),
        fixture_path(renamed_path),
        fixture_path(head_path),
    )
}

fn stage(number: u8, name: &str, doctrine: &str) {
    println!("-- Stage {number}: {name} --");
    println!("doctrine: {doctrine}");
}
