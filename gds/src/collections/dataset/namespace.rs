//! Top-level Dataset DSL namespace shell.
//!
//! This module is the single, authoritative entry point for the dataset-side
//! DSL "namespace" surface. It mirrors the role of
//! [`crate::collections::dataframe::namespace`] for the DataFrame DSL, but is
//! tailored to the semantic-first Dataset layer.
//!
//! Two things live here:
//!
//! 1. **The 2×2 matrix wrappers and their `.ds()` extension traits.** These are
//!    the lightweight dataset-flavored namespace structs attached to the four
//!    Polars-shaped surfaces — `Expr`, `LazyFrame`, `DataFrame`, `Series` — via
//!    the modules [`crate::collections::dataset::expr`],
//!    [`crate::collections::dataset::lazy`],
//!    [`crate::collections::dataset::frame`], and
//!    [`crate::collections::dataset::series`]. They are re-exported here so
//!    that `use crate::collections::dataset::namespace::*;` brings in the
//!    dataset DSL shell in one line.
//!
//! 2. **The dataset namespace registry.** Custom dataset namespaces (e.g.
//!    `corpus`, `text`, `image`, `audio`, `tabular`, `tree`) are registered
//!    against the dataset-side registry in
//!    [`crate::collections::dataset::namespaces`]. The registration functions
//!    and target/error types are re-exported here for ergonomics and to keep
//!    the dataset namespace registry distinct from the DataFrame namespace
//!    registry.
//!
//! Boundary:
//! - The DSL shell here is intentionally thin. Concrete builders for specific
//!   namespaces (`feature`, `text`, `tree`, `dataop`, `dataset`) live under
//!   [`crate::collections::dataset::namespaces`] and are not re-exported from
//!   this shell module.
//! - This module does not own semantic/compiler surfaces such as `Plan`,
//!   `Feature`, `Model`, or `ToolChain`; those are exposed via the dataset
//!   barrel under their own dedicated modules.

// ---- 2x2 DSL matrix wrappers ------------------------------------------------

pub use crate::collections::dataset::expr::{DatasetExprNameSpace, ExprDatasetExt};
pub use crate::collections::dataset::frame::{DataFrameDatasetExt, DatasetDataFrameNameSpace};
pub use crate::collections::dataset::lazy::{
    DatasetLazyFrameNameSpace, FeatureLazyFrameNameSpace, LazyFrameDatasetExt,
    TreeLazyFrameNameSpace,
};
pub use crate::collections::dataset::series::{DatasetSeriesNameSpace, SeriesDatasetExt};

// ---- Dataset namespace registry --------------------------------------------

pub use crate::collections::dataset::namespaces::{
    is_dataset_namespace_registered, register_corpus_namespace, register_dataset_namespace,
    NameSpaceError,
};
