use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};

// Integer ranges
pub const I8_MIN: i8 = i8::MIN;
pub const I16_MIN: i16 = i16::MIN;
pub const I32_MIN: i32 = i32::MIN;
pub const I64_MIN: i64 = i64::MIN;
pub const I128_MIN: i128 = i128::MIN;
pub const I8_MAX: i8 = i8::MAX;
pub const I16_MAX: i16 = i16::MAX;
pub const I32_MAX: i32 = i32::MAX;
pub const I64_MAX: i64 = i64::MAX;
pub const I128_MAX: i128 = i128::MAX;
pub const U8_MAX: u8 = u8::MAX;
pub const U16_MAX: u16 = u16::MAX;
pub const U32_MAX: u32 = u32::MAX;
pub const U64_MAX: u64 = u64::MAX;
pub const U128_MAX: u128 = u128::MAX;

// Temporal
pub const SECONDS_PER_DAY: u32 = 86_400;
pub const SECONDS_PER_HOUR: u32 = 3_600;
pub const NS_PER_SECOND: u32 = 1_000_000_000;
pub const US_PER_SECOND: u32 = 1_000_000;
pub const MS_PER_SECOND: u32 = 1_000;

pub fn epoch_date() -> NaiveDate {
    NaiveDate::from_ymd_opt(1970, 1, 1).expect("hardcoded epoch date should always be valid")
}

pub fn epoch() -> NaiveDateTime {
    NaiveDateTime::new(
        epoch_date(),
        NaiveTime::from_hms_opt(0, 0, 0).expect("00:00:00 midnight should always exist"),
    )
}

pub fn epoch_utc() -> DateTime<Utc> {
    Utc.from_utc_datetime(&epoch())
}
