//! Date range helpers.

use polars::lazy::dsl::{date_range as pl_date_range, date_ranges as pl_date_ranges};
use polars::prelude::Expr;
use polars::time::{ClosedWindow, Duration};

pub fn date_range(start: Expr, end: Expr, interval: Duration, closed: ClosedWindow) -> Expr {
    pl_date_range(start, end, interval, closed)
}

pub fn date_ranges(start: Expr, end: Expr, interval: Duration, closed: ClosedWindow) -> Expr {
    pl_date_ranges(start, end, interval, closed)
}
