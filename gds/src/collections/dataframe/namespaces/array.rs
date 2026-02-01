//! Array namespace for SeriesModel (py-polars inspired).

use polars::prelude::Series;

#[derive(Debug, Clone)]
pub struct ArrayNameSpace {
    series: Series,
}

impl ArrayNameSpace {
    pub fn new(series: Series) -> Self {
        Self { series }
    }

    pub fn series(&self) -> &Series {
        &self.series
    }

    pub fn into_series(self) -> Series {
        self.series
    }
}
