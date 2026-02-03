//! String namespace for SeriesModel (py-polars inspired).

use polars::prelude::{
    DataTypeExpr, Expr, PolarsResult, Series, StrptimeOptions, TimeUnit, TimeZone,
};

use crate::collections::dataframe::expr::SeriesExprString;
use crate::collections::dataframe::expressions::string::ExprString;

#[derive(Debug, Clone)]
pub struct StringNameSpace {
    series: Series,
}

impl StringNameSpace {
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
        F: FnOnce(ExprString) -> Expr,
    {
        SeriesExprString::new(self.series.clone()).apply(f)
    }

    pub fn ends_with(&self, sub: &str) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.ends_with(sub))
    }

    pub fn ends_with_expr(&self, sub: Expr) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.ends_with_expr(sub))
    }

    pub fn starts_with(&self, sub: &str) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.starts_with(sub))
    }

    pub fn starts_with_expr(&self, sub: Expr) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.starts_with_expr(sub))
    }

    pub fn contains(&self, pat: &str, literal: bool, strict: bool) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.contains(pat, literal, strict))
    }

    pub fn contains_expr(&self, pat: Expr, literal: bool, strict: bool) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.contains_expr(pat, literal, strict))
    }

    pub fn contains_any(
        &self,
        _patterns: Expr,
        _ascii_case_insensitive: bool,
    ) -> PolarsResult<Series> {
        todo!()
    }

    pub fn find(&self, pat: &str, literal: bool, strict: bool) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.find(pat, literal, strict))
    }

    pub fn find_expr(&self, pat: Expr, literal: bool, strict: bool) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.find_expr(pat, literal, strict))
    }

    pub fn extract(&self, pat: &str, group_index: usize) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.extract(pat, group_index))
    }

    pub fn extract_expr(&self, pat: Expr, group_index: usize) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.extract_expr(pat, group_index))
    }

    pub fn extract_all(&self, pat: &str) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.extract_all(pat))
    }

    pub fn extract_all_expr(&self, pat: Expr) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.extract_all_expr(pat))
    }

    pub fn extract_many(
        &self,
        _patterns: Expr,
        _ascii_case_insensitive: bool,
        _overlapping: bool,
        _leftmost: bool,
    ) -> PolarsResult<Series> {
        todo!()
    }

    pub fn find_many(
        &self,
        _patterns: Expr,
        _ascii_case_insensitive: bool,
        _overlapping: bool,
        _leftmost: bool,
    ) -> PolarsResult<Series> {
        todo!()
    }

    pub fn count_matches(&self, pat: &str, literal: bool) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.count_matches(pat, literal))
    }

    pub fn count_matches_expr(&self, pat: Expr, literal: bool) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.count_matches_expr(pat, literal))
    }

    pub fn replace(&self, pat: &str, value: &str, literal: bool) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.replace(pat, value, literal))
    }

    pub fn replace_expr(&self, pat: Expr, value: Expr, literal: bool) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.replace_expr(pat, value, literal))
    }

    pub fn replace_n(&self, pat: Expr, value: Expr, literal: bool, n: i64) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.replace_n(pat, value, literal, n))
    }

    pub fn replace_all(&self, pat: &str, value: &str, literal: bool) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.replace_all(pat, value, literal))
    }

    pub fn replace_all_expr(&self, pat: Expr, value: Expr, literal: bool) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.replace_all_expr(pat, value, literal))
    }

    pub fn strip_chars(&self, matches: &str) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.strip_chars(matches))
    }

    pub fn strip_chars_expr(&self, matches: Expr) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.strip_chars_expr(matches))
    }

    pub fn strip_chars_start(&self, matches: &str) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.strip_chars_start(matches))
    }

    pub fn strip_chars_start_expr(&self, matches: Expr) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.strip_chars_start_expr(matches))
    }

    pub fn strip_chars_end(&self, matches: &str) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.strip_chars_end(matches))
    }

    pub fn strip_chars_end_expr(&self, matches: Expr) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.strip_chars_end_expr(matches))
    }

    pub fn strip_prefix(&self, prefix: &str) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.strip_prefix(prefix))
    }

    pub fn strip_prefix_expr(&self, prefix: Expr) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.strip_prefix_expr(prefix))
    }

    pub fn strip_suffix(&self, suffix: &str) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.strip_suffix(suffix))
    }

    pub fn strip_suffix_expr(&self, suffix: Expr) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.strip_suffix_expr(suffix))
    }

    pub fn split(&self, by: &str) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.split(by))
    }

    pub fn split_expr(&self, by: Expr) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.split_expr(by))
    }

    pub fn split_inclusive(&self, by: &str) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.split_inclusive(by))
    }

    pub fn split_inclusive_expr(&self, by: Expr) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.split_inclusive_expr(by))
    }

    pub fn split_regex(&self, pat: Expr, strict: bool) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.split_regex(pat, strict))
    }

    pub fn split_regex_inclusive(&self, pat: Expr, strict: bool) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.split_regex_inclusive(pat, strict))
    }

    pub fn split_exact(&self, by: &str, n: usize) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.split_exact(by, n))
    }

    pub fn split_exact_expr(&self, by: Expr, n: usize) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.split_exact_expr(by, n))
    }

    pub fn split_exact_inclusive(&self, by: &str, n: usize) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.split_exact_inclusive(by, n))
    }

    pub fn split_exact_inclusive_expr(&self, by: Expr, n: usize) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.split_exact_inclusive_expr(by, n))
    }

    pub fn splitn(&self, by: &str, n: usize) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.splitn(by, n))
    }

    pub fn splitn_expr(&self, by: Expr, n: usize) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.splitn_expr(by, n))
    }

    pub fn to_lowercase(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.to_lowercase())
    }

    pub fn to_uppercase(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.to_uppercase())
    }

    pub fn len_bytes(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.len_bytes())
    }

    pub fn len_chars(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.len_chars())
    }

    pub fn slice(&self, offset: Expr, length: Expr) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.slice(offset, length))
    }

    pub fn head(&self, n: Expr) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.head(n))
    }

    pub fn tail(&self, n: Expr) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.tail(n))
    }

    pub fn strptime(
        &self,
        dtype: impl Into<DataTypeExpr>,
        options: StrptimeOptions,
        ambiguous: Expr,
    ) -> PolarsResult<Series> {
        let dtype = dtype.into();
        self.apply_expr(|expr| expr.strptime(dtype, options, ambiguous))
    }

    pub fn to_date(&self, options: StrptimeOptions) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.to_date(options))
    }

    pub fn to_datetime(
        &self,
        time_unit: Option<TimeUnit>,
        time_zone: Option<TimeZone>,
        options: StrptimeOptions,
        ambiguous: Expr,
    ) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.to_datetime(time_unit, time_zone, options, ambiguous))
    }

    pub fn to_time(&self, options: StrptimeOptions) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.to_time(options))
    }

    pub fn to_decimal(&self, scale: usize) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.to_decimal(scale))
    }

    pub fn hex_decode(&self, _strict: bool) -> PolarsResult<Series> {
        todo!()
    }

    pub fn base64_decode(&self, _strict: bool) -> PolarsResult<Series> {
        todo!()
    }

    pub fn json_decode(&self, _dtype: impl Into<DataTypeExpr>) -> PolarsResult<Series> {
        todo!()
    }

    pub fn json_path_match(&self, _pat: Expr) -> PolarsResult<Series> {
        todo!()
    }

    pub fn join(&self, delimiter: &str, ignore_nulls: bool) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.join(delimiter, ignore_nulls))
    }

    pub fn escape_regex(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.escape_regex())
    }
}
