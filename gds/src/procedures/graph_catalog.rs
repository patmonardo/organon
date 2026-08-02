//! Graph Procedure Facade
//!
//! Provides execution-oriented Graph Procedures and Views, mirroring Java GraphCatalogProcedureFacade.
//! Handles graph catalog operations like listing, dropping, projecting, streaming, and writing.
//!
//! This facade focuses on procedure-specific behaviors and delegates to business logic.

use std::sync::Arc;

use crate::types::catalog::CatalogError;
use crate::types::catalog::GraphCatalog;
use crate::types::graph_store::DatabaseId;
use crate::types::user::User;

/// Trait for Graph Procedure Facade operations
pub trait GraphProcedureFacade {
    /// Check if a graph exists
    fn graph_exists(&self, graph_name: &str) -> bool;

    /// List graphs
    fn list_graphs(&self) -> Vec<String>;

    /// Drop a graph
    fn drop_graph(&self, graph_name: &str) -> Result<(), CatalogError>;
}

/// Simple request scoped dependencies for now
pub struct RequestScopedDependencies {
    pub user: User,
    pub database_id: DatabaseId,
    pub graph_catalog: Arc<dyn GraphCatalog>,
}

impl RequestScopedDependencies {
    pub fn new(user: User, database_id: DatabaseId, graph_catalog: Arc<dyn GraphCatalog>) -> Self {
        Self {
            user,
            database_id,
            graph_catalog,
        }
    }
}

/// Local implementation of GraphProcedureFacade
pub struct LocalGraphProcedureFacade {
    request_scoped_dependencies: RequestScopedDependencies,
}

impl LocalGraphProcedureFacade {
    pub fn new(request_scoped_dependencies: RequestScopedDependencies) -> Self {
        Self {
            request_scoped_dependencies,
        }
    }
}

impl GraphProcedureFacade for LocalGraphProcedureFacade {
    fn graph_exists(&self, graph_name: &str) -> bool {
        self.request_scoped_dependencies
            .graph_catalog
            .get(graph_name)
            .is_some()
    }

    fn list_graphs(&self) -> Vec<String> {
        let mut graph_names = self
            .request_scoped_dependencies
            .graph_catalog
            .list(None, false)
            .into_iter()
            .map(|entry| entry.name)
            .collect::<Vec<_>>();
        graph_names.sort_unstable();
        graph_names
    }

    fn drop_graph(&self, graph_name: &str) -> Result<(), CatalogError> {
        GraphCatalog::drop(
            self.request_scoped_dependencies.graph_catalog.as_ref(),
            &[graph_name],
            true,
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::catalog::InMemoryGraphCatalog;

    fn empty_facade() -> LocalGraphProcedureFacade {
        LocalGraphProcedureFacade::new(RequestScopedDependencies::new(
            User::from("alice"),
            DatabaseId::new("neo4j"),
            Arc::new(InMemoryGraphCatalog::new()),
        ))
    }

    #[test]
    fn empty_catalog_reports_no_graphs() {
        let facade = empty_facade();

        assert!(!facade.graph_exists("missing"));
        assert!(facade.list_graphs().is_empty());
    }

    #[test]
    fn dropping_a_missing_graph_returns_catalog_error() {
        let facade = empty_facade();

        assert!(matches!(
            facade.drop_graph("missing"),
            Err(CatalogError::NotFound(name)) if name == "missing"
        ));
    }
}
