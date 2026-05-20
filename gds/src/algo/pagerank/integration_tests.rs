//! PageRank integration tests

#[cfg(test)]
mod tests {
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

    fn store_from_outgoing(outgoing: Vec<Vec<i64>>) -> DefaultGraphStore {
        let node_count = outgoing.len();

        let mut incoming: Vec<Vec<i64>> = vec![Vec::new(); node_count];
        for (source, targets) in outgoing.iter().enumerate() {
            for &target in targets {
                if target >= 0 {
                    let t = target as usize;
                    if t < node_count {
                        incoming[t].push(source as i64);
                    }
                }
            }
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
    fn pagerank_empty_edges_keeps_java_alpha_seed_and_converges() {
        let store = store_from_outgoing(vec![vec![], vec![], vec![]]);
        let graph = GraphFacade::new(Arc::new(store));

        let stats = graph
            .pagerank()
            .iterations(50)
            .tolerance(1e-12)
            .damping_factor(0.85)
            .stats()
            .unwrap();

        assert!(stats.converged);
        assert!(stats.iterations_ran <= 50);

        // Java's Pregel PageRank seeds each active node with alpha, not a normalized 1/N mass.
        let scores: Vec<f64> = graph
            .pagerank()
            .iterations(1)
            .tolerance(1e-12)
            .damping_factor(0.85)
            .stream()
            .unwrap()
            .map(|x| x.score)
            .collect();

        let sum: f64 = scores.iter().sum();
        assert!((sum - 0.45).abs() < 1e-9);
        for &s in &scores {
            assert!((s - 0.15).abs() < 1e-6);
        }
    }

    #[test]
    fn pagerank_chain_orders_sink_highest() {
        // 0 -> 1 -> 2 (2 is dangling)
        let store = store_from_outgoing(vec![vec![1], vec![2], vec![]]);
        let graph = GraphFacade::new(Arc::new(store));

        let scores: Vec<f64> = graph
            .pagerank()
            .iterations(100)
            .tolerance(1e-10)
            .damping_factor(0.85)
            .stream()
            .unwrap()
            .map(|x| x.score)
            .collect();

        assert_eq!(scores.len(), 3);
        assert!(scores[2] > scores[1]);
        assert!(scores[1] > scores[0]);
    }

    #[test]
    fn pagerank_out_of_range_sources_fall_back_to_all_nodes() {
        let store = store_from_outgoing(vec![vec![1], vec![2], vec![]]);
        let graph = GraphFacade::new(Arc::new(store));

        let scores: Vec<f64> = graph
            .pagerank()
            .source_nodes(vec![99])
            .iterations(10)
            .stream()
            .unwrap()
            .map(|x| x.score)
            .collect();

        let sum: f64 = scores.iter().sum();
        assert!(sum > 0.45);
        assert!(scores.iter().all(|score| *score > 0.0));
    }

    #[test]
    fn article_rank_uses_degree_smoothed_denominator() {
        let store = Arc::new(store_from_outgoing(vec![vec![1], vec![2], vec![]]));
        let graph = GraphFacade::new(Arc::clone(&store));

        let pagerank_scores: Vec<f64> = graph
            .pagerank()
            .iterations(20)
            .tolerance(1e-12)
            .stream()
            .unwrap()
            .map(|x| x.score)
            .collect();

        let article_scores: Vec<f64> = GraphFacade::new(store)
            .pagerank()
            .article_rank()
            .iterations(20)
            .tolerance(1e-12)
            .stream()
            .unwrap()
            .map(|x| x.score)
            .collect();

        assert_eq!(pagerank_scores.len(), article_scores.len());
        assert!(pagerank_scores
            .iter()
            .zip(article_scores.iter())
            .any(|(left, right)| (left - right).abs() > 1e-9));
    }

    #[test]
    fn eigenvector_scores_are_l2_normalized() {
        let store = store_from_outgoing(vec![vec![1], vec![2], vec![0]]);
        let graph = GraphFacade::new(Arc::new(store));

        let scores: Vec<f64> = graph
            .pagerank()
            .eigenvector()
            .iterations(20)
            .tolerance(1e-12)
            .stream()
            .unwrap()
            .map(|x| x.score)
            .collect();

        let norm = scores.iter().map(|score| score * score).sum::<f64>().sqrt();
        assert!((norm - 1.0).abs() < 1e-9);
        assert!(scores.iter().all(|score| *score > 0.0));
    }
}
