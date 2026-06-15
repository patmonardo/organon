//! Unified task abstraction surface for Kernel <-> Agent reciprocity.
//!
//! This module provides a stable, top-level entry point that composes
//! task progress, concurrency, Pregel runtime, and task-memory concerns
//! without moving implementation ownership out of their existing modules.
//! Canonical naming is used throughout this surface: `memory` is the
//! authoritative namespace for task memory semantics.

pub mod concurrency;
pub mod memory;
pub mod pregel;
pub mod prelude;
pub mod progress;
pub mod runtime;
pub mod workbench;

#[allow(unused_imports)]
pub use pregel::*;
#[allow(unused_imports)]
pub use prelude::*;
#[allow(unused_imports)]
pub use progress::*;
#[allow(unused_imports)]
pub use runtime::*;
#[allow(unused_imports)]
pub use workbench::*;
