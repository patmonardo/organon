//! Typed internal scripts for Semantic Dataset knowledge processing.

use crate::collections::dataframe::GDSExpr;
use crate::collections::dataset::{
    Dataset, EvalMode, Plan, PlanAttentionReport, PlanEnv, PlanError, Step,
};
use crate::prints::PrintEnvelope;

use super::{GdsShell, ShellAddress, ShellSchema};

const SHELL_DATASET_BINDING: &str = "shell";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellScriptMode {
    Preview,
    Fit,
}

impl From<ShellScriptMode> for EvalMode {
    fn from(mode: ShellScriptMode) -> Self {
        match mode {
            ShellScriptMode::Preview => EvalMode::Preview,
            ShellScriptMode::Fit => EvalMode::Fit,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ShellScript {
    plan: Plan,
    mode: ShellScriptMode,
    preview_rows: usize,
    run_id: Option<String>,
}

impl ShellScript {
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        Self {
            plan: Plan::from_var(SHELL_DATASET_BINDING).named(name),
            mode: ShellScriptMode::Fit,
            preview_rows: 1_000,
            run_id: None,
        }
    }

    pub fn preview(mut self, rows: usize) -> Self {
        self.mode = ShellScriptMode::Preview;
        self.preview_rows = rows.max(1);
        self
    }

    pub fn fit(mut self) -> Self {
        self.mode = ShellScriptMode::Fit;
        self
    }

    pub fn with_run_id(mut self, run_id: impl Into<String>) -> Self {
        self.run_id = Some(run_id.into());
        self
    }

    pub fn filter(mut self, predicate: GDSExpr) -> Self {
        self.plan = self.plan.filter(predicate);
        self
    }

    pub fn select<I>(mut self, expressions: I) -> Self
    where
        I: IntoIterator<Item = GDSExpr>,
    {
        self.plan = self.plan.select(expressions);
        self
    }

    pub fn with_columns<I>(mut self, expressions: I) -> Self
    where
        I: IntoIterator<Item = GDSExpr>,
    {
        self.plan = self.plan.with_columns(expressions);
        self
    }

    pub fn project_item(mut self, expression: GDSExpr) -> Self {
        self.plan = self.plan.project_item(expression);
        self
    }

    pub fn push_step(mut self, step: Step) -> Self {
        self.plan = self.plan.push_step(step);
        self
    }

    pub fn name(&self) -> Option<&str> {
        self.plan.name()
    }

    pub fn mode(&self) -> ShellScriptMode {
        self.mode
    }

    pub fn plan(&self) -> &Plan {
        &self.plan
    }

    pub fn describe(&self) -> String {
        self.plan.describe_steps()
    }

    pub(crate) fn execute(
        &self,
        dataset: Dataset,
    ) -> Result<(Dataset, PlanAttentionReport, PrintEnvelope), ShellScriptError> {
        self.validate()?;
        let env = PlanEnv::new()
            .with_preview_rows(self.preview_rows)
            .bind_dataset(SHELL_DATASET_BINDING, dataset);
        let mode = self.mode.into();
        let (dataset, print) = self.plan.eval_with_print(&env, mode, self.run_id.clone())?;
        let report = serde_json::from_value(print.payload.clone())?;
        Ok((dataset, report, print))
    }

    fn validate(&self) -> Result<(), ShellScriptError> {
        for step in self.plan.steps() {
            let unsupported = match step {
                Step::Split(_) => Some("split"),
                Step::Batch(_) => Some("batch"),
                Step::DataOp(_) => Some("dataop"),
                Step::Filter(_) | Step::Select(_) | Step::WithColumns(_) | Step::Item(_) => None,
            };
            if let Some(step) = unsupported {
                return Err(ShellScriptError::UnsupportedStep(step.to_string()));
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct ShellScriptResult {
    shell: GdsShell,
    report: PlanAttentionReport,
    print: PrintEnvelope,
    before_address: ShellAddress,
    after_address: ShellAddress,
    before_schema: Option<ShellSchema>,
    after_schema: Option<ShellSchema>,
    mode: ShellScriptMode,
}

impl ShellScriptResult {
    pub(crate) fn new(
        shell: GdsShell,
        report: PlanAttentionReport,
        print: PrintEnvelope,
        before_address: ShellAddress,
        before_schema: Option<ShellSchema>,
        mode: ShellScriptMode,
    ) -> Self {
        let after_address = shell.address();
        let after_schema = shell.seed().map(|seed| seed.schema().clone());
        Self {
            shell,
            report,
            print,
            before_address,
            after_address,
            before_schema,
            after_schema,
            mode,
        }
    }

    pub fn shell(&self) -> &GdsShell {
        &self.shell
    }

    pub fn into_shell(self) -> GdsShell {
        self.shell
    }

    pub fn report(&self) -> &PlanAttentionReport {
        &self.report
    }

    pub fn print(&self) -> &PrintEnvelope {
        &self.print
    }

    pub fn before_address(&self) -> ShellAddress {
        self.before_address
    }

    pub fn after_address(&self) -> ShellAddress {
        self.after_address
    }

    pub fn before_schema(&self) -> Option<&ShellSchema> {
        self.before_schema.as_ref()
    }

    pub fn after_schema(&self) -> Option<&ShellSchema> {
        self.after_schema.as_ref()
    }

    pub fn mode(&self) -> ShellScriptMode {
        self.mode
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ShellScriptError {
    #[error("shell script requires a DataFrame or Dataset body")]
    MissingBody,

    #[error("shell script step `{0}` is not executable yet")]
    UnsupportedStep(String),

    #[error(transparent)]
    Plan(#[from] PlanError),

    #[error("shell script attention report is invalid: {0}")]
    Report(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::dataframe::{col, lit};
    use crate::form::ProgramFeatures;

    fn knowledge_shell() -> GdsShell {
        let frame = crate::tbl_def!(
            (term_id: i64 => [1, 2, 3]),
            (term: ["model", "feature", "plan"]),
        )
        .expect("build knowledge frame");
        GdsShell::from_dataset(Dataset::named("knowledge", frame)).with_program_features(
            ProgramFeatures::new(
                "shell.knowledge".to_string(),
                vec!["Knowledge".to_string()],
                Vec::new(),
            ),
        )
    }

    #[test]
    fn script_transforms_dataset_and_preserves_semantic_commitments() {
        let shell = knowledge_shell();
        let script = ShellScript::new("derive-knowledge")
            .with_run_id("run-1")
            .with_columns([lit("mediated").alias("state")])
            .select([col("term_id"), col("term"), col("state")]);

        let result = shell.run_script(&script).expect("run shell script");
        let output = result.shell();

        assert_eq!(output.dataset().expect("output dataset").row_count(), 3);
        assert_eq!(
            output.dataset().expect("output dataset").column_names(),
            vec!["term_id", "term", "state"]
        );
        assert_eq!(
            output.program().expect("preserved program").program_name(),
            "shell.knowledge"
        );
        assert!(output.logic_frame().is_none());
        assert_eq!(result.report().row_count, Some(3));
        assert_eq!(
            result.report().observed_columns.as_ref().unwrap(),
            &vec![
                "term_id".to_string(),
                "term".to_string(),
                "state".to_string()
            ]
        );
        assert_eq!(result.print().provenance.run_id.as_deref(), Some("run-1"));
        assert_eq!(result.before_address(), result.after_address());
        assert_ne!(result.before_schema(), result.after_schema());
    }

    #[test]
    fn preview_limits_result_without_consuming_source_shell() {
        let shell = knowledge_shell();
        let script = ShellScript::new("preview-knowledge").select([col("term")]);

        let result = shell.preview_script(&script, 1).expect("preview script");

        assert_eq!(result.mode(), ShellScriptMode::Preview);
        assert_eq!(result.shell().dataset().unwrap().row_count(), 1);
        assert_eq!(shell.dataset().unwrap().row_count(), 3);
    }

    #[test]
    fn unsupported_control_step_fails_explicitly() {
        let shell = knowledge_shell();
        let script = ShellScript::new("batch-knowledge").push_step(Step::Batch(2));

        let error = shell
            .run_script(&script)
            .expect_err("batch must not be a no-op");

        assert!(matches!(
            error,
            ShellScriptError::UnsupportedStep(step) if step == "batch"
        ));
    }

    #[test]
    fn script_requires_a_shell_body() {
        let error = GdsShell::new()
            .run_script(&ShellScript::new("missing-body"))
            .expect_err("empty shell must fail");

        assert!(matches!(error, ShellScriptError::MissingBody));
    }
}
