//! Collections ordering/grouping example (expr-driven).
//!
//! Run with:
//!   cargo run -p gds --example collections_order_group_exprs

use gds::collections::dataframe::{
    col, lit, when, GDSFrameError, PolarsSortMultipleOptions, TableBuilder,
};

fn main() -> Result<(), GDSFrameError> {
    let table = TableBuilder::new()
        .with_i64_column("id", &[1, 2, 3, 4, 5, 6])
        .with_f64_column("score", &[10.0, 25.0, 40.0, 15.0, 30.0, 22.0])
        .with_f64_column("weight", &[1.1, 0.8, 1.5, 1.0, 1.2, 0.7])
        .with_string_column(
            "region",
            &[
                "NE".to_string(),
                "NE".to_string(),
                "SW".to_string(),
                "SW".to_string(),
                "NE".to_string(),
                "SW".to_string(),
            ],
        )
        .build()?;

    println!("Input table:\n{}", table.fmt_table());

    // Derive a weighted score and a banding column.
    let weighted = table.with_columns_exprs(&[
        (col("score") * col("weight")).alias("weighted_score"),
        when(col("score").gt(lit(20.0)))
            .then(lit(1.0))
            .otherwise(lit(0.0))
            .alias("is_high"),
    ])?;

    // Order by region (asc), then weighted_score (desc).
    let ordered = weighted.order_by_columns(
        &["region", "weighted_score"],
        PolarsSortMultipleOptions::new().with_order_descending_multi([false, true]),
    )?;
    println!(
        "Ordered by region, weighted_score desc:\n{}",
        ordered.fmt_table()
    );

    // Group by region and aggregate.
    let grouped = ordered.group_by_columns(
        &["region"],
        &[
            col("weighted_score").mean().alias("avg_weighted"),
            col("score").max().alias("max_score"),
            col("id").count().alias("rows"),
        ],
    )?;
    println!("Grouped by region:\n{}", grouped.fmt_table());

    Ok(())
}
