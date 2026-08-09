use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::applications::form::evidence::{
    DeferredFormEvidenceProvider, FormEvidenceCollectionRequest, FormEvidenceProvider,
};
use crate::applications::form::runtime::prepare_linked_form;
use crate::applications::form::runtime::FormRuntimeEvidenceExpectation;
use crate::applications::form::runtime::FormTaskJobReceipt;
use crate::applications::form::runtime::FormVmOperationReceipt;
use crate::form::FormVmEvidenceRef;
use crate::form::ProgramSpec;
use crate::form::{
    FormControlCommand, FormControlReceipt, FormEvidenceContract, FormReturnContract, FormRunId,
    FormRunRegistry, FormRunRegistryError, FormRunReport, FormTerminalDisposition, FormVmOperation,
    FormVmOperationKind, InMemoryFormRunRegistry, ManagedFormRun,
};
use crate::projection::eval::algorithm::ExecutionMode;
use crate::task::concurrency::TerminationFlag;
use crate::types::catalog::GraphCatalog;

use super::concept::{
    appearance_from_input, FormContract, FormEvalError, FormEvalRequest, FormEvalResult,
    FormEvaluator, FormPreEvalTrace, MonadicEvaluationSlot, MonadicEvaluationState,
};
use super::mediation::{
    apply_execution_plan, apply_task_job_receipt, ProgramFormApplyBackend, ProgramFormApplyPrint,
    ProgramFormFailure,
};

/// Judgement layer: orchestrates Eval(Form) -> Apply(Form) -> Print.
pub struct ProgramFormApi {
    evaluator: FormEvaluator,
    run_registry: Arc<dyn FormRunRegistry>,
    evidence_provider: Arc<dyn FormEvidenceProvider>,
}

impl ProgramFormApi {
    pub fn new() -> Self {
        Self {
            evaluator: FormEvaluator::new(),
            run_registry: Arc::new(InMemoryFormRunRegistry::new()),
            evidence_provider: Arc::new(DeferredFormEvidenceProvider),
        }
    }

    pub fn with_run_registry(run_registry: Arc<dyn FormRunRegistry>) -> Self {
        Self {
            evaluator: FormEvaluator::new(),
            run_registry,
            evidence_provider: Arc::new(DeferredFormEvidenceProvider),
        }
    }

    pub fn with_evidence_provider(mut self, provider: Arc<dyn FormEvidenceProvider>) -> Self {
        self.evidence_provider = provider;
        self
    }

    pub fn inspect_run(&self, run_id: &FormRunId) -> Option<ManagedFormRun> {
        self.run_registry.get(run_id)
    }

    pub fn inspect_run_report(&self, run_id: &FormRunId) -> Option<FormRunReport> {
        self.run_registry.get(run_id).map(|run| run.report())
    }

    /// Applies an operational control command to a managed Form run.
    ///
    /// The returned receipt records whether the command crossed the lifecycle
    /// boundary; rejected commands are still rationally inspectable outcomes.
    pub fn control_run(
        &self,
        run_id: &FormRunId,
        command: FormControlCommand,
    ) -> Result<FormControlReceipt, FormRunRegistryError> {
        self.run_registry.control(run_id, command)
    }

    pub fn evaluate(&self, program: ProgramSpec) -> Result<FormEvalResult, FormProgramError> {
        self.evaluator
            .evaluate(FormEvalRequest::new(program))
            .map_err(FormProgramError::Evaluate)
    }

