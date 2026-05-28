//! Dataset authoring DSL.
//!
//! Namespaces, helper functions, macros, and stdlib resources live here.
//! These are authoring surfaces, not first-principles architecture.

pub mod expressions;
pub mod functions;
pub mod macros;
pub mod namespaces;
pub mod stdlib;

pub use namespaces::{
    is_dataset_namespace_registered, register_corpus_namespace, register_dataset_namespace,
    NameSpaceError,
};
