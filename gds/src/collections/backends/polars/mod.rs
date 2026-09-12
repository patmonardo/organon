//! Polars Collections backend.
//!
//! Arrow remains the primitive columnar substrate. This backend owns the
//! table-level capabilities that are specific to Polars: DataFrames, deferred
//! relational plans, optimizer flags, plan inspection, and materialization.

mod descriptor;
mod frame;
mod persistence;

pub use descriptor::{PolarsBackendCapabilities, PolarsBackendDescriptor};
pub use frame::PolarsFrameBackend;
pub use persistence::{
    PersistentFrameError, PersistentFrameFormat, PersistentFrameManifest, PersistentPolarsFrame,
    PersistentPolarsStore,
};
