//! Linear space helpers.

use polars::lazy::dsl::{linear_space as pl_linear_space, linear_spaces as pl_linear_spaces};
use polars::prelude::{ClosedInterval, Expr, PolarsResult};

pub fn linear_space(start: Expr, end: Expr, num_samples: Expr, closed: ClosedInterval) -> Expr {
    pl_linear_space(start, end, num_samples, closed)
}

pub fn linear_spaces(
    start: Expr,
    end: Expr,
    num_samples: Expr,
    closed: ClosedInterval,
    as_array: bool,
) -> PolarsResult<Expr> {
    pl_linear_spaces(start, end, num_samples, closed, as_array)
}
