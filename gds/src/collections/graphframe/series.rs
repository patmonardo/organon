//! GraphFrame Series facade.
//!
//! Eager column entrypoint for graphframe surfaces.

use crate::collections::dataframe::GDSSeries;

#[derive(Debug, Clone)]
pub struct GraphFrameSeriesNameSpace {
    series: GDSSeries,
}

impl GraphFrameSeriesNameSpace {
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

pub trait SeriesGraphFrameExt {
    fn gf(self) -> GraphFrameSeriesNameSpace;
}

impl SeriesGraphFrameExt for GDSSeries {
    fn gf(self) -> GraphFrameSeriesNameSpace {
        GraphFrameSeriesNameSpace::new(self)
    }
}
