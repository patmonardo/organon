//! First-order logic inference — port of `nltk/inference/`.
//!
//! Self-contained: no external dependencies on Prover9, Mace4, etc.
//! NLTK's `prover9.py` and `mace.py` are subprocess wrappers and are
//! intentionally **not** ported here.
//!
//! Currently provides:
//! - [`api`] — abstract `Prover` and `ModelBuilder` traits (mirrors
//!   `nltk/inference/api.py`).
//! - [`tableau`] — native first-order semantic-tableau prover (port of
//!   `nltk/inference/tableau.py`).
//! - [`resolution`] — native first-order resolution prover (port of
//!   `nltk/inference/resolution.py`). Implements the same `Prover`
//!   trait so it can be swapped under [`api::ProverCommand`].
//!
//! Planned slices: `nonmonotonic`.

pub mod api;
pub mod resolution;
pub mod tableau;
