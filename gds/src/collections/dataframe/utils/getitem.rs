//! Getitem helpers for DataFrame/Series (py-polars inspired).

use polars::error::PolarsError;
use polars::prelude::{
    BooleanChunked, Column, DataFrame, IdxCa, NamedFrom, NewChunkedArray, PolarsResult, Series,
};

use crate::collections::dataframe::utils::slice::{slice_dataframe, slice_series, SliceSpec};

/// Series selection keys.
#[derive(Debug, Clone)]
pub enum SeriesKey {
    Index(i64),
    Slice(SliceSpec),
    Indices(Vec<i64>),
    Mask(Vec<bool>),
}

/// Series getitem result.
#[derive(Debug, Clone)]
pub enum SeriesGetItem {
    Value(polars::prelude::AnyValue<'static>),
    Series(Series),
}

/// Row selector for DataFrame getitem.
#[derive(Debug, Clone)]
pub enum RowSelector {
    Index(i64),
    Slice(SliceSpec),
    Indices(Vec<i64>),
    Mask(Vec<bool>),
}

/// Column selector for DataFrame getitem.
#[derive(Debug, Clone)]
pub enum ColSelector {
    Name(String),
    Names(Vec<String>),
    Index(usize),
    Indices(Vec<usize>),
    Mask(Vec<bool>),
    Slice(SliceSpec),
}

/// DataFrame selection key.
#[derive(Debug, Clone)]
pub enum DataFrameKey {
    Column(String),
    Columns(Vec<String>),
    ColumnIndex(usize),
    ColumnIndices(Vec<usize>),
    ColumnMask(Vec<bool>),
    Row(RowSelector),
    Tuple(RowSelector, ColSelector),
}

/// DataFrame getitem result.
#[derive(Debug, Clone)]
pub enum DataFrameGetItem {
    Frame(DataFrame),
    Series(Series),
    Value(polars::prelude::AnyValue<'static>),
}

pub fn get_series_item_by_key(series: &Series, key: SeriesKey) -> PolarsResult<SeriesGetItem> {
    match key {
        SeriesKey::Index(index) => {
            let index = normalize_index(index, series.len())?;
            let value = series.get(index)?;
            Ok(SeriesGetItem::Value(value.into_static()))
        }
        SeriesKey::Slice(spec) => {
            let sliced = slice_series(series, spec)?;
            Ok(SeriesGetItem::Series(sliced))
        }
        SeriesKey::Indices(indices) => {
            let indices = normalize_indices(&indices, series.len())?;
            let taken = series.take(&IdxCa::new("idx".into(), indices))?;
            Ok(SeriesGetItem::Series(taken))
        }
        SeriesKey::Mask(mask) => {
            if mask.len() != series.len() {
                return Err(PolarsError::ComputeError(
                    format!(
                        "expected {} values when selecting series by boolean mask, got {}",
                        series.len(),
                        mask.len()
                    )
                    .into(),
                ));
            }
            let mask = BooleanChunked::from_slice("mask".into(), &mask);
            let filtered = series.filter(&mask)?;
            Ok(SeriesGetItem::Series(filtered))
        }
    }
}

pub fn get_df_item_by_key(df: &DataFrame, key: DataFrameKey) -> PolarsResult<DataFrameGetItem> {
    match key {
        DataFrameKey::Column(name) => {
            let series = df
                .column(&name)?
                .as_series()
                .cloned()
                .ok_or_else(|| PolarsError::ComputeError("column is not a series".into()))?;
            Ok(DataFrameGetItem::Series(series))
        }
        DataFrameKey::Columns(names) => {
            let refs: Vec<&str> = names.iter().map(|name| name.as_str()).collect();
            Ok(DataFrameGetItem::Frame(df.select(refs)?))
        }
        DataFrameKey::ColumnIndex(index) => {
            let series = column_at_index(df, index)?;
            Ok(DataFrameGetItem::Series(series))
        }
        DataFrameKey::ColumnIndices(indices) => {
            let columns = indices
                .into_iter()
                .map(|index| column_at_index(df, index))
                .collect::<PolarsResult<Vec<_>>>()?;
            Ok(DataFrameGetItem::Frame(DataFrame::new(
                columns.into_iter().map(Column::from).collect(),
            )?))
        }
        DataFrameKey::ColumnMask(mask) => {
            if mask.len() != df.width() {
                return Err(PolarsError::ComputeError(
                    format!(
                        "expected {} values when selecting columns by boolean mask, got {}",
                        df.width(),
                        mask.len()
                    )
                    .into(),
                ));
            }
            let mut columns = Vec::new();
            for (idx, column) in df.get_columns().iter().enumerate() {
                if mask[idx] {
                    columns.push(column.clone());
                }
            }
            Ok(DataFrameGetItem::Frame(DataFrame::new(columns)?))
        }
        DataFrameKey::Row(selector) => {
            let frame = select_rows(df, selector)?;
            Ok(DataFrameGetItem::Frame(frame))
        }
        DataFrameKey::Tuple(row_selector, col_selector) => {
            let selection = select_columns(df, col_selector)?;
            match selection {
                ColumnSelection::Series(series) => match select_series(series, row_selector)? {
                    SeriesGetItem::Value(value) => Ok(DataFrameGetItem::Value(value)),
                    SeriesGetItem::Series(series) => Ok(DataFrameGetItem::Series(series)),
                },
                ColumnSelection::Frame(frame) => {
                    let frame = select_rows(&frame, row_selector)?;
                    Ok(DataFrameGetItem::Frame(frame))
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
enum ColumnSelection {
    Frame(DataFrame),
    Series(Series),
}

fn select_columns(df: &DataFrame, selector: ColSelector) -> PolarsResult<ColumnSelection> {
    match selector {
        ColSelector::Name(name) => Ok(ColumnSelection::Series(
            df.column(&name)?
                .as_series()
                .cloned()
                .ok_or_else(|| PolarsError::ComputeError("column is not a series".into()))?,
        )),
        ColSelector::Names(names) => {
            let refs: Vec<&str> = names.iter().map(|name| name.as_str()).collect();
            Ok(ColumnSelection::Frame(df.select(refs)?))
        }
        ColSelector::Index(index) => Ok(ColumnSelection::Series(column_at_index(df, index)?)),
        ColSelector::Indices(indices) => {
            let columns = indices
                .into_iter()
                .map(|index| column_at_index(df, index))
                .collect::<PolarsResult<Vec<_>>>()?;
            Ok(ColumnSelection::Frame(DataFrame::new(
                columns.into_iter().map(Column::from).collect(),
            )?))
        }
        ColSelector::Mask(mask) => {
            if mask.len() != df.width() {
                return Err(PolarsError::ComputeError(
                    format!(
                        "expected {} values when selecting columns by boolean mask, got {}",
                        df.width(),
                        mask.len()
                    )
                    .into(),
                ));
            }
            let mut columns = Vec::new();
            for (idx, column) in df.get_columns().iter().enumerate() {
                if mask[idx] {
                    columns.push(column.clone());
                }
            }
            Ok(ColumnSelection::Frame(DataFrame::new(columns)?))
        }
        ColSelector::Slice(spec) => {
            let indices = slice_indices(df.width(), spec)?;
            let columns = indices
                .into_iter()
                .map(|index| column_at_index(df, index))
                .collect::<PolarsResult<Vec<_>>>()?;
            Ok(ColumnSelection::Frame(DataFrame::new(
                columns.into_iter().map(Column::from).collect(),
            )?))
        }
    }
}

fn select_rows(df: &DataFrame, selector: RowSelector) -> PolarsResult<DataFrame> {
    match selector {
        RowSelector::Index(index) => {
            let index = normalize_index(index, df.height())? as i64;
            Ok(df.slice(index, 1))
        }
        RowSelector::Slice(spec) => slice_dataframe(df, spec),
        RowSelector::Indices(indices) => {
            let indices = normalize_indices(&indices, df.height())?;
            df.take(&IdxCa::new("idx".into(), indices))
        }
        RowSelector::Mask(mask) => {
            if mask.len() != df.height() {
                return Err(PolarsError::ComputeError(
                    format!(
                        "expected {} values when selecting rows by boolean mask, got {}",
                        df.height(),
                        mask.len()
                    )
                    .into(),
                ));
            }
            let mask = BooleanChunked::from_slice("mask".into(), &mask);
            df.filter(&mask)
        }
    }
}

fn select_series(series: Series, selector: RowSelector) -> PolarsResult<SeriesGetItem> {
    let key = match selector {
        RowSelector::Index(index) => SeriesKey::Index(index),
        RowSelector::Slice(spec) => SeriesKey::Slice(spec),
        RowSelector::Indices(indices) => SeriesKey::Indices(indices),
        RowSelector::Mask(mask) => SeriesKey::Mask(mask),
    };
    get_series_item_by_key(&series, key)
}

fn column_at_index(df: &DataFrame, index: usize) -> PolarsResult<Series> {
    let column = df.get_columns().get(index).ok_or_else(|| {
        PolarsError::ComputeError(format!("column index {index} is out of bounds").into())
    })?;
    column
        .as_series()
        .cloned()
        .ok_or_else(|| PolarsError::ComputeError("column is not a series".into()))
}

fn normalize_index(index: i64, len: usize) -> PolarsResult<usize> {
    let len_i64 = len as i64;
    let mut idx = index;
    if idx < 0 {
        idx += len_i64;
    }
    if idx < 0 || idx >= len_i64 {
        return Err(PolarsError::ComputeError(
            format!("index {index} is out of bounds for length {len}").into(),
        ));
    }
    Ok(idx as usize)
}

fn normalize_indices(indices: &[i64], len: usize) -> PolarsResult<Vec<u32>> {
    let mut normalized = Vec::with_capacity(indices.len());
    for index in indices {
        let index = normalize_index(*index, len)?;
        let value: u32 = index
            .try_into()
            .map_err(|_| PolarsError::ComputeError("index overflow".into()))?;
        normalized.push(value);
    }
    Ok(normalized)
}

fn slice_indices(width: usize, spec: SliceSpec) -> PolarsResult<Vec<usize>> {
    let width_i64 = width as i64;
    let step = spec.step.unwrap_or(1);
    if step == 0 {
        return Err(PolarsError::ComputeError(
            "slice step cannot be zero".into(),
        ));
    }

    let (start, stop) = if step > 0 {
        let mut start = spec.start.unwrap_or(0);
        let mut stop = spec.stop.unwrap_or(width_i64);

        if start < 0 {
            start += width_i64;
        }
        if start < 0 {
            start = 0;
        }
        if start > width_i64 {
            start = width_i64;
        }

        if stop < 0 {
            stop += width_i64;
        }
        if stop < 0 {
            stop = 0;
        }
        if stop > width_i64 {
            stop = width_i64;
        }

        (start, stop)
    } else {
        let mut start = spec.start.unwrap_or(width_i64 - 1);
        let mut stop = spec.stop.unwrap_or(-1);

        if start < 0 {
            start += width_i64;
        }
        if start < -1 {
            start = -1;
        }
        if start >= width_i64 {
            start = width_i64 - 1;
        }

        if stop < 0 {
            stop += width_i64;
        }
        if stop < -1 {
            stop = -1;
        }
        if stop >= width_i64 {
            stop = width_i64 - 1;
        }

        (start, stop)
    };

    let mut indices = Vec::new();
    let mut current = start;

    if step > 0 {
        while current < stop {
            if current >= 0 && current < width_i64 {
                indices.push(current as usize);
            }
            current += step;
        }
    } else {
        while current > stop {
            if current >= 0 && current < width_i64 {
                indices.push(current as usize);
            }
            current += step;
        }
    }

    if indices.is_empty() && (start == stop || width == 0) {
        return Ok(Vec::new());
    }

    Ok(indices)
}
