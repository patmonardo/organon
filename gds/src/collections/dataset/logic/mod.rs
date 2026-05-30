//! `logic` — the LogicFrame Concept-return fold.
//!
//! Dataset is the master client of DataFrame: evidence, model, and plan in one
//! surface. This module implements the universal Collections/kernel aspect of
//! `LogicFrame`: `Corpus`, `LanguageModel`, and semantic forms gathered as the
//! returned Dataset end-view.
//!
pub mod logic_form;
pub mod logic_frame;
mod manifest;

pub use logic_form::*;
pub use logic_frame::*;
pub use manifest::*;
