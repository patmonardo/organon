//! Struct namespace for GDSSeries (py-polars inspired).

use polars::prelude::{Expr, PlSmallStr, PolarsResult, Series};

use crate::collections::dataframe::expr::SeriesExprStruct;
use crate::collections::dataframe::expressions::structure::ExprStruct;

#[derive(Debug, Clone)]
pub struct StructNameSpace {
    series: Series,
}

impl StructNameSpace {
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
        F: FnOnce(ExprStruct) -> Expr,
    {
        SeriesExprStruct::new(self.series.clone()).apply(f)
    }

    pub fn field(&self, name: &str) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.field(name))
    }

    pub fn field_many(&self, names: &[&str]) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.field_many(names))
    }

    pub fn field_by_index(&self, index: i64) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.field_by_index(index))
    }

    pub fn unnest(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.unnest())
    }

    pub fn rename_fields(&self, names: &[&str]) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.rename_fields(names))
    }

    pub fn rename_fields_pl(&self, names: &[PlSmallStr]) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.rename_fields_pl(names))
    }

    pub fn json_encode(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.json_encode())
    }

    pub fn with_fields(&self, fields: Vec<Expr>) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.with_fields(fields))
    }
}
