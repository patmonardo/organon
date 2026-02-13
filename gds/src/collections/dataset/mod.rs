//! Collections datasets.
//!
//! High-level dataset management and Python-like expression helpers live here.
//! This is where we can map dataset families (e.g. pytorch-geometric) into
//! a consistent registry and reuse DataFrame expressions without adding
//! Rust-heavy call sites.
//!
//! Boundary note:
//! - This module exposes the Kernel-side dataset/tooling vocabulary and a
//!   top-level ToolChain facade for authoring.
//! - GUI workflow adaptation (GDSL -> TS-JSON -> React/Next MVC) is a
//!   Relative/TypeScript concern and remains outside this crate.
//! - Inference and semantics that belong to Logic:Model UserLand can consume
//!   this vocabulary without being executed in the GDS kernel runtime.
//! - SDSL is treated here as a specification language: model/feature artifacts
//!   and genus/species classification can drive dataset compilation IR, while
//!   DataFrame lowerings remain generated artifacts of that specification.
//!
//! Dragon Seed note:
//! - The top-level modules read like a compiler: catalog/registry, schema,
//!   plans, features, models, and the DSL namespaces that bind it together.

pub mod catalog;
pub mod catalog_index;
pub mod codegen;
pub mod compile_ir;
pub mod corpus;
pub mod dataset;
pub mod dependency;
pub mod error;
pub mod expr;
pub mod expressions;
pub mod featstruct;
pub mod feature;
pub mod frame;
pub mod functions;
pub mod io;
pub mod lazy;
pub mod r#macro;
pub mod metrics;
pub mod model;
pub mod namespace;
pub mod namespaces;
pub mod parse;
pub mod parser;
pub mod plan;
pub mod prelude;
pub mod registry;
pub mod schema;
pub mod series;
pub mod stdlib;
pub mod stem;
pub mod stemmer;
pub mod streaming;
pub mod tag;
pub mod tagger;
pub mod token;
pub mod tokenizer;
pub mod toolchain;
pub mod tree;
pub mod utils;

// Keep the module surface small and explicitly export the core public items.
pub use catalog::DatasetCatalog;
pub use dataset::Dataset;
pub use dependency::{DependencyEdge, DependencyGraph, DependencyNode};
pub use error::DatasetIoError;
pub use featstruct::{
    format_featstruct, parse_featstruct, parse_featvalue, subsumes_featstruct, unify_featstruct,
    FeatBindings, FeatDict, FeatList, FeatPath, FeatPathSegment, FeatReentranceId, FeatStruct,
    FeatStructParseError, FeatStructSet, FeatValue,
};
pub use feature::{Feature, FeatureSpace, FeatureView};
pub use feature::{
    FeatureCondition, FeatureExpr, FeatureExprNameSpace, FeatureNamespace, FeaturePath,
    FeaturePosition, FeatureRule, FeatureSeries, FeatureSeriesNameSpace, FeatureSpec,
    FeatureTemplate, FeatureValue,
};
pub use functions::model::lm::{
    AbsoluteDiscountingInterpolated, KneserNeyInterpolated, LanguageModel, Laplace, Lidstone,
    LmBase, LmError, NgramCounter, StupidBackoff, Vocabulary, WittenBellInterpolated, MLE,
};
pub use functions::model::preprocessing::{
    everygrams, ngrams, pad_both_ends, pad_both_ends_default, padded_everygram_pipeline,
    padded_everygrams,
};
pub use functions::scan_text_dir;
pub use metrics::{BinaryMetrics, MetricError};
pub use model::{
    Model, ModelAttributeUpdate, ModelContext, ModelDelta, ModelId, ModelKind, ModelReport,
    ModelResult, ModelScore, ModelSpace, ModelSpec, ModelState, ModelView, NoOpLanguageModel,
    NoOpParser, NoOpTagger,
};

// Dataset tabular DSL matrix exports.
pub use expr::{DatasetExprNameSpace, ExprDatasetExt};
pub use frame::{DataFrameDatasetExt, DatasetDataFrameNameSpace};
pub use lazy::{
    DatasetLazyFrameNameSpace, FeatureLazyFrameNameSpace, LazyFrameDatasetExt,
    TreeLazyFrameNameSpace,
};
pub use series::{DatasetSeriesNameSpace, SeriesDatasetExt};

