//! Length helpers.

use polars::prelude::{len as pl_len, Expr};

pub fn len() -> Expr {
    pl_len()
}
