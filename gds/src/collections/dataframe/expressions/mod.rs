//! Expression namespace facades (py-polars inspired).

pub mod array;
pub mod binary;
pub mod categorical;
pub mod datetime;
pub mod expr;
pub mod ext;
pub mod list;
pub mod meta;
pub mod name;
pub mod string;
pub mod struct_;
pub mod whenthen;

pub use array::ExprArray;
pub use binary::ExprBinary;
pub use categorical::ExprCategorical;
pub use datetime::ExprDateTime;
pub use expr::ExprNamespace;
pub use ext::ExprExt;
pub use list::ExprList;
pub use meta::ExprMeta;
pub use name::ExprName;
pub use string::ExprString;
pub use struct_::ExprStruct;

use polars::prelude::Expr;

pub fn expr_ns(expr: Expr) -> ExprNamespace {
    ExprNamespace::new(expr)
}

pub fn arr_ns(expr: Expr) -> ExprArray {
    ExprArray::new(expr)
}

pub fn binary_ns(expr: Expr) -> ExprBinary {
    ExprBinary::new(expr)
}

pub fn cat_ns(expr: Expr) -> ExprCategorical {
    ExprCategorical::new(expr)
}

pub fn dt_ns(expr: Expr) -> ExprDateTime {
    ExprDateTime::new(expr)
}

pub fn ext_ns(expr: Expr) -> ExprExt {
    ExprExt::new(expr)
}

pub fn list_ns(expr: Expr) -> ExprList {
    ExprList::new(expr)
}

pub fn meta_ns(expr: Expr) -> ExprMeta {
    ExprMeta::new(expr)
}

pub fn name_ns(expr: Expr) -> ExprName {
    ExprName::new(expr)
}

pub fn str_ns(expr: Expr) -> ExprString {
    ExprString::new(expr)
}

pub fn struct_ns(expr: Expr) -> ExprStruct {
    ExprStruct::new(expr)
}
