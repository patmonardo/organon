//! Dataset-level namespace facade.
//!
//! Retired: the dataset-layer namespace glue now lives in `series.rs`.
//!
//! This module remains as a small compatibility shim to keep older imports
//! working while we converge on the dataset facade layout.

pub use crate::collections::dataset::lazy::{
    DatasetLazyFrameNameSpace, FeatureLazyFrameNameSpace, LazyFrameDatasetExt,
    TreeLazyFrameNameSpace,
};
