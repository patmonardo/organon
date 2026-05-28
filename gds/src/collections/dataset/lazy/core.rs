//! Dataset DSL — `LazyFrame` namespace.
//!
//! Part of the dataset-side 2×2 matrix that mirrors the DataFrame DSL shell:
//!
//! |        | lazy        | eager        |
//! |--------|-------------|--------------|
//! | scalar | `Expr`      | `Series`     |
//! | frame  | `LazyFrame` | `DataFrame`  |
//!
//! This file provides [`DatasetLazyFrameNameSpace`], the lazy-frame entry
//! point, plus the [`LazyFrameDatasetExt`] trait that attaches `.ds()` onto
//! both `GDSLazyFrame` and Polars `LazyFrame`.
//!
//! From here you reach the dataset-flavored sub-namespaces
//! [`FeatureLazyFrameNameSpace`] and [`TreeLazyFrameNameSpace`], which are
//! the lazy-frame side of feature/tree pipelines.

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
