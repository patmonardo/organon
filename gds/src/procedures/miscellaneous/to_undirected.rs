//! ToUndirected procedure facade.
//!
//! Procedure facade that delegates to the undirected algorithm runtimes.

use crate::algo::algorithms::Result;
use crate::algo::undirected::{
    ToUndirectedAggregations, ToUndirectedComputationRuntime, ToUndirectedConfig,
    ToUndirectedStats, ToUndirectedStorageRuntime,
};
use crate::task::concurrency::TerminationFlag;
use crate::task::progress::{TaskProgressTracker, Tasks};
use crate::core::Aggregation;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::prelude::DefaultGraphStore;
use std::sync::Arc;

pub struct ToUndirectedFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: ToUndirectedConfig,
}

impl ToUndirectedFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: ToUndirectedConfig::default(),
        }
    }

    pub fn relationship_type(mut self, rel_type: impl Into<String>) -> Self {
        self.config.relationship_type = rel_type.into();
        self
    }

    pub fn mutate_relationship_type(mut self, rel_type: impl Into<String>) -> Self {
        self.config.mutate_relationship_type = rel_type.into();
        self
    }

    pub fn mutate_graph_name(mut self, graph_name: impl Into<String>) -> Self {
        self.config.mutate_graph_name = graph_name.into();
        self
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.config.concurrency = concurrency;
        self
    }

    pub fn aggregation(mut self, aggregation: Aggregation) -> Self {
        let aggregations = self
            .config
            .aggregation
            .take()
            .unwrap_or_else(ToUndirectedAggregations::new)
            .with_global(aggregation);
        self.config.aggregation = Some(aggregations);
        self
    }

    pub fn property_aggregation(
        mut self,
        property_key: impl Into<String>,
        aggregation: Aggregation,
    ) -> Self {
        let aggregations = self
            .config
            .aggregation
            .take()
            .unwrap_or_else(ToUndirectedAggregations::new)
            .with_property(property_key, aggregation);
        self.config.aggregation = Some(aggregations);
        self
    }

    fn validate(&self, config: &ToUndirectedConfig) -> Result<()> {
        if config.relationship_type.is_empty() {
            return Err(AlgorithmError::Execution(
                "relationshipType must be provided".to_string(),
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
        config: &ToUndirectedConfig,
    ) -> Result<crate::algo::undirected::ToUndirectedResult> {
        let mut computation = ToUndirectedComputationRuntime::new();
        let storage = ToUndirectedStorageRuntime::new(config.concurrency);
        let termination = TerminationFlag::running_true();
        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("to_undirected".to_string(), 0),
            config.concurrency,
        );

        self.validate(config)?;
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

    /// Produce a new graph store with an undirected relationship type added.
    pub fn to_store(&self, graph_name: &str) -> Result<DefaultGraphStore> {
        let mut config = self.config.clone();
        config.mutate_graph_name = graph_name.to_string();
        self.compute(&config).map(|r| r.graph_store)
    }

    pub fn stats(&self) -> Result<ToUndirectedStats> {
        let result = self.compute(&self.config)?;
        Ok(ToUndirectedStats {
            graph_name: result.graph_name,
            mutate_relationship_type: result.mutate_relationship_type,
            node_count: result.node_count,
            relationship_count: result.relationship_count,
        })
    }

    pub fn graph_name(&self) -> String {
        self.config.mutate_graph_name.clone()
    }
}
