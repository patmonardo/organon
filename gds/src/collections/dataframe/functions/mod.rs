//! Expression helpers and selectors for Polars-backed DataFrames.

use polars::prelude::Expr;

pub type PolarsExpr = Expr;

pub mod aggregation;
pub mod col;
pub mod datatype;
pub mod expr;
pub mod len;
pub mod lit;
pub mod misc;
pub mod range;
pub mod repeat;
pub mod structure;
pub mod temporal;
pub mod whenthen;

pub use aggregation::*;
pub use col::{col, cols, expr_col, expr_cols};
pub use datatype::{dtype_of, self_dtype, struct_with_fields};
pub use expr::*;
pub use len::len;
pub use lit::lit;
pub use misc::*;
pub use range::*;
pub use repeat::repeat;
pub use structure::{record, structure};
pub use temporal::{date, datetime, duration, time};
pub use whenthen::{expr_when, when};
