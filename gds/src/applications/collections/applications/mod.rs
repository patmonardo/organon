//! Grouped application objects for the Dataset-first Collections facade.
//!
//! Populated in Batch 3:
//! - [`catalog_applications`] — list / register / load dataset artifacts.
//! - [`compilation_applications`] — compile GDSL source and materialize the
//!   resulting `DatasetCompilation`.
//!
//! Populated in Batch 4:
//! - [`feature_applications`] — evaluate features and emit attention reports
//!   over a cataloged dataset.
//! - [`introspection_applications`] — report backend / format / storage
//!   modifiers for a named catalog entry.

pub mod catalog_applications;
pub mod compilation_applications;
pub mod feature_applications;
pub mod introspection_applications;
