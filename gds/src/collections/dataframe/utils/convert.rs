//! Temporal conversion helpers inspired by py-polars _utils/convert.

use std::fmt;
use std::str::FromStr;

use chrono::offset::Offset;
use chrono::prelude::{TimeZone, Timelike};
use chrono::{DateTime, Duration, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use chrono_tz::Tz;
use polars::prelude::TimeUnit;

use crate::collections::dataframe::utils::constants::{
    epoch, epoch_date, epoch_utc, NS_PER_SECOND, SECONDS_PER_DAY,
};

const MICROS_PER_DAY: i64 = SECONDS_PER_DAY as i64 * 1_000_000;

/// Input that can be parsed as a duration string.
pub enum DurationInput<'a> {
    Duration(Duration),
    Str(&'a str),
}

/// Time representations returned by `to_py_datetime`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemporalDateTime {
    Naive(NaiveDateTime),
    Zoned(DateTime<FixedOffset>),
}

/// Errors emitted while converting between Pythonic temporal types and Rust.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConvertError {
    DurationOverflow,
    DateOutOfRange(i64),
    TimeOutOfRange(i64),
    UnknownTimeZone(String),
    InvalidFixedOffset(String),
}

impl fmt::Display for ConvertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ConvertError::*;
        match self {
            DurationOverflow => write!(f, "duration overflowed while converting"),
            DateOutOfRange(value) => write!(f, "date {value} is outside the supported range"),
            TimeOutOfRange(value) => write!(f, "time {value} is outside the supported range"),
            UnknownTimeZone(tz) => write!(f, "unknown time zone: {tz}"),
            InvalidFixedOffset(offset) => write!(f, "invalid fixed offset: {offset}"),
        }
    }
}

impl std::error::Error for ConvertError {}

/// Normalize the input so that either a duration string is returned (and
/// canonicalized when a `Duration` was supplied) or the original string is
/// returned untouched.
pub fn parse_as_duration_string(
    input: Option<DurationInput<'_>>,
) -> Result<Option<String>, ConvertError> {
    match input {
        None => Ok(None),
        Some(DurationInput::Str(text)) => Ok(Some(text.to_string())),
        Some(DurationInput::Duration(duration)) => {
            Ok(Some(timedelta_to_duration_string(duration)?))
        }
    }
}

/// Convert a chrono `Duration` into the human-readable style used by Polars.
pub fn timedelta_to_duration_string(duration: Duration) -> Result<String, ConvertError> {
    let (days, seconds, micros) = duration_components(duration)?;

    if duration >= Duration::zero() {
        let mut parts = String::new();
        if days != 0 {
            parts.push_str(&format!("{days}d"));
        }
        if seconds != 0 {
            parts.push_str(&format!("{seconds}s"));
        }
        if micros != 0 {
            parts.push_str(&format!("{micros}us"));
        }
        if parts.is_empty() {
            parts.push_str("0us");
        }
        Ok(parts)
    } else if seconds == 0 && micros == 0 {
        Ok(format!("{days}d"))
    } else {
        let corrected_days = days + 1;
        let corrected_seconds = SECONDS_PER_DAY as i64 - (seconds + (micros > 0) as i64);
        let corrected_micros = if micros != 0 { 1_000_000 - micros } else { 0 };

        let mut result = if corrected_days != 0 {
            format!("{corrected_days}d")
        } else {
            "-".to_string()
        };
        if corrected_seconds != 0 {
            result.push_str(&format!("{corrected_seconds}s"));
        }
        if corrected_micros != 0 {
            result.push_str(&format!("{corrected_micros}us"));
        }
        Ok(result)
    }
}

/// Negate a duration string (adds/removes the leading `-`).
pub fn negate_duration_string(duration: &str) -> String {
    if duration.starts_with('-') {
        duration[1..].to_string()
    } else {
        format!("-{duration}")
    }
}

pub fn date_to_int(date: NaiveDate) -> i64 {
    (date.signed_duration_since(epoch_date())).num_days()
}

