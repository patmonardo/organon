//! Dataset authoring DSL.
//!
//! Namespaces, helper functions, and macros live here.
//! These are authoring surfaces, not first-principles architecture.

pub mod expressions;
pub mod functions;
pub mod macros;
pub mod namespaces;

pub use namespaces::{
    is_dataset_namespace_registered, register_corpus_namespace, register_dataset_namespace,
    NameSpaceError,
};
