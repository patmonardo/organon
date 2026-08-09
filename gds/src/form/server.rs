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
    pub outcome: FormVmOutcome,
}

impl ManagedFormRun {
    pub fn active(linked_form_id: impl Into<String>, lifecycle: FormVmLifecycle) -> Self {
        let run_id = FormRunId::new(lifecycle.run_id.clone());
        Self {
            run_id,
            linked_form_id: linked_form_id.into(),
            control_state: FormControlState::Running,
            graph_contracts: Vec::new(),
            task_frames: Vec::new(),
            task_submissions: Vec::new(),
            task_jobs: Vec::new(),
            evidence: lifecycle.evidence.clone(),
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
    ) -> Self {
        let run_id = FormRunId::new(lifecycle.run_id.clone());
        Self {
            run_id,
            linked_form_id: linked_form_id.into(),
            control_state: control_state(lifecycle.state),
            graph_contracts,
            task_frames,
            task_submissions,
            task_jobs,
            evidence: lifecycle.evidence.clone(),
            outcome: lifecycle.outcome,
            lifecycle,
        }
    }

    pub fn report(&self) -> FormRunReport {
        let summary = match self.outcome {
            FormVmOutcome::Succeeded => format!(
                "Form run {} returned successfully after satisfying its executable lifecycle.",
                self.run_id.as_str()
            ),
            FormVmOutcome::Failed => match &self.lifecycle.fault {
                Some(fault) => format!(
                    "Form run {} faulted with code {}: {}",
                    self.run_id.as_str(),
                    fault.code,
                    fault.message
                ),
                None => format!(
                    "Form run {} returned unsuccessfully with {} evidence reference(s); no FormVM infrastructure fault was recorded.",
                    self.run_id.as_str(),
                    self.evidence.len()
                ),
            },
            FormVmOutcome::ReadyForEvaluation => format!(
                "Form run {} is linked and ready for application evaluation.",
                self.run_id.as_str()
            ),
            FormVmOutcome::Pending => {
                format!(
                    "Form run {} has not reached a terminal judgment.",
                    self.run_id.as_str()
                )
            }
            FormVmOutcome::Canceled => format!(
                "Form run {} was canceled before it could return a successful Form.",
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
    pub summary: String,
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
            entry.run.evidence = entry.run.lifecycle.evidence.clone();
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
        );
        let run_id = run.run_id.clone();
        registry.register(run).unwrap();

        let stored = registry.get(&run_id).expect("managed run should exist");
        assert_eq!(stored.linked_form_id, "linked-form-1");
        assert!(stored.report().summary.contains("returned successfully"));
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
}
