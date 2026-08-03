//! Convenience imports for the unified task surface.

pub mod full;

pub use crate::task::concurrency::Concurrency;
pub use crate::task::concurrency::Executor;
pub use crate::task::concurrency::TerminatedException;
pub use crate::task::concurrency::TerminationFlag;
pub use crate::task::concurrency::OPEN_GDS_DEFAULT_CONCURRENCY;
pub use crate::task::daemon::TaskDaemon;
pub use crate::task::evaluator::TaskEvaluator;
pub use crate::task::evaluator::TaskExecutionContext;
pub use crate::task::frame::TaskFrame;
pub use crate::task::frame::TaskObjectiveRef;
pub use crate::task::frame::TaskReturnContract;
pub use crate::task::frame::TaskReturnPolicy;
pub use crate::task::job::TaskJob;
pub use crate::task::job::TaskJobReceipt;
pub use crate::task::job::TaskJobState;

pub use crate::task::memory::Estimate;
pub use crate::task::memory::MemoryEstimation;
pub use crate::task::memory::MemoryEstimationResult;
pub use crate::task::memory::MemoryEstimations;
pub use crate::task::memory::MemoryRange;
pub use crate::task::memory::MemoryTree;
pub use crate::task::memory::TaskMemoryContainer;
pub use crate::task::runtime::TaskRuntime;
pub use crate::task::runtime::TaskStage;
pub use crate::task::spec::TaskSpec;
pub use crate::task::spec::TaskSpecError;
pub use crate::task::spec::TaskWorkflow;
pub use crate::task::workbench::task_workbench_track;
pub use crate::task::workbench::task_workbench_tracks;
pub use crate::task::workbench::TaskWorkbenchTrack;

pub use crate::task::pregel::*;
pub use crate::task::progress::*;
