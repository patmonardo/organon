//! List namespace for expressions (py-polars inspired).

use polars::prelude::Expr;

#[derive(Debug, Clone)]
pub struct ExprList {
    expr: Expr,
}

impl ExprList {
    pub fn new(expr: Expr) -> Self {
        Self { expr }
    }

    pub fn expr(&self) -> &Expr {
        &self.expr
    }

    pub fn into_expr(self) -> Expr {
        self.expr
    }
}
