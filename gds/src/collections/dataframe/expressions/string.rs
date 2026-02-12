//! String namespace for expressions (py-polars inspired).

use polars::prelude::{
    lit, DataType, DataTypeExpr, Expr, StrptimeOptions, TimeUnit, TimeZone, UnicodeForm,
};

use crate::collections::dataframe::expressions::binary::BinaryEncoding;

#[derive(Debug, Clone)]
pub struct ExprString {
    expr: Expr,
}

impl ExprString {
    pub fn new(expr: Expr) -> Self {
        Self { expr }
    }

    pub fn expr(&self) -> &Expr {
        &self.expr
    }

    pub fn into_expr(self) -> Expr {
        self.expr
    }

    pub fn ends_with(self, sub: &str) -> Expr {
        self.expr.str().ends_with(lit(sub))
    }

    pub fn ends_with_expr(self, sub: Expr) -> Expr {
        self.expr.str().ends_with(sub)
    }

    pub fn starts_with(self, sub: &str) -> Expr {
        self.expr.str().starts_with(lit(sub))
    }

    pub fn starts_with_expr(self, sub: Expr) -> Expr {
        self.expr.str().starts_with(sub)
    }

    pub fn contains(self, pat: &str, literal: bool, strict: bool) -> Expr {
        if literal {
            self.expr.str().contains_literal(lit(pat))
        } else {
            self.expr.str().contains(lit(pat), strict)
        }
    }

    pub fn contains_expr(self, pat: Expr, literal: bool, strict: bool) -> Expr {
        if literal {
            self.expr.str().contains_literal(pat)
        } else {
            self.expr.str().contains(pat, strict)
        }
    }

    pub fn contains_any(self, patterns: Expr, ascii_case_insensitive: bool) -> Expr {
        self.expr
            .str()
            .contains_any(patterns, ascii_case_insensitive)
    }

    pub fn find(self, pat: &str, literal: bool, strict: bool) -> Expr {
        if literal {
            self.expr.str().find_literal(lit(pat))
        } else {
            self.expr.str().find(lit(pat), strict)
        }
    }

    pub fn find_expr(self, pat: Expr, literal: bool, strict: bool) -> Expr {
        if literal {
            self.expr.str().find_literal(pat)
        } else {
            self.expr.str().find(pat, strict)
        }
    }

    pub fn extract(self, pat: &str, group_index: usize) -> Expr {
        self.expr.str().extract(lit(pat), group_index)
    }

    pub fn extract_expr(self, pat: Expr, group_index: usize) -> Expr {
        self.expr.str().extract(pat, group_index)
    }

    pub fn extract_all(self, pat: &str) -> Expr {
        self.expr.str().extract_all(lit(pat))
    }

    pub fn extract_all_expr(self, pat: Expr) -> Expr {
        self.expr.str().extract_all(pat)
    }

    pub fn extract_many(
        self,
        patterns: Expr,
        ascii_case_insensitive: bool,
        overlapping: bool,
        leftmost: bool,
    ) -> Expr {
        let _ = leftmost;
        self.expr
            .str()
            .extract_many(patterns, ascii_case_insensitive, overlapping)
    }

    pub fn find_many(
        self,
        patterns: Expr,
        ascii_case_insensitive: bool,
        overlapping: bool,
        leftmost: bool,
    ) -> Expr {
        let _ = leftmost;
        self.expr
            .str()
            .find_many(patterns, ascii_case_insensitive, overlapping)
    }

    pub fn count_matches(self, pat: &str, literal: bool) -> Expr {
        self.expr.str().count_matches(lit(pat), literal)
    }

    pub fn count_matches_expr(self, pat: Expr, literal: bool) -> Expr {
        self.expr.str().count_matches(pat, literal)
    }

    pub fn replace(self, pat: &str, value: &str, literal: bool) -> Expr {
        self.expr.str().replace(lit(pat), lit(value), literal)
    }

    pub fn replace_expr(self, pat: Expr, value: Expr, literal: bool) -> Expr {
        self.expr.str().replace(pat, value, literal)
    }

    pub fn replace_n(self, pat: Expr, value: Expr, literal: bool, n: i64) -> Expr {
        self.expr.str().replace_n(pat, value, literal, n)
    }

