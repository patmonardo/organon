//! Betweenness Centrality integration tests

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;

    use crate::algo::betweenness::storage::BetweennessCentralityStorageRuntime;
    use crate::config::GraphStoreConfig;
    use crate::procedures::GraphFacade;
    use crate::projection::Orientation;
    use crate::projection::RelationshipType;
    use crate::types::graph::{RelationshipTopology, SimpleIdMap};
    use crate::types::graph_store::{
        Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName,
    };
    use crate::types::prelude::GraphStore;
    use crate::types::properties::relationship::{
        DefaultRelationshipPropertyValues, RelationshipPropertyValues,
    };
    use crate::types::schema::{Direction, MutableGraphSchema};

    fn store_from_undirected_edges(
        node_count: usize,
        edges: &[(usize, usize)],
    ) -> DefaultGraphStore {
        let mut outgoing: Vec<Vec<i64>> = vec![Vec::new(); node_count];
        let mut incoming: Vec<Vec<i64>> = vec![Vec::new(); node_count];

        for &(a, b) in edges {
            outgoing[a].push(b as i64);
            outgoing[b].push(a as i64);
            incoming[a].push(b as i64);
            incoming[b].push(a as i64);
        }

        let rel_type = RelationshipType::of("REL");

        let mut schema_builder = MutableGraphSchema::empty();
        schema_builder
            .relationship_schema_mut()
            .add_relationship_type(rel_type.clone(), Direction::Undirected);
        let schema = schema_builder.build();

        let mut relationship_topologies = HashMap::new();
        relationship_topologies.insert(
            rel_type,
            RelationshipTopology::new(outgoing, Some(incoming)),
        );

        let original_ids: Vec<i64> = (0..node_count as i64).collect();
        let id_map = SimpleIdMap::from_original_ids(original_ids);

        DefaultGraphStore::new(
            GraphStoreConfig::default(),
            GraphName::new("g"),
            DatabaseInfo::new(
                DatabaseId::new("db"),
                DatabaseLocation::remote("localhost", 7687, None, None),
            ),
            schema,
            Capabilities::default(),
            id_map,
            relationship_topologies,
        )
    }

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

        let original_ids: Vec<i64> = (0..node_count as i64).collect();
        let id_map = SimpleIdMap::from_original_ids(original_ids);

        DefaultGraphStore::new(
            GraphStoreConfig::default(),
            GraphName::new("g"),
            DatabaseInfo::new(
                DatabaseId::new("db"),
                DatabaseLocation::remote("localhost", 7687, None, None),
            ),
            schema,
            Capabilities::default(),
            id_map,
            relationship_topologies,
        )
    }

    fn store_from_weighted_directed_edges(
        node_count: usize,
        edges: &[(usize, usize, f64)],
        property_name: &str,
    ) -> DefaultGraphStore {
        let mut outgoing: Vec<Vec<i64>> = vec![Vec::new(); node_count];
        let mut incoming: Vec<Vec<i64>> = vec![Vec::new(); node_count];
        let mut weights = Vec::with_capacity(edges.len());

        for &(source, target, weight) in edges {
            outgoing[source].push(target as i64);
            incoming[target].push(source as i64);
            weights.push(weight);
        }

        let rel_type = RelationshipType::of("REL");

        let mut schema_builder = MutableGraphSchema::empty();
        schema_builder
            .relationship_schema_mut()
            .add_relationship_type(rel_type.clone(), Direction::Directed);
        let schema = schema_builder.build();

        let mut relationship_topologies = HashMap::new();
        relationship_topologies.insert(
            rel_type.clone(),
            RelationshipTopology::new(outgoing, Some(incoming)),
        );

        let original_ids: Vec<i64> = (0..node_count as i64).collect();
        let id_map = SimpleIdMap::from_original_ids(original_ids);

        let mut store = DefaultGraphStore::new(
            GraphStoreConfig::default(),
            GraphName::new("g"),
            DatabaseInfo::new(
                DatabaseId::new("db"),
                DatabaseLocation::remote("localhost", 7687, None, None),
            ),
            schema,
            Capabilities::default(),
            id_map,
            relationship_topologies,
        );

        let element_count = weights.len();
        let property_values: Arc<dyn RelationshipPropertyValues> = Arc::new(
            DefaultRelationshipPropertyValues::with_values(weights, 0.0, element_count),
        );
        store
            .add_relationship_property(rel_type, property_name, property_values)
            .unwrap();

        store
    }

    #[test]
    fn path_graph_middle_is_bridge_node() {
        // 0-1-2
        let store = store_from_undirected_edges(3, &[(0, 1), (1, 2)]);
        let graph = GraphFacade::new(Arc::new(store));

        let rows: Vec<_> = graph
            .betweenness()
            .direction("both")
            .concurrency(2)
            .stream()
            .unwrap()
            .collect();

        let mut scores = vec![0.0; 3];
        for r in rows {
            scores[r.node_id as usize] = r.score;
        }

        assert!((scores[1] - 1.0).abs() < 1e-9);
        assert!(scores[0].abs() < 1e-9);
        assert!(scores[2].abs() < 1e-9);
    }

    #[test]
    fn random_degree_sampling_is_deterministic() {
        let store = store_from_undirected_edges(
            6,
            &[(0, 1), (0, 2), (0, 3), (1, 4), (2, 5), (3, 4), (4, 5)],
        );
        let storage =
            BetweennessCentralityStorageRuntime::new(&store, Orientation::Undirected, None)
                .unwrap();

        let first = storage
            .select_sources("random_degree", Some(3), 42)
            .unwrap();
        let second = storage.select_sources("randomDegree", Some(3), 42).unwrap();

        assert_eq!(first, second);
        assert_eq!(first.len(), 3);
    }

    #[test]
    fn invalid_sampling_strategy_fails_fast() {
        let store = store_from_undirected_edges(3, &[(0, 1), (1, 2)]);
        let storage =
            BetweennessCentralityStorageRuntime::new(&store, Orientation::Undirected, None)
                .unwrap();

        assert!(storage
            .select_sources("highest_degree", Some(2), 42)
            .is_err());
    }

    #[test]
    fn directed_orientation_changes_against_undirected_projection() {
        let store = store_from_directed_edges(4, &[(0, 1), (1, 3), (0, 2)]);
        let graph = GraphFacade::new(Arc::new(store));

        let outgoing: Vec<_> = graph
            .betweenness()
            .direction("outgoing")
            .stream()
            .unwrap()
            .map(|row| row.score)
            .collect();
        let both: Vec<_> = graph
            .betweenness()
            .direction("both")
            .stream()
            .unwrap()
            .map(|row| row.score)
            .collect();

        assert_ne!(outgoing, both);
    }

    #[test]
    fn incoming_direction_uses_inverse_relationships() {
        let store = store_from_directed_edges(3, &[(0, 1), (1, 2)]);
        let graph = GraphFacade::new(Arc::new(store));

        let incoming: Vec<_> = graph
            .betweenness()
            .direction("incoming")
            .stream()
            .unwrap()
            .map(|row| row.score)
            .collect();

        assert!((incoming[1] - 1.0).abs() < 1e-9);
        assert!(incoming[0].abs() < 1e-9);
        assert!(incoming[2].abs() < 1e-9);
    }

    #[test]
    fn undirected_direction_adds_missing_inverse_relationships() {
        let store = store_from_directed_edges(3, &[(0, 1), (1, 2)]);
        let graph = GraphFacade::new(Arc::new(store));

        let both: Vec<_> = graph
            .betweenness()
            .direction("both")
            .stream()
            .unwrap()
            .map(|row| row.score)
            .collect();

        assert!((both[1] - 1.0).abs() < 1e-9);
        assert!(both[0].abs() < 1e-9);
        assert!(both[2].abs() < 1e-9);
    }

    #[test]
    fn weighted_relationship_property_selects_dijkstra_traversal() {
        let store = store_from_weighted_directed_edges(
            3,
            &[(0, 2, 10.0), (0, 1, 1.0), (1, 2, 1.0)],
            "cost",
        );
        let graph = GraphFacade::new(Arc::new(store));

        let unweighted: Vec<_> = graph
            .betweenness()
            .direction("outgoing")
            .stream()
            .unwrap()
            .map(|row| row.score)
            .collect();
        let weighted: Vec<_> = graph
            .betweenness()
            .direction("outgoing")
            .relationship_weight_property(Some("cost".to_string()))
            .stream()
            .unwrap()
            .map(|row| row.score)
            .collect();

        assert!(unweighted[1].abs() < 1e-9);
        assert!((weighted[1] - 1.0).abs() < 1e-9);
    }

    #[test]
    fn config_accepts_java_style_aliases() {
        let config: crate::algo::betweenness::BetweennessCentralityConfig =
            serde_json::from_value(serde_json::json!({
                "direction": "outgoing",
                "relationshipWeightProperty": "cost",
                "samplingStrategy": "randomDegree",
                "samplingSize": 2,
                "randomSeed": 7,
                "concurrency": 1
            }))
            .unwrap();

        assert_eq!(config.relationship_weight_property.as_deref(), Some("cost"));
        assert_eq!(config.sampling_strategy, "randomDegree");
        assert_eq!(config.sampling_size, Some(2));
        assert_eq!(config.random_seed, 7);
    }
}
