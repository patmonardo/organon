//! Categorical namespace for expressions (py-polars inspired).

use polars::prelude::Expr;

#[derive(Debug, Clone)]
pub struct ExprCategorical {
    expr: Expr,
}

impl ExprCategorical {
    pub fn new(expr: Expr) -> Self {
        Self { expr }
    }

    pub fn expr(&self) -> &Expr {
        &self.expr
    }

    pub fn into_expr(self) -> Expr {
        self.expr
    }

    pub fn get_categories(self) -> Expr {
        self.expr.cat().get_categories()
    }

    #[cfg(feature = "strings")]
    pub fn len_bytes(self) -> Expr {
        self.expr.cat().len_bytes()
    }

    #[cfg(feature = "strings")]
    pub fn len_chars(self) -> Expr {
        self.expr.cat().len_chars()
    }

    #[cfg(feature = "strings")]
    pub fn starts_with(self, prefix: &str) -> Expr {
        self.expr.cat().starts_with(prefix.to_string())
    }

    #[cfg(feature = "strings")]
    pub fn ends_with(self, suffix: &str) -> Expr {
        self.expr.cat().ends_with(suffix.to_string())
    }

    #[cfg(feature = "strings")]
    pub fn slice(self, offset: i64, length: Option<usize>) -> Expr {
        self.expr.cat().slice(offset, length)
    }
}
