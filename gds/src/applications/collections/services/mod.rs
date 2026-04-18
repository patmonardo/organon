//! Shared Dataset application services for the Collections facade.
//!
//! Populated in Batch 2:
//! - [`dataset_catalog_service`] — trait for per-(user, database) dataset
//!   catalog resolution.
//!
//! Planned for later batches:
//! - `dataset_compilation_service`
//! - `dataset_eval_service`
//! - `dataset_capability_service`

pub mod dataset_catalog_service;

pub use dataset_catalog_service::*;
