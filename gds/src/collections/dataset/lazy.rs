//! Dataset-level LazyFrame facade.
//!
//! This module is the dataset-layer ergonomic entrypoint for attaching
//! dataset-specific namespaces to lazy tabular pipelines.
//!
//! Matrix alignment:
//! - `Expr` is to `LazyFrame` as `Series` is to `DataFrame`.
//! - This file provides the dataset-side `LazyFrame` half of the `Expr`↔`LazyFrame` pair.

use crate::collections::dataframe::GDSLazyFrame;
use crate::collections::dataset::feature::Feature;

#[derive(Clone)]
pub struct DatasetLazyFrameNameSpace {
    lf: GDSLazyFrame,
}

impl DatasetLazyFrameNameSpace {
    pub fn new(lf: GDSLazyFrame) -> Self {
        Self { lf }
    }

    pub fn lazyframe(&self) -> &GDSLazyFrame {
        &self.lf
    }

    pub fn into_lazyframe(self) -> GDSLazyFrame {
        self.lf
    }

    pub fn feature(&self) -> FeatureLazyFrameNameSpace {
        FeatureLazyFrameNameSpace::new(self.lf.clone())
    }

    pub fn tree(&self) -> TreeLazyFrameNameSpace {
        TreeLazyFrameNameSpace::new(self.lf.clone())
    }
}

#[derive(Clone)]
pub struct FeatureLazyFrameNameSpace {
    lf: GDSLazyFrame,
}

impl FeatureLazyFrameNameSpace {
    pub fn new(lf: GDSLazyFrame) -> Self {
        Self { lf }
    }

    pub fn lazyframe(&self) -> &GDSLazyFrame {
        &self.lf
    }

    pub fn into_lazyframe(self) -> GDSLazyFrame {
        self.lf
    }

    pub fn apply(&self, feature: &Feature) -> GDSLazyFrame {
        let lf = feature.apply_to_lazyframe(self.lf.clone().into_lazyframe());
        GDSLazyFrame::new(lf)
    }
}

#[derive(Clone)]
pub struct TreeLazyFrameNameSpace {
    lf: GDSLazyFrame,
}

impl TreeLazyFrameNameSpace {
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

pub trait LazyFrameDatasetExt {
    fn ds(self) -> DatasetLazyFrameNameSpace;
}

impl LazyFrameDatasetExt for GDSLazyFrame {
    fn ds(self) -> DatasetLazyFrameNameSpace {
        DatasetLazyFrameNameSpace::new(self)
    }
}

impl LazyFrameDatasetExt for polars::prelude::LazyFrame {
    fn ds(self) -> DatasetLazyFrameNameSpace {
        DatasetLazyFrameNameSpace::new(GDSLazyFrame::new(self))
    }
}
