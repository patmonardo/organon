//! Dataset-level namespaces facade.
//!
//! Minimal LazyFrame namespace glue for dataset surfaces.
use crate::collections::dataset::feature::Feature;
use polars::prelude::LazyFrame;

#[derive(Clone)]
pub struct DatasetLazyFrameNameSpace {
    lf: LazyFrame,
}

impl DatasetLazyFrameNameSpace {
    pub fn new(lf: LazyFrame) -> Self {
        Self { lf }
    }

    pub fn lazyframe(&self) -> &LazyFrame {
        &self.lf
    }

    pub fn into_lazyframe(self) -> LazyFrame {
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
    lf: LazyFrame,
}

impl FeatureLazyFrameNameSpace {
    pub fn new(lf: LazyFrame) -> Self {
        Self { lf }
    }

    pub fn lazyframe(&self) -> &LazyFrame {
        &self.lf
    }

    pub fn into_lazyframe(self) -> LazyFrame {
        self.lf
    }

    pub fn apply(&self, feature: &Feature) -> LazyFrame {
        feature.apply_to_lazyframe(self.lf.clone())
    }
}

#[derive(Clone)]
pub struct TreeLazyFrameNameSpace {
    lf: LazyFrame,
}

impl TreeLazyFrameNameSpace {
    pub fn new(lf: LazyFrame) -> Self {
        Self { lf }
    }

    pub fn lazyframe(&self) -> &LazyFrame {
        &self.lf
    }

    pub fn into_lazyframe(self) -> LazyFrame {
        self.lf
    }
}

pub trait LazyFrameDatasetExt {
    fn ds(self) -> DatasetLazyFrameNameSpace;
}

impl LazyFrameDatasetExt for LazyFrame {
    fn ds(self) -> DatasetLazyFrameNameSpace {
        DatasetLazyFrameNameSpace::new(self)
    }
}
