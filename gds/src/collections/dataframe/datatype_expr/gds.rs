//! DataTypeExpr model wrapper.

use polars::prelude::{DataType, DataTypeExpr, DataTypeSelector, Expr, PolarsResult, Schema};

use crate::collections::dataframe::datatypes::GDSDataType;

use super::array::DataTypeExprArrNameSpace;
use super::list::DataTypeExprListNameSpace;
use super::structure::DataTypeExprStructNameSpace;

#[derive(Debug, Clone)]
pub struct GDSDataTypeExpr {
    expr: DataTypeExpr,
}

#[derive(Debug, Clone)]
pub enum GDSDataTypeExprInput {
    GDS(GDSDataTypeExpr),
    DataType(GDSDataType),
    Polars(DataTypeExpr),
}

impl GDSDataTypeExprInput {
    pub fn into_polars_expr(self) -> DataTypeExpr {
        match self {
            Self::GDS(expr) => expr.into_inner(),
            Self::DataType(dtype) => DataTypeExpr::Literal(dtype),
            Self::Polars(expr) => expr,
        }
    }
}

impl From<GDSDataTypeExpr> for GDSDataTypeExprInput {
    fn from(value: GDSDataTypeExpr) -> Self {
        Self::GDS(value)
    }
}

impl From<GDSDataType> for GDSDataTypeExprInput {
    fn from(value: GDSDataType) -> Self {
        Self::DataType(value)
    }
}

impl From<DataTypeExpr> for GDSDataTypeExprInput {
    fn from(value: DataTypeExpr) -> Self {
        Self::Polars(value)
    }
}

impl GDSDataTypeExpr {
    pub fn new(expr: DataTypeExpr) -> Self {
        Self { expr }
    }

    pub fn literal(dtype: GDSDataType) -> Self {
        Self::new(DataTypeExpr::Literal(dtype))
    }

    pub fn of_expr(expr: Expr) -> Self {
        Self::new(DataTypeExpr::OfExpr(Box::new(expr)))
    }

    pub fn self_dtype() -> Self {
        Self::new(DataTypeExpr::SelfDtype)
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

    pub fn structure(&self) -> DataTypeExprStructNameSpace {
        DataTypeExprStructNameSpace::new(self.expr.clone())
    }

    pub fn collect_dtype(&self, schema: &Schema) -> PolarsResult<DataType> {
        self.expr.clone().into_datatype(schema)
    }
}
