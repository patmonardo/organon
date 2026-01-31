//! Expression helpers and selectors for Polars-backed DataFrames.

use polars::lazy::dsl::{
    arange as pl_arange, arg_where as pl_arg_where, as_struct as pl_as_struct,
    coalesce as pl_coalesce, concat_arr as pl_concat_arr, concat_list as pl_concat_list,
    concat_str as pl_concat_str, cov as pl_cov, cum_fold_exprs as pl_cum_fold_exprs,
    cum_reduce_exprs as pl_cum_reduce_exprs, date_range as pl_date_range,
    date_ranges as pl_date_ranges, datetime as pl_datetime, datetime_range as pl_datetime_range,
    datetime_ranges as pl_datetime_ranges, duration as pl_duration, fold_exprs as pl_fold_exprs,
    format_str as pl_format_str, int_range as pl_int_range, int_ranges as pl_int_ranges,
    linear_space as pl_linear_space, linear_spaces as pl_linear_spaces,
    max_horizontal as pl_max_horizontal, mean_horizontal as pl_mean_horizontal,
    min_horizontal as pl_min_horizontal, pearson_corr as pl_pearson_corr,
    reduce_exprs as pl_reduce_exprs, sum_horizontal as pl_sum_horizontal,
    time_range as pl_time_range, time_ranges as pl_time_ranges, DataTypeExpr, DatetimeArgs,
    DurationArgs,
};
use polars::polars_utils::pl_str::PlSmallStr;
use polars::prelude::{
    all_horizontal as pl_all_horizontal, any_horizontal as pl_any_horizontal, col as pl_col,
    cols as pl_cols, len as pl_len, lit as pl_lit, repeat as pl_repeat, when as pl_when,
    ClosedInterval, DataType, Expr, Literal, PlanCallback, PolarsResult, Series, TimeUnit,
    TimeZone, When,
};
use polars::time::{ClosedWindow, Duration};

pub type PolarsExpr = Expr;

/// Select a column by name (Polars-style helper).
pub fn col(name: &str) -> Expr {
    pl_col(name)
}

/// Select multiple columns by name (Polars-style helper).
pub fn cols(names: &[&str]) -> Vec<Expr> {
    names.iter().map(|name| pl_col(*name)).collect()
}

/// Create a literal expression (Polars-style helper).
pub fn lit<T: Literal>(value: T) -> Expr {
    pl_lit(value)
}

/// Return the number of rows in the context (Polars-style helper).
pub fn len() -> Expr {
    pl_len()
}

/// Conditional expression builder (Polars-style helper).
pub fn when(predicate: Expr) -> When {
    pl_when(predicate)
}

/// Repeat a value to create a column of length `n` (Polars-style helper).
pub fn repeat<E: Into<Expr>>(value: E, n: Expr) -> Expr {
    pl_repeat(value, n)
}

fn select_expr_from_names(names: &[&str]) -> Expr {
    if names.is_empty() {
        pl_col("*")
    } else {
        pl_cols(names.iter().copied()).as_expr()
    }
}

/// Evaluate a bitwise AND over columns (Polars-style helper).
pub fn all(names: &[&str], ignore_nulls: bool) -> Expr {
    select_expr_from_names(names).all(ignore_nulls)
}

/// Evaluate a bitwise OR over columns (Polars-style helper).
pub fn any(names: &[&str], ignore_nulls: bool) -> Expr {
    select_expr_from_names(names).any(ignore_nulls)
}

/// Get the maximum value of one or more columns (Polars-style helper).
pub fn max(names: &[&str]) -> Expr {
    select_expr_from_names(names).max()
}

/// Get the minimum value of one or more columns (Polars-style helper).
pub fn min(names: &[&str]) -> Expr {
    select_expr_from_names(names).min()
}

/// Sum one or more columns (Polars-style helper).
pub fn sum(names: &[&str]) -> Expr {
    select_expr_from_names(names).sum()
}