    pub fn evaluate_apply_print(
        &self,
        request: ProgramFormRequest,
        catalog: Arc<dyn GraphCatalog>,
    ) -> Result<ProgramFormPrint, FormProgramError> {
        let ProgramFormRequest {
            program,
            default_input,
            op_inputs,
            username,
            execution_mode,
            apply_backend,
            fail_fast,
        } = request;

        let appearance = appearance_from_input(&default_input);
        let eval = self
            .evaluator
            .evaluate_with_appearance(FormEvalRequest::new(program), appearance.clone())
            .map_err(FormProgramError::Evaluate)?;

        let mut vm_lifecycle = eval.vm_lifecycle;
        let linked_form = LinkedFormPrint {
            form_id: eval.linked_form_id,
            operations: eval.vm_operations,
            link_report: eval.link_report,
        };
        vm_lifecycle
            .begin_evaluation()
            .map_err(|error| FormProgramError::Evaluate(FormEvalError::Lifecycle(error)))?;

        let mut runtime_preparation = prepare_linked_form(
            &eval.executable,
            &default_input,
            &op_inputs,
            execution_mode,
            &username,
            catalog.clone(),
        )
        .map_err(FormProgramError::Runtime)?;

        let task_managed = runtime_preparation.task_submission().is_some();
        let termination = TerminationFlag::running_true();
        if task_managed {
            self.run_registry
                .register_active(
                    ManagedFormRun::active(linked_form.form_id.clone(), vm_lifecycle.clone()),
                    termination.clone(),
                )
                .map_err(|error| {
                    FormProgramError::Runtime(format!(
                        "run registry rejected active Form: {error:?}"
                    ))
                })?;
        }

        let task_job_receipt = if task_managed {
            let graph_name = appearance.as_deref().ok_or_else(|| {
                FormProgramError::Runtime(
                    "task execution requires an appearance or graphName".to_string(),
                )
            })?;
            runtime_preparation
                .execute_task(graph_name, catalog.clone(), termination)
                .map_err(FormProgramError::Runtime)?
                .cloned()
        } else {
            None
        };
        for receipt in &runtime_preparation.receipts {
            vm_lifecycle.record_operation_mediation(
                receipt.operation_sequence,
                FormVmEvidenceRef::new(
                    receipt.evidence_kind.clone(),
                    receipt.evidence_reference.clone(),
                ),
            );
        }
        if let Some(receipt) = &task_job_receipt {
            vm_lifecycle
                .record_task_observation(
                    receipt.job_id.clone(),
                    receipt.state.clone(),
                    receipt.invocation_count,
                )
                .map_err(|error| FormProgramError::Evaluate(FormEvalError::Lifecycle(error)))?;
        }

        let apply = match &task_job_receipt {
            Some(receipt) => apply_task_job_receipt(&eval.executable.operations, receipt),
            None => apply_execution_plan(
                &eval.executable.operations,
                &default_input,
                &op_inputs,
                &username,
                execution_mode,
                apply_backend,
                fail_fast,
                catalog,
            )?,
        };

        let completion_evidence = [
            FormVmEvidenceRef::new("executed_operations", apply.executed.len().to_string()),
            FormVmEvidenceRef::new("failed_operations", apply.failed.len().to_string()),
        ];
        for evidence in &completion_evidence {
            vm_lifecycle.add_evidence(evidence.clone());
        }
        let return_contract = form_return_contract(
            &eval.executable.operations,
            task_managed,
            runtime_preparation.evidence_expectation(),
        );
        if let Some(contract) = &return_contract {
            let collected = self
                .evidence_provider
                .collect(&FormEvidenceCollectionRequest {
                    run_id: FormRunId::new(vm_lifecycle.run_id.clone()),
                    linked_form_id: linked_form.form_id.clone(),
                    binding: contract.evidence.binding.clone(),
                    task_job_id: task_job_receipt
                        .as_ref()
                        .map(|receipt| receipt.job_id.clone()),
                })
                .map_err(|error| {
                    FormProgramError::Runtime(format!(
                        "evidence provider failed to collect Form references: {error}"
                    ))
                })?;
            for evidence in collected {
                vm_lifecycle.add_evidence(evidence);
            }
        }
        let return_judgment = return_contract
            .as_ref()
            .map(|contract| contract.judge(apply.failed.is_empty(), &vm_lifecycle.evidence));
        let canceled = task_job_receipt
            .as_ref()
            .is_some_and(|receipt| receipt.state == "canceled");
        let return_satisfied = return_judgment
            .as_ref()
            .is_none_or(|judgment| judgment.satisfied);
        let terminal_succeeded = !canceled && apply.failed.is_empty() && return_satisfied;
        let terminal_disposition = FormTerminalDisposition::determine(
            canceled,
            task_job_receipt.as_ref().map(|receipt| receipt.succeeded),
            apply.failed.is_empty(),
            return_satisfied,
            vm_lifecycle.fault.is_some(),
        );
        let monadic_state = if terminal_succeeded {
            MonadicEvaluationState::Succeeded
        } else {
            MonadicEvaluationState::Failed
        };
        if canceled {
            vm_lifecycle
                .cancel()
                .map_err(|error| FormProgramError::Evaluate(FormEvalError::Lifecycle(error)))?;
        } else {
            vm_lifecycle
                .complete_evaluation(terminal_succeeded, std::iter::empty())
                .map_err(|error| FormProgramError::Evaluate(FormEvalError::Lifecycle(error)))?;
        }

        let mut pre_eval = eval.pre_eval;
        pre_eval.set_monadic_state(monadic_state.clone());

        let organic_unity = OrganicUnityCheck::evaluate(
            apply_backend,
            &ProgramFormEvalPrint {
                selected_forms: eval.plan.selected_forms.clone(),
                patterns: eval.plan.patterns.clone(),
            },
            &apply,
            &pre_eval,
            &MonadicEvaluationSlot {
                state: monadic_state.clone(),
            },
            terminal_succeeded,
        );

        let graph_contracts = runtime_preparation
            .receipts
            .iter()
            .filter(|receipt| receipt.evidence_kind == "graph_processing_contract")
            .map(|receipt| receipt.evidence_reference.clone())
            .collect();
        let task_frames = runtime_preparation
            .receipts
            .iter()
            .filter(|receipt| receipt.evidence_kind == "task_frame")
            .map(|receipt| receipt.evidence_reference.clone())
            .collect();
        let task_submissions = runtime_preparation
            .receipts
            .iter()
            .filter(|receipt| receipt.evidence_kind == "task_daemon_submission")
            .map(|receipt| receipt.evidence_reference.clone())
            .collect();
        let task_jobs = task_job_receipt
            .iter()
            .map(|receipt| receipt.job_id.clone())
            .collect();
        let managed_run = ManagedFormRun::returned(
            linked_form.form_id.clone(),
            vm_lifecycle.clone(),
            graph_contracts,
            task_frames,
            task_submissions,
            task_jobs,
            return_contract,
            return_judgment,
            terminal_disposition,
        );
        let run_report = managed_run.report();
        if task_managed {
            self.run_registry.replace(managed_run).map_err(|error| {
                FormProgramError::Runtime(format!(
                    "run registry rejected Form completion: {error:?}"
                ))
            })?;
        } else {
            self.run_registry.register(managed_run).map_err(|error| {
                FormProgramError::Runtime(format!("run registry rejected Form: {error:?}"))
            })?;
        }

        Ok(ProgramFormPrint {
            ok: terminal_succeeded,
            backend: apply_backend,
            eval: ProgramFormEvalPrint {
                selected_forms: eval.plan.selected_forms,
                patterns: eval.plan.patterns,
            },
            apply,
            contract: eval.contract,
            pre_eval,
            evaluation_slot: MonadicEvaluationSlot {
                state: monadic_state,
            },
            organic_unity,
            vm_lifecycle,
            linked_form,
            operation_receipts: runtime_preparation.receipts,
            task_job_receipt,
            run_report,
        })
    }