pub use catalog_index::DatasetCatalogIndex;
pub use codegen::{render_rust_dsl_module, DslCodegenOptions};
pub use compile_ir::{DatasetCompilation, DatasetNode, DatasetNodeKind};
pub use expressions::dataop::{
    DataFrameLoweringArtifact, DatasetAspectArtifact, DatasetDataOp, DatasetDataOpExpr,
};
pub use expressions::io::{DatasetIoExpr, DatasetSource};
pub use expressions::metadata::DatasetMetadataExpr;
pub use expressions::projection::{DatasetProjectionExpr, DatasetProjectionKind};
pub use expressions::registry::DatasetRegistryExpr;
pub use expressions::reporting::{DatasetReportExpr, DatasetReportKind};
pub use namespaces::dataop::DataOpNs;
pub use namespaces::dataset::DatasetNs;
pub use namespaces::feature::{FeatureExprNameSpace as DatasetFeatureExprNameSpace, FeatureNs};
pub use namespaces::is_dataset_namespace_registered;
pub use namespaces::register_corpus_namespace;
pub use namespaces::register_dataset_namespace;
pub use namespaces::text::TextNs;
pub use namespaces::tree::TreeNs;
pub use parse::{Parse, ParseForest, ParseKind};
pub use parser::{BracketedParser, DependencyParser, FlatParser, JsonParser, MarkupParser, Parser};
pub use plan::{EvalMode as DatasetEvalMode, Plan as DatasetPlan, PlanEnv, PlanError};
pub use registry::{DatasetArtifact, DatasetMetadata, DatasetRegistry, DatasetSplit};
pub use schema::{FeatureSchema, ModelSchema, SymbolDef, SymbolTable};
pub use stdlib::{
    catalog_resource_tables, data_home, data_home_with, fetch_resource, list_resources,
    resource_dir, BracketedCorpusReader, ConcatenatedCorpusView, CorpusFiles, CorpusReader,
    CorpusResource, DatasetResource, DatasetResourceReport, PlaintextCorpusReader,
    StreamBackedCorpusView, WordListCorpusReader, XmlCorpusReader,
};
pub use stem::{Stem, StemKind};
pub use stemmer::{IdentityStemmer, LowercaseStemmer, SimpleSuffixStemmer, Stemmer};
pub use streaming::{StreamingBatchIter, StreamingDataset};
pub use tag::Tag;
pub use tagger::{DefaultTagger, LookupTagger, RegexTagger, Tagger, UnigramTagger};
pub use token::{Token, TokenKind, TokenSpan};
pub use tokenizer::{
    BlanklineTokenizer, CharTokenizer, JsonTokenizer, LineBlankMode, LineTokenizer,
    MarkupTokenizer, RegexpTokenizer, SExprTokenizer, SpaceTokenizer, StringSplitTokenizer,
    TabTokenizer, Tokenizer, WhitespaceTokenizer, WordPunctTokenizer,
};
pub use toolchain::{
    DatasetPipeline, DatasetPipelineArtifacts, DatasetToolChain, GenusSpecies, LogicalEngineIntent,
    ModelSpecRef, MvcEngineIntent, SdslSpecification,
};
pub use tree::{
    format_bracketed, format_pretty, parse_bracketed, MultiParentedIndex, MultiParentedNode,
    MultiParentedTree, MultiParentedValue, ParentedIndex, ParentedNode, ParentedTree,
    ParentedValue, ProbabilisticTree, TreeCollection, TreeExpr, TreeId, TreeIndex, TreeLeafExpr,
    TreeLeafValue, TreeNamespace, TreeNode, TreeOp, TreeParseError, TreePos, TreeSeries,
    TreeSeriesNameSpace, TreeSpan, TreeTraversal, TreeValue,
};
pub use utils::download::{
    copy_local, download_if_missing, download_to_dir, download_url, stream_to_writer,
    DownloadReport,
};
pub use utils::extract::{extract_archive, ExtractReport};

// Export specialized datasets (Corpus) as a convenience type.
pub use corpus::Corpus;
