//! Datetime range helpers.

use polars::prelude::{Expr, TimeUnit, TimeZone};
use polars::time::{ClosedWindow, Duration};
use polars_plan::dsl::functions::{
    datetime_range as pl_datetime_range, datetime_ranges as pl_datetime_ranges,
};

pub fn datetime_range(
    start: Expr,
    end: Expr,
    interval: Duration,
    closed: ClosedWindow,
    time_unit: Option<TimeUnit>,
    time_zone: Option<TimeZone>,
) -> Expr {
    pl_datetime_range(
        Some(start),
        Some(end),
        Some(interval),
        None,
        closed,
        time_unit,
        time_zone,
    )
    .expect("datetime_range expression construction should be valid")
}

pub fn datetime_ranges(
    start: Expr,
    end: Expr,
    interval: Duration,
    closed: ClosedWindow,
    time_unit: Option<TimeUnit>,
    time_zone: Option<TimeZone>,
) -> Expr {
    pl_datetime_ranges(
        Some(start),
        Some(end),
        Some(interval),
        None,
        closed,
        time_unit,
        time_zone,
    )
    .expect("datetime_ranges expression construction should be valid")
}
