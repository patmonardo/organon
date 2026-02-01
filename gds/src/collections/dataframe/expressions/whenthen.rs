//! When/then expression helpers (py-polars inspired).

use polars::prelude::{Expr, When};

#[derive(Clone)]
pub struct WhenExpr {
    when: When,
}

impl WhenExpr {
    pub fn new(when: When) -> Self {
        Self { when }
    }

    pub fn then(self, expr: Expr) -> ThenExpr {
        ThenExpr::new(self.when, expr)
    }
}

#[derive(Clone)]
pub struct ThenExpr {
    when: When,
    then_expr: Expr,
}

impl ThenExpr {
    pub fn new(when: When, then_expr: Expr) -> Self {
        Self { when, then_expr }
    }

    pub fn otherwise(self, expr: Expr) -> Expr {
        self.when.then(self.then_expr).otherwise(expr)
    }
}
