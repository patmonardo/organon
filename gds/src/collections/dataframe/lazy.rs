//! LazyFrame facade (py-polars shaped).

use std::collections::HashMap;

use polars::prelude::{
    DataFrame, Expr, IdxSize, IntoLazy, LazyFrame, PlSmallStr, PolarsResult, SchemaRef,
    SortMultipleOptions,
};
use polars_plan::frame::OptFlags;

use crate::collections::dataframe::utils::parse::{
    parse_into_list_of_expressions_for_df, ExprInput, ParseExprOptions,
};

fn lazyframe_schema_df(lf: &LazyFrame) -> PolarsResult<DataFrame> {
    let mut cloned = lf.clone();
    let schema = cloned.collect_schema()?;
    Ok(DataFrame::empty_with_schema(schema.as_ref()))
}

/// Optimization flags wrapper mirroring py-polars QueryOptFlags.
#[derive(Debug, Clone)]
pub struct QueryOptFlags {
    opt_flags: OptFlags,
}

impl Default for QueryOptFlags {
    fn default() -> Self {
        Self {
            opt_flags: OptFlags::default(),
        }
    }
}

impl QueryOptFlags {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn none() -> Self {
        Self {
            opt_flags: OptFlags::empty(),
        }
    }

    pub fn from_opt_flags(opt_flags: OptFlags) -> Self {
        Self { opt_flags }
    }

    pub fn opt_flags(&self) -> OptFlags {
        self.opt_flags
    }

    pub fn into_opt_flags(self) -> OptFlags {
        self.opt_flags
    }

    pub fn predicate_pushdown(&self) -> bool {
        self.opt_flags.contains(OptFlags::PREDICATE_PUSHDOWN)
    }

    pub fn set_predicate_pushdown(&mut self, value: bool) -> &mut Self {
        self.opt_flags.set(OptFlags::PREDICATE_PUSHDOWN, value);
        self
    }

    pub fn projection_pushdown(&self) -> bool {
        self.opt_flags.contains(OptFlags::PROJECTION_PUSHDOWN)
    }

    pub fn set_projection_pushdown(&mut self, value: bool) -> &mut Self {
        self.opt_flags.set(OptFlags::PROJECTION_PUSHDOWN, value);
        self
    }

    pub fn simplify_expression(&self) -> bool {
        self.opt_flags.contains(OptFlags::SIMPLIFY_EXPR)
    }

    pub fn set_simplify_expression(&mut self, value: bool) -> &mut Self {
        self.opt_flags.set(OptFlags::SIMPLIFY_EXPR, value);
        self
    }

    pub fn slice_pushdown(&self) -> bool {
        self.opt_flags.contains(OptFlags::SLICE_PUSHDOWN)
    }

    pub fn set_slice_pushdown(&mut self, value: bool) -> &mut Self {
        self.opt_flags.set(OptFlags::SLICE_PUSHDOWN, value);
        self
    }

    pub fn comm_subplan_elim(&self) -> bool {
        self.opt_flags.contains(OptFlags::COMM_SUBPLAN_ELIM)
    }

    pub fn set_comm_subplan_elim(&mut self, value: bool) -> &mut Self {
        self.opt_flags.set(OptFlags::COMM_SUBPLAN_ELIM, value);
        self
    }

    pub fn comm_subexpr_elim(&self) -> bool {
        self.opt_flags.contains(OptFlags::COMM_SUBEXPR_ELIM)
    }

    pub fn set_comm_subexpr_elim(&mut self, value: bool) -> &mut Self {
        self.opt_flags.set(OptFlags::COMM_SUBEXPR_ELIM, value);
        self
    }

    pub fn cluster_with_columns(&self) -> bool {
        self.opt_flags.contains(OptFlags::CLUSTER_WITH_COLUMNS)
    }

    pub fn set_cluster_with_columns(&mut self, value: bool) -> &mut Self {
        self.opt_flags.set(OptFlags::CLUSTER_WITH_COLUMNS, value);
        self
    }

    pub fn check_order_observe(&self) -> bool {
        self.opt_flags.contains(OptFlags::CHECK_ORDER_OBSERVE)
    }

    pub fn set_check_order_observe(&mut self, value: bool) -> &mut Self {
        self.opt_flags.set(OptFlags::CHECK_ORDER_OBSERVE, value);
        self
    }

