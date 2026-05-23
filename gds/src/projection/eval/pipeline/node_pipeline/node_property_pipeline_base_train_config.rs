use crate::types::graph_store::DefaultGraphStore;
use crate::types::graph_store::GraphStore;
use crate::types::schema::NodeLabel;
use std::collections::HashSet;
use std::sync::Arc;

/// Base configuration trait for node property pipeline training.
///
/// This trait combines training configuration with graph-specific settings
/// for node property prediction pipelines (classification and regression).
pub trait NodePropertyPipelineBaseTrainConfig {
    /// Returns the name of the pipeline to train.
    fn pipeline(&self) -> &str;

    /// Returns the target node labels for training.
    ///
    /// Default is to project all node labels.
    fn target_node_labels(&self) -> Vec<String> {
        vec!["*".to_string()]
    }

    /// Returns the relationship types used for training.
    ///
    /// Empty means all relationship types, matching Java's default predict/train config semantics.
    fn relationship_types(&self) -> Vec<String> {
        Vec::new()
    }

    /// Returns the target property to predict.
    fn target_property(&self) -> &str;

    /// Returns the random seed for reproducibility.
    fn random_seed(&self) -> Option<u64>;

    /// Resolves target node labels from the graph store.
    ///
    /// This handles the special case of "*" (all labels) and validates
    /// that the specified labels exist in the graph.
    fn target_node_label_identifiers(
        &self,
        graph_store: &Arc<DefaultGraphStore>,
    ) -> HashSet<NodeLabel> {
        self.validate_target_node_label_identifiers(graph_store)
            .unwrap_or_else(|_| {
                self.target_node_labels()
                    .iter()
                    .map(|label| NodeLabel::of(label.as_str()))
                    .collect()
            })
    }

    /// Resolves and validates target node labels from the graph store.
    fn validate_target_node_label_identifiers(
        &self,
        graph_store: &Arc<DefaultGraphStore>,
    ) -> Result<HashSet<NodeLabel>, NodePropertyPipelineConfigError> {
        let target_labels = self.target_node_labels();

        if target_labels.len() == 1 && target_labels[0] == "*" {
            // Project all node labels
            Ok(graph_store.node_labels())
        } else {
            // Convert specified labels
            let labels = target_labels
                .iter()
                .map(|label| NodeLabel::of(label.as_str()))
                .collect::<HashSet<_>>();

            let missing_labels = labels
                .iter()
                .filter(|label| !graph_store.has_node_label(label))
                .map(|label| label.name().to_string())
                .collect::<Vec<_>>();

            if missing_labels.is_empty() {
                Ok(labels)
            } else {
                Err(NodePropertyPipelineConfigError::MissingNodeLabels {
                    labels: missing_labels,
                })
            }
        }
    }

    /// Returns node labels for filtering (same as target node labels).
    fn node_labels(&self) -> Vec<String> {
        self.target_node_labels()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodePropertyPipelineConfigError {
    MissingNodeLabels { labels: Vec<String> },
}

impl std::fmt::Display for NodePropertyPipelineConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingNodeLabels { labels } => {
                write!(
                    f,
                    "Target node labels do not exist in the graph: {:?}",
                    labels
                )
            }
        }
    }
}

