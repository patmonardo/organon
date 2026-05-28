//! Dataset frame module wiring.
//!
//! Keep this module root thin: activate and re-export the primary frame
//! implementation surface from `core`.

mod core;

pub use core::*;
