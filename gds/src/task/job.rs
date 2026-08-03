//! Job identity and evidence-bearing execution receipts.

use crate::task::progress::JobId;
use crate::task::spec::TaskSpec;

#[derive(Debug, Clone)]
pub struct TaskJob<Program> {
    job_id: JobId,
    owner: String,
    spec: TaskSpec,
    program: Program,
}

impl<Program> TaskJob<Program> {
    pub fn new(owner: impl Into<String>, spec: TaskSpec, program: Program) -> Self {
        Self::with_job_id(JobId::new(), owner, spec, program)
    }

    pub fn with_job_id(
        job_id: JobId,
        owner: impl Into<String>,
        spec: TaskSpec,
        program: Program,
    ) -> Self {
        Self {
            job_id,
            owner: owner.into(),
            spec,
            program,
        }
    }

    pub fn job_id(&self) -> &JobId {
        &self.job_id
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn spec(&self) -> &TaskSpec {
        &self.spec
    }

    pub fn program(&self) -> &Program {
        &self.program
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskJobState {
    Accepted,
    Running,
    Succeeded,
    Failed,
    Canceled,
}

impl TaskJobState {
    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Succeeded | Self::Failed | Self::Canceled)
    }
}

#[derive(Debug, Clone)]
pub struct TaskJobReceipt<Output> {
    job_id: JobId,
    owner: String,
    spec: TaskSpec,
    state: TaskJobState,
    output: Option<Output>,
    error: Option<String>,
}

impl<Output> TaskJobReceipt<Output> {
    pub(crate) fn succeeded(job_id: JobId, owner: String, spec: TaskSpec, output: Output) -> Self {
        Self {
            job_id,
            owner,
            spec,
            state: TaskJobState::Succeeded,
            output: Some(output),
            error: None,
        }
    }

    pub(crate) fn failed(job_id: JobId, owner: String, spec: TaskSpec, error: String) -> Self {
        Self {
            job_id,
            owner,
            spec,
            state: TaskJobState::Failed,
            output: None,
            error: Some(error),
        }
    }

    pub(crate) fn canceled(job_id: JobId, owner: String, spec: TaskSpec) -> Self {
        Self {
            job_id,
            owner,
            spec,
            state: TaskJobState::Canceled,
            output: None,
            error: None,
        }
    }

    pub fn job_id(&self) -> &JobId {
        &self.job_id
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn spec(&self) -> &TaskSpec {
        &self.spec
    }

    pub fn state(&self) -> TaskJobState {
        self.state
    }

    pub fn output(&self) -> Option<&Output> {
        self.output.as_ref()
    }

    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }
}