    /// RootAgent establishes a projection world; SingleAgents execute ProgramForms inside it.
    pub fn start_agent_session(
        &self,
        projection: RootProjectionContext,
        single_agent: impl Into<String>,
    ) -> ProgramFormSession {
        ProgramFormSession::for_agent_framework(projection, single_agent)
    }

    /// External/public callers can execute in the same world model through facade semantics.
    pub fn start_public_session(
        &self,
        projection: RootProjectionContext,
        single_agent: impl Into<String>,
    ) -> ProgramFormSession {
        ProgramFormSession::for_public_facade(projection, single_agent)
    }

    /// Execute a ProgramForm request within a pre-conceived projection world.
    pub fn execute_session(
        &self,
        session: &ProgramFormSession,
        program: ProgramSpec,
        catalog: Arc<dyn GraphCatalog>,
    ) -> Result<ProgramFormPrint, FormProgramError> {
        let mut request = match session.apply_backend {
            ProgramFormApplyBackend::ExecuteSpec => {
                ProgramFormRequest::for_agent_framework(program, session.single_agent.clone())
            }
            ProgramFormApplyBackend::DirectCompute => {
                ProgramFormRequest::for_public_facade(program, session.single_agent.clone())
            }
        };

        request.execution_mode = session.execution_mode;
        request.fail_fast = session.fail_fast;
        request.default_input = session.default_input();

        self.evaluate_apply_print(request, catalog)
    }
}

