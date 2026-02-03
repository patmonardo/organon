//! RustScript list namespace example (Polars-first).
//!
//! Run with:
//!   cargo run -p gds --example collections_list_namespace_rustscript

use gds::collections::dataframe::{
    col, list_ns, series_list_i64, PolarsDataFrameCollection, SeriesModel,
};
use polars::prelude::*;

fn main() -> PolarsResult<()> {
    // DataFrame + ExprList facade (Pythonic list ops).
    let values = series_list_i64("values", &[vec![1, 2, 3], vec![3, 3, 2], vec![]]);
    let df = DataFrame::new(vec![values.clone().into()])?;

    let result = df
        .clone()
        .lazy()
        .with_columns([
            list_ns(col("values")).len().alias("len"),
            list_ns(col("values")).sum().alias("sum"),
            list_ns(col("values")).unique(true).alias("unique"),
            list_ns(col("values")).get(0, true).alias("first"),
            list_ns(col("values")).contains(2_i64, true).alias("has_2"),
        ])
        .collect()?;

    println!("{}", PolarsDataFrameCollection::new(result).fmt_table());

    // SeriesModel + ListNameSpace (no DataFrame needed).
    let series = SeriesModel::from_list_i64("values", &[vec![1, 2, 3], vec![3, 3, 2], vec![]]);
    let lengths = series.list().len()?;
    let uniques = series.list().unique(true)?;

    println!("{lengths:?}");
    println!("{uniques:?}");

    Ok(())
}
