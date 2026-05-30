//! Feature module wiring.
//!
//! Keep `mod.rs` structural; implementation lives in `core.rs`.

pub mod core;
pub mod featstruct;
mod frames;
pub mod mediator;
pub mod role;
pub mod spec;

pub use core::*;
pub use frames::*;
pub use mediator::*;
pub use spec::*;
