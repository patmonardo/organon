//! Collections join operations (DataFrame DSL example).
//!
//! Run with:
//!   cargo run -p gds --example collections_join_operations

use gds::collections::dataframe::{GDSDataFrame, GDSFrameError, JoinType};

fn build_left() -> Result<GDSDataFrame, GDSFrameError> {
    Ok(gds::tbl_def!(
        (id: i64 => [1, 2, 3]),
        (left: ["l1", "l2", "l3"]),
    )?)
}

fn build_right() -> Result<GDSDataFrame, GDSFrameError> {
    Ok(gds::tbl_def!(
        (id: i64 => [2, 3, 4]),
        (right: ["r2", "r3", "r4"]),
    )?)
}

fn main() -> Result<(), GDSFrameError> {
    println!("== Collections join walkthrough ==");
    println!("Joins show how multiple analytic bodies can be brought into relation inside the Collections layer.");

    let left = build_left()?;
    let right = build_right()?;

    let inner = gds::join!(left, right, on = [id], how = inner)?;
    println!("inner\n{}\n", inner.fmt_table());

    let left_join = gds::join!(left, right, on = [id], how = left)?;
    println!("left\n{}\n", left_join.fmt_table());

    let right_join = gds::join!(left, right, on = [id], how = right)?;
    println!("right\n{}\n", right_join.fmt_table());

    let full = gds::join!(left, right, on = [id], how = full)?;
    println!("full\n{}\n", full.fmt_table());

    let semi = left.join_on(&right, &["id"], JoinType::Semi, None)?;
    println!("semi\n{}\n", semi.fmt_table());

    Ok(())
}
