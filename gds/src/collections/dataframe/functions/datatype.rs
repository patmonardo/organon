//! Datatype helpers.

use polars::polars_utils::pl_str::PlSmallStr;
use polars::prelude::{DataTypeExpr, Expr};

pub fn dtype_of(expr: Expr) -> DataTypeExpr {
    DataTypeExpr::OfExpr(Box::new(expr))
}

pub fn self_dtype() -> DataTypeExpr {
    DataTypeExpr::SelfDtype
}

pub fn struct_with_fields(fields: &[(impl AsRef<str>, DataTypeExpr)]) -> DataTypeExpr {
    DataTypeExpr::StructWithFields(
        fields
            .iter()
            .map(|(name, dtype)| (PlSmallStr::from(name.as_ref()), dtype.clone()))
            .collect(),
    )
}
