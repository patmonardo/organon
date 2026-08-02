use crate::algo::bridges::BRIDGESAlgorithmSpec;
use crate::algo::bridges::BridgesConfig;
use crate::projection::eval::algorithm::AlgorithmSpec;
use serde_json::json;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bridges_algorithm_spec_contract_basics() {
        let algorithm = BRIDGESAlgorithmSpec::new("test_graph".to_string());
        assert_eq!(algorithm.name(), "bridges");
        assert_eq!(algorithm.graph_name(), "test_graph");
    }

    #[test]
    fn test_bridges_algorithm_spec_parses_and_validates_config() {
        let algorithm = BRIDGESAlgorithmSpec::new("test_graph".to_string());

        let parsed = algorithm.parse_config(&json!({})).unwrap();
        let config: BridgesConfig = serde_json::from_value(parsed).unwrap();
        assert_eq!(config.concurrency, 4);

        let error = algorithm
            .parse_config(&json!({ "concurrency": 0 }))
            .unwrap_err();
        assert!(error.to_string().contains("concurrency"));
    }
}