/// Mean of one or more columns (Polars-style helper).
pub fn mean(names: &[&str]) -> Expr {
    select_expr_from_names(names).mean()
}

/// Compute logical AND horizontally across expressions (Polars-style helper).
pub fn all_horizontal(exprs: &[Expr]) -> PolarsResult<Expr> {
    pl_all_horizontal(exprs)
}

/// Compute logical OR horizontally across expressions (Polars-style helper).
pub fn any_horizontal(exprs: &[Expr]) -> PolarsResult<Expr> {
    pl_any_horizontal(exprs)
}

/// Compute row-wise maximum across expressions (Polars-style helper).
pub fn max_horizontal(exprs: &[Expr]) -> PolarsResult<Expr> {
    pl_max_horizontal(exprs)
}

/// Compute row-wise minimum across expressions (Polars-style helper).
pub fn min_horizontal(exprs: &[Expr]) -> PolarsResult<Expr> {
    pl_min_horizontal(exprs)
}

/// Compute row-wise sum across expressions (Polars-style helper).
pub fn sum_horizontal(exprs: &[Expr], ignore_nulls: bool) -> PolarsResult<Expr> {
    pl_sum_horizontal(exprs, ignore_nulls)
}

/// Compute row-wise mean across expressions (Polars-style helper).
pub fn mean_horizontal(exprs: &[Expr], ignore_nulls: bool) -> PolarsResult<Expr> {
    pl_mean_horizontal(exprs, ignore_nulls)
}

/// Get indices where condition is true (Polars-style helper).
pub fn arg_where(condition: Expr) -> Expr {
    pl_arg_where(condition)
}

/// Coalesce values horizontally across expressions (Polars-style helper).
pub fn coalesce(exprs: &[Expr]) -> Expr {
    pl_coalesce(exprs)
}

/// Pearson correlation between two expressions (Polars-style helper).
pub fn corr(a: Expr, b: Expr) -> Expr {
    pl_pearson_corr(a, b)
}

/// Covariance between two expressions (Polars-style helper).
pub fn cov(a: Expr, b: Expr, ddof: u8) -> Expr {
    pl_cov(a, b, ddof)
}

/// Fold expressions horizontally (Polars-style helper).
pub fn fold(
    acc: Expr,
    function: PlanCallback<(Series, Series), Series>,
    exprs: &[Expr],
    returns_scalar: bool,
    return_dtype: Option<DataTypeExpr>,
) -> Expr {
    pl_fold_exprs(acc, function, exprs, returns_scalar, return_dtype)
}

/// Reduce expressions horizontally (Polars-style helper).
pub fn reduce(
    function: PlanCallback<(Series, Series), Series>,
    exprs: &[Expr],
    returns_scalar: bool,
    return_dtype: Option<DataTypeExpr>,
) -> Expr {
    pl_reduce_exprs(function, exprs, returns_scalar, return_dtype)
}

/// Cumulative fold across expressions (Polars-style helper).
pub fn cum_fold(
    acc: Expr,
    function: PlanCallback<(Series, Series), Series>,
    exprs: &[Expr],
    returns_scalar: bool,
    return_dtype: Option<DataTypeExpr>,
    include_init: bool,
) -> Expr {
    pl_cum_fold_exprs(
        acc,
        function,
        exprs,
        returns_scalar,
        return_dtype,
        include_init,
    )
}

/// Cumulative reduce across expressions (Polars-style helper).
pub fn cum_reduce(
    function: PlanCallback<(Series, Series), Series>,
    exprs: &[Expr],
    returns_scalar: bool,
    return_dtype: Option<DataTypeExpr>,
) -> Expr {
    pl_cum_reduce_exprs(function, exprs, returns_scalar, return_dtype)
}

/// Generate a range of integers (Polars-style helper).
pub fn arange(start: Expr, end: Expr, step: i64, dtype: DataType) -> Expr {
    pl_arange(start, end, step, dtype)
}

