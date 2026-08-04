//! Canonical Form-to-Shell procedure mediation.

use std::sync::Arc;

use serde::Serialize;
use serde_json::json;
use serde_json::Value;

use crate::procedures::pipelines::LocalPipelinesProcedureFacade;
use crate::procedures::GraphFacade;
use crate::projection::eval::algorithm::ExecutionMode;
use crate::shell::ShellAddress;
use crate::shell::ShellAlgebra;
use crate::shell::ShellComponentMode;
use crate::shell::ShellComponentPlan;
use crate::shell::ShellPipeline;
use crate::shell::ShellProcedureEvaluator;
use crate::shell::ShellProcedureResult;
use crate::shell::ShellRegister;
use crate::types::catalog::GraphCatalog;

#[derive(Debug, Clone)]
pub struct FormBusSubmission {
    pub graph_name: String,
    pub operation: String,
    pub payload: Value,
    pub execution_mode: ExecutionMode,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FormBusReceipt {
    pub service: &'static str,
    pub runtime: &'static str,
    pub operation: String,
    pub graph_name: String,
    pub component: String,
    pub mode: ShellComponentMode,
}

#[derive(Debug, Default)]
pub struct FormBusNexus;

impl FormBusNexus {
    pub fn execute_pagerank(
        submission: &FormBusSubmission,
        catalog: Arc<dyn GraphCatalog>,
    ) -> Result<Value, FormBusNexusError> {
        if !submission.operation.eq_ignore_ascii_case("pagerank") {
            return Err(FormBusNexusError::UnsupportedOperation(
                submission.operation.clone(),
            ));
        }

        let plan = Self::compile_pagerank_plan(submission)?;
        let graph_store = catalog
            .get(&submission.graph_name)
            .ok_or_else(|| FormBusNexusError::GraphNotFound(submission.graph_name.clone()))?;
        let runtime = ShellProcedureEvaluator::new(
            GraphFacade::new(graph_store),
            LocalPipelinesProcedureFacade::default(),
        );
        let result = runtime
            .invoke_plan(&plan)
            .map_err(|error| FormBusNexusError::Shell(error.to_string()))?;
        let invocation = result
            .invocations()
            .first()
            .ok_or(FormBusNexusError::MissingInvocation)?;
        let receipt = FormBusReceipt {
            service: "form.shell",
            runtime: "ShellProcedureRuntime",
            operation: submission.operation.clone(),
            graph_name: submission.graph_name.clone(),
            component: invocation.component().as_str().to_string(),
            mode: invocation.mode(),
        };
        let result = pagerank_result_json(invocation.result())?;

        Ok(json!({
            "ok": true,
            "op": submission.operation,
            "result": result,
            "busReceipt": receipt,
        }))
    }

    pub fn compile_pagerank_plan(
        submission: &FormBusSubmission,
    ) -> Result<ShellComponentPlan, FormBusNexusError> {
        let mode = shell_mode(submission.execution_mode)?;
        let mut builder = ShellComponentPlan::new(form_shell_address())
            .component("pagerank", mode)
            .map_err(|error| FormBusNexusError::Plan(error.to_string()))?;

        let payload = submission
            .payload
            .as_object()
            .ok_or(FormBusNexusError::InvalidPayload)?;
        for (key, value) in payload {
            if key != "op" && key != "graphName" && key != "graph_name" {
                builder = builder.with_input(key, value.clone());
            }
        }

        Ok(builder.finish())
    }
}

fn form_shell_address() -> ShellAddress {
    ShellAddress::new(
        ShellRegister::Unified,
        ShellPipeline::ModelFeaturePlan,
        ShellAlgebra::ProgramFeature,
    )
}

fn shell_mode(mode: ExecutionMode) -> Result<ShellComponentMode, FormBusNexusError> {
    match mode {
        ExecutionMode::Stream => Ok(ShellComponentMode::Stream),
        ExecutionMode::Stats => Ok(ShellComponentMode::Stats),
        ExecutionMode::MutateNodeProperty => Ok(ShellComponentMode::Mutate),
        ExecutionMode::WriteNodeProperty => Ok(ShellComponentMode::Write),
        ExecutionMode::Train
        | ExecutionMode::WriteRelationship
        | ExecutionMode::MutateRelationship => Err(FormBusNexusError::UnsupportedMode(mode)),
    }
}

fn pagerank_result_json(result: &ShellProcedureResult) -> Result<Value, FormBusNexusError> {
    let value = match result {
        ShellProcedureResult::PageRankStream(rows) => serde_json::to_value(rows),
        ShellProcedureResult::PageRankStats(stats) => serde_json::to_value(stats),
        ShellProcedureResult::PageRankEstimate(memory) => serde_json::to_value(memory),
        ShellProcedureResult::PageRankMutate(result) => serde_json::to_value(&result.summary),
        ShellProcedureResult::PageRankWrite(result) => serde_json::to_value(result),
        _ => return Err(FormBusNexusError::UnexpectedResult),
    };

    value.map_err(|error| FormBusNexusError::Serialization(error.to_string()))
}

#[derive(Debug, thiserror::Error)]
pub enum FormBusNexusError {
    #[error("Form Bus payload must be a JSON object")]
    InvalidPayload,

