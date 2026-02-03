//! Array datatype expression namespace.

use polars::prelude::{DataTypeExpr, Expr};

use super::GDSDataTypeExpr;

#[derive(Debug, Clone)]
pub struct DataTypeExprArrNameSpace {
    expr: DataTypeExpr,
}

impl DataTypeExprArrNameSpace {
    pub fn new(expr: DataTypeExpr) -> Self {
        Self { expr }
    }

    pub fn inner_dtype(&self) -> GDSDataTypeExpr {
        GDSDataTypeExpr::new(self.expr.clone().arr().inner_dtype())
    }

    pub fn width(&self) -> Expr {
        self.expr.clone().arr().width()
    }

    pub fn shape(&self) -> Expr {
        self.expr.clone().arr().shape()
    }
}
