//! Catalog-root and service-initialization helpers for the Dataset-first
//! Collections facade.
//!
//! Populated in Batch 2:
//! - [`dataset_catalog_loader`] — per-(user, database) disk-backed catalog
//!   resolution.
//! - [`collections_service_loader`] — process-wide singleton.

pub mod collections_service_loader;
pub mod dataset_catalog_loader;

pub use collections_service_loader::*;
pub use dataset_catalog_loader::*;
