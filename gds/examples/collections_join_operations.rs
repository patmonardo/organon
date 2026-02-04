//! Polars join operations (Collections example).
//!
//! Run with:
//!   cargo run -p gds --example collections_join_operations

use gds::collections::dataframe::{series, GDSDataFrame, GDSFrameError, JoinType};

fn build_left() -> Result<GDSDataFrame, GDSFrameError> {
    Ok(GDSDataFrame::from_series(vec![
        series("id", &[1i64, 2, 3]),
        series("left", &["l1", "l2", "l3"]),
    ])?)
}

fn build_right() -> Result<GDSDataFrame, GDSFrameError> {
    Ok(GDSDataFrame::from_series(vec![
        series("id", &[2i64, 3, 4]),
        series("right", &["r2", "r3", "r4"]),
    ])?)
}

fn main() -> Result<(), GDSFrameError> {
    let left = build_left()?;
    let right = build_right()?;

    let inner = left.join_on(&right, &["id"], JoinType::Inner, None)?;
    println!("inner\n{}\n", inner.fmt_table());

    let left_join = left.join_on(&right, &["id"], JoinType::Left, None)?;
    println!("left\n{}\n", left_join.fmt_table());

    let right_join = left.join_on(&right, &["id"], JoinType::Right, None)?;
    println!("right\n{}\n", right_join.fmt_table());

    let full = left.join_on(&right, &["id"], JoinType::Full, None)?;
    println!("full\n{}\n", full.fmt_table());

    let semi = left.join_on(&right, &["id"], JoinType::Semi, None)?;
    println!("semi\n{}\n", semi.fmt_table());

    Ok(())
}
