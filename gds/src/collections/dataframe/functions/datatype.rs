//! Datatype helpers.

use polars::polars_utils::pl_str::PlSmallStr;
use polars::prelude::{DataTypeExpr, Expr};

use crate::collections::dataframe::{GDSDataTypeExpr, GDSDataTypeExprInput};

pub fn dtype_of(expr: Expr) -> GDSDataTypeExpr {
    GDSDataTypeExpr::of_expr(expr)
}

pub fn self_dtype() -> GDSDataTypeExpr {
    GDSDataTypeExpr::self_dtype()
}

pub fn struct_with_fields(fields: &[(impl AsRef<str>, GDSDataTypeExprInput)]) -> GDSDataTypeExpr {
    GDSDataTypeExpr::new(DataTypeExpr::StructWithFields(
        fields
            .iter()
            .map(|(name, dtype)| {
                (
                    PlSmallStr::from(name.as_ref()),
                    dtype.clone().into_polars_expr(),
                )
            })
            .collect(),
    ))
}
