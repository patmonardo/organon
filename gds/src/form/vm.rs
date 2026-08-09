//! Root Form virtual machine.
//!
//! The machine holds the PureForm principle as its invariant state while governing
//! ApplicationForm, GivenForm, ProgramFeature, and Eval moments as one lifecycle.
//! Empirical execution remains outside Form core and reports its result back into
//! the lifecycle before the Form returns.

use super::FormLinker;
use super::FormVmOperationKind;
use super::GivenFormEnvelope;
use super::LinkedExecutableForm;
use super::ProgramExecutionPlan;
use super::ProgramFeature;
use super::ProgramFeatures;
use super::ProgramSpec;
use super::ProgramSpecError;
use super::PureFormPrinciple;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{error::Error, fmt};

static NEXT_FORM_VM_RUN_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FormVmLifecycleState {
    Created,
    Loaded,
    PrincipleEstablished,
    Linked,
    ApplicationProjected,
    FeaturesArticulated,
    ReadyForEvaluation,
    Evaluating,
    Returned,
    Faulted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FormVmOutcome {
    Pending,
    ReadyForEvaluation,
    Succeeded,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind", content = "subject")]
pub enum FormVmLifecycleEventKind {
    RunCreated,
    FormLoaded,
    PrincipleEstablished,
    ExecutableLinked(String),
    OperationPrepared(u64),
    OperationMediated(u64),
    ApplicationProjected(String),
    ProgramFeatureArticulated(String),
    MorphPatternEmitted(String),
    EvaluationReady,
    EvaluationStarted,
    EvaluationCompleted,
    FormReturned,
    Faulted(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormVmLifecycleEvent {
    pub run_id: String,
    pub sequence: u64,
    pub observed_at_unix_ms: u128,
    pub state: FormVmLifecycleState,
    pub event: FormVmLifecycleEventKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormVmEvidenceRef {
    pub kind: String,
    pub reference: String,
}

impl FormVmEvidenceRef {
    pub fn new(kind: impl Into<String>, reference: impl Into<String>) -> Self {
        Self {
            kind: kind.into(),
            reference: reference.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormVmFault {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormVmLifecycle {
    pub run_id: String,
    pub state: FormVmLifecycleState,
    pub outcome: FormVmOutcome,
    pub events: Vec<FormVmLifecycleEvent>,
    pub evidence: Vec<FormVmEvidenceRef>,
    pub fault: Option<FormVmFault>,
}

impl FormVmLifecycle {
    fn new() -> Self {
        let run_id = format!(
            "form-vm-{}-{}",
            now_unix_ms(),
            NEXT_FORM_VM_RUN_ID.fetch_add(1, Ordering::Relaxed)
        );
        let mut lifecycle = Self {
            run_id,
            state: FormVmLifecycleState::Created,
            outcome: FormVmOutcome::Pending,
            events: Vec::new(),
            evidence: Vec::new(),
            fault: None,
        };
        lifecycle.record(FormVmLifecycleEventKind::RunCreated);
        lifecycle
    }

    fn transition(&mut self, state: FormVmLifecycleState, event: FormVmLifecycleEventKind) {
        self.state = state;
        self.record(event);
    }

    fn record(&mut self, event: FormVmLifecycleEventKind) {
        self.events.push(FormVmLifecycleEvent {
            run_id: self.run_id.clone(),
            sequence: self.events.len() as u64,
            observed_at_unix_ms: now_unix_ms(),
            state: self.state,
            event,
        });
    }

    pub fn add_evidence(&mut self, evidence: FormVmEvidenceRef) {
        if !self.evidence.contains(&evidence) {
            self.evidence.push(evidence);
        }
    }

    pub fn record_operation_mediation(&mut self, sequence: u64, evidence: FormVmEvidenceRef) {
        self.add_evidence(evidence);
        self.record(FormVmLifecycleEventKind::OperationMediated(sequence));
    }

    pub fn begin_evaluation(&mut self) -> Result<(), FormVmLifecycleError> {
        if self.state != FormVmLifecycleState::ReadyForEvaluation {
            return Err(FormVmLifecycleError::InvalidTransition {
                from: self.state,
                to: FormVmLifecycleState::Evaluating,
            });
        }
        self.transition(
            FormVmLifecycleState::Evaluating,
            FormVmLifecycleEventKind::EvaluationStarted,
        );
        Ok(())
    }

    pub fn complete_evaluation(
        &mut self,
        succeeded: bool,
        evidence: impl IntoIterator<Item = FormVmEvidenceRef>,
    ) -> Result<(), FormVmLifecycleError> {
        if self.state != FormVmLifecycleState::Evaluating {
            return Err(FormVmLifecycleError::InvalidTransition {
                from: self.state,
                to: FormVmLifecycleState::Returned,
            });
        }
        for evidence_ref in evidence {
            self.add_evidence(evidence_ref);
        }
        self.record(FormVmLifecycleEventKind::EvaluationCompleted);
        self.state = FormVmLifecycleState::Returned;
        self.outcome = if succeeded {
            FormVmOutcome::Succeeded
        } else {
            FormVmOutcome::Failed
        };
        self.record(FormVmLifecycleEventKind::FormReturned);
        Ok(())
    }

    fn fail(&mut self, code: &str, message: String) {
        self.state = FormVmLifecycleState::Faulted;
        self.outcome = FormVmOutcome::Failed;
        self.fault = Some(FormVmFault {
            code: code.to_string(),
            message: message.clone(),
        });
        self.record(FormVmLifecycleEventKind::Faulted(message));
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormVmLifecycleError {
    InvalidTransition {
        from: FormVmLifecycleState,
        to: FormVmLifecycleState,
    },
}

impl fmt::Display for FormVmLifecycleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTransition { from, to } => {
                write!(
                    f,
                    "invalid FormVM lifecycle transition from {from:?} to {to:?}"
                )
            }
        }
    }
}

impl Error for FormVmLifecycleError {}

fn now_unix_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PureFormVmPhase {
    Loaded,
    PrincipleProjected,
    ApplicationProjected,
    FeaturesArticulated,
    ReturnedToEval,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PureFormOp {
    LoadPrinciple,
    ProjectPrinciple,
    ProjectApplicationForm(String),
    ArticulateProgramFeature(ProgramFeature),
    EmitMorphPattern(String),
    ReturnToEval,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PureFormVmTraceEntry {
    pub phase: PureFormVmPhase,
    pub operation: PureFormOp,
}

impl PureFormVmTraceEntry {
    pub fn new(phase: PureFormVmPhase, operation: PureFormOp) -> Self {
        Self { phase, operation }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PureFormVmTrace {
    pub entries: Vec<PureFormVmTraceEntry>,
}

impl PureFormVmTrace {
    fn push(&mut self, phase: PureFormVmPhase, operation: PureFormOp) {
        self.entries
            .push(PureFormVmTraceEntry::new(phase, operation));
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PureFormVmOutput {
    pub executable: LinkedExecutableForm,
    pub principle: PureFormPrinciple,
    pub plan: ProgramExecutionPlan,
    pub features: ProgramFeatures,
    pub given_forms: Vec<GivenFormEnvelope>,
    pub trace: PureFormVmTrace,
    pub lifecycle: FormVmLifecycle,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormVmMonitoredRun {
    pub lifecycle: FormVmLifecycle,
    pub output: Option<PureFormVmOutput>,
    pub error: Option<ProgramSpecError>,
}

#[derive(Debug, Clone, Default)]
pub struct FormVm;

impl FormVm {
    pub fn new() -> Self {
        Self
    }

    pub fn run(
        &self,
        program: &ProgramSpec,
        appearance: Option<String>,
    ) -> Result<PureFormVmOutput, ProgramSpecError> {
        let monitored = self.run_monitored(program, appearance);
        match (monitored.output, monitored.error) {
            (Some(output), None) => Ok(output),
            (None, Some(error)) => Err(error),
            _ => unreachable!("a monitored FormVM run must produce output or an error"),
        }
    }

    /// Run the Form lifecycle without losing failed-run monitoring information.
    pub fn run_monitored(
        &self,
        program: &ProgramSpec,
        appearance: Option<String>,
    ) -> FormVmMonitoredRun {
        let mut lifecycle = FormVmLifecycle::new();
        lifecycle.transition(
            FormVmLifecycleState::Loaded,
            FormVmLifecycleEventKind::FormLoaded,
        );
        let executable = match FormLinker::new().link(program, appearance) {
            Ok(executable) => executable,
            Err(error) => return failed_run(lifecycle, error),
        };
        self.run_linked_with_lifecycle(executable, lifecycle)
    }

    /// Start a new monitored run from an already-linked, immutable Form artifact.
    pub fn run_linked(&self, executable: &LinkedExecutableForm) -> PureFormVmOutput {
        let mut lifecycle = FormVmLifecycle::new();
        lifecycle.transition(
            FormVmLifecycleState::Loaded,
            FormVmLifecycleEventKind::FormLoaded,
        );
        self.run_linked_with_lifecycle(executable.clone(), lifecycle)
            .output
            .expect("an already-linked Form must produce VM output")
    }

    fn run_linked_with_lifecycle(
        &self,
        executable: LinkedExecutableForm,
        mut lifecycle: FormVmLifecycle,
    ) -> FormVmMonitoredRun {
        let principle = executable.principle.clone();
        let plan = executable.plan.clone();
        let features = executable.features.clone();
        let given_forms = executable.given_forms.clone();

        lifecycle.add_evidence(FormVmEvidenceRef::new(
            "principle",
            features.program_name.clone(),
        ));
        lifecycle.transition(
            FormVmLifecycleState::PrincipleEstablished,
            FormVmLifecycleEventKind::PrincipleEstablished,
        );
        lifecycle.add_evidence(FormVmEvidenceRef::new(
            "linked_executable_form",
            executable.form_id.clone(),
        ));
        lifecycle.transition(
            FormVmLifecycleState::Linked,
            FormVmLifecycleEventKind::ExecutableLinked(executable.form_id.clone()),
        );

        for operation in &executable.operations {
            lifecycle.record(FormVmLifecycleEventKind::OperationPrepared(
                operation.sequence,
            ));
            if let FormVmOperationKind::DeferredCompatibility { pattern } = &operation.kind {
                lifecycle.add_evidence(FormVmEvidenceRef::new(
                    "deferred_compatibility_operation",
                    pattern.clone(),
                ));
            }
        }

        let mut trace = PureFormVmTrace::default();
        trace.push(PureFormVmPhase::Loaded, PureFormOp::LoadPrinciple);
        trace.push(
            PureFormVmPhase::PrincipleProjected,
            PureFormOp::ProjectPrinciple,
        );

        for given_form in &given_forms {
            let name = given_form.principled_effect.application_form.name.clone();
            lifecycle.add_evidence(FormVmEvidenceRef::new("application_form", name.clone()));
            lifecycle.transition(
                FormVmLifecycleState::ApplicationProjected,
                FormVmLifecycleEventKind::ApplicationProjected(name.clone()),
            );
            trace.push(
                PureFormVmPhase::ApplicationProjected,
                PureFormOp::ProjectApplicationForm(
                    given_form.principled_effect.application_form.name.clone(),
                ),
            );
        }

        for feature in &features.features {
            lifecycle.transition(
                FormVmLifecycleState::FeaturesArticulated,
                FormVmLifecycleEventKind::ProgramFeatureArticulated(feature.value.clone()),
            );
            trace.push(
                PureFormVmPhase::FeaturesArticulated,
                PureFormOp::ArticulateProgramFeature(feature.clone()),
            );
        }

        for pattern in &plan.patterns {
            lifecycle.add_evidence(FormVmEvidenceRef::new("morph_pattern", pattern.clone()));
            lifecycle.transition(
                FormVmLifecycleState::FeaturesArticulated,
                FormVmLifecycleEventKind::MorphPatternEmitted(pattern.clone()),
            );
            trace.push(
                PureFormVmPhase::FeaturesArticulated,
                PureFormOp::EmitMorphPattern(pattern.clone()),
            );
        }

        trace.push(PureFormVmPhase::ReturnedToEval, PureFormOp::ReturnToEval);
        lifecycle.state = FormVmLifecycleState::ReadyForEvaluation;
        lifecycle.outcome = FormVmOutcome::ReadyForEvaluation;
        lifecycle.record(FormVmLifecycleEventKind::EvaluationReady);

        let output = PureFormVmOutput {
            executable,
            principle,
            plan,
            features,
            given_forms,
            trace,
            lifecycle: lifecycle.clone(),
        };
        FormVmMonitoredRun {
            lifecycle,
            output: Some(output),
            error: None,
        }
    }
}

/// Compatibility name for callers written before the VM became the root Form authority.
pub type PureFormVm = FormVm;

fn failed_run(mut lifecycle: FormVmLifecycle, error: ProgramSpecError) -> FormVmMonitoredRun {
    lifecycle.fail("PROGRAM_SPEC_ERROR", error.to_string());
    FormVmMonitoredRun {
        lifecycle,
        output: None,
        error: Some(error),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::form::ApplicationForm;
    use crate::form::Context;
    use crate::form::FormShape;
    use crate::form::Morph;
    use crate::form::Shape;
    use crate::form::Specification;

    fn organon_program() -> ProgramSpec {
        ProgramSpec::new(
            FormShape::new(
                Shape::default(),
                Context::new(vec![], vec![], "kernel".to_string(), vec![]),
                Morph::new(vec!["base.normalize".to_string()]),
            ),
            Specification::new("form.organon".to_string(), None, HashMap::new()),
            vec![],
            vec![ApplicationForm::organon()],
            vec!["organon".to_string()],
        )
    }

    #[test]
    fn pure_form_vm_preserves_principle_and_virtual_projections() {
        let program = organon_program();
        let expected_principle = program.form.as_principle();
        let expected_plan = program
            .compile_execution_plan()
            .expect("Organon plan should compile");
        let expected_features = program
            .define_features()
            .expect("Organon features should compile");

        let output = PureFormVm::new()
            .run(&program, Some("graph://organon".to_string()))
            .expect("PureForm VM should run");

        assert_eq!(output.principle, expected_principle);
        assert_eq!(output.plan, expected_plan);
        assert_eq!(output.features, expected_features);
        assert_eq!(output.given_forms.len(), 1);
        assert_eq!(
            output.given_forms[0]
                .principled_effect
                .application_form
                .name,
            "organon"
        );
        assert_eq!(
            output.given_forms[0]
                .principled_effect
                .appearance
                .as_deref(),
            Some("graph://organon")
        );

        let emitted_patterns = output
            .trace
            .entries
            .iter()
            .filter_map(|entry| match &entry.operation {
                PureFormOp::EmitMorphPattern(pattern) => Some(pattern.clone()),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(emitted_patterns, output.plan.patterns);
        assert_eq!(
            output.trace.entries.first().map(|entry| entry.phase),
            Some(PureFormVmPhase::Loaded)
        );
        assert_eq!(
            output.trace.entries.last().map(|entry| entry.phase),
            Some(PureFormVmPhase::ReturnedToEval)
        );
        assert_eq!(
            output.trace.entries.last().map(|entry| &entry.operation),
            Some(&PureFormOp::ReturnToEval)
        );
    }

    #[test]
    fn pure_form_vm_propagates_program_selection_errors() {
        let mut program = organon_program();
        program.selected_forms.push("organon".to_string());

        let error = PureFormVm::new()
            .run(&program, None)
            .expect_err("duplicate selection should fail");

        assert_eq!(
            error,
            ProgramSpecError::DuplicateFormSelection("organon".to_string())
        );

        program.selected_forms = vec!["missing".to_string()];
        let error = PureFormVm::new()
            .run(&program, None)
            .expect_err("unknown selection should fail");
        assert_eq!(
            error,
            ProgramSpecError::UnknownApplicationForm("missing".to_string())
        );
    }

    #[test]
    fn pure_form_vm_projects_application_forms_in_selection_order() {
        let mut program = organon_program();
        program.application_forms.push(ApplicationForm::new(
            "reflection".to_string(),
            "organon-platform".to_string(),
            vec!["program".to_string()],
            vec!["form.reflect".to_string()],
            HashMap::new(),
        ));
        program.selected_forms = vec!["reflection".to_string(), "organon".to_string()];

        let output = PureFormVm::new()
            .run(&program, None)
            .expect("ordered virtual projection should succeed");
        let projected_forms = output
            .trace
            .entries
            .iter()
            .filter_map(|entry| match &entry.operation {
                PureFormOp::ProjectApplicationForm(name) => Some(name.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>();

        assert_eq!(projected_forms, vec!["reflection", "organon"]);
        assert_eq!(output.plan.selected_forms, projected_forms);
    }

    #[test]
    fn monitored_run_exposes_ordered_ready_lifecycle() {
        let run = PureFormVm::new().run_monitored(&organon_program(), None);

        assert!(run.error.is_none());
        assert_eq!(
            run.lifecycle.state,
            FormVmLifecycleState::ReadyForEvaluation
        );
        assert_eq!(run.lifecycle.outcome, FormVmOutcome::ReadyForEvaluation);
        assert!(run
            .lifecycle
            .events
            .windows(2)
            .all(|pair| pair[0].sequence + 1 == pair[1].sequence));
        assert!(run.lifecycle.events.iter().any(|event| {
            event.event == FormVmLifecycleEventKind::ApplicationProjected("organon".to_string())
        }));
        assert_eq!(
            run.lifecycle.events.last().map(|event| &event.event),
            Some(&FormVmLifecycleEventKind::EvaluationReady)
        );
    }

    #[test]
    fn monitored_run_retains_faulted_program_lifecycle() {
        let mut program = organon_program();
        program.selected_forms = vec!["missing".to_string()];

        let run = PureFormVm::new().run_monitored(&program, None);

        assert!(run.output.is_none());
        assert_eq!(run.lifecycle.state, FormVmLifecycleState::Faulted);
        assert_eq!(run.lifecycle.outcome, FormVmOutcome::Failed);
        assert_eq!(
            run.lifecycle
                .fault
                .as_ref()
                .map(|fault| fault.code.as_str()),
            Some("PROGRAM_SPEC_ERROR")
        );
        assert!(matches!(
            run.lifecycle.events.last().map(|event| &event.event),
            Some(FormVmLifecycleEventKind::Faulted(_))
        ));
    }

    #[test]
    fn lifecycle_is_completed_by_application_evaluation() {
        let mut lifecycle = PureFormVm::new()
            .run(&organon_program(), None)
            .expect("FormVM should prepare evaluation")
            .lifecycle;

        lifecycle
            .begin_evaluation()
            .expect("ready lifecycle should begin evaluation");
        lifecycle
            .complete_evaluation(true, [FormVmEvidenceRef::new("result", "graph://organon")])
            .expect("evaluating lifecycle should return");

        assert_eq!(lifecycle.state, FormVmLifecycleState::Returned);
        assert_eq!(lifecycle.outcome, FormVmOutcome::Succeeded);
        assert!(lifecycle
            .evidence
            .contains(&FormVmEvidenceRef::new("result", "graph://organon")));
    }

    #[test]
    fn vm_runs_an_already_linked_form_without_reinterpreting_patterns() {
        let executable = FormLinker::new()
            .link(&organon_program(), None)
            .expect("Organon Form should link");
        let expected_id = executable.form_id.clone();

        let output = FormVm::new().run_linked(&executable);

        assert_eq!(output.executable.form_id, expected_id);
        assert!(output.lifecycle.events.iter().any(|event| {
            event.event == FormVmLifecycleEventKind::ExecutableLinked(expected_id.clone())
        }));
        assert_eq!(
            output
                .lifecycle
                .events
                .iter()
                .filter(|event| matches!(
                    event.event,
                    FormVmLifecycleEventKind::OperationPrepared(_)
                ))
                .count(),
            executable.operations.len()
        );
    }
}
