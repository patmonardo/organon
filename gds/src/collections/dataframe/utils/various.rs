//! General utility helpers inspired by py-polars `_utils/various.py`.

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::Hash;
use std::io::IsTerminal;
use std::path::{Path, PathBuf};
use std::str::FromStr;

/// Input shape for `extend_bool`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoolInput {
    One(bool),
    Many(Vec<bool>),
}

/// Byte-size units used by `scale_bytes`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizeUnit {
    Bytes,
    Kilobytes,
    Megabytes,
    Gigabytes,
    Terabytes,
}

impl FromStr for SizeUnit {
    type Err = VariousError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "b" | "bytes" => Ok(Self::Bytes),
            "kb" | "kilobytes" => Ok(Self::Kilobytes),
            "mb" | "megabytes" => Ok(Self::Megabytes),
            "gb" | "gigabytes" => Ok(Self::Gigabytes),
            "tb" | "terabytes" => Ok(Self::Terabytes),
            other => Err(VariousError::InvalidSizeUnit(other.to_string())),
        }
    }
}

/// Errors returned by helpers in this module.
#[derive(Debug, Clone, PartialEq)]
pub enum VariousError {
    MissingHomeDirectory,
    IsDirectory(PathBuf),
    LengthMismatch {
        value_name: String,
        got: usize,
        match_name: String,
        expected: usize,
    },
    InvalidPercentile(f64),
    InvalidSizeUnit(String),
}

impl fmt::Display for VariousError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingHomeDirectory => {
                write!(f, "cannot expand '~' because HOME is not set")
            }
            Self::IsDirectory(path) => {
                write!(f, "expected a file path; {:?} is a directory", path)
            }
            Self::LengthMismatch {
                value_name,
                got,
                match_name,
                expected,
            } => {
                write!(
                    f,
                    "the length of `{}` ({}) does not match the length of `{}` ({})",
                    value_name, got, match_name, expected
                )
            }
            Self::InvalidPercentile(value) => {
                write!(
                    f,
                    "`percentiles` must all be in the range [0, 1], got {value}"
                )
            }
            Self::InvalidSizeUnit(unit) => {
                write!(
                    f,
                    "`unit` must be one of {{'b', 'kb', 'mb', 'gb', 'tb'}}, got {unit:?}"
                )
            }
        }
    }
}

impl std::error::Error for VariousError {}

/// Expand `~` and optionally verify that the resulting path is not a directory.
pub fn normalize_filepath(
    path: impl AsRef<Path>,
    check_not_directory: bool,
) -> Result<PathBuf, VariousError> {
    let raw = path.as_ref().to_string_lossy();
    let expanded = if raw == "~" || raw.starts_with("~/") {
        let home = std::env::var("HOME").map_err(|_| VariousError::MissingHomeDirectory)?;
        let suffix = raw.strip_prefix('~').unwrap_or_default();
        format!("{home}{suffix}")
    } else {
        raw.to_string()
    };

    let path_buf = PathBuf::from(expanded);
    if check_not_directory && path_buf.exists() && path_buf.is_dir() {
        return Err(VariousError::IsDirectory(path_buf));
    }

    Ok(path_buf)
}

/// Parse a semantic-ish version string into comparable integer components.
///
/// Example: `"1.23.0-beta.2"` -> `[1, 23, 0, 2]`.
pub fn parse_version(version: &str) -> Vec<u32> {
    version
        .split('.')
        .map(|part| {
            part.chars()
                .filter(|ch| ch.is_ascii_digit())
                .collect::<String>()
        })
        .map(|digits| {
            if digits.is_empty() {
                0
            } else {
                digits.parse::<u32>().unwrap_or(0)
            }
        })
        .collect()
}

/// Return unique values while preserving order.
pub fn ordered_unique<T>(values: &[T]) -> Vec<T>
where
    T: Clone + Eq + Hash,
{
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for value in values {
        if seen.insert(value.clone()) {
            out.push(value.clone());
        }
    }
    out
}

