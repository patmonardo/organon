//! Selector helpers for DataFrame-backed column selection (py-polars inspired).

use polars::prelude::{col, DataFrame, DataType, Expr};
use regex::Regex;

/// Column selector for DataFrame schemas.
#[derive(Debug, Clone)]
pub enum Selector {
    All,
    ByName(Vec<String>),
    ByIndex(Vec<usize>),
    StartsWith(String),
    EndsWith(String),
    Contains(String),
    Matches(Regex),
    ByDtype(Vec<DataType>),
    Numeric,
    Integer,
    SignedInteger,
    UnsignedInteger,
    Float,
    Boolean,
    String,
    Binary,
    Temporal,
    Date,
    Datetime,
    Time,
    Duration,
    Categorical,
    List,
    Struct,
    First,
    Last,
    Or(Vec<Selector>),
    And(Vec<Selector>),
    Not(Box<Selector>),
    Exclude {
        base: Box<Selector>,
        remove: Box<Selector>,
    },
}

impl Selector {
    /// Combine selectors using logical OR.
    pub fn or(self, other: Selector) -> Selector {
        Selector::Or(vec![self, other])
    }

    /// Combine selectors using logical AND.
    pub fn and(self, other: Selector) -> Selector {
        Selector::And(vec![self, other])
    }

    /// Negate a selector.
    pub fn not(self) -> Selector {
        Selector::Not(Box::new(self))
    }

    /// Exclude a selector from another.
    pub fn exclude(self, other: Selector) -> Selector {
        Selector::Exclude {
            base: Box::new(self),
            remove: Box::new(other),
        }
    }
}

/// Select all columns.
pub fn all() -> Selector {
    Selector::All
}

/// Select columns by name.
pub fn by_name(names: &[&str]) -> Selector {
    Selector::ByName(names.iter().map(|name| (*name).to_string()).collect())
}

/// Select columns by index.
pub fn by_index(indices: &[usize]) -> Selector {
    Selector::ByIndex(indices.to_vec())
}

/// Select columns whose name starts with the prefix.
pub fn starts_with(prefix: &str) -> Selector {
    Selector::StartsWith(prefix.to_string())
}

/// Select columns whose name ends with the suffix.
pub fn ends_with(suffix: &str) -> Selector {
    Selector::EndsWith(suffix.to_string())
}

/// Select columns whose name contains the substring.
pub fn contains(needle: &str) -> Selector {
    Selector::Contains(needle.to_string())
}

/// Select columns whose name matches the regex.
pub fn matches(pattern: &str) -> Result<Selector, regex::Error> {
    Ok(Selector::Matches(Regex::new(pattern)?))
}

/// Select columns by dtype.
pub fn by_dtype(dtypes: &[DataType]) -> Selector {
    Selector::ByDtype(dtypes.to_vec())
}

/// Select numeric columns (integers and floats).
pub fn numeric() -> Selector {
    Selector::Numeric
}

/// Select integer columns (signed + unsigned).
pub fn integer() -> Selector {
    Selector::Integer
}

/// Select signed integer columns.
pub fn signed_integer() -> Selector {
    Selector::SignedInteger
}

/// Select unsigned integer columns.
pub fn unsigned_integer() -> Selector {
    Selector::UnsignedInteger
}

/// Select float columns.
pub fn float() -> Selector {
    Selector::Float
}

/// Select boolean columns.
pub fn boolean() -> Selector {
    Selector::Boolean
}

/// Select string columns.
pub fn string() -> Selector {
    Selector::String
}

/// Select binary columns.
pub fn binary() -> Selector {
    Selector::Binary
}

/// Select temporal columns (date, datetime, time, duration).
pub fn temporal() -> Selector {
    Selector::Temporal
}

/// Select date columns.
pub fn date() -> Selector {
    Selector::Date
}

/// Select datetime columns.
pub fn datetime() -> Selector {
    Selector::Datetime
}

/// Select time columns.
pub fn time() -> Selector {
    Selector::Time
}

/// Select duration columns.
pub fn duration() -> Selector {
    Selector::Duration
}

/// Select categorical columns.
pub fn categorical() -> Selector {
    Selector::Categorical
}

/// Select list columns.
pub fn list() -> Selector {
    Selector::List
}

/// Select struct columns.
pub fn struct_() -> Selector {
    Selector::Struct
}

