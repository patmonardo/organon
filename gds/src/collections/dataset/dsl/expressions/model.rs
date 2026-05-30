//! Model expressions for dataset-level DSL.
//!
//! Speculative Component-facing descriptors for Model authoring. These are
//! deliberately small and declarative: they describe Model intent without
//! replacing the concrete `Model` Component or the underlying runtime types.
//!
//! This module should stay readable on first pass. It names the pieces an
//! automation script can talk about; the dialectical Model machinery remains
//! in `crate::collections::dataset::model`.

use crate::collections::dataset::model::{
    ModelContext, ModelDelta, ModelId, ModelKind, ModelReport, ModelResult, ModelScore, ModelSpec,
    ModelState, ModelView,
};

#[derive(Debug, Clone, PartialEq)]
pub enum ModelExpr {
    Id(ModelId),
    Kind(ModelKind),
    View(ModelView),
    Spec(ModelSpec),
    Context(ModelContext),
    Delta(ModelDelta),
    Result(ModelResult),
    State(ModelState),
    Score(ModelScore),
    Report(ModelReport),
}

impl ModelExpr {
    pub fn id(id: impl Into<String>) -> Self {
        Self::Id(ModelId(id.into()))
    }

    pub fn kind(kind: ModelKind) -> Self {
        Self::Kind(kind)
    }

    pub fn view(view: ModelView) -> Self {
        Self::View(view)
    }

    pub fn spec(spec: ModelSpec) -> Self {
        Self::Spec(spec)
    }

    pub fn context(context: ModelContext) -> Self {
        Self::Context(context)
    }

    pub fn delta(delta: ModelDelta) -> Self {
        Self::Delta(delta)
    }

    pub fn result(result: ModelResult) -> Self {
        Self::Result(result)
    }

    pub fn state(state: ModelState) -> Self {
        Self::State(state)
    }

    pub fn score(score: ModelScore) -> Self {
        Self::Score(score)
    }

    pub fn report(report: ModelReport) -> Self {
        Self::Report(report)
    }
}