/// Generate a range of integers (Polars-style helper).
pub fn int_range<D: Into<DataTypeExpr>>(start: Expr, end: Expr, step: i64, dtype: D) -> Expr {
    pl_int_range(start, end, step, dtype)
}

/// Generate a range of integers per row (Polars-style helper).
pub fn int_ranges<D: Into<DataTypeExpr>>(start: Expr, end: Expr, step: Expr, dtype: D) -> Expr {
    pl_int_ranges(start, end, step, dtype)
}

/// Create a date range from `start` and `end` expressions (Polars-style helper).
pub fn date_range(start: Expr, end: Expr, interval: Duration, closed: ClosedWindow) -> Expr {
    pl_date_range(start, end, interval, closed)
}

/// Create a column of date ranges (Polars-style helper).
pub fn date_ranges(start: Expr, end: Expr, interval: Duration, closed: ClosedWindow) -> Expr {
    pl_date_ranges(start, end, interval, closed)
}

/// Create a datetime range from `start` and `end` expressions (Polars-style helper).
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

/// Create a column of datetime ranges (Polars-style helper).
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

/// Generate a time range (Polars-style helper).
pub fn time_range(start: Expr, end: Expr, interval: Duration, closed: ClosedWindow) -> Expr {
    pl_time_range(start, end, interval, closed)
}

/// Create a column of time ranges (Polars-style helper).
pub fn time_ranges(start: Expr, end: Expr, interval: Duration, closed: ClosedWindow) -> Expr {
    pl_time_ranges(start, end, interval, closed)
}

/// Generate a series of equally-spaced points (Polars-style helper).
pub fn linear_space(start: Expr, end: Expr, num_samples: Expr, closed: ClosedInterval) -> Expr {
    pl_linear_space(start, end, num_samples, closed)
}

/// Generate per-row linearly-spaced sequences (Polars-style helper).
pub fn linear_spaces(
    start: Expr,
    end: Expr,
    num_samples: Expr,
    closed: ClosedInterval,
    as_array: bool,
) -> PolarsResult<Expr> {
    pl_linear_spaces(start, end, num_samples, closed, as_array)
}

/// Construct a datetime expression (Polars-style helper).
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

/// Construct a date expression (Polars-style helper).
pub fn date(year: Expr, month: Expr, day: Expr) -> Expr {
    pl_datetime(DatetimeArgs::new(year, month, day)).cast(DataType::Date)
}

/// Construct a time expression (Polars-style helper).
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

/// Construct a duration expression (Polars-style helper).
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

/// Concat list expressions (Polars-style helper).
pub fn concat_list(exprs: &[Expr]) -> PolarsResult<Expr> {
    pl_concat_list(exprs)
}

/// Concat array expressions (Polars-style helper).
pub fn concat_arr(exprs: &[Expr]) -> PolarsResult<Expr> {
    pl_concat_arr(exprs.to_vec())
}

/// Concat string expressions (Polars-style helper).
pub fn concat_str(exprs: &[Expr], separator: &str, ignore_nulls: bool) -> Expr {
    pl_concat_str(exprs, separator, ignore_nulls)
}

/// Format string with expressions (Polars-style helper).
pub fn format(format: &str, args: &[Expr]) -> PolarsResult<Expr> {
    pl_format_str(format, args)
}

/// Create a struct expression (Polars-style helper).
pub fn struct_(exprs: Vec<Expr>) -> Expr {
    pl_as_struct(exprs)
}

/// Create a datatype expression from an expression (Polars-style helper).
pub fn dtype_of(expr: Expr) -> DataTypeExpr {
    DataTypeExpr::OfExpr(Box::new(expr))
}

/// Create a datatype expression representing `self` (Polars-style helper).
pub fn self_dtype() -> DataTypeExpr {
    DataTypeExpr::SelfDtype
}

