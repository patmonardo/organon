//! HashGNN storage runtime.
//!
//! This is the **Gross pole**: obtaining graph views and validating the presence
//! of node properties used for feature extraction.

use crate::projection::RelationshipType;
use crate::types::graph::Graph;
use crate::types::graph_store::{GraphStore, GraphViewSpec};
use std::collections::HashSet;
use std::sync::Arc;

/// HashGNN storage runtime.
#[derive(Debug, Default, Clone)]
pub struct HashGNNStorageRuntime;

impl HashGNNStorageRuntime {
    pub fn new() -> Self {
        Self
    }

    /// Validate that all requested node properties exist.
    pub fn validate_feature_properties(
        &self,
        graph: &dyn Graph,
        feature_properties: &[String],
    ) -> Result<(), String> {
        for key in feature_properties {
            if graph.node_properties(key).is_none() {
                return Err(format!(
                    "Missing node property `{key}`. Consider using a default value in the property projection."
                ));
            }
        }
        Ok(())
    }

    pub fn relationship_graphs<G: GraphStore>(
        &self,
        graph_store: &G,
        graph: &Arc<dyn Graph>,
        heterogeneous: bool,
    ) -> Result<Vec<Arc<dyn Graph>>, String> {
        if !heterogeneous {
            return Ok(vec![Graph::concurrent_view(graph.as_ref())]);
        }

        let mut relationship_types = graph_store
            .relationship_types()
            .into_iter()
            .collect::<Vec<_>>();
        relationship_types.sort_by(|left, right| left.name().cmp(right.name()));

        relationship_types
            .into_iter()
            .map(|relationship_type| {
                let selected_types =
                    HashSet::from([RelationshipType::of(relationship_type.name())]);
                graph_store
                    .get_graph_view(&GraphViewSpec::new().with_relationship_types(selected_types))
                    .map_err(|error| {
                        format!(
                            "Failed to construct HashGNN view for relationship type '{}': {error}",
                            relationship_type.name()
                        )
                    })
            })
            .collect()
    }
}
