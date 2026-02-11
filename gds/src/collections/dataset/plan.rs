//! Dataset Plans (DataOps): lazy computation graphs over Datasets.
//!
//! This is a small, intentionally conservative kernel surface:
//! - A `Plan` is a lazily described transformation from a `Dataset` source.
//! - The canonical row/record artifact is a Polars `Struct` field named `item`.
//! - Plans can be evaluated in different modes (preview vs fit).
//!
//! The long-term intent is to compile tabular-only subgraphs to Polars `LazyFrame`
//! plans, while keeping unstructured/streaming nodes in a higher-level IR.

use std::collections::BTreeMap;

use polars::prelude::{Expr, LazyFrame};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::collections::dataframe::GDSFrameError;
use crate::collections::dataset::expressions::dataop::{DatasetDataOp, DatasetDataOpExpr};
use crate::collections::dataset::{Dataset, DatasetSplit};
use crate::prints::{PrintEnvelope, PrintKind, PrintProvenance};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanStepReport {
    pub index: usize,
    pub op: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanAttentionReport {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,

    pub mode: String,

    pub source: JsonValue,

    pub steps: Vec<PlanStepReport>,

    /// Best-effort planned output columns inferred from expressions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planned_columns: Option<Vec<String>>,

    /// Observed output columns after evaluation (if available).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_columns: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_count: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_hint: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_hint: Option<String>,
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
pub enum Source {
    Var(String),
    Value(Dataset),
}

#[derive(Debug, Clone)]
pub enum Step {
    Filter(Expr),
    Select(Vec<Expr>),
    WithColumns(Vec<Expr>),

    /// Canonical record column construction.
    ///
    /// Convention: the output is aliased to the `item` column.
    Item(Expr),

    /// Logical split label (does not change the data by itself).
    Split(DatasetSplit),

    /// Hint for downstream evaluation (batching / streaming).
    Batch(usize),

    /// Dataset data-op step (Input/Encode/Transform/Decode/Output).
    DataOp(DatasetDataOpExpr),
}

#[derive(Debug, Clone)]
pub struct Plan {
    name: Option<String>,
    source: Source,
    steps: Vec<Step>,
}

impl Plan {
    pub fn new(source: Source) -> Self {
        Self {
            name: None,
            source,
            steps: Vec::new(),
        }
    }

    pub fn named(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn source(&self) -> &Source {
        &self.source
    }

    pub fn steps(&self) -> &[Step] {
        &self.steps
    }

    /// Whether this plan ends with an `item` projection step.
    pub fn ends_with_item(&self) -> bool {
        self.steps
            .last()
            .is_some_and(|s| matches!(s, Step::Item(_)))
    }

    /// Whether this plan contains any `item` projection step.
    pub fn has_item(&self) -> bool {
        self.steps.iter().any(|s| matches!(s, Step::Item(_)))
    }

    /// Append an `item` projection step.
    ///
    /// This expresses the idea that `item`/Struct is an *idealization* (a canonical
    /// feature-map artifact) that you can project into when you want to compile
    /// or interop with well-known DataFrame processing.
    ///
    /// Convention: `item_expr` should already be aliased to the `item` column.
    pub fn project_item(mut self, item_expr: Expr) -> Self {
        self.steps.push(Step::Item(item_expr));
        self
    }

    pub fn push_step(mut self, step: Step) -> Self {
        self.steps.push(step);
        self
    }

    pub fn describe_steps(&self) -> String {
        let mut lines = Vec::new();
        if let Some(name) = self.name() {
            lines.push(format!("plan: {name}"));
        } else {
            lines.push("plan".to_string());
        }

        let src = match &self.source {
            Source::Var(v) => format!("source var({v})"),
            Source::Value(ds) => ds
                .name()
                .map(|n| format!("source dataset({n})"))
                .unwrap_or_else(|| "source dataset(<unnamed>)".to_string()),
        };
        lines.push(src);

        for (i, step) in self.steps.iter().enumerate() {
            let s = match step {
                Step::Filter(_) => "filter".to_string(),
                Step::Select(exprs) => format!("select [{} exprs]", exprs.len()),
                Step::WithColumns(exprs) => format!("with_columns [{} exprs]", exprs.len()),
                Step::Item(_) => "item".to_string(),
                Step::Split(split) => format!("split({split:?})"),
                Step::Batch(n) => format!("batch({n})"),
                Step::DataOp(op) => format!("dataop({})", describe_dataop(op.op())),
            };
            lines.push(format!("  {i}: {s}"));
        }

        lines.join("\n")
    }

    pub fn eval_dataset(&self, env: &PlanEnv, mode: EvalMode) -> Result<Dataset, PlanError> {
        let mut ds = match &self.source {
            Source::Var(name) => env
                .get_dataset(name)
                .cloned()
                .ok_or_else(|| PlanError::Message(format!("unbound dataset var: {name}")))?,
            Source::Value(ds) => ds.clone(),
        };

        if mode == EvalMode::Preview {
            ds = ds.head(env.preview_rows());
        }

        for step in &self.steps {
            match step {
                Step::Filter(pred) => {
                    ds = ds.filter_expr(pred.clone())?;
                }
                Step::Select(exprs) => {
                    ds = ds.select(exprs)?;
                }
                Step::WithColumns(exprs) => {
                    ds = ds.with_columns(exprs)?;
                }
                Step::Item(item_expr) => {
                    // Convention: item_expr should already be aliased to "item".
                    ds = ds.with_columns(&[item_expr.clone()])?;
                }
                Step::Split(_) => {
                    // Label-only for now.
                }
                Step::Batch(_) => {
                    // Hint-only for now; streaming evaluation will consume this later.
                }
                Step::DataOp(_) => {
                    // Dataset-level data-op: eval handled by higher-level adapters.
                }
            }
        }

        Ok(ds)
    }

    fn planned_output_columns(&self) -> Option<Vec<String>> {
        // Heuristic:
        // - If there is at least one Select step, the *last* Select defines the outputs.
        // - Otherwise, we cannot know without schema execution; return None.
        // (We will still include item/with_columns hints in step detail.)
        let last_select = self.steps.iter().rev().find_map(|s| match s {
            Step::Select(exprs) => Some(exprs),
            _ => None,
        })?;

        let mut names = expr_output_names(last_select);
        if names.is_empty() {
            return None;
        }
        // Deduplicate while preserving order.
        let mut seen = std::collections::BTreeSet::new();
        names.retain(|n| seen.insert(n.clone()));
        Some(names)
    }

    pub fn attention_report(&self, env: Option<&PlanEnv>, mode: EvalMode) -> PlanAttentionReport {
        let source = match &self.source {
            Source::Var(name) => serde_json::json!({"var": name}),
            Source::Value(ds) => serde_json::json!({
                "dataset": ds.name().unwrap_or("<unnamed>"),
                "rows": ds.row_count(),
                "cols": ds.column_count(),
            }),
        };

        let mut steps = Vec::with_capacity(self.steps.len());
        let mut batch_hint: Option<usize> = None;
        let mut split_hint: Option<String> = None;

        for (index, step) in self.steps.iter().enumerate() {
            let (op, detail) = match step {
                Step::Filter(expr) => (
                    "filter".to_string(),
                    Some(serde_json::json!({
                        "roots": expr.clone().meta().root_names().into_iter().map(|s| s.to_string()).collect::<Vec<_>>()
                    })),
                ),
                Step::Select(exprs) => (
                    "select".to_string(),
                    Some(serde_json::json!({
                        "expr_count": exprs.len(),
                        "outputs": expr_output_names(exprs),
                    })),
                ),
                Step::WithColumns(exprs) => (
                    "with_columns".to_string(),
                    Some(serde_json::json!({
                        "expr_count": exprs.len(),
                        "outputs": expr_output_names(exprs),
                    })),
                ),
                Step::Item(expr) => (
                    "item".to_string(),
                    Some(serde_json::json!({
                        "column": "item",
                        "roots": expr.clone().meta().root_names().into_iter().map(|s| s.to_string()).collect::<Vec<_>>()
                    })),
                ),
                Step::Split(split) => {
                    split_hint = Some(format!("{split:?}"));
                    (
                        "split".to_string(),
                        Some(serde_json::json!({"value": format!("{split:?}")})),
                    )
                }
                Step::Batch(n) => {
                    batch_hint = Some(*n);
                    ("batch".to_string(), Some(serde_json::json!({"size": n})))
                }
                Step::DataOp(op) => ("dataop".to_string(), Some(dataop_report_detail(op.op()))),
            };
            steps.push(PlanStepReport { index, op, detail });
        }

        // Optionally compute observed columns by evaluating preview.
        let mut observed_columns = None;
        let mut row_count = None;
        let mut column_count = None;
        if let Some(env) = env {
            if let Ok(observed) = self.eval_dataset(env, mode) {
                observed_columns = Some(observed.column_names());
                row_count = Some(observed.row_count());
                column_count = Some(observed.column_count());
            }
        }

        PlanAttentionReport {
            plan_name: self.name.clone(),
            mode: format!("{mode:?}"),
            source,
            steps,
            planned_columns: self.planned_output_columns(),
            observed_columns,
            row_count,
            column_count,
            batch_hint,
            split_hint,
        }
    }

    pub fn to_print_envelope(
        &self,
        env: Option<&PlanEnv>,
        mode: EvalMode,
        run_id: Option<String>,
    ) -> PrintEnvelope {
        let report = self.attention_report(env, mode);

        PrintEnvelope::new(
            PrintKind::Ml,
            PrintProvenance {
                source: "gds::collections::dataset::plan".to_string(),
                run_id,
                kernel_version: None,
            },
            serde_json::to_value(report).unwrap_or_else(
                |e| serde_json::json!({"error": format!("failed to serialize plan report: {e}")}),
            ),
        )
    }

    pub fn eval_with_print(
        &self,
        env: &PlanEnv,
        mode: EvalMode,
        run_id: Option<String>,
    ) -> Result<(Dataset, PrintEnvelope), PlanError> {
        let ds = self.eval_dataset(env, mode)?;
        let print = self.to_print_envelope(Some(env), mode, run_id);
        Ok((ds, print))
    }

    /// Compose this plan with another plan by appending the other's steps.
    ///
    /// Semantics:
    /// - Keeps `self`'s source.
    /// - Appends `next`'s steps (cloned) to `self`.
    /// - If `self` has no name but `next` does, adopts `next`'s name.
    pub fn chain(mut self, next: &Plan) -> Self {
        if self.name.is_none() {
            self.name = next.name.clone();
        }

        self.steps.extend(next.steps.iter().cloned());
        self
    }

    /// Apply the plan's tabular steps to a Polars `LazyFrame`.
    ///
    /// This is the bridge that lets Plans act like "models" over streaming
    /// batches: each batch is represented as a `LazyFrame` and we apply the
    /// same feature/selection steps.
    ///
    /// Non-tabular control steps (`Split`, `Batch`) are ignored.
    pub fn apply_to_lazyframe(&self, mut lf: LazyFrame) -> LazyFrame {
        for step in &self.steps {
            match step {
                Step::Filter(pred) => {
                    lf = lf.filter(pred.clone());
                }
                Step::Select(exprs) => {
                    lf = lf.select(exprs.clone());
                }
                Step::WithColumns(exprs) => {
                    lf = lf.with_columns(exprs.clone());
                }
                Step::Item(item_expr) => {
                    lf = lf.with_columns([item_expr.clone()]);
                }
                Step::Split(_) | Step::Batch(_) | Step::DataOp(_) => {
                    // Control-plane only.
                }
            }
        }
        lf
    }

    /// Convert this Plan into a reusable streaming transform.
    ///
    /// The returned closure is suitable for `StreamingDataset::with_transform`.
    pub fn to_streaming_transform(&self) -> Box<dyn Fn(LazyFrame) -> LazyFrame + Send + Sync> {
        let steps = self.steps.clone();
        Box::new(move |mut lf: LazyFrame| {
            for step in &steps {
                match step {
                    Step::Filter(pred) => {
                        lf = lf.filter(pred.clone());
                    }
                    Step::Select(exprs) => {
                        lf = lf.select(exprs.clone());
                    }
                    Step::WithColumns(exprs) => {
                        lf = lf.with_columns(exprs.clone());
                    }
                    Step::Item(item_expr) => {
                        lf = lf.with_columns([item_expr.clone()]);
                    }
                    Step::Split(_) | Step::Batch(_) | Step::DataOp(_) => {}
                }
            }
            lf
        })
    }
}

fn describe_dataop(op: &DatasetDataOp) -> String {
    match op {
        DatasetDataOp::Input { name, .. } => format!("input:{name}"),
        DatasetDataOp::Encode { name, .. } => format!("encode:{name}"),
        DatasetDataOp::Transform { name, .. } => format!("transform:{name}"),
        DatasetDataOp::Decode { name, .. } => format!("decode:{name}"),
        DatasetDataOp::Output { name, .. } => format!("output:{name}"),
    }
}

fn dataop_report_detail(op: &DatasetDataOp) -> JsonValue {
    match op {
        DatasetDataOp::Input { name, detail } => serde_json::json!({
            "kind": "input",
            "name": name,
            "detail": detail,
        }),
        DatasetDataOp::Encode { name, detail } => serde_json::json!({
            "kind": "encode",
            "name": name,
            "detail": detail,
        }),
        DatasetDataOp::Transform { name, detail } => serde_json::json!({
            "kind": "transform",
            "name": name,
            "detail": detail,
        }),
        DatasetDataOp::Decode { name, detail } => serde_json::json!({
            "kind": "decode",
            "name": name,
            "detail": detail,
        }),
        DatasetDataOp::Output { name, detail } => serde_json::json!({
            "kind": "output",
            "name": name,
            "detail": detail,
        }),
    }
}

fn expr_output_names(exprs: &[Expr]) -> Vec<String> {
    exprs
        .iter()
        .filter_map(|expr| expr.clone().meta().output_name().ok())
        .map(|s| s.to_string())
        .collect()
}
