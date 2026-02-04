//! Categorical namespace for GDSSeries (py-polars inspired).

use polars::prelude::{Expr, PolarsResult, Series};

use crate::collections::dataframe::expr::SeriesExprCategorical;
use crate::collections::dataframe::expressions::categorical::ExprCategorical;

#[derive(Debug, Clone)]
pub struct CategoricalNameSpace {
    series: Series,
}

impl CategoricalNameSpace {
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
        F: FnOnce(ExprCategorical) -> Expr,
    {
        SeriesExprCategorical::new(self.series.clone()).apply(f)
    }

    pub fn get_categories(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.get_categories())
    }

    pub fn len_bytes(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.len_bytes())
    }

    pub fn len_chars(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.len_chars())
    }

    pub fn starts_with(&self, prefix: &str) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.starts_with(prefix))
    }

    pub fn ends_with(&self, suffix: &str) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.ends_with(suffix))
    }

    pub fn slice(&self, offset: i64, length: Option<usize>) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.slice(offset, length))
    }
}
