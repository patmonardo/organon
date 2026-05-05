//! Collections Expr example (Expr as first-class components).
//!
//! Run with:
//!   cargo run -p gds --example collections_expr_basic

use gds::collections::dataframe::GDSFrameError;

fn main() -> Result<(), GDSFrameError> {
    println!("== Collections Expr walkthrough ==");
    println!("Exprs are the reusable analytic grammar of the DataFrame layer.");

    let table = gds::tbl_def!(
        (id: i64 => [1, 2, 3, 4, 5]),
        (score: f64 => [10.0, 25.0, 40.0, 15.0, 30.0]),
    )?;

    println!("Input table:\n{}", table.fmt_table());

    // Exprs are first-class: build them independently, then reuse.
    let high_score = gds::expr!(score > 20.0);

    // Filter rows using a predicate expression.
    let filtered = table.filter_expr(high_score.clone())?;
    println!("Filtered (score > 20):\n{}", filtered.fmt_table());

    // Create a derived column using an expression.
    let tagged = table.with_columns(&[high_score.alias("is_high")])?;
    println!("With derived column (is_high):\n{}", tagged.fmt_table());

    // Group by and aggregate using expressions.
    let grouped = gds::group_by!(tagged, [is_high], [gds::agg!(score.mean => "mean_score")])?;
    println!("Grouped by is_high:\n{}", grouped.fmt_table());

    Ok(())
}
