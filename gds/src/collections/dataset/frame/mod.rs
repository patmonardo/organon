//! Dataset frame module wiring.
//!
//! Keep this module root thin: activate the primary frame implementation
//! surface from `core`, and host the Dataset-facing expr/lazy/series/
//! streaming body modules under one home.

mod core;
pub mod expr;
pub mod lazy;
pub mod series;
pub mod streaming;

pub use core::*;
pub use expr::*;
pub use lazy::*;
pub use series::*;
pub use streaming::*;
