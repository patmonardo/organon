//! Collections datasets.
//!
//! High-level dataset management and Python-like expression helpers live here.
//! This is where we can map dataset families (e.g. pytorch-geometric) into
//! a consistent registry and reuse DataFrame expressions without adding
//! Rust-heavy call sites.

pub mod catalog;
pub mod corpus;
pub mod dataset;
pub mod download;
pub mod error;
pub mod expr;
pub mod expressions;
pub mod extract;
pub mod featstruct;
pub mod feature;
pub mod frame;
pub mod functions;
pub mod io;
pub mod lazy;
pub mod r#macro;
pub mod model;
pub mod namespace;
pub mod namespaces;
pub mod parse;
pub mod plan;
pub mod prelude;
pub mod registry;
pub mod schema;
pub mod series;
pub mod stem;
pub mod streaming;
pub mod tag;
pub mod token;
pub mod tree;

// Keep the module surface small and explicitly export the core public items.
pub use catalog::DatasetCatalog;
pub use dataset::Dataset;
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
pub use model::{
    Model, ModelAttributeUpdate, ModelContext, ModelDelta, ModelId, ModelKind, ModelReport,
    ModelResult, ModelScore, ModelSpec, ModelState, ModelView, NoOpLanguageModel, NoOpParser,
    NoOpTagger,
};

// Dataset tabular DSL matrix exports.
pub use expr::{DatasetExprNameSpace, ExprDatasetExt};
pub use frame::{DataFrameDatasetExt, DatasetDataFrameNameSpace};
pub use lazy::{
    DatasetLazyFrameNameSpace, FeatureLazyFrameNameSpace, LazyFrameDatasetExt,
    TreeLazyFrameNameSpace,
};
pub use series::{DatasetSeriesNameSpace, SeriesDatasetExt};

pub use namespaces::dataset::DatasetNs;
pub use parse::{Parse, ParseForest, ParseKind};
pub use plan::{EvalMode as DatasetEvalMode, Plan as DatasetPlan, PlanEnv, PlanError};
pub use registry::{DatasetArtifact, DatasetMetadata, DatasetRegistry, DatasetSplit};
pub use schema::FeatureSchema;
pub use stem::{Stem, StemKind};
pub use streaming::{StreamingBatchIter, StreamingDataset};
pub use tag::Tag;
pub use token::{Token, TokenKind, TokenSpan};
pub use tree::{
    format_bracketed, format_pretty, parse_bracketed, MultiParentedIndex, MultiParentedNode,
    MultiParentedTree, MultiParentedValue, ParentedIndex, ParentedNode, ParentedTree,
    ParentedValue, ProbabilisticTree, TreeCollection, TreeExpr, TreeId, TreeIndex, TreeLeafExpr,
    TreeLeafValue, TreeNamespace, TreeNode, TreeOp, TreeParseError, TreePos, TreeSeries,
    TreeSeriesNameSpace, TreeSpan, TreeTraversal, TreeValue,
};

// Export specialized datasets (Corpus) as a convenience type.
pub use corpus::Corpus;
