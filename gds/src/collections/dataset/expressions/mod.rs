//! Dataset DSL expression namespaces.
//!
//! These namespaces will mirror the DataFrame expression tree but remain
//! a dataset-level API surface layered above frames.

pub mod text;

pub use text::{lowercase_expr, token_count_expr, tokenize_expr};
