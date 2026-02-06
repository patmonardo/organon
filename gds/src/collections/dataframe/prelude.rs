//! DataFrame prelude: curated, stable exports for common DataFrame usage.
//!
//! Import with:
//! ```rust
//! use gds::collections::dataframe::prelude::*;
//! ```

// Common expression helpers
pub use crate::collections::dataframe::col;
pub use crate::collections::dataframe::lit;
pub use crate::collections::dataframe::when;

// Core types & builders
pub use crate::collections::dataframe::GDSFrameError;
pub use crate::collections::dataframe::PolarsSortMultipleOptions;
pub use crate::collections::dataframe::TableBuilder;

// Expose some useful helpers from frame and streaming
pub use crate::collections::dataframe::GDSDataFrame;
