//! Extension namespace for GDSSeries (py-polars inspired).

use polars::prelude::{DataType, Series};

#[derive(Debug, Clone)]
pub struct ExtNameSpace {
    series: Series,
}

impl ExtNameSpace {
    pub fn new(series: Series) -> Self {
        Self { series }
    }

    pub fn series(&self) -> &Series {
        &self.series
    }

    pub fn into_series(self) -> Series {
        self.series
    }

    /// Create a Series with an extension `dtype` (seed pass: cast to dtype).
    pub fn to(&self, dtype: &DataType) -> polars::prelude::PolarsResult<Series> {
        self.series.cast(dtype)
    }

    /// Get the storage values of an extension dtype (seed pass: identity).
    pub fn storage(&self) -> Series {
        self.series.clone()
    }
}
