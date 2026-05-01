//! RustScript array namespace example (GDS DataFrame DSL).
//!
//! Run with:
//!   cargo run -p gds --example collections_array_namespace_rustscript

use gds::collections::dataframe::{
    arr_ns, series_list_i64, GDSDataFrame, GDSFrameError, GDSSeries,
};
use polars::prelude::*;

fn main() -> Result<(), GDSFrameError> {
    let vals = series_list_i64("vals", &[vec![1, 2, 3], vec![3, 2, 1]]);
    let tag0 = Series::new("".into(), &["x", "y"]);
    let tag1 = Series::new("".into(), &["a", "b"]);
    let tags = Series::new("tags".into(), &[tag0, tag1]);
    let idx = Series::new("idx".into(), [1i64, -1]);

    let df = GDSDataFrame::from_series(vec![vals.clone(), tags, idx])?.with_columns(&[
        gds::col!(vals)
            .cast(DataType::Array(Box::new(DataType::Int64), 3))
            .alias("vals"),
        gds::col!(tags)
            .cast(DataType::Array(Box::new(DataType::String), 2))
            .alias("tags"),
    ])?;

    let result = df.with_columns(&[
        arr_ns(gds::col!(vals)).len().alias("len"),
        arr_ns(gds::col!(vals)).sum().alias("sum"),
        arr_ns(gds::col!(vals)).sort(false, false).alias("sorted"),
        arr_ns(gds::col!(vals))
            .get_expr(gds::col!(idx), true)
            .alias("picked"),
        arr_ns(gds::col!(tags)).join("-", true).alias("joined"),
    ])?;

    println!("{}", result.fmt_table());

    // GDSSeries + ArrayNameSpace (no DataFrame needed).
    let array_series = GDSSeries::new(vals.cast(&DataType::Array(Box::new(DataType::Int64), 3))?);
    let as_array = array_series.arr().slice(0, 2, true)?;

    println!("{as_array:?}");

    Ok(())
}
