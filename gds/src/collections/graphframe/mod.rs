//! GraphFrame: Polars-backed graph table DSL.
//!
//! This module follows the RustScript 2×2 entrypoint matrix:
//! - `expr.rs` / `lazy.rs`
//! - `series.rs` / `frame.rs`
//!
//! Additional surfaces like `pgql/` live alongside the entrypoints.

pub mod expr;
pub mod frame;
pub mod functions;
pub mod lazy;
pub mod r#macro;
// PGQL is work-in-progress and currently depends on crates not in the default build.
// Keep it behind an explicit feature gate so GraphFrame can compile cleanly.
#[cfg(feature = "graphframe_pgql")]
pub mod pgql;
pub mod prelude;
pub mod series;

pub use expr::{ExprGraphFrameExt, GraphFrameExprNameSpace};
pub use frame::{DataFrameGraphFrameExt, GraphFrameDataFrameNameSpace};
pub use lazy::{GraphFrameLazyFrameNameSpace, LazyFrameGraphFrameExt};
pub use series::{GraphFrameSeriesNameSpace, SeriesGraphFrameExt};