impl std::error::Error for NodePropertyPipelineConfigError {}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockTrainConfig {
        pipeline_name: String,
        target_labels: Vec<String>,
        target_prop: String,
        seed: Option<u64>,
    }

    impl NodePropertyPipelineBaseTrainConfig for MockTrainConfig {
        fn pipeline(&self) -> &str {
            &self.pipeline_name
        }

        fn target_node_labels(&self) -> Vec<String> {
            self.target_labels.clone()
        }

        fn target_property(&self) -> &str {
            &self.target_prop
        }

        fn random_seed(&self) -> Option<u64> {
            self.seed
        }
    }

    #[test]
    fn test_default_target_node_labels() {
        let config = MockTrainConfig {
            pipeline_name: "test-pipeline".to_string(),
            target_labels: vec!["*".to_string()],
            target_prop: "label".to_string(),
            seed: Some(42),
        };

        let labels = config.target_node_labels();
        assert_eq!(labels, vec!["*"]);
    }

    #[test]
    fn test_node_labels_returns_target_labels() {
        let config = MockTrainConfig {
            pipeline_name: "test-pipeline".to_string(),
            target_labels: vec!["Person".to_string(), "Company".to_string()],
            target_prop: "label".to_string(),
            seed: Some(42),
        };

        let labels = config.node_labels();
        assert_eq!(labels, vec!["Person", "Company"]);
    }

    #[test]
    fn test_default_relationship_types_is_empty() {
        let config = MockTrainConfig {
            pipeline_name: "test-pipeline".to_string(),
            target_labels: vec!["*".to_string()],
            target_prop: "label".to_string(),
            seed: Some(42),
        };

        assert!(config.relationship_types().is_empty());
    }

    #[test]
    fn test_pipeline_name() {
        let config = MockTrainConfig {
            pipeline_name: "my-classification-pipeline".to_string(),
            target_labels: vec!["*".to_string()],
            target_prop: "label".to_string(),
            seed: Some(42),
        };

        assert_eq!(config.pipeline(), "my-classification-pipeline");
    }

    #[test]
    fn test_target_property() {
        let config = MockTrainConfig {
            pipeline_name: "test-pipeline".to_string(),
            target_labels: vec!["*".to_string()],
            target_prop: "fraudScore".to_string(),
            seed: Some(42),
        };

        assert_eq!(config.target_property(), "fraudScore");
    }

    #[test]
    fn test_random_seed() {
        let config = MockTrainConfig {
            pipeline_name: "test-pipeline".to_string(),
            target_labels: vec!["*".to_string()],
            target_prop: "label".to_string(),
            seed: Some(12345),
        };

        assert_eq!(config.random_seed(), Some(12345));
    }

    #[test]
    fn test_validate_target_labels_with_wildcard_returns_graph_labels() {
        use crate::types::random::RandomGraphConfig;

        let graph_store = Arc::new(
            DefaultGraphStore::random(&RandomGraphConfig {
                seed: Some(42),
                node_count: 50,
                ..RandomGraphConfig::default()
            })
            .expect("random graph"),
        );
        let config = MockTrainConfig {
            pipeline_name: "test-pipeline".to_string(),
            target_labels: vec!["*".to_string()],
            target_prop: "label".to_string(),
            seed: Some(42),
        };

        let labels = config
            .validate_target_node_label_identifiers(&graph_store)
            .expect("wildcard labels should resolve");

        assert_eq!(labels, graph_store.node_labels());
    }

    #[test]
    fn test_validate_target_labels_errors_for_missing_label() {
        use crate::types::random::RandomGraphConfig;

        let graph_store = Arc::new(
            DefaultGraphStore::random(&RandomGraphConfig {
                seed: Some(42),
                node_count: 50,
                ..RandomGraphConfig::default()
            })
            .expect("random graph"),
        );
        let config = MockTrainConfig {
            pipeline_name: "test-pipeline".to_string(),
            target_labels: vec!["__MissingLabel__".to_string()],
            target_prop: "label".to_string(),
            seed: Some(42),
        };

        let err = config
            .validate_target_node_label_identifiers(&graph_store)
            .expect_err("missing labels should fail validation");

        assert_eq!(
            err,
            NodePropertyPipelineConfigError::MissingNodeLabels {
                labels: vec!["__MissingLabel__".to_string()]
            }
        );
    }

    #[test]
    fn test_no_random_seed() {
        let config = MockTrainConfig {
            pipeline_name: "test-pipeline".to_string(),
            target_labels: vec!["*".to_string()],
            target_prop: "label".to_string(),
            seed: None,
        };

        assert_eq!(config.random_seed(), None);
    }
}
