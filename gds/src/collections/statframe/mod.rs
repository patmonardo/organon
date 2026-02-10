//! StatFrame: statistical ML surface for dataset-style workflows.
//!
//! This module mirrors the Dataset NS/Expr/Functions pattern for
//! statistical modeling and feature semantics. See README.md for
//! the current design notes.

pub mod expr;
pub mod frame;
pub mod lazy;
pub mod r#macro;
pub mod prelude;
pub mod series;

pub mod expressions;
pub mod functions;

pub use expr::{ExprStatFrameExt, StatFrameExprNameSpace};
pub use frame::{DataFrameStatFrameExt, StatFrameDataFrameNameSpace};
pub use lazy::{LazyFrameStatFrameExt, StatFrameLazyFrameNameSpace};
pub use series::{SeriesStatFrameExt, StatFrameSeriesNameSpace};
