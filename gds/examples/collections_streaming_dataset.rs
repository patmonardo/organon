//! Streaming Dataset example (Dataset + LazyFrame pipeline).
//!
//! Run with:
//!   cargo run -p gds --example collections_streaming_dataset

use gds::collections::dataframe::{col, lit, when, GDSDataFrame, GDSFrameError, TableBuilder};
use gds::collections::dataset::{Dataset, StreamingDataset};
use gds::collections::extensions::streaming::StreamingConfig;

fn main() -> Result<(), GDSFrameError> {
    // Build a dataset (Polars-backed).
    let dataset = Dataset::from_builder(
        TableBuilder::new()
            .with_i64_column("id", &[1, 2, 3, 4, 5, 6, 7, 8])
            .with_f64_column("score", &[10.0, 25.0, 40.0, 15.0, 30.0, 22.0, 12.0, 35.0])
            .with_f64_column("weight", &[1.1, 0.8, 1.5, 1.0, 1.2, 0.7, 1.3, 0.9]),
    )?;

    println!("Dataset rows: {}", dataset.row_count());

    // Build a streaming dataset with a LazyFrame transform.
    let streaming_summary = StreamingDataset::with_config(
        dataset.clone(),
        3,
        StreamingConfig {
            enable_new_streaming: false,
        },
    )
    .with_transform(|lazy| {
        lazy.with_columns([(col("score") * col("weight")).alias("weighted_score")])
            .with_columns([when(col("score").gt(lit(20.0)))
                .then(lit(1.0))
                .otherwise(lit(0.0))
                .alias("is_high")])
            .filter(col("score").gt(lit(12.0)))
            .group_by([col("is_high")])
            .agg([
                col("weighted_score").mean().alias("avg_weighted"),
                col("id").count().alias("rows"),
            ])
            .sort_by_exprs([col("is_high")], Default::default())
    });

    let summary = streaming_summary
        .iter()
        .next()
        .ok_or_else(|| "missing summary batch".to_string())??;
    println!(
        "Streaming dataset summary:\n{}",
        GDSDataFrame::new(summary).fmt_table()
    );

    // Stream raw batches (no transform) for inspection.
    let streaming_batches = StreamingDataset::new(dataset, 3);
    for (index, batch) in streaming_batches.iter().enumerate() {
        let batch_df = batch?;
        let offset = index * streaming_batches.batch_size();
        println!(
            "Batch @{}:\n{}",
            offset,
            GDSDataFrame::new(batch_df).fmt_table()
        );
    }

    Ok(())
}
