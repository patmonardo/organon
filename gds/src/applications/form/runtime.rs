//! Empirical adapters for linked FormVM operations.
//!
//! Form core owns the instruction language. This module binds those instructions
//! to GraphFrame and TaskFrame because graph catalogs and runtime resources are
//! application/kernel concerns rather than PureForm state.

use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::collections::graphframe::feature_grammar::{
    validate_graph_feature_grammar, GraphFeatureCardinality, GraphFeatureGrammarForm,
    GraphFeatureRule, GraphFeatureStratum, GraphFeatureValueType,
};
use crate::collections::graphframe::{
    GraphAgentProcessingContract, GraphAutomationProfile, GraphFeatureGrammarExpr, GraphFrame,
    GraphModelExpr, GraphPlanExpr, GraphProcedureExpr, GraphTaskDaemon,
    GraphTaskDaemonRuntimeBundle, GraphTaskRuntimeProfile,
};
use crate::form::{FormVmOperationKind, LinkedExecutableForm};
use crate::projection::eval::algorithm::ExecutionMode;
use crate::shell::ShellComponentMode;
use crate::task::concurrency::TerminationFlag;
use crate::types::catalog::GraphCatalog;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FormVmOperationStatus {
    Prepared,
    Delegated,
    Deferred,
    Executed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormVmOperationReceipt {
    pub operation_sequence: u64,
    pub operation: String,
    pub status: FormVmOperationStatus,
    pub evidence_kind: String,
    pub evidence_reference: String,
}

pub struct FormVmRuntimePreparation {
    graph_contract: Option<GraphAgentProcessingContract>,
    task_submission: Option<FormTaskSubmissionPlan>,
    task_job_receipt: Option<FormTaskJobReceipt>,
    pub receipts: Vec<FormVmOperationReceipt>,
}

impl FormVmRuntimePreparation {
    pub fn graph_contract(&self) -> Option<&GraphAgentProcessingContract> {
        self.graph_contract.as_ref()
    }

    pub fn task_frame(
        &self,
    ) -> Option<&crate::task::frame::TaskFrame<crate::collections::graphframe::GraphExecutionIntent>>
    {
        self.graph_contract
            .as_ref()
            .map(GraphAgentProcessingContract::task_frame)
    }

    pub fn task_submission(&self) -> Option<&FormTaskSubmissionPlan> {
        self.task_submission.as_ref()
    }

    pub fn task_job_receipt(&self) -> Option<&FormTaskJobReceipt> {
        self.task_job_receipt.as_ref()
    }

    pub fn execute_task(
        &mut self,
        graph_name: &str,
        catalog: Arc<dyn GraphCatalog>,
        termination: TerminationFlag,
    ) -> Result<Option<&FormTaskJobReceipt>, String> {
        let Some(submission_plan) = self.task_submission.as_mut() else {
            return Ok(None);
        };
        let contract = self.graph_contract.take().ok_or_else(|| {
            "Task daemon submission requires an owned Graph processing contract".to_string()
        })?;
        let store = catalog
            .get(graph_name)
            .ok_or_else(|| format!("graph `{graph_name}` was not found for task execution"))?;
        let submission = contract.into_task_daemon_submission(submission_plan.owner.clone());
        let runtime =
            GraphTaskDaemonRuntimeBundle::for_profile(store, GraphTaskRuntimeProfile::Analytics);
        let receipt = GraphTaskDaemon::new()
            .run_with_runtime_profile(
                submission,
                &runtime,
                GraphTaskRuntimeProfile::Analytics,
                termination,
            )
            .map_err(|error| format!("Graph task daemon rejected submission: {error}"))?;
        submission_plan.submitted = true;
        self.task_job_receipt = Some(FormTaskJobReceipt {
            job_id: receipt.job_id().to_string(),
            task_name: receipt.task_name().to_string(),
            owner: receipt.owner().to_string(),
            state: format!("{:?}", receipt.state()).to_ascii_lowercase(),
            succeeded: matches!(receipt.state(), crate::task::job::TaskJobState::Succeeded),
            error: receipt.error().map(ToOwned::to_owned),
            trace: receipt.trace().to_vec(),
            invocation_count: receipt
                .output()
                .map(|output| output.invocations().len())
                .unwrap_or_default(),
        });
        if let Some(operation_receipt) = self
            .receipts
            .iter_mut()
            .find(|receipt| receipt.operation == "execute_task")
        {
            operation_receipt.status = FormVmOperationStatus::Executed;
            operation_receipt.evidence_reference = format!(
                "{}:{}:{}:submitted=true",
                submission_plan.runtime, submission_plan.route, submission_plan.task_name
            );
        }
        Ok(self.task_job_receipt.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormTaskJobReceipt {
    pub job_id: String,
    pub task_name: String,
    pub owner: String,
    pub state: String,
    pub succeeded: bool,
    pub error: Option<String>,
    pub trace: Vec<String>,
    pub invocation_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormTaskSubmissionPlan {
    pub runtime: String,
    pub route: String,
    pub task_name: String,
    pub owner: String,
    pub submitted: bool,
}

pub trait FormTaskRuntime: Send + Sync {
    fn prepare(
        &self,
        contract: &GraphAgentProcessingContract,
        runtime: &str,
        owner: &str,
    ) -> Result<FormTaskSubmissionPlan, String>;
}

#[derive(Debug, Clone, Default)]
pub struct DeferredGraphTaskRuntime;

impl FormTaskRuntime for DeferredGraphTaskRuntime {
    fn prepare(
        &self,
        contract: &GraphAgentProcessingContract,
        runtime: &str,
        owner: &str,
    ) -> Result<FormTaskSubmissionPlan, String> {
        Ok(FormTaskSubmissionPlan {
            runtime: runtime.to_string(),
            route: contract.daemon_route().as_str().to_string(),
            task_name: contract
                .task_frame()
                .task_name()
                .unwrap_or("graphframe::formvm::anonymous")
                .to_string(),
            owner: owner.to_string(),
            submitted: false,
        })
    }
}

pub struct GraphFormRuntimeRequest<'a> {
    pub executable: &'a LinkedExecutableForm,
    pub graph_name: &'a str,
    pub default_input: &'a Value,
    pub op_inputs: &'a HashMap<String, Value>,
    pub execution_mode: ExecutionMode,
    pub catalog: Arc<dyn GraphCatalog>,
}

pub trait GraphFormRuntimeProvider: Send + Sync {
    fn compile(
        &self,
        request: GraphFormRuntimeRequest<'_>,
    ) -> Result<GraphAgentProcessingContract, String>;
}

#[derive(Debug, Clone, Default)]
pub struct DefaultGraphFormRuntimeProvider;

impl GraphFormRuntimeProvider for DefaultGraphFormRuntimeProvider {
    fn compile(
        &self,
        request: GraphFormRuntimeRequest<'_>,
    ) -> Result<GraphAgentProcessingContract, String> {
        let store = request
            .catalog
            .get(request.graph_name)
            .ok_or_else(|| format!("graph `{}` was not found for FormVM", request.graph_name))?;
        let frame = GraphFrame::from_store(store)
            .map_err(|error| format!("GraphFrame determination failed: {error}"))?;
        let grammar_name = format!("formvm.{}", request.executable.form_id);
        let mut plan = frame
            .plan()
            .push_expr(GraphModelExpr::new(format!(
                "{}.runtime-model",
                request.executable.form_id
            )))
            .push_expr(GraphFeatureGrammarExpr::new(grammar_name.clone()).with_version("v1"))
            .push_expr(GraphPlanExpr::new(format!(
                "{}.runtime-plan",
                request.executable.form_id
            )));
        for candidate in &request.executable.operations {
            let FormVmOperationKind::InvokeOperator { service, operator } = &candidate.kind else {
                continue;
            };
            if service != "form.algorithms" {
                continue;
            }
            let alias = algorithm_alias(operator);
            let mut procedure =
                GraphProcedureExpr::new(alias.clone(), shell_mode(request.execution_mode)?);
            if let Some(inputs) = request.op_inputs.get(&alias).and_then(Value::as_object) {
                for (key, value) in inputs {
                    procedure = procedure.with_input(key, value.clone());
                }
            }
            plan = plan.procedure(procedure);
        }

        let grammar = validate_graph_feature_grammar(
            GraphFeatureGrammarForm::new(grammar_name, "v1").with_feature_rule(
                GraphFeatureRule::new(
                    GraphFeatureStratum::Graph,
                    "formvm_runtime",
                    GraphFeatureValueType::Symbolic,
                    true,
                    GraphFeatureCardinality::One,
                ),
            ),
        )
        .map_err(|error| format!("GraphFrame feature grammar failed: {error}"))?;
        let concurrency = request
            .default_input
            .get("concurrency")
            .and_then(Value::as_u64)
            .and_then(|value| usize::try_from(value).ok())
            .unwrap_or(1)
            .max(1);
        plan.compile_agent_processing_contract(
            &grammar,
            GraphAutomationProfile::AgentAnalytics,
            concurrency,
        )
        .map_err(|error| format!("GraphFrame processing-contract compilation failed: {error}"))
    }
}

pub fn prepare_linked_form(
    executable: &LinkedExecutableForm,
    default_input: &Value,
    op_inputs: &HashMap<String, Value>,
    execution_mode: ExecutionMode,
    owner: &str,
    catalog: Arc<dyn GraphCatalog>,
) -> Result<FormVmRuntimePreparation, String> {
    prepare_linked_form_with_provider(
        executable,
        default_input,
        op_inputs,
        execution_mode,
        owner,
        catalog,
        &DefaultGraphFormRuntimeProvider,
        &DeferredGraphTaskRuntime,
    )
}

pub fn prepare_linked_form_with_provider(
    executable: &LinkedExecutableForm,
    default_input: &Value,
    op_inputs: &HashMap<String, Value>,
    execution_mode: ExecutionMode,
    owner: &str,
    catalog: Arc<dyn GraphCatalog>,
    graph_provider: &dyn GraphFormRuntimeProvider,
    task_runtime: &dyn FormTaskRuntime,
) -> Result<FormVmRuntimePreparation, String> {
    let mut graph_contract = None;
    let mut task_submission = None;
    let mut receipts = Vec::new();

    for operation in &executable.operations {
        match &operation.kind {
            FormVmOperationKind::EstablishPrinciple => receipts.push(receipt(
                operation.sequence,
                "establish_principle",
                FormVmOperationStatus::Prepared,
                "pure_form_principle",
                &executable.form_id,
            )),
            FormVmOperationKind::ProjectApplication { application_form } => {
                receipts.push(receipt(
                    operation.sequence,
                    "project_application",
                    FormVmOperationStatus::Prepared,
                    "application_form",
                    application_form,
                ));
            }
            FormVmOperationKind::DetermineGraph { binding } => {
                let graph_name = graph_name(default_input)?;
                let contract = graph_provider.compile(GraphFormRuntimeRequest {
                    executable,
                    graph_name,
                    default_input,
                    op_inputs,
                    execution_mode,
                    catalog: catalog.clone(),
                })?;
                receipts.push(receipt(
                    operation.sequence,
                    "determine_graph",
                    FormVmOperationStatus::Prepared,
                    "graph_processing_contract",
                    &format!(
                        "{binding}:{graph_name}:{}",
                        contract.transmission_spec().objective_identity
                    ),
                ));
                graph_contract = Some(contract);
            }
            FormVmOperationKind::ConstituteTask { binding } => {
                let contract = graph_contract.as_ref().ok_or_else(|| {
                    "TaskFrame constitution requires a prior Graph processing contract".to_string()
                })?;
                let task_reference = contract
                    .task_frame()
                    .task_name()
                    .map(ToOwned::to_owned)
                    .unwrap_or_else(|| format!("{binding}:anonymous"));
                receipts.push(receipt(
                    operation.sequence,
                    "constitute_task",
                    FormVmOperationStatus::Prepared,
                    "task_frame",
                    &task_reference,
                ));
            }
            FormVmOperationKind::ExecuteTask { runtime } => {
                let contract = graph_contract.as_ref().ok_or_else(|| {
                    "Task execution requires a prior Graph processing contract".to_string()
                })?;
                let submission = task_runtime.prepare(contract, runtime, owner)?;
                receipts.push(receipt(
                    operation.sequence,
                    "execute_task",
                    FormVmOperationStatus::Deferred,
                    "task_daemon_submission",
                    &format!(
                        "{}:{}:{}:submitted={}",
                        submission.runtime,
                        submission.route,
                        submission.task_name,
                        submission.submitted
                    ),
                ));
                task_submission = Some(submission);
            }
            FormVmOperationKind::InvokeOperator { service, operator } => receipts.push(receipt(
                operation.sequence,
                "invoke_operator",
                FormVmOperationStatus::Delegated,
                service,
                operator,
            )),
            FormVmOperationKind::CollectEvidence { binding } => receipts.push(receipt(
                operation.sequence,
                "collect_evidence",
                FormVmOperationStatus::Prepared,
                "evidence_contract",
                binding,
            )),
            FormVmOperationKind::ReturnForm { binding } => receipts.push(receipt(
                operation.sequence,
                "return_form",
                FormVmOperationStatus::Prepared,
                "return_contract",
                binding,
            )),
            FormVmOperationKind::DeferredCompatibility { pattern } => receipts.push(receipt(
                operation.sequence,
                "deferred_compatibility",
                FormVmOperationStatus::Deferred,
                "compatibility_pattern",
                pattern,
            )),
        }
    }

    Ok(FormVmRuntimePreparation {
        graph_contract,
        task_submission,
        task_job_receipt: None,
        receipts,
    })
}

fn receipt(
    operation_sequence: u64,
    operation: &str,
    status: FormVmOperationStatus,
    evidence_kind: &str,
    evidence_reference: &str,
) -> FormVmOperationReceipt {
    FormVmOperationReceipt {
        operation_sequence,
        operation: operation.to_string(),
        status,
        evidence_kind: evidence_kind.to_string(),
        evidence_reference: evidence_reference.to_string(),
    }
}

fn graph_name(default_input: &Value) -> Result<&str, String> {
    default_input
        .get("graphName")
        .or_else(|| default_input.get("graph_name"))
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .ok_or_else(|| "DetermineGraph requires default_input.graphName".to_string())
}

fn algorithm_alias(pattern: &str) -> String {
    for prefix in ["algo.", "algorithm.", "applications.algorithms."] {
        if let Some(alias) = pattern.strip_prefix(prefix) {
            return alias.to_string();
        }
    }
    pattern.to_string()
}

fn shell_mode(mode: ExecutionMode) -> Result<ShellComponentMode, String> {
    match mode {
        ExecutionMode::Stream => Ok(ShellComponentMode::Stream),
        ExecutionMode::Stats => Ok(ShellComponentMode::Stats),
        ExecutionMode::MutateNodeProperty => Ok(ShellComponentMode::Mutate),
        ExecutionMode::WriteNodeProperty => Ok(ShellComponentMode::Write),
        ExecutionMode::Train
        | ExecutionMode::WriteRelationship
        | ExecutionMode::MutateRelationship => Err(format!(
            "GraphFrame does not support execution mode {mode:?}"
        )),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::json;

    use super::*;
    use crate::form::{
        ApplicationForm, Context, FormLinker, FormShape, Morph, ProgramSpec, Shape, Specification,
    };
    use crate::types::catalog::InMemoryGraphCatalog;
    use crate::types::graph_store::DefaultGraphStore;
    use crate::types::random::RandomGraphConfig;

    fn linked_organon() -> LinkedExecutableForm {
        let program = ProgramSpec::new(
            FormShape::new(
                Shape::default(),
                Context::default(),
                Morph::new(vec!["algo.pagerank".to_string()]),
            ),
            Specification::new("form.organon".to_string(), None, HashMap::new()),
            vec![],
            vec![ApplicationForm::organon()],
            vec!["organon".to_string()],
        );
        FormLinker::new()
            .link(&program, Some("graph://runtime".to_string()))
            .expect("Organon Form should link")
    }

    #[test]
    fn prepares_real_graph_intent_and_task_frame_from_linked_operations() {
        let catalog = Arc::new(InMemoryGraphCatalog::new());
        catalog.set(
            "runtime",
            Arc::new(
                DefaultGraphStore::random(&RandomGraphConfig::seeded(42))
                    .expect("random graph should build"),
            ),
        );

        let mut preparation = prepare_linked_form(
            &linked_organon(),
            &json!({"graphName": "runtime", "concurrency": 2}),
            &HashMap::new(),
            ExecutionMode::Stream,
            "organon",
            catalog.clone(),
        )
        .expect("linked Form should prepare runtime frames");

        let contract = preparation
            .graph_contract()
            .expect("Graph processing contract should be determined");
        let intent = contract.task_frame().program();
        assert_eq!(intent.program().len(), 1);
        assert_eq!(
            intent.program().calls()[0].component.as_str(),
            "gds.algorithms.centrality.pagerank"
        );

        let task_frame = preparation
            .task_frame()
            .expect("TaskFrame should be constituted");
        assert_eq!(task_frame.workflow().frames().len(), 2);
        assert!(preparation.receipts.iter().any(|receipt| {
            receipt.operation == "determine_graph"
                && receipt.status == FormVmOperationStatus::Prepared
                && receipt.evidence_kind == "graph_processing_contract"
        }));
        let submission = preparation
            .task_submission()
            .expect("task submission should be planned");
        assert_eq!(submission.runtime, "graph-task-daemon");
        assert_eq!(submission.route, "algorithm");
        assert!(!submission.submitted);

        let job = preparation
            .execute_task("runtime", catalog, TerminationFlag::running_true())
            .expect("task daemon should accept linked Form")
            .expect("task receipt should be produced");
        assert!(job.succeeded, "task failed: {:?}", job.error);
        assert!(!job.job_id.is_empty());
        assert_eq!(job.invocation_count, 1);
        assert!(preparation.task_submission().unwrap().submitted);
        assert!(preparation.receipts.iter().any(|receipt| {
            receipt.operation == "constitute_task"
                && receipt.status == FormVmOperationStatus::Prepared
        }));
    }
}
