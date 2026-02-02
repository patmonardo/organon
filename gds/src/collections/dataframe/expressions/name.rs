//! Name namespace for expressions (py-polars inspired).

use polars::polars_utils::pl_str::PlSmallStr;
use polars::prelude::{Expr, PlanCallback};

#[derive(Debug, Clone)]
pub struct ExprName {
    expr: Expr,
}

impl ExprName {
    pub fn new(expr: Expr) -> Self {
        Self { expr }
    }

    pub fn expr(&self) -> &Expr {
        &self.expr
    }

    pub fn into_expr(self) -> Expr {
        self.expr
    }

    pub fn keep(self) -> Expr {
        self.expr.name().keep()
    }

    pub fn map(self, function: PlanCallback<PlSmallStr, PlSmallStr>) -> Expr {
        self.expr.name().map(function)
    }

    pub fn prefix(self, prefix: &str) -> Expr {
        self.expr.name().prefix(prefix)
    }

    pub fn suffix(self, suffix: &str) -> Expr {
        self.expr.name().suffix(suffix)
    }

    pub fn replace(self, pattern: &str, value: &str, literal: bool) -> Expr {
        self.expr.name().replace(pattern, value, literal)
    }

    pub fn to_lowercase(self) -> Expr {
        self.expr.name().to_lowercase()
    }

    pub fn to_uppercase(self) -> Expr {
        self.expr.name().to_uppercase()
    }

    pub fn map_fields(self, function: PlanCallback<PlSmallStr, PlSmallStr>) -> Expr {
        self.expr.name().map_fields(function)
    }

    pub fn prefix_fields(self, prefix: &str) -> Expr {
        self.expr.name().prefix_fields(prefix)
    }

    pub fn suffix_fields(self, suffix: &str) -> Expr {
        self.expr.name().suffix_fields(suffix)
    }
}
