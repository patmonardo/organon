//! Collections Series example (eager Series operations).
//!
//! Run with:
//!   cargo run -p gds --example collections_series_basic

use gds::collections::dataframe::{
    col, series, GDSDataFrame, GDSFrameError, PolarsSortMultipleOptions,
};

fn main() -> Result<(), GDSFrameError> {
    // Build Series using the Collections helpers.
    let values = series("values", &[-3i64, -1, 2, 4]);
    let weights = series("weights", &[0.5, 1.0, 1.5, 2.0]);
    let mask = series("mask", &[true, false, true, true]);

    let df = GDSDataFrame::from_series(vec![values, weights, mask])?;

    println!("values:\n{}", df.select_columns(&["values"])?.fmt_table());
    println!("weights:\n{}", df.select_columns(&["weights"])?.fmt_table());

    // Basic operations (Series-like via expressions).
    let abs_values = df.select(&[col("values").abs().alias("abs_values")])?;
    println!("abs(values):\n{}", abs_values.fmt_table());

    let stats = df.select(&[
        col("values").sum().alias("sum_values"),
        col("weights").mean().alias("mean_weights"),
    ])?;
    println!("stats:\n{}", stats.fmt_table());

    // Filter using a boolean mask column.
    let filtered = df.filter_expr(col("mask"))?.select_columns(&["values"])?;
    println!("filtered(values):\n{}", filtered.fmt_table());

    // Sort and take head.
    let sorted = df
        .order_by_columns(&["values"], PolarsSortMultipleOptions::default())?
        .select_columns(&["values"])?;
    let head = sorted.head(2);
    println!("sorted(values):\n{}", sorted.fmt_table());
    println!("head(sorted, 2):\n{}", head.fmt_table());

    Ok(())
}
