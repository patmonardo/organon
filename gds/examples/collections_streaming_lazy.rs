//! Collections streaming example (LazyFrame-focused).
//!
//! Run with:
//!   cargo run -p gds --example collections_streaming_lazy

use gds::collections::dataframe::{GDSDataFrame, GDSFrameError, GDSLazyFrame, GDSStreamingFrame};
use gds::collections::extensions::streaming::{StreamingConfig, StreamingSupport};

fn main() -> Result<(), GDSFrameError> {
    let table = gds::tbl_def!(
        (id: i64 => [1, 2, 3, 4, 5]),
        (score: f64 => [10.0, 25.0, 40.0, 15.0, 30.0]),
    )?;

    let mut streaming = GDSStreamingFrame::from(table.into_inner());
    streaming.enable_streaming(StreamingConfig {
        enable_new_streaming: false,
    })?;

    // Build a LazyFrame pipeline.
    let lazy = GDSLazyFrame::new(streaming.stream_lazy())
        .select_exprs(vec![gds::col!(id), gds::col!(score)])
        .filter(gds::expr!(score > 20.0));

    let df = streaming.collect_streaming_lazy(lazy.into_lazyframe())?;
    println!(
        "Streaming LazyFrame result:\n{}",
        GDSDataFrame::new(df).fmt_table()
    );

    Ok(())
}
