//! PureForm virtual machine.
//!
//! The machine holds the PureForm principle as its invariant state and records
//! ApplicationForm, GivenForm, ProgramFeature, and Eval moments as virtual
//! projections. Empirical execution remains outside Form core.

use super::GivenFormEnvelope;
use super::ProgramExecutionPlan;
use super::ProgramFeature;
use super::ProgramFeatures;
use super::ProgramSpec;
use super::ProgramSpecError;
use super::PureFormPrinciple;

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
    pub principle: PureFormPrinciple,
    pub plan: ProgramExecutionPlan,
    pub features: ProgramFeatures,
    pub given_forms: Vec<GivenFormEnvelope>,
    pub trace: PureFormVmTrace,
}

#[derive(Debug, Clone, Default)]
pub struct PureFormVm;

impl PureFormVm {
    pub fn new() -> Self {
        Self
    }

    pub fn run(
        &self,
        program: &ProgramSpec,
        appearance: Option<String>,
    ) -> Result<PureFormVmOutput, ProgramSpecError> {
        let principle = program.form.as_principle();
        let plan = program.compile_execution_plan()?;
        let features = program.define_features()?;
        let given_forms = program.given_forms(appearance)?;

        let mut trace = PureFormVmTrace::default();
        trace.push(PureFormVmPhase::Loaded, PureFormOp::LoadPrinciple);
        trace.push(
            PureFormVmPhase::PrincipleProjected,
            PureFormOp::ProjectPrinciple,
        );

        for given_form in &given_forms {
            trace.push(
                PureFormVmPhase::ApplicationProjected,
                PureFormOp::ProjectApplicationForm(
                    given_form.principled_effect.application_form.name.clone(),
                ),
            );
        }

        for feature in &features.features {
            trace.push(
                PureFormVmPhase::FeaturesArticulated,
                PureFormOp::ArticulateProgramFeature(feature.clone()),
            );
        }

        for pattern in &plan.patterns {
            trace.push(
                PureFormVmPhase::FeaturesArticulated,
                PureFormOp::EmitMorphPattern(pattern.clone()),
            );
        }

        trace.push(PureFormVmPhase::ReturnedToEval, PureFormOp::ReturnToEval);

        Ok(PureFormVmOutput {
            principle,
            plan,
            features,
            given_forms,
            trace,
        })
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
}