    pub fn replace_all(self, pat: &str, value: &str, literal: bool) -> Expr {
        self.expr.str().replace_all(lit(pat), lit(value), literal)
    }

    pub fn replace_all_expr(self, pat: Expr, value: Expr, literal: bool) -> Expr {
        self.expr.str().replace_all(pat, value, literal)
    }

    pub fn replace_many(
        self,
        patterns: Expr,
        replace_with: Expr,
        ascii_case_insensitive: bool,
    ) -> Expr {
        self.expr
            .str()
            .replace_many(patterns, replace_with, ascii_case_insensitive)
    }

    pub fn strip_chars(self, matches: &str) -> Expr {
        self.expr.str().strip_chars(lit(matches))
    }

    pub fn strip_chars_expr(self, matches: Expr) -> Expr {
        self.expr.str().strip_chars(matches)
    }

    pub fn strip_chars_start(self, matches: &str) -> Expr {
        self.expr.str().strip_chars_start(lit(matches))
    }

    pub fn strip_chars_start_expr(self, matches: Expr) -> Expr {
        self.expr.str().strip_chars_start(matches)
    }

    pub fn strip_chars_end(self, matches: &str) -> Expr {
        self.expr.str().strip_chars_end(lit(matches))
    }

    pub fn strip_chars_end_expr(self, matches: Expr) -> Expr {
        self.expr.str().strip_chars_end(matches)
    }

    pub fn strip_prefix(self, prefix: &str) -> Expr {
        self.expr.str().strip_prefix(lit(prefix))
    }

    pub fn strip_prefix_expr(self, prefix: Expr) -> Expr {
        self.expr.str().strip_prefix(prefix)
    }

    pub fn strip_suffix(self, suffix: &str) -> Expr {
        self.expr.str().strip_suffix(lit(suffix))
    }

    pub fn strip_suffix_expr(self, suffix: Expr) -> Expr {
        self.expr.str().strip_suffix(suffix)
    }

    pub fn pad_start(self, length: i64, fill_char: char) -> Expr {
        self.expr.str().pad_start(lit(length), fill_char)
    }

    pub fn pad_start_expr(self, length: Expr, fill_char: char) -> Expr {
        self.expr.str().pad_start(length, fill_char)
    }

    pub fn pad_end(self, length: i64, fill_char: char) -> Expr {
        self.expr.str().pad_end(lit(length), fill_char)
    }

    pub fn pad_end_expr(self, length: Expr, fill_char: char) -> Expr {
        self.expr.str().pad_end(length, fill_char)
    }

    pub fn zfill(self, length: i64) -> Expr {
        self.expr.str().zfill(lit(length))
    }

    pub fn zfill_expr(self, length: Expr) -> Expr {
        self.expr.str().zfill(length)
    }

    pub fn split(self, by: &str) -> Expr {
        self.expr.str().split(lit(by))
    }

    pub fn split_expr(self, by: Expr) -> Expr {
        self.expr.str().split(by)
    }

    pub fn split_inclusive(self, by: &str) -> Expr {
        self.expr.str().split_inclusive(lit(by))
    }

    pub fn split_inclusive_expr(self, by: Expr) -> Expr {
        self.expr.str().split_inclusive(by)
    }

    pub fn split_regex(self, pat: Expr, _strict: bool) -> Expr {
        // Polars 0.52 does not provide split_regex, forward to `split` which accepts an Expr.
        self.expr.str().split(pat)
    }

    pub fn split_regex_inclusive(self, pat: Expr, _strict: bool) -> Expr {
        // Polars 0.52 does not provide split_regex_inclusive, forward to `split_inclusive`.
        self.expr.str().split_inclusive(pat)
    }

    pub fn split_exact(self, by: &str, n: usize) -> Expr {
        self.expr.str().split_exact(lit(by), n)
    }

    pub fn split_exact_expr(self, by: Expr, n: usize) -> Expr {
        self.expr.str().split_exact(by, n)
    }

    pub fn split_exact_inclusive(self, by: &str, n: usize) -> Expr {
        self.expr.str().split_exact_inclusive(lit(by), n)
    }

