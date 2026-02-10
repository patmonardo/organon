//! GraphFrame expression facade.
//!
//! GraphFrame is a Polars-backed graph table surface. This module provides
//! graphframe-flavored `polars::Expr` namespaces.

use polars::prelude::{col, Expr};

#[derive(Debug, Clone)]
pub struct GraphFrameExprNameSpace {
    expr: Expr,
}

impl GraphFrameExprNameSpace {
    pub fn new(expr: Expr) -> Self {
        Self { expr }
    }

    pub fn col(name: &str) -> Self {
        Self::new(col(name))
    }

    pub fn expr(&self) -> Expr {
        self.expr.clone()
    }
}

pub trait ExprGraphFrameExt {
    fn gf(self) -> GraphFrameExprNameSpace;
}

impl ExprGraphFrameExt for Expr {
    fn gf(self) -> GraphFrameExprNameSpace {
        GraphFrameExprNameSpace::new(self)
    }
}
