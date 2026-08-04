//! Agent-facing TaskFrame controller.
//!
//! TaskFrame is the shell-facing half of the PureForm operating system. It does
//! not define graph semantics itself; instead it receives a graph-form intent
//! from GraphFrame and turns it into a staged workflow that can be executed by
//! the command shell and its procedure runtime.

use crate::collections::graphframe::GraphExecutionIntent;
use crate::shell::ShellComponentMode;
use crate::task::concurrency::Concurrency;
use crate::task::job::TaskJob;
use crate::task::runtime::TaskFrameKind;
use crate::task::runtime::TaskFrameStorageBackend;
use crate::task::runtime::TaskStage;
use crate::task::spec::TaskMonitoringLevel;
use crate::task::spec::TaskSpec;
use crate::task::spec::TaskSpecError;
use crate::task::spec::TaskWorkflow;

pub use crate::task::spec::TaskObjectiveRef;
pub use crate::task::spec::TaskReturnContract;
pub use crate::task::spec::TaskReturnPolicy;

/// Agent policy used when constituting an objective intent as a TaskFrame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TaskFramePolicy {
    concurrency: Concurrency,
    monitoring_level: TaskMonitoringLevel,
}

impl TaskFramePolicy {
    pub fn new(concurrency: usize) -> Self {
        Self {
            concurrency: Concurrency::of(concurrency.max(1)),
            monitoring_level: TaskMonitoringLevel::Detailed,
        }
    }

    pub fn concurrency(&self) -> Concurrency {
        self.concurrency
    }

    pub fn monitoring_level(&self) -> TaskMonitoringLevel {
        self.monitoring_level
    }

    pub fn with_monitoring_level(mut self, monitoring_level: TaskMonitoringLevel) -> Self {
        self.monitoring_level = monitoring_level;
        self
    }
}

/// Agent-facing controller that constitutes objective intent as a Task workflow.
#[derive(Debug, Clone)]
pub struct TaskFrame<Program> {
    namespace: String,
    task_name: Option<String>,
    monitoring_level: Option<TaskMonitoringLevel>,
    objective: TaskObjectiveRef,
    workflow: TaskWorkflow,
    program: Program,
    return_contract: TaskReturnContract,
}

