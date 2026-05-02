//! `sem` — the SemDataset Concept-return fold.
//!
//! Dataset is the master client of DataFrame: evidence, model, and plan in one
//! surface. This module implements the universal Collections/kernel aspect of
//! `SemDataset`: `Corpus`, `LanguageModel`, and semantic forms gathered as the
//! returned Dataset end-view.
//!
//! The older top-level `dataset::semantic` path remains a compatibility shim.

pub mod semdataset;
pub mod semform;

pub use semdataset::{SemDataset, SemError};
pub use semform::SemForm;
