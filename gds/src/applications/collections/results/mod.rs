//! Serde-facing response models for the Dataset-first Collections facade.
//!
//! Populated in Batch 2:
//! - [`dataset_catalog_results`]
//! - [`dataset_compilation_results`]
//!
//! Planned for later batches:
//! - `dataset_eval_results`
//! - `dataset_capability_results`

pub mod dataset_capability_results;
pub mod dataset_catalog_results;
pub mod dataset_compilation_results;
pub mod dataset_eval_results;

pub use dataset_capability_results::*;
pub use dataset_catalog_results::*;
pub use dataset_compilation_results::*;
pub use dataset_eval_results::*;