fn form_return_contract(
    operations: &[FormVmOperation],
    task_managed: bool,
    runtime_expectation: Option<&FormRuntimeEvidenceExpectation>,
) -> Option<FormReturnContract> {
    let return_binding = operations
        .iter()
        .find_map(|operation| match &operation.kind {
            FormVmOperationKind::ReturnForm { binding } => Some(binding.clone()),
            _ => None,
        })?;
    let evidence_binding = operations
        .iter()
        .find_map(|operation| match &operation.kind {
            FormVmOperationKind::CollectEvidence { binding } => Some(binding.clone()),
            _ => None,
        })
        .unwrap_or_else(|| "formvm.lifecycle-evidence".to_string());
    let mut required_kinds = vec![
        "executed_operations".to_string(),
        "failed_operations".to_string(),
    ];
    if operations
        .iter()
        .any(|operation| matches!(operation.kind, FormVmOperationKind::CollectEvidence { .. }))
    {
        required_kinds.push("evidence_contract".to_string());
    }
    if task_managed {
        required_kinds.extend([
            "graph_processing_contract".to_string(),
            "task_frame".to_string(),
            "task_daemon_submission".to_string(),
            "task_job".to_string(),
        ]);
    }
    let requires_persisted_artifact = runtime_expectation
        .is_some_and(FormRuntimeEvidenceExpectation::requires_persisted_artifact);
    if requires_persisted_artifact {
        required_kinds.push("dataset_artifact".to_string());
    }
    Some(FormReturnContract {
        binding: return_binding,
        require_successful_execution: true,
        evidence: FormEvidenceContract {
            binding: evidence_binding,
            required_kinds,
            expected_outputs: runtime_expectation
                .map(|expectation| expectation.expected_outputs.clone())
                .unwrap_or_default(),
            requires_persisted_artifact,
        },
    })
}

impl Default for ProgramFormApi {
    fn default() -> Self {
        Self::new()
    }
}

/// Projection world established by a RootAgent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RootProjectionContext {
    pub projection_id: String,
    pub graph_name: String,
    pub root_agent: String,
    pub pipeline_path: Vec<String>,
}

impl RootProjectionContext {
    pub fn new(
        projection_id: impl Into<String>,
        graph_name: impl Into<String>,
        root_agent: impl Into<String>,
    ) -> Self {
        Self {
            projection_id: projection_id.into(),
            graph_name: graph_name.into(),
            root_agent: root_agent.into(),
            pipeline_path: vec![
                "codegen".to_string(),
                "factory".to_string(),
                "eval".to_string(),
            ],
        }
    }

    pub fn with_pipeline_path(mut self, pipeline_path: Vec<String>) -> Self {
        self.pipeline_path = pipeline_path;
        self
    }
}

/// Agent session scoped to a single projection world.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgramFormSession {
    pub projection: RootProjectionContext,
    pub single_agent: String,
    pub apply_backend: ProgramFormApplyBackend,
    pub execution_mode: ExecutionMode,
    pub fail_fast: bool,
}

impl ProgramFormSession {
    pub fn for_agent_framework(
        projection: RootProjectionContext,
        single_agent: impl Into<String>,
    ) -> Self {
        Self {
            projection,
            single_agent: single_agent.into(),
            apply_backend: ProgramFormApplyBackend::ExecuteSpec,
            execution_mode: ExecutionMode::Stream,
            fail_fast: true,
        }
    }

