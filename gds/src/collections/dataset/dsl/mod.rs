//! Dataset authoring DSL.
//!
//! This is the initial Dataset Component API: the small set of entry points
//! that make the layer navigable before the later Model / Feature / Plan
//! Components are involved.
//!
//! Initial layout:
//!
//! - `namespaces` — namespace-bearing Component surfaces for orchestration,
//!   expressions, text, tree, and the later semantic folds.
//! - `expressions` — typed expression value objects that describe Dataset-side
//!   authoring intent without executing it.
//! - `functions` — thin constructors that keep the DSL ergonomic and
//!   predictable.
//! - `macros` — syntactic helpers where the authoring surface benefits from a
//!   compact spelling.
//!
//! In the inner language of the dialectical cube, namespace entry points are
//! the exposure layer, while Components own the actual namespace-bearing
//! authority.
//!
//! The rule of thumb is simple: namespaces define where to look, expressions
//! define what can be described, and functions define how to build the pieces.

pub mod expressions;
pub mod functions;
pub mod macros;
pub mod namespaces;

pub use namespaces::expr::ExprNs;
pub use namespaces::feature::FeatureExprNs;
pub use namespaces::feature::FeatureNs;
pub use namespaces::model::ModelNs;
pub use namespaces::plan::PlanNs;
pub use namespaces::text::TextNs;
pub use namespaces::treens::TreeNs;
pub use namespaces::{
    is_dataset_namespace_registered, register_corpus_namespace, register_dataset_namespace,
    NameSpaceError,
};
