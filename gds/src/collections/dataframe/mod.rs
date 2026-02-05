//! Polars DataFrame integration for Collections.

pub mod array;
pub mod column;
pub mod config;
pub mod datatype_expr;
pub mod datatypes;
pub mod errors;
pub mod expr;
pub mod expressions;
pub mod frame;
pub mod functions;
pub mod interchange;
pub mod lazy;
pub mod namespace;
pub mod namespaces;
pub mod row;
pub mod schema;
pub mod selectors;
pub mod series;
pub mod streaming;
pub mod table;
pub mod utils;

pub use array::*;
pub use column::*;
pub use datatype_expr::*;
pub use datatypes::*;
pub use expr::*;
pub use frame::*;
pub use functions::*;
pub use interchange::*;
pub use lazy::*;
pub use namespace::*;
pub use row::*;
pub use schema::*;
pub use selectors::{
    expand_exprs, expand_selector, selector_all, selector_by_dtype, selector_by_name,
    selector_contains, selector_matches, selector_numeric, selector_starts_with, selector_string,
    selector_temporal, Selector,
};
pub use series::*;
pub use streaming::*;
pub use table::*;

pub type PolarsSortMultipleOptions = polars::prelude::SortMultipleOptions;
pub type JoinType = polars::prelude::JoinType;
pub use errors::GDSFrameError;
