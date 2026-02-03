//! Selector helpers for DataFrame-backed column selection (py-polars inspired).

use polars::prelude::{col, DataFrame, DataType, Expr};
use regex::Regex;

/// Column selector for DataFrame schemas.
#[derive(Debug, Clone)]
pub enum Selector {
    All,
    ByName(Vec<String>),
    ByIndex(Vec<usize>),
    ByIndexSigned {
        indices: Vec<isize>,
        require_all: bool,
    },
    StartsWith(String),
    StartsWithAny(Vec<String>),
    EndsWith(String),
    EndsWithAny(Vec<String>),
    Contains(String),
    ContainsAny(Vec<String>),
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
    Enum,
    Decimal,
    Object,
    List,
    Array,
    Struct,
    Nested,
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

/// Select columns by signed index (negative indices are relative to the end).
pub fn by_index_signed(indices: &[isize], require_all: bool) -> Selector {
    Selector::ByIndexSigned {
        indices: indices.to_vec(),
        require_all,
    }
}

/// Select columns whose name starts with the prefix.
pub fn starts_with(prefix: &str) -> Selector {
    Selector::StartsWith(prefix.to_string())
}

/// Select columns whose name starts with any of the prefixes.
pub fn starts_with_any(prefixes: &[&str]) -> Selector {
    Selector::StartsWithAny(
        prefixes
            .iter()
            .map(|prefix| (*prefix).to_string())
            .collect(),
    )
}

/// Select columns whose name ends with the suffix.
pub fn ends_with(suffix: &str) -> Selector {
    Selector::EndsWith(suffix.to_string())
}

/// Select columns whose name ends with any of the suffixes.
pub fn ends_with_any(suffixes: &[&str]) -> Selector {
    Selector::EndsWithAny(
        suffixes
            .iter()
            .map(|suffix| (*suffix).to_string())
            .collect(),
    )
}

/// Select columns whose name contains the substring.
pub fn contains(needle: &str) -> Selector {
    Selector::Contains(needle.to_string())
}

/// Select columns whose name contains any of the substrings.
pub fn contains_any(needles: &[&str]) -> Selector {
    Selector::ContainsAny(needles.iter().map(|needle| (*needle).to_string()).collect())
}

/// Select columns whose name matches the regex.
pub fn matches(pattern: &str) -> Result<Selector, regex::Error> {
    Ok(Selector::Matches(Regex::new(pattern)?))
}

/// Select columns with alphabetic names.
pub fn alpha(ascii_only: bool, ignore_spaces: bool) -> Result<Selector, regex::Error> {
    let re_alpha = if ascii_only {
        r"a-zA-Z"
    } else {
        r"\p{Alphabetic}"
    };
    let re_space = if ignore_spaces { " " } else { "" };
    let pattern = format!("^[{re_alpha}{re_space}]+$");
    matches(&pattern)
}

/// Select columns with alphanumeric names.
pub fn alphanumeric(ascii_only: bool, ignore_spaces: bool) -> Result<Selector, regex::Error> {
    let re_alpha = if ascii_only {
        r"a-zA-Z"
    } else {
        r"\p{Alphabetic}"
    };
    let re_digit = if ascii_only { "0-9" } else { r"\d" };
    let re_space = if ignore_spaces { " " } else { "" };
    let pattern = format!("^[{re_alpha}{re_digit}{re_space}]+$");
    matches(&pattern)
}

/// Select columns with digit-only names.
pub fn digit(ascii_only: bool) -> Result<Selector, regex::Error> {
    let re_digit = if ascii_only { "[0-9]" } else { r"\d" };
    let pattern = format!("^{re_digit}+$");
    matches(&pattern)
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

/// Select string and categorical columns.
pub fn string_with_categorical() -> Selector {
    Selector::String.or(Selector::Categorical)
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

/// Select enum columns.
pub fn enum_() -> Selector {
    Selector::Enum
}

/// Select decimal columns.
pub fn decimal() -> Selector {
    Selector::Decimal
}

/// Select object columns.
pub fn object() -> Selector {
    Selector::Object
}

/// Select list columns.
pub fn list() -> Selector {
    Selector::List
}

/// Select array columns.
pub fn array() -> Selector {
    Selector::Array
}

/// Select struct columns.
pub fn struct_() -> Selector {
    Selector::Struct
}

/// Select struct columns.
pub fn record() -> Selector {
    struct_()
}

/// Select nested columns (list/array/struct).
pub fn nested() -> Selector {
    Selector::Nested
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
    let by_index_signed = resolve_signed_indices(selector, total);
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
            if selector_matches(selector, &info, by_index_signed.as_ref()) {
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

fn resolve_signed_indices(selector: &Selector, total: usize) -> Option<Vec<usize>> {
    match selector {
        Selector::ByIndexSigned {
            indices,
            require_all,
        } => {
            let mut resolved = Vec::with_capacity(indices.len());
            for index in indices {
                let idx = if *index < 0 {
                    let offset = total as isize + *index;
                    if offset < 0 {
                        if *require_all {
                            return Some(Vec::new());
                        }
                        continue;
                    }
                    offset as usize
                } else {
                    *index as usize
                };
                if idx >= total {
                    if *require_all {
                        return Some(Vec::new());
                    }
                    continue;
                }
                resolved.push(idx);
            }
            Some(resolved)
        }
        _ => None,
    }
}

#[derive(Debug)]
struct ColumnInfo<'a> {
    name: &'a str,
    dtype: &'a DataType,
    index: usize,
    total: usize,
}

fn selector_matches(
    selector: &Selector,
    info: &ColumnInfo<'_>,
    resolved_indices: Option<&Vec<usize>>,
) -> bool {
    match selector {
        Selector::All => true,
        Selector::ByName(names) => names.iter().any(|name| name == info.name),
        Selector::ByIndex(indices) => indices.iter().any(|index| *index == info.index),
        Selector::ByIndexSigned { .. } => resolved_indices
            .map(|indices| indices.iter().any(|index| *index == info.index))
            .unwrap_or(false),
        Selector::StartsWith(prefix) => info.name.starts_with(prefix),
        Selector::StartsWithAny(prefixes) => {
            prefixes.iter().any(|prefix| info.name.starts_with(prefix))
        }
        Selector::EndsWith(suffix) => info.name.ends_with(suffix),
        Selector::EndsWithAny(suffixes) => {
            suffixes.iter().any(|suffix| info.name.ends_with(suffix))
        }
        Selector::Contains(needle) => info.name.contains(needle),
        Selector::ContainsAny(needles) => needles.iter().any(|needle| info.name.contains(needle)),
        Selector::Matches(regex) => regex.is_match(info.name),
        Selector::ByDtype(dtypes) => dtypes.iter().any(|dtype| dtype == info.dtype),
        Selector::Numeric => is_numeric(info.dtype),
        Selector::Integer => is_integer(info.dtype),
        Selector::SignedInteger => is_signed_integer(info.dtype),
        Selector::UnsignedInteger => is_unsigned_integer(info.dtype),
        Selector::Float => is_float(info.dtype),
        Selector::Boolean => matches!(info.dtype, DataType::Boolean),
        Selector::String => matches!(info.dtype, DataType::String),
        Selector::Binary => matches!(info.dtype, DataType::Binary | DataType::BinaryOffset),
        Selector::Temporal => is_temporal(info.dtype),
        Selector::Date => matches!(info.dtype, DataType::Date),
        Selector::Datetime => matches!(info.dtype, DataType::Datetime(_, _)),
        Selector::Time => matches!(info.dtype, DataType::Time),
        Selector::Duration => matches!(info.dtype, DataType::Duration(_)),
        Selector::Categorical => matches!(info.dtype, DataType::Categorical(_, _)),
        Selector::Enum => matches!(info.dtype, DataType::Enum(_, _)),
        Selector::Decimal => matches!(info.dtype, DataType::Decimal(_, _)),
        Selector::Object => matches!(info.dtype, DataType::Object(_)),
        Selector::List => matches!(info.dtype, DataType::List(_)),
        Selector::Array => matches!(info.dtype, DataType::Array(_, _)),
        Selector::Struct => matches!(info.dtype, DataType::Struct(_)),
        Selector::Nested => matches!(
            info.dtype,
            DataType::List(_) | DataType::Array(_, _) | DataType::Struct(_)
        ),
        Selector::First => info.index == 0,
        Selector::Last => info.index + 1 == info.total,
        Selector::Or(selectors) => selectors
            .iter()
            .any(|sel| selector_matches(sel, info, resolved_indices)),
        Selector::And(selectors) => selectors
            .iter()
            .all(|sel| selector_matches(sel, info, resolved_indices)),
        Selector::Not(selector) => !selector_matches(selector, info, resolved_indices),
        Selector::Exclude { base, remove } => {
            selector_matches(base, info, resolved_indices)
                && !selector_matches(remove, info, resolved_indices)
        }
    }
}

fn is_numeric(dtype: &DataType) -> bool {
    is_integer(dtype) || is_float(dtype) || matches!(dtype, DataType::Decimal(_, _))
}

fn is_integer(dtype: &DataType) -> bool {
    is_signed_integer(dtype) || is_unsigned_integer(dtype)
}

fn is_signed_integer(dtype: &DataType) -> bool {
    matches!(
        dtype,
        DataType::Int8 | DataType::Int16 | DataType::Int32 | DataType::Int64 | DataType::Int128
    )
}

fn is_unsigned_integer(dtype: &DataType) -> bool {
    matches!(
        dtype,
        DataType::UInt8
            | DataType::UInt16
            | DataType::UInt32
            | DataType::UInt64
            | DataType::UInt128
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
