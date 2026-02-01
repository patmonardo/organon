//! Meta namespace for expressions (py-polars inspired).

use polars::prelude::Expr;

#[derive(Debug, Clone)]
pub struct ExprMeta {
    expr: Expr,
}

impl ExprMeta {
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
