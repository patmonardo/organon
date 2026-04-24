//! Abstract prover and model-builder interfaces.
//!
//! Port of `nltk/inference/api.py` (the parts that survive in pure Rust).
//! Python uses ABCs + `BaseProverCommand` mixins; we collapse to a small
//! pair of traits plus a concrete [`ProverCommand`] container.

use crate::ml::nlp::sem::logic::Expression;
use std::error::Error;
use std::fmt;

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

/// An error encountered while running a prover or model builder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InferenceError(pub String);

impl fmt::Display for InferenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Error for InferenceError {}

/// Outcome of an attempted proof. Mirrors the `(bool, str)` return of
/// NLTK's `Prover._prove`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofResult {
    /// True iff the goal was proved from the assumptions.
    pub proved: bool,
    /// Human-readable trace of the proof attempt (may be empty).
    pub proof: String,
}

/// Outcome of an attempted model build. Mirrors `(bool, Valuation)`.
#[derive(Debug, Clone)]
pub struct ModelResult<M> {
    /// True iff a model satisfying the assumptions was built.
    pub built: bool,
    /// The model itself (`None` if not built).
    pub model: Option<M>,
}

// ---------------------------------------------------------------------------
// Traits
// ---------------------------------------------------------------------------

/// Abstract prover interface. Mirrors NLTK `Prover`.
pub trait Prover {
    /// Try to prove `goal` from `assumptions`. If `goal` is `None`, the
    /// prover attempts to derive a contradiction from `assumptions`
    /// alone.
    fn prove(
        &self,
        goal: Option<&Expression>,
        assumptions: &[Expression],
        verbose: bool,
    ) -> Result<ProofResult, InferenceError>;
}

/// Abstract model-builder interface. Mirrors NLTK `ModelBuilder`.
///
/// `M` is the concrete model type (e.g. a `Valuation`). Open formulas
/// are treated as universally quantified.
pub trait ModelBuilder<M> {
    fn build_model(
        &self,
        goal: Option<&Expression>,
        assumptions: &[Expression],
        verbose: bool,
    ) -> Result<ModelResult<M>, InferenceError>;
}

// ---------------------------------------------------------------------------
// Command containers
// ---------------------------------------------------------------------------

/// A reusable bundle of `(prover, goal, assumptions)` that caches its
/// result. Mirrors `BaseProverCommand`.
pub struct ProverCommand<'a, P: Prover> {
    prover: &'a P,
    goal: Option<Expression>,
    assumptions: Vec<Expression>,
    cached: Option<ProofResult>,
}

impl<'a, P: Prover> ProverCommand<'a, P> {
    pub fn new(prover: &'a P, goal: Option<Expression>, assumptions: Vec<Expression>) -> Self {
        Self {
            prover,
            goal,
            assumptions,
            cached: None,
        }
    }

    pub fn add_assumptions(&mut self, new_assumptions: impl IntoIterator<Item = Expression>) {
        self.assumptions.extend(new_assumptions);
        self.cached = None;
    }

    pub fn retract_assumptions(&mut self, retracted: &[Expression]) {
        let before = self.assumptions.len();
        self.assumptions.retain(|a| !retracted.contains(a));
        if self.assumptions.len() != before {
            self.cached = None;
        }
    }

    pub fn assumptions(&self) -> &[Expression] {
        &self.assumptions
    }

    pub fn goal(&self) -> Option<&Expression> {
        self.goal.as_ref()
    }

    pub fn prove(&mut self, verbose: bool) -> Result<bool, InferenceError> {
        if self.cached.is_none() {
            let res = self
                .prover
                .prove(self.goal.as_ref(), &self.assumptions, verbose)?;
            self.cached = Some(res);
        }
        Ok(self.cached.as_ref().unwrap().proved)
    }

    pub fn proof(&self) -> Option<&str> {
        self.cached.as_ref().map(|r| r.proof.as_str())
    }
}
