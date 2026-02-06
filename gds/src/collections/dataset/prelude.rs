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

// Streaming convenience types
pub use crate::collections::dataset::streaming::{StreamingBatchIter, StreamingDataset};

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
