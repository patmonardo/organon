//! Collections streaming example (LazyFrame-focused).
//!
//! Run with:
//!   cargo run -p gds --example collections_streaming_lazy

use gds::collections::dataframe::{
    col, lit, GDSDataFrame, PolarsStreamingFrame, TableBuilder,
};
use gds::collections::extensions::streaming::{StreamingConfig, StreamingSupport};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let table = TableBuilder::new()
        .with_i64_column("id", &[1, 2, 3, 4, 5])
        .with_f64_column("score", &[10.0, 25.0, 40.0, 15.0, 30.0])
        .build()?;

    let mut streaming = PolarsStreamingFrame::from(table.into_inner());
    streaming.enable_streaming(StreamingConfig {
        enable_new_streaming: false,
    })?;

    // Build a LazyFrame pipeline.
    let lazy = streaming
        .stream_lazy()
        .select([col("id"), col("score")])
        .filter(col("score").gt(lit(20.0)));

    let df = streaming.collect_streaming_lazy(lazy)?;
    println!(
        "Streaming LazyFrame result:\n{}",
        GDSDataFrame::new(df).fmt_table()
    );

    Ok(())
}
