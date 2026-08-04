//! GraphFrame: Polars-backed graph table DSL.
//!
//! This module follows the RustScript 2×2 entrypoint matrix:
//! - `expr.rs` / `lazy.rs`
//! - `series.rs` / `frame.rs`
//!
//! Additional surfaces like `pgql/` live alongside the entrypoints.

pub mod expr;
pub mod feature_grammar;
pub mod frame;
pub mod functions;
pub mod lazy;
pub mod r#macro;
pub mod model;
// PGQL is work-in-progress and currently depends on crates not in the default build.
// Keep it behind an explicit feature gate so GraphFrame can compile cleanly.
#[cfg(feature = "graphframe_pgql")]
pub mod pgql;
pub mod plan;
pub mod plugins;
pub mod prelude;
pub mod rational_language;
pub mod series;

pub use expr::{
    GraphFeatureGrammarExpr, GraphFrameExpr, GraphModelExpr, GraphPlanExpr, GraphProcedureExpr,
    GraphViewExpr,
};
pub use feature_grammar::*;
pub use frame::{GraphFrame, GraphFrameError, SharedGraphStore};
pub use lazy::{GraphExecutionIntent, GraphFramePlan, GraphFramePureFormReciprocity};
pub use model::{GraphFrameModelExt, GraphModelNameSpace};
pub use plan::{GraphFramePlanExt, GraphPlanNameSpace};
pub use plugins::{
    GraphFeatureGrammarPlugin, GRAPH_FEATURE_GRAMMAR_LANGUAGE_ID, GRAPH_FEATURE_GRAMMAR_PLUGIN_ID,
};
pub use rational_language::{
    lower_graph_semantics, observe_graph_density, validate_graph_density_plan,
    GraphRationalLanguageError, GraphSemanticLowering,
};
pub use series::{GraphFrameSeriesNameSpace, SeriesGraphFrameExt};
