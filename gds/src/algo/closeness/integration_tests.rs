use crate::algo::closeness::CLOSENESSAlgorithmSpec;
use crate::algo::closeness::ClosenessCentralityConfig;
use crate::projection::eval::algorithm::AlgorithmSpec;
use serde_json::json;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Arc;

    use crate::config::GraphStoreConfig;
    use crate::procedures::GraphFacade;
    use crate::projection::RelationshipType;
    use crate::types::graph::{MappedNodeId, RelationshipTopology, SimpleIdMap};
    use crate::types::graph_store::{
        Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName,
    };
    use crate::types::schema::{Direction, MutableGraphSchema};

    fn store_from_directed_edges(node_count: usize, edges: &[(usize, usize)]) -> DefaultGraphStore {
        let mut outgoing: Vec<Vec<MappedNodeId>> = vec![Vec::new(); node_count];
        let mut incoming: Vec<Vec<MappedNodeId>> = vec![Vec::new(); node_count];

        for &(source, target) in edges {
            outgoing[source].push(
                MappedNodeId::try_from(target).expect("fixture target must fit mapped ID space"),
            );
            incoming[target].push(
                MappedNodeId::try_from(source).expect("fixture source must fit mapped ID space"),
            );
        }

        let rel_type = RelationshipType::of("REL");

        let mut schema_builder = MutableGraphSchema::empty();
        schema_builder
            .relationship_schema_mut()
            .add_relationship_type(rel_type.clone(), Direction::Directed);
        let schema = schema_builder.build();

        let mut relationship_topologies = HashMap::new();
        relationship_topologies.insert(
            rel_type,
            RelationshipTopology::new(outgoing, Some(incoming)),
        );

        DefaultGraphStore::new(
            GraphStoreConfig::default(),
            GraphName::new("g"),
            DatabaseInfo::new(
                DatabaseId::new("db"),
                DatabaseLocation::remote("localhost", 7687, None, None),
            ),
            schema,
            Capabilities::default(),
            SimpleIdMap::from_original_ids(
                (0..node_count).map(|node| {
                    i64::try_from(node).expect("fixture node must fit original ID space")
                }),
            ),
            relationship_topologies,
        )
    }

    #[test]
    fn test_closeness_algorithm_spec_contract_basics() {
        let algorithm = CLOSENESSAlgorithmSpec::new("test_graph".to_string());
        assert_eq!(algorithm.name(), "closeness");
        assert_eq!(algorithm.graph_name(), "test_graph");
    }

    #[test]
    fn closeness_algorithm_spec_parses_and_validates_config() {
        let algorithm = CLOSENESSAlgorithmSpec::new("test_graph".to_string());

        let parsed = algorithm
            .parse_config(&json!({ "useWassermanFaust": true }))
            .unwrap();
        let config: ClosenessCentralityConfig = serde_json::from_value(parsed).unwrap();
        assert!(config.wasserman_faust);
        assert_eq!(config.direction, "both");
        assert_eq!(config.concurrency, 4);

        let error = algorithm
            .parse_config(&json!({ "direction": "sideways" }))
            .unwrap_err();
        assert!(error.to_string().contains("direction"));
    }

    #[test]
    fn incoming_direction_uses_inverse_relationships() {
        let store = store_from_directed_edges(3, &[(0, 1), (1, 2)]);
        let graph = GraphFacade::new(Arc::new(store));

        let outgoing: Vec<_> = graph
            .closeness()
            .direction("outgoing")
            .stream()
            .unwrap()
            .map(|row| row.score)
            .collect();
        let incoming: Vec<_> = graph
            .closeness()
            .direction("incoming")
            .stream()
            .unwrap()
            .map(|row| row.score)
            .collect();

        assert!(outgoing[0].abs() < 1e-9);
        assert!((outgoing[1] - 1.0).abs() < 1e-9);
        assert!((outgoing[2] - (2.0 / 3.0)).abs() < 1e-9);

        assert!((incoming[0] - (2.0 / 3.0)).abs() < 1e-9);
        assert!((incoming[1] - 1.0).abs() < 1e-9);
        assert!(incoming[2].abs() < 1e-9);
    }

    #[test]
    fn undirected_direction_adds_missing_inverse_relationships() {
        let store = store_from_directed_edges(3, &[(0, 1), (1, 2)]);
        let graph = GraphFacade::new(Arc::new(store));

        let both: Vec<_> = graph
            .closeness()
            .direction("both")
            .stream()
            .unwrap()
            .map(|row| row.score)
            .collect();

        assert!((both[0] - (2.0 / 3.0)).abs() < 1e-9);
        assert!((both[1] - 1.0).abs() < 1e-9);
        assert!((both[2] - (2.0 / 3.0)).abs() < 1e-9);
    }

    #[test]
    fn wasserman_faust_aliases_match_java_style_config() {
        let config: crate::algo::closeness::ClosenessCentralityConfig =
            serde_json::from_value(serde_json::json!({
                "useWassermanFaust": true,
                "direction": "both",
                "concurrency": 1
            }))
            .unwrap();

        assert!(config.wasserman_faust);

        let short_alias: crate::algo::closeness::ClosenessCentralityConfig =
            serde_json::from_value(serde_json::json!({
                "useWasserman": true
            }))
            .unwrap();

        assert!(short_alias.wasserman_faust);
    }
}
