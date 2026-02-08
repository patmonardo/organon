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

// Model composition helpers
pub use crate::collections::dataset::model::ModelWithFeatures;

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

// Tree surface (namespace + collection)
pub use crate::collections::dataset::tree::{TreeCollection, TreeNamespace, TreeSeries};

// LazyFrame namespace glue
pub use crate::collections::dataset::namespace::LazyFrameDatasetExt;

// Utilities
pub use crate::collections::dataset::functions::scan_text_dir;

// Registry types
pub use crate::collections::dataset::registry::{
    DatasetArtifact, DatasetMetadata, DatasetRegistry, DatasetSplit,
};

// Expression helpers (low-volume, stable helpers under expressions)
pub use crate::collections::dataset::expressions::text::{
    lowercase_expr, token_count_expr, tokenize_expr,
};
