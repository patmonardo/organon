//! Integer range helpers.

use polars::lazy::dsl::{
    arange as pl_arange, int_range as pl_int_range, int_ranges as pl_int_ranges, DataTypeExpr,
};
use polars::prelude::{DataType, Expr};

pub fn arange(start: Expr, end: Expr, step: i64, dtype: DataType) -> Expr {
    pl_arange(start, end, step, dtype)
}

pub fn int_range<D: Into<DataTypeExpr>>(start: Expr, end: Expr, step: i64, dtype: D) -> Expr {
    pl_int_range(start, end, step, dtype)
}

pub fn int_ranges<D: Into<DataTypeExpr>>(start: Expr, end: Expr, step: Expr, dtype: D) -> Expr {
    pl_int_ranges(start, end, step, dtype)
}
