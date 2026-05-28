//! Dataset lazy module wiring.
//!
//! Keep this module root thin: activate and re-export the primary lazy
//! namespace surface from `core`.

mod core;

pub use core::*;
