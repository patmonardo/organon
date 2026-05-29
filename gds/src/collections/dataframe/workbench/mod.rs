//! DataFrame Workbench.
//!
//! Operator-facing catalog and execution tracks for the DataFrame collections
//! layer.
//!
//! Examples remain the readable entrypoints; this module is the stable menu and
//! dispatch surface for people who already know the system.

pub mod catalog;

pub use catalog::*;
