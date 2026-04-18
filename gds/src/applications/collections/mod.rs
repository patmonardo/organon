//! Dataset-first Applications facade for the Collections domain.
//!
//! This module is the internal surface behind the TS-JSON `facade = "collections"`
//! branch. Its first concrete public contract is explicitly dataset-centered:
//! it exposes catalog, compilation, materialization, evaluation, and capability
//! operations over the existing Dataset toolchain, following the
//! `graph_store_catalog` application/service layout rather than the algorithms
//! procedure-facade pattern.
//!
//! Scaffold only in Batch 1: subgroups are empty placeholders that will be
//! filled in as the vertical slice (catalog -> compilation -> evaluation) lands.

pub mod applications;
pub mod configs;
pub mod loaders;
pub mod results;
pub mod services;

pub use applications::*;
