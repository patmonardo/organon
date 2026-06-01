//! Model module wiring.
//!
//! Keep `mod.rs` structural; implementation lives in `core.rs`.

pub mod core;
pub mod exec;
mod frames;
pub mod image;
mod mediator;
pub mod prep;

pub use core::*;
pub use exec::*;
pub use frames::*;
pub use image::*;
pub use mediator::*;
pub use prep::*;
