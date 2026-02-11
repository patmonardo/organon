//! Dataset DSL expression namespaces.
//!
//! These namespaces will mirror the DataFrame expression tree but remain
//! a dataset-level API surface layered above frames.

pub mod dataop;
pub mod feature;
pub mod io;
pub mod metadata;
pub mod model;
pub mod parse;
pub mod projection;
pub mod registry;
pub mod reporting;
pub mod stem;
pub mod tag;
pub mod text;
pub mod token;
pub mod tree;
