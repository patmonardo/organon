use std::sync::Arc;

use crate::core::model::ModelCatalog;
use crate::core::utils::warnings::UserLogStore;
use crate::task::memory::MemoryTracker;
use crate::task::progress::TaskStore;
use crate::types::catalog::CatalogError;
use crate::types::catalog::GraphCatalog;
use crate::types::graph_store::DatabaseId;
use crate::types::user::User;

use crate::procedures::graph::GraphFacade;
use crate::procedures::graph_catalog::LocalGraphProcedureFacade;
use crate::procedures::graph_catalog::RequestScopedDependencies as GraphCatalogDependencies;
use crate::procedures::memory::MemoryFacade;
use crate::procedures::model_catalog::shared_in_memory_model_catalog;
use crate::procedures::model_catalog::LocalModelCatalogProcedureFacade;
use crate::procedures::model_catalog::ModelCatalogFacade;
use crate::procedures::operations::ApplicationsFacade;
use crate::procedures::operations::LocalOperationsProcedureFacade;

use super::ShellProcedureRuntime;

/// Shell-owned control dependencies spanning procedure control networks.
///
/// This struct intentionally spans beyond algorithm invocation so the Shell can
/// orchestrate graph, catalog, memory, operation toggles, and model lifecycle
/// from one control locus.
#[derive(Clone)]
pub struct ShellProcedureControlDependencies {
    pub user: User,
    pub database_id: DatabaseId,
    pub graph_catalog: Arc<dyn GraphCatalog>,
    pub task_store: Arc<dyn TaskStore>,
    pub user_log_store: Arc<dyn UserLogStore>,
    pub memory_tracker: Arc<MemoryTracker>,
    pub model_catalog: Arc<ModelCatalogFacade>,
}

impl ShellProcedureControlDependencies {
    pub fn new(
        user: User,
        database_id: DatabaseId,
        graph_catalog: Arc<dyn GraphCatalog>,
        task_store: Arc<dyn TaskStore>,
        user_log_store: Arc<dyn UserLogStore>,
        memory_tracker: Arc<MemoryTracker>,
    ) -> Self {
        Self {
            user,
            database_id,
            graph_catalog,
            task_store,
            user_log_store,
            memory_tracker,
            model_catalog: shared_in_memory_model_catalog(),
        }
    }

    pub fn with_model_catalog(mut self, model_catalog: Arc<ModelCatalogFacade>) -> Self {
        self.model_catalog = model_catalog;
        self
    }
}

/// Shell control facade spanning all local procedure-control surfaces.
pub struct ShellProcedureControl {
    deps: ShellProcedureControlDependencies,
}

impl ShellProcedureControl {
    pub fn new(deps: ShellProcedureControlDependencies) -> Self {
        Self { deps }
    }

    pub fn dependencies(&self) -> &ShellProcedureControlDependencies {
        &self.deps
    }

    pub fn graph_catalog(&self) -> LocalGraphProcedureFacade {
        LocalGraphProcedureFacade::new(GraphCatalogDependencies::new(
            self.deps.user.clone(),
            self.deps.database_id.clone(),
            Arc::clone(&self.deps.graph_catalog),
        ))
    }

    pub fn operations(&self) -> LocalOperationsProcedureFacade {
        let applications = Arc::new(ApplicationsFacade::new(
            Arc::clone(&self.deps.task_store),
            Arc::clone(&self.deps.user_log_store),
        ));
        LocalOperationsProcedureFacade::new(self.deps.user.clone(), applications)
    }

    pub fn memory(&self) -> MemoryFacade {
        MemoryFacade::new(
            self.deps.user.clone(),
            Arc::clone(&self.deps.memory_tracker),
        )
    }

    pub fn model_catalog(&self) -> LocalModelCatalogProcedureFacade {
        LocalModelCatalogProcedureFacade::new(
            self.deps.user.clone(),
            Arc::clone(&self.deps.model_catalog),
        )
    }

    pub fn graph(&self, graph_name: &str) -> Result<GraphFacade, CatalogError> {
        let store = self
            .deps
            .graph_catalog
            .get(graph_name)
            .ok_or_else(|| CatalogError::NotFound(graph_name.to_string()))?;
        Ok(GraphFacade::new(store))
    }

    pub fn runtime_with_graph(
        &self,
        graph_name: &str,
        pipelines: crate::procedures::pipelines::LocalPipelinesProcedureFacade,
    ) -> Result<ShellProcedureRuntime, CatalogError> {
        let graph = self.graph(graph_name)?;
        Ok(ShellProcedureRuntime::new(graph, pipelines))
    }

    pub fn model_count(&self) -> usize {
        ModelCatalog::model_count(self.deps.model_catalog.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GraphStoreConfig;
    use crate::core::utils::warnings::PerDatabaseUserLogStore;
    use crate::procedures::graph_catalog::GraphProcedureFacade;
    use crate::task::progress::PerDatabaseTaskStore;
    use crate::types::catalog::InMemoryGraphCatalog;
    use crate::types::graph::SimpleIdMap;
    use crate::types::graph_store::Capabilities;
    use crate::types::graph_store::DatabaseInfo;
    use crate::types::graph_store::DatabaseLocation;
    use crate::types::graph_store::GraphName;
    use crate::types::prelude::DefaultGraphStore;
    use crate::types::schema::GraphSchema;
    use std::collections::HashMap;

    fn build_store(name: &str) -> DefaultGraphStore {
        DefaultGraphStore::new(
            GraphStoreConfig::default(),
            GraphName::new(name),
            DatabaseInfo::new(
                DatabaseId::new("neo4j"),
                DatabaseLocation::remote("localhost", 7687, None, None),
            ),
            GraphSchema::empty(),
            Capabilities::default(),
            SimpleIdMap::from_original_ids([0, 1]),
            HashMap::new(),
        )
    }

    fn facade() -> ShellProcedureControl {
        let graph_catalog: Arc<dyn GraphCatalog> = Arc::new(InMemoryGraphCatalog::new());
        graph_catalog.set("g", Arc::new(build_store("g")));

        let deps = ShellProcedureControlDependencies::new(
            User::from("alice"),
            DatabaseId::new("neo4j"),
            graph_catalog,
            Arc::new(PerDatabaseTaskStore::new()),
            Arc::new(PerDatabaseUserLogStore::new()),
            Arc::new(MemoryTracker::new(u64::MAX)),
        );

        ShellProcedureControl::new(deps)
    }

    #[test]
    fn resolves_graph_and_control_surfaces() {
        let facade = facade();

        assert!(facade.graph("g").is_ok());
        assert!(facade.graph_catalog().graph_exists("g"));
        assert!(facade.memory().list().is_empty());
        assert_eq!(facade.model_count(), 0);

        // Control surface should be constructible for user-scoped operations.
        let _operations = facade.operations();
    }

    #[test]
    fn missing_graph_is_reported_with_not_found() {
        let facade = facade();

        assert!(matches!(
            facade.graph("missing"),
            Err(CatalogError::NotFound(name)) if name == "missing"
        ));
    }
}
