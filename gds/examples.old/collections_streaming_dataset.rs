//! Streaming Dataset example (Dataset + LazyFrame pipeline).
//!
//! Run with:
//!   cargo run -p gds --example collections_streaming_dataset --features dataset
#![cfg(feature = "dataset")]

use gds::collections::dataframe::{GDSDataFrame, GDSFrameError};
use gds::collections::dataset::{Dataset, StreamingDataset};
use gds::collections::extensions::streaming::StreamingConfig;

fn main() -> Result<(), GDSFrameError> {
    println!("== Streaming Dataset walkthrough ==");
    println!("A Dataset is the semantic source; streaming exposes its analytic body in motion.");

    let table = gds::tbl_def!(
        (id: i64 => [1, 2, 3, 4, 5, 6, 7, 8]),
        (score: f64 => [10.0, 25.0, 40.0, 15.0, 30.0, 22.0, 12.0, 35.0]),
        (weight: f64 => [1.1, 0.8, 1.5, 1.0, 1.2, 0.7, 1.3, 0.9]),
    )?;
    let dataset = Dataset::new(table);

    println!("Dataset rows: {}", dataset.row_count());
    println!("Dataset columns: {:?}", dataset.column_names());

    // Build a streaming dataset with a LazyFrame transform.
    let streaming_summary = StreamingDataset::with_config(
        dataset.clone(),
        3,
        StreamingConfig {
            enable_new_streaming: false,
        },
    )
    .with_transform(|lazy| {
        lazy.with_columns([(gds::col!(score) * gds::col!(weight)).alias("weighted_score")])
            .with_columns([gds::when!(gds::expr!(score > 20.0))
                .then(gds::lit!(1.0))
                .otherwise(gds::lit!(0.0))
                .alias("is_high")])
            .filter(gds::expr!(score > 12.0))
            .group_by([gds::col!(is_high)])
            .agg([
                gds::col!(weighted_score).mean().alias("avg_weighted"),
                gds::col!(id).count().alias("rows"),
            ])
            .sort_by_exprs([gds::col!(is_high)], Default::default())
    });

    let summary = streaming_summary
        .iter()
        .next()
        .ok_or_else(|| "missing summary batch".to_string())??;
    println!(
        "Streaming semantic summary (derived view):\n{}",
        GDSDataFrame::new(summary).fmt_table()
    );

    // Stream raw batches (no transform) for inspection.
    let streaming_batches = StreamingDataset::new(dataset, 3);
    for (index, batch) in streaming_batches.iter().enumerate() {
        let batch_df = batch?;
        let offset = index * streaming_batches.batch_size();
        println!(
            "Batch @{} (analytic body slice):\n{}",
            offset,
            GDSDataFrame::new(batch_df).fmt_table()
        );
    }

    Ok(())
}
