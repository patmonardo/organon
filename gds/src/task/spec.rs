//! Validated execution contracts owned by the Task system.

use std::collections::HashSet;

use crate::task::runtime::TaskStage;

/// Objective evidence addressed by an Agent-facing task controller.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskObjectiveRef {
    source: String,
    identity: String,
}

impl TaskObjectiveRef {
    pub fn new(source: impl Into<String>, identity: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            identity: identity.into(),
        }
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }
}

/// Whether the expected task return remains immediate or requires persistence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskReturnPolicy {
    Ephemeral,
    Persisted,
}

/// Monitoring detail attached to a task execution contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskMonitoringLevel {
    Off,
    Basic,
    Detailed,
}

/// Expected outputs and persistence semantics of a task workflow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskReturnContract {
    outputs: Vec<String>,
    policy: TaskReturnPolicy,
}

impl TaskReturnContract {
    pub fn ephemeral(outputs: Vec<String>) -> Self {
        Self {
            outputs,
            policy: TaskReturnPolicy::Ephemeral,
        }
    }

    pub fn persisted(outputs: Vec<String>) -> Self {
        Self {
            outputs,
            policy: TaskReturnPolicy::Persisted,
        }
    }

    pub fn outputs(&self) -> &[String] {
        &self.outputs
    }

    pub fn policy(&self) -> TaskReturnPolicy {
        self.policy
    }

    pub fn requires_persistence(&self) -> bool {
        self.policy == TaskReturnPolicy::Persisted
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum TaskSpecError {
    #[error("TaskWorkflow must contain at least one stage")]
    EmptyPlan,

    #[error("TaskSpec namespace must not be empty")]
    EmptyNamespace,

    #[error("TaskStage at index {index} has namespace `{actual}`, expected `{expected}`")]
    NamespaceMismatch {
        index: usize,
        expected: String,
        actual: String,
    },

    #[error("TaskStage `{stage}` does not consume an output of the preceding stage")]
    DiscontinuousStage { stage: String },
}

/// Ordered execution stages validated as one Task plan.
#[derive(Debug, Clone)]
pub struct TaskWorkflow {
    frames: Vec<TaskStage>,
}

impl TaskWorkflow {
    pub fn new(frames: Vec<TaskStage>) -> Result<Self, TaskSpecError> {
        let first = frames.first().ok_or(TaskSpecError::EmptyPlan)?;
        let namespace = first.namespace();

        for (index, frame) in frames.iter().enumerate().skip(1) {
            if frame.namespace() != namespace {
                return Err(TaskSpecError::NamespaceMismatch {
                    index,
                    expected: namespace.to_string(),
                    actual: frame.namespace().to_string(),
                });
            }

            let previous_outputs: HashSet<&str> = frames[index - 1]
                .io_spec()
                .outputs()
                .iter()
                .map(String::as_str)
                .collect();
            let consumes_previous_output = frame
                .io_spec()
                .inputs()
                .iter()
                .any(|input| previous_outputs.contains(input.as_str()));

            if !previous_outputs.is_empty()
                && !frame.io_spec().inputs().is_empty()
                && !consumes_previous_output
            {
                return Err(TaskSpecError::DiscontinuousStage {
                    stage: frame.description(),
                });
            }
        }

        Ok(Self { frames })
    }

    pub fn frames(&self) -> &[TaskStage] {
        &self.frames
    }

    pub fn len(&self) -> usize {
        self.frames.len()
    }

    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }

    pub fn into_frames(self) -> Vec<TaskStage> {
        self.frames
    }
}

/// Validated contract submitted to Task execution infrastructure.
#[derive(Debug, Clone)]
pub struct TaskSpec {
    namespace: String,
    task_name: String,
    monitoring_level: TaskMonitoringLevel,
    workflow: TaskWorkflow,
    objective: TaskObjectiveRef,
    return_contract: TaskReturnContract,
}

impl TaskSpec {
    fn default_task_name(namespace: &str, workflow: &TaskWorkflow) -> String {
        format!(
            "{}::{}",
            namespace,
            workflow
                .frames()
                .first()
                .map(|frame| frame.pipeline().to_string())
                .unwrap_or_default()
        )
    }

