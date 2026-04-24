//! Semantic representation layer ported from NLTK `nltk/sem/`.
//!
//! Layering:
//! - [`logic`] — lambda calculus + first-order logic AST + parser
//!   (port of `nltk/sem/logic.py`, untyped slice).
//! - [`evaluate`] — Tarski model + `satisfy()` interpreter
//!   (port of `nltk/sem/evaluate.py`).
//!
//! Companion data layer lives in
//! [`crate::collections::dataset::featstruct`] (NLTK
//! `nltk/featstruct.py`). Note the deliberate naming overlap with
//! [`crate::collections::dataset::valuation`]: that module is the
//! row-shaped R4 access object (Feature → cell), whereas
//! [`evaluate::Valuation`] is the symbol-table valuation of NLTK's
//! model theory (predicate-name → relation). The bridge between the
//! two is a future architectural conversation.

pub mod evaluate;
pub mod logic;
pub mod skolemize;
