//! Dataset prelude: curated, stable exports for users of the dataset DSL.
//!
//! This module intentionally keeps the public surface small and opinionated.
//! Import with:
//!
//! ```rust
//! use gds::collections::dataset::prelude::*;
//! ```

// Core dataset types
pub use crate::collections::dataset::catalog::DatasetCatalog;
pub use crate::collections::dataset::corpus::Corpus;
pub use crate::collections::dataset::dataset::Dataset;

// Plan / DataOps (lazy computation graphs)
pub use crate::collections::dataset::plan::{EvalMode, Plan, PlanEnv, PlanError, Source, Step};

// Features (TypedDict-like `item` Struct)
pub use crate::collections::dataset::feature::{
    Feature, FeatureExpr, FeatureExprNameSpace, FeatureNamespace, FeatureSpace, FeatureView,
};

// Model interface
pub use crate::collections::dataset::model::{
    Model, ModelAttributeUpdate, ModelContext, ModelDelta, ModelId, ModelKind, ModelReport,
    ModelResult, ModelScore, ModelSpec, ModelState, ModelView, NoOpLanguageModel, NoOpParser,
    NoOpTagger,
};

// Language model helpers
pub use crate::collections::dataset::functions::model::lm::{
    AbsoluteDiscountingInterpolated, KneserNeyInterpolated, LanguageModel, Laplace, Lidstone,
    LmBase, LmError, NgramCounter, StupidBackoff, Vocabulary, WittenBellInterpolated, MLE,
};
pub use crate::collections::dataset::functions::model::preprocessing::{
    everygrams, ngrams, pad_both_ends, pad_both_ends_default, padded_everygram_pipeline,
    padded_everygrams,
};

// Selector-driven schema views
pub use crate::collections::dataset::schema::FeatureSchema;

// Streaming convenience types
pub use crate::collections::dataset::streaming::{StreamingBatchIter, StreamingDataset};

// Token surface
pub use crate::collections::dataset::token::{Token, TokenKind, TokenSpan};

// Stem surface
pub use crate::collections::dataset::stem::{Stem, StemKind};

// Parse surface
pub use crate::collections::dataset::parse::{Parse, ParseForest, ParseKind};

// Feature structure surface
pub use crate::collections::dataset::featstruct::{
    format_featstruct, parse_featstruct, parse_featvalue, subsumes_featstruct, unify_featstruct,
    FeatBindings, FeatDict, FeatList, FeatPath, FeatPathSegment, FeatReentranceId, FeatStruct,
    FeatStructParseError, FeatStructSet, FeatValue,
};

// Tag surface
pub use crate::collections::dataset::tag::Tag;

// Tree surface (namespace + collection)
pub use crate::collections::dataset::tree::{
    format_bracketed, format_pretty, parse_bracketed, pretty_print, MultiParentedIndex,
    MultiParentedNode, MultiParentedTree, MultiParentedValue, ParentedIndex, ParentedNode,
    ParentedTree, ParentedValue, PrettyOptions, ProbabilisticTree, TreeCollection, TreeNamespace,
    TreeParseError, TreeSeries, TreeTraversal,
};

// Tabular DSL matrix: extension traits
pub use crate::collections::dataset::expr::ExprDatasetExt;
pub use crate::collections::dataset::frame::DataFrameDatasetExt;
pub use crate::collections::dataset::lazy::LazyFrameDatasetExt;
pub use crate::collections::dataset::series::SeriesDatasetExt;

// Dataset expression namespace
pub use crate::collections::dataset::namespaces::dataset::DatasetNs;

// Utilities
pub use crate::collections::dataset::functions::scan_text_dir;

// Registry types
pub use crate::collections::dataset::registry::{
    DatasetArtifact, DatasetMetadata, DatasetRegistry, DatasetSplit,
};

// Expression helpers (low-volume, stable helpers under expressions)
pub use crate::collections::dataset::expressions::parse::{
    parse_end_expr, parse_field_expr, parse_kind_expr, parse_score_expr, parse_start_expr,
    parse_tree_expr,
};
pub use crate::collections::dataset::expressions::stem::{
    stem_end_expr, stem_field_expr, stem_kind_expr, stem_start_expr, stem_text_expr,
};
pub use crate::collections::dataset::expressions::tag::{
    tag_end_expr, tag_field_expr, tag_start_expr, tag_tag_expr, tag_text_expr,
};
pub use crate::collections::dataset::expressions::text::{
    lowercase_expr, token_count_expr, tokenize_expr,
};
pub use crate::collections::dataset::expressions::token::{
    token_end_expr, token_field_expr, token_kind_expr, token_start_expr, token_text_expr,
};
