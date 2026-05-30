use crate::collections::dataframe::GDSExpr as Expr;
use crate::collections::dataset::lab::protocol::dataop::{DatasetDataOp, DatasetDataOpExpr};
use crate::collections::dataset::Dataset;
use crate::prints::{PrintEnvelope, PrintKind, PrintProvenance};
use polars::prelude::LazyFrame;
use serde_json::Value as JsonValue;

use super::concept::{
    CognitionMode, ConceptTriad, LawOfAppearance, PlanPrinciple, PlanPrincipleReport,
    PlanSynthesis, PlanSynthesisReport,
};
use super::report::{PlanAttentionReport, PlanStepReport};
use super::runtime::{EvalMode, PlanEnv, PlanError, PlanSource, Step};

#[derive(Debug, Clone)]
pub struct Plan {
    name: Option<String>,
    source: PlanSource,
    steps: Vec<Step>,
    synthesis: PlanSynthesis,
    principle: Option<PlanPrinciple>,
}

impl Plan {
    pub fn from_var(name: impl Into<String>) -> Self {
        Self::new(PlanSource::var(name))
    }

    pub fn from_dataset(dataset: Dataset) -> Self {
        Self::new(PlanSource::value(dataset))
    }

    pub fn new(source: PlanSource) -> Self {
        Self {
            name: None,
            source,
            steps: Vec::new(),
            synthesis: PlanSynthesis::default(),
            principle: None,
        }
    }

