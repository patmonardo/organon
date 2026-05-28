//! Dataset series module wiring.
//!
//! Keep this module root thin: activate and re-export the eager-series DSL
//! namespace surface from `core`.

mod core;

pub use core::*;
