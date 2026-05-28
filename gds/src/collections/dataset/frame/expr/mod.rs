//! Dataset expr module wiring.
//!
//! Keep this module root thin: activate and re-export the primary expr
//! implementation surface from `core`.

mod core;

pub use core::*;
