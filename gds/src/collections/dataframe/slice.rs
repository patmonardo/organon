//! Slice helpers for DataFrame/Series (py-polars inspired).

use polars::error::PolarsError;
use polars::lazy::frame::LazyFrame;
use polars::prelude::{Column, DataFrame, Expr, IdxCa, NamedFrom, PolarsResult, Series};

/// Python-like slice specification (start/stop/step).
#[derive(Debug, Clone, Copy, Default)]
pub struct SliceSpec {
    pub start: Option<i64>,
    pub stop: Option<i64>,
    pub step: Option<i64>,
}

impl SliceSpec {
    pub fn new(start: Option<i64>, stop: Option<i64>, step: Option<i64>) -> Self {
        Self { start, stop, step }
    }
}

/// Apply a python-like slice to a DataFrame.
pub fn slice_dataframe(df: &DataFrame, spec: SliceSpec) -> PolarsResult<DataFrame> {
    let helper = SliceHelper::new(df.height(), spec)?;

    if helper.slice_length == 0 {
        return Ok(df.slice(0, 0));
    }

    if helper.is_unbounded && (helper.stride == 1 || helper.stride == -1) {
        return if helper.stride == -1 {
            Ok(df.clone().reverse())
        } else {
            Ok(df.clone())
        };
    }

    if helper.start >= 0 && helper.stop >= 0 && helper.stride == 1 {
        return Ok(df.slice(helper.start, (helper.stop - helper.start) as usize));
    }

    if helper.stride < 0 && helper.slice_length == 1 {
        return Ok(df.slice(helper.start, 1));
    }

    let indices = build_indices(&helper)?;
    df.take(&indices)
}

/// Apply a python-like slice to a Series.
pub fn slice_series(series: &Series, spec: SliceSpec) -> PolarsResult<Series> {
    let df = DataFrame::new(vec![Column::from(series.clone())])?;
    let sliced = slice_dataframe(&df, spec)?;
    let column = sliced
        .column(series.name())
        .map_err(|_| PolarsError::ComputeError("series column missing after slice".into()))?;
    column
        .as_series()
        .ok_or_else(|| PolarsError::ComputeError("column is not a series".into()))
        .map(|s| s.clone())
}

/// Apply a python-like slice to a LazyFrame (restricted).
pub fn slice_lazyframe(lf: &LazyFrame, spec: SliceSpec) -> PolarsResult<LazyFrame> {
    let start = spec.start.unwrap_or(0);
    let step = spec.step.unwrap_or(1);

    if step == 0 {
        return Err(PolarsError::ComputeError(
            "slice step cannot be zero".into(),
        ));
    }

    if let Some(stop) = spec.stop {
        if stop < 0 {
            return Err(PolarsError::ComputeError(
                "lazy slice with negative stop is not supported".into(),
            ));
        }
    }

    if step < 0 && (start > 0 || spec.stop.is_some()) && spec.stop != Some(start) {
        return Err(PolarsError::ComputeError(
            "lazy slice with negative step requires bounded start/stop".into(),
        ));
    }

    let stop = spec.stop;

    if step > 0 {
        if let Some(stop) = stop {
            if start >= stop {
                return Ok(lf.clone().slice(0, 0));
            }
            if start == 0 {
                return Ok(lf.clone().slice(0, to_idx_size(stop as usize)?));
            }
            let length = (stop - start) as usize;
            return Ok(lf.clone().slice(start, to_idx_size(length)?));
        }
        return Ok(lf.clone().slice(start, u32::MAX));
    }

    // step < 0
    if stop.is_none() && start == 0 {
        return Ok(lf.clone().reverse());
    }

    if let Some(stop) = stop {
        if start == stop {
            return Ok(lf.clone().slice(0, 0));
        }
        if start > stop {
            let length = (start - stop) as usize;
            let lazy = lf.clone().slice(stop + 1, to_idx_size(length)?).reverse();
            return Ok(lazy);
        }
    }

    Err(PolarsError::ComputeError(
        "unsupported lazy slice pattern".into(),
    ))
}

#[derive(Debug, Clone, Copy)]
struct SliceHelper {
    start: i64,
    stop: i64,
    stride: i64,
    slice_length: usize,
    is_unbounded: bool,
}

impl SliceHelper {
    fn new(len: usize, spec: SliceSpec) -> PolarsResult<Self> {
        let len_i64 = len as i64;
        let stride = spec.step.unwrap_or(1);

        if stride == 0 {
            return Err(PolarsError::ComputeError(
                "slice step cannot be zero".into(),
            ));
        }

        let is_unbounded = if stride > 0 {
            spec.stop.is_none()
        } else {
            spec.start.is_none()
        };

        if len == 0 {
            return Ok(Self {
                start: 0,
                stop: 0,
                stride,
                slice_length: 0,
                is_unbounded,
            });
        }

        let (mut start, mut stop) = slice_indices(len_i64, spec.start, spec.stop, stride);

        let slice_length = if is_unbounded {
            len
        } else if stride > 0 {
            if stop <= start {
                0
            } else {
                ((stop - start - 1) / stride + 1) as usize
            }
        } else if stop >= start {
            0
        } else {
            ((start - stop - 1) / (-stride) + 1) as usize
        };

        if slice_length == 0 {
            start = 0;
            stop = 0;
        }

        Ok(Self {
            start,
            stop,
            stride,
            slice_length,
            is_unbounded,
        })
    }
}

fn slice_indices(len: i64, start: Option<i64>, stop: Option<i64>, step: i64) -> (i64, i64) {
    if len == 0 {
        return (0, 0);
    }

    if step > 0 {
        let mut start = start.unwrap_or(0);
        let mut stop = stop.unwrap_or(len);

        if start < 0 {
            start += len;
        }
        if start < 0 {
            start = 0;
        }
        if start > len {
            start = len;
        }

        if stop < 0 {
            stop += len;
        }
        if stop < 0 {
            stop = 0;
        }
        if stop > len {
            stop = len;
        }

        (start, stop)
    } else {
        let mut start = start.unwrap_or(len - 1);
        let mut stop = stop.unwrap_or(-1);

        if start < 0 {
            start += len;
        }
        if start < -1 {
            start = -1;
        }
        if start >= len {
            start = len - 1;
        }

        if stop < 0 {
            stop += len;
        }
        if stop < -1 {
            stop = -1;
        }
        if stop >= len {
            stop = len - 1;
        }

        (start, stop)
    }
}

/// Utility to build a literal equality predicate (used by higher-level slice logic).
#[allow(dead_code)]
pub fn eq_lit(name: &str, value: Expr) -> Expr {
    polars::prelude::col(name).eq(value)
}

fn build_indices(helper: &SliceHelper) -> PolarsResult<IdxCa> {
    let mut indices = Vec::with_capacity(helper.slice_length);
    let mut current = helper.start;
    for _ in 0..helper.slice_length {
        if current < 0 {
            return Err(PolarsError::ComputeError(
                "slice index computed as negative".into(),
            ));
        }
        let value: u32 = current
            .try_into()
            .map_err(|_| PolarsError::ComputeError("slice index overflow".into()))?;
        indices.push(value);
        current += helper.stride;
    }
    Ok(IdxCa::new("idx".into(), indices))
}

fn to_idx_size(value: usize) -> PolarsResult<u32> {
    value
        .try_into()
        .map_err(|_| PolarsError::ComputeError("slice length overflow".into()))
}
