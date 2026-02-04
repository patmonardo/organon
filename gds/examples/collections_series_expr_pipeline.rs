//! Series/Expr pipeline example.
//!
//! Run with:
//!   cargo run -p gds --example collections_series_expr_pipeline

use gds::collections::dataframe::{col, series, series_expr, GDSDataFrame, GDSFrameError};

fn main() -> Result<(), GDSFrameError> {
    let s = series("text", &["ax", "by", "cx"]);
    let contains_x = series_expr(s).str().apply(|e| e.ends_with("x"))?;
    println!("contains_x = {:?}", contains_x);

    let nums = series("nums", &[1, 2, 3]);
    let total = series_expr(nums.clone()).apply(|e| e.into_expr().sum())?;
    println!("sum = {:?}", total);

    let avg = series_expr(nums).apply(|e| e.into_expr().mean())?;
    println!("mean = {:?}", avg);

    let a = series("a", &[1, 2, 3]);
    let b = series("b", &[10, 20, 30]);
    let df = GDSDataFrame::from_series(vec![a, b])?;
    let result = df.with_columns(&[
        (col("a") + col("b")).alias("sum_ab"),
        (col("a") * col("b")).alias("prod_ab"),
    ])?;
    println!("expr_result = {}", result.fmt_table());

    Ok(())
}
