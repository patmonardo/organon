//! Execution-instrument inversion for Task-owned lifecycle control.

use std::cell::RefCell;

use crate::task::concurrency::TerminationFlag;
use crate::task::progress::JobId;
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
        self.trace.borrow_mut().push(entry.into());
    }

    pub fn trace(&self) -> Vec<String> {
        self.trace.borrow().clone()
    }
}

pub trait TaskEvaluator<Program> {
    type Output;
    type Error: std::fmt::Display;

    fn evaluate(
        &self,
        program: &Program,
        context: &TaskExecutionContext<'_>,
    ) -> Result<Self::Output, Self::Error>;
}
