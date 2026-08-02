//! K1-Coloring integration tests

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;

    use crate::algo::k1coloring::K1COLORINGAlgorithmSpec;
    use crate::algo::k1coloring::K1ColoringConfig;
    use crate::config::GraphStoreConfig;
    use crate::procedures::GraphFacade;
    use crate::projection::eval::algorithm::AlgorithmSpec;
    use crate::projection::RelationshipType;
    use crate::types::graph::{MappedNodeId, RelationshipTopology, SimpleIdMap};
    use crate::types::graph_store::{
        Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName,
    };
    use crate::types::schema::{Direction, MutableGraphSchema};
    use serde_json::json;

    fn node(value: u64) -> MappedNodeId {
        MappedNodeId::new(value)
    }

    #[test]
    fn k1coloring_algorithm_spec_parses_and_validates_config() {
        let spec = K1COLORINGAlgorithmSpec::new("test_graph".to_string());

        let parsed = spec
            .parse_config(&json!({
                "maxIterations": 20,
                "minBatchSize": 64
            }))
            .unwrap();
        let config: K1ColoringConfig = serde_json::from_value(parsed).unwrap();
        assert_eq!(config.concurrency, 4);
        assert_eq!(config.max_iterations, 20);
        assert_eq!(config.min_batch_size, 64);

        let error = spec
            .parse_config(&json!({ "maxIterations": 0 }))
            .unwrap_err();
        assert!(error.to_string().contains("maxIterations"));
    }

    fn store_from_outgoing(outgoing: Vec<Vec<MappedNodeId>>) -> DefaultGraphStore {
        let node_count = outgoing.len();

        let mut incoming: Vec<Vec<MappedNodeId>> = vec![Vec::new(); node_count];
        for (source, targets) in outgoing.iter().enumerate() {
            for &target in targets {
                let target = target
                    .to_usize()
                    .expect("fixture target must fit the dense index domain");
                if target < node_count {
                    incoming[target].push(
                        MappedNodeId::try_from(source)
                            .expect("fixture source must fit the mapped ID domain"),
                    );
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

        let original_ids: Vec<i64> = (0..node_count)
            .map(|node| i64::try_from(node).expect("fixture node must fit the original ID domain"))
            .collect();
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

    fn assert_valid_coloring(outgoing: &[Vec<MappedNodeId>], colors: &[u64]) {
        for (u, nbrs) in outgoing.iter().enumerate() {
            for &v in nbrs {
                let v = v
                    .to_usize()
                    .expect("fixture target must fit the dense index domain");
                if v == u {
                    continue;
                }
                assert_ne!(colors[u], colors[v], "conflict on edge {u}->{v}");
            }
        }
    }

    #[test]
    fn k1coloring_triangle_uses_three_colors() {
        // Triangle (undirected modeled as symmetric directed edges)
        let outgoing = vec![
            vec![node(1), node(2)],
            vec![node(0), node(2)],
            vec![node(0), node(1)],
        ];
        let store = store_from_outgoing(outgoing.clone());
        let graph = GraphFacade::new(Arc::new(store));

        let result = graph.k1coloring().max_iterations(20).run().unwrap();
        assert!(result.did_converge);
        assert_eq!(result.colors.len(), 3);

        assert_valid_coloring(&outgoing, &result.colors);

        let used: std::collections::HashSet<u64> = result.colors.iter().copied().collect();
        assert_eq!(used.len(), 3);
    }

    #[test]
    fn k1coloring_path_is_two_colorable() {
        // Path 0-1-2-3
        let outgoing = vec![
            vec![node(1)],
            vec![node(0), node(2)],
            vec![node(1), node(3)],
            vec![node(2)],
        ];
        let store = store_from_outgoing(outgoing.clone());
        let graph = GraphFacade::new(Arc::new(store));

        let result = graph.k1coloring().max_iterations(20).run().unwrap();
        assert_valid_coloring(&outgoing, &result.colors);

        let used: std::collections::HashSet<u64> = result.colors.iter().copied().collect();
        assert!(!used.is_empty());
    }

    #[test]
    fn k1coloring_is_valid_at_concurrency_one_and_four() {
        let outgoing = vec![
            vec![node(1), node(2)],
            vec![node(0), node(2), node(3)],
            vec![node(0), node(1), node(4)],
            vec![node(1), node(4), node(5)],
            vec![node(2), node(3), node(5)],
            vec![node(3), node(4)],
        ];

        for concurrency in [1, 4] {
            let store = store_from_outgoing(outgoing.clone());
            let graph = GraphFacade::new(Arc::new(store));
            let result = graph
                .k1coloring()
                .concurrency(concurrency)
                .max_iterations(20)
                .run()
                .unwrap();

            assert!(result.did_converge);
            assert_valid_coloring(&outgoing, &result.colors);
        }
    }

    #[test]
    fn k1coloring_ignores_self_loops_when_validating_colors() {
        let outgoing = vec![vec![node(0), node(1)], vec![node(0), node(1)]];
        let store = store_from_outgoing(outgoing.clone());
        let graph = GraphFacade::new(Arc::new(store));

        let result = graph.k1coloring().concurrency(4).run().unwrap();

        assert!(result.did_converge);
        assert_valid_coloring(&outgoing, &result.colors);
    }

    #[test]
    fn k1coloring_isolated_nodes_converge_to_single_color() {
        let outgoing = vec![vec![], vec![], vec![], vec![]];
        let store = store_from_outgoing(outgoing.clone());
        let graph = GraphFacade::new(Arc::new(store));

        let result = graph.k1coloring().max_iterations(3).run().unwrap();
        assert!(result.did_converge);
        assert_eq!(result.ran_iterations, 1);
        assert_eq!(result.colors, vec![0, 0, 0, 0]);
        assert_valid_coloring(&outgoing, &result.colors);
    }

    #[test]
    fn k1coloring_final_iteration_convergence_reports_non_converged() {
        let outgoing = vec![vec![], vec![], vec![], vec![]];
        let store = store_from_outgoing(outgoing.clone());
        let graph = GraphFacade::new(Arc::new(store));

        let result = graph.k1coloring().max_iterations(1).run().unwrap();
        assert!(!result.did_converge);
        assert_eq!(result.ran_iterations, 1);
        assert_eq!(result.colors, vec![0, 0, 0, 0]);
        assert_valid_coloring(&outgoing, &result.colors);
    }

    #[test]
    fn k1coloring_empty_graph_converges_without_iterations() {
        let outgoing = Vec::new();
        let store = store_from_outgoing(outgoing);
        let graph = GraphFacade::new(Arc::new(store));

        let result = graph.k1coloring().run().unwrap();
        assert!(result.did_converge);
        assert_eq!(result.ran_iterations, 0);
        assert!(result.colors.is_empty());
    }

    #[test]
    fn k1coloring_memory_estimate_tracks_concurrency() {
        let outgoing = vec![vec![node(1)], vec![node(0)]];
        let store = store_from_outgoing(outgoing);
        let graph = GraphFacade::new(Arc::new(store));

        let single = graph.k1coloring().concurrency(1).estimate_memory().unwrap();
        let parallel = graph.k1coloring().concurrency(4).estimate_memory().unwrap();

        assert!(parallel.min() > single.min());
        assert_eq!(parallel.min(), parallel.max());
    }

    #[test]
    fn k1coloring_stats_include_node_count() {
        let outgoing = vec![vec![node(1)], vec![node(0), node(2)], vec![node(1)]];
        let store = store_from_outgoing(outgoing);
        let graph = GraphFacade::new(Arc::new(store));

        let stats = graph.k1coloring().max_iterations(20).stats().unwrap();
        assert_eq!(stats.node_count, 3);
        assert!(stats.did_converge);
        assert!(stats.ran_iterations > 0);
        assert!(stats.color_count > 0);
    }
}
