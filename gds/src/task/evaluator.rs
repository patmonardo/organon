//! Execution-instrument inversion for Task-owned lifecycle control.

use std::cell::RefCell;

use crate::task::concurrency::TerminationFlag;
use crate::task::progress::JobId;
use crate::task::spec::TaskMonitoringLevel;
use crate::task::spec::TaskSpec;

pub struct TaskExecutionContext<'a> {
    job_id: &'a JobId,
    owner: &'a str,
    spec: &'a TaskSpec,
    termination: TerminationFlag,
    trace: RefCell<Vec<String>>,
}

impl<'a> TaskExecutionContext<'a> {
    pub(crate) fn new(
        job_id: &'a JobId,
        owner: &'a str,
        spec: &'a TaskSpec,
        termination: TerminationFlag,
    ) -> Self {
        Self {
            job_id,
            owner,
            spec,
            termination,
            trace: RefCell::new(Vec::new()),
        }
    }

    pub fn job_id(&self) -> &JobId {
        self.job_id
    }

    pub fn owner(&self) -> &str {
        self.owner
    }

    pub fn spec(&self) -> &TaskSpec {
        self.spec
    }

    pub fn is_running(&self) -> bool {
        self.termination.running()
    }

    pub fn termination_flag(&self) -> &TerminationFlag {
        &self.termination
    }

    pub fn push_trace(&self, entry: impl Into<String>) {
        self.push_trace_at(TaskMonitoringLevel::Basic, entry);
    }

    pub fn push_trace_at(&self, level: TaskMonitoringLevel, entry: impl Into<String>) {
        if self.spec.monitoring_level() < level {
            return;
        }

        self.trace
            .borrow_mut()
            .push(self.format_trace_entry(entry.into()));
    }

    pub fn push_stage_trace_at(
        &self,
        level: TaskMonitoringLevel,
        stage: &str,
        entry: impl Into<String>,
    ) {
        if self.spec.monitoring_level() < level {
            return;
        }

        self.trace
            .borrow_mut()
            .push(self.format_stage_trace_entry(stage, entry.into()));
    }

    pub fn trace(&self) -> Vec<String> {
        self.trace.borrow().clone()
    }

    fn format_trace_entry(&self, event: String) -> String {
        format!(
            "task={} owner={} job={} event={}",
            self.spec.task_name(),
            self.owner,
            self.job_id,
            event
        )
    }

    fn format_stage_trace_entry(&self, stage: &str, event: String) -> String {
        format!(
            "task={} owner={} job={} stage={} event={}",
            self.spec.task_name(),
            self.owner,
            self.job_id,
            stage,
            event
        )
    }
}

pub trait TaskEvaluator<Program> {
    type Output;
    type Error: std::fmt::Display;

    fn error_classification(&self, _error: &Self::Error) -> &'static str {
        "evaluation"
    }

    fn evaluate(
        &self,
        program: &Program,
        context: &TaskExecutionContext<'_>,
    ) -> Result<Self::Output, Self::Error>;
}
