//! Synchronous Task lifecycle authority.

use crate::task::concurrency::TerminationFlag;
use crate::task::evaluator::TaskEvaluator;
use crate::task::evaluator::TaskExecutionContext;
use crate::task::job::TaskJob;
use crate::task::job::TaskJobReceipt;
use crate::task::spec::TaskSpec;

#[derive(Debug, Clone, Copy, Default)]
pub struct TaskDaemon;

impl TaskDaemon {
    pub fn new() -> Self {
        Self
    }

    pub fn submit<Program>(
        &self,
        owner: impl Into<String>,
        spec: TaskSpec,
        program: Program,
    ) -> TaskJob<Program> {
        TaskJob::new(owner, spec, program)
    }

    pub fn run<Program, Evaluator>(
        &self,
        job: TaskJob<Program>,
        evaluator: &Evaluator,
        termination: TerminationFlag,
    ) -> TaskJobReceipt<Evaluator::Output>
    where
        Evaluator: TaskEvaluator<Program>,
    {
        let receipt_spec = job.spec().clone();
        let context = TaskExecutionContext::new(job.job_id(), job.owner(), job.spec(), termination);
        context.push_trace("task.eval.begin");

        if !context.is_running() {
            context.push_trace("task.eval.end");
            return TaskJobReceipt::canceled(
                job.job_id().clone(),
                job.owner().to_string(),
                receipt_spec,
                context.trace(),
            );
        }

        match evaluator.evaluate(job.program(), &context) {
            Ok(output) if context.is_running() => {
                context.push_trace("task.eval.end");
                TaskJobReceipt::succeeded(
                    job.job_id().clone(),
                    job.owner().to_string(),
                    receipt_spec,
                    output,
                    context.trace(),
                )
            }
            Ok(_) => {
                context.push_trace("task.eval.end");
                TaskJobReceipt::canceled(
                    job.job_id().clone(),
                    job.owner().to_string(),
                    receipt_spec,
                    context.trace(),
                )
            }
            Err(error) => {
                context.push_trace(format!(
                    "task.eval.error class={} message={}",
                    evaluator.error_classification(&error),
                    error
                ));
                context.push_trace("task.eval.end");
                TaskJobReceipt::failed(
                    job.job_id().clone(),
                    job.owner().to_string(),
                    receipt_spec,
                    error.to_string(),
                    context.trace(),
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::concurrency::Concurrency;
    use crate::task::evaluator::TaskExecutionContext;
    use crate::task::job::TaskJobState;
    use crate::task::runtime::TaskStage;
    use crate::task::spec::TaskMonitoringLevel;
    use crate::task::spec::TaskWorkflow;

    struct TestEvaluator {
        fail: bool,
    }

    impl TaskEvaluator<String> for TestEvaluator {
        type Output = String;
        type Error = &'static str;

        fn evaluate(
            &self,
            program: &String,
            context: &TaskExecutionContext<'_>,
        ) -> Result<Self::Output, Self::Error> {
            assert_eq!(context.owner(), "organon");
            if self.fail {
                Err("evaluation failed")
            } else {
                Ok(format!("{}:{}", context.spec().namespace(), program))
            }
        }
    }

    fn spec() -> TaskSpec {
        let frame = TaskStage::new(
            "task".to_string(),
            "pipeline::Test".to_string(),
            vec!["evaluate".to_string()],
            1,
            Concurrency::of(1),
        );
        TaskSpec::new("task", TaskWorkflow::new(vec![frame]).unwrap()).unwrap()
    }

    #[test]
    fn task_daemon_records_evaluator_trace_in_receipt() {
        let daemon = TaskDaemon::new();
        let job = daemon.submit("organon", spec(), "program".to_string());

        let receipt = daemon.run(
            job,
            &TestEvaluator { fail: false },
            TerminationFlag::running_true(),
        );

        assert_eq!(receipt.state(), TaskJobState::Succeeded);
        assert!(receipt
            .trace()
            .iter()
            .any(|entry| entry.contains("task=task::pipeline::Test")));
        assert!(receipt
            .trace()
            .iter()
            .any(|entry| entry.contains("event=task.eval.begin")));
        assert!(receipt
            .trace()
            .iter()
            .any(|entry| entry.contains("task.eval.end")));
    }

    #[test]
    fn task_daemon_receipt_exposes_task_name() {
        let daemon = TaskDaemon::new();
        let job = daemon.submit("organon", spec(), "program".to_string());

        let receipt = daemon.run(
            job,
            &TestEvaluator { fail: false },
            TerminationFlag::running_true(),
        );

        assert_eq!(receipt.task_name(), "task::pipeline::Test");
    }

    #[test]
    fn task_daemon_returns_success_receipt() {
        let daemon = TaskDaemon::new();
        let job = daemon.submit("organon", spec(), "program".to_string());
        let job_id = job.job_id().clone();

        let receipt = daemon.run(
            job,
            &TestEvaluator { fail: false },
            TerminationFlag::running_true(),
        );

        assert_eq!(receipt.job_id(), &job_id);
        assert_eq!(receipt.state(), TaskJobState::Succeeded);
        assert_eq!(receipt.output().map(String::as_str), Some("task:program"));
        assert_eq!(receipt.spec().namespace(), "task");
        assert_eq!(receipt.spec().workflow().len(), 1);
        assert_eq!(receipt.spec().objective().source(), "task");
    }

    #[test]
    fn task_daemon_records_evaluator_failure() {
        let daemon = TaskDaemon::new();
        let job = daemon.submit("organon", spec(), "program".to_string());

        let receipt = daemon.run(
            job,
            &TestEvaluator { fail: true },
            TerminationFlag::running_true(),
        );

        assert_eq!(receipt.state(), TaskJobState::Failed);
        assert_eq!(receipt.error(), Some("evaluation failed"));
        assert!(receipt.trace().iter().any(|entry| entry
            .contains("event=task.eval.error class=evaluation message=evaluation failed")));
        assert_eq!(
            receipt.spec().workflow().frames()[0].pipeline(),
            "pipeline::Test"
        );
    }

    #[test]
    fn task_daemon_honors_cancellation_before_evaluation() {
        let daemon = TaskDaemon::new();
        let job = daemon.submit("organon", spec(), "program".to_string());

        let receipt = daemon.run(
            job,
            &TestEvaluator { fail: false },
            TerminationFlag::stop_running(),
        );

        assert_eq!(receipt.state(), TaskJobState::Canceled);
        assert!(receipt.output().is_none());
        assert!(receipt.spec().return_contract().outputs().is_empty());
    }

    #[test]
    fn task_daemon_can_disable_trace_collection() {
        let daemon = TaskDaemon::new();
        let job = daemon.submit(
            "organon",
            spec().with_monitoring_level(TaskMonitoringLevel::Off),
            "program".to_string(),
        );

        let receipt = daemon.run(
            job,
            &TestEvaluator { fail: false },
            TerminationFlag::running_true(),
        );

        assert_eq!(receipt.state(), TaskJobState::Succeeded);
        assert!(receipt.trace().is_empty());
    }
}