/// Select first column.
pub fn first() -> Selector {
    Selector::First
}

/// Select last column.
pub fn last() -> Selector {
    Selector::Last
}

/// Expand a selector to column names for a specific DataFrame.
pub fn expand_selector(df: &DataFrame, selector: &Selector) -> Vec<String> {
    let total = df.width();
    df.get_columns()
        .iter()
        .enumerate()
        .filter_map(|(index, series)| {
            let info = ColumnInfo {
                name: series.name(),
                dtype: series.dtype(),
                index,
                total,
            };
            if selector_matches(selector, &info) {
                Some(info.name.to_string())
            } else {
                None
            }
        })
        .collect()
}

/// Expand a selector to Polars expressions for a specific DataFrame.
pub fn expand_exprs(df: &DataFrame, selector: &Selector) -> Vec<Expr> {
    expand_selector(df, selector)
        .into_iter()
        .map(|name| col(&name))
        .collect()
}

#[derive(Debug)]
struct ColumnInfo<'a> {
    name: &'a str,
    dtype: &'a DataType,
    index: usize,
    total: usize,
}

fn selector_matches(selector: &Selector, info: &ColumnInfo<'_>) -> bool {
    match selector {
        Selector::All => true,
        Selector::ByName(names) => names.iter().any(|name| name == info.name),
        Selector::ByIndex(indices) => indices.iter().any(|index| *index == info.index),
        Selector::StartsWith(prefix) => info.name.starts_with(prefix),
        Selector::EndsWith(suffix) => info.name.ends_with(suffix),
        Selector::Contains(needle) => info.name.contains(needle),
        Selector::Matches(regex) => regex.is_match(info.name),
        Selector::ByDtype(dtypes) => dtypes.iter().any(|dtype| dtype == info.dtype),
        Selector::Numeric => is_numeric(info.dtype),
        Selector::Integer => is_integer(info.dtype),
        Selector::SignedInteger => is_signed_integer(info.dtype),
        Selector::UnsignedInteger => is_unsigned_integer(info.dtype),
        Selector::Float => is_float(info.dtype),
        Selector::Boolean => matches!(info.dtype, DataType::Boolean),
        Selector::String => matches!(info.dtype, DataType::String),
        Selector::Binary => matches!(info.dtype, DataType::Binary),
        Selector::Temporal => is_temporal(info.dtype),
        Selector::Date => matches!(info.dtype, DataType::Date),
        Selector::Datetime => matches!(info.dtype, DataType::Datetime(_, _)),
        Selector::Time => matches!(info.dtype, DataType::Time),
        Selector::Duration => matches!(info.dtype, DataType::Duration(_)),
        Selector::Categorical => matches!(info.dtype, DataType::Categorical(_, _)),
        Selector::List => matches!(info.dtype, DataType::List(_)),
        Selector::Struct => matches!(info.dtype, DataType::Struct(_)),
        Selector::First => info.index == 0,
        Selector::Last => info.index + 1 == info.total,
        Selector::Or(selectors) => selectors.iter().any(|sel| selector_matches(sel, info)),
        Selector::And(selectors) => selectors.iter().all(|sel| selector_matches(sel, info)),
        Selector::Not(selector) => !selector_matches(selector, info),
        Selector::Exclude { base, remove } => {
            selector_matches(base, info) && !selector_matches(remove, info)
        }
    }
}

fn is_numeric(dtype: &DataType) -> bool {
    is_integer(dtype) || is_float(dtype)
}

fn is_integer(dtype: &DataType) -> bool {
    is_signed_integer(dtype) || is_unsigned_integer(dtype)
}

fn is_signed_integer(dtype: &DataType) -> bool {
    matches!(
        dtype,
        DataType::Int8 | DataType::Int16 | DataType::Int32 | DataType::Int64
    )
}

fn is_unsigned_integer(dtype: &DataType) -> bool {
    matches!(
        dtype,
        DataType::UInt8 | DataType::UInt16 | DataType::UInt32 | DataType::UInt64
    )
}

fn is_float(dtype: &DataType) -> bool {
    matches!(dtype, DataType::Float32 | DataType::Float64)
}

fn is_temporal(dtype: &DataType) -> bool {
    matches!(
        dtype,
        DataType::Date | DataType::Datetime(_, _) | DataType::Time | DataType::Duration(_)
    )
}
