//! Collections datasets.
//!
//! The Dataset layer is the semantic-first DSL shell of the Collections SDK.
//! It sits alongside the runtime/tabular [`crate::collections::dataframe`]
//! layer and shares its 2×2 (`Expr`/`LazyFrame`/`DataFrame`/`Series`) shape,
//! while owning a distinct vocabulary of plans, features, models, and
//! toolchains.
//!
//! Public surface (read this before adding new exports):
//!
//! 1. **DSL shell** (`expr`, `frame`, `lazy`, `series`, `namespace`) — the
//!    dataset-flavored 2×2 matrix and the dataset namespace registry. This is
//!    the lightweight, Polars-shaped entry point. The single most useful
//!    starting line is `use crate::collections::dataset::namespace::*;`.
//! 2. **Semantic / compiler surfaces** (`dataset`, `plan`, `feature`,
//!    `featstruct`, `model`, `schema`, `toolchain`, `compile`,
//!    `expressions`) — the kernel-side dataset/tooling vocabulary, including
//!    the top-level ToolChain façade for authoring SDSL/GDSL programs.
//! 3. **Catalog, IO, and stdlib** (`catalog`, `corpus`, `io`,
//!    `registry`, `stdlib`, `streaming`, `utils`) — discovery, persistence,
//!    and resource access for datasets.
//! 4. **Linguistic stdlib** (`token`, `tokenizer`, `parse`, `parser`, `tag`,
//!    `tagger`, `stem`, `stemmer`, `tree`, `functions::model`) —
//!    NLTK-flavored building blocks used by the Dataset DSL.
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

pub mod annotation;
pub mod artifact;
pub mod catalog;
pub mod codegen;
pub mod collocations;
pub mod compile;
pub mod corpus;
pub mod dataset;
pub mod document;
pub mod error;
pub mod expr;
pub mod expressions;
pub mod featstruct;
pub mod feature;
pub mod feature_role;
pub mod frame;
pub mod functions;
pub mod grammar;
pub mod graph;
pub mod io;
pub mod lazy;
pub mod lm;
pub mod r#macro;
pub mod metrics;
pub mod model;
pub mod model_exec;
pub mod model_image;
pub mod model_prep;
pub mod namespace;
pub mod namespaces;
pub mod parse;
pub mod parser;
pub mod plan;
pub mod prelude;
pub mod probability;
pub mod registry;
pub mod schema;
pub mod series;
pub mod source;
pub mod stdlib;
pub mod stem;
pub mod stemmer;
pub mod streaming;
pub mod tag;
pub mod tagger;
pub mod text;
pub mod tgrep;
pub mod token;
pub mod tokenizer;
pub mod toolchain;
pub mod tree;
pub mod utils;
pub mod valuation;

// =============================================================================
// Public surface
// =============================================================================
//
// Exports below are grouped to match the four sections described in the module
// header: (1) DSL shell, (2) semantic/compiler surfaces, (3) catalog/IO/stdlib,
// (4) linguistic stdlib. Keep new exports in the matching section; do not
// re-export deeper builder namespaces from `namespaces::*` here unless they
// belong to the top-level shell.

// -----------------------------------------------------------------------------
// (1) DSL shell — dataset-side 2×2 matrix and namespace registry.
// -----------------------------------------------------------------------------

pub use expr::{DatasetExprNameSpace, ExprDatasetExt};
pub use frame::{DataFrameDatasetExt, DatasetDataFrameNameSpace};
pub use lazy::{
    DatasetLazyFrameNameSpace, FeatureLazyFrameNameSpace, LazyFrameDatasetExt,
    TreeLazyFrameNameSpace,
};
pub use series::{DatasetSeriesNameSpace, SeriesDatasetExt};

// Dataset-side namespace registry (distinct from the DataFrame registry).
pub use namespaces::{
    is_dataset_namespace_registered, register_corpus_namespace, register_dataset_namespace,
};

// Built-in dataset namespace builders surfaced at the top level.
pub use namespaces::dataop::DataOpNs;
pub use namespaces::dataset::DatasetNs;
pub use namespaces::feature::{FeatureExprNameSpace, FeatureNs};
pub use namespaces::text::TextNs;
pub use namespaces::tree::TreeNs;

// -----------------------------------------------------------------------------
// (2) Semantic / compiler surfaces — Dataset, Plan, Feature, Model, ToolChain,
//     compile IR, and the dataset expression algebra.
// -----------------------------------------------------------------------------

pub use dataset::Dataset;
pub use plan::{EvalMode as DatasetEvalMode, Plan as DatasetPlan, PlanEnv, PlanError};

pub use featstruct::{
    format_featstruct, parse_featstruct, parse_featvalue, subsumes_featstruct, unify_featstruct,
    FeatBindings, FeatDict, FeatList, FeatPath, FeatPathSegment, FeatReentranceId, FeatStruct,
    FeatStructParseError, FeatStructSet, FeatValue,
};
pub use feature::{Feature, FeatureSpace, FeatureView};
pub use feature::{
    FeatureCondition, FeatureExpr, FeatureNamespace, FeaturePath, FeaturePosition, FeatureRule,
    FeatureSeries, FeatureSeriesNameSpace, FeatureSpec, FeatureTemplate, FeatureValue,
};

