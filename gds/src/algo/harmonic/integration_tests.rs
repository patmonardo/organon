use crate::algo::harmonic::HARMONICAlgorithmSpec;
use crate::projection::eval::algorithm::AlgorithmSpec;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GraphStoreConfig;
    use crate::procedures::GraphFacade;
    use crate::projection::RelationshipType;
    use crate::types::graph::{RelationshipTopology, SimpleIdMap};
    use crate::types::graph_store::{
        Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName,
    };
    use crate::types::schema::{Direction, MutableGraphSchema};
    use std::collections::HashMap;
    use std::sync::Arc;

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

    fn harmonic_scores(store: DefaultGraphStore, concurrency: usize) -> Vec<f64> {
        GraphFacade::new(Arc::new(store))
            .harmonic()
            .direction("both")
            .concurrency(concurrency)
            .stream()
            .unwrap()
            .map(|row| row.score)
            .collect()
    }

    fn assert_scores_close(left: &[f64], right: &[f64]) {
        assert_eq!(left.len(), right.len());
        for (idx, (left_score, right_score)) in left.iter().zip(right.iter()).enumerate() {
            assert!(
                (left_score - right_score).abs() < 1e-9,
                "score mismatch at node {idx}: {left_score} != {right_score}"
            );
        }
    }

    #[test]
    fn test_harmonic_algorithm_spec_contract_basics() {
        let algorithm = HARMONICAlgorithmSpec::new("test_graph".to_string());
        assert_eq!(algorithm.name(), "harmonic");
        assert_eq!(algorithm.graph_name(), "test_graph");
    }

    #[test]
    fn path_graph_scores_match_formula() {
        let store = store_from_undirected_edges(3, &[(0, 1), (1, 2)]);
        let scores = harmonic_scores(store, 4);

        assert_scores_close(&scores, &[0.75, 1.0, 0.75]);
    }

    #[test]
    fn disconnected_graph_unreachable_pairs_contribute_zero() {
        let store = store_from_undirected_edges(3, &[(0, 1)]);
        let scores = harmonic_scores(store, 4);

        assert_scores_close(&scores, &[0.5, 0.5, 0.0]);
    }

    #[test]
    fn harmonic_parallel_matches_single_worker() {
        let edges = &[(0, 1), (1, 2), (2, 3), (1, 4), (4, 5), (5, 6), (2, 6)];
        let single = harmonic_scores(store_from_undirected_edges(7, edges), 1);
        let parallel = harmonic_scores(store_from_undirected_edges(7, edges), 4);

        assert_scores_close(&single, &parallel);
    }
}
