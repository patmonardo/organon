//! Operations Facade
//!
//! Provides operations for progress tracking, feature toggles, and user logs,
//! mirroring Java OperationsProcedureFacade.

use std::sync::Arc;

use crate::core::utils::feature_toggles::AdjacencyPackingStrategy;
use crate::core::utils::feature_toggles::FeatureConfig;
use crate::core::utils::feature_toggles::FeatureToggle;
pub use crate::core::utils::warnings::UserLogEntry;
use crate::core::utils::warnings::UserLogStore;
use crate::task::progress::JobId;
use crate::task::progress::TaskStore;
use crate::task::progress::UserTask;
use crate::types::user::User;

pub struct ApplicationsFacade {
    task_store: Arc<dyn TaskStore>,
    user_log_store: Arc<dyn UserLogStore>,
}

impl ApplicationsFacade {
    pub fn new(task_store: Arc<dyn TaskStore>, user_log_store: Arc<dyn UserLogStore>) -> Self {
        Self {
            task_store,
            user_log_store,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProgressResult {
    pub job_id: String,
    pub task_name: String,
    pub progress: usize,
    pub total: usize,
    pub status: String,
}

impl From<UserTask> for ProgressResult {
    fn from(user_task: UserTask) -> Self {
        let progress = user_task.task().get_progress();
        Self {
            job_id: user_task.job_id().as_string().to_string(),
            task_name: user_task.task().description().to_string(),
            progress: progress.progress(),
            total: progress.volume(),
            status: user_task.task().status().to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FeatureStringValue {
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FeatureState {
    pub enabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FeatureLongValue {
    pub value: i64,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum OperationsError {
    #[error("Unknown adjacency packing strategy: {0}")]
    UnknownAdjacencyPackingStrategy(String),
    #[error("pages per thread must be between 1 and {max}, got {value}")]
    InvalidPagesPerThread { value: i64, max: i32 },
}

/// Trait for Operations Procedure Facade
pub trait OperationsProcedureFacade {
    fn enable_adjacency_compression_memory_tracking(&self, value: bool);
    fn enable_arrow_database_import(&self, value: bool);
    fn list_progress(&self, job_id: &str) -> Vec<ProgressResult>;
    fn query_user_log(&self, job_id: &str) -> Vec<UserLogEntry>;
    fn reset_adjacency_packing_strategy(&self) -> Vec<FeatureStringValue>;
    fn reset_enable_adjacency_compression_memory_tracking(&self) -> Vec<FeatureState>;
    fn reset_enable_arrow_database_import(&self) -> Vec<FeatureState>;
    fn reset_pages_per_thread(&self) -> Vec<FeatureLongValue>;
    fn reset_use_mixed_adjacency_list(&self) -> Vec<FeatureState>;
    fn reset_use_packed_adjacency_list(&self) -> Vec<FeatureState>;
    fn reset_use_reordered_adjacency_list(&self) -> Vec<FeatureState>;
    fn reset_use_uncompressed_adjacency_list(&self) -> Vec<FeatureState>;
    fn set_adjacency_packing_strategy(&self, strategy: &str) -> Result<(), OperationsError>;
    fn set_pages_per_thread(&self, value: i64) -> Result<(), OperationsError>;
    fn set_use_mixed_adjacency_list(&self, value: bool);
    fn set_use_packed_adjacency_list(&self, value: bool);
    fn set_use_reordered_adjacency_list(&self, value: bool);
    fn set_use_uncompressed_adjacency_list(&self, value: bool);
}

/// Local implementation of OperationsProcedureFacade
pub struct LocalOperationsProcedureFacade {
    user: User,
    applications_facade: Arc<ApplicationsFacade>,
}

impl LocalOperationsProcedureFacade {
    pub fn new(user: User, applications_facade: Arc<ApplicationsFacade>) -> Self {
        Self {
            user,
            applications_facade,
        }
    }
}

fn reset_toggle(toggle: FeatureToggle) -> Vec<FeatureState> {
    toggle.reset();
    vec![FeatureState {
        enabled: toggle.is_enabled(),
    }]
}

fn adjacency_packing_strategy_name(strategy: AdjacencyPackingStrategy) -> &'static str {
    match strategy {
        AdjacencyPackingStrategy::BlockAlignedTail => "BLOCK_ALIGNED_TAIL",
        AdjacencyPackingStrategy::VarLongTail => "VAR_LONG_TAIL",
        AdjacencyPackingStrategy::PackedTail => "PACKED_TAIL",
        AdjacencyPackingStrategy::InlinedHeadPackedTail => "INLINED_HEAD_PACKED_TAIL",
    }
}

fn parse_adjacency_packing_strategy(
    strategy: &str,
) -> Result<AdjacencyPackingStrategy, OperationsError> {
    match strategy.trim().to_ascii_uppercase().as_str() {
        "BLOCK_ALIGNED_TAIL" => Ok(AdjacencyPackingStrategy::BlockAlignedTail),
        "VAR_LONG_TAIL" => Ok(AdjacencyPackingStrategy::VarLongTail),
        "PACKED_TAIL" => Ok(AdjacencyPackingStrategy::PackedTail),
        "INLINED_HEAD_PACKED_TAIL" => Ok(AdjacencyPackingStrategy::InlinedHeadPackedTail),
        _ => Err(OperationsError::UnknownAdjacencyPackingStrategy(
            strategy.to_string(),
        )),
    }
}

impl OperationsProcedureFacade for LocalOperationsProcedureFacade {
    fn enable_adjacency_compression_memory_tracking(&self, value: bool) {
        FeatureToggle::EnableAdjacencyCompressionMemoryTracking.toggle(value);
    }

    fn enable_arrow_database_import(&self, value: bool) {
        FeatureToggle::EnableArrowDatabaseImport.toggle(value);
    }

    fn list_progress(&self, job_id: &str) -> Vec<ProgressResult> {
        let mut tasks = if job_id.trim().is_empty() {
            self.applications_facade
                .task_store
                .query_by_username(self.user.username())
        } else {
            self.applications_facade
                .task_store
                .query(self.user.username(), &JobId::from(job_id))
                .into_iter()
                .collect()
        };
        tasks.sort_by(|left, right| left.job_id().as_string().cmp(right.job_id().as_string()));
        tasks.into_iter().map(ProgressResult::from).collect()
    }

    fn query_user_log(&self, job_id: &str) -> Vec<UserLogEntry> {
        let entries = self
            .applications_facade
            .user_log_store
            .query(self.user.username());
        if job_id.trim().is_empty() {
            return entries;
        }

        let Some(user_task) = self
            .applications_facade
            .task_store
            .query(self.user.username(), &JobId::from(job_id))
        else {
            return Vec::new();
        };
        entries
            .into_iter()
            .filter(|entry| entry.task_name() == user_task.task().description())
            .collect()
    }

    fn reset_adjacency_packing_strategy(&self) -> Vec<FeatureStringValue> {
        let strategy = AdjacencyPackingStrategy::default();
        FeatureConfig::set_adjacency_packing_strategy(strategy);
        vec![FeatureStringValue {
            value: adjacency_packing_strategy_name(strategy).to_string(),
        }]
    }

    fn reset_enable_adjacency_compression_memory_tracking(&self) -> Vec<FeatureState> {
        reset_toggle(FeatureToggle::EnableAdjacencyCompressionMemoryTracking)
    }

    fn reset_enable_arrow_database_import(&self) -> Vec<FeatureState> {
        reset_toggle(FeatureToggle::EnableArrowDatabaseImport)
    }

    fn reset_pages_per_thread(&self) -> Vec<FeatureLongValue> {
        let value = FeatureConfig::PAGES_PER_THREAD_DEFAULT as i32;
        FeatureConfig::set_pages_per_thread(value);
        vec![FeatureLongValue {
            value: i64::from(value),
        }]
    }

    fn reset_use_mixed_adjacency_list(&self) -> Vec<FeatureState> {
        reset_toggle(FeatureToggle::UseMixedAdjacencyList)
    }

    fn reset_use_packed_adjacency_list(&self) -> Vec<FeatureState> {
        reset_toggle(FeatureToggle::UsePackedAdjacencyList)
    }

    fn reset_use_reordered_adjacency_list(&self) -> Vec<FeatureState> {
        reset_toggle(FeatureToggle::UseReorderedAdjacencyList)
    }

    fn reset_use_uncompressed_adjacency_list(&self) -> Vec<FeatureState> {
        reset_toggle(FeatureToggle::UseUncompressedAdjacencyList)
    }

    fn set_adjacency_packing_strategy(&self, strategy: &str) -> Result<(), OperationsError> {
        FeatureConfig::set_adjacency_packing_strategy(parse_adjacency_packing_strategy(strategy)?);
        Ok(())
    }

    fn set_pages_per_thread(&self, value: i64) -> Result<(), OperationsError> {
        let value = i32::try_from(value).map_err(|_| OperationsError::InvalidPagesPerThread {
            value,
            max: i32::MAX,
        })?;
        if value < 1 {
            return Err(OperationsError::InvalidPagesPerThread {
                value: i64::from(value),
                max: i32::MAX,
            });
        }
        FeatureConfig::set_pages_per_thread(value);
        Ok(())
    }

    fn set_use_mixed_adjacency_list(&self, value: bool) {
        FeatureToggle::UseMixedAdjacencyList.toggle(value);
    }

    fn set_use_packed_adjacency_list(&self, value: bool) {
        FeatureToggle::UsePackedAdjacencyList.toggle(value);
    }

    fn set_use_reordered_adjacency_list(&self, value: bool) {
        FeatureToggle::UseReorderedAdjacencyList.toggle(value);
    }

    fn set_use_uncompressed_adjacency_list(&self, value: bool) {
        FeatureToggle::UseUncompressedAdjacencyList.toggle(value);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;
    use crate::core::utils::warnings::PerDatabaseUserLogStore;
    use crate::task::progress::PerDatabaseTaskStore;
    use crate::task::progress::Task;

    static TEST_LOCK: Mutex<()> = Mutex::new(());

    fn facade() -> LocalOperationsProcedureFacade {
        LocalOperationsProcedureFacade::new(
            User::from("alice"),
            Arc::new(ApplicationsFacade::new(
                Arc::new(PerDatabaseTaskStore::new()),
                Arc::new(PerDatabaseUserLogStore::new()),
            )),
        )
    }

    #[test]
    fn feature_toggle_setters_and_resets_update_shared_state() {
        let _guard = TEST_LOCK.lock().expect("operations test lock");
        let facade = facade();

        facade.enable_arrow_database_import(false);
        assert!(!FeatureToggle::EnableArrowDatabaseImport.is_enabled());
        assert_eq!(
            facade.reset_enable_arrow_database_import(),
            vec![FeatureState { enabled: true }]
        );
        assert!(FeatureToggle::EnableArrowDatabaseImport.is_enabled());

        facade.set_use_packed_adjacency_list(true);
        assert!(FeatureToggle::UsePackedAdjacencyList.is_enabled());
        assert_eq!(
            facade.reset_use_packed_adjacency_list(),
            vec![FeatureState { enabled: false }]
        );
    }

    #[test]
    fn configuration_setters_validate_and_reset_values() {
        let _guard = TEST_LOCK.lock().expect("operations test lock");
        let facade = facade();

        facade
            .set_adjacency_packing_strategy("var_long_tail")
            .expect("valid strategy");
        assert_eq!(
            FeatureConfig::adjacency_packing_strategy(),
            AdjacencyPackingStrategy::VarLongTail
        );
        assert_eq!(
            facade.reset_adjacency_packing_strategy(),
            vec![FeatureStringValue {
                value: "INLINED_HEAD_PACKED_TAIL".to_string(),
            }]
        );

        facade.set_pages_per_thread(8).expect("valid page count");
        assert_eq!(FeatureConfig::pages_per_thread(), 8);
        assert!(matches!(
            facade.set_pages_per_thread(0),
            Err(OperationsError::InvalidPagesPerThread { value: 0, .. })
        ));
        assert_eq!(
            facade.reset_pages_per_thread(),
            vec![FeatureLongValue { value: 4 }]
        );
    }

    #[test]
    fn progress_queries_are_scoped_by_user_and_job() {
        let task_store = Arc::new(PerDatabaseTaskStore::new());
        let user_log_store = Arc::new(PerDatabaseUserLogStore::new());
        let applications = Arc::new(ApplicationsFacade::new(task_store.clone(), user_log_store));
        let facade = LocalOperationsProcedureFacade::new(User::from("alice"), applications);
        let alice_job = JobId::from("alice-job");
        let bob_job = JobId::from("bob-job");
        task_store.store(
            "alice".to_string(),
            alice_job.clone(),
            Task::leaf("alice-task".to_string(), 10),
        );
        task_store.store(
            "bob".to_string(),
            bob_job,
            Task::leaf("bob-task".to_string(), 20),
        );

        let all = facade.list_progress("");
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].job_id, "alice-job");
        assert_eq!(all[0].task_name, "alice-task");
        assert_eq!(facade.list_progress("alice-job"), all);
        assert!(facade.list_progress("bob-job").is_empty());
    }

    #[test]
    fn user_log_queries_are_scoped_through_the_requested_job() {
        let task_store = Arc::new(PerDatabaseTaskStore::new());
        let user_log_store = Arc::new(PerDatabaseUserLogStore::new());
        let applications = Arc::new(ApplicationsFacade::new(
            task_store.clone(),
            user_log_store.clone(),
        ));
        let facade = LocalOperationsProcedureFacade::new(User::from("alice"), applications);
        let task = Task::new("import".to_string(), vec![]);
        task_store.store("alice".to_string(), JobId::from("import-job"), task.clone());
        user_log_store.add_user_log_message("alice", &task, "warning".to_string());
        user_log_store.add_user_log_message("bob", &task, "private".to_string());

        let entries = facade.query_user_log("import-job");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].message(), "warning");
        assert!(facade.query_user_log("missing-job").is_empty());
    }
}
