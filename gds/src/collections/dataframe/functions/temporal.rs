//! Temporal constructor helpers.

use polars::lazy::dsl::{
    datetime as pl_datetime, duration as pl_duration, DatetimeArgs, DurationArgs,
};
use polars::prelude::{lit as pl_lit, DataType, Expr, TimeUnit, TimeZone};

pub fn datetime(
    year: Expr,
    month: Expr,
    day: Expr,
    hour: Option<Expr>,
    minute: Option<Expr>,
    second: Option<Expr>,
    microsecond: Option<Expr>,
    time_unit: Option<TimeUnit>,
    time_zone: Option<TimeZone>,
    ambiguous: Option<Expr>,
) -> Expr {
    let mut args = DatetimeArgs::new(year, month, day);
    if let Some(value) = hour {
        args = args.with_hour(value);
    }
    if let Some(value) = minute {
        args = args.with_minute(value);
    }
    if let Some(value) = second {
        args = args.with_second(value);
    }
    if let Some(value) = microsecond {
        args = args.with_microsecond(value);
    }
    if let Some(unit) = time_unit {
        args = args.with_time_unit(unit);
    }
    args = args.with_time_zone(time_zone);
    if let Some(value) = ambiguous {
        args = args.with_ambiguous(value);
    }
    pl_datetime(args)
}

pub fn date(year: Expr, month: Expr, day: Expr) -> Expr {
    pl_datetime(DatetimeArgs::new(year, month, day)).cast(DataType::Date)
}

pub fn time(
    hour: Option<Expr>,
    minute: Option<Expr>,
    second: Option<Expr>,
    microsecond: Option<Expr>,
) -> Expr {
    let mut args = DatetimeArgs::new(pl_lit(1970), pl_lit(1), pl_lit(1));
    if let Some(value) = hour {
        args = args.with_hour(value);
    }
    if let Some(value) = minute {
        args = args.with_minute(value);
    }
    if let Some(value) = second {
        args = args.with_second(value);
    }
    if let Some(value) = microsecond {
        args = args.with_microsecond(value);
    }
    pl_datetime(args).cast(DataType::Time)
}

pub fn duration(
    weeks: Option<Expr>,
    days: Option<Expr>,
    hours: Option<Expr>,
    minutes: Option<Expr>,
    seconds: Option<Expr>,
    milliseconds: Option<Expr>,
    microseconds: Option<Expr>,
    nanoseconds: Option<Expr>,
    time_unit: Option<TimeUnit>,
) -> Expr {
    let mut args = DurationArgs::new();
    if let Some(value) = weeks {
        args.weeks = value;
    }
    if let Some(value) = days {
        args.days = value;
    }
    if let Some(value) = hours {
        args.hours = value;
    }
    if let Some(value) = minutes {
        args.minutes = value;
    }
    if let Some(value) = seconds {
        args.seconds = value;
    }
    if let Some(value) = milliseconds {
        args.milliseconds = value;
    }
    if let Some(value) = microseconds {
        args.microseconds = value;
    }
    if let Some(value) = nanoseconds {
        args.nanoseconds = value;
    }
    if let Some(unit) = time_unit {
        args.time_unit = unit;
    }
    pl_duration(args)
}
