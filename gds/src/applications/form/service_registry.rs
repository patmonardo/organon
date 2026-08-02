use crate::applications::form::service_manifest::FormServiceExecutionState;
use crate::applications::form::service_manifest::FormServiceMachine;
use crate::applications::form::service_manifest::FormServiceManifest;
use crate::applications::form::service_manifest::ShellDaemonRuntimeProfile;
use crate::form::ProgramSpec;
use crate::shell::BuiltinComponentSuite;
use crate::shell::ShellComponentDescriptor;

const PROGRAM_SERVICE_ID: &str = "form.program";
const ALGORITHM_SERVICE_ID: &str = "form.algorithms";
const DATASET_SERVICE_ID: &str = "form.datasets";
const SHELL_SERVICE_ID: &str = "form.shell";
const RECURSION_SERVICE_ID: &str = "form.recursion";

pub fn service_manifest(
    program: &ProgramSpec,
    service_override: Option<&str>,
) -> Result<FormServiceManifest, String> {
    if let Some(service_id) = service_override {
        if !matches!(
            service_id,
            PROGRAM_SERVICE_ID
                | ALGORITHM_SERVICE_ID
                | DATASET_SERVICE_ID
                | SHELL_SERVICE_ID
                | RECURSION_SERVICE_ID
        ) {
            return Err(format!("unknown Form serviceId: {service_id}"));
        }
    }

    let mut machines = vec![FormServiceMachine::new(
        PROGRAM_SERVICE_ID,
        "Normalize and evaluate a PureForm Program with ProgramFeatures evidence",
        FormServiceExecutionState::Actual,
        "ProgramForm/ExecuteSpec",
        Vec::new(),
    )];
    let mut algorithm_components = Vec::new();
    let mut unresolved_patterns = Vec::new();
    let execution_plan = program
        .compile_execution_plan()
        .map_err(|error| format!("Form service activation failed: {error}"))?;
    let mut dataset_activated = service_override == Some(DATASET_SERVICE_ID);
    let mut shell_activated = service_override == Some(SHELL_SERVICE_ID);
    let mut recursion_activated = service_override == Some(RECURSION_SERVICE_ID);

    for pattern in &execution_plan.patterns {
        if let Some(alias) = algorithm_alias(pattern) {
            let suite = BuiltinComponentSuite::algorithms();
            let component = suite.find(alias).or_else(|| {
                alias
                    .rsplit_once('.')
                    .and_then(|(_, canonical_alias)| suite.find(canonical_alias))
            });
            match component {
                Some(component) => push_unique(&mut algorithm_components, *component.descriptor()),
                None => unresolved_patterns.push(pattern.clone()),
            }
        } else if is_dataset_pattern(pattern) {
            dataset_activated = true;
        } else if is_shell_pattern(pattern) {
            shell_activated = true;
        } else if is_recursion_pattern(pattern) {
            recursion_activated = true;
        }
    }

    let algorithm_activated = service_override == Some(ALGORITHM_SERVICE_ID)
        || !algorithm_components.is_empty()
        || !unresolved_patterns.is_empty();
    let pagerank_shell_actual = algorithm_components
        .iter()
        .any(|component| component.id.as_str() == "gds.algorithms.centrality.pagerank");
    if algorithm_activated {
        let execution_state = if algorithm_components.is_empty() {
            FormServiceExecutionState::Planned
        } else {
            FormServiceExecutionState::Bindable
        };
        machines.push(FormServiceMachine::new(
            ALGORITHM_SERVICE_ID,
            "Resolve algorithm operators into Shell-bindable component plans",
            execution_state,
            "ShellProcedureRuntime",
            algorithm_components.clone(),
        ));
    }

    if dataset_activated {
        machines.push(FormServiceMachine::new(
            DATASET_SERVICE_ID,
            "Compile and materialize dataset-shaped Programs",
            FormServiceExecutionState::Planned,
            "DatasetCompilationRuntime",
            Vec::new(),
        ));
    }

    if recursion_activated {
        machines.push(
            FormServiceMachine::new(
                RECURSION_SERVICE_ID,
                "Apply triadic recursive mediation over Form execution cycles",
                FormServiceExecutionState::Planned,
                "FormTriadicRecursionRuntime",
                Vec::new(),
            )
            .with_daemon_runtime(ShellDaemonRuntimeProfile::new(
                "long-lived",
                "restart_on_failure",
                15_000,
                "iterative_checkpoint",
            )),
        );
    }

    if algorithm_activated || dataset_activated || recursion_activated {
        shell_activated = true;
    }

    if shell_activated {
        let shell_machine = if pagerank_shell_actual {
            FormServiceMachine::new(
                SHELL_SERVICE_ID,
                "Mediate canonical Form PageRank plans through Shell procedures",
                FormServiceExecutionState::Actual,
                "ShellProcedureRuntime",
                algorithm_components
                    .iter()
                    .copied()
                    .filter(|component| {
                        component.id.as_str() == "gds.algorithms.centrality.pagerank"
                    })
                    .collect(),
            )
        } else {
            FormServiceMachine::new(
                SHELL_SERVICE_ID,
                "Run Form plans as long-lived task-daemon processes with supervision",
                FormServiceExecutionState::Planned,
                "ShellTaskDaemonRuntime",
                Vec::new(),
            )
            .with_daemon_runtime(ShellDaemonRuntimeProfile::new(
                "long-lived",
                "restart_on_failure",
                30_000,
                "periodic_snapshot",
            ))
        };
        machines.push(shell_machine);
    }

    Ok(FormServiceManifest::new(machines, unresolved_patterns))
}

