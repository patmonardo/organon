//! Collections Catalog (Disk-First)
//!
//! Split into types + catalog logic modules.

pub mod catalog;
pub mod disk;
pub mod schema;
pub mod types;
pub mod unity;

pub use catalog::*;
pub use disk::*;
pub use schema::*;
pub use types::*;
pub use unity::*;
