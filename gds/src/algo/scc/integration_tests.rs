//! SCC Integration Tests
//!
//! Validates SCC behavior via the Graph facade and DefaultGraphStore fixture helpers.

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;

    use crate::algo::scc::SCCAlgorithmSpec;
    use crate::algo::scc::SccConfig;
    use crate::config::GraphStoreConfig;
    use crate::procedures::GraphFacade;
    use crate::projection::eval::algorithm::AlgorithmSpec;
    use crate::projection::RelationshipType;
    use crate::types::graph::MappedNodeId;
    use crate::types::graph::OriginalNodeId;
    use crate::types::graph::RelationshipTopology;
    use crate::types::graph::SimpleIdMap;
    use crate::types::graph_store::{
        Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName,
        GraphStore,
    };
    use crate::types::schema::{Direction, MutableGraphSchema};
    use serde_json::json;

    fn node(value: u64) -> MappedNodeId {
        MappedNodeId::new(value)
    }

    #[test]
    fn scc_algorithm_spec_parses_and_validates_config() {
        let spec = SCCAlgorithmSpec::new("test_graph".to_string());

        let parsed = spec.parse_config(&json!({})).unwrap();
        let config: SccConfig = serde_json::from_value(parsed).unwrap();
        assert_eq!(config.concurrency, 4);

        let error = spec.parse_config(&json!({ "concurrency": 0 })).unwrap_err();
        assert!(error.to_string().contains("concurrency"));
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
            .map(|node| {
                100_i64
                    .checked_add(
                        i64::try_from(node).expect("fixture node must fit the original ID domain"),
                    )
                    .expect("fixture original ID must not overflow")
            })
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

    #[test]
    fn scc_single_cycle_is_one_component() {
        // 0 -> 1 -> 2 -> 0
        let store = store_from_outgoing(vec![vec![node(1)], vec![node(2)], vec![node(0)]]);
        let id_map = store.nodes();
        assert_eq!(
            id_map.to_mapped_node_id(OriginalNodeId::new(100)),
            Some(node(0))
        );
        assert_eq!(
            id_map.to_original_node_id(node(2)),
            Some(OriginalNodeId::new(102))
        );
        let graph = GraphFacade::new(Arc::new(store));

        let result = graph.scc().run().unwrap();
        assert_eq!(result.component_count, 1);
        assert_eq!(result.components.len(), 3);
        assert_eq!(result.components, vec![0, 0, 0]);
        assert_eq!(result.components[0], result.components[1]);
        assert_eq!(result.components[1], result.components[2]);
    }

    #[test]
    fn scc_two_disjoint_cycles_are_two_components() {
        // (0 <-> 1) and (2 <-> 3)
        let store = store_from_outgoing(vec![
            vec![node(1)],
            vec![node(0)],
            vec![node(3)],
            vec![node(2)],
        ]);
        let graph = GraphFacade::new(Arc::new(store));

        let result = graph.scc().run().unwrap();
        assert_eq!(result.component_count, 2);
        assert_eq!(result.components.len(), 4);
        assert_eq!(result.components, vec![0, 0, 2, 2]);
        assert_eq!(result.components[0], result.components[1]);
        assert_eq!(result.components[2], result.components[3]);
        assert_ne!(result.components[0], result.components[2]);
    }

    #[test]
    fn scc_directed_chain_has_each_node_its_own_component() {
        // 0 -> 1 -> 2 -> 3
        let store = store_from_outgoing(vec![vec![node(1)], vec![node(2)], vec![node(3)], vec![]]);
        let graph = GraphFacade::new(Arc::new(store));

        let result = graph.scc().run().unwrap();
        assert_eq!(result.component_count, 4);
        assert_eq!(result.components.len(), 4);
        assert_eq!(result.components, vec![0, 1, 2, 3]);
        assert_ne!(result.components[0], result.components[1]);
        assert_ne!(result.components[1], result.components[2]);
        assert_ne!(result.components[2], result.components[3]);
    }

    #[test]
    fn scc_isolated_nodes_each_form_component() {
        let store = store_from_outgoing(vec![vec![], vec![], vec![]]);
        let graph = GraphFacade::new(Arc::new(store));

        let result = graph.scc().run().unwrap();
        assert_eq!(result.component_count, 3);
        assert_eq!(result.components.len(), 3);
        assert_eq!(result.components, vec![0, 1, 2]);
        assert_ne!(result.components[0], result.components[1]);
        assert_ne!(result.components[1], result.components[2]);
    }

    #[test]
    fn scc_empty_graph_has_no_components() {
        let store = store_from_outgoing(vec![]);
        let graph = GraphFacade::new(Arc::new(store));

        let result = graph.scc().run().unwrap();
        assert_eq!(result.component_count, 0);
        assert!(result.components.is_empty());
    }

    #[test]
    fn scc_mixed_component_shapes() {
        // (0 <-> 1), (2 -> 3 -> 4 -> 2), and 5 isolated.
        let store = store_from_outgoing(vec![
            vec![node(1)],
            vec![node(0)],
            vec![node(3)],
            vec![node(4)],
            vec![node(2)],
            vec![],
        ]);
        let graph = GraphFacade::new(Arc::new(store));

        let result = graph.scc().run().unwrap();
        assert_eq!(result.component_count, 3);
        assert_eq!(result.components, vec![0, 0, 2, 2, 2, 5]);
        assert_eq!(result.components[0], result.components[1]);
        assert_eq!(result.components[2], result.components[3]);
        assert_eq!(result.components[3], result.components[4]);
        assert_ne!(result.components[0], result.components[2]);
        assert_ne!(result.components[0], result.components[5]);
        assert_ne!(result.components[2], result.components[5]);
    }

    #[test]
    fn scc_many_isolated_nodes_each_form_component() {
        let store = store_from_outgoing(vec![vec![]; 1_000]);
        let graph = GraphFacade::new(Arc::new(store));

        let result = graph.scc().run().unwrap();
        assert_eq!(result.component_count, 1_000);
        assert_eq!(result.components.len(), 1_000);
        assert_eq!(result.components[0], 0);
        assert_eq!(result.components[999], 999);
    }

    #[test]
    fn scc_stats_include_node_count() {
        let store = store_from_outgoing(vec![vec![node(1)], vec![node(2)], vec![node(0)], vec![]]);
        let graph = GraphFacade::new(Arc::new(store));

        let stats = graph.scc().stats().unwrap();
        assert_eq!(stats.node_count, 4);
        assert_eq!(stats.component_count, 2);
    }
}
