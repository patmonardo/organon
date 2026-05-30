//! Shell helpers for entering Dataset 2x2 namespaces directly.
//!
//! These helpers keep the internal DSL terse when authors want explicit access
//! to frame/lazy/series/expr namespaces without starting from extension-method
//! chaining.

use crate::collections::dataframe::GDSExpr as Expr;

use crate::collections::dataframe::col;
use crate::collections::dataframe::{GDSDataFrame, GDSLazyFrame, GDSSeries};
use crate::collections::dataset::frame::expr::{DatasetExprNs, ExprDatasetExt};
use crate::collections::dataset::frame::lazy::DatasetLazyFrameNs;
use crate::collections::dataset::frame::series::DatasetSeriesNs;
use crate::collections::dataset::frame::DatasetDataFrameNs;

/// Enter the Dataset Expr namespace from a column name.
pub fn ds_col(name: impl AsRef<str>) -> DatasetExprNs {
    col(name.as_ref()).ds()
}

/// Enter the Dataset Expr namespace from an existing expression.
pub fn ds_expr(expr: Expr) -> DatasetExprNs {
    DatasetExprNs::new(expr)
}

/// Enter the Dataset DataFrame namespace.
pub fn ds_frame(df: GDSDataFrame) -> DatasetDataFrameNs {
    DatasetDataFrameNs::new(df)
}

/// Enter the Dataset LazyFrame namespace.
pub fn ds_lazy(lf: GDSLazyFrame) -> DatasetLazyFrameNs {
    DatasetLazyFrameNs::new(lf)
}

/// Enter the Dataset Series namespace.
pub fn ds_series(series: GDSSeries) -> DatasetSeriesNs {
    DatasetSeriesNs::new(series)
}
