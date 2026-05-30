//! Dataset DSL expression namespaces.
//!
//! These modules define the first Dataset expression landscape: small,
//! typed descriptors that sit above frames and below the later semantic
//! planning layers.
//!
//! Current layout:
//!
//! - `binary` — byte-oriented helpers that lower onto the DataFrame binary
//!   expression surface.
//! - `feature` — script-facing Feature expression data; structural Feature
//!   specifications live in `crate::collections::dataset::feature`.
//! - `model`, `plan` — speculative Component-facing expression wrappers for
//!   the dialectical middle.
//! - `parse`, `stem`, `tag`, `text`, `token`, `tree` — domain-specific
//!   expression families for language and tree authoring.
//!
//! The intent is to keep these expressions declarative: they describe the
//! shape of Dataset work, but they do not become the runtime by themselves.

pub mod binary;
pub mod feature;
pub mod model;
pub mod parse;
pub mod plan;
pub mod stem;
pub mod tag;
pub mod text;
pub mod token;
pub mod tree;
