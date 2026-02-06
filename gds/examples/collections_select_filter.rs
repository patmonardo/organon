//! Collections select/filter example (eager DataFrame + Expr usage).
//!
//! Run with:
//!   cargo run -p gds --example collections_select_filter

use gds::collections::dataframe::prelude::*;
use gds::{agg, filter, group_by, order_by, select, select_columns, tbl_def};

fn main() -> Result<(), GDSFrameError> {
    let table = tbl_def!(
        (id: i64 => [1, 2, 3, 4, 5]),
        (score: f64 => [10.0, 25.0, 40.0, 15.0, 30.0]),
        (weight: f64 => [1.1, 0.8, 1.5, 1.0, 1.2]),
        (group: ["A", "B", "A", "B", "A"]),
    )?;

    println!("Input table:\n{}", table.fmt_table());

    // Eager select by column names.
    let selected = select_columns!(table, [id, score])?;
    println!("Selected columns:\n{}", selected.fmt_table());

    // Filter with an expression (score > 20).
    let filtered = filter!(table, score > 20.0)?;
    println!("Filtered rows (score > 20):\n{}", filtered.fmt_table());

    // Select using expressions (rename score -> score_x2).
    let projected = select!(table, id, (score * 2.0) as "score_x2")?;
    println!("Selected exprs (score_x2):\n{}", projected.fmt_table());

    // Order by score descending.
    let ordered = order_by!(table, [score], desc)?;
    println!("Ordered by score desc:\n{}", ordered.fmt_table());

    // Group by a column and aggregate (concise macro form).
    let grouped = group_by!(
        table,
        [group],
        [agg!(score.mean => "avg_score"), agg!(id.count => "rows")]
    )?;
    println!("Grouped by group:\n{}", grouped.fmt_table());

    Ok(())
}
