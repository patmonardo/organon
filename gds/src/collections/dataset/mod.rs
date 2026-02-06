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
pub mod functions;
pub mod io;
pub mod namespaces;
pub mod prelude;
pub mod registry;
pub mod streaming;

// Keep the module surface small and explicitly export the core public items.
pub use catalog::DatasetCatalog;
pub use dataset::Dataset;
pub use error::DatasetIoError;
pub use functions::scan_text_dir;
pub use registry::{DatasetArtifact, DatasetMetadata, DatasetRegistry, DatasetSplit};
pub use streaming::{StreamingBatchIter, StreamingDataset};

// Export specialized datasets (Corpus) as a convenience type.
pub use corpus::Corpus;
