//! Polars DataFrame integration for Collections.

use polars::error::PolarsError;
use polars::frame::row::Row;
use polars::prelude::{
    col, Column, DataFrame, DataType, Expr, IntoLazy, PlSmallStr, Schema, Series,
    SortMultipleOptions,
};

use crate::collections::dataframe::selectors::{expand_selector, Selector};

pub use polars::prelude::Column as PolarsColumn;
pub use polars::prelude::DataType as PolarsDataType;
pub use polars::prelude::Series as PolarsSeries;

/// Trait for DataFrame-backed Collections with full Polars access.
pub trait DataFrameCollection: Send + Sync {
    /// Immutable access to the underlying DataFrame.
    fn dataframe(&self) -> &DataFrame;

    /// Mutable access to the underlying DataFrame.
    fn dataframe_mut(&mut self) -> &mut DataFrame;

    /// Number of rows in the DataFrame.
    fn row_count(&self) -> usize {
        self.dataframe().height()
    }

    /// Number of columns in the DataFrame.
    fn column_count(&self) -> usize {
        self.dataframe().width()
    }

    /// Column names.
    fn column_names(&self) -> Vec<String> {
        self.dataframe()
            .get_column_names()
            .iter()
            .map(|name| name.to_string())
            .collect()
    }

    /// Column data types.
    fn dtypes(&self) -> Vec<DataType> {
        self.dataframe()
            .get_columns()
            .iter()
            .map(|series| series.dtype().clone())
            .collect()
    }

    /// Whether the DataFrame has zero rows.
    fn is_empty(&self) -> bool {
        self.dataframe().height() == 0
    }

    /// Access a column by name.
    fn column(&self, name: &str) -> Result<&Column, PolarsError> {
        self.dataframe().column(name)
    }

    /// Select a subset of columns by name.
    fn select_columns(&self, columns: &[&str]) -> Result<DataFrame, PolarsError> {
        let selection: Vec<&str> = columns.iter().copied().collect();
        self.dataframe().select(selection)
    }

    /// Select columns using Polars expressions (py-polars style).
    fn select(&self, exprs: &[Expr]) -> Result<DataFrame, PolarsError> {
        self.dataframe()
            .clone()
            .lazy()
            .select(exprs.to_vec())
            .collect()
    }

    /// Return the first N rows.
    fn head(&self, n: usize) -> DataFrame {
        self.dataframe().head(Some(n))
    }

    /// Return the last N rows.
    fn tail(&self, n: usize) -> DataFrame {
        self.dataframe().tail(Some(n))
    }

    /// Slice rows by offset and length.
    fn slice(&self, offset: i64, length: usize) -> DataFrame {
        self.dataframe().slice(offset, length)
    }
}

/// Polars-backed DataFrame collection wrapper.
#[derive(Debug, Clone)]
pub struct PolarsDataFrameCollection {
    df: DataFrame,
}

impl PolarsDataFrameCollection {
    pub fn new(df: DataFrame) -> Self {
        Self { df }
    }

    /// Create an empty DataFrame with a given schema.
    pub fn empty_with_schema(schema: &Schema) -> Self {
        let df = DataFrame::empty_with_schema(schema);
        Self { df }
    }

    pub fn dataframe(&self) -> &DataFrame {
        &self.df
    }

    pub fn dataframe_mut(&mut self) -> &mut DataFrame {
        &mut self.df
    }

    pub fn from_series(columns: Vec<Series>) -> Result<Self, PolarsError> {
        let cols: Vec<Column> = columns.into_iter().map(Column::from).collect();
        let df = DataFrame::new(cols)?;
        Ok(Self { df })
    }

    /// Construct from Polars columns.
    pub fn from_columns(columns: Vec<Column>) -> Result<Self, PolarsError> {
        let df = DataFrame::new(columns)?;
        Ok(Self { df })
    }

    /// Construct from rows (slower; row-wise input).
    pub fn from_rows(rows: &[Row]) -> Result<Self, PolarsError> {
        let df = DataFrame::from_rows(rows)?;
        Ok(Self { df })
    }

    /// Construct from rows with an explicit schema.
    pub fn from_rows_and_schema(rows: &[Row], schema: &Schema) -> Result<Self, PolarsError> {
        let df = DataFrame::from_rows_and_schema(rows, schema)?;
        Ok(Self { df })
    }

    /// Select a subset of columns (eager).
    pub fn select_columns(&self, columns: &[&str]) -> Result<Self, PolarsError> {
        let selection: Vec<&str> = columns.iter().copied().collect();
        let df = self.df.select(selection)?;
        Ok(Self { df })
    }

    /// Select columns using Polars expressions (py-polars style).
    pub fn select(&self, exprs: &[Expr]) -> Result<Self, PolarsError> {
        let df = self.df.clone().lazy().select(exprs.to_vec()).collect()?;
        Ok(Self { df })
    }

