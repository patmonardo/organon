//! RustScript list namespace example (GDS DataFrame DSL).
//!
//! Run with:
//!   cargo run -p gds --example collections_list_namespace_rustscript

use gds::collections::dataframe::{
    list_ns, series_list_i64, GDSDataFrame, GDSFrameError, GDSSeries,
};

fn main() -> Result<(), GDSFrameError> {
    // DataFrame + ExprList facade (Pythonic list ops).
    let values = series_list_i64("values", &[vec![1, 2, 3], vec![3, 3, 2], vec![]]);
    let df = GDSDataFrame::from_series(vec![values.clone()])?;

    let result = gds::mutate!(
        df,
        len = { list_ns(gds::col!(values)).len() },
        sum = { list_ns(gds::col!(values)).sum() },
        unique = { list_ns(gds::col!(values)).unique(true) },
        first = { list_ns(gds::col!(values)).get(0, true) },
        has_2 = { list_ns(gds::col!(values)).contains(2_i64, true) },
    )?;

    println!("{}", result.fmt_table());

    // GDSSeries + ListNameSpace (no DataFrame needed).
    let series = GDSSeries::from_list_i64("values", &[vec![1, 2, 3], vec![3, 3, 2], vec![]]);
    let lengths = series.list().len()?;
    let uniques = series.list().unique(true)?;

    println!("{lengths:?}");
    println!("{uniques:?}");

    Ok(())
}
