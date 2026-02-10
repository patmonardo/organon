//! StatFrame Series facade.
//!
//! Eager column entrypoint for statframe surfaces.

use crate::collections::dataframe::GDSSeries;

#[derive(Debug, Clone)]
pub struct StatFrameSeriesNameSpace {
    series: GDSSeries,
}

impl StatFrameSeriesNameSpace {
    pub fn new(series: GDSSeries) -> Self {
        Self { series }
    }

    pub fn series(&self) -> &GDSSeries {
        &self.series
    }

    pub fn into_series(self) -> GDSSeries {
        self.series
    }
}

pub trait SeriesStatFrameExt {
    fn sf(self) -> StatFrameSeriesNameSpace;
}

impl SeriesStatFrameExt for GDSSeries {
    fn sf(self) -> StatFrameSeriesNameSpace {
        StatFrameSeriesNameSpace::new(self)
    }
}
