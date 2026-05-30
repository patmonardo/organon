use std::collections::BTreeMap;

use crate::collections::dataframe::GDSExpr as Expr;

use crate::collections::dataframe::GDSFrameError;
use crate::collections::dataset::lab::protocol::dataop::DatasetDataOpExpr;
use crate::collections::dataset::{Dataset, DatasetSplit};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvalMode {
    Preview,
    Fit,
}

#[derive(Debug, thiserror::Error)]
pub enum PlanError {
    #[error("plan error: {0}")]
    Message(String),

    #[error(transparent)]
    Frame(#[from] GDSFrameError),
}

#[derive(Debug, Clone)]
pub struct PlanEnv {
    datasets: BTreeMap<String, Dataset>,
    preview_rows: usize,
}

impl Default for PlanEnv {
    fn default() -> Self {
        Self {
            datasets: BTreeMap::new(),
            preview_rows: 1_000,
        }
    }
}

impl PlanEnv {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_preview_rows(mut self, n: usize) -> Self {
        self.preview_rows = n.max(1);
        self
    }

    pub fn bind_dataset(mut self, name: impl Into<String>, ds: Dataset) -> Self {
        self.datasets.insert(name.into(), ds);
        self
    }

    pub fn get_dataset(&self, name: &str) -> Option<&Dataset> {
        self.datasets.get(name)
    }

    pub fn preview_rows(&self) -> usize {
        self.preview_rows
    }
}

#[derive(Debug, Clone)]
pub enum PlanSource {
    Var(String),
    Value(Dataset),
}

impl PlanSource {
    pub fn var(name: impl Into<String>) -> Self {
        Self::Var(name.into())
    }

    pub fn value(dataset: Dataset) -> Self {
        Self::Value(dataset)
    }
}

#[derive(Debug, Clone)]
pub enum Step {
    Filter(Expr),
    Select(Vec<Expr>),
    WithColumns(Vec<Expr>),
    Item(Expr),
    Split(DatasetSplit),
    Batch(usize),
    DataOp(DatasetDataOpExpr),
}

impl Step {
    pub fn is_tabular_kernel_step(&self) -> bool {
        matches!(
            self,
            Step::Filter(_) | Step::Select(_) | Step::WithColumns(_) | Step::Item(_)
        )
    }

    pub fn is_control_plane_step(&self) -> bool {
        matches!(self, Step::Split(_) | Step::Batch(_) | Step::DataOp(_))
    }
}