    pub fn named(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn source(&self) -> &PlanSource {
        &self.source
    }

    pub fn steps(&self) -> &[Step] {
        &self.steps
    }

    pub fn synthesis(&self) -> &PlanSynthesis {
        &self.synthesis
    }

    pub fn with_model_anchor(mut self, model_anchor: impl Into<String>) -> Self {
        self.synthesis = self.synthesis.with_model_anchor(model_anchor);
        self
    }

    pub fn with_image_anchor(mut self, image_anchor: impl Into<String>) -> Self {
        self.synthesis = self.synthesis.with_image_anchor(image_anchor);
        self
    }

    pub fn with_feature_anchor(mut self, feature_anchor: impl Into<String>) -> Self {
        self.synthesis = self.synthesis.with_feature_anchor(feature_anchor);
        self
    }

    pub fn with_feature_anchors<I>(mut self, feature_anchors: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<String>,
    {
        for feature_anchor in feature_anchors {
            self.synthesis = self.synthesis.with_feature_anchor(feature_anchor);
        }
        self
    }

    pub fn is_unified_image_feature_plan(&self) -> bool {
        self.synthesis.is_unified()
    }

    pub fn principle(&self) -> Option<&PlanPrinciple> {
        self.principle.as_ref()
    }

    pub fn with_principle_triad(mut self, triad: ConceptTriad) -> Self {
        let mut principle = self
            .principle
            .take()
            .unwrap_or_else(|| PlanPrinciple::rational(triad));
        principle.triad = triad;
        if principle
            .law_of_appearance
            .as_ref()
            .is_some_and(|law| !law.is_empty())
        {
            principle.mode = CognitionMode::Empirical;
        } else {
            principle.mode = CognitionMode::Rational;
            principle.law_of_appearance = None;
        }
        self.principle = Some(principle);
        self
    }

    pub fn with_mock_law_of_appearance<I>(mut self, feature_anchors: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<String>,
    {
        let law = LawOfAppearance::mock_from_feature_anchors(feature_anchors);
        let triad = self
            .principle
            .as_ref()
            .map(|p| p.triad)
            .unwrap_or(ConceptTriad::ModelFeaturePlan);
        let principle = self
            .principle
            .take()
            .unwrap_or_else(|| PlanPrinciple::rational(triad))
            .with_law_of_appearance(law);
        self.principle = Some(principle);
        self
    }

    pub fn with_empirical_observation(
        mut self,
        feature_anchor: impl Into<String>,
        evidence: impl Into<String>,
    ) -> Self {
        let triad = self
            .principle
            .as_ref()
            .map(|p| p.triad)
            .unwrap_or(ConceptTriad::ModelFeaturePlan);
        let law = self
            .principle
            .as_ref()
            .and_then(|p| p.law_of_appearance.clone())
            .unwrap_or_default()
            .add_observation(feature_anchor, evidence);
        self.principle = Some(
            self.principle
                .take()
                .unwrap_or_else(|| PlanPrinciple::rational(triad))
                .with_law_of_appearance(law),
        );
        self
    }

    pub fn ends_with_item(&self) -> bool {
        self.steps
            .last()
            .is_some_and(|s| matches!(s, Step::Item(_)))
    }

    pub fn has_item(&self) -> bool {
        self.steps.iter().any(|s| matches!(s, Step::Item(_)))
    }

    pub fn project_item(mut self, item_expr: Expr) -> Self {
        self.steps.push(Step::Item(item_expr));
        self
    }

    pub fn push_step(mut self, step: Step) -> Self {
        self.steps.push(step);
        self
    }

    pub fn filter(self, predicate: Expr) -> Self {
        self.push_step(Step::Filter(predicate))
    }

    pub fn select<I>(self, exprs: I) -> Self
    where
        I: IntoIterator<Item = Expr>,
    {
        self.push_step(Step::Select(exprs.into_iter().collect()))
    }

    pub fn with_columns<I>(self, exprs: I) -> Self
    where
        I: IntoIterator<Item = Expr>,
    {
        self.push_step(Step::WithColumns(exprs.into_iter().collect()))
    }

    pub fn split(self, split: crate::collections::dataset::DatasetSplit) -> Self {
        self.push_step(Step::Split(split))
    }

    pub fn batch(self, n: usize) -> Self {
        self.push_step(Step::Batch(n))
    }

    pub fn dataop(self, op: DatasetDataOpExpr) -> Self {
        self.push_step(Step::DataOp(op))
    }

    pub fn describe_steps(&self) -> String {
        let mut lines = Vec::new();
        if let Some(name) = self.name() {
            lines.push(format!("plan: {name}"));
        } else {
            lines.push("plan".to_string());
        }

        let src = match &self.source {
            PlanSource::Var(v) => format!("source var({v})"),
            PlanSource::Value(ds) => ds
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
            PlanSource::Var(name) => env
                .get_dataset(name)
                .cloned()
                .ok_or_else(|| PlanError::Message(format!("unbound dataset var: {name}")))?,
            PlanSource::Value(ds) => ds.clone(),
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
                    ds = ds.with_columns(&[item_expr.clone()])?;
                }
                Step::Split(_) => {}
                Step::Batch(_) => {}
                Step::DataOp(_) => {}
            }
        }

        Ok(ds)
    }

    fn planned_output_columns(&self) -> Option<Vec<String>> {
        let last_select = self.steps.iter().rev().find_map(|s| match s {
            Step::Select(exprs) => Some(exprs),
            _ => None,
        })?;

        let mut names = expr_output_names(last_select);
        if names.is_empty() {
            return None;
        }
        let mut seen = std::collections::BTreeSet::new();
        names.retain(|n| seen.insert(n.clone()));
        Some(names)
    }

    pub fn attention_report(&self, env: Option<&PlanEnv>, mode: EvalMode) -> PlanAttentionReport {
        let source = match &self.source {
            PlanSource::Var(name) => serde_json::json!({"var": name}),
            PlanSource::Value(ds) => serde_json::json!({
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
            synthesis: (!self.synthesis.is_empty())
                .then(|| PlanSynthesisReport::from(&self.synthesis)),
            principle: self.principle.as_ref().map(PlanPrincipleReport::from),
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

    pub fn chain(mut self, next: &Plan) -> Self {
        if self.name.is_none() {
            self.name = next.name.clone();
        }

        if self.synthesis.model_anchor.is_none() {
            self.synthesis.model_anchor = next.synthesis.model_anchor.clone();
        }
        if self.synthesis.image_anchor.is_none() {
            self.synthesis.image_anchor = next.synthesis.image_anchor.clone();
        }
        for feature_anchor in &next.synthesis.feature_anchors {
            if !self
                .synthesis
                .feature_anchors
                .iter()
                .any(|existing| existing == feature_anchor)
            {
                self.synthesis.feature_anchors.push(feature_anchor.clone());
            }
        }

        self.principle = self.principle.or_else(|| next.principle.clone());
        self.steps.extend(next.steps.iter().cloned());
        self
    }

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
                Step::Split(_) | Step::Batch(_) | Step::DataOp(_) => {}
            }
        }
        lf
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plan_synthesis_unifies_image_and_features() {
        let plan = Plan::from_var("ds")
            .with_model_anchor("model:tagger:v1")
            .with_image_anchor("ontology-image:model:tagger:v1")
            .with_feature_anchor("feature:token")
            .with_feature_anchor("feature:lemma");

        assert!(plan.is_unified_image_feature_plan());
        assert_eq!(
            plan.synthesis().model_anchor.as_deref(),
            Some("model:tagger:v1")
        );
        assert_eq!(plan.synthesis().feature_anchors.len(), 2);
    }

    #[test]
    fn attention_report_includes_synthesis_when_present() {
        let plan = Plan::from_var("ds")
            .with_image_anchor("ontology-image:model:1")
            .with_feature_anchors(["feature:a", "feature:b"]);

        let report = plan.attention_report(None, EvalMode::Preview);
        assert!(report.synthesis.is_some());
        assert!(report.synthesis.expect("synthesis").unified);
    }

    #[test]
    fn mock_law_of_appearance_marks_empirical_transition() {
        let plan = Plan::from_var("ds")
            .with_principle_triad(ConceptTriad::ModelFeaturePlan)
            .with_mock_law_of_appearance(["feature:token", "feature:lemma"]);

        let principle = plan.principle().expect("principle");
        assert!(principle.is_empirical_transition());
        assert_eq!(principle.mode, CognitionMode::Empirical);
    }

    #[test]
    fn attention_report_includes_principle_when_present() {
        let plan = Plan::from_var("ds")
            .with_principle_triad(ConceptTriad::EntityPropertyRelation)
            .with_empirical_observation("feature:cursor-move", "mock-observation:cursor");

        let report = plan.attention_report(None, EvalMode::Preview);
        let principle = report.principle.expect("principle");
        assert_eq!(principle.triad, "entity-property-relation");
        assert!(principle.empirical_transition);
        assert_eq!(principle.observation_count, 1);
    }
}
