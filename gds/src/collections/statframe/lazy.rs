//! StatFrame LazyFrame facade.
//!
//! Placeholder: statframe is container-agnostic today, but we still provide the
//! prescribed `lazy.rs` entrypoint so stat transforms can compose with Polars
//! pipelines as the module grows.

use crate::collections::dataframe::GDSLazyFrame;

#[derive(Clone)]
pub struct StatFrameLazyFrameNameSpace {
    lf: GDSLazyFrame,
}

impl StatFrameLazyFrameNameSpace {
    pub fn new(lf: GDSLazyFrame) -> Self {
        Self { lf }
    }

    pub fn lazyframe(&self) -> &GDSLazyFrame {
        &self.lf
    }

    pub fn into_lazyframe(self) -> GDSLazyFrame {
        self.lf
    }
}

pub trait LazyFrameStatFrameExt {
    fn sf(self) -> StatFrameLazyFrameNameSpace;
}

impl LazyFrameStatFrameExt for GDSLazyFrame {
    fn sf(self) -> StatFrameLazyFrameNameSpace {
        StatFrameLazyFrameNameSpace::new(self)
    }
}

impl LazyFrameStatFrameExt for polars::prelude::LazyFrame {
    fn sf(self) -> StatFrameLazyFrameNameSpace {
        StatFrameLazyFrameNameSpace::new(GDSLazyFrame::new(self))
    }
}