pub use model::{
    Model, ModelAttributeUpdate, ModelContext, ModelDelta, ModelId, ModelKind, ModelReport,
    ModelResult, ModelScore, ModelSpace, ModelSpec, ModelState, ModelView, NoOpLanguageModel,
    NoOpParser, NoOpTagger,
};
pub use model_exec::{
    execute_essence, execute_feature, execute_marked, ExecutedFeature, Execution, ExecutionAction,
};
pub use model_image::{realize_from_essence, realize_image, ImageOptions};
pub use model_prep::{
    prepare_model, FeatureMark, MarkRequirement, MarkedFeature, Modality, ModelEssence,
    ModelPrepExt, PreparationError, PreparationReport, PreparationStep,
};
pub use schema::{FeatureSchema, ModelSchema, SymbolDef, SymbolTable};

pub use lm::*;

pub use toolchain::{
    DatasetPipeline, DatasetPipelineArtifacts, DatasetToolChain, GdslSourceLoweringError,
    GenusSpecies, LogicalEngineIntent, ModelSpecRef, MvcEngineIntent, SdslSpecification,
};

pub use codegen::{render_rust_dsl_module, DslCodegenOptions};
pub use compile::{
    ontology_image_from_program_features, DatasetCompilation, DatasetCompilationArtifacts,
    DatasetNode, DatasetNodeKind, OntologyDataFrameImage, OntologyDataFrameImageTables,
    OntologyImageConstraintRow, OntologyImageFeatureRow, OntologyImageModelRow,
    OntologyImageProvenanceRow, OntologyImageQueryRow, OntologyRuntimeMode,
};

pub use expressions::dataop::{
    DataFrameLoweringArtifact, DatasetAspectArtifact, DatasetDataOp, DatasetDataOpExpr,
};
pub use expressions::io::{DatasetIoExpr, DatasetSource};
pub use expressions::metadata::DatasetMetadataExpr;
pub use expressions::projection::{DatasetProjectionExpr, DatasetProjectionKind};
pub use expressions::registry::DatasetRegistryExpr;
pub use expressions::reporting::{DatasetReportExpr, DatasetReportKind};

pub use functions::{
    dataop_decode, dataop_encode, dataop_input, dataop_output, dataop_transform, io_path, io_url,
    metadata, pipeline, project_corpus, project_graph, project_text, registry, registry_versioned,
    report_profile, report_summary, scan_text_dir, text_decode, text_encode, text_input,
    text_lifecycle, text_output, text_transform,
};

pub use metrics::{BinaryMetrics, MetricError};

// -----------------------------------------------------------------------------
// (3) Catalog, IO, registry, and stdlib resources.
// -----------------------------------------------------------------------------

pub use artifact::{DatasetArtifactKind, DatasetArtifactProfile};
pub use catalog::{DatasetCatalog, DatasetCatalogIndex};
pub use corpus::{AnnotatorEffort, Corpus, CorpusError};
pub use error::DatasetIoError;
pub use registry::{DatasetArtifact, DatasetMetadata, DatasetRegistry, DatasetSplit};
pub use streaming::{StreamingBatchIter, StreamingDataset};

pub use stdlib::{
    catalog_resource_tables, data_home, data_home_with, fetch_resource, list_resources,
    resource_dir, BracketedCorpusReader, ConcatenatedCorpusView, CorpusFiles, CorpusReader,
    CorpusResource, DatasetResource, DatasetResourceReport, PlaintextCorpusReader,
    StreamBackedCorpusView, WordListCorpusReader, XmlCorpusReader,
};
pub use utils::download::{
    copy_local, download_if_missing, download_to_dir, download_url, stream_to_writer,
    DownloadReport,
};
pub use utils::extract::{extract_archive, ExtractReport};

// -----------------------------------------------------------------------------
// (4) Linguistic stdlib — NLTK-flavored building blocks (parse, tag, stem,
//     tokenize, tree, language models).
// -----------------------------------------------------------------------------

pub use parse::{Parse, ParseForest, ParseKind};
pub use parser::{BracketedParser, DependencyParser, FlatParser, JsonParser, MarkupParser, Parser};
pub use stem::{Stem, StemKind};
pub use stemmer::{IdentityStemmer, LowercaseStemmer, SimpleSuffixStemmer, Stemmer};
pub use tag::{str2tuple, tuple2str, untag, Tag};
pub use tagger::{DefaultTagger, LookupTagger, RegexTagger, Tagger, UnigramTagger};
pub use token::{Token, TokenKind, TokenSpan};
pub use tokenizer::{
    align_token_texts, align_tokens, blankline_tokenize, line_tokenize, regexp_span_tokenize,
    regexp_tokenize, spans_to_relative, string_span_tokenize, wordpunct_tokenize,
    BlanklineTokenizer, CharTokenizer, JsonTokenizer, LineBlankMode, LineTokenizer, MWETokenizer,
    MarkupTokenizer, RegexpTokenizer, SExprTokenizer, SpaceTokenizer, StringSplitTokenizer,
    TabTokenizer, Tokenizer, WhitespaceTokenizer, WordPunctTokenizer,
};
pub use tree::{
    format_bracketed, format_pretty, parse_bracketed, MultiParentedIndex, MultiParentedNode,
    MultiParentedTree, MultiParentedValue, ParentedIndex, ParentedNode, ParentedTree,
    ParentedValue, ProbabilisticTree, TreeCollection, TreeExpr, TreeId, TreeIndex, TreeLeafExpr,
    TreeLeafValue, TreeNamespace, TreeNode, TreeOp, TreeParseError, TreePos, TreeSeries,
    TreeSeriesNameSpace, TreeSpan, TreeTraversal, TreeValue,
};

pub use functions::model::preprocessing::{
    everygrams, ngrams, pad_both_ends, pad_both_ends_default, padded_everygram_pipeline,
    padded_everygrams,
};
