use crate::algo::closeness::CLOSENESSAlgorithmSpec;
use crate::projection::eval::algorithm::AlgorithmSpec;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Arc;

    use crate::config::GraphStoreConfig;
    use crate::procedures::GraphFacade;
    use crate::projection::RelationshipType;
    use crate::types::graph::{RelationshipTopology, SimpleIdMap};
    use crate::types::graph_store::{
        Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName,
    };
    use crate::types::schema::{Direction, MutableGraphSchema};

    fn store_from_directed_edges(node_count: usize, edges: &[(usize, usize)]) -> DefaultGraphStore {
        let mut outgoing: Vec<Vec<i64>> = vec![Vec::new(); node_count];
        let mut incoming: Vec<Vec<i64>> = vec![Vec::new(); node_count];

        for &(source, target) in edges {
            outgoing[source].push(target as i64);
            incoming[target].push(source as i64);
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
            SimpleIdMap::from_original_ids(0..node_count as i64),
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
