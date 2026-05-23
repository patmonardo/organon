//! Java GDS: pipeline/src/main/java/org/neo4j/gds/ml/pipeline/PipelineGraphFilter.java
//!
//! Filter struct for dataset splits (node labels + relationship types).
//!
//! This is a simple value class that specifies which node labels and relationship types
//! should be included when filtering a graph for a specific dataset split (TRAIN, TEST, etc.).

/// Filter specification for dataset splits in pipeline execution.
///
/// Specifies which node labels and relationship types should be included
/// when creating filtered graph views for training, testing, or feature input.
///
/// # Java Source
/// ```java
/// @ValueClass
/// public interface PipelineGraphFilter {
///     Collection<NodeLabel> nodeLabels();
///
///     @Value.Default
///     default Collection<RelationshipType> relationshipTypes() {
///         return List.of();
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PipelineGraphFilter {
    /// Node labels to include in the filtered graph
    pub node_labels: Vec<String>,

    /// Relationship types to include in the filtered graph (defaults to empty)
    pub relationship_types: Vec<String>,
}

impl PipelineGraphFilter {
    /// Create a new pipeline graph filter.
    ///
    /// # Arguments
    /// * `node_labels` - Node labels to include
    /// * `relationship_types` - Relationship types to include (defaults to empty if None)
    pub fn new(node_labels: Vec<String>, relationship_types: Option<Vec<String>>) -> Self {
        Self {
            node_labels,
            relationship_types: relationship_types.unwrap_or_default(),
        }
    }

    /// Borrow the node labels included in the filtered graph.
    pub fn node_labels(&self) -> &[String] {
        &self.node_labels
    }

    /// Borrow the relationship types included in the filtered graph.
    pub fn relationship_types(&self) -> &[String] {
        &self.relationship_types
    }

    /// Create a filter with only node labels (no relationship type filtering).
    pub fn with_node_labels(node_labels: Vec<String>) -> Self {
        Self {
            node_labels,
            relationship_types: Vec::new(),
        }
    }

    /// Create a filter with node labels and relationship types.
    pub fn with_relationship_types(
        node_labels: Vec<String>,
        relationship_types: Vec<String>,
    ) -> Self {
        Self {
            node_labels,
            relationship_types,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_with_relationships() {
        let filter =
            PipelineGraphFilter::new(vec!["Person".to_string()], Some(vec!["KNOWS".to_string()]));

        assert_eq!(filter.node_labels, vec!["Person"]);
        assert_eq!(filter.relationship_types, vec!["KNOWS"]);
    }

    #[test]
    fn test_new_defaults_relationships() {
        let filter = PipelineGraphFilter::new(vec!["Person".to_string()], None);

        assert_eq!(filter.node_labels, vec!["Person"]);
        assert!(filter.relationship_types.is_empty());
    }

    #[test]
    fn test_with_node_labels() {
        let filter = PipelineGraphFilter::with_node_labels(vec!["Person".to_string()]);

        assert_eq!(filter.node_labels, vec!["Person"]);
        assert!(filter.relationship_types.is_empty());
    }

    #[test]
    fn test_accessors() {
        let filter = PipelineGraphFilter::with_relationship_types(
            vec!["Person".to_string()],
            vec!["KNOWS".to_string()],
        );

        assert_eq!(filter.node_labels(), ["Person"]);
        assert_eq!(filter.relationship_types(), ["KNOWS"]);
    }

    #[test]
    fn test_with_relationship_types() {
        let filter = PipelineGraphFilter::with_relationship_types(
            vec!["Person".to_string()],
            vec!["KNOWS".to_string()],
        );

        assert_eq!(filter.node_labels, vec!["Person"]);
        assert_eq!(filter.relationship_types, vec!["KNOWS"]);
    }
}
