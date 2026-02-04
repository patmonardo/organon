//! GDS DataFrame facade (Collections + framing adapter).

use polars::error::PolarsError;
use polars::frame::row::Row;
use polars::lazy::dsl::{len as expr_len, lit as expr_lit};
use polars::prelude::{
    col, AnyValue, Column, DataFrame, DataFrameJoinOps, Expr, IntoLazy, JoinArgs, JoinType,
    JoinTypeOptions, PlSmallStr, QuantileMethod, Schema, Series, SortMultipleOptions,
};
use std::collections::{HashMap, HashSet};

use crate::collections::dataframe::selectors::{expand_selector, Selector};
use crate::collections::dataframe::utils::construction::{
    dataframe_from_columns, dataframe_from_columns_vec, dataframe_from_records,
    dataframe_from_rows, dataframe_from_series, ConstructionOptions,
};
use crate::collections::dataframe::utils::getitem::{
    get_df_item_by_key, DataFrameGetItem, DataFrameKey,
};
use crate::collections::dataframe::utils::parse::{
    parse_into_list_of_expressions_for_df, ExprInput, ParseExprOptions,
};
use crate::collections::dataframe::utils::slice::{slice_dataframe, SliceSpec};
use crate::collections::extensions::framing::{
    FrameShape, FramingConfig, FramingError, FramingSupport,
};

/// GDS DataFrame wrapper for the Collections SDK.
#[derive(Debug, Clone)]
pub struct GDSDataFrame {
    df: DataFrame,
    framing_config: Option<FramingConfig>,
    is_framing_enabled: bool,
}

impl GDSDataFrame {
    fn non_key_columns(&self, keys: &[&str]) -> Vec<String> {
        let key_set: HashSet<&str> = keys.iter().copied().collect();
        self.df
            .get_column_names()
            .iter()
            .filter(|name| !key_set.contains(name.as_str()))
            .map(|name| name.to_string())
            .collect()
    }

    pub fn new(df: DataFrame) -> Self {
        Self {
            df,
            framing_config: None,
            is_framing_enabled: false,
        }
    }

    /// Create an empty DataFrame with a given schema.
    pub fn empty_with_schema(schema: &Schema) -> Self {
        let df = DataFrame::empty_with_schema(schema);
        Self::new(df)
    }

    pub fn dataframe(&self) -> &DataFrame {
        &self.df
    }

    pub fn dataframe_mut(&mut self) -> &mut DataFrame {
        &mut self.df
    }

    pub fn row_count(&self) -> usize {
        self.df.height()
    }

    pub fn column_count(&self) -> usize {
        self.df.width()
    }

    pub fn column_names(&self) -> Vec<String> {
        self.df
            .get_column_names()
            .iter()
            .map(|name| name.to_string())
            .collect()
    }

    pub fn dtypes(&self) -> Vec<polars::prelude::DataType> {
        self.df.dtypes().iter().cloned().collect()
    }

    pub fn is_empty(&self) -> bool {
        self.df.height() == 0
    }

    pub fn from_series(columns: Vec<Series>) -> Result<Self, PolarsError> {
        dataframe_from_series(columns, ConstructionOptions::default())
    }

    /// Construct from Polars series with construction options.
    pub fn from_series_with_options(
        columns: Vec<Series>,
        options: ConstructionOptions,
    ) -> Result<Self, PolarsError> {
        dataframe_from_series(columns, options)
    }

    /// Construct from Polars columns.
    pub fn from_columns(columns: Vec<Column>) -> Result<Self, PolarsError> {
        dataframe_from_columns_vec(columns, ConstructionOptions::default())
    }

    /// Construct from Polars columns with construction options.
    pub fn from_columns_with_options(
        columns: Vec<Column>,
        options: ConstructionOptions,
    ) -> Result<Self, PolarsError> {
        dataframe_from_columns_vec(columns, options)
    }

    /// Construct from rows (slower; row-wise input).
    pub fn from_rows(rows: &[Row]) -> Result<Self, PolarsError> {
        let rows = rows
            .iter()
            .map(|row| row.0.iter().cloned().map(AnyValue::into_static).collect())
            .collect::<Vec<_>>();
        dataframe_from_rows(&rows, ConstructionOptions::default())
    }

    /// Construct from rows with construction options.
    pub fn from_rows_with_options(
        rows: &[Row],
        options: ConstructionOptions,
    ) -> Result<Self, PolarsError> {
        let rows = rows
            .iter()
            .map(|row| row.0.iter().cloned().map(AnyValue::into_static).collect())
            .collect::<Vec<_>>();
        dataframe_from_rows(&rows, options)
    }

