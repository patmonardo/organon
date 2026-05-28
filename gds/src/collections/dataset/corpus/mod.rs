//! Corpus module wiring.
//!
//! Keep this module root thin: activate corpus subfeatures and re-export the
//! public corpus surface from `core`.

pub mod annotation;
pub mod core;
pub mod document;
pub mod source;

pub use annotation::*;
pub use core::*;
pub use document::*;
pub use source::*;
