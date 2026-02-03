//! Datetime namespace for expressions (py-polars inspired).

use polars::prelude::{lit, DataType, Expr, NonExistent, Scalar, TimeUnit, TimeZone};

#[derive(Debug, Clone)]
pub struct ExprDateTime {
    expr: Expr,
}

fn null_expr() -> Expr {
    Expr::Literal(Scalar::null(DataType::Null).into())
}

fn opt_expr(value: Option<Expr>) -> Expr {
    value.unwrap_or_else(null_expr)
}

fn opt_lit_i32(value: Option<i32>) -> Expr {
    value.map(lit).unwrap_or_else(null_expr)
}

impl ExprDateTime {
    pub fn new(expr: Expr) -> Self {
        Self { expr }
    }

    pub fn expr(&self) -> &Expr {
        &self.expr
    }

    pub fn into_expr(self) -> Expr {
        self.expr
    }

    pub fn add_business_days(
        self,
        n: i64,
        week_mask: [bool; 7],
        holidays: Vec<i32>,
        roll: polars::prelude::Roll,
    ) -> Expr {
        self.expr
            .dt()
            .add_business_days(lit(n), week_mask, holidays, roll)
    }

    pub fn add_business_days_expr(
        self,
        n: Expr,
        week_mask: [bool; 7],
        holidays: Vec<i32>,
        roll: polars::prelude::Roll,
    ) -> Expr {
        self.expr
            .dt()
            .add_business_days(n, week_mask, holidays, roll)
    }

    pub fn to_string(self, format: &str) -> Expr {
        self.expr.dt().to_string(format)
    }

    pub fn strftime(self, format: &str) -> Expr {
        self.to_string(format)
    }

    pub fn cast_time_unit(self, time_unit: TimeUnit) -> Expr {
        self.expr.dt().cast_time_unit(time_unit)
    }

    pub fn with_time_unit(self, time_unit: TimeUnit) -> Expr {
        self.expr.dt().with_time_unit(time_unit)
    }

    pub fn convert_time_zone(self, time_zone: TimeZone) -> Expr {
        self.expr.dt().convert_time_zone(time_zone)
    }

    pub fn replace_time_zone(
        self,
        time_zone: Option<TimeZone>,
        ambiguous: Expr,
        non_existent: NonExistent,
    ) -> Expr {
        self.expr
            .dt()
            .replace_time_zone(time_zone, ambiguous, non_existent)
    }

    pub fn millennium(self) -> Expr {
        self.expr.dt().millennium()
    }

    pub fn century(self) -> Expr {
        self.expr.dt().century()
    }

    pub fn year(self) -> Expr {
        self.expr.dt().year()
    }

    pub fn is_business_day(self, week_mask: [bool; 7], holidays: Vec<i32>) -> Expr {
        self.expr.dt().is_business_day(week_mask, holidays)
    }

    pub fn is_leap_year(self) -> Expr {
        self.expr.dt().is_leap_year()
    }

    pub fn iso_year(self) -> Expr {
        self.expr.dt().iso_year()
    }

    pub fn month(self) -> Expr {
        self.expr.dt().month()
    }

    pub fn days_in_month(self) -> Expr {
        self.expr.dt().days_in_month()
    }

    pub fn quarter(self) -> Expr {
        self.expr.dt().quarter()
    }

    pub fn week(self) -> Expr {
        self.expr.dt().week()
    }

    pub fn weekday(self) -> Expr {
        self.expr.dt().weekday()
    }

    pub fn day(self) -> Expr {
        self.expr.dt().day()
    }

    pub fn ordinal_day(self) -> Expr {
        self.expr.dt().ordinal_day()
    }

    pub fn time(self) -> Expr {
        self.expr.dt().time()
    }

    pub fn date(self) -> Expr {
        self.expr.dt().date()
    }

    pub fn datetime(self) -> Expr {
        self.expr.dt().datetime()
    }

    pub fn hour(self) -> Expr {
        self.expr.dt().hour()
    }

    pub fn minute(self) -> Expr {
        self.expr.dt().minute()
    }

    pub fn second(self) -> Expr {
        self.expr.dt().second()
    }

    pub fn second_fractional(self) -> Expr {
        let expr = self.expr;
        let seconds = expr
            .clone()
            .dt()
            .second()
            .cast(polars::prelude::DataType::Float64);
        let nanos = expr
            .dt()
            .nanosecond()
            .cast(polars::prelude::DataType::Float64);
        seconds + (nanos / lit(1_000_000_000.0))
    }