fn algorithm_alias(pattern: &str) -> Option<&str> {
    let pattern = pattern.trim();
    for prefix in ["algo.", "algorithm.", "applications.algorithms."] {
        if let Some(alias) = pattern.strip_prefix(prefix) {
            return (!alias.is_empty()).then_some(alias);
        }
    }

    (!pattern.is_empty() && !pattern.contains('.')).then_some(pattern)
}

fn is_dataset_pattern(pattern: &str) -> bool {
    pattern.starts_with("dataset.")
        || pattern.starts_with("dataframe.")
        || pattern.starts_with("collections.")
}

fn is_shell_pattern(pattern: &str) -> bool {
    pattern.starts_with("shell.")
        || pattern.starts_with("task.")
        || pattern.starts_with("procedure.")
}

fn is_recursion_pattern(pattern: &str) -> bool {
    pattern.starts_with("recursion.")
        || pattern.starts_with("recursive.")
        || pattern.starts_with("triad.")
        || pattern.starts_with("triadic.")
        || pattern.starts_with("kernel.triadic.")
}

fn push_unique(
    components: &mut Vec<ShellComponentDescriptor>,
    descriptor: ShellComponentDescriptor,
) {
    if !components.iter().any(|current| current.id == descriptor.id) {
        components.push(descriptor);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::form::ApplicationForm;
    use crate::form::Context;
    use crate::form::FormShape;
    use crate::form::Morph;
    use crate::form::Shape;
    use crate::form::Specification;
    use crate::shell::ShellComponentCategory;
    use crate::shell::ShellComponentMode;
    use std::collections::HashMap;

    fn program(patterns: Vec<&str>) -> ProgramSpec {
        ProgramSpec::new(
            FormShape::new(
                Shape::default(),
                Context::default(),
                Morph::new(patterns.into_iter().map(str::to_string).collect()),
            ),
            Specification::new("form.program".to_string(), None, HashMap::new()),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
    }

    #[test]
    fn advertises_program_and_shell_bindable_pagerank_machines() {
        let manifest = service_manifest(&program(vec!["base.normalize", "algo.pagerank"]), None)
            .expect("manifest should resolve");

        assert_eq!(
            manifest.activated_services,
            vec!["form.program", "form.algorithms", "form.shell"]
        );
        assert_eq!(
            manifest.machines[0].execution_state,
            FormServiceExecutionState::Actual
        );
        assert_eq!(
            manifest.machines[1].execution_state,
            FormServiceExecutionState::Bindable
        );
        assert_eq!(manifest.machines[1].components.len(), 1);

        let pagerank = manifest.machines[1].components[0];
        assert_eq!(pagerank.id.as_str(), "gds.algorithms.centrality.pagerank");
        assert_eq!(pagerank.category, ShellComponentCategory::Centrality);
        assert!(pagerank.modes.contains(&ShellComponentMode::Stream));
        assert!(manifest.unresolved_patterns.is_empty());

        let shell = manifest
            .machines
            .iter()
            .find(|machine| machine.service_id == SHELL_SERVICE_ID)
            .expect("shell machine expected");
        assert_eq!(shell.execution_state, FormServiceExecutionState::Actual);
        assert_eq!(shell.runtime_binding, "ShellProcedureRuntime");
        assert_eq!(shell.components, vec![pagerank]);
        assert!(shell.daemon_runtime.is_none());
    }

    #[test]
    fn activates_dataset_machine_without_claiming_execution() {
        let manifest = service_manifest(
            &program(vec!["dataframe.seed", "dataset.model", "dataset.plan"]),
            None,
        )
        .expect("manifest should resolve");

        assert_eq!(
            manifest.activated_services,
            vec!["form.program", "form.datasets", "form.shell"]
        );
        assert_eq!(
            manifest.machines[1].execution_state,
            FormServiceExecutionState::Planned
        );
    }

    #[test]
    fn service_override_activates_a_known_planned_machine() {
        let manifest = service_manifest(&program(vec!["base.normalize"]), Some("form.algorithms"))
            .expect("known override should activate");

        assert_eq!(
            manifest.activated_services,
            vec!["form.program", "form.algorithms", "form.shell"]
        );
        assert_eq!(
            manifest.machines[1].execution_state,
            FormServiceExecutionState::Planned
        );
        assert!(service_manifest(&program(vec!["base.normalize"]), Some("form.unknown")).is_err());
    }

    #[test]
    fn preserves_unresolved_algorithm_patterns() {
        let manifest = service_manifest(&program(vec!["algo.not_registered"]), None)
            .expect("unresolved patterns are evidence, not activation errors");

        assert_eq!(manifest.unresolved_patterns, vec!["algo.not_registered"]);
        assert_eq!(
            manifest.machines[1].execution_state,
            FormServiceExecutionState::Planned
        );
    }

    #[test]
    fn infers_algorithm_machine_from_selected_application_form() {
        let mut program = program(vec!["base.normalize"]);
        program.application_forms = vec![ApplicationForm::new(
            "centrality".to_string(),
            "graph-ml".to_string(),
            Vec::new(),
            vec!["applications.algorithms.centrality.pagerank".to_string()],
            HashMap::new(),
        )];
        program.selected_forms = vec!["centrality".to_string()];

        let manifest = service_manifest(&program, None).expect("manifest should resolve");

        assert_eq!(
            manifest.activated_services,
            vec!["form.program", "form.algorithms", "form.shell"]
        );
        assert_eq!(
            manifest.machines[1].components[0].id.as_str(),
            "gds.algorithms.centrality.pagerank"
        );
    }

    #[test]
    fn explicit_shell_service_override_activates_daemon_contract() {
        let manifest = service_manifest(&program(vec!["base.normalize"]), Some("form.shell"))
            .expect("shell override should activate");

        assert_eq!(
            manifest.activated_services,
            vec!["form.program", "form.shell"]
        );

        let shell = manifest
            .machines
            .iter()
            .find(|machine| machine.service_id == SHELL_SERVICE_ID)
            .expect("shell machine expected");
        assert_eq!(shell.execution_state, FormServiceExecutionState::Planned);
        assert_eq!(shell.runtime_binding, "ShellTaskDaemonRuntime");
        assert_eq!(
            shell
                .daemon_runtime
                .as_ref()
                .map(|profile| profile.supervision_policy.as_str()),
            Some("restart_on_failure")
        );
    }

    #[test]
    fn triadic_recursion_pattern_activates_recursion_and_shell_machines() {
        let manifest = service_manifest(&program(vec!["triadic.reflective_cycle"]), None)
            .expect("triadic recursion should activate");

        assert_eq!(
            manifest.activated_services,
            vec!["form.program", "form.recursion", "form.shell"]
        );

        let recursion = manifest
            .machines
            .iter()
            .find(|machine| machine.service_id == RECURSION_SERVICE_ID)
            .expect("recursion machine expected");
        assert_eq!(recursion.runtime_binding, "FormTriadicRecursionRuntime");
        assert_eq!(
            recursion
                .daemon_runtime
                .as_ref()
                .map(|profile| profile.checkpoint_policy.as_str()),
            Some("iterative_checkpoint")
        );
    }

    #[test]
    fn recursion_service_override_activates_planned_machine() {
        let manifest = service_manifest(&program(vec!["base.normalize"]), Some("form.recursion"))
            .expect("recursion override should activate");

        assert_eq!(
            manifest.activated_services,
            vec!["form.program", "form.recursion", "form.shell"]
        );
    }
}
