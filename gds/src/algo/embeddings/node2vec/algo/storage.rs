//! Node2Vec storage runtime.
//!
//! This is the **Gross pole**: concerns around graph/property validation and view selection.

use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::graph::Graph;

#[derive(Debug, Default, Clone)]
pub struct Node2VecStorageRuntime;

impl Node2VecStorageRuntime {
    pub fn new() -> Self {
        Self
    }

    /// Validates relationship weights are non-negative when weights are present.
    pub fn validate_non_negative_weights(&self, graph: &dyn Graph) -> Result<(), AlgorithmError> {
        if !graph.has_relationship_property() {
            return Ok(());
        }

        let fallback = graph.default_property_value();
        for node in 0..graph.node_count() {
            for cursor in graph.stream_relationships_weighted(node as i64, fallback) {
                if !cursor.weight().is_finite() || cursor.weight() < 0.0 {
                    return Err(AlgorithmError::Execution(
                        "Node2Vec only supports finite non-negative relationship weights".into(),
                    ));
                }
            }
        }

        Ok(())
    }

    pub fn validate_source_nodes(
        &self,
        graph: &dyn Graph,
        source_nodes: &[i64],
    ) -> Result<(), AlgorithmError> {
        let node_count = graph.node_count() as i64;
        if let Some(source) = source_nodes
            .iter()
            .copied()
            .find(|source| *source < 0 || *source >= node_count)
        {
            return Err(AlgorithmError::Execution(format!(
                "Node2Vec source node {source} is outside mapped node range 0..{node_count}"
            )));
        }
        Ok(())
    }
}