    pub fn fast_projection(&self) -> bool {
        self.opt_flags.contains(OptFlags::FAST_PROJECTION)
    }

    pub fn set_fast_projection(&mut self, value: bool) -> &mut Self {
        self.opt_flags.set(OptFlags::FAST_PROJECTION, value);
        self
    }

    pub fn type_coercion(&self) -> bool {
        self.opt_flags.contains(OptFlags::TYPE_COERCION)
    }

    pub fn set_type_coercion(&mut self, value: bool) -> &mut Self {
        self.opt_flags.set(OptFlags::TYPE_COERCION, value);
        self
    }

    pub fn type_check(&self) -> bool {
        self.opt_flags.contains(OptFlags::TYPE_CHECK)
    }

    pub fn set_type_check(&mut self, value: bool) -> &mut Self {
        self.opt_flags.set(OptFlags::TYPE_CHECK, value);
        self
    }

    pub fn eager(&self) -> bool {
        self.opt_flags.contains(OptFlags::EAGER)
    }

    pub fn set_eager(&mut self, value: bool) -> &mut Self {
        self.opt_flags.set(OptFlags::EAGER, value);
        self
    }

    pub fn update(
        &mut self,
        predicate_pushdown: Option<bool>,
        projection_pushdown: Option<bool>,
        simplify_expression: Option<bool>,
        slice_pushdown: Option<bool>,
        comm_subplan_elim: Option<bool>,
        comm_subexpr_elim: Option<bool>,
        cluster_with_columns: Option<bool>,
        check_order_observe: Option<bool>,
        fast_projection: Option<bool>,
        type_coercion: Option<bool>,
        type_check: Option<bool>,
    ) -> &mut Self {
        if let Some(value) = predicate_pushdown {
            self.set_predicate_pushdown(value);
        }
        if let Some(value) = projection_pushdown {
            self.set_projection_pushdown(value);
        }
        if let Some(value) = simplify_expression {
            self.set_simplify_expression(value);
        }
        if let Some(value) = slice_pushdown {
            self.set_slice_pushdown(value);
        }
        if let Some(value) = comm_subplan_elim {
            self.set_comm_subplan_elim(value);
        }
        if let Some(value) = comm_subexpr_elim {
            self.set_comm_subexpr_elim(value);
        }
        if let Some(value) = cluster_with_columns {
            self.set_cluster_with_columns(value);
        }
        if let Some(value) = check_order_observe {
            self.set_check_order_observe(value);
        }
        if let Some(value) = fast_projection {
            self.set_fast_projection(value);
        }
        if let Some(value) = type_coercion {
            self.set_type_coercion(value);
        }
        if let Some(value) = type_check {
            self.set_type_check(value);
        }
        self
    }
}

/// Polars LazyFrame wrapper for the Collections SDK.
#[derive(Clone)]
pub struct GDSLazyFrame {
    lf: LazyFrame,
}

impl GDSLazyFrame {
    pub fn new(lf: LazyFrame) -> Self {
        Self { lf }
    }

    pub fn from_dataframe(df: DataFrame) -> Self {
        Self { lf: df.lazy() }
    }

    pub fn lazyframe(&self) -> &LazyFrame {
        &self.lf
    }

    pub fn into_lazyframe(self) -> LazyFrame {
        self.lf
    }

    pub fn with_optimizations(self, optimizations: QueryOptFlags) -> Self {
        Self {
            lf: self.lf.with_optimizations(optimizations.into_opt_flags()),
        }
    }

    pub fn collect(self) -> PolarsResult<DataFrame> {
        self.lf.collect()
    }

    pub fn collect_with_optimizations(
        self,
        optimizations: QueryOptFlags,
    ) -> PolarsResult<DataFrame> {
        self.lf
            .with_optimizations(optimizations.into_opt_flags())
            .collect()
    }

    pub fn collect_schema(&mut self) -> PolarsResult<SchemaRef> {
        self.lf.collect_schema()
    }

    pub fn select_exprs(self, exprs: Vec<Expr>) -> Self {
        Self {
            lf: self.lf.select(exprs),
        }
    }

