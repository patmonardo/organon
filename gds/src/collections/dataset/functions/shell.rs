//! Shell helpers for entering Dataset 2x2 namespaces directly.
//!
//! These helpers keep the internal DSL terse when authors want explicit access
//! to frame/lazy/series/expr namespaces without starting from extension-method
//! chaining.

use polars::prelude::Expr;

use crate::collections::dataframe::{GDSDataFrame, GDSLazyFrame, GDSSeries};
use crate::collections::dataset::expr::DatasetExprNameSpace;
use crate::collections::dataset::frame::DatasetDataFrameNameSpace;
use crate::collections::dataset::lazy::DatasetLazyFrameNameSpace;
use crate::collections::dataset::series::DatasetSeriesNameSpace;

/// Enter the Dataset Expr namespace from a column name.
pub fn ds_col(name: impl AsRef<str>) -> DatasetExprNameSpace {
    DatasetExprNameSpace::col(name.as_ref())
}

/// Enter the Dataset Expr namespace from an existing expression.
pub fn ds_expr(expr: Expr) -> DatasetExprNameSpace {
    DatasetExprNameSpace::new(expr)
}

/// Enter the Dataset DataFrame namespace.
pub fn ds_frame(df: GDSDataFrame) -> DatasetDataFrameNameSpace {
    DatasetDataFrameNameSpace::new(df)
}

/// Enter the Dataset LazyFrame namespace.
pub fn ds_lazy(lf: GDSLazyFrame) -> DatasetLazyFrameNameSpace {
    DatasetLazyFrameNameSpace::new(lf)
}

/// Enter the Dataset Series namespace.
pub fn ds_series(series: GDSSeries) -> DatasetSeriesNameSpace {
    DatasetSeriesNameSpace::new(series)
}