    pub fn for_public_facade(
        projection: RootProjectionContext,
        single_agent: impl Into<String>,
    ) -> Self {
        Self {
            projection,
            single_agent: single_agent.into(),
            apply_backend: ProgramFormApplyBackend::DirectCompute,
            execution_mode: ExecutionMode::Stream,
            fail_fast: true,
        }
    }

    fn default_input(&self) -> Value {
        json!({
            "graphName": self.projection.graph_name,
            "projectionId": self.projection.projection_id,
            "rootAgent": self.projection.root_agent,
            "pipelinePath": self.projection.pipeline_path,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ProgramFormRequest {
    pub program: ProgramSpec,
    pub default_input: Value,
    pub op_inputs: HashMap<String, Value>,
    pub username: String,
    pub execution_mode: ExecutionMode,
    pub apply_backend: ProgramFormApplyBackend,
    pub fail_fast: bool,
}

impl ProgramFormRequest {
    pub fn new(program: ProgramSpec) -> Self {
        Self {
            program,
            default_input: json!({}),
            op_inputs: HashMap::new(),
            username: "form-api".to_string(),
            execution_mode: ExecutionMode::Stream,
            apply_backend: ProgramFormApplyBackend::ExecuteSpec,
            fail_fast: true,
        }
    }

    /// Constructor for the Agent framework / FormDB world.
    ///
    /// This always chooses specification-driven execution (`ExecuteSpec`).
    pub fn for_agent_framework(program: ProgramSpec, username: impl Into<String>) -> Self {
        let mut request = Self::new(program);
        request.username = username.into();
        request.apply_backend = ProgramFormApplyBackend::ExecuteSpec;
        request
    }

    /// Constructor for external/public facade usage.
    ///
    /// This uses direct application compute dispatch (`DirectCompute`).
    pub fn for_public_facade(program: ProgramSpec, username: impl Into<String>) -> Self {
        let mut request = Self::new(program);
        request.username = username.into();
        request.apply_backend = ProgramFormApplyBackend::DirectCompute;
        request
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramFormPrint {
    pub ok: bool,
    pub backend: ProgramFormApplyBackend,
    pub eval: ProgramFormEvalPrint,
    pub apply: ProgramFormApplyPrint,
    pub contract: FormContract,
    pub pre_eval: FormPreEvalTrace,
    pub evaluation_slot: MonadicEvaluationSlot,
    pub organic_unity: OrganicUnityReport,
    pub vm_lifecycle: crate::form::FormVmLifecycle,
    pub linked_form: LinkedFormPrint,
    pub operation_receipts: Vec<FormVmOperationReceipt>,
    pub task_job_receipt: Option<FormTaskJobReceipt>,
    pub run_report: FormRunReport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkedFormPrint {
    pub form_id: String,
    pub operations: Vec<crate::form::FormVmOperation>,
    pub link_report: crate::form::FormLinkReport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramFormEvalPrint {
    pub selected_forms: Vec<String>,
    pub patterns: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrganicUnityStatus {
    Coherent,
    Illusory,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganicUnityReport {
    pub status: OrganicUnityStatus,
    pub reasons: Vec<String>,
}

impl OrganicUnityReport {
    fn coherent() -> Self {
        Self {
            status: OrganicUnityStatus::Coherent,
            reasons: Vec::new(),
        }
    }

    fn illusory(reasons: Vec<String>) -> Self {
        Self {
            status: OrganicUnityStatus::Illusory,
            reasons,
        }
    }
}

pub(crate) struct OrganicUnityCheck;

impl OrganicUnityCheck {
    pub(crate) fn evaluate(
        backend: ProgramFormApplyBackend,
        eval: &ProgramFormEvalPrint,
        apply: &ProgramFormApplyPrint,
        pre_eval: &FormPreEvalTrace,
        evaluation_slot: &MonadicEvaluationSlot,
        ok: bool,
    ) -> OrganicUnityReport {
        let mut reasons = Vec::new();

        if pre_eval.given_forms.is_empty() {
            reasons.push("No given-form decomposition was produced".to_string());
        }

        let selected: std::collections::HashSet<&str> =
            eval.selected_forms.iter().map(String::as_str).collect();

        for gf in &pre_eval.given_forms {
            if !selected.contains(gf.application_form.as_str()) {
                reasons.push(format!(
                    "Given-form '{}' is not present in selected forms",
                    gf.application_form
                ));
            }

            if gf.effect_entity.trim().is_empty()
                || gf.effect_property.trim().is_empty()
                || gf.effect_aspect.trim().is_empty()
            {
                reasons.push(format!(
                    "Given-form '{}' has an incomplete principled-effect decomposition",
                    gf.application_form
                ));
            }

            if gf.monadic_evaluation != evaluation_slot.state {
                reasons.push(format!(
                    "Given-form '{}' monadic state is out of sync with evaluation slot",
                    gf.application_form
                ));
            }
        }

        for executed in &apply.executed {
            if executed.spec_binding.trim().is_empty() {
                reasons.push(format!(
                    "Pattern '{}' has no spec binding provenance",
                    executed.pattern
                ));
            }
        }

        if backend.is_spec_driven()
            && apply
                .executed
                .iter()
                .any(|item| !item.spec_binding.starts_with("spec."))
        {
            reasons.push("Spec-driven execution includes non-spec provenance bindings".to_string());
        }

        if ok && !apply.failed.is_empty() {
            reasons.push("Print marked ok but failed executions are present".to_string());
        }

        match evaluation_slot.state {
            MonadicEvaluationState::Succeeded if !ok => {
                reasons.push("Monadic slot is Succeeded while print is not ok".to_string())
            }
            MonadicEvaluationState::Failed if ok => {
                reasons.push("Monadic slot is Failed while print is ok".to_string())
            }
            _ => {}
        }

        if reasons.is_empty() {
            OrganicUnityReport::coherent()
        } else {
            OrganicUnityReport::illusory(reasons)
        }
    }
}

#[derive(Debug)]
pub enum FormProgramError {
    Evaluate(FormEvalError),
    InvalidInputShape(String),
    Apply(ProgramFormFailure),
    Runtime(String),
}

impl fmt::Display for FormProgramError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Evaluate(error) => write!(f, "program form evaluate stage failed: {error}"),
            Self::InvalidInputShape(field) => {
                write!(f, "program form request field must be JSON object: {field}")
            }
            Self::Apply(failure) => write!(
                f,
                "program form apply stage failed for pattern '{}' (op='{}')",
                failure.pattern, failure.op
            ),
            Self::Runtime(message) => write!(f, "FormVM runtime preparation failed: {message}"),
        }
    }
}

impl Error for FormProgramError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Evaluate(error) => Some(error),
            Self::InvalidInputShape(_) | Self::Apply(_) | Self::Runtime(_) => None,
        }
    }
}

#[cfg(test)]
mod return_contract_tests {
    use super::*;

    #[test]
    fn persisted_graph_outflow_requires_observed_dataset_artifact() {
        let operations = vec![
            FormVmOperation::new(
                0,
                Some("dataset.evidence".to_string()),
                Some("organon".to_string()),
                FormVmOperationKind::CollectEvidence {
                    binding: "dataset.evidence-return".to_string(),
                },
            ),
            FormVmOperation::new(
                1,
                Some("form.return".to_string()),
                Some("organon".to_string()),
                FormVmOperationKind::ReturnForm {
                    binding: "eval-form.organic-unity".to_string(),
                },
            ),
        ];
        let expectation = FormRuntimeEvidenceExpectation {
            return_policy: "persisted".to_string(),
            expected_outputs: vec!["graphframe.dataset.graph".to_string()],
        };

        let contract = form_return_contract(&operations, true, Some(&expectation))
            .expect("return operation should define a contract");

        assert!(contract.evidence.requires_persisted_artifact);
        assert_eq!(
            contract.evidence.expected_outputs,
            vec!["graphframe.dataset.graph"]
        );
        assert!(contract
            .evidence
            .required_kinds
            .contains(&"dataset_artifact".to_string()));
    }
}
