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
}
