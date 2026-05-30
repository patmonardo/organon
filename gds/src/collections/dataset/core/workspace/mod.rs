//! Dataset lifecycle workspace helpers.
//!
//! This module provides a concrete platform primitive for dataset engineering:
//! initialize a semantic dataset folder, scaffold semantic-frame metadata,
//! build a schema catalog frame, stage archive intake, and ingest archives
//! into raw data folders.

mod catalog;
mod init;
mod inventory;
mod lifecycle;
mod types;

#[cfg(test)]
mod tests;

pub use types::*;
