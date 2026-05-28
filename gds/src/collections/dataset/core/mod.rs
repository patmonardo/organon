//! Dataset core substrate.
//!
//! Identity, schema, artifact classification, IO errors, metrics, valuation,
//! and small support utilities live here. The Dataset root re-exports the
//! durable public types; implementation modules stay grouped under `core`.

pub mod artifact;
pub mod catalog;
pub mod dataset;
pub mod error;
pub mod graph;
pub mod io;
pub mod metrics;
pub mod schema;
pub mod utils;
pub mod valuation;

pub use artifact::*;
pub use catalog::*;
pub use dataset::*;
pub use error::*;
pub use graph::*;
pub use io::*;
pub use metrics::*;
pub use schema::*;
pub use valuation::*;
