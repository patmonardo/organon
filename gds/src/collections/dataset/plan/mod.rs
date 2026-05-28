//! Dataset plan module wiring.
//!
//! Keep this module root thin: activate and re-export the primary plan
//! implementation surface from `core`.

mod core;

pub use core::*;
