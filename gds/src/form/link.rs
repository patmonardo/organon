//! Deterministic linking from a GivenForm Program into a FormVM executable.

use super::{
    FormVmOperation, FormVmOperationKind, GivenFormEnvelope, ProgramExecutionPlan, ProgramFeatures,
    ProgramSpec, ProgramSpecError, PureFormPrinciple,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FormLinkBindingStatus {
    Linked,
    DeferredCompatibility,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormLinkBinding {
    pub pattern: String,
    pub operation_sequence: u64,
    pub status: FormLinkBindingStatus,
    pub rationale: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormLinkReport {
    pub bindings: Vec<FormLinkBinding>,
    pub deferred_patterns: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkedExecutableForm {
    pub form_id: String,
    pub principle: PureFormPrinciple,
    pub plan: ProgramExecutionPlan,
    pub features: ProgramFeatures,
    pub given_forms: Vec<GivenFormEnvelope>,
    pub operations: Vec<FormVmOperation>,
    pub link_report: FormLinkReport,
}

#[derive(Debug, Clone, Default)]
pub struct FormLinker;

impl FormLinker {
    pub fn new() -> Self {
        Self
    }

    pub fn link(
        &self,
        program: &ProgramSpec,
        appearance: Option<String>,
    ) -> Result<LinkedExecutableForm, ProgramSpecError> {
        let principle = program.form.as_principle();
        let plan = program.compile_execution_plan()?;
        let features = program.define_features()?;
        let given_forms = program.given_forms(appearance)?;
        let mut operations = Vec::new();
        let mut report = FormLinkReport::default();

        push_operation(
            &mut operations,
            None,
            None,
            FormVmOperationKind::EstablishPrinciple,
        );

        for given_form in &given_forms {
            let name = given_form.principled_effect.application_form.name.clone();
            push_operation(
                &mut operations,
                None,
                Some(name.clone()),
                FormVmOperationKind::ProjectApplication {
                    application_form: name,
                },
            );
        }

        let mut lowered_patterns = plan
            .patterns
            .iter()
            .map(|pattern| {
                let source_application = source_application(program, pattern);
                let lowered = lower_pattern(program, pattern, source_application.as_deref());
                (pattern.clone(), source_application, lowered)
            })
            .collect::<Vec<_>>();
        lowered_patterns.sort_by_key(|(_, _, (kind, _, _))| operation_phase(kind));

        for (pattern, source_application, (kind, status, rationale)) in lowered_patterns {
            let sequence = push_operation(
                &mut operations,
                Some(pattern.clone()),
                source_application,
                kind,
            );
            if status == FormLinkBindingStatus::DeferredCompatibility {
                report.deferred_patterns.push(pattern.clone());
            }
            report.bindings.push(FormLinkBinding {
                pattern: pattern.clone(),
                operation_sequence: sequence,
                status,
                rationale,
            });
        }

        let form_id = linked_form_id(program, &operations);
        Ok(LinkedExecutableForm {
            form_id,
            principle,
            plan,
            features,
            given_forms,
            operations,
            link_report: report,
        })
    }
}

fn operation_phase(kind: &FormVmOperationKind) -> u8 {
    match kind {
        FormVmOperationKind::EstablishPrinciple
        | FormVmOperationKind::ProjectApplication { .. }
        | FormVmOperationKind::DeferredCompatibility { .. } => 0,
        FormVmOperationKind::DetermineGraph { .. } => 1,
        FormVmOperationKind::ConstituteTask { .. } => 2,
        FormVmOperationKind::InvokeOperator { .. } => 3,
        FormVmOperationKind::CollectEvidence { .. } => 4,
        FormVmOperationKind::ReturnForm { .. } => 5,
    }
}

fn push_operation(
    operations: &mut Vec<FormVmOperation>,
    source_pattern: Option<String>,
    source_application: Option<String>,
    kind: FormVmOperationKind,
) -> u64 {
    let sequence = operations.len() as u64;
    operations.push(FormVmOperation::new(
        sequence,
        source_pattern,
        source_application,
        kind,
    ));
    sequence
}

fn source_application(program: &ProgramSpec, pattern: &str) -> Option<String> {
    let selected = if program.selected_forms.is_empty() {
        program
            .application_forms
            .iter()
            .map(|form| form.name.as_str())
            .collect::<Vec<_>>()
    } else {
        program.selected_forms.iter().map(String::as_str).collect()
    };

    program
        .application_forms
        .iter()
        .find(|form| {
            selected.contains(&form.name.as_str())
                && form.patterns.iter().any(|candidate| candidate == pattern)
        })
        .map(|form| form.name.clone())
}

fn lower_pattern(
    program: &ProgramSpec,
    pattern: &str,
    source_application: Option<&str>,
) -> (FormVmOperationKind, FormLinkBindingStatus, String) {
    let binding = |key: &str, fallback: &str| {
        source_application
            .and_then(|name| {
                program
                    .application_forms
                    .iter()
                    .find(|form| form.name == name)
            })
            .and_then(|form| form.specifications.get(key))
            .cloned()
            .unwrap_or_else(|| fallback.to_string())
    };

    match pattern {
        "graphframe.determine" => (
            FormVmOperationKind::DetermineGraph {
                binding: binding("graphframe", "graphframe.execution-intent"),
            },
            FormLinkBindingStatus::Linked,
            "GraphFrame determines a GraphExecutionIntent at run time".to_string(),
        ),
        "taskframe.constitute" => (
            FormVmOperationKind::ConstituteTask {
                binding: binding("taskframe", "taskframe.workflow-constitution"),
            },
            FormLinkBindingStatus::Linked,
            "TaskFrame constitutes determined graph intent as monitored work".to_string(),
        ),
        "dataset.evidence" => (
            FormVmOperationKind::CollectEvidence {
                binding: binding("dataset", "dataset.evidence-return"),
            },
            FormLinkBindingStatus::Linked,
            "Dataset evidence mediates empirical execution back into Form".to_string(),
        ),
        "form.return" => (
            FormVmOperationKind::ReturnForm {
                binding: binding("return", "eval-form.organic-unity"),
            },
            FormLinkBindingStatus::Linked,
            "Form return closes the executable lifecycle".to_string(),
        ),
        _ if is_operator_pattern(pattern) => (
            FormVmOperationKind::InvokeOperator {
                service: operator_service(pattern).to_string(),
                operator: pattern.to_string(),
            },
            FormLinkBindingStatus::Linked,
            "ProgramFeature is bound to an executable operator service".to_string(),
        ),
        _ => (
            FormVmOperationKind::DeferredCompatibility {
                pattern: pattern.to_string(),
            },
            FormLinkBindingStatus::DeferredCompatibility,
            "Pattern is preserved explicitly for the compatibility evaluator".to_string(),
        ),
    }
}

fn is_operator_pattern(pattern: &str) -> bool {
    !pattern.contains('.')
        || [
            "algo.",
            "algorithm.",
            "applications.algorithms.",
            "shell.",
            "procedure.",
            "task.",
            "collections.",
            "dataframe.",
        ]
        .iter()
        .any(|prefix| pattern.starts_with(prefix))
}

fn operator_service(pattern: &str) -> &'static str {
    if pattern.starts_with("shell.")
        || pattern.starts_with("procedure.")
        || pattern.starts_with("task.")
    {
        "form.shell"
    } else if pattern.starts_with("collections.") || pattern.starts_with("dataframe.") {
        "form.datasets"
    } else {
        "form.algorithms"
    }
}

fn linked_form_id(program: &ProgramSpec, operations: &[FormVmOperation]) -> String {
    let mut hash = 0xcbf29ce484222325_u64;
    for value in std::iter::once(program.gdsl.name.as_str()).chain(
        operations
            .iter()
            .filter_map(|operation| operation.source_pattern.as_deref()),
    ) {
        for byte in value.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= 0xff;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("linked-form-{hash:016x}")
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::form::{ApplicationForm, Context, FormShape, Morph, Shape, Specification};

    fn organon_program() -> ProgramSpec {
        ProgramSpec::new(
            FormShape::new(
                Shape::default(),
                Context::default(),
                Morph::new(vec!["base.normalize".to_string()]),
            ),
            Specification::new("form.organon".to_string(), None, HashMap::new()),
            vec![],
            vec![ApplicationForm::organon()],
            vec!["organon".to_string()],
        )
    }

    #[test]
    fn links_organon_architecture_as_typed_operations() {
        let linked = FormLinker::new().link(&organon_program(), None).unwrap();
        let kinds = linked
            .operations
            .iter()
            .map(|operation| &operation.kind)
            .collect::<Vec<_>>();

        assert!(kinds.iter().any(|kind| matches!(
            kind,
            FormVmOperationKind::DetermineGraph { binding }
                if binding == "graphframe.execution-intent"
        )));
        assert!(kinds.iter().any(|kind| matches!(
            kind,
            FormVmOperationKind::ConstituteTask { binding }
                if binding == "taskframe.workflow-constitution"
        )));
        assert!(kinds
            .iter()
            .any(|kind| matches!(kind, FormVmOperationKind::CollectEvidence { .. })));
        assert!(kinds
            .iter()
            .any(|kind| matches!(kind, FormVmOperationKind::ReturnForm { .. })));
        assert_eq!(linked.link_report.deferred_patterns, vec!["base.normalize"]);
        let determine = linked
            .operations
            .iter()
            .position(|operation| {
                matches!(operation.kind, FormVmOperationKind::DetermineGraph { .. })
            })
            .unwrap();
        let constitute = linked
            .operations
            .iter()
            .position(|operation| {
                matches!(operation.kind, FormVmOperationKind::ConstituteTask { .. })
            })
            .unwrap();
        let evidence = linked
            .operations
            .iter()
            .position(|operation| {
                matches!(operation.kind, FormVmOperationKind::CollectEvidence { .. })
            })
            .unwrap();
        assert!(determine < constitute && constitute < evidence);
    }

    #[test]
    fn linked_identity_is_deterministic() {
        let linker = FormLinker::new();
        let first = linker.link(&organon_program(), None).unwrap();
        let second = linker.link(&organon_program(), None).unwrap();
        assert_eq!(first.form_id, second.form_id);
    }
}
