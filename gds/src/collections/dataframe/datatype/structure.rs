//! Struct datatype expression namespace.

use polars::prelude::{DataTypeExpr, Expr};

use super::GDSDataTypeExpr;

#[derive(Debug, Clone)]
pub struct DataTypeExprStructNameSpace {
    expr: DataTypeExpr,
}

impl DataTypeExprStructNameSpace {
    pub fn new(expr: DataTypeExpr) -> Self {
        Self { expr }
    }

    pub fn field_dtype(&self, field_name: &str) -> GDSDataTypeExpr {
        GDSDataTypeExpr::new(self.expr.clone().struct_().field_dtype_by_name(field_name))
    }

    pub fn field_dtype_by_index(&self, index: i64) -> GDSDataTypeExpr {
        GDSDataTypeExpr::new(self.expr.clone().struct_().field_dtype_by_index(index))
    }

    pub fn field_names(&self) -> Expr {
        self.expr.clone().struct_().field_names()
    }
}
