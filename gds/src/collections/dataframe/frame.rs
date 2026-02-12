//! GDS DataFrame facade (Collections + framing adapter).

use either::Either;
use polars::error::{ErrString, PolarsError};
use polars::frame::row::Row;
use polars::lazy::dsl::{len as expr_len, lit as expr_lit};
use polars::prelude::{
    col, AnyValue, Column, DataFrame, DataFrameJoinOps, Expr, FillNullStrategy, IntoLazy, JoinArgs,
    JoinType, JoinTypeOptions, PlSmallStr, QuantileMethod, Schema, SchemaExt, Series,
    SortMultipleOptions, UnpivotArgsIR,
};
use std::collections::{HashMap, HashSet};

use polars_ops::frame::join::{AsofJoin, AsofJoinBy, AsofStrategy};
use polars_ops::frame::pivot::UnpivotDF;
use polars_ops::frame::DataFrameOps;

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

    /// Create a copy of this DataFrame.
    pub fn clone(&self) -> Self {
        Self::new(self.df.clone())
    }

    /// Create an empty (n = 0) or null-filled (n > 0) copy of the DataFrame.
    pub fn clear(&self, n: usize) -> Result<Self, PolarsError> {
        if n == 0 {
            let df = DataFrame::empty_with_schema(&self.df.schema());
            return Ok(Self::new(df));
        }
        let schema = self.df.schema();
        let mut columns = Vec::with_capacity(schema.len());
        for field in schema.iter_fields() {
            let series = Series::full_null(field.name().clone(), n, field.dtype());
            columns.push(Column::from(series));
        }
        Ok(Self::new(DataFrame::new(columns)?))
    }

    /// Get the first n rows.
    pub fn limit(&self, n: usize) -> Self {
        let df = self.df.head(Some(n));
        Self::new(df)
    }

    /// Reverse the row order.
    pub fn reverse(&self) -> Self {
        let df = self.df.clone().reverse();
        Self::new(df)
    }

    /// Get the DataFrame as a list of Series.
    pub fn get_columns(&self) -> Result<Vec<Series>, PolarsError> {
        self.df
            .get_columns()
            .iter()
            .map(|column| {
                column
                    .as_series()
                    .cloned()
                    .ok_or_else(|| PolarsError::ComputeError("column is not a series".into()))
            })
            .collect()
    }

    /// Get a single column by name.
    pub fn get_column(&self, name: &str) -> Result<Series, PolarsError> {
        let column = self.df.column(name)?;
        column
            .as_series()
            .cloned()
            .ok_or_else(|| PolarsError::ComputeError("column is not a series".into()))
    }

    /// Find the index of a column by name.
    pub fn get_column_index(&self, name: &str) -> Result<usize, PolarsError> {
        self.df
            .get_column_names_str()
            .iter()
            .position(|col| *col == name)
            .ok_or_else(|| PolarsError::ColumnNotFound(ErrString::from(name.to_string())))
    }

    /// Rename a single column by name.
    pub fn rename_column(&self, column: &str, name: &str) -> Result<Self, PolarsError> {
        let mut df = self.df.clone();
        df.rename(column, PlSmallStr::from(name))?;
        Ok(Self::new(df))
    }

    /// Rename multiple columns by applying (from, to) pairs in order.
    pub fn rename_columns(&self, mapping: &[(&str, &str)]) -> Result<Self, PolarsError> {
        let mut df = self.df.clone();
        for (from, to) in mapping {
            df.rename(from, PlSmallStr::from(*to))?;
        }
        Ok(Self::new(df))
    }

    /// Cast columns using a schema (column name -> dtype).
    pub fn cast(&self, schema: &Schema, strict: bool) -> Result<Self, PolarsError> {
        if strict {
            for (name, _) in schema.iter() {
                if self.df.get_column_index(name).is_none() {
                    return Err(PolarsError::ColumnNotFound(ErrString::from(
                        name.to_string(),
                    )));
                }
            }
        }

        let mut columns = Vec::with_capacity(self.df.width());
        for column in self.df.get_columns() {
            let next = match schema.get(column.name().as_str()) {
                Some(dtype) => column.cast(dtype)?,
                None => column.clone(),
            };
            columns.push(next);
        }
        Ok(Self::new(DataFrame::new(columns)?))
    }

    /// Rechunk columns into a single contiguous chunk.
    pub fn rechunk(&self) -> Self {
        let mut df = self.df.clone();
        df.rechunk_mut();
        Self::new(df)
    }

    /// Replace a column at an index with a new Series.
    pub fn replace_column(&self, index: usize, column: Series) -> Result<Self, PolarsError> {
        let mut df = self.df.clone();
        df.replace_column(index, Column::from(column))?;
        Ok(Self::new(df))
    }

    /// Insert a column at an index.
    pub fn insert_column(&self, index: usize, column: Series) -> Result<Self, PolarsError> {
        let mut df = self.df.clone();
        df.insert_column(index, Column::from(column))?;
        Ok(Self::new(df))
    }

    /// Drop a single column by name.
    pub fn drop_column(&self, name: &str) -> Result<Self, PolarsError> {
        let df = self.df.drop(name)?;
        Ok(Self::new(df))
    }

    /// Drop multiple columns by name.
    pub fn drop_columns(&self, columns: &[&str], strict: bool) -> Result<Self, PolarsError> {
        if columns.is_empty() {
            return Ok(Self::new(self.df.clone()));
        }
        if strict {
            for name in columns {
                if self.df.get_column_index(name).is_none() {
                    return Err(PolarsError::ColumnNotFound(ErrString::from(
                        (*name).to_string(),
                    )));
                }
            }
        }
        let df = self.df.drop_many(columns.iter().copied());
        Ok(Self::new(df))
    }

    /// Drop a column in place and return it.
    pub fn drop_in_place(&mut self, name: &str) -> Result<Series, PolarsError> {
        let column = self.df.drop_in_place(name)?;
        column
            .as_series()
            .cloned()
            .ok_or_else(|| PolarsError::ComputeError("column is not a series".into()))
    }

    /// Fill null values using a strategy (forward/backward/mean/min/max).
    pub fn fill_null_strategy(&self, strategy: FillNullStrategy) -> Result<Self, PolarsError> {
        let df = self.df.fill_null(strategy)?;
        Ok(Self::new(df))
    }

    /// Fill null values using an expression.
    pub fn fill_null_expr(&self, value: Expr) -> Result<Self, PolarsError> {
        let df = self.df.clone().lazy().fill_null(value).collect()?;
        Ok(Self::new(df))
    }

    /// Fill NaN values using an expression.
    pub fn fill_nan_expr(&self, value: Expr) -> Result<Self, PolarsError> {
        let df = self.df.clone().lazy().fill_nan(value).collect()?;
        Ok(Self::new(df))
    }

    /// Drop rows with nulls in the selected columns (or all columns).
    pub fn drop_nulls(&self, subset: Option<&[&str]>) -> Result<Self, PolarsError> {
        let df = match subset {
            Some(names) => {
                let names = names
                    .iter()
                    .map(|name| (*name).to_string())
                    .collect::<Vec<_>>();
                self.df.drop_nulls::<String>(Some(names.as_slice()))?
            }
            None => self.df.drop_nulls::<String>(None)?,
        };
        Ok(Self::new(df))
    }

    /// Drop rows with NaN values in the selected columns (or all columns).
    pub fn drop_nans(&self, subset: Option<&[&str]>) -> Result<Self, PolarsError> {
        let columns: Vec<Expr> = match subset {
            Some(names) => names.iter().map(|name| col(*name)).collect(),
            None => self
                .df
                .get_column_names_str()
                .iter()
                .map(|name| col(*name))
                .collect(),
        };

        if columns.is_empty() {
            return Ok(Self::new(self.df.clone()));
        }

        let mut predicate = columns[0].clone().is_nan().not().fill_null(expr_lit(true));
        for expr in columns.into_iter().skip(1) {
            let next = expr.is_nan().not().fill_null(expr_lit(true));
            predicate = predicate.and(next);
        }

        let df = self.df.clone().lazy().filter(predicate).collect()?;
        Ok(Self::new(df))
    }

    /// Return a new DataFrame grown horizontally by stacking multiple Series.
    pub fn hstack(&self, columns: &[Series]) -> Result<Self, PolarsError> {
        let columns = columns
            .iter()
            .cloned()
            .map(Column::from)
            .collect::<Vec<_>>();
        let df = self.df.hstack(&columns)?;
        Ok(Self::new(df))
    }

    /// Grow this DataFrame horizontally by stacking multiple Series in place.
    pub fn hstack_in_place(&mut self, columns: &[Series]) -> Result<(), PolarsError> {
        let columns = columns
            .iter()
            .cloned()
            .map(Column::from)
            .collect::<Vec<_>>();
        self.df = self.df.hstack(&columns)?;
        Ok(())
    }

    /// Grow this DataFrame vertically by stacking another DataFrame.
    pub fn vstack(&self, other: &GDSDataFrame) -> Result<Self, PolarsError> {
        let df = self.df.vstack(&other.df)?;
        Ok(Self::new(df))
    }

    /// Grow this DataFrame vertically by stacking another DataFrame in place.
    pub fn vstack_in_place(&mut self, other: &GDSDataFrame) -> Result<(), PolarsError> {
        self.df.vstack_mut(&other.df)?;
        Ok(())
    }

    /// Extend the memory backed by this DataFrame with the values from other.
    pub fn extend(&mut self, other: &GDSDataFrame) -> Result<(), PolarsError> {
        self.df.extend(&other.df)?;
        Ok(())
    }

    /// Shift values by the given number of indices.
    pub fn shift(&self, periods: i64) -> Self {
        let df = self.df.shift(periods);
        Self::new(df)
    }

    /// Shift values and fill the gaps with an expression.
    pub fn shift_and_fill(&self, periods: i64, fill_value: Expr) -> Result<Self, PolarsError> {
        let df = self
            .df
            .clone()
            .lazy()
            .shift_and_fill(expr_lit(periods), fill_value)
            .collect()?;
        Ok(Self::new(df))
    }

    /// Sample a fixed number of rows.
    pub fn sample_n(
        &self,
        n: usize,
        with_replacement: bool,
        shuffle: bool,
        seed: Option<u64>,
    ) -> Result<Self, PolarsError> {
        let df = self
            .df
            .sample_n_literal(n, with_replacement, shuffle, seed)?;
        Ok(Self::new(df))
    }

    /// Sample a fraction of rows.
    pub fn sample_frac(
        &self,
        fraction: f64,
        with_replacement: bool,
        shuffle: bool,
        seed: Option<u64>,
    ) -> Result<Self, PolarsError> {
        let n = (self.df.height() as f64 * fraction) as usize;
        self.sample_n(n, with_replacement, shuffle, seed)
    }

    /// Sample rows by count or fraction. Provide exactly one of `n` or `fraction`.
    pub fn sample(
        &self,
        n: Option<usize>,
        fraction: Option<f64>,
        with_replacement: bool,
        shuffle: bool,
        seed: Option<u64>,
    ) -> Result<Self, PolarsError> {
        match (n, fraction) {
            (Some(n), None) => self.sample_n(n, with_replacement, shuffle, seed),
            (None, Some(fraction)) => self.sample_frac(fraction, with_replacement, shuffle, seed),
            _ => Err(PolarsError::ComputeError(
                "sample requires exactly one of n or fraction".into(),
            )),
        }
    }

    /// Explode the DataFrame to long format by exploding the given columns.
    pub fn explode(&self, columns: &[&str]) -> Result<Self, PolarsError> {
        let df = self.df.explode(columns.iter().copied())?;
        Ok(Self::new(df))
    }

    /// Unnest struct columns into separate fields.
    pub fn unnest(&self, columns: &[&str], separator: Option<&str>) -> Result<Self, PolarsError> {
        let cols = columns
            .iter()
            .map(|name| PlSmallStr::from(*name))
            .collect::<Vec<_>>();
        let df = self.df.unnest(cols, separator)?;
        Ok(Self::new(df))
    }

    /// Unpivot a DataFrame from wide to long format.
    pub fn unpivot(
        &self,
        on: &[&str],
        index: &[&str],
        variable_name: Option<&str>,
        value_name: Option<&str>,
    ) -> Result<Self, PolarsError> {
        let args = UnpivotArgsIR {
            on: on.iter().map(|name| PlSmallStr::from(*name)).collect(),
            index: index.iter().map(|name| PlSmallStr::from(*name)).collect(),
            variable_name: variable_name.map(PlSmallStr::from),
            value_name: value_name.map(PlSmallStr::from),
        };
        let df = self.df.unpivot2(args)?;
        Ok(Self::new(df))
    }

    /// Transpose a DataFrame.
    pub fn transpose(&self, keep_names_as: Option<&str>) -> Result<Self, PolarsError> {
        let mut df = self.df.clone();
        let df = df.transpose(keep_names_as, None)?;
        Ok(Self::new(df))
    }

    /// Transpose a DataFrame with explicit column names.
    pub fn transpose_with_names(
        &self,
        keep_names_as: Option<&str>,
        new_column_names: Vec<String>,
    ) -> Result<Self, PolarsError> {
        let mut df = self.df.clone();
        let df = df.transpose(keep_names_as, Some(Either::Right(new_column_names)))?;
        Ok(Self::new(df))
    }

    /// Split into multiple DataFrames partitioned by groups.
    pub fn partition_by(
        &self,
        columns: &[&str],
        include_key: bool,
        stable: bool,
    ) -> Result<Vec<GDSDataFrame>, PolarsError> {
        let dfs = if stable {
            self.df
                .partition_by_stable(columns.iter().copied(), include_key)?
        } else {
            self.df.partition_by(columns.iter().copied(), include_key)?
        };
        Ok(dfs.into_iter().map(GDSDataFrame::new).collect())
    }

    /// Convert categorical variables into dummy/indicator variables.
    pub fn to_dummies(
        &self,
        columns: Option<&[&str]>,
        separator: Option<&str>,
        drop_first: bool,
        drop_nulls: bool,
    ) -> Result<Self, PolarsError> {
        let df = match columns {
            Some(columns) => self.df.columns_to_dummies(
                columns.iter().copied().collect(),
                separator,
                drop_first,
                drop_nulls,
            )?,
            None => self.df.to_dummies(separator, drop_first, drop_nulls)?,
        };
        Ok(Self::new(df))
    }

    /// Get the values of a single row by index.
    pub fn row(&self, index: usize) -> Result<Vec<AnyValue<'static>>, PolarsError> {
        let row = self.df.get_row(index)?;
        Ok(row.0.into_iter().map(AnyValue::into_static).collect())
    }

    /// Return all rows as a vector of row values.
    pub fn rows(&self) -> Result<Vec<Vec<AnyValue<'static>>>, PolarsError> {
        let mut out = Vec::with_capacity(self.df.height());
        for index in 0..self.df.height() {
            out.push(self.row(index)?);
        }
        Ok(out)
    }

    /// Return an iterator over row values. Uses a materialized Vec internally.
    pub fn iter_rows(
        &self,
        _buffer_size: usize,
    ) -> Result<std::vec::IntoIter<Vec<AnyValue<'static>>>, PolarsError> {
        Ok(self.rows()?.into_iter())
    }

    /// Return an iterator over the columns as Series.
    pub fn iter_columns(&self) -> Result<std::vec::IntoIter<Series>, PolarsError> {
        Ok(self.get_columns()?.into_iter())
    }

    /// Return a vector of DataFrame slices, each with up to n_rows rows.
    pub fn iter_slices(&self, n_rows: usize) -> Result<Vec<GDSDataFrame>, PolarsError> {
        if n_rows == 0 {
            return Err(PolarsError::ComputeError(
                "iter_slices requires n_rows > 0".into(),
            ));
        }
        let mut slices = Vec::new();
        let mut offset = 0i64;
        let height = self.df.height();
        while (offset as usize) < height {
            let len = std::cmp::min(n_rows, height - offset as usize);
            slices.push(GDSDataFrame::new(self.df.slice(offset, len)));
            offset += n_rows as i64;
        }
        Ok(slices)
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

    /// Create an eager GroupBy wrapper over the DataFrame.
    pub fn group_by_builder(
        &self,
        keys: &[&str],
        maintain_order: bool,
    ) -> Result<GDSGroupBy<'_>, PolarsError> {
        let group_by = if maintain_order {
            self.df.group_by_stable(keys.iter().copied())?
        } else {
            self.df.group_by(keys.iter().copied())?
        };
        Ok(GDSGroupBy::new(group_by))
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

    /// As-of join using explicit left/right keys.
    pub fn join_asof(
        &self,
        other: &GDSDataFrame,
        left_on: &str,
        right_on: &str,
        strategy: AsofStrategy,
        tolerance: Option<AnyValue<'static>>,
        allow_eq: bool,
        check_sortedness: bool,
        suffix: Option<&str>,
        coalesce: bool,
    ) -> Result<Self, PolarsError> {
        let left_key = self.df.column(left_on)?.as_materialized_series();
        let right_key = other.df.column(right_on)?.as_materialized_series();
        let df = self.df._join_asof(
            &other.df,
            left_key,
            right_key,
            strategy,
            tolerance,
            suffix.map(PlSmallStr::from),
            None,
            coalesce,
            allow_eq,
            check_sortedness,
        )?;
        Ok(Self::new(df))
    }

    /// As-of join using explicit left/right keys and by-group columns.
    pub fn join_asof_by(
        &self,
        other: &GDSDataFrame,
        left_on: &str,
        right_on: &str,
        left_by: &[&str],
        right_by: &[&str],
        strategy: AsofStrategy,
        tolerance: Option<AnyValue<'static>>,
        allow_eq: bool,
        check_sortedness: bool,
        suffix: Option<&str>,
        coalesce: bool,
    ) -> Result<Self, PolarsError> {
        let left_key = self.df.column(left_on)?.as_materialized_series();
        let right_key = other.df.column(right_on)?.as_materialized_series();
        let left_by = left_by
            .iter()
            .map(|name| PlSmallStr::from(*name))
            .collect::<Vec<_>>();
        let right_by = right_by
            .iter()
            .map(|name| PlSmallStr::from(*name))
            .collect::<Vec<_>>();
        let df = self.df._join_asof_by(
            &other.df,
            left_key,
            right_key,
            left_by,
            right_by,
            strategy,
            tolerance,
            suffix.map(PlSmallStr::from),
            None,
            coalesce,
            allow_eq,
            check_sortedness,
        )?;
        Ok(Self::new(df))
    }

    /// Join using predicates (lazy join_where under the hood).
    pub fn join_where(
        &self,
        other: &GDSDataFrame,
        predicates: Vec<Expr>,
        args: JoinArgs,
    ) -> Result<Self, PolarsError> {
        let lf = self
            .df
            .clone()
            .lazy()
            .join_builder()
            .with(other.df.clone().lazy())
            .how(args.how)
            .validate(args.validation)
            .join_nulls(args.nulls_equal)
            .coalesce(args.coalesce)
            .maintain_order(args.maintain_order);

        let lf = match args.suffix {
            Some(suffix) => lf.suffix(suffix),
            None => lf,
        };

        let df = lf.join_where(predicates).collect()?;
        Ok(Self::new(df))
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

/// Eager GroupBy wrapper for GDS DataFrame operations.
pub struct GDSGroupBy<'a> {
    group_by: polars::prelude::GroupBy<'a>,
}

#[allow(deprecated)]
impl<'a> GDSGroupBy<'a> {
    pub fn new(group_by: polars::prelude::GroupBy<'a>) -> Self {
        Self { group_by }
    }

    pub fn select(self, columns: &[&str]) -> Self {
        let group_by = self.group_by.select(columns.iter().copied());
        Self { group_by }
    }

    pub fn mean(&self) -> Result<GDSDataFrame, PolarsError> {
        Ok(GDSDataFrame::new(self.group_by.mean()?))
    }

    pub fn sum(&self) -> Result<GDSDataFrame, PolarsError> {
        Ok(GDSDataFrame::new(self.group_by.sum()?))
    }

    pub fn min(&self) -> Result<GDSDataFrame, PolarsError> {
        Ok(GDSDataFrame::new(self.group_by.min()?))
    }

    pub fn max(&self) -> Result<GDSDataFrame, PolarsError> {
        Ok(GDSDataFrame::new(self.group_by.max()?))
    }

    pub fn first(&self) -> Result<GDSDataFrame, PolarsError> {
        Ok(GDSDataFrame::new(self.group_by.first()?))
    }

    pub fn last(&self) -> Result<GDSDataFrame, PolarsError> {
        Ok(GDSDataFrame::new(self.group_by.last()?))
    }

    pub fn n_unique(&self) -> Result<GDSDataFrame, PolarsError> {
        Ok(GDSDataFrame::new(self.group_by.n_unique()?))
    }

    pub fn quantile(
        &self,
        quantile: f64,
        method: QuantileMethod,
    ) -> Result<GDSDataFrame, PolarsError> {
        Ok(GDSDataFrame::new(self.group_by.quantile(quantile, method)?))
    }

    pub fn median(&self) -> Result<GDSDataFrame, PolarsError> {
        Ok(GDSDataFrame::new(self.group_by.median()?))
    }

    pub fn count(&self) -> Result<GDSDataFrame, PolarsError> {
        Ok(GDSDataFrame::new(self.group_by.count()?))
    }

    pub fn groups(&self) -> Result<GDSDataFrame, PolarsError> {
        Ok(GDSDataFrame::new(self.group_by.groups()?))
    }
}
