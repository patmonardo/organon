//! Datetime namespace for SeriesModel (py-polars inspired).

use polars::prelude::{Expr, NonExistent, PolarsResult, Series, TimeUnit, TimeZone};

use crate::collections::dataframe::expr::SeriesExprDateTime;
use crate::collections::dataframe::expressions::datetime::ExprDateTime;

#[derive(Debug, Clone)]
pub struct DateTimeNameSpace {
    series: Series,
}

impl DateTimeNameSpace {
    pub fn new(series: Series) -> Self {
        Self { series }
    }

    pub fn series(&self) -> &Series {
        &self.series
    }

    pub fn into_series(self) -> Series {
        self.series
    }

    fn apply_expr<F>(&self, f: F) -> PolarsResult<Series>
    where
        F: FnOnce(ExprDateTime) -> Expr,
    {
        SeriesExprDateTime::new(self.series.clone()).apply(f)
    }

    pub fn add_business_days(
        &self,
        n: i64,
        week_mask: [bool; 7],
        holidays: Vec<i32>,
        roll: polars::prelude::Roll,
    ) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.add_business_days(n, week_mask, holidays, roll))
    }

    pub fn add_business_days_expr(
        &self,
        n: Expr,
        week_mask: [bool; 7],
        holidays: Vec<i32>,
        roll: polars::prelude::Roll,
    ) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.add_business_days_expr(n, week_mask, holidays, roll))
    }

    pub fn to_string(&self, format: &str) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.to_string(format))
    }

    pub fn strftime(&self, format: &str) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.strftime(format))
    }

    pub fn cast_time_unit(&self, time_unit: TimeUnit) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.cast_time_unit(time_unit))
    }

    pub fn with_time_unit(&self, time_unit: TimeUnit) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.with_time_unit(time_unit))
    }

    pub fn convert_time_zone(&self, time_zone: TimeZone) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.convert_time_zone(time_zone))
    }

    pub fn replace_time_zone(
        &self,
        time_zone: Option<TimeZone>,
        ambiguous: Expr,
        non_existent: NonExistent,
    ) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.replace_time_zone(time_zone, ambiguous, non_existent))
    }

    pub fn millennium(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.millennium())
    }

    pub fn century(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.century())
    }

    pub fn year(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.year())
    }

    pub fn is_business_day(
        &self,
        week_mask: [bool; 7],
        holidays: Vec<i32>,
    ) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.is_business_day(week_mask, holidays))
    }

    pub fn is_leap_year(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.is_leap_year())
    }

    pub fn iso_year(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.iso_year())
    }

    pub fn month(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.month())
    }

    pub fn days_in_month(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.days_in_month())
    }

    pub fn quarter(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.quarter())
    }

    pub fn week(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.week())
    }

    pub fn weekday(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.weekday())
    }

    pub fn day(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.day())
    }

    pub fn ordinal_day(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.ordinal_day())
    }

    pub fn time(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.time())
    }

    pub fn date(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.date())
    }

    pub fn datetime(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.datetime())
    }

    pub fn hour(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.hour())
    }

    pub fn minute(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.minute())
    }

    pub fn second(&self, fractional: bool) -> PolarsResult<Series> {
        if fractional {
            self.second_fractional()
        } else {
            self.apply_expr(|expr| expr.second())
        }
    }

    pub fn second_fractional(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.second_fractional())
    }

    pub fn millisecond(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.millisecond())
    }

    pub fn microsecond(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.microsecond())
    }

    pub fn nanosecond(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.nanosecond())
    }

    pub fn timestamp(&self, time_unit: TimeUnit) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.timestamp(time_unit))
    }

    pub fn timestamp_ms(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.timestamp_ms())
    }

    pub fn timestamp_us(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.timestamp_us())
    }

    pub fn timestamp_ns(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.timestamp_ns())
    }

    pub fn epoch(&self, time_unit: TimeUnit) -> PolarsResult<Series> {
        // Python exposes `dt.epoch(time_unit)`; forward to timestamp
        self.apply_expr(|expr| expr.timestamp(time_unit))
    }

    pub fn truncate(&self, every: &str) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.truncate(every))
    }

    pub fn truncate_expr(&self, every: Expr) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.truncate_expr(every))
    }

    pub fn round(&self, every: &str) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.round(every))
    }

    pub fn round_expr(&self, every: Expr) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.round_expr(every))
    }

    // pub fn offset_by(&self, by: &str) -> PolarsResult<Series> {
    //     self.apply_expr(|expr| expr.offset_by(by))
    // }

    pub fn combine(&self, time: Expr, time_unit: TimeUnit) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.combine(time, time_unit))
    }

    pub fn month_start(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.month_start())
    }

    pub fn month_end(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.month_end())
    }

    pub fn base_utc_offset(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.base_utc_offset())
    }

    pub fn dst_offset(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.dst_offset())
    }

    pub fn total_days(&self, fractional: bool) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.total_days(fractional))
    }

    pub fn total_hours(&self, fractional: bool) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.total_hours(fractional))
    }

    pub fn total_minutes(&self, fractional: bool) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.total_minutes(fractional))
    }

    pub fn total_seconds(&self, fractional: bool) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.total_seconds(fractional))
    }

    pub fn total_milliseconds(&self, fractional: bool) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.total_milliseconds(fractional))
    }

    pub fn total_microseconds(&self, fractional: bool) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.total_microseconds(fractional))
    }

    pub fn total_nanoseconds(&self, fractional: bool) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.total_nanoseconds(fractional))
    }

    pub fn replace(
        &self,
        year: Expr,
        month: Expr,
        day: Expr,
        hour: Expr,
        minute: Expr,
        second: Expr,
        microsecond: Expr,
        ambiguous: Expr,
    ) -> PolarsResult<Series> {
        self.apply_expr(|expr| {
            expr.replace(
                year,
                month,
                day,
                hour,
                minute,
                second,
                microsecond,
                ambiguous,
            )
        })
    }

    pub fn replace_opt(
        &self,
        year: Option<Expr>,
        month: Option<Expr>,
        day: Option<Expr>,
        hour: Option<Expr>,
        minute: Option<Expr>,
        second: Option<Expr>,
        microsecond: Option<Expr>,
        ambiguous: Expr,
    ) -> PolarsResult<Series> {
        self.apply_expr(|expr| {
            expr.replace_opt(
                year,
                month,
                day,
                hour,
                minute,
                second,
                microsecond,
                ambiguous,
            )
        })
    }

    pub fn replace_opt_lit(
        &self,
        year: Option<i32>,
        month: Option<i32>,
        day: Option<i32>,
        hour: Option<i32>,
        minute: Option<i32>,
        second: Option<i32>,
        microsecond: Option<i32>,
        ambiguous: &str,
    ) -> PolarsResult<Series> {
        self.apply_expr(|expr| {
            expr.replace_opt_lit(
                year,
                month,
                day,
                hour,
                minute,
                second,
                microsecond,
                ambiguous,
            )
        })
    }
}
