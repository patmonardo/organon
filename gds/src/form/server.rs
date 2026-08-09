//! Operational management records for the Form Server.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};

use super::{FormVmEvidenceRef, FormVmLifecycle, FormVmLifecycleState, FormVmOutcome};
use crate::task::concurrency::TerminationFlag;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FormRunId(String);

impl FormRunId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FormControlState {
    Ready,
    Running,
    Returned,
    Canceled,
    Faulted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FormControlCommand {
    Cancel,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormControlReceipt {
    pub run_id: FormRunId,
    pub command: FormControlCommand,
    pub accepted: bool,
    pub previous_state: FormControlState,
    pub current_state: FormControlState,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormEvidenceContract {
    pub binding: String,
    pub required_kinds: Vec<String>,
    pub expected_outputs: Vec<String>,
    pub requires_persisted_artifact: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormReturnContract {
    pub binding: String,
    pub require_successful_execution: bool,
    pub evidence: FormEvidenceContract,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormReturnJudgment {
    pub satisfied: bool,
    pub execution_succeeded: bool,
    pub missing_evidence_kinds: Vec<String>,
    pub rationale: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FormEvidenceAuthority {
    FormServer,
    GraphFrame,
    TaskFrame,
    TaskDaemon,
    Dataset,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FormEvidenceStatus {
    Declared,
    Observed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormEvidenceRecord {
    pub kind: String,
    pub reference: String,
    pub authority: FormEvidenceAuthority,
    pub status: FormEvidenceStatus,
    pub body_embedded: bool,
}

impl FormEvidenceRecord {
    pub fn from_reference(reference: &FormVmEvidenceRef) -> Self {
        let (authority, status) = match reference.kind.as_str() {
            "evidence_contract" | "return_contract" => {
                (FormEvidenceAuthority::Dataset, FormEvidenceStatus::Declared)
            }
            "dataset_artifact" => (FormEvidenceAuthority::Dataset, FormEvidenceStatus::Observed),
            "graph_processing_contract" => (
                FormEvidenceAuthority::GraphFrame,
                FormEvidenceStatus::Observed,
            ),
            "task_frame" => (
                FormEvidenceAuthority::TaskFrame,
                FormEvidenceStatus::Observed,
            ),
            "task_daemon_submission" | "task_job" => (
                FormEvidenceAuthority::TaskDaemon,
                FormEvidenceStatus::Observed,
            ),
            "task_component_count" => (
                FormEvidenceAuthority::TaskDaemon,
                FormEvidenceStatus::Observed,
            ),
            _ => (
                FormEvidenceAuthority::FormServer,
                FormEvidenceStatus::Observed,
            ),
        };
        Self {
            kind: reference.kind.clone(),
            reference: reference.reference.clone(),
            authority,
            status,
            body_embedded: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FormTerminalDisposition {
    Running,
    Succeeded,
    OperatorFailed,
    TaskFailed,
    ReturnContractFailed,
    Canceled,
    FormVmFault,
}

impl FormTerminalDisposition {
    pub fn determine(
        canceled: bool,
        task_succeeded: Option<bool>,
        operators_succeeded: bool,
        return_contract_satisfied: bool,
        form_vm_faulted: bool,
    ) -> Self {
        if form_vm_faulted {
            Self::FormVmFault
        } else if canceled {
            Self::Canceled
        } else if task_succeeded == Some(false) {
            Self::TaskFailed
        } else if !operators_succeeded {
            Self::OperatorFailed
        } else if !return_contract_satisfied {
            Self::ReturnContractFailed
        } else {
            Self::Succeeded
        }
    }
}

impl FormReturnContract {
    pub fn judge(
        &self,
        execution_succeeded: bool,
        evidence: &[FormVmEvidenceRef],
    ) -> FormReturnJudgment {
        let missing_evidence_kinds = self
            .evidence
            .required_kinds
            .iter()
            .filter(|required| !evidence.iter().any(|item| &item.kind == *required))
            .cloned()
            .collect::<Vec<_>>();
        let execution_requirement_satisfied =
            !self.require_successful_execution || execution_succeeded;
        let satisfied = execution_requirement_satisfied && missing_evidence_kinds.is_empty();
        let rationale = if satisfied {
            format!(
                "Return binding {} was satisfied by successful execution and all required evidence.",
                self.binding
            )
        } else if !execution_requirement_satisfied {
            format!(
                "Return binding {} was not satisfied because execution did not succeed.",
                self.binding
            )
        } else {
            format!(
                "Return binding {} was not satisfied because evidence kinds [{}] were absent.",
                self.binding,
                missing_evidence_kinds.join(", ")
            )
        };
        FormReturnJudgment {
            satisfied,
            execution_succeeded,
            missing_evidence_kinds,
            rationale,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedFormRun {
    pub run_id: FormRunId,
    pub linked_form_id: String,
    pub lifecycle: FormVmLifecycle,
    pub control_state: FormControlState,
    pub graph_contracts: Vec<String>,
    pub task_frames: Vec<String>,
    pub task_submissions: Vec<String>,
    pub task_jobs: Vec<String>,
    pub evidence: Vec<FormVmEvidenceRef>,
    pub evidence_records: Vec<FormEvidenceRecord>,
    pub return_contract: Option<FormReturnContract>,
    pub return_judgment: Option<FormReturnJudgment>,
    pub terminal_disposition: FormTerminalDisposition,
    pub outcome: FormVmOutcome,
}

impl ManagedFormRun {
    pub fn active(linked_form_id: impl Into<String>, lifecycle: FormVmLifecycle) -> Self {
        let run_id = FormRunId::new(lifecycle.run_id.clone());
        let evidence_records = lifecycle
            .evidence
            .iter()
            .map(FormEvidenceRecord::from_reference)
            .collect();
        Self {
            run_id,
            linked_form_id: linked_form_id.into(),
            control_state: FormControlState::Running,
            graph_contracts: Vec::new(),
            task_frames: Vec::new(),
            task_submissions: Vec::new(),
            task_jobs: Vec::new(),
            evidence: lifecycle.evidence.clone(),
            evidence_records,
            return_contract: None,
            return_judgment: None,
            terminal_disposition: FormTerminalDisposition::Running,
            outcome: lifecycle.outcome,
            lifecycle,
        }
    }

    pub fn returned(
        linked_form_id: impl Into<String>,
        lifecycle: FormVmLifecycle,
        graph_contracts: Vec<String>,
        task_frames: Vec<String>,
        task_submissions: Vec<String>,
        task_jobs: Vec<String>,
        return_contract: Option<FormReturnContract>,
        return_judgment: Option<FormReturnJudgment>,
        terminal_disposition: FormTerminalDisposition,
    ) -> Self {
        let run_id = FormRunId::new(lifecycle.run_id.clone());
        let evidence_records = lifecycle
            .evidence
            .iter()
            .map(FormEvidenceRecord::from_reference)
            .collect();
        Self {
            run_id,
            linked_form_id: linked_form_id.into(),
            control_state: control_state(lifecycle.state),
            graph_contracts,
            task_frames,
            task_submissions,
            task_jobs,
            evidence: lifecycle.evidence.clone(),
            evidence_records,
            return_contract,
            return_judgment,
            terminal_disposition,
            outcome: lifecycle.outcome,
            lifecycle,
        }
    }

    pub fn report(&self) -> FormRunReport {
        let summary = match self.terminal_disposition {
            FormTerminalDisposition::Succeeded => self
                .return_judgment
                .as_ref()
                .map(|judgment| {
                    format!(
                        "Form run {} returned successfully. {}",
                        self.run_id.as_str(),
                        judgment.rationale
                    )
                })
                .unwrap_or_else(|| {
                    format!(
                    "Form run {} returned successfully after satisfying its executable lifecycle.",
                    self.run_id.as_str()
                )
                }),
            FormTerminalDisposition::FormVmFault => self.lifecycle.fault.as_ref().map_or_else(
                || {
                    format!(
                        "Form run {} encountered an unspecified FormVM infrastructure fault.",
                        self.run_id.as_str()
                    )
                },
                |fault| {
                    format!(
                        "Form run {} faulted with code {}: {}",
                        self.run_id.as_str(),
                        fault.code,
                        fault.message
                    )
                },
            ),
            FormTerminalDisposition::OperatorFailed => format!(
                "Form run {} returned unsuccessfully because one or more operators failed.",
                self.run_id.as_str()
            ),
            FormTerminalDisposition::TaskFailed => format!(
                "Form run {} returned unsuccessfully because its delegated task failed.",
                self.run_id.as_str()
            ),
            FormTerminalDisposition::ReturnContractFailed => self
                .return_judgment
                .as_ref()
                .map(|judgment| {
                    format!(
                        "Form run {} returned unsuccessfully. {}",
                        self.run_id.as_str(),
                        judgment.rationale
                    )
                })
                .unwrap_or_else(|| {
                    format!(
                        "Form run {} did not satisfy its return contract.",
                        self.run_id.as_str()
                    )
                }),
            FormTerminalDisposition::Canceled => format!(
                "Form run {} was canceled before it could return a successful Form.",
                self.run_id.as_str()
            ),
            FormTerminalDisposition::Running => format!(
                "Form run {} has not reached a terminal judgment.",
                self.run_id.as_str()
            ),
        };
        FormRunReport {
            run_id: self.run_id.clone(),
            linked_form_id: self.linked_form_id.clone(),
            control_state: self.control_state,
            outcome: self.outcome,
            graph_contracts: self.graph_contracts.clone(),
            task_frames: self.task_frames.clone(),
            task_submissions: self.task_submissions.clone(),
            task_jobs: self.task_jobs.clone(),
            evidence: self.evidence.clone(),
            evidence_records: self.evidence_records.clone(),
            return_contract: self.return_contract.clone(),
            return_judgment: self.return_judgment.clone(),
            terminal_disposition: self.terminal_disposition,
            summary,
        }
    }
}

fn control_state(state: FormVmLifecycleState) -> FormControlState {
    match state {
        FormVmLifecycleState::ReadyForEvaluation => FormControlState::Ready,
        FormVmLifecycleState::Evaluating => FormControlState::Running,
        FormVmLifecycleState::Returned => FormControlState::Returned,
        FormVmLifecycleState::Canceled => FormControlState::Canceled,
        FormVmLifecycleState::Faulted => FormControlState::Faulted,
        _ => FormControlState::Ready,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormRunReport {
    pub run_id: FormRunId,
    pub linked_form_id: String,
    pub control_state: FormControlState,
    pub outcome: FormVmOutcome,
    pub graph_contracts: Vec<String>,
    pub task_frames: Vec<String>,
    pub task_submissions: Vec<String>,
    pub task_jobs: Vec<String>,
    pub evidence: Vec<FormVmEvidenceRef>,
    pub evidence_records: Vec<FormEvidenceRecord>,
    pub return_contract: Option<FormReturnContract>,
    pub return_judgment: Option<FormReturnJudgment>,
    pub terminal_disposition: FormTerminalDisposition,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormRunStateSummary {
    pub run_id: FormRunId,
    pub control_state: FormControlState,
    pub terminal_disposition: FormTerminalDisposition,
    pub graph_contract_count: usize,
    pub task_frame_count: usize,
    pub task_job_count: usize,
    pub declared_evidence_count: usize,
    pub observed_evidence_count: usize,
    pub return_contract_satisfied: Option<bool>,
    pub statement: String,
}

impl FormRunReport {
    pub fn state_summary(&self) -> FormRunStateSummary {
        let declared_evidence_count = self
            .evidence_records
            .iter()
            .filter(|record| record.status == FormEvidenceStatus::Declared)
            .count();
        let observed_evidence_count = self
            .evidence_records
            .iter()
            .filter(|record| record.status == FormEvidenceStatus::Observed)
            .count();
        let return_contract_satisfied = self
            .return_judgment
            .as_ref()
            .map(|judgment| judgment.satisfied);
        let statement = format!(
            "Form run {} is {:?}; it has {} GraphFrame contract(s), {} TaskFrame(s), {} task job(s), {} declared evidence binding(s), and {} observed evidence reference(s).",
            self.run_id.as_str(),
            self.control_state,
            self.graph_contracts.len(),
            self.task_frames.len(),
            self.task_jobs.len(),
            declared_evidence_count,
            observed_evidence_count
        );
        FormRunStateSummary {
            run_id: self.run_id.clone(),
            control_state: self.control_state,
            terminal_disposition: self.terminal_disposition,
            graph_contract_count: self.graph_contracts.len(),
            task_frame_count: self.task_frames.len(),
            task_job_count: self.task_jobs.len(),
            declared_evidence_count,
            observed_evidence_count,
            return_contract_satisfied,
            statement,
        }
    }
}

pub trait FormRunRegistry: Send + Sync {
    fn register(&self, run: ManagedFormRun) -> Result<(), FormRunRegistryError>;
    fn register_active(
        &self,
        run: ManagedFormRun,
        termination: TerminationFlag,
    ) -> Result<(), FormRunRegistryError>;
    fn replace(&self, run: ManagedFormRun) -> Result<(), FormRunRegistryError>;
    fn get(&self, run_id: &FormRunId) -> Option<ManagedFormRun>;
    fn control(
        &self,
        run_id: &FormRunId,
        command: FormControlCommand,
    ) -> Result<FormControlReceipt, FormRunRegistryError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormRunRegistryError {
    DuplicateRun(FormRunId),
    UnknownRun(FormRunId),
    Unavailable,
}

#[derive(Clone)]
struct FormRunRegistryEntry {
    run: ManagedFormRun,
    termination: Option<TerminationFlag>,
}

#[derive(Clone, Default)]
pub struct InMemoryFormRunRegistry {
    runs: Arc<RwLock<HashMap<FormRunId, FormRunRegistryEntry>>>,
}

impl InMemoryFormRunRegistry {
    pub fn new() -> Self {
        Self::default()
    }
}

impl FormRunRegistry for InMemoryFormRunRegistry {
    fn register(&self, run: ManagedFormRun) -> Result<(), FormRunRegistryError> {
        let mut runs = self
            .runs
            .write()
            .map_err(|_| FormRunRegistryError::Unavailable)?;
        if runs.contains_key(&run.run_id) {
            return Err(FormRunRegistryError::DuplicateRun(run.run_id));
        }
        runs.insert(
            run.run_id.clone(),
            FormRunRegistryEntry {
                run,
                termination: None,
            },
        );
        Ok(())
    }

    fn register_active(
        &self,
        run: ManagedFormRun,
        termination: TerminationFlag,
    ) -> Result<(), FormRunRegistryError> {
        let mut runs = self
            .runs
            .write()
            .map_err(|_| FormRunRegistryError::Unavailable)?;
        if runs.contains_key(&run.run_id) {
            return Err(FormRunRegistryError::DuplicateRun(run.run_id));
        }
        runs.insert(
            run.run_id.clone(),
            FormRunRegistryEntry {
                run,
                termination: Some(termination),
            },
        );
        Ok(())
    }

    fn replace(&self, run: ManagedFormRun) -> Result<(), FormRunRegistryError> {
        let mut runs = self
            .runs
            .write()
            .map_err(|_| FormRunRegistryError::Unavailable)?;
        if !runs.contains_key(&run.run_id) {
            return Err(FormRunRegistryError::UnknownRun(run.run_id));
        }
        runs.insert(
            run.run_id.clone(),
            FormRunRegistryEntry {
                run,
                termination: None,
            },
        );
        Ok(())
    }

    fn get(&self, run_id: &FormRunId) -> Option<ManagedFormRun> {
        self.runs
            .read()
            .ok()?
            .get(run_id)
            .map(|entry| entry.run.clone())
    }

    fn control(
        &self,
        run_id: &FormRunId,
        command: FormControlCommand,
    ) -> Result<FormControlReceipt, FormRunRegistryError> {
        let mut runs = self
            .runs
            .write()
            .map_err(|_| FormRunRegistryError::Unavailable)?;
        let entry = runs
            .get_mut(run_id)
            .ok_or_else(|| FormRunRegistryError::UnknownRun(run_id.clone()))?;
        let previous_state = entry.run.control_state;
        let accepted = entry.termination.is_some()
            && matches!(
                previous_state,
                FormControlState::Ready | FormControlState::Running
            );
        if accepted {
            if let Some(termination) = &entry.termination {
                termination.request_termination();
            }
            entry
                .run
                .lifecycle
                .cancel()
                .map_err(|_| FormRunRegistryError::Unavailable)?;
            entry.run.control_state = FormControlState::Canceled;
            entry.run.outcome = FormVmOutcome::Canceled;
            entry.run.terminal_disposition = FormTerminalDisposition::Canceled;
            entry.run.evidence = entry.run.lifecycle.evidence.clone();
            entry.run.evidence_records = entry
                .run
                .evidence
                .iter()
                .map(FormEvidenceRecord::from_reference)
                .collect();
        }
        Ok(FormControlReceipt {
            run_id: run_id.clone(),
            command,
            accepted,
            previous_state,
            current_state: entry.run.control_state,
            message: if accepted {
                format!(
                    "Cancellation was accepted for Form run {}.",
                    run_id.as_str()
                )
            } else {
                format!(
                    "Cancellation was rejected because Form run {} is already {:?}.",
                    run_id.as_str(),
                    previous_state
                )
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn returned_lifecycle() -> FormVmLifecycle {
        FormVmLifecycle {
            run_id: "form-run-test".to_string(),
            state: FormVmLifecycleState::Returned,
            outcome: FormVmOutcome::Succeeded,
            events: Vec::new(),
            evidence: vec![FormVmEvidenceRef::new("result", "dataset://result/1")],
            fault: None,
        }
    }

    fn evaluating_lifecycle() -> FormVmLifecycle {
        FormVmLifecycle {
            run_id: "form-run-active".to_string(),
            state: FormVmLifecycleState::Evaluating,
            outcome: FormVmOutcome::Pending,
            events: Vec::new(),
            evidence: Vec::new(),
            fault: None,
        }
    }

    #[test]
    fn registry_preserves_managed_run_and_rational_report() {
        let registry = InMemoryFormRunRegistry::new();
        let run = ManagedFormRun::returned(
            "linked-form-1",
            returned_lifecycle(),
            vec!["graph-contract-1".to_string()],
            vec!["task-frame-1".to_string()],
            vec!["task-submission-1".to_string()],
            vec!["task-job-1".to_string()],
            None,
            None,
            FormTerminalDisposition::Succeeded,
        );
        let run_id = run.run_id.clone();
        registry.register(run).unwrap();

        let stored = registry.get(&run_id).expect("managed run should exist");
        assert_eq!(stored.linked_form_id, "linked-form-1");
        assert!(stored.report().summary.contains("returned successfully"));
        let state = stored.report().state_summary();
        assert_eq!(state.graph_contract_count, 1);
        assert_eq!(state.task_frame_count, 1);
        assert_eq!(state.task_job_count, 1);
        assert_eq!(state.observed_evidence_count, 1);
        assert!(state.statement.contains("1 GraphFrame contract(s)"));
    }

    #[test]
    fn registry_rejects_duplicate_run_identity() {
        let registry = InMemoryFormRunRegistry::new();
        let run = ManagedFormRun::returned(
            "linked-form-1",
            returned_lifecycle(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            None,
            None,
            FormTerminalDisposition::Succeeded,
        );
        registry.register(run.clone()).unwrap();
        assert_eq!(
            registry.register(run),
            Err(FormRunRegistryError::DuplicateRun(FormRunId::new(
                "form-run-test"
            )))
        );
    }

    #[test]
    fn cancellation_signals_execution_and_returns_a_control_receipt() {
        let registry = InMemoryFormRunRegistry::new();
        let termination = TerminationFlag::running_true();
        let run = ManagedFormRun::active("linked-form-active", evaluating_lifecycle());
        let run_id = run.run_id.clone();
        registry
            .register_active(run, termination.clone())
            .expect("active run should register");

        let receipt = registry
            .control(&run_id, FormControlCommand::Cancel)
            .expect("control request should be processed");

        assert!(receipt.accepted);
        assert_eq!(receipt.previous_state, FormControlState::Running);
        assert_eq!(receipt.current_state, FormControlState::Canceled);
        assert!(!termination.running());
        let stored = registry
            .get(&run_id)
            .expect("managed run should remain inspectable");
        assert_eq!(stored.lifecycle.state, FormVmLifecycleState::Canceled);
        assert_eq!(stored.outcome, FormVmOutcome::Canceled);
    }

    #[test]
    fn cancellation_of_terminal_run_is_rejected_without_mutation() {
        let registry = InMemoryFormRunRegistry::new();
        let run = ManagedFormRun::returned(
            "linked-form-1",
            returned_lifecycle(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            None,
            None,
            FormTerminalDisposition::Succeeded,
        );
        let run_id = run.run_id.clone();
        registry.register(run).unwrap();

        let receipt = registry
            .control(&run_id, FormControlCommand::Cancel)
            .expect("terminal rejection is itself a valid receipt");

        assert!(!receipt.accepted);
        assert_eq!(receipt.current_state, FormControlState::Returned);
        assert_eq!(
            registry.get(&run_id).unwrap().outcome,
            FormVmOutcome::Succeeded
        );
    }

    #[test]
    fn return_contract_reports_missing_evidence_in_precise_terms() {
        let contract = FormReturnContract {
            binding: "eval-form.organic-unity".to_string(),
            require_successful_execution: true,
            evidence: FormEvidenceContract {
                binding: "dataset.evidence-return".to_string(),
                required_kinds: vec!["task_job".to_string(), "dataset_result".to_string()],
                expected_outputs: Vec::new(),
                requires_persisted_artifact: false,
            },
        };

        let judgment = contract.judge(true, &[FormVmEvidenceRef::new("task_job", "task-job-1")]);

        assert!(!judgment.satisfied);
        assert_eq!(judgment.missing_evidence_kinds, vec!["dataset_result"]);
        assert_eq!(
            judgment.rationale,
            "Return binding eval-form.organic-unity was not satisfied because evidence kinds [dataset_result] were absent."
        );
    }

    #[test]
    fn return_contract_judgment_survives_report_serialization() {
        let contract = FormReturnContract {
            binding: "eval-form.organic-unity".to_string(),
            require_successful_execution: true,
            evidence: FormEvidenceContract {
                binding: "dataset.evidence-return".to_string(),
                required_kinds: vec!["result".to_string()],
                expected_outputs: vec!["graphframe.compute.result".to_string()],
                requires_persisted_artifact: false,
            },
        };
        let lifecycle = returned_lifecycle();
        let judgment = contract.judge(true, &lifecycle.evidence);
        let report = ManagedFormRun::returned(
            "linked-form-1",
            lifecycle,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Some(contract),
            Some(judgment),
            FormTerminalDisposition::Succeeded,
        )
        .report();

        let json = serde_json::to_value(report).expect("Form run report should serialize");
        assert_eq!(json["returnJudgment"]["satisfied"], true);
        assert_eq!(json["terminalDisposition"], "succeeded");
        assert_eq!(
            json["returnContract"]["evidence"]["binding"],
            "dataset.evidence-return"
        );
    }

    #[test]
    fn terminal_disposition_distinguishes_failure_authorities() {
        assert_eq!(
            FormTerminalDisposition::determine(false, Some(false), false, false, false),
            FormTerminalDisposition::TaskFailed
        );
        assert_eq!(
            FormTerminalDisposition::determine(false, None, false, false, false),
            FormTerminalDisposition::OperatorFailed
        );
        assert_eq!(
            FormTerminalDisposition::determine(false, Some(true), true, false, false),
            FormTerminalDisposition::ReturnContractFailed
        );
        assert_eq!(
            FormTerminalDisposition::determine(true, Some(false), false, false, true),
            FormTerminalDisposition::FormVmFault
        );
    }

    #[test]
    fn evidence_records_preserve_authority_without_embedding_bodies() {
        let declared = FormEvidenceRecord::from_reference(&FormVmEvidenceRef::new(
            "evidence_contract",
            "dataset.evidence-return",
        ));
        let observed =
            FormEvidenceRecord::from_reference(&FormVmEvidenceRef::new("task_job", "task-job-1"));

        assert_eq!(declared.authority, FormEvidenceAuthority::Dataset);
        assert_eq!(declared.status, FormEvidenceStatus::Declared);
        assert_eq!(observed.authority, FormEvidenceAuthority::TaskDaemon);
        assert_eq!(observed.status, FormEvidenceStatus::Observed);
        assert!(!declared.body_embedded);
        assert!(!observed.body_embedded);
    }
}