    pub fn millisecond(self) -> Expr {
        self.expr.dt().millisecond()
    }

    pub fn microsecond(self) -> Expr {
        self.expr.dt().microsecond()
    }

    pub fn nanosecond(self) -> Expr {
        self.expr.dt().nanosecond()
    }

    pub fn timestamp(self, time_unit: TimeUnit) -> Expr {
        self.expr.dt().timestamp(time_unit)
    }

    pub fn timestamp_ms(self) -> Expr {
        self.expr.dt().timestamp(TimeUnit::Milliseconds)
    }

    pub fn timestamp_us(self) -> Expr {
        self.expr.dt().timestamp(TimeUnit::Microseconds)
    }

    pub fn timestamp_ns(self) -> Expr {
        self.expr.dt().timestamp(TimeUnit::Nanoseconds)
    }

    pub fn truncate(self, every: &str) -> Expr {
        self.expr.dt().truncate(lit(every))
    }

    pub fn truncate_expr(self, every: Expr) -> Expr {
        self.expr.dt().truncate(every)
    }

    pub fn round(self, every: &str) -> Expr {
        self.expr.dt().round(lit(every))
    }

    pub fn round_expr(self, every: Expr) -> Expr {
        self.expr.dt().round(every)
    }

    pub fn combine(self, time: Expr, time_unit: TimeUnit) -> Expr {
        self.expr.dt().combine(time, time_unit)
    }

    pub fn month_start(self) -> Expr {
        self.expr.dt().month_start()
    }

    pub fn month_end(self) -> Expr {
        self.expr.dt().month_end()
    }

    pub fn base_utc_offset(self) -> Expr {
        self.expr.dt().base_utc_offset()
    }

    pub fn dst_offset(self) -> Expr {
        self.expr.dt().dst_offset()
    }

    pub fn total_days(self, fractional: bool) -> Expr {
        self.expr.dt().total_days(fractional)
    }

    pub fn total_hours(self, fractional: bool) -> Expr {
        self.expr.dt().total_hours(fractional)
    }

    pub fn total_minutes(self, fractional: bool) -> Expr {
        self.expr.dt().total_minutes(fractional)
    }

    pub fn total_seconds(self, fractional: bool) -> Expr {
        self.expr.dt().total_seconds(fractional)
    }

    pub fn total_milliseconds(self, fractional: bool) -> Expr {
        self.expr.dt().total_milliseconds(fractional)
    }

    pub fn total_microseconds(self, fractional: bool) -> Expr {
        self.expr.dt().total_microseconds(fractional)
    }

    pub fn total_nanoseconds(self, fractional: bool) -> Expr {
        self.expr.dt().total_nanoseconds(fractional)
    }

    pub fn replace(
        self,
        year: Expr,
        month: Expr,
        day: Expr,
        hour: Expr,
        minute: Expr,
        second: Expr,
        microsecond: Expr,
        ambiguous: Expr,
    ) -> Expr {
        self.expr.dt().replace(
            year,
            month,
            day,
            hour,
            minute,
            second,
            microsecond,
            ambiguous,
        )
    }

    pub fn replace_opt(
        self,
        year: Option<Expr>,
        month: Option<Expr>,
        day: Option<Expr>,
        hour: Option<Expr>,
        minute: Option<Expr>,
        second: Option<Expr>,
        microsecond: Option<Expr>,
        ambiguous: Expr,
    ) -> Expr {
        self.replace(
            opt_expr(year),
            opt_expr(month),
            opt_expr(day),
            opt_expr(hour),
            opt_expr(minute),
            opt_expr(second),
            opt_expr(microsecond),
            ambiguous,
        )
    }

    pub fn replace_opt_lit(
        self,
        year: Option<i32>,
        month: Option<i32>,
        day: Option<i32>,
        hour: Option<i32>,
        minute: Option<i32>,
        second: Option<i32>,
        microsecond: Option<i32>,
        ambiguous: &str,
    ) -> Expr {
        self.replace_opt(
            Some(opt_lit_i32(year)),
            Some(opt_lit_i32(month)),
            Some(opt_lit_i32(day)),
            Some(opt_lit_i32(hour)),
            Some(opt_lit_i32(minute)),
            Some(opt_lit_i32(second)),
            Some(opt_lit_i32(microsecond)),
            lit(ambiguous),
        )
    }
}
