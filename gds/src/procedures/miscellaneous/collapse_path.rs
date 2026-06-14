//! CollapsePath procedure facade.
//!
//! CollapsePath procedure facade.
//!
//! Validates configuration, builds storage/computation runtimes, and returns a
//! new graph store with collapsed edges added as a relationship type.

use crate::algo::algorithms::Result;
use crate::algo::walking::{
    CollapsePathComputationRuntime, CollapsePathConfig, CollapsePathStats,
    CollapsePathStorageRuntime,
};
use crate::task::concurrency::TerminationFlag;
use crate::core::utils::progress::{TaskProgressTracker, Tasks};
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::prelude::DefaultGraphStore;
use std::sync::Arc;

pub struct CollapsePathFacade {
    graph_store: Arc<DefaultGraphStore>,
    path_templates: Vec<Vec<String>>,
    mutate_relationship_type: String,
    mutate_graph_name: String,
    allow_self_loops: bool,
    concurrency: usize,
}

impl CollapsePathFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            path_templates: Vec::new(),
            mutate_relationship_type: "collapsed".to_string(),
            mutate_graph_name: "collapse_path".to_string(),
            allow_self_loops: false,
            concurrency: 4,
        }
    }

    pub fn path_templates<I, P>(mut self, templates: I) -> Self
    where
        I: IntoIterator<Item = P>,
        P: IntoIterator,
        P::Item: Into<String>,
    {
        self.path_templates = templates
            .into_iter()
            .map(|path| path.into_iter().map(|s| s.into()).collect())
            .collect();
        self
    }

    pub fn mutate_relationship_type(mut self, rel_type: impl Into<String>) -> Self {
        self.mutate_relationship_type = rel_type.into();
        self
    }

    pub fn mutate_graph_name(mut self, graph_name: impl Into<String>) -> Self {
        self.mutate_graph_name = graph_name.into();
        self
    }

    pub fn allow_self_loops(mut self, allow: bool) -> Self {
        self.allow_self_loops = allow;
        self
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    fn config_for_graph(&self, graph_name: impl Into<String>) -> CollapsePathConfig {
        CollapsePathConfig {
            path_templates: self.path_templates.clone(),
            mutate_relationship_type: self.mutate_relationship_type.clone(),
            mutate_graph_name: graph_name.into(),
            allow_self_loops: self.allow_self_loops,
            concurrency: self.concurrency,
        }
    }

    fn validate(&self, config: &CollapsePathConfig) -> Result<()> {
        if config.path_templates.is_empty() {
            return Err(AlgorithmError::Execution(
                "pathTemplates must be provided".to_string(),
            ));
        }
        if config.mutate_relationship_type.is_empty() {
            return Err(AlgorithmError::Execution(
                "mutateRelationshipType must be provided".to_string(),
            ));
        }
        if config.mutate_graph_name.is_empty() {
            return Err(AlgorithmError::Execution(
                "mutateGraphName must be provided".to_string(),
            ));
        }
        if config.concurrency == 0 {
            return Err(AlgorithmError::Execution(
                "concurrency must be greater than zero".to_string(),
            ));
        }
        Ok(())
    }

    fn compute(
        &self,
        config: &CollapsePathConfig,
    ) -> Result<crate::algo::walking::CollapsePathResult> {
        self.validate(config)?;

        let mut computation = CollapsePathComputationRuntime::new(config.allow_self_loops);
        let storage = CollapsePathStorageRuntime::new(config.concurrency);
        let termination = TerminationFlag::running_true();
        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("collapse_path".to_string(), 0),
            config.concurrency,
        );

        storage
            .compute_with_controls(
                &self.graph_store,
                config,
                &mut computation,
                &termination,
                &mut progress_tracker,
            )
            .map_err(AlgorithmError::Execution)
    }

    /// Produce a new graph store with collapsed paths.
    pub fn to_store(&self, graph_name: &str) -> Result<DefaultGraphStore> {
        let config = self.config_for_graph(graph_name);
        self.compute(&config).map(|r| r.graph_store)
    }

    pub fn stats(&self, graph_name: &str) -> Result<CollapsePathStats> {
        let config = self.config_for_graph(graph_name);
        let result = self.compute(&config)?;
        Ok(CollapsePathStats {
            graph_name: graph_name.to_string(),
            mutate_relationship_type: self.mutate_relationship_type.clone(),
            node_count: result.node_count,
            relationship_count: result.relationship_count,
        })
    }

    pub fn graph_name(&self) -> String {
        self.mutate_graph_name.clone()
    }
}
