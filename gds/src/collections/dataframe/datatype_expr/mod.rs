//! DataTypeExpr helpers and namespaces (seed pass), inspired by py-polars.

pub mod array;
pub mod list;
pub mod struct_;

use polars::prelude::{DataType, DataTypeExpr, DataTypeSelector, Expr, PolarsResult, Schema};

pub use array::DataTypeExprArrNameSpace;
pub use list::DataTypeExprListNameSpace;
pub use struct_::DataTypeExprStructNameSpace;

#[derive(Debug, Clone)]
pub struct DataTypeExprModel {
    expr: DataTypeExpr,
}

impl DataTypeExprModel {
    pub fn new(expr: DataTypeExpr) -> Self {
        Self { expr }
    }

    pub fn expr(&self) -> &DataTypeExpr {
        &self.expr
    }

    pub fn into_inner(self) -> DataTypeExpr {
        self.expr
    }

    pub fn inner_dtype(&self) -> Self {
        Self::new(self.expr.clone().inner_dtype())
    }

    pub fn display(&self) -> Expr {
        self.expr.clone().display()
    }

    pub fn matches(&self, selector: DataTypeSelector) -> Expr {
        self.expr.clone().matches(selector)
    }

    pub fn wrap_in_list(&self) -> Self {
        Self::new(self.expr.clone().wrap_in_list())
    }

    pub fn wrap_in_array(&self, width: usize) -> Self {
        Self::new(self.expr.clone().wrap_in_array(width))
    }

    pub fn to_unsigned_integer(&self) -> Self {
        Self::new(self.expr.clone().int().to_unsigned())
    }

    pub fn to_signed_integer(&self) -> Self {
        Self::new(self.expr.clone().int().to_signed())
    }

    pub fn default_value(&self, n: usize, numeric_to_one: bool, num_list_values: usize) -> Expr {
        self.expr
            .clone()
            .default_value(n, numeric_to_one, num_list_values)
    }

    pub fn list(&self) -> DataTypeExprListNameSpace {
        DataTypeExprListNameSpace::new(self.expr.clone())
    }

    pub fn arr(&self) -> DataTypeExprArrNameSpace {
        DataTypeExprArrNameSpace::new(self.expr.clone())
    }

    pub fn struct_(&self) -> DataTypeExprStructNameSpace {
        DataTypeExprStructNameSpace::new(self.expr.clone())
    }

    pub fn collect_dtype(&self, schema: &Schema) -> PolarsResult<DataType> {
        self.expr.clone().into_datatype(schema)
    }
}
