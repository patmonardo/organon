//! Shell Workbench.
//!
//! Operator-facing catalog and execution tracks for Shell as an agent-facing
//! subsystem.
//!
//! Examples remain the executable entrypoints; this module is the stable menu
//! and dispatch surface for Shell slices of interest.

pub mod catalog;

pub use catalog::*;
