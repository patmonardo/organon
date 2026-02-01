//! Collections Expr example (Expr as first-class components).
//!
//! Run with:
//!   cargo run -p gds --example collections_expr_basic

use gds::collections::dataframe::{col, lit, when, TableBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build a DataFrame via the Collections facade.
    let table = TableBuilder::new()
        .with_i64_column("id", &[1, 2, 3, 4, 5])
        .with_f64_column("score", &[10.0, 25.0, 40.0, 15.0, 30.0])
        .build()?;

    println!("Input table:\n{}", table.fmt_table());

    // Exprs are first-class: build them independently, then reuse.
    let score_col = col("score");
    let high_score = score_col.clone().gt(lit(20.0));

    // Filter rows using a predicate expression.
    let filtered = table.filter_expr(high_score.clone())?;
    println!("Filtered (score > 20):\n{}", filtered.fmt_table());

    // Create a derived column using an expression.
    let tagged = table.with_columns_exprs(&[when(high_score)
        .then(lit(1.0))
        .otherwise(lit(0.0))
        .alias("is_high")])?;
    println!("With derived column (is_high):\n{}", tagged.fmt_table());

    // Group by and aggregate using expressions.
    let grouped =
        tagged.group_by_columns(&["is_high"], &[col("score").mean().alias("mean_score")])?;
    println!("Grouped by is_high:\n{}", grouped.fmt_table());

    Ok(())
}
