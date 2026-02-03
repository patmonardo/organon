//! Binary namespace for SeriesModel (py-polars inspired).

use polars::prelude::{Expr, PolarsResult, Series};

use crate::collections::dataframe::expr::SeriesExprBinary;
use crate::collections::dataframe::expressions::binary::{
    BinaryEncoding, BinaryEndianness, BinarySizeUnit, ExprBinary,
};

#[derive(Debug, Clone)]
pub struct BinaryNameSpace {
    series: Series,
}

impl BinaryNameSpace {
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
        F: FnOnce(ExprBinary) -> Expr,
    {
        SeriesExprBinary::new(self.series.clone()).apply(f)
    }

    pub fn contains(&self, literal: &[u8]) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.contains(literal))
    }

    pub fn contains_expr(&self, literal: Expr) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.contains_expr(literal))
    }

    pub fn ends_with(&self, suffix: &[u8]) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.ends_with(suffix))
    }

    pub fn ends_with_expr(&self, suffix: Expr) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.ends_with_expr(suffix))
    }

    pub fn starts_with(&self, prefix: &[u8]) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.starts_with(prefix))
    }

    pub fn starts_with_expr(&self, prefix: Expr) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.starts_with_expr(prefix))
    }

    pub fn size_bytes(&self) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.size_bytes())
    }

    pub fn size(&self, unit: BinarySizeUnit) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.size(unit))
    }

    pub fn decode(&self, encoding: BinaryEncoding, strict: bool) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.decode(encoding, strict))
    }

    pub fn encode(&self, encoding: BinaryEncoding) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.encode(encoding))
    }

    pub fn reinterpret(
        &self,
        dtype: impl Into<polars::prelude::DataTypeExpr>,
        endianness: BinaryEndianness,
    ) -> PolarsResult<Series> {
        let dtype = dtype.into();
        self.apply_expr(|expr| expr.reinterpret(dtype, endianness))
    }

    pub fn slice(&self, offset: i64, length: i64) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.slice(offset, length))
    }

    pub fn head(&self, n: i64) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.head(n))
    }

    pub fn tail(&self, n: i64) -> PolarsResult<Series> {
        self.apply_expr(|expr| expr.tail(n))
    }
}
