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
//! 1. **DataFrame shell** (`frame`, `series`, `expr`, plus `lazy`,
//!    `namespace`) — the Dataset-facing view of the Polars-shaped body.
//! 2. **Essence middle** (`model`, `feature`, `plan`) — semantic addresses,
//!    deferred Meta Plans, model preparation, execution, and ontology images.
//! 3. **Concept return** (`corpus`, `lm`, `sem`) — evidence, language,
//!    and semantic forms gathered into end-stage Dataset objects.
//! 4. **SDK services** (`toolchain`, `compile`, `expressions`, `functions`,
//!    `catalog`, `registry`, `io`, `stdlib`) — compilation, resource access,
//!    namespace builders, and GDSL/SDSL authoring support.
//!
//! Compatibility note: top-level modules such as `token`, `stemmer`,
//! `document`, `source`, `model_prep`, `featstruct`, and `semantic` are
//! intentionally kept as shim modules. The canonical homes are `lm::*` for
//! LanguageModel SubFeatures, `corpus::*` for Corpus SubFeatures,
//! `model::*`/`feature::*`/`plan::*` for the Essence fold, and `sem::*` for
//! the SemDataset return fold, but the old paths remain prelude-friendly while
//! the SDK namespace settles.
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
pub mod sem;
pub mod semantic;
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
// Exports below keep the nine-moment SDK readable while preserving old public
// paths. New implementation modules should prefer the canonical `corpus::*`,
// `lm::*`, and `sem::*` homes; top-level linguistic, evidentiary, and semantic
// modules are compatibility shims for existing callers and the curated prelude.

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
pub use sem::{SemDataset, SemError, SemForm};

pub use feature::featstruct::{
    format_featstruct, parse_featstruct, parse_featvalue, subsumes_featstruct, unify_featstruct,
    FeatBindings, FeatDict, FeatList, FeatPath, FeatPathSegment, FeatReentranceId, FeatStruct,
    FeatStructParseError, FeatStructSet, FeatValue,
};
pub use feature::{Feature, FeatureSpace, FeatureView};
pub use feature::{
    FeatureCondition, FeatureExpr, FeatureNamespace, FeaturePath, FeaturePosition, FeatureRule,
    FeatureSeries, FeatureSeriesNameSpace, FeatureSpec, FeatureTemplate, FeatureValue,
};

pub use model::exec::{
    execute_essence, execute_feature, execute_marked, ExecutedFeature, Execution, ExecutionAction,
};
pub use model::image::{realize_from_essence, realize_image, ImageOptions};
pub use model::prep::{
    prepare_model, FeatureMark, MarkRequirement, MarkedFeature, Modality, ModelEssence,
    ModelPrepExt, PreparationError, PreparationReport, PreparationStep,
};
pub use model::{
    Model, ModelAttributeUpdate, ModelContext, ModelDelta, ModelId, ModelKind, ModelReport,
    ModelResult, ModelScore, ModelSpace, ModelSpec, ModelState, ModelView, NoOpLanguageModel,
    NoOpParser, NoOpTagger,
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
