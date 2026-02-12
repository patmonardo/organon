//! Dataset compiler utilities.
//!
//! These helpers support an LM-first dataset->DSL compilation workflow:
//! - compile an explicit IR graph
//! - build a catalog/index for iterative exploration
//! - emit deterministic Rust DSL module text

pub mod catalog_index;
pub mod codegen;
pub mod compile_ir;

pub use catalog_index::DatasetCatalogIndex;
pub use codegen::{render_rust_dsl_module, DslCodegenOptions};
pub use compile_ir::{DatasetCompilation, DatasetNode, DatasetNodeKind};
