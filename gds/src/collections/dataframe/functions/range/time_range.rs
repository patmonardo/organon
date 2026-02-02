//! Time range helpers.

use polars::lazy::dsl::{time_range as pl_time_range, time_ranges as pl_time_ranges};
use polars::prelude::Expr;
use polars::time::{ClosedWindow, Duration};

pub fn time_range(start: Expr, end: Expr, interval: Duration, closed: ClosedWindow) -> Expr {
    pl_time_range(start, end, interval, closed)
}

pub fn time_ranges(start: Expr, end: Expr, interval: Duration, closed: ClosedWindow) -> Expr {
    pl_time_ranges(start, end, interval, closed)
}
