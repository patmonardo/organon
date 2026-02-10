//! GraphFrame LazyFrame facade.
//!
//! This module is the graphframe-side entrypoint for attaching graph-specific
//! namespaces to lazy tabular pipelines.

use crate::collections::dataframe::GDSLazyFrame;

#[derive(Clone)]
pub struct GraphFrameLazyFrameNameSpace {
    lf: GDSLazyFrame,
}

impl GraphFrameLazyFrameNameSpace {
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

pub trait LazyFrameGraphFrameExt {
    fn gf(self) -> GraphFrameLazyFrameNameSpace;
}

impl LazyFrameGraphFrameExt for GDSLazyFrame {
    fn gf(self) -> GraphFrameLazyFrameNameSpace {
        GraphFrameLazyFrameNameSpace::new(self)
    }
}

impl LazyFrameGraphFrameExt for polars::prelude::LazyFrame {
    fn gf(self) -> GraphFrameLazyFrameNameSpace {
        GraphFrameLazyFrameNameSpace::new(GDSLazyFrame::new(self))
    }
}
