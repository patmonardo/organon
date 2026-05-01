//! Collections Series example (eager Series operations).
//!
//! Run with:
//!   cargo run -p gds --example collections_series_basic

use gds::collections::dataframe::{series, GDSDataFrame, GDSFrameError};

fn main() -> Result<(), GDSFrameError> {
    // Build Series using the Collections helpers.
    let values = series("values", &[-3i64, -1, 2, 4]);
    let weights = series("weights", &[0.5, 1.0, 1.5, 2.0]);
    let mask = series("mask", &[true, false, true, true]);

    let df = GDSDataFrame::from_series(vec![values, weights, mask])?;

    println!("values:\n{}", gds::sc!(df, values)?.fmt_table());
    println!("weights:\n{}", gds::sc!(df, weights)?.fmt_table());

    // Basic operations (Series-like via expressions).
    let abs_values = df.select(&[gds::col!(values).abs().alias("abs_values")])?;
    println!("abs(values):\n{}", abs_values.fmt_table());

    let stats = df.select(&[
        gds::col!(values).sum().alias("sum_values"),
        gds::col!(weights).mean().alias("mean_weights"),
    ])?;
    println!("stats:\n{}", stats.fmt_table());

    // Filter using a boolean mask column.
    let filtered = gds::where_!(df, mask)?.select_columns(&gds::selector![values])?;
    println!("filtered(values):\n{}", filtered.fmt_table());

    // Sort and take head.
    let sorted = gds::arrange!(df, [values])?.select_columns(&gds::selector![values])?;
    let head = sorted.head(2);
    println!("sorted(values):\n{}", sorted.fmt_table());
    println!("head(sorted, 2):\n{}", head.fmt_table());

    Ok(())
}