/// Create a datatype expression representing a struct with fields.
pub fn struct_with_fields(fields: &[(impl AsRef<str>, DataTypeExpr)]) -> DataTypeExpr {
    DataTypeExpr::StructWithFields(
        fields
            .iter()
            .map(|(name, dtype)| (PlSmallStr::from(name.as_ref()), dtype.clone()))
            .collect(),
    )
}

/// Select a column by name.
pub fn expr_col(name: &str) -> Expr {
    pl_col(name)
}

/// Select multiple columns by name.
pub fn expr_cols(names: &[&str]) -> Vec<Expr> {
    names.iter().map(|name| pl_col(*name)).collect()
}

/// Literal expressions.
pub fn expr_lit_i64(value: i64) -> Expr {
    pl_lit(value)
}

pub fn expr_lit_i32(value: i32) -> Expr {
    pl_lit(value)
}

pub fn expr_lit_i16(value: i16) -> Expr {
    pl_lit(value)
}

pub fn expr_lit_i8(value: i8) -> Expr {
    pl_lit(value)
}

pub fn expr_lit_u64(value: u64) -> Expr {
    pl_lit(value)
}

pub fn expr_lit_u32(value: u32) -> Expr {
    pl_lit(value)
}

pub fn expr_lit_u16(value: u16) -> Expr {
    pl_lit(value)
}

pub fn expr_lit_u8(value: u8) -> Expr {
    pl_lit(value)
}

pub fn expr_lit_f64(value: f64) -> Expr {
    pl_lit(value)
}

pub fn expr_lit_f32(value: f32) -> Expr {
    pl_lit(value)
}

pub fn expr_lit_bool(value: bool) -> Expr {
    pl_lit(value)
}

pub fn expr_lit_str(value: &str) -> Expr {
    pl_lit(value)
}

/// Comparison and logical helpers.
pub fn expr_eq(lhs: Expr, rhs: Expr) -> Expr {
    lhs.eq(rhs)
}

pub fn expr_neq(lhs: Expr, rhs: Expr) -> Expr {
    lhs.neq(rhs)
}

pub fn expr_gt(lhs: Expr, rhs: Expr) -> Expr {
    lhs.gt(rhs)
}

pub fn expr_gte(lhs: Expr, rhs: Expr) -> Expr {
    lhs.gt_eq(rhs)
}

pub fn expr_lt(lhs: Expr, rhs: Expr) -> Expr {
    lhs.lt(rhs)
}

pub fn expr_lte(lhs: Expr, rhs: Expr) -> Expr {
    lhs.lt_eq(rhs)
}

pub fn expr_and(lhs: Expr, rhs: Expr) -> Expr {
    lhs.and(rhs)
}

pub fn expr_or(lhs: Expr, rhs: Expr) -> Expr {
    lhs.or(rhs)
}

pub fn expr_not(expr: Expr) -> Expr {
    expr.not()
}

pub fn expr_alias(expr: Expr, name: &str) -> Expr {
    expr.alias(name)
}

/// Null-handling helpers.
pub fn expr_is_null(expr: Expr) -> Expr {
    expr.is_null()
}

pub fn expr_is_not_null(expr: Expr) -> Expr {
    expr.is_not_null()
}

pub fn expr_fill_null(expr: Expr, value: Expr) -> Expr {
    expr.fill_null(value)
}

/// Type coercion.
pub fn expr_cast(expr: Expr, dtype: DataType) -> Expr {
    expr.cast(dtype)
}

/// Aggregations.
pub fn expr_sum(expr: Expr) -> Expr {
    expr.sum()
}

pub fn expr_mean(expr: Expr) -> Expr {
    expr.mean()
}

pub fn expr_min(expr: Expr) -> Expr {
    expr.min()
}

pub fn expr_max(expr: Expr) -> Expr {
    expr.max()
}

pub fn expr_count(expr: Expr) -> Expr {
    expr.count()
}

/// Conditional expression builder.
pub fn expr_when(predicate: Expr, then_expr: Expr, otherwise_expr: Expr) -> Expr {
    pl_when(predicate).then(then_expr).otherwise(otherwise_expr)
}
