//! Dataset protocol descriptors.
//!
//! This module hosts declarative Dataset DSL descriptor types (IO, metadata,
//! projection, registry, reporting, and staged data-op descriptors). These are
//! protocol artifacts, not Expr namespace facades.

pub mod dataop;
pub mod io;
pub mod metadata;
pub mod projection;
pub mod registry;
pub mod reporting;
