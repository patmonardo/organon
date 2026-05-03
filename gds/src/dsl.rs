//! GDS Shell: curated internal DSL surface for DataFrame + Dataset authoring.
//!
//! This module is the intended user-facing entrypoint for the Framework shell
//! language. DataFrame supplies the executable analytic body; Dataset supplies
//! the semantic framework shell and central pipeline architecture. Together
//! they form the internal GDSL surface.

// Core DataFrame + Dataset types, traits, and helper functions.
pub use crate::collections::dataframe::prelude::*;
pub use crate::collections::dataset::prelude::*;

// Fundamental macro vocabulary.
pub use crate::{
    agg, arrange, col, corpus, ds, expr, filter, get, group_by, io, join, lit, mutate, otherwise,
    pipeline, plan, q, sc, sel, select, select_columns, split, stream, summarize, tbl, tbl_def, td,
    then, when, where_,
};