/// Ensure name uniqueness by appending a suffix to subsequent duplicates.
///
/// Example: `["a", "a", "a"]` -> `["a", "a1", "a2"]`.
pub fn deduplicate_names(names: impl IntoIterator<Item = String>) -> Vec<String> {
    let mut seen: HashMap<String, usize> = HashMap::new();
    let mut deduped = Vec::new();

    for name in names {
        let count = seen.entry(name.clone()).or_insert(0);
        if *count == 0 {
            deduped.push(name);
        } else {
            deduped.push(format!("{}{}", name, *count));
        }
        *count += 1;
    }

    deduped
}

/// Expand a bool-or-bool-sequence to an exact expected length.
pub fn extend_bool(
    value: BoolInput,
    n_match: usize,
    value_name: &str,
    match_name: &str,
) -> Result<Vec<bool>, VariousError> {
    let values = match value {
        BoolInput::One(single) => vec![single; n_match],
        BoolInput::Many(values) => values,
    };

    if values.len() != n_match {
        return Err(VariousError::LengthMismatch {
            value_name: value_name.to_string(),
            got: values.len(),
            match_name: match_name.to_string(),
            expected: n_match,
        });
    }

    Ok(values)
}

/// Normalize percentile input, optionally injecting median (`0.5`).
pub fn parse_percentiles(
    percentiles: Option<Vec<f64>>,
    inject_median: bool,
) -> Result<Vec<f64>, VariousError> {
    let values = percentiles.unwrap_or_default();

    for value in &values {
        if !(0.0..=1.0).contains(value) {
            return Err(VariousError::InvalidPercentile(*value));
        }
    }

    let mut under = values
        .iter()
        .copied()
        .filter(|p| *p < 0.5)
        .collect::<Vec<_>>();
    let mut at_or_over = values
        .iter()
        .copied()
        .filter(|p| *p >= 0.5)
        .collect::<Vec<_>>();
    under.sort_by(f64::total_cmp);
    at_or_over.sort_by(f64::total_cmp);

    if inject_median && (at_or_over.is_empty() || at_or_over[0] != 0.5) {
        at_or_over.insert(0, 0.5);
    }

    under.extend(at_or_over);
    Ok(under)
}

/// Escape only regex metacharacters relevant to Rust regex syntax.
pub fn re_escape(input: &str) -> String {
    let rust_metachars: [char; 18] = [
        '\\', '?', '(', ')', '|', '[', ']', '{', '}', '^', '$', '#', '&', '~', '.', '+', '*', '-',
    ];
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        if rust_metachars.contains(&ch) {
            out.push('\\');
        }
        out.push(ch);
    }
    out
}

/// Best-effort detection of an interactive terminal with color support.
pub fn in_terminal_that_supports_colour() -> bool {
    if !std::io::stdout().is_terminal() {
        return std::env::var("PYCHARM_HOSTED").as_deref() == Ok("1");
    }

    if cfg!(windows) {
        std::env::var_os("ANSICON").is_some()
            || std::env::var_os("WT_SESSION").is_some()
            || std::env::var("TERM_PROGRAM").as_deref() == Ok("vscode")
            || std::env::var("TERM").as_deref() == Ok("xterm-256color")
    } else {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordered_unique() {
        let values = vec![1, 2, 2, 3, 1, 4];
        assert_eq!(ordered_unique(&values), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_deduplicate_names() {
        let names = vec!["a".to_string(), "a".to_string(), "a".to_string()];
        assert_eq!(deduplicate_names(names), vec!["a", "a1", "a2"]);
    }

    #[test]
    fn test_parse_percentiles() {
        let values = parse_percentiles(Some(vec![0.9, 0.1]), true).unwrap();
        assert_eq!(values, vec![0.1, 0.5, 0.9]);
    }

    #[test]
    fn test_re_escape() {
        assert_eq!(re_escape("a+b(c)"), "a\\+b\\(c\\)");
    }
}