    pub fn split_exact_inclusive_expr(self, by: Expr, n: usize) -> Expr {
        self.expr.str().split_exact_inclusive(by, n)
    }

    pub fn splitn(self, by: &str, n: usize) -> Expr {
        self.expr.str().splitn(lit(by), n)
    }

    pub fn splitn_expr(self, by: Expr, n: usize) -> Expr {
        self.expr.str().splitn(by, n)
    }

    pub fn to_lowercase(self) -> Expr {
        self.expr.str().to_lowercase()
    }

    pub fn to_uppercase(self) -> Expr {
        self.expr.str().to_uppercase()
    }

    pub fn reverse(self) -> Expr {
        self.expr.str().reverse()
    }

    pub fn normalize(self, form: UnicodeForm) -> Expr {
        self.expr.str().normalize(form)
    }

    pub fn len_bytes(self) -> Expr {
        self.expr.str().len_bytes()
    }

    pub fn len_chars(self) -> Expr {
        self.expr.str().len_chars()
    }

    pub fn slice(self, offset: Expr, length: Expr) -> Expr {
        self.expr.str().slice(offset, length)
    }

    pub fn head(self, n: Expr) -> Expr {
        self.expr.str().head(n)
    }

    pub fn tail(self, n: Expr) -> Expr {
        self.expr.str().tail(n)
    }

    pub fn strptime(
        self,
        dtype: impl Into<DataTypeExpr>,
        options: StrptimeOptions,
        ambiguous: Expr,
    ) -> Expr {
        self.expr.str().strptime(dtype, options, ambiguous)
    }

    pub fn to_date(self, options: StrptimeOptions) -> Expr {
        self.expr.str().to_date(options)
    }

    pub fn to_datetime(
        self,
        time_unit: Option<TimeUnit>,
        time_zone: Option<TimeZone>,
        options: StrptimeOptions,
        ambiguous: Expr,
    ) -> Expr {
        self.expr
            .str()
            .to_datetime(time_unit, time_zone, options, ambiguous)
    }

    pub fn to_time(self, options: StrptimeOptions) -> Expr {
        self.expr.str().to_time(options)
    }

    pub fn to_decimal(self, scale: usize) -> Expr {
        self.expr.str().to_decimal(scale)
    }

    pub fn to_integer(self, base: i64, dtype: Option<DataType>, strict: bool) -> Expr {
        self.expr.str().to_integer(lit(base), dtype, strict)
    }

    pub fn to_integer_expr(self, base: Expr, dtype: Option<DataType>, strict: bool) -> Expr {
        self.expr.str().to_integer(base, dtype, strict)
    }

    pub fn hex_decode(self, strict: bool) -> Expr {
        self.expr.str().hex_decode(strict)
    }

    pub fn base64_decode(self, strict: bool) -> Expr {
        self.expr.str().base64_decode(strict)
    }

    pub fn json_decode(self, dtype: impl Into<DataTypeExpr>) -> Expr {
        self.expr.str().json_decode(dtype)
    }

    pub fn json_path_match(self, pat: Expr) -> Expr {
        self.expr.str().json_path_match(pat)
    }

    pub fn decode(self, encoding: BinaryEncoding, strict: bool) -> Expr {
        match encoding {
            BinaryEncoding::Hex => self.expr.str().hex_decode(strict),
            BinaryEncoding::Base64 => self.expr.str().base64_decode(strict),
        }
    }

    pub fn encode(self, encoding: BinaryEncoding) -> Expr {
        match encoding {
            BinaryEncoding::Hex => self.expr.str().hex_encode(),
            BinaryEncoding::Base64 => self.expr.str().base64_encode(),
        }
    }

    pub fn explode(self) -> Expr {
        self.expr.str().split(lit("")).explode()
    }

    pub fn join(self, delimiter: &str, ignore_nulls: bool) -> Expr {
        self.expr.str().join(delimiter, ignore_nulls)
    }

    pub fn concat(self, delimiter: Option<&str>, ignore_nulls: bool) -> Expr {
        self.join(delimiter.unwrap_or("-"), ignore_nulls)
    }

    pub fn escape_regex(self) -> Expr {
        self.expr.str().escape_regex()
    }
}
