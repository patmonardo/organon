//! RustScript array namespace example (Polars-first).
//!
//! Run with:
//!   cargo run -p gds --example collections_array_namespace_rustscript

use gds::collections::dataframe::{arr_ns, col, series_list_i64, SeriesModel};
use polars::prelude::*;

fn main() -> PolarsResult<()> {
    let vals = series_list_i64("vals", &[vec![1, 2, 3], vec![3, 2, 1]]);
    let tag0 = Series::new("".into(), &["x", "y"]);
    let tag1 = Series::new("".into(), &["a", "b"]);
    let tags = Series::new("tags".into(), &[tag0, tag1]);
    let idx = Series::new("idx".into(), [1i64, -1]);

    let df = DataFrame::new(vec![vals.clone().into(), tags.clone().into(), idx.into()])?
        .lazy()
        .with_columns([
            col("vals")
                .cast(DataType::Array(Box::new(DataType::Int64), 3))
                .alias("vals"),
            col("tags")
                .cast(DataType::Array(Box::new(DataType::String), 2))
                .alias("tags"),
        ])
        .collect()?;

    let result = df
        .clone()
        .lazy()
        .with_columns([
            arr_ns(col("vals")).len().alias("len"),
            arr_ns(col("vals")).sum().alias("sum"),
            arr_ns(col("vals")).sort(false, false, true).alias("sorted"),
            arr_ns(col("vals"))
                .get_expr(col("idx"), true)
                .alias("picked"),
            arr_ns(col("tags")).join("-", true).alias("joined"),
        ])
        .collect()?;

    println!("{result}");

    // SeriesModel + ArrayNameSpace (no DataFrame needed).
    let array_series = SeriesModel::new(vals.cast(&DataType::Array(Box::new(DataType::Int64), 3))?);
    let as_array = array_series.arr().slice(0, 2, true)?;

    println!("{as_array:?}");

    Ok(())
}
