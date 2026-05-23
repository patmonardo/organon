use serde_json::Value;

use crate::procedures::pipelines::types::AnyMap;
use crate::projection::eval::pipeline::node_pipeline::NodePropertyPipelineBaseTrainConfig;

pub trait NodeRegressionPredictPipelineConfig {
    fn graph_name(&self) -> &str;
    fn concurrency(&self) -> usize;
    fn model_name(&self) -> &str;
    fn model_user(&self) -> &str;
    fn username(&self) -> String;
    fn target_node_labels(&self) -> &[String];
    fn relationship_types(&self) -> &[String];

    fn node_labels(&self) -> &[String] {
        self.target_node_labels()
    }
}

#[derive(Debug, Clone)]
pub struct NodeRegressionPredictPipelineBaseConfig {
    graph_name: String,
    concurrency: usize,
    model_name: String,
    model_user: String,
    username_override: Option<String>,
    target_node_labels: Vec<String>,
    relationship_types: Vec<String>,
}

impl NodeRegressionPredictPipelineBaseConfig {
    pub fn new(
        graph_name: String,
        concurrency: usize,
        model_name: String,
        model_user: String,
        username_override: Option<String>,
        target_node_labels: Vec<String>,
        relationship_types: Vec<String>,
    ) -> Self {
        Self {
            graph_name,
            concurrency,
            model_name,
            model_user,
            username_override,
            target_node_labels,
            relationship_types,
        }
    }

    pub fn from_map(username: String, mut config: AnyMap) -> Self {
        let graph_name = config
            .remove("graphName")
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| "graph".to_string());

        let concurrency = config
            .remove("concurrency")
            .and_then(|v| v.as_u64())
            .map(|v| v as usize)
            .unwrap_or_else(num_cpus::get);

        let model_name = config
            .remove("modelName")
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| "default_model".to_string());

        let model_user = config
            .remove("modelUser")
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| username.clone());

        let target_node_labels = config
            .remove("targetNodeLabels")
            .and_then(|v| v.as_array().cloned())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let relationship_types = config
            .remove("relationshipTypes")
            .and_then(|v| v.as_array().cloned())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        Self::new(
            graph_name,
            concurrency,
            model_name,
            model_user,
            None,
            target_node_labels,
            relationship_types,
        )
    }

    pub fn with_train_config_defaults(
        mut self,
        train_config: &dyn NodePropertyPipelineBaseTrainConfig,
    ) -> Self {
        if self.target_node_labels.is_empty() {
            self.target_node_labels = train_config.target_node_labels();
        }
        if self.relationship_types.is_empty() {
            self.relationship_types = train_config.relationship_types();
        }
        self
    }

    pub fn to_map(&self) -> AnyMap {
        let mut map = AnyMap::new();
        map.insert(
            "graphName".to_string(),
            Value::String(self.graph_name.clone()),
        );
        map.insert(
            "concurrency".to_string(),
            Value::Number(serde_json::Number::from(self.concurrency as i64)),
        );
        map.insert(
            "modelName".to_string(),
            Value::String(self.model_name.clone()),
        );
        map.insert(
            "modelUser".to_string(),
            Value::String(self.model_user.clone()),
        );
        map.insert(
            "targetNodeLabels".to_string(),
            Value::Array(
                self.target_node_labels
                    .iter()
                    .map(|v| Value::String(v.clone()))
                    .collect(),
            ),
        );
        map.insert(
            "relationshipTypes".to_string(),
            Value::Array(
                self.relationship_types
                    .iter()
                    .map(|v| Value::String(v.clone()))
                    .collect(),
            ),
        );
        map
    }
}

impl NodeRegressionPredictPipelineConfig for NodeRegressionPredictPipelineBaseConfig {
    fn graph_name(&self) -> &str {
        &self.graph_name
    }

    fn concurrency(&self) -> usize {
        self.concurrency
    }

    fn model_name(&self) -> &str {
        &self.model_name
    }

    fn model_user(&self) -> &str {
        &self.model_user
    }

    fn username(&self) -> String {
        self.username_override
            .clone()
            .unwrap_or_else(|| self.model_user.clone())
    }

    fn target_node_labels(&self) -> &[String] {
        &self.target_node_labels
    }

    fn relationship_types(&self) -> &[String] {
        &self.relationship_types
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestTrainConfig;

    impl NodePropertyPipelineBaseTrainConfig for TestTrainConfig {
        fn pipeline(&self) -> &str {
            "pipeline"
        }

        fn target_node_labels(&self) -> Vec<String> {
            vec!["Person".to_string()]
        }

        fn relationship_types(&self) -> Vec<String> {
            vec!["KNOWS".to_string()]
        }

        fn target_property(&self) -> &str {
            "target"
        }

        fn random_seed(&self) -> Option<u64> {
            Some(42)
        }
    }

    #[test]
    fn test_from_map_leaves_absent_filters_empty() {
        let config =
            NodeRegressionPredictPipelineBaseConfig::from_map("alice".to_string(), AnyMap::new());

        assert!(config.target_node_labels().is_empty());
        assert!(config.relationship_types().is_empty());
    }

    #[test]
    fn test_train_config_defaults_fill_absent_filters() {
        let config =
            NodeRegressionPredictPipelineBaseConfig::from_map("alice".to_string(), AnyMap::new())
                .with_train_config_defaults(&TestTrainConfig);

        assert_eq!(config.target_node_labels(), &["Person".to_string()]);
        assert_eq!(config.relationship_types(), &["KNOWS".to_string()]);
    }

    #[test]
    fn test_train_config_defaults_preserve_user_filters() {
        let mut map = AnyMap::new();
        map.insert(
            "targetNodeLabels".to_string(),
            Value::Array(vec![Value::String("Movie".to_string())]),
        );
        map.insert(
            "relationshipTypes".to_string(),
            Value::Array(vec![Value::String("RATED".to_string())]),
        );

        let config = NodeRegressionPredictPipelineBaseConfig::from_map("alice".to_string(), map)
            .with_train_config_defaults(&TestTrainConfig);

        assert_eq!(config.target_node_labels(), &["Movie".to_string()]);
        assert_eq!(config.relationship_types(), &["RATED".to_string()]);
    }
}
