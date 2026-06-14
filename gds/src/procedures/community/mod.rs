pub mod approx_max_kcut;
pub mod conductance;
pub mod k1coloring;
pub mod kcore;
pub mod kmeans;
pub mod label_propagation;
pub mod leiden;
pub mod louvain;
pub mod modularity;
pub mod modularity_optimization;
pub mod scc;
pub mod triangle;
pub mod wcc;

use crate::task::concurrency::Concurrency;
use crate::core::utils::progress::{
    EmptyTaskRegistryFactory, JobId, LeafTask, Task, TaskProgressTracker, TaskRegistry,
    TaskRegistryFactory,
};

pub use approx_max_kcut::*;
pub use conductance::*;
pub use k1coloring::*;
pub use kcore::*;
pub use kmeans::*;
pub use label_propagation::*;
pub use leiden::*;
pub use louvain::*;
pub use modularity::*;
pub use modularity_optimization::*;
pub use scc::*;
pub use triangle::*;
pub use wcc::*;

pub(crate) fn progress_tracker(
    task: LeafTask,
    concurrency: usize,
    task_registry: Option<&TaskRegistry>,
) -> TaskProgressTracker {
    progress_tracker_for_task(task.base().clone(), concurrency, task_registry)
}

pub(crate) fn progress_tracker_for_task(
    base_task: Task,
    concurrency: usize,
    task_registry: Option<&TaskRegistry>,
) -> TaskProgressTracker {
    struct PrebuiltTaskRegistryFactory(TaskRegistry);

    impl TaskRegistryFactory for PrebuiltTaskRegistryFactory {
        fn new_instance(&self, _job_id: JobId) -> TaskRegistry {
            self.0.clone()
        }
    }

    let concurrency = Concurrency::of(concurrency.max(1));

    if let Some(registry) = task_registry {
        let registry_factory = PrebuiltTaskRegistryFactory(registry.clone());
        TaskProgressTracker::with_registry(base_task, concurrency, JobId::new(), &registry_factory)
    } else {
        let registry_factory = EmptyTaskRegistryFactory;
        TaskProgressTracker::with_registry(base_task, concurrency, JobId::new(), &registry_factory)
    }
}