    #[error("Form Bus operation `{0}` is not supported by the PageRank Nexus")]
    UnsupportedOperation(String),

    #[error("graph `{0}` was not found for Form Bus execution")]
    GraphNotFound(String),

    #[error("PageRank cannot be mediated through Shell in execution mode {0:?}")]
    UnsupportedMode(ExecutionMode),

    #[error("could not compile Shell Component plan: {0}")]
    Plan(String),

    #[error("Shell procedure execution failed: {0}")]
    Shell(String),

    #[error("Shell procedure plan produced no invocation")]
    MissingInvocation,

    #[error("Shell procedure returned a non-PageRank result")]
    UnexpectedResult,

    #[error("could not serialize Shell procedure result: {0}")]
    Serialization(String),
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::types::catalog::InMemoryGraphCatalog;
    use crate::types::prelude::DefaultGraphStore;
    use crate::types::random::RandomGraphConfig;
    use crate::types::random::RandomRelationshipConfig;

    use super::*;

    #[test]
    fn compiles_pagerank_submission_into_program_feature_shell_plan() {
        let submission = FormBusSubmission {
            graph_name: "social".to_string(),
            operation: "pagerank".to_string(),
            payload: json!({
                "op": "pagerank",
                "graphName": "social",
                "maxIterations": 12,
                "dampingFactor": 0.9,
            }),
            execution_mode: ExecutionMode::Stream,
        };

        let plan = FormBusNexus::compile_pagerank_plan(&submission).unwrap();

        assert_eq!(plan.origin(), form_shell_address());
        assert_eq!(plan.len(), 1);
        assert_eq!(
            plan.calls()[0].component.as_str(),
            "gds.algorithms.centrality.pagerank"
        );
        assert_eq!(plan.calls()[0].mode, ShellComponentMode::Stream);
        assert_eq!(
            plan.calls()[0].inputs.get("maxIterations"),
            Some(&json!(12))
        );
        assert_eq!(
            plan.calls()[0].inputs.get("dampingFactor"),
            Some(&json!(0.9))
        );
        assert_eq!(plan.calls()[0].inputs.get("graphName"), None);
    }

    #[test]
    fn rejects_training_as_non_shell_pagerank_mode() {
        let submission = FormBusSubmission {
            graph_name: "social".to_string(),
            operation: "pagerank".to_string(),
            payload: json!({}),
            execution_mode: ExecutionMode::Train,
        };

        let error = FormBusNexus::compile_pagerank_plan(&submission).unwrap_err();

        assert!(matches!(
            error,
            FormBusNexusError::UnsupportedMode(ExecutionMode::Train)
        ));
    }

    #[test]
    fn executes_pagerank_through_shell_runtime_and_emits_receipt() {
        let config = RandomGraphConfig {
            seed: Some(7),
            node_count: 8,
            relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
            ..RandomGraphConfig::default()
        };
        let catalog = Arc::new(InMemoryGraphCatalog::new());
        catalog.set(
            "social",
            Arc::new(DefaultGraphStore::random(&config).unwrap()),
        );
        let submission = FormBusSubmission {
            graph_name: "social".to_string(),
            operation: "pagerank".to_string(),
            payload: json!({
                "maxIterations": 12,
                "dampingFactor": 0.85,
            }),
            execution_mode: ExecutionMode::Stream,
        };

        let response = FormBusNexus::execute_pagerank(&submission, catalog).unwrap();

        assert_eq!(response["ok"], true);
        assert_eq!(response["result"].as_array().unwrap().len(), 8);
        assert_eq!(response["busReceipt"]["service"], "form.shell");
        assert_eq!(response["busReceipt"]["runtime"], "ShellProcedureRuntime");
        assert_eq!(
            response["busReceipt"]["component"],
            "gds.algorithms.centrality.pagerank"
        );
    }

    #[test]
    fn reports_missing_graph_before_shell_invocation() {
        let submission = FormBusSubmission {
            graph_name: "missing".to_string(),
            operation: "pagerank".to_string(),
            payload: json!({}),
            execution_mode: ExecutionMode::Stream,
        };

        let error =
            FormBusNexus::execute_pagerank(&submission, Arc::new(InMemoryGraphCatalog::new()))
                .unwrap_err();

        assert!(matches!(error, FormBusNexusError::GraphNotFound(name) if name == "missing"));
    }
}
