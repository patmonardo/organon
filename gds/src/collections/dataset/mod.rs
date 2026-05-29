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
//!    `catalog`, `registry`, `io`) — compilation, resource access,
//!    namespace builders, and GDSL/SDSL authoring support.
//! 5. **Core substrate services** (`core::stdlib`) — resource download/cache,
//!    corpus readers, and low-level dataset utilities outside DSL protocol.
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
//! - The root should read like an organism, not like an implementation dump:
//!   `core`, `dsl`, domain folds, and a small `lab` for speculative machinery.

pub mod core;
pub mod corpus;
pub mod dsl;
pub mod feature;
pub mod frame;
pub mod lab;
pub mod language;
pub mod logic;
pub mod model;
pub mod plan;
pub mod prelude;

// =============================================================================
// Public surface
// =============================================================================
//
// Exports below use module-level glob re-exports for a uniform surface.

pub use core::*;
pub use corpus::*;
pub use dsl::namespaces::dataop::DataOpNs;
pub use dsl::namespaces::dataset::DatasetNs;
pub use dsl::namespaces::feature::{FeatureExprNameSpace, FeatureNs};
pub use dsl::namespaces::text::TextNs;
pub use dsl::namespaces::treens::TreeNs;
pub use dsl::namespaces::{
    is_dataset_namespace_registered, register_corpus_namespace, register_dataset_namespace,
    NameSpaceError,
};
pub use feature::*;
pub use frame::*;
pub use lab::compile::{
    ontology_image_from_program_features, render_rust_dsl_module, DatasetCompilation,
    DatasetCompilationArtifacts, DatasetCompilationIndex, DatasetNode, DatasetNodeKind,
    DslCodegenOptions, OntologyDataFrameImage, OntologyDataFrameImageTables,
    OntologyImageConstraintRow, OntologyImageFeatureRow, OntologyImageModelRow,
    OntologyImageProvenanceRow, OntologyImageQueryRow, OntologyRuntimeMode,
};
pub use lab::protocol::dataop::{
    DataFrameLoweringArtifact, DatasetAspectArtifact, DatasetDataOp, DatasetDataOpExpr,
};
pub use lab::protocol::io::{DatasetIoExpr, DatasetSource};
pub use lab::protocol::metadata::DatasetMetadataExpr;
pub use lab::protocol::projection::{DatasetProjectionExpr, DatasetProjectionKind};
pub use lab::protocol::registry::DatasetRegistryExpr;
pub use lab::protocol::reporting::{DatasetReportExpr, DatasetReportKind};
pub use lab::toolchain::{
    DatasetPipeline, DatasetPipelineArtifacts, DatasetToolChain, FeatureSpecRef,
    GdslSourceLoweringError, GenusSpecies, LogicalEngineIntent, ModelSpecRef, MvcEngineIntent,
    SdslSpecification,
};
pub use language::*;
pub use logic::*;
pub use model::*;
pub use plan::*;
