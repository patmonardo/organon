use crate::algo::degree_centrality::DEGREE_CENTRALITYAlgorithmSpec;
use crate::algo::degree_centrality::DegreeCentralityConfig;
use crate::projection::eval::algorithm::AlgorithmSpec;
use serde_json::json;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_degree_centrality_algorithm_spec_contract_basics() {
        let algorithm = DEGREE_CENTRALITYAlgorithmSpec::new("test_graph".to_string());
        assert_eq!(algorithm.name(), "degree_centrality");
        assert_eq!(algorithm.graph_name(), "test_graph");
    }

    #[test]
    fn test_degree_centrality_algorithm_spec_parses_and_validates_config() {
        let algorithm = DEGREE_CENTRALITYAlgorithmSpec::new("test_graph".to_string());

        let parsed = algorithm
            .parse_config(&json!({
                "normalize": true,
                "relationshipWeightProperty": "cost"
            }))
            .unwrap();
        let config: DegreeCentralityConfig = serde_json::from_value(parsed).unwrap();
        assert!(config.normalize);
        assert_eq!(config.orientation, "natural");
        assert_eq!(config.relationship_weight_property.as_deref(), Some("cost"));
        assert_eq!(config.concurrency, 4);

        let error = algorithm
            .parse_config(&json!({ "orientation": "sideways" }))
            .unwrap_err();
        assert!(error.to_string().contains("orientation"));
    }
}