pub fn time_to_int(time: NaiveTime) -> i64 {
    let seconds = time.num_seconds_from_midnight() as i64;
    let nanos = time.nanosecond() as i64;
    seconds * (NS_PER_SECOND as i64) + nanos
}

pub fn datetime_to_int(datetime: DateTime<Utc>, unit: TimeUnit) -> Result<i64, ConvertError> {
    let duration = datetime.signed_duration_since(epoch_utc());
    duration_value(duration, unit)
}

pub fn timedelta_to_int(duration: Duration, unit: TimeUnit) -> Result<i64, ConvertError> {
    duration_value(duration, unit)
}

pub fn to_py_date(value: i64) -> Result<NaiveDate, ConvertError> {
    epoch_date()
        .checked_add_signed(Duration::days(value))
        .ok_or(ConvertError::DateOutOfRange(value))
}

pub fn to_py_time(value: i64) -> Result<NaiveTime, ConvertError> {
    let seconds = value.div_euclid(NS_PER_SECOND as i64);
    let remainder = value.rem_euclid(NS_PER_SECOND as i64) as u32;
    let seconds_since_midnight = seconds.rem_euclid(SECONDS_PER_DAY as i64) as u32;
    NaiveTime::from_num_seconds_from_midnight_opt(seconds_since_midnight, remainder)
        .ok_or(ConvertError::TimeOutOfRange(value))
}

pub fn to_py_datetime(
    value: i64,
    unit: TimeUnit,
    time_zone: Option<&str>,
) -> Result<TemporalDateTime, ConvertError> {
    let duration = duration_for_unit(value, unit);
    let naive = epoch() + duration;
    match time_zone {
        None => Ok(TemporalDateTime::Naive(naive)),
        Some(tz) => Ok(TemporalDateTime::Zoned(localize_datetime(naive, tz)?)),
    }
}

pub fn to_py_timedelta(value: i64, unit: TimeUnit) -> Duration {
    duration_for_unit(value, unit)
}

fn duration_components(duration: Duration) -> Result<(i64, i64, i64), ConvertError> {
    let total_micros = duration
        .num_microseconds()
        .ok_or(ConvertError::DurationOverflow)?;
    let days = total_micros.div_euclid(MICROS_PER_DAY);
    let remainder = total_micros.rem_euclid(MICROS_PER_DAY);
    let seconds = remainder / 1_000_000;
    let micros = remainder % 1_000_000;
    Ok((days, seconds, micros))
}

fn duration_value(duration: Duration, unit: TimeUnit) -> Result<i64, ConvertError> {
    match unit {
        TimeUnit::Nanoseconds => duration
            .num_nanoseconds()
            .ok_or(ConvertError::DurationOverflow),
        TimeUnit::Microseconds => duration
            .num_microseconds()
            .ok_or(ConvertError::DurationOverflow),
        TimeUnit::Milliseconds => Ok(duration.num_milliseconds()),
    }
}

fn duration_for_unit(value: i64, unit: TimeUnit) -> Duration {
    match unit {
        TimeUnit::Nanoseconds => Duration::nanoseconds(value),
        TimeUnit::Microseconds => Duration::microseconds(value),
        TimeUnit::Milliseconds => Duration::milliseconds(value),
    }
}

fn localize_datetime(
    dt: NaiveDateTime,
    time_zone: &str,
) -> Result<DateTime<FixedOffset>, ConvertError> {
    if let Ok(tz) = time_zone.parse::<Tz>() {
        let zoned = tz.from_utc_datetime(&dt);
        Ok(zoned.with_timezone(&zoned.offset().fix()))
    } else {
        let fixed = parse_fixed_tz_offset(time_zone)?;
        Ok(fixed.from_utc_datetime(&dt))
    }
}

fn parse_fixed_tz_offset(offset: &str) -> Result<FixedOffset, ConvertError> {
    FixedOffset::from_str(offset).map_err(|_| ConvertError::UnknownTimeZone(offset.to_string()))
}
