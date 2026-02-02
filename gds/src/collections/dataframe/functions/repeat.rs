//! Repeat helpers.

use polars::prelude::{repeat as pl_repeat, Expr};

pub fn repeat<E: Into<Expr>>(value: E, n: Expr) -> Expr {
    pl_repeat(value, n)
}