impl<Program> TaskFrame<Program> {
    pub fn new(
        namespace: impl Into<String>,
        objective: TaskObjectiveRef,
        workflow: TaskWorkflow,
        program: Program,
        return_contract: TaskReturnContract,
    ) -> Self {
        Self {
            namespace: namespace.into(),
            task_name: None,
            monitoring_level: None,
            objective,
            workflow,
            program,
            return_contract,
        }
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn objective(&self) -> &TaskObjectiveRef {
        &self.objective
    }

    pub fn task_name(&self) -> Option<&str> {
        self.task_name.as_deref()
    }

    pub fn workflow(&self) -> &TaskWorkflow {
        &self.workflow
    }

    pub fn program(&self) -> &Program {
        &self.program
    }

    pub fn return_contract(&self) -> &TaskReturnContract {
        &self.return_contract
    }

    pub fn with_task_name(mut self, task_name: impl Into<String>) -> Self {
        self.task_name = Some(task_name.into());
        self
    }

    pub fn with_monitoring_level(mut self, monitoring_level: TaskMonitoringLevel) -> Self {
        self.monitoring_level = Some(monitoring_level);
        self
    }

    pub fn validate(&self) -> Result<(), TaskSpecError> {
        let spec = if let Some(task_name) = self.task_name.clone() {
            TaskSpec::with_named_control_contract(
                self.namespace.clone(),
                task_name,
                self.workflow.clone(),
                self.objective.clone(),
                self.return_contract.clone(),
            )
        } else {
            TaskSpec::with_control_contract(
                self.namespace.clone(),
                self.workflow.clone(),
                self.objective.clone(),
                self.return_contract.clone(),
            )
        }?;

        let _spec = if let Some(monitoring_level) = self.monitoring_level {
            spec.with_monitoring_level(monitoring_level)
        } else {
            spec
        };

        Ok(())
    }

    pub fn into_spec(self) -> Result<TaskSpec, TaskSpecError> {
        let Self {
            namespace,
            task_name,
            monitoring_level,
            objective,
            workflow,
            program: _,
            return_contract,
        } = self;

        let spec = if let Some(task_name) = task_name {
            TaskSpec::with_named_control_contract(
                namespace,
                task_name,
                workflow,
                objective,
                return_contract,
            )
        } else {
            TaskSpec::with_control_contract(namespace, workflow, objective, return_contract)
        }?;

        Ok(if let Some(monitoring_level) = monitoring_level {
            spec.with_monitoring_level(monitoring_level)
        } else {
            spec
        })
    }

    pub fn into_job(self, owner: impl Into<String>) -> Result<TaskJob<Program>, TaskSpecError> {
        let Self {
            namespace,
            task_name,
            monitoring_level,
            objective,
            workflow,
            program,
            return_contract,
        } = self;
        let spec = if let Some(task_name) = task_name {
            TaskSpec::with_named_control_contract(
                namespace,
                task_name,
                workflow,
                objective,
                return_contract,
            )?
        } else {
            TaskSpec::with_control_contract(namespace, workflow, objective, return_contract)?
        };
        let spec = if let Some(monitoring_level) = monitoring_level {
            spec.with_monitoring_level(monitoring_level)
        } else {
            spec
        };
        Ok(TaskJob::new(owner, spec, program))
    }
}

impl TaskFrame<GraphExecutionIntent> {
    pub fn from_graph_intent(
        intent: GraphExecutionIntent,
        policy: TaskFramePolicy,
    ) -> Result<Self, TaskSpecError> {
        let concurrency = policy.concurrency();
        let monitoring_level = policy.monitoring_level();
        let task_name = graph_execution_task_name(&intent);
        let objective = intent.objective().clone();
        let return_contract = intent.return_contract().clone();
        let seed = TaskStage::new(
            "graphframe".to_string(),
            "pipeline::GraphFrameSeed".to_string(),
            vec!["graphframe.view.compile".to_string()],
            1,
            concurrency,
        )
        .with_image_kind(TaskFrameKind::ShellProgram)
        .with_storage_backend(TaskFrameStorageBackend::GraphStore)
        .with_inputs(vec!["graphstore.view_spec".to_string()])
        .with_outputs(vec![
            "graphframe.seed".to_string(),
            format!(
                "graphframe.relationship_types.{}",
                intent.view_spec().relationship_types().len()
            ),
        ]);
        let compute = TaskStage::new(
            "graphframe".to_string(),
            "pipeline::GraphFrameCompute".to_string(),
            intent.compute_steps().to_vec(),
            intent.estimated_volume(),
            concurrency,
        )
        .with_image_kind(TaskFrameKind::GraphAlgorithm)
        .with_storage_backend(TaskFrameStorageBackend::GraphStore)
        .with_inputs(vec!["graphframe.seed".to_string()])
        .with_outputs(vec!["graphframe.compute.result".to_string()]);

        let mut stages = vec![seed, compute];
        if return_contract.requires_persistence() {
            stages.push(
                TaskStage::new(
                    "graphframe".to_string(),
                    "pipeline::GraphFramePersist".to_string(),
                    vec!["graphframe.persist.dataset".to_string()],
                    1,
                    concurrency,
                )
                .with_image_kind(TaskFrameKind::ProcedurePipeline)
                .with_storage_backend(TaskFrameStorageBackend::PolarsPropertyStore)
                .with_inputs(vec!["graphframe.compute.result".to_string()])
                .with_outputs(return_contract.outputs().to_vec())
                .with_side_effects(vec!["dataset.catalog.write".to_string()]),
            );
        }

        Ok(Self::new(
            "graphframe",
            objective,
            TaskWorkflow::new(stages)?,
            intent,
            return_contract,
        )
        .with_task_name(task_name)
        .with_monitoring_level(monitoring_level))
    }
}

fn graph_execution_task_name(intent: &GraphExecutionIntent) -> String {
    let plan = intent.program();
    let Some(primary_call) = plan.calls().first() else {
        return "graphframe::shell::noop".to_string();
    };

    let component_name = primary_call
        .descriptor()
        .map(|descriptor| descriptor.alias)
        .unwrap_or_else(|| primary_call.component.as_str());
    let suffix = if plan.len() > 1 {
        format!("+{}", plan.len() - 1)
    } else {
        String::new()
    };

    format!(
        "graphframe::shell::{}::{}{}",
        component_name,
        shell_mode_name(primary_call.mode),
        suffix
    )
}

fn shell_mode_name(mode: ShellComponentMode) -> &'static str {
    match mode {
        ShellComponentMode::Invoke => "invoke",
        ShellComponentMode::Stream => "stream",
        ShellComponentMode::Stats => "stats",
        ShellComponentMode::Estimate => "estimate",
        ShellComponentMode::Mutate => "mutate",
        ShellComponentMode::Write => "write",
    }
}

