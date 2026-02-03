//! Extension namespace for expressions (py-polars inspired).

use polars::prelude::{DataTypeExpr, Expr};

#[derive(Debug, Clone)]
pub struct ExprExt {
    expr: Expr,
}

impl ExprExt {
    pub fn new(expr: Expr) -> Self {
        Self { expr }
    }

    pub fn expr(&self) -> &Expr {
        &self.expr
    }

    pub fn into_expr(self) -> Expr {
        self.expr
    }

    /// Convert to an extension `dtype` (seed pass: cast to dtype expr).
    pub fn to(&self, dtype: impl Into<DataTypeExpr>) -> Expr {
        self.expr.clone().cast(dtype.into())
    }

    /// Get the storage values of an extension dtype (seed pass: identity).
    pub fn storage(&self) -> Expr {
        self.expr.clone()
    }
}
