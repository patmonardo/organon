//! Plan expressions for dataset-level DSL.
//!
//! Speculative Component-facing descriptors for Plan authoring. These values
//! stay declarative and let the DSL talk about Plan pieces without making the
//! expression layer itself into the runtime.

use crate::collections::dataframe::GDSExpr as Expr;
use crate::collections::dataset::lab::protocol::dataop::DatasetDataOpExpr;
use crate::collections::dataset::plan::{
    EvalMode, Plan, PlanAttentionReport, PlanEnv, PlanSource, PlanStepReport, Step,
};
use crate::collections::dataset::DatasetSplit;

#[derive(Debug, Clone)]
pub enum PlanExpr {
    Plan(Plan),
    Source(PlanSource),
    Step(Step),
    Env(PlanEnv),
    Mode(EvalMode),
    AttentionReport(PlanAttentionReport),
    StepReport(PlanStepReport),
    Split(DatasetSplit),
    Expr(Expr),
    DataOp(DatasetDataOpExpr),
}

impl PlanExpr {
    pub fn plan(plan: Plan) -> Self {
        Self::Plan(plan)
    }

    pub fn source(source: PlanSource) -> Self {
        Self::Source(source)
    }

    pub fn step(step: Step) -> Self {
        Self::Step(step)
    }

    pub fn env(env: PlanEnv) -> Self {
        Self::Env(env)
    }

    pub fn mode(mode: EvalMode) -> Self {
        Self::Mode(mode)
    }

    pub fn attention_report(report: PlanAttentionReport) -> Self {
        Self::AttentionReport(report)
    }

    pub fn step_report(report: PlanStepReport) -> Self {
        Self::StepReport(report)
    }

    pub fn split(split: DatasetSplit) -> Self {
        Self::Split(split)
    }

    pub fn expr(expr: Expr) -> Self {
        Self::Expr(expr)
    }

    pub fn dataop(op: DatasetDataOpExpr) -> Self {
        Self::DataOp(op)
    }
}
