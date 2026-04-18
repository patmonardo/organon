//! Serde-facing request/config models for the Dataset-first Collections facade.
//!
//! Populated in Batch 2:
//! - [`dataset_catalog_config`]
//! - [`dataset_compilation_config`]
//!
//! Planned for later batches:
//! - `dataset_eval_config`
//! - `dataset_execution_config` (optional backend/capability envelope)

pub mod dataset_catalog_config;
pub mod dataset_compilation_config;
pub mod dataset_eval_config;

pub use dataset_catalog_config::*;
pub use dataset_compilation_config::*;
pub use dataset_eval_config::*;
