//! Collections Backends: Backend Implementations
//!
//! This module provides backend implementations for Collections:
//! - Huge: Paged arrays for billions of elements (RAM)
//! - Vec: Simple wrappers around std::vec::Vec (RAM)
//! - Arrow: Columnar primitive arrays and zero-copy interchange
//! - Polars: DataFrame and lazy relational-plan execution
//!
//! The `factory` module provides helpers for creating backends from configuration.

pub mod arrow;
pub mod factory;
pub mod huge;
pub mod polars;
pub mod vec;

#[allow(ambiguous_glob_reexports)]
pub use arrow::*;
#[allow(ambiguous_glob_reexports)]
pub use factory::*;
#[allow(ambiguous_glob_reexports)]
pub use huge::*;
#[allow(ambiguous_glob_reexports)]
pub use polars::*;
#[allow(ambiguous_glob_reexports)]
pub use vec::*;
