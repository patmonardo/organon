//! List datatype expression namespace.

use polars::prelude::DataTypeExpr;

use super::DataTypeExprModel;

#[derive(Debug, Clone)]
pub struct DataTypeExprListNameSpace {
    expr: DataTypeExpr,
}

impl DataTypeExprListNameSpace {
    pub fn new(expr: DataTypeExpr) -> Self {
        Self { expr }
    }

    pub fn inner_dtype(&self) -> DataTypeExprModel {
        DataTypeExprModel::new(self.expr.clone().list().inner_dtype())
    }
}