    /// Select columns using a selector (py-polars style).
    pub fn select_selector(&self, selector: &Selector) -> Result<Self, PolarsError> {
        let names = expand_selector(&self.df, selector);
        let columns: Vec<&str> = names.iter().map(|name| name.as_str()).collect();
        self.select_columns(&columns)
    }

    /// Filter rows using a Polars predicate expression.
    pub fn filter_expr(&self, predicate: Expr) -> Result<Self, PolarsError> {
        let df = self.df.clone().lazy().filter(predicate).collect()?;
        Ok(Self { df })
    }

    /// Python-Polars alias for filter expression.
    pub fn filter(&self, predicate: Expr) -> Result<Self, PolarsError> {
        self.filter_expr(predicate)
    }

    /// Add or replace columns using Polars expressions.
    pub fn with_columns_exprs(&self, exprs: &[Expr]) -> Result<Self, PolarsError> {
        let df = self
            .df
            .clone()
            .lazy()
            .with_columns(exprs.to_vec())
            .collect()?;
        Ok(Self { df })
    }

    /// Python-Polars alias for with_columns.
    pub fn with_columns(&self, exprs: &[Expr]) -> Result<Self, PolarsError> {
        self.with_columns_exprs(exprs)
    }

    /// Order rows by named columns (eager), using Polars sort options.
    pub fn order_by_columns(
        &self,
        columns: &[&str],
        options: SortMultipleOptions,
    ) -> Result<Self, PolarsError> {
        let by: Vec<PlSmallStr> = columns.iter().map(|name| (*name).into()).collect();
        let df = self.df.sort(by, options)?;
        Ok(Self { df })
    }

    /// Python-Polars alias for sort by columns (eager).
    pub fn sort(
        &self,
        columns: &[&str],
        options: SortMultipleOptions,
    ) -> Result<Self, PolarsError> {
        self.order_by_columns(columns, options)
    }

    /// Order rows by expressions (lazy), using Polars sort options.
    pub fn order_by_exprs(
        &self,
        exprs: &[Expr],
        options: SortMultipleOptions,
    ) -> Result<Self, PolarsError> {
        let df = self
            .df
            .clone()
            .lazy()
            .sort_by_exprs(exprs.to_vec(), options)
            .collect()?;
        Ok(Self { df })
    }

    /// Python-Polars alias for sort by expressions (lazy).
    pub fn sort_by_exprs(
        &self,
        exprs: &[Expr],
        options: SortMultipleOptions,
    ) -> Result<Self, PolarsError> {
        self.order_by_exprs(exprs, options)
    }

    /// Group by columns and aggregate using Polars expressions.
    pub fn group_by_exprs(&self, keys: &[Expr], aggs: &[Expr]) -> Result<Self, PolarsError> {
        let df = self
            .df
            .clone()
            .lazy()
            .group_by(keys.to_vec())
            .agg(aggs.to_vec())
            .collect()?;
        Ok(Self { df })
    }

    /// Group by named columns and aggregate using Polars expressions.
    pub fn group_by_columns(&self, keys: &[&str], aggs: &[Expr]) -> Result<Self, PolarsError> {
        let key_exprs: Vec<Expr> = keys.iter().map(|name| col(*name)).collect();
        self.group_by_exprs(&key_exprs, aggs)
    }

    /// Python-Polars alias for group_by columns.
    pub fn group_by(&self, keys: &[&str], aggs: &[Expr]) -> Result<Self, PolarsError> {
        self.group_by_columns(keys, aggs)
    }

    /// Pretty-print the DataFrame using Polars fmt output.
    pub fn fmt_table(&self) -> String {
        format!("{}", self.df)
    }

    /// Return the first N rows as a new collection.
    pub fn head(&self, n: usize) -> Self {
        let df = self.df.head(Some(n));
        Self { df }
    }

    /// Return the last N rows as a new collection.
    pub fn tail(&self, n: usize) -> Self {
        let df = self.df.tail(Some(n));
        Self { df }
    }

    /// Slice rows by offset and length as a new collection.
    pub fn slice(&self, offset: i64, length: usize) -> Self {
        let df = self.df.slice(offset, length);
        Self { df }
    }

    pub fn into_inner(self) -> DataFrame {
        self.df
    }
}

impl From<DataFrame> for PolarsDataFrameCollection {
    fn from(df: DataFrame) -> Self {
        Self::new(df)
    }
}

impl DataFrameCollection for PolarsDataFrameCollection {
    fn dataframe(&self) -> &DataFrame {
        &self.df
    }

    fn dataframe_mut(&mut self) -> &mut DataFrame {
        &mut self.df
    }
}
