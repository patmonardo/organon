//! Dataset Streaming Lazy fixture.
//!
//! Exercises lazy transform application inside StreamingDataset batches.
//!
//! Run with:
//!   cargo run -p gds --example dataset_streaming_lazy

use std::fs;
use std::path::{Path, PathBuf};

use gds::collections::dataframe::{col, lit};
use gds::collections::dataset::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Dataset Streaming Lazy ==");

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    // ------------------------------------------------------------------ Stage 0
    stage(
        0,
        "Source Dataset",
        "Prepare a dataset that will be consumed by streaming lazy batches.",
    );

    let frame = gds::tbl_def!(
        (id: i64 => [1, 2, 3, 4, 5, 6, 7, 8]),
        (label: ["a", "b", "c", "d", "e", "f", "g", "h"]),
        (score: i64 => [5, 10, 15, 20, 25, 30, 35, 40]),
    )?;
    let dataset = Dataset::named("streaming-lazy", frame);

    let source_path = fixture_root.join("00-source.csv");
    dataset.table().write_csv(&path_string(&source_path))?;
    println!("source rows: {}", dataset.row_count());
    println!("persisted: {}", fixture_path(&source_path));
    println!();

    // ------------------------------------------------------------------ Stage 1
    stage(
        1,
        "Lazy Transform",
        "Attach a lazy transform closure applied independently per batch.",
    );

    let streaming = StreamingDataset::new(dataset.clone(), 3).with_transform(|lf| {
        lf.with_columns(vec![(col("score") + lit(1)).alias("score_plus_one")])
            .filter(col("score_plus_one").gt(lit(20)))
            .select(vec![col("id"), col("label"), col("score_plus_one")])
    });

    let transform_path = fixture_root.join("01-transform.txt");
    fs::write(
        &transform_path,
        format!(
            "batch_size={}\nrow_count={}\ntransform=with_columns+filter+select\n",
            streaming.batch_size(),
            streaming.row_count(),
        ),
    )?;
    println!("batch size: {}", streaming.batch_size());
    println!("persisted: {}", fixture_path(&transform_path));
    println!();

    // ------------------------------------------------------------------ Stage 2
    stage(
        2,
        "Streaming Collect",
        "Execute transformed batches and summarize resulting batch shapes.",
    );

    let mut total_rows = 0usize;
    let mut shapes: Vec<(usize, usize)> = Vec::new();
    for batch in streaming.iter() {
        let batch = batch?;
        total_rows += batch.height();
        shapes.push((batch.height(), batch.width()));
    }

    let collect_path = fixture_root.join("02-batches.txt");
    fs::write(
        &collect_path,
        format!(
            "batch_count={}\ntotal_rows_after_filter={}\nshapes={:?}\n",
            shapes.len(),
            total_rows,
            shapes,
        ),
    )?;
    println!("batches: {}", shapes.len());
    println!("rows after lazy filter: {}", total_rows);
    println!("persisted: {}", fixture_path(&collect_path));

    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(&source_path, &transform_path, &collect_path),
    )?;
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/collections/dataset/dataset_streaming_lazy")
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/dataset/dataset_streaming_lazy/{file_name}")
}

fn path_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn manifest(source_path: &Path, transform_path: &Path, collect_path: &Path) -> String {
    format!(
        "Dataset Streaming Lazy Fixture\n\n\
         Namespace: dataset::streaming::lazy\n\n\
         00 Source\n\
         artifact: {}\n\
         meaning: source dataset for batch-wise lazy evaluation.\n\n\
         01 Transform\n\
         artifact: {}\n\
         meaning: attached lazy transform closure for each batch.\n\n\
         02 Batches\n\
         artifact: {}\n\
         meaning: post-transform batch execution summary.\n",
        fixture_path(source_path),
        fixture_path(transform_path),
        fixture_path(collect_path),
    )
}

fn stage(number: u8, name: &str, doctrine: &str) {
    println!("-- Stage {number}: {name} --");
    println!("doctrine: {doctrine}");
}
