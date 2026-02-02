//! Meta namespace for expressions (py-polars inspired).

use polars::prelude::{Expr, PlSmallStr, PolarsResult, Schema};

#[derive(Debug, Clone)]
pub struct ExprMeta {
    expr: Expr,
}

impl ExprMeta {
    pub fn new(expr: Expr) -> Self {
        Self { expr }
    }

    pub fn expr(&self) -> &Expr {
        &self.expr
    }

    pub fn into_expr(self) -> Expr {
        self.expr
    }

    pub fn pop(self, schema: Option<&Schema>) -> PolarsResult<Vec<Expr>> {
        self.expr.meta().pop(schema)
    }

    pub fn root_names(&self) -> Vec<PlSmallStr> {
        self.expr.clone().meta().root_names()
    }

    pub fn is_simple_projection(&self, schema: Option<&Schema>) -> bool {
        self.expr.clone().meta().is_simple_projection(schema)
    }

    pub fn output_name(&self) -> PolarsResult<PlSmallStr> {
        self.expr.clone().meta().output_name()
    }

    pub fn undo_aliases(self) -> Expr {
        self.expr.meta().undo_aliases()
    }

    pub fn has_multiple_outputs(&self) -> bool {
        self.expr.clone().meta().has_multiple_outputs()
    }

    pub fn is_column(&self) -> bool {
        self.expr.clone().meta().is_column()
    }

    pub fn is_column_selection(&self, allow_aliasing: bool) -> bool {
        self.expr.clone().meta().is_column_selection(allow_aliasing)
    }

    pub fn is_literal(&self, allow_aliasing: bool) -> bool {
        self.expr.clone().meta().is_literal(allow_aliasing)
    }

    pub fn is_regex_projection(&self) -> bool {
        self.expr.clone().meta().is_regex_projection()
    }

    pub fn tree_format(&self, schema: Option<&Schema>) -> PolarsResult<String> {
        let formatter = self
            .expr
            .clone()
            .meta()
            .into_tree_formatter(false, schema)?;
        Ok(formatter.to_string())
    }

    pub fn tree_format_dot(&self, schema: Option<&Schema>) -> PolarsResult<String> {
        let formatter = self.expr.clone().meta().into_tree_formatter(true, schema)?;
        Ok(formatter.to_string())
    }
}