#[cfg(test)]
mod tests {
    use crate::task::concurrency::Concurrency;
    use crate::task::runtime::TaskStage;
    use crate::task::spec::TaskMonitoringLevel;
    use crate::task::spec::TaskSpecError;
    use crate::task::spec::TaskWorkflow;

    use super::TaskFrame;
    use super::TaskObjectiveRef;
    use super::TaskReturnContract;

    fn workflow(namespace: &str) -> TaskWorkflow {
        TaskWorkflow::new(vec![
            TaskStage::new(
                namespace.to_string(),
                "seed".to_string(),
                vec!["graphframe.view.compile".to_string()],
                1,
                Concurrency::of(1),
            )
            .with_inputs(vec!["graphstore".to_string()])
            .with_outputs(vec!["graphframe.seed".to_string()]),
            TaskStage::new(
                namespace.to_string(),
                "compute".to_string(),
                vec!["graphframe.procedure.pagerank".to_string()],
                1,
                Concurrency::of(1),
            )
            .with_inputs(vec!["graphframe.seed".to_string()])
            .with_outputs(vec!["pagerank.rows".to_string()]),
        ])
        .expect("workflow should be continuous")
    }

    #[test]
    fn controller_preserves_objective_workflow_program_and_return_contract() {
        let frame = TaskFrame::new(
            "graphframe",
            TaskObjectiveRef::new("graphstore", "social::selected-view"),
            workflow("graphframe"),
            "pagerank-program",
            TaskReturnContract::ephemeral(vec!["pagerank.rows".to_string()]),
        );

        assert_eq!(frame.namespace(), "graphframe");
        assert_eq!(frame.objective().source(), "graphstore");
        assert_eq!(frame.objective().identity(), "social::selected-view");
        assert_eq!(frame.workflow().len(), 2);
        assert_eq!(frame.program(), &"pagerank-program");
        assert_eq!(frame.return_contract().outputs(), &["pagerank.rows"]);
        assert!(!frame.return_contract().requires_persistence());
        assert!(frame.validate().is_ok());
    }

    #[test]
    fn controller_into_job_preserves_validated_spec_and_program() {
        let frame = TaskFrame::new(
            "graphframe",
            TaskObjectiveRef::new("graphstore", "social::selected-view"),
            workflow("graphframe"),
            "pagerank-program",
            TaskReturnContract::ephemeral(vec!["pagerank.rows".to_string()]),
        );

        let job = frame
            .into_job("organon")
            .expect("controller should validate");

        assert_eq!(job.owner(), "organon");
        assert_eq!(job.spec().namespace(), "graphframe");
        assert_eq!(job.spec().workflow().len(), 2);
        assert_eq!(job.spec().objective().identity(), "social::selected-view");
        assert_eq!(job.spec().return_contract().outputs(), &["pagerank.rows"]);
        assert_eq!(job.program(), &"pagerank-program");
        assert_eq!(job.spec().monitoring_level(), TaskMonitoringLevel::Basic);
    }

    #[test]
    fn controller_preserves_custom_task_name_in_job_spec() {
        let frame = TaskFrame::new(
            "graphframe",
            TaskObjectiveRef::new("graphstore", "social::selected-view"),
            workflow("graphframe"),
            "pagerank-program",
            TaskReturnContract::ephemeral(vec!["pagerank.rows".to_string()]),
        )
        .with_task_name("graphframe::shell::pagerank::stats");

        let job = frame
            .into_job("organon")
            .expect("controller should validate");

        assert_eq!(job.task_name(), "graphframe::shell::pagerank::stats");
    }

    #[test]
    fn controller_preserves_custom_monitoring_level_in_job_spec() {
        let frame = TaskFrame::new(
            "graphframe",
            TaskObjectiveRef::new("graphstore", "social::selected-view"),
            workflow("graphframe"),
            "pagerank-program",
            TaskReturnContract::ephemeral(vec!["pagerank.rows".to_string()]),
        )
        .with_monitoring_level(TaskMonitoringLevel::Detailed);

        let job = frame
            .into_job("organon")
            .expect("controller should validate");

        assert_eq!(job.spec().monitoring_level(), TaskMonitoringLevel::Detailed);
    }

    #[test]
    fn controller_rejects_a_workflow_from_another_namespace() {
        let frame = TaskFrame::new(
            "graphframe",
            TaskObjectiveRef::new("graphstore", "social::selected-view"),
            workflow("shell"),
            "pagerank-program",
            TaskReturnContract::ephemeral(vec!["pagerank.rows".to_string()]),
        );

        assert!(matches!(
            frame.validate(),
            Err(TaskSpecError::NamespaceMismatch { .. })
        ));
    }
}