    pub fn new(
        namespace: impl Into<String>,
        workflow: TaskWorkflow,
    ) -> Result<Self, TaskSpecError> {
        let namespace = namespace.into();
        let outputs = workflow
            .frames()
            .last()
            .map(|frame| frame.io_spec().outputs().to_vec())
            .unwrap_or_default();
        let objective = TaskObjectiveRef::new("task", namespace.clone());
        let return_contract = TaskReturnContract::ephemeral(outputs);
        Self::with_control_contract(namespace, workflow, objective, return_contract)
    }

    pub fn with_named_control_contract(
        namespace: impl Into<String>,
        task_name: impl Into<String>,
        workflow: TaskWorkflow,
        objective: TaskObjectiveRef,
        return_contract: TaskReturnContract,
    ) -> Result<Self, TaskSpecError> {
        let namespace = namespace.into();
        let task_name = task_name.into();
        if namespace.trim().is_empty() {
            return Err(TaskSpecError::EmptyNamespace);
        }

        for (index, frame) in workflow.frames().iter().enumerate() {
            if frame.namespace() != namespace {
                return Err(TaskSpecError::NamespaceMismatch {
                    index,
                    expected: namespace.clone(),
                    actual: frame.namespace().to_string(),
                });
            }
        }

        Ok(Self {
            namespace,
            task_name,
            monitoring_level: TaskMonitoringLevel::Basic,
            workflow,
            objective,
            return_contract,
        })
    }

    pub fn with_control_contract(
        namespace: impl Into<String>,
        workflow: TaskWorkflow,
        objective: TaskObjectiveRef,
        return_contract: TaskReturnContract,
    ) -> Result<Self, TaskSpecError> {
        let namespace = namespace.into();
        let task_name = Self::default_task_name(&namespace, &workflow);

        Self::with_named_control_contract(
            namespace,
            task_name,
            workflow,
            objective,
            return_contract,
        )
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn task_name(&self) -> &str {
        &self.task_name
    }

    pub fn monitoring_level(&self) -> TaskMonitoringLevel {
        self.monitoring_level
    }

    pub fn with_monitoring_level(mut self, monitoring_level: TaskMonitoringLevel) -> Self {
        self.monitoring_level = monitoring_level;
        self
    }

    pub fn workflow(&self) -> &TaskWorkflow {
        &self.workflow
    }

    pub fn objective(&self) -> &TaskObjectiveRef {
        &self.objective
    }

    pub fn return_contract(&self) -> &TaskReturnContract {
        &self.return_contract
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::concurrency::Concurrency;

    fn frame(pipeline: &str, inputs: Vec<&str>, outputs: Vec<&str>) -> TaskStage {
        TaskStage::new(
            "graphframe".to_string(),
            pipeline.to_string(),
            vec![pipeline.to_string()],
            1,
            Concurrency::of(1),
        )
        .with_inputs(inputs.into_iter().map(str::to_string).collect())
        .with_outputs(outputs.into_iter().map(str::to_string).collect())
    }

    #[test]
    fn task_spec_accepts_an_ordered_workflow() {
        let plan = TaskWorkflow::new(vec![
            frame("seed", vec!["graphstore"], vec!["seed"]),
            frame("compute", vec!["seed"], vec!["result"]),
            frame("persist", vec!["result"], vec!["dataset"]),
        ])
        .unwrap();

        let spec = TaskSpec::new("graphframe", plan).unwrap();

        assert_eq!(spec.namespace(), "graphframe");
        assert_eq!(spec.workflow().len(), 3);
    }

    #[test]
    fn task_frame_plan_rejects_empty_plans() {
        assert!(matches!(
            TaskWorkflow::new(Vec::new()),
            Err(TaskSpecError::EmptyPlan)
        ));
    }

    #[test]
    fn task_frame_plan_rejects_discontinuous_stages() {
        let result = TaskWorkflow::new(vec![
            frame("seed", vec!["graphstore"], vec!["seed"]),
            frame("compute", vec!["other"], vec!["result"]),
        ]);

        assert!(matches!(
            result,
            Err(TaskSpecError::DiscontinuousStage { .. })
        ));
    }
}