    pub fn select(
        self,
        inputs: Vec<ExprInput>,
        named_inputs: Option<HashMap<String, ExprInput>>,
    ) -> PolarsResult<Self> {
        let df = lazyframe_schema_df(&self.lf)?;
        let exprs = parse_into_list_of_expressions_for_df(
            &df,
            &inputs,
            named_inputs.as_ref(),
            ParseExprOptions::default(),
        )?;
        Ok(Self {
            lf: self.lf.select(exprs),
        })
    }

    pub fn with_columns_exprs(self, exprs: Vec<Expr>) -> Self {
        Self {
            lf: self.lf.with_columns(exprs),
        }
    }

    pub fn with_columns(
        self,
        inputs: Vec<ExprInput>,
        named_inputs: Option<HashMap<String, ExprInput>>,
    ) -> PolarsResult<Self> {
        let df = lazyframe_schema_df(&self.lf)?;
        let exprs = parse_into_list_of_expressions_for_df(
            &df,
            &inputs,
            named_inputs.as_ref(),
            ParseExprOptions::default(),
        )?;
        Ok(Self {
            lf: self.lf.with_columns(exprs),
        })
    }

    pub fn filter(self, expr: Expr) -> Self {
        Self {
            lf: self.lf.filter(expr),
        }
    }

    pub fn sort(self, columns: Vec<impl Into<PlSmallStr>>, options: SortMultipleOptions) -> Self {
        let columns: Vec<PlSmallStr> = columns.into_iter().map(Into::into).collect();
        Self {
            lf: self.lf.sort(columns, options),
        }
    }

    pub fn sort_by_exprs(self, exprs: Vec<Expr>, options: SortMultipleOptions) -> Self {
        Self {
            lf: self.lf.sort_by_exprs(exprs, options),
        }
    }

    pub fn limit(self, n: IdxSize) -> Self {
        Self {
            lf: self.lf.limit(n),
        }
    }

    pub fn group_by(self, by: Vec<Expr>, maintain_order: bool) -> GDSLazyGroupBy {
        let group_by = if maintain_order {
            self.lf.group_by_stable(by)
        } else {
            self.lf.group_by(by)
        };
        GDSLazyGroupBy::new(group_by)
    }
}

impl From<LazyFrame> for GDSLazyFrame {
    fn from(lf: LazyFrame) -> Self {
        Self::new(lf)
    }
}

impl From<DataFrame> for GDSLazyFrame {
    fn from(df: DataFrame) -> Self {
        Self::from_dataframe(df)
    }
}

impl From<GDSLazyFrame> for LazyFrame {
    fn from(wrapper: GDSLazyFrame) -> Self {
        wrapper.lf
    }
}

/// Polars LazyGroupBy wrapper for the Collections SDK.
pub struct GDSLazyGroupBy {
    group_by: polars::prelude::LazyGroupBy,
}

impl GDSLazyGroupBy {
    pub fn new(group_by: polars::prelude::LazyGroupBy) -> Self {
        Self { group_by }
    }

    pub fn agg(self, aggs: Vec<Expr>) -> GDSLazyFrame {
        GDSLazyFrame::new(self.group_by.agg(aggs))
    }

    pub fn head(self, n: Option<usize>) -> GDSLazyFrame {
        GDSLazyFrame::new(self.group_by.head(n))
    }

    pub fn tail(self, n: Option<usize>) -> GDSLazyFrame {
        GDSLazyFrame::new(self.group_by.tail(n))
    }
}

#[cfg(test)]
mod tests {
    use polars::prelude::{col, Column, DataFrame, IntoLazy, NamedFrom, Series};

    use super::GDSLazyFrame;
    use crate::collections::dataframe::utils::parse::ExprInput;

    #[test]
    fn lazyframe_select_parity() {
        let df = DataFrame::new(vec![
            Column::from(Series::new("a".into(), &[1i64, 2, 3])),
            Column::from(Series::new("b".into(), &[10i64, 20, 30])),
        ])
        .expect("dataframe");

        let out = GDSLazyFrame::from_dataframe(df.clone())
            .select(vec![ExprInput::column("a")], None)
            .expect("select")
            .collect()
            .expect("collect");

        let expected = df
            .lazy()
            .select([col("a")])
            .collect()
            .expect("collect expected");

        assert_eq!(out.shape(), expected.shape());
        assert_eq!(out.column("a").unwrap(), expected.column("a").unwrap());
    }
}
