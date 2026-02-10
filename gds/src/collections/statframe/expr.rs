//! StatFrame expression facade.
//!
//! StatFrame is a statistical modeling DSL. Its core expression type is `StatExpr`
//! (container-agnostic). This module provides a small namespace wrapper and an
//! extension trait to make the surface consistent with the RustScript 2×2 pattern.

use crate::collections::statframe::expressions::stat_expr::StatExpr;

#[derive(Debug, Clone)]
pub struct StatFrameExprNameSpace {
    expr: StatExpr,
}

impl StatFrameExprNameSpace {
    pub fn new(expr: StatExpr) -> Self {
        Self { expr }
    }

    pub fn expr(&self) -> &StatExpr {
        &self.expr
    }

    pub fn into_expr(self) -> StatExpr {
        self.expr
    }
}

pub trait ExprStatFrameExt {
    fn sf(self) -> StatFrameExprNameSpace;
}

impl ExprStatFrameExt for StatExpr {
    fn sf(self) -> StatFrameExprNameSpace {
        StatFrameExprNameSpace::new(self)
    }
}
