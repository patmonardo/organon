//! Series/Expr pipeline example.
//!
//! Run with:
//!   cargo run -p gds --example collections_series_expr_pipeline

use gds::collections::dataframe::{lit, series, series_expr};
use polars::error::PolarsError;
use polars::lazy::dsl::{cum_reduce_exprs, SpecialEq};
use polars::prelude::{col, Column, DataFrame, IntoLazy, PlanCallback};
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = series("text", &["ax", "by", "cx"]);
    let contains_x = series_expr(s)
        .str_()
        .apply(|e| e.ns().ends_with(lit("x")))?;
    println!("contains_x = {:?}", contains_x);

    let nums = series("nums", &[1, 2, 3]);
    let total = series_expr(nums.clone()).apply(|e| e.expr().clone().sum())?;
    println!("sum = {:?}", total);

    let avg = series_expr(nums).apply(|e| e.expr().clone().mean())?;
    println!("mean = {:?}", avg);

    let a = series("a", &[1, 2, 3]);
    let b = series("b", &[10, 20, 30]);
    let df = DataFrame::new(vec![Column::from(a), Column::from(b)])?;
    let callback = PlanCallback::Rust(SpecialEq::new(Arc::new(|(lhs, rhs)| &lhs + &rhs)));
    let cum = df
        .lazy()
        .select([cum_reduce_exprs(
            callback,
            [col("a"), col("b")],
            false,
            None,
        )])
        .collect()?
        .select_at_idx(0)
        .map(|column| column.as_materialized_series().clone())
        .ok_or_else(|| PolarsError::ComputeError("cum_reduce produced no output".into()))?;
    println!("cum_reduce_sum = {:?}", cum);

    Ok(())
}
