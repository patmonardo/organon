//! Expr namespace facade (py-polars inspired).

use polars::prelude::Expr;

use crate::collections::dataframe::expressions::array::ExprArray;
use crate::collections::dataframe::expressions::binary::ExprBinary;
use crate::collections::dataframe::expressions::categorical::ExprCategorical;
use crate::collections::dataframe::expressions::datetime::ExprDateTime;
use crate::collections::dataframe::expressions::ext::ExprExt;
use crate::collections::dataframe::expressions::list::ExprList;
use crate::collections::dataframe::expressions::meta::ExprMeta;
use crate::collections::dataframe::expressions::name::ExprName;
use crate::collections::dataframe::expressions::string::ExprString;
use crate::collections::dataframe::expressions::structure::ExprStruct;

#[derive(Debug, Clone)]
pub struct ExprNamespace {
    expr: Expr,
}

impl ExprNamespace {
    pub fn new(expr: Expr) -> Self {
        Self { expr }
    }

    pub fn expr(&self) -> &Expr {
        &self.expr
    }

    pub fn into_expr(self) -> Expr {
        self.expr
    }

    pub fn arr(&self) -> ExprArray {
        ExprArray::new(self.expr.clone())
    }

    pub fn bin(&self) -> ExprBinary {
        ExprBinary::new(self.expr.clone())
    }

    pub fn cat(&self) -> ExprCategorical {
        ExprCategorical::new(self.expr.clone())
    }

    pub fn dt(&self) -> ExprDateTime {
        ExprDateTime::new(self.expr.clone())
    }

    pub fn ext(&self) -> ExprExt {
        ExprExt::new(self.expr.clone())
    }

    pub fn list(&self) -> ExprList {
        ExprList::new(self.expr.clone())
    }

    pub fn meta(&self) -> ExprMeta {
        ExprMeta::new(self.expr.clone())
    }

    pub fn name(&self) -> ExprName {
        ExprName::new(self.expr.clone())
    }

    pub fn str(&self) -> ExprString {
        ExprString::new(self.expr.clone())
    }

    pub fn struct_(&self) -> ExprStruct {
        ExprStruct::new(self.expr.clone())
    }

    pub fn record(&self) -> ExprStruct {
        self.struct_()
    }
}