    /// Construct from rows with an explicit schema.
    pub fn from_rows_and_schema(rows: &[Row], schema: &Schema) -> Result<Self, PolarsError> {
        let rows = rows
            .iter()
            .map(|row| row.0.iter().cloned().map(AnyValue::into_static).collect())
            .collect::<Vec<_>>();
        dataframe_from_rows(
            &rows,
            ConstructionOptions {
                schema: Some(schema.clone()),
                ..ConstructionOptions::default()
            },
        )
    }

    /// Construct from column-oriented AnyValue data (py-polars style).
    pub fn from_any_columns(
        data: &HashMap<String, Vec<AnyValue<'static>>>,
        options: ConstructionOptions,
    ) -> Result<Self, PolarsError> {
        dataframe_from_columns(data, options)
    }

    /// Construct from row-oriented AnyValue data (py-polars style).
    pub fn from_any_rows(
        rows: &[Vec<AnyValue<'static>>],
        options: ConstructionOptions,
    ) -> Result<Self, PolarsError> {
        dataframe_from_rows(rows, options)
    }

    /// Construct from record-oriented AnyValue data (py-polars style).
    pub fn from_any_records(
        records: &[HashMap<String, AnyValue<'static>>],
        options: ConstructionOptions,
    ) -> Result<Self, PolarsError> {
        dataframe_from_records(records, options)
    }

    /// Select a subset of columns (eager).
    pub fn select_columns(&self, columns: &[&str]) -> Result<Self, PolarsError> {
        let selection: Vec<&str> = columns.iter().copied().collect();
        let df = self.df.select(selection)?;
        Ok(Self::new(df))
    }

    /// Select columns using Polars expressions (py-polars style).
    pub fn select(&self, exprs: &[Expr]) -> Result<Self, PolarsError> {
        let df = self.df.clone().lazy().select(exprs.to_vec()).collect()?;
        Ok(Self::new(df))
    }

    /// Select columns from parsed inputs (py-polars parsing helpers).
    pub fn select_inputs(
        &self,
        inputs: &[ExprInput],
        named_inputs: Option<&HashMap<String, ExprInput>>,
        options: ParseExprOptions,
    ) -> Result<Self, PolarsError> {
        let exprs = parse_into_list_of_expressions_for_df(&self.df, inputs, named_inputs, options)?;
        self.select(&exprs)
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
        Ok(Self::new(df))
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
        Ok(Self::new(df))
    }

    /// Python-Polars alias for with_columns.
    pub fn with_columns(&self, exprs: &[Expr]) -> Result<Self, PolarsError> {
        self.with_columns_exprs(exprs)
    }

    /// Add/replace columns from parsed inputs (py-polars parsing helpers).
    pub fn with_columns_inputs(
        &self,
        inputs: &[ExprInput],
        named_inputs: Option<&HashMap<String, ExprInput>>,
        options: ParseExprOptions,
    ) -> Result<Self, PolarsError> {
        let exprs = parse_into_list_of_expressions_for_df(&self.df, inputs, named_inputs, options)?;
        self.with_columns(&exprs)
    }

    /// Order rows by named columns (eager), using Polars sort options.
    pub fn order_by_columns(
        &self,
        columns: &[&str],
        options: SortMultipleOptions,
    ) -> Result<Self, PolarsError> {
        let by: Vec<PlSmallStr> = columns.iter().map(|name| (*name).into()).collect();
        let df = self.df.sort(by, options)?;
        Ok(Self::new(df))
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
        Ok(Self::new(df))
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
        Ok(Self::new(df))
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

    /// Group by columns and collect all values (list aggregation) for non-key columns.
    pub fn group_by_all(&self, keys: &[&str]) -> Result<Self, PolarsError> {
        let aggs: Vec<Expr> = self
            .non_key_columns(keys)
            .into_iter()
            .map(|name| col(name.as_str()).implode())
            .collect();
        self.group_by_columns(keys, &aggs)
    }

    /// Group by columns and count rows.
    pub fn group_by_len(&self, keys: &[&str], name: Option<&str>) -> Result<Self, PolarsError> {
        let expr = match name {
            Some(alias) => expr_len().alias(alias),
            None => expr_len(),
        };
        self.group_by_columns(keys, &[expr])
    }

    /// Python-Polars alias for len/count.
    pub fn group_by_count(&self, keys: &[&str], name: Option<&str>) -> Result<Self, PolarsError> {
        let alias = name.unwrap_or("count");
        self.group_by_len(keys, Some(alias))
    }

    /// Group by columns and take first values.
    pub fn group_by_first(&self, keys: &[&str], ignore_nulls: bool) -> Result<Self, PolarsError> {
        let aggs: Vec<Expr> = self
            .non_key_columns(keys)
            .into_iter()
            .map(|name| {
                let expr = col(name.as_str());
                if ignore_nulls {
                    expr.drop_nulls().first()
                } else {
                    expr.first()
                }
            })
            .collect();
        self.group_by_columns(keys, &aggs)
    }

    /// Group by columns and take last values.
    pub fn group_by_last(&self, keys: &[&str], ignore_nulls: bool) -> Result<Self, PolarsError> {
        let aggs: Vec<Expr> = self
            .non_key_columns(keys)
            .into_iter()
            .map(|name| {
                let expr = col(name.as_str());
                if ignore_nulls {
                    expr.drop_nulls().last()
                } else {
                    expr.last()
                }
            })
            .collect();
        self.group_by_columns(keys, &aggs)
    }

    /// Group by columns and take maximum values.
    pub fn group_by_max(&self, keys: &[&str]) -> Result<Self, PolarsError> {
        let aggs: Vec<Expr> = self
            .non_key_columns(keys)
            .into_iter()
            .map(|name| col(name.as_str()).max())
            .collect();
        self.group_by_columns(keys, &aggs)
    }

    /// Group by columns and take minimum values.
    pub fn group_by_min(&self, keys: &[&str]) -> Result<Self, PolarsError> {
        let aggs: Vec<Expr> = self
            .non_key_columns(keys)
            .into_iter()
            .map(|name| col(name.as_str()).min())
            .collect();
        self.group_by_columns(keys, &aggs)
    }

    /// Group by columns and take mean values.
    pub fn group_by_mean(&self, keys: &[&str]) -> Result<Self, PolarsError> {
        let aggs: Vec<Expr> = self
            .non_key_columns(keys)
            .into_iter()
            .map(|name| col(name.as_str()).mean())
            .collect();
        self.group_by_columns(keys, &aggs)
    }

    /// Group by columns and take median values.
    pub fn group_by_median(&self, keys: &[&str]) -> Result<Self, PolarsError> {
        let aggs: Vec<Expr> = self
            .non_key_columns(keys)
            .into_iter()
            .map(|name| col(name.as_str()).median())
            .collect();
        self.group_by_columns(keys, &aggs)
    }

    /// Group by columns and count unique values per group.
    pub fn group_by_n_unique(&self, keys: &[&str]) -> Result<Self, PolarsError> {
        let aggs: Vec<Expr> = self
            .non_key_columns(keys)
            .into_iter()
            .map(|name| col(name.as_str()).n_unique())
            .collect();
        self.group_by_columns(keys, &aggs)
    }

    /// Group by columns and compute quantiles per group.
    pub fn group_by_quantile(
        &self,
        keys: &[&str],
        quantile: f64,
        method: QuantileMethod,
    ) -> Result<Self, PolarsError> {
        let q_expr = expr_lit(quantile);
        let aggs: Vec<Expr> = self
            .non_key_columns(keys)
            .into_iter()
            .map(|name| col(name.as_str()).quantile(q_expr.clone(), method))
            .collect();
        self.group_by_columns(keys, &aggs)
    }

    /// Group by columns and compute sums per group.
    pub fn group_by_sum(&self, keys: &[&str]) -> Result<Self, PolarsError> {
        let aggs: Vec<Expr> = self
            .non_key_columns(keys)
            .into_iter()
            .map(|name| col(name.as_str()).sum())
            .collect();
        self.group_by_columns(keys, &aggs)
    }

    /// Join with another collection using explicit left/right keys.
    pub fn join(
        &self,
        other: &GDSDataFrame,
        left_on: &[&str],
        right_on: &[&str],
        args: JoinArgs,
        options: Option<JoinTypeOptions>,
    ) -> Result<Self, PolarsError> {
        let left_on = left_on
            .iter()
            .map(|name| PlSmallStr::from(*name))
            .collect::<Vec<_>>();
        let right_on = right_on
            .iter()
            .map(|name| PlSmallStr::from(*name))
            .collect::<Vec<_>>();
        let df = self.df.join(&other.df, left_on, right_on, args, options)?;
        Ok(Self::new(df))
    }

    /// Join with another collection using the same key columns.
    pub fn join_on(
        &self,
        other: &GDSDataFrame,
        on: &[&str],
        how: JoinType,
        options: Option<JoinTypeOptions>,
    ) -> Result<Self, PolarsError> {
        self.join(other, on, on, JoinArgs::new(how), options)
    }

    /// Pretty-print the DataFrame using Polars fmt output.
    pub fn fmt_table(&self) -> String {
        format!("{}", self.df)
    }

    /// Return the first N rows as a new collection.
    pub fn head(&self, n: usize) -> Self {
        let df = self.df.head(Some(n));
        Self::new(df)
    }

    /// Return the last N rows as a new collection.
    pub fn tail(&self, n: usize) -> Self {
        let df = self.df.tail(Some(n));
        Self::new(df)
    }

    /// Slice rows by offset and length as a new collection.
    pub fn slice(&self, offset: i64, length: usize) -> Self {
        let df = self.df.slice(offset, length);
        Self::new(df)
    }

    /// Slice rows using a Python-like slice spec.
    pub fn slice_spec(&self, spec: SliceSpec) -> Result<Self, PolarsError> {
        let df = slice_dataframe(&self.df, spec)?;
        Ok(Self::new(df))
    }

    /// Python-like getitem selection.
    pub fn getitem(&self, key: DataFrameKey) -> Result<DataFrameGetItem, PolarsError> {
        get_df_item_by_key(&self.df, key)
    }

    pub fn into_inner(self) -> DataFrame {
        self.df
    }

    fn resolve_shape(&self) -> Result<FrameShape, FramingError> {
        let config = self
            .framing_config
            .as_ref()
            .ok_or(FramingError::MissingConfig)?;

        let mut shape = config.shape;
        let rows = self.df.height();
        let cols = self.df.width();

        if shape.rows == 0 {
            shape.rows = rows;
        }
        if shape.cols == 0 {
            shape.cols = cols;
        }

        if shape.rows != rows || shape.cols != cols {
            return Err(FramingError::InvalidShape);
        }

        Ok(shape)
    }

    fn ensure_shape(&self) -> Result<FrameShape, FramingError> {
        if !self.is_framing_enabled {
            return Err(FramingError::FramingNotEnabled);
        }
        self.resolve_shape()
    }
}

impl From<DataFrame> for GDSDataFrame {
    fn from(df: DataFrame) -> Self {
        Self::new(df)
    }
}

impl FramingSupport<AnyValue<'static>> for GDSDataFrame {
    fn enable_framing(&mut self, config: FramingConfig) -> Result<(), FramingError> {
        self.framing_config = Some(config);
        self.is_framing_enabled = true;
        self.resolve_shape()?;
        Ok(())
    }

    fn disable_framing(&mut self) {
        self.framing_config = None;
        self.is_framing_enabled = false;
    }

    fn is_framing_enabled(&self) -> bool {
        self.is_framing_enabled
    }

    fn frame_shape(&self) -> Option<FrameShape> {
        if !self.is_framing_enabled {
            return None;
        }
        self.resolve_shape().ok()
    }

    fn get_cell(&self, row: usize, col: usize) -> Result<AnyValue<'static>, FramingError> {
        let shape = self.ensure_shape()?;
        if row >= shape.rows || col >= shape.cols {
            return Err(FramingError::OutOfBounds { row, col });
        }
        let row_values = self.df.get(row).ok_or(FramingError::MissingValue(row))?;
        let value = row_values
            .get(col)
            .ok_or(FramingError::MissingValue(row))?
            .clone()
            .into_static();
        Ok(value)
    }

    fn set_cell(
        &mut self,
        _row: usize,
        _col: usize,
        _value: AnyValue<'static>,
    ) -> Result<(), FramingError> {
        Err(FramingError::UnsupportedOperation(
            "Polars DataFrame framing is read-only; mutate via Polars APIs".to_string(),
        ))
    }

    fn row_values(&self, row: usize) -> Result<Vec<AnyValue<'static>>, FramingError> {
        let shape = self.ensure_shape()?;
        if row >= shape.rows {
            return Err(FramingError::OutOfBounds { row, col: 0 });
        }
        let values = self
            .df
            .get(row)
            .ok_or(FramingError::MissingValue(row))?
            .into_iter()
            .map(|value| value.into_static())
            .collect();
        Ok(values)
    }

    fn col_values(&self, col: usize) -> Result<Vec<AnyValue<'static>>, FramingError> {
        let shape = self.ensure_shape()?;
        if col >= shape.cols {
            return Err(FramingError::OutOfBounds { row: 0, col });
        }
        let mut values = Vec::with_capacity(shape.rows);
        for row in 0..shape.rows {
            values.push(self.get_cell(row, col)?);
        }
        Ok(values)
    }
}
