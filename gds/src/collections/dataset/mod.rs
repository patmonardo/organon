//! Collections datasets.
//!
//! The Dataset layer is the semantic-first SDK shell of Collections. It sits
//! above the runtime/tabular [`crate::collections::dataframe`] layer, keeps the
//! DataFrame/Polars body available, and recursively names that body through the
//! Dataset pipeline.
//!
//! The canonical pipeline has nine moments:
//!
//! ```text
//! Frame:Series::Expr -> Model:Feature::Plan -> Corpus:Language::Semantics
//! ```
//!
//! DataFrame remains the constrained runtime substrate. Dataset is the SDK that
//! mediates the substrate into models, features, plans, corpora, language
//! models, semantic forms, compilation artifacts, and GDSL/SDSL toolchains.
//!
//! Public surface (read this before adding new exports):
//!
//! 1. **DataFrame shell** (`frame`, `series`, `expr`, plus `lazy`) — the
//!    Dataset-facing view of the Polars-shaped body.
//! 2. **Essence middle** (`model`, `feature`, `plan`) — semantic addresses,
//!    deferred Meta Plans, model preparation, execution, and ontology images.
//! 3. **Concept return** (`corpus`, `lm`, `logic`) — evidence, language,
//!    and semantic forms gathered into end-stage Dataset objects.
//! 4. **SDK services** (`toolchain`, `compile`, `expressions`, `functions`,
//!    `catalog`, `registry`, `io`, `stdlib`) — compilation, resource access,
//!    namespace builders, and GDSL/SDSL authoring support.
//!
//! Canonical module homes: `language::*` for LanguageModel SubFeatures,
//! `corpus::*` for Corpus SubFeatures, `model::*`/`feature::*`/`plan::*` for
//! the Essence fold, and `logic::*` for the LogicFrame return fold.
//!
//! Boundary notes:
//! - GUI workflow adaptation (GDSL → TS-JSON → React/Next MVC) is a
//!   Relative/TypeScript concern and remains outside this crate.
//! - Inference and semantics that belong to Logic:Model UserLand can consume
//!   this vocabulary without being executed in the GDS kernel runtime.
//! - SDSL is treated here as a specification language: model/feature artifacts
//!   and genus/species classification drive dataset compilation IR, while
//!   DataFrame lowerings remain generated artifacts of that specification.
//! - The "entities" extracted from datasets are therefore primarily storable
//!   SDSL/GDSL plans and ontological/epistemological programs, rather than
//!   graph-analytic entities from a GraphFrame layer.
//!
//! Dragon Seed note:
//! - The top-level modules read like a compiler: catalog/registry, schema,
//!   plans, features, models, and the DSL namespaces that bind it together.

pub mod algebra;
pub mod artifact;
pub mod catalog;
pub mod codegen;
pub mod collocations;
pub mod compile;
pub mod corpus;
pub mod dataset;
pub mod error;
pub mod expr;
pub mod expressions;
pub mod feature;
pub mod frame;
pub mod functions;
pub mod grammar;
pub mod graph;
pub mod io;
pub mod language;
pub mod lazy;
pub mod logic;
pub mod macros;
pub mod metrics;
pub mod model;
pub mod namespaces;
pub mod plan;
pub mod prelude;
pub mod probability;
pub mod protocol;
pub mod registry;
pub mod schema;
pub mod series;
pub mod stdlib;
pub mod streaming;
pub mod text;
pub mod tgrep;
pub mod toolchain;
pub mod utils;
pub mod valuation;

// =============================================================================
// Public surface
// =============================================================================
//
// Exports below use module-level glob re-exports for a uniform surface.

pub use artifact::*;
pub use catalog::*;
pub use codegen::*;
pub use collocations::*;
pub use compile::*;
pub use corpus::*;
pub use dataset::*;
pub use error::*;
pub use expr::*;
pub use feature::*;
pub use frame::*;
pub use functions::*;
pub use grammar::*;
pub use graph::*;
pub use io::*;
pub use language::*;
pub use lazy::*;
pub use logic::*;
pub use metrics::*;
pub use model::*;
pub use namespaces::*;
pub use plan::*;
pub use probability::*;
pub use protocol::dataop::*;
pub use protocol::io::*;
pub use protocol::metadata::*;
pub use protocol::projection::*;
pub use protocol::registry::*;
pub use protocol::reporting::*;
pub use registry::*;
pub use schema::*;
pub use series::*;
pub use stdlib::*;
pub use streaming::*;
pub use text::*;
pub use tgrep::*;
pub use toolchain::*;
pub use utils::*;
pub use valuation::*;
