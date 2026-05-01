//! Collections ordering/grouping example (expr-driven).
//!
//! Run with:
//!   cargo run -p gds --example collections_order_group_exprs

use gds::collections::dataframe::{GDSFrameError, PolarsSortMultipleOptions};

fn main() -> Result<(), GDSFrameError> {
    println!("== Collections ordering/grouping walkthrough ==");
    println!("This is the kind of entity-analysis body the Dataset can drive into view for downstream inspection.");

    let table = gds::tbl_def!(
        (id: i64 => [1, 2, 3, 4, 5, 6]),
        (score: f64 => [10.0, 25.0, 40.0, 15.0, 30.0, 22.0]),
        (weight: f64 => [1.1, 0.8, 1.5, 1.0, 1.2, 0.7]),
        (region: ["NE", "NE", "SW", "SW", "NE", "SW"]),
    )?;

    println!("Input table:\n{}", table.fmt_table());

    // Derive a weighted score and a banding column.
    let weighted = gds::mutate!(
        table,
        weighted_score = (score * weight),
        is_high = (score > 20.0),
    )?;

    // Order by region (asc), then weighted_score (desc).
    let ordered = weighted.order_by_columns(
        &gds::selector![region, weighted_score],
        PolarsSortMultipleOptions::new().with_order_descending_multi([false, true]),
    )?;
    println!(
        "Ordered by region, weighted_score desc:\n{}",
        ordered.fmt_table()
    );

    // Group by region and aggregate.
    let grouped = gds::group_by!(
        ordered,
        [region],
        [
            gds::agg!(weighted_score.mean => "avg_weighted"),
            gds::agg!(score.max => "max_score"),
            gds::agg!(id.count => "rows")
        ]
    )?;
    println!("Grouped by region:\n{}", grouped.fmt_table());

    Ok(())
}
