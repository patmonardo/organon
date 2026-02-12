//! Series/Expr pipeline helpers.

use polars::error::PolarsError;
use polars::prelude::{col as pl_col, Column, DataFrame, Expr, IntoLazy, PolarsResult, Series};

use crate::collections::dataframe::expressions::array::ExprArray;
use crate::collections::dataframe::expressions::binary::ExprBinary;
use crate::collections::dataframe::expressions::categorical::ExprCategorical;
use crate::collections::dataframe::expressions::datetime::ExprDateTime;
use crate::collections::dataframe::expressions::expr::ExprNamespace;
use crate::collections::dataframe::expressions::ext::ExprExt;
use crate::collections::dataframe::expressions::list::ExprList;
use crate::collections::dataframe::expressions::meta::ExprMeta;
use crate::collections::dataframe::expressions::name::ExprName;
use crate::collections::dataframe::expressions::string::ExprString;
use crate::collections::dataframe::expressions::structure::ExprStruct;
use crate::collections::dataframe::utils::udfs::{
    suggest_from_function_name, MapTarget, UdfError, UdfRewriteSuggestion,
};

/// Wrapper that enables an Expr namespace pipeline from a Series.
#[derive(Debug, Clone)]
pub struct SeriesExpr {
    series: Series,
}

#[derive(Debug, Clone)]
pub struct SeriesExprString {
    series: Series,
}

#[derive(Debug, Clone)]
pub struct SeriesExprList {
    series: Series,
}

#[derive(Debug, Clone)]
pub struct SeriesExprDateTime {
    series: Series,
}

#[derive(Debug, Clone)]
pub struct SeriesExprBinary {
    series: Series,
}

#[derive(Debug, Clone)]
pub struct SeriesExprStruct {
    series: Series,
}

#[derive(Debug, Clone)]
pub struct SeriesExprCategorical {
    series: Series,
}

#[derive(Debug, Clone)]
pub struct SeriesExprArray {
    series: Series,
}

#[derive(Debug, Clone)]
pub struct SeriesExprName {
    series: Series,
}

#[derive(Debug, Clone)]
pub struct SeriesExprMeta {
    series: Series,
}

#[derive(Debug, Clone)]
pub struct SeriesExprExt {
    series: Series,
}

macro_rules! impl_series_expr_ns {
    ($wrapper:ident, $ns:ident) => {
        impl $wrapper {
            pub fn new(series: Series) -> Self {
                Self { series }
            }

            pub fn apply<F>(&self, f: F) -> PolarsResult<Series>
            where
                F: FnOnce($ns) -> Expr,
            {
                let expr = f($ns::new(series_col_expr(&self.series)));
                eval_series_expr(&self.series, expr)
            }
        }
    };
    ($wrapper:ident, $ns:ident, result) => {
        impl_series_expr_ns!($wrapper, $ns);

        impl $wrapper {
            pub fn apply_result<F>(&self, f: F) -> PolarsResult<Series>
            where
                F: FnOnce($ns) -> PolarsResult<Expr>,
            {
                let expr = f($ns::new(series_col_expr(&self.series)))?;
                eval_series_expr(&self.series, expr)
            }
        }
    };
}

/// Create a Series/Expr pipeline wrapper.
pub fn series_expr(series: Series) -> SeriesExpr {
    SeriesExpr::new(series)
}

/// Expr namespace helper for list operations.
pub fn list_ns(expr: Expr) -> ExprList {
    ExprList::new(expr)
}

/// Expr namespace helper for array operations.
pub fn arr_ns(expr: Expr) -> ExprArray {
    ExprArray::new(expr)
}

/// Expr namespace helper for string operations.
pub fn str_ns(expr: Expr) -> ExprString {
    ExprString::new(expr)
}

fn series_col_expr(series: &Series) -> Expr {
    pl_col(series.name().as_str())
}

impl SeriesExpr {
    pub fn new(series: Series) -> Self {
        Self { series }
    }

    pub fn series(&self) -> &Series {
        &self.series
    }

    /// Build an Expr namespace rooted at this Series.
    pub fn expr(&self) -> ExprNamespace {
        ExprNamespace::new(series_col_expr(&self.series))
    }

    pub fn str(&self) -> SeriesExprString {
        SeriesExprString::new(self.series.clone())
    }

    pub fn list(&self) -> SeriesExprList {
        SeriesExprList::new(self.series.clone())
    }

    pub fn dt(&self) -> SeriesExprDateTime {
        SeriesExprDateTime::new(self.series.clone())
    }

    pub fn binary(&self) -> SeriesExprBinary {
        SeriesExprBinary::new(self.series.clone())
    }

    pub fn structure(&self) -> SeriesExprStruct {
        SeriesExprStruct::new(self.series.clone())
    }

    pub fn record(&self) -> SeriesExprStruct {
        self.structure()
    }

    pub fn cat(&self) -> SeriesExprCategorical {
        SeriesExprCategorical::new(self.series.clone())
    }

    pub fn arr(&self) -> SeriesExprArray {
        SeriesExprArray::new(self.series.clone())
    }

    pub fn name(&self) -> SeriesExprName {
        SeriesExprName::new(self.series.clone())
    }

    pub fn meta(&self) -> SeriesExprMeta {
        SeriesExprMeta::new(self.series.clone())
    }

    pub fn ext(&self) -> SeriesExprExt {
        SeriesExprExt::new(self.series.clone())
    }

    /// Evaluate an expression against this Series.
    pub fn eval_expr(&self, expr: Expr) -> PolarsResult<Series> {
        eval_series_expr(&self.series, expr)
    }

    /// Apply a transformation using the Expr namespace and evaluate it.
    pub fn apply<F>(&self, f: F) -> PolarsResult<Series>
    where
        F: FnOnce(ExprNamespace) -> Expr,
    {
        let expr = f(self.expr());
        eval_series_expr(&self.series, expr)
    }

    /// Suggest a native expression rewrite for an Expr-level UDF name.
    pub fn suggest_udf_rewrite(
        &self,
        function_name: &str,
    ) -> Result<Option<UdfRewriteSuggestion>, UdfError> {
        suggest_from_function_name(function_name, self.series.name().as_str(), MapTarget::Expr)
    }
}

impl_series_expr_ns!(SeriesExprString, ExprString);
impl_series_expr_ns!(SeriesExprList, ExprList);
impl_series_expr_ns!(SeriesExprDateTime, ExprDateTime);
impl_series_expr_ns!(SeriesExprBinary, ExprBinary, result);
impl_series_expr_ns!(SeriesExprStruct, ExprStruct);
impl_series_expr_ns!(SeriesExprCategorical, ExprCategorical);
impl_series_expr_ns!(SeriesExprArray, ExprArray, result);
impl_series_expr_ns!(SeriesExprName, ExprName);
impl_series_expr_ns!(SeriesExprMeta, ExprMeta);
impl_series_expr_ns!(SeriesExprExt, ExprExt);

fn eval_series_expr(series: &Series, expr: Expr) -> PolarsResult<Series> {
    let df = DataFrame::new(vec![Column::from(series.clone())])?;
    let out = df.lazy().select([expr]).collect()?;
    out.select_at_idx(0)
        .map(|column| column.as_materialized_series().clone())
        .ok_or_else(|| PolarsError::ComputeError("expression produced no output".into()))
}
