//! Struct namespace for expressions (py-polars inspired).

use polars::prelude::{Expr, PlSmallStr};

#[derive(Debug, Clone)]
pub struct ExprStruct {
    expr: Expr,
}

impl ExprStruct {
    pub fn new(expr: Expr) -> Self {
        Self { expr }
    }

    pub fn expr(&self) -> &Expr {
        &self.expr
    }

    pub fn into_expr(self) -> Expr {
        self.expr
    }

    pub fn field(self, name: &str) -> Expr {
        self.expr.struct_().field_by_name(name)
    }

    pub fn field_many(self, names: &[&str]) -> Expr {
        self.expr.struct_().field_by_names(names.iter().copied())
    }

    pub fn field_by_index(self, index: i64) -> Expr {
        self.expr.struct_().field_by_index(index)
    }

    pub fn unnest(self) -> Expr {
        self.field("*")
    }

    pub fn rename_fields(self, names: &[&str]) -> Expr {
        self.expr.struct_().rename_fields(names.iter().copied())
    }

    pub fn rename_fields_pl(self, names: &[PlSmallStr]) -> Expr {
        self.expr.struct_().rename_fields(names.iter().cloned())
    }

    pub fn json_encode(self) -> Expr {
        self.expr.struct_().json_encode()
    }

    pub fn with_fields(self, fields: Vec<Expr>) -> Expr {
        self.expr.struct_().with_fields(fields)
    }
}
