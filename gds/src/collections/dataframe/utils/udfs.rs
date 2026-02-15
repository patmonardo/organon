//! UDF analysis helpers inspired by py-polars `_utils/udfs.py`.
//!
//! Rust cannot introspect closure bytecode the same way Python can, so this
//! module focuses on portable pieces: map-target classification, known
//! function rewrite suggestions, expression cleanup, and standardized warnings.

use std::fmt;

use once_cell::sync::Lazy;
use regex::{Captures, Regex};

use crate::collections::dataframe::errors::PolarsInefficientMapWarning;
use crate::collections::dataframe::utils::various::{in_terminal_that_supports_colour, re_escape};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MapTarget {
    Expr,
    Frame,
    Series,
}

impl MapTarget {
    pub fn api_label(self) -> (&'static str, &'static str) {
        match self {
            Self::Expr => ("Expr", "expressions"),
            Self::Frame => ("DataFrame", "dataframe"),
            Self::Series => ("Series", "series"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UdfRewriteSuggestion {
    pub original_call: String,
    pub suggested_expression: String,
    pub udf_label: String,
    pub map_target: MapTarget,
}

impl UdfRewriteSuggestion {
    pub fn warning_message(&self) -> String {
        let (cls, apitype) = self.map_target.api_label();
        let (before, after) = if in_terminal_that_supports_colour() {
            (
                format!("  \x1b[31m- {}\x1b[0m", self.original_call),
                format!("  \x1b[32m+ {}\x1b[0m", self.suggested_expression),
            )
        } else {
            (
                format!("  - {}", self.original_call),
                format!("  + {}", self.suggested_expression),
            )
        };

        format!(
            "\n{cls}.map_elements is significantly slower than the native {apitype} API.\n\
Only use it when you cannot express the logic natively.\n\
Replace this expression...\n{before}\nwith this one instead:\n{after}"
        )
    }

    pub fn as_warning(&self) -> PolarsInefficientMapWarning {
        PolarsInefficientMapWarning::new(self.warning_message())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UdfError {
    UnsupportedMapTarget(MapTarget),
}

impl fmt::Display for UdfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedMapTarget(target) => {
                write!(f, "UDF rewrite not supported for map target: {target:?}")
            }
        }
    }
}

impl std::error::Error for UdfError {}

static RE_IMPLICIT_BOOL: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"pl\.col\("([^"]*)"\) & pl\.col\("([^"]*)"\)\.(.+)"#)
        .expect("valid implicit bool regex")
});
static RE_STRIP_BOOL: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^bool\((.+)\)$").expect("valid bool-strip regex"));
static RE_SERIES_NAMES: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(s|srs\d?|series)\.").expect("valid series-name regex"));

static MODULE_FUNC_TO_EXPR: Lazy<std::collections::HashMap<&'static str, &'static str>> =
    Lazy::new(|| {
        std::collections::HashMap::from([
            ("math.acos", "arccos"),
            ("math.acosh", "arccosh"),
            ("math.asin", "arcsin"),
            ("math.asinh", "arcsinh"),
            ("math.atan", "arctan"),
            ("math.atanh", "arctanh"),
            ("json.loads", "str.json_decode"),
        ])
    });

/// Try to produce a native-expression suggestion for a bare UDF function name.
///
/// Example inputs: `math.sin`, `json.loads`, `numpy.sqrt`, `np.log1p`, `abs`.
pub fn suggest_from_function_name(
    function_name: &str,
    col: &str,
    map_target: MapTarget,
) -> Result<Option<UdfRewriteSuggestion>, UdfError> {
    if map_target == MapTarget::Frame {
        return Err(UdfError::UnsupportedMapTarget(map_target));
    }

    let normalized = function_name.trim();

    let suggested = if let Some(mapped) = MODULE_FUNC_TO_EXPR.get(normalized) {
        format!("{}.{}()", target_name(col, "", map_target), mapped)
    } else if let Some(rest) = normalized.strip_prefix("math.") {
        format!("{}.{}()", target_name(col, "", map_target), rest)
    } else if let Some(rest) = normalized
        .strip_prefix("np.")
        .or_else(|| normalized.strip_prefix("numpy."))
    {
        format!("{}.{}()", target_name(col, "", map_target), rest)
    } else {
        match normalized {
            "abs" => format!("{}.abs()", target_name(col, "", map_target)),
            "int" => format!("{}.cast(pl.Int64)", target_name(col, "", map_target)),
            "float" => format!("{}.cast(pl.Float64)", target_name(col, "", map_target)),
            "str" => format!("{}.cast(pl.String)", target_name(col, "", map_target)),
            _ => return Ok(None),
        }
    };

    let original_call = format!(
        "{}.map_elements({normalized})",
        target_name(col, "", map_target)
    );
    Ok(Some(UdfRewriteSuggestion {
        original_call,
        suggested_expression: suggested,
        udf_label: normalized.to_string(),
        map_target,
    }))
}

/// Remove redundant bool wrapping from expression text.
pub fn strip_bool_wrapper(expr: &str) -> String {
    RE_STRIP_BOOL
        .captures(expr)
        .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
        .unwrap_or_else(|| expr.to_string())
}

/// Drop redundant implicit-bool constructs that arise in expression rewrites.
pub fn omit_implicit_bool(expr: &str) -> String {
    let mut out = expr.to_string();
    loop {
        let next = RE_IMPLICIT_BOOL
            .replace_all(&out, |caps: &Captures<'_>| {
                let lhs = caps.get(1).map(|m| m.as_str()).unwrap_or_default();
                let rhs = caps.get(2).map(|m| m.as_str()).unwrap_or_default();
                let suffix = caps.get(3).map(|m| m.as_str()).unwrap_or_default();

                if lhs == rhs {
                    format!(r#"pl.col("{}").{}"#, lhs, suffix)
                } else {
                    caps.get(0)
                        .map(|m| m.as_str().to_string())
                        .unwrap_or_default()
                }
            })
            .to_string();

        if next == out {
            break;
        }
        out = next;
    }
    out
}

/// Replace `pl.col(old_col)` references with `pl.col(new_col)` in an expression.
pub fn rewrite_col_reference(expr: &str, old_col: &str, new_col: &str) -> String {
    if old_col == new_col {
        return expr.to_string();
    }

    let pattern = format!(r#"pl\.col\("{}"\)"#, re_escape(old_col));
    Regex::new(&pattern)
        .ok()
        .map(|rx| {
            rx.replace_all(expr, format!(r#"pl.col("{}")"#, re_escape(new_col)))
                .to_string()
        })
        .unwrap_or_else(|| expr.to_string())
}

/// Infer a readable map target name used in generated suggestions.
pub fn target_name(col: &str, expression: &str, map_target: MapTarget) -> String {
    match map_target {
        MapTarget::Expr => format!(r#"pl.col("{}")"#, col),
        MapTarget::Series => {
            if let Some(name) = RE_SERIES_NAMES
                .captures(expression)
                .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
            {
                name
            } else {
                "s".to_string()
            }
        }
        MapTarget::Frame => "df".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_bool_wrapper() {
        assert_eq!(strip_bool_wrapper("bool(pl.col(\"x\"))"), "pl.col(\"x\")");
        assert_eq!(strip_bool_wrapper("pl.col(\"x\")"), "pl.col(\"x\")");
    }

    #[test]
    fn test_omit_implicit_bool() {
        let input = r#"pl.col("d") & pl.col("d").dt.date()"#;
        let output = omit_implicit_bool(input);
        assert_eq!(output, r#"pl.col("d").dt.date()"#);
    }

    #[test]
    fn test_suggest_from_function_name_builtin() {
        let suggestion = suggest_from_function_name("abs", "x", MapTarget::Expr)
            .unwrap()
            .expect("should suggest");
        assert!(suggestion.suggested_expression.contains(".abs()"));
    }

    #[test]
    fn test_suggest_from_function_name_module() {
        let suggestion = suggest_from_function_name("json.loads", "x", MapTarget::Expr)
            .unwrap()
            .expect("should suggest");
        assert!(suggestion.suggested_expression.contains("str.json_decode"));
    }
}
