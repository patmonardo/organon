//! Dataset Streaming Procedure fixture.
//!
//! Exercises StreamingDataset and StreamingBatchIter: batch iteration over
//! a Dataset, batch-size configuration, and Plan-driven transform.
//!
//! Run with:
//!   cargo run -p gds --example dataset_streaming_procedure

use std::fs;
use std::path::{Path, PathBuf};

use gds::collections::dataset::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Dataset Streaming Procedure ==");

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    // ------------------------------------------------------------------ Stage 0
    stage(
        0,
        "Source Dataset",
        "A named Dataset backs the streaming iterator.",
    );

    // Build a small dataset with 10 rows.
    let frame = gds::tbl_def!(
        (id: i64 => [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
        (value: ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"]),
    )?;
    let dataset = Dataset::named("streaming-demo", frame);

    let row_count = dataset.row_count();
    println!("rows: {}", row_count);

    let source_path = fixture_root.join("00-source.csv");
    dataset.table().write_csv(&path_string(&source_path))?;
    println!("persisted: {}", fixture_path(&source_path));
    println!();

    // ------------------------------------------------------------------ Stage 1
    stage(
        1,
        "Batch Iteration",
        "StreamingDataset partitions rows into fixed-size batches.",
    );

    let batch_size = 3;
    let streaming = StreamingDataset::new(dataset.clone(), batch_size);
    println!("batch size: {}", streaming.batch_size());
    println!("source rows: {}", streaming.row_count());

    let mut batch_shapes: Vec<(usize, usize)> = Vec::new();
    for (i, batch_result) in streaming.iter().enumerate() {
        let batch = batch_result?;
        let shape = (batch.height(), batch.width());
        println!("  batch {}: {} rows x {} cols", i, shape.0, shape.1);
        batch_shapes.push(shape);
    }

    let total_batched: usize = batch_shapes.iter().map(|(r, _)| r).sum();
    println!("total rows batched: {}", total_batched);

    let iter_path = fixture_root.join("01-iteration.txt");
    fs::write(
        &iter_path,
        format!(
            "batch_size: {}\nsource_rows: {}\nbatches: {}\ntotal_batched: {}\nbatch_shapes:\n{}\n",
            batch_size,
            streaming.row_count(),
            batch_shapes.len(),
            total_batched,
            batch_shapes
                .iter()
                .enumerate()
                .map(|(i, (r, c))| format!("  {i}: {r}r x {c}c"))
                .collect::<Vec<_>>()
                .join("\n"),
        ),
    )?;
    println!("persisted: {}", fixture_path(&iter_path));
    println!();

    // ------------------------------------------------------------------ Stage 2
    stage(
        2,
        "Plan-Driven Streaming",
        "StreamingDataset::with_plan reads Batch hint from the Plan to override batch size.",
    );

    // Build a Plan that carries a Batch(4) hint — with_plan extracts it.
    let plan = Plan::new(Source::Value(dataset.clone())).push_step(Step::Batch(4));

    let planned = StreamingDataset::new(dataset.clone(), 3).with_plan(&plan);
    println!("batch size (plan hint): {}", planned.batch_size());

    let mut plan_shapes: Vec<(usize, usize)> = Vec::new();
    for (i, batch_result) in planned.iter().enumerate() {
        let batch = batch_result?;
        let shape = (batch.height(), batch.width());
        println!("  batch {}: {} rows x {} cols", i, shape.0, shape.1);
        plan_shapes.push(shape);
    }

    let plan_path = fixture_root.join("02-plan-streaming.txt");
    fs::write(
        &plan_path,
        format!(
            "plan_batch_size: {}\nbatches: {}\nbatch_shapes:\n{}\n",
            planned.batch_size(),
            plan_shapes.len(),
            plan_shapes
                .iter()
                .enumerate()
                .map(|(i, (r, c))| format!("  {i}: {r}r x {c}c"))
                .collect::<Vec<_>>()
                .join("\n"),
        ),
    )?;
    println!("persisted: {}", fixture_path(&plan_path));

    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(&source_path, &iter_path, &plan_path),
    )?;
    println!("\nmanifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/collections/dataset/dataset_streaming_procedure")
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/dataset/dataset_streaming_procedure/{file_name}")
}

fn path_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn manifest(source_path: &Path, iter_path: &Path, plan_path: &Path) -> String {
    format!(
        "Dataset Streaming Procedure Fixture\n\n\
         Namespace: dataset::streaming\n\n\
         00 Source\n\
         artifact: {}\n\
         meaning: 10-row named Dataset used as streaming source.\n\n\
         01 Iteration\n\
         artifact: {}\n\
         meaning: StreamingDataset with batch_size=3, full row coverage.\n\n\
         02 Plan Streaming\n\
         artifact: {}\n\
         meaning: with_plan drives batch size and Select transform per batch.\n",
        fixture_path(source_path),
        fixture_path(iter_path),
        fixture_path(plan_path),
    )
}

fn stage(number: u8, name: &str, doctrine: &str) {
    println!("-- Stage {number}: {name} --");
    println!("doctrine: {doctrine}");
}
