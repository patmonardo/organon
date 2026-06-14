use crate::task::concurrency::TerminationFlag;
use std::error::Error as StdError;

/// Trainer for ML pipelines with termination support.
///
/// This trait represents the core training logic for ML pipelines,
/// including model selection, hyperparameter tuning, and training.
///
/// # Type Parameters
///
/// * `RESULT` - The training result type (e.g., trained model, metrics)
///
pub trait PipelineTrainer {
    /// The result type produced by training.
    type Result;

    /// Set the termination flag used by this trainer.
    ///
    fn set_termination_flag(&mut self, _termination_flag: TerminationFlag) {}

    /// Run the training process.
    ///
    /// This executes model selection, hyperparameter tuning, and training
    /// to produce a trained model and metrics.
    ///
    fn run(&mut self) -> Result<Self::Result, Box<dyn StdError + Send + Sync>>;

    /// Check if training has been terminated.
    ///
    /// Training can be stopped early via termination signals.
    /// Implementations should check this periodically during training.
    ///
    fn is_terminated(&self) -> bool {
        false // Default: no termination
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockTrainer {
        result: String,
        termination_flag: TerminationFlag,
    }

    impl PipelineTrainer for MockTrainer {
        type Result = String;

        fn set_termination_flag(&mut self, termination_flag: TerminationFlag) {
            self.termination_flag = termination_flag;
        }

        fn run(&mut self) -> Result<Self::Result, Box<dyn StdError + Send + Sync>> {
            if self.is_terminated() {
                return Err("Training terminated".into());
            }
            Ok(self.result.clone())
        }

        fn is_terminated(&self) -> bool {
            !self.termination_flag.running()
        }
    }

    #[test]
    fn test_pipeline_trainer_run() {
        let mut trainer = MockTrainer {
            result: "trained model".to_string(),
            termination_flag: TerminationFlag::running_true(),
        };

        let result = trainer.run();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "trained model");
    }

    #[test]
    fn test_pipeline_trainer_terminated() {
        let mut trainer = MockTrainer {
            result: "trained model".to_string(),
            termination_flag: TerminationFlag::stop_running(),
        };

        let result = trainer.run();
        assert!(result.is_err());
    }

    #[test]
    fn test_pipeline_trainer_set_termination_flag() {
        let mut trainer = MockTrainer {
            result: "trained model".to_string(),
            termination_flag: TerminationFlag::running_true(),
        };

        trainer.set_termination_flag(TerminationFlag::stop_running());

        assert!(trainer.is_terminated());
    }
}
