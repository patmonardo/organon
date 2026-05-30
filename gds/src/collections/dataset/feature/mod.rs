//! Feature module wiring.
//!
//! Keep `mod.rs` structural; implementation lives in `core.rs`.

pub mod core;
pub mod featstruct;
pub mod role;
pub mod spec;

pub use core::*;
pub use spec::*;
