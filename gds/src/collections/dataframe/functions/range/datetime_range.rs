//! Datetime range helpers.

use polars::lazy::dsl::{
    datetime_range as pl_datetime_range, datetime_ranges as pl_datetime_ranges,
};
use polars::prelude::{Expr, TimeUnit, TimeZone};
use polars::time::{ClosedWindow, Duration};

pub fn datetime_range(
    start: Expr,
    end: Expr,
    interval: Duration,
    closed: ClosedWindow,
    time_unit: Option<TimeUnit>,
    time_zone: Option<TimeZone>,
) -> Expr {
    pl_datetime_range(start, end, interval, closed, time_unit, time_zone)
}

pub fn datetime_ranges(
    start: Expr,
    end: Expr,
    interval: Duration,
    closed: ClosedWindow,
    time_unit: Option<TimeUnit>,
    time_zone: Option<TimeZone>,
) -> Expr {
    pl_datetime_ranges(start, end, interval, closed, time_unit, time_zone)
}
