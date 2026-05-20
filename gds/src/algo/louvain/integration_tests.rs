//! Louvain Integration Tests
//!
//! These tests are smoke checks to ensure the facade wiring works end-to-end.

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;

    use crate::config::GraphStoreConfig;
    use crate::procedures::GraphFacade;
    use crate::projection::RelationshipType;
    use crate::types::graph::RelationshipTopology;
    use crate::types::graph::SimpleIdMap;
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
    fn louvain_splits_isolated_nodes() {
        // 0--1 connected, 2 isolated => expect two communities.
        let store = store_from_outgoing(vec![vec![1], vec![0], vec![]]);
        let graph = GraphFacade::new(Arc::new(store));

        let result = graph.louvain().run().unwrap();
        assert_eq!(result.data.len(), 3);
        assert_eq!(result.data[0], result.data[1]);
        assert_ne!(result.data[2], result.data[0]);

        let stats = graph.louvain().stats().unwrap();
        assert_eq!(stats.node_count, 3);
        assert_eq!(stats.community_count, 2);
    }

    #[test]
    fn louvain_empty_graph_is_empty() {
        let store = store_from_outgoing(vec![]);
        let graph = GraphFacade::new(Arc::new(store));

        let result = graph.louvain().run().unwrap();
        assert!(result.data.is_empty());

        let stats = graph.louvain().stats().unwrap();
        assert_eq!(stats.node_count, 0);
        assert_eq!(stats.community_count, 0);
    }

    #[test]
    fn louvain_stats_include_node_count() {
        let stats =
            crate::algo::louvain::LouvainResultBuilder::new(crate::algo::louvain::LouvainResult {
                data: vec![1, 1, 2],
                ran_levels: 1,
                modularities: vec![0.25],
                modularity: 0.25,
                intermediate_communities: None,
                node_count: 3,
                execution_time: std::time::Duration::default(),
            })
            .stats();

        assert_eq!(stats.node_count, 3);
        assert_eq!(stats.community_count, 2);
        assert_eq!(stats.ran_levels, 1);
        assert_eq!(stats.modularity, 0.25);
    }

    #[test]
    fn louvain_tracks_intermediate_communities_when_enabled() {
        let store = store_from_outgoing(vec![vec![1], vec![0], vec![]]);
        let graph = GraphFacade::new(Arc::new(store));

        let result = graph
            .louvain()
            .with_spec_config(crate::algo::louvain::LouvainConfig {
                include_intermediate_communities: true,
                ..crate::algo::louvain::LouvainConfig::default()
            })
            .unwrap()
            .run()
            .unwrap();

        let levels = result.intermediate_communities.as_ref().unwrap();
        assert!(!levels.is_empty());
        assert!(levels.iter().all(|level| level.len() == 3));
        assert_eq!(result.intermediate_communities(0).len(), levels.len());
        assert_eq!(result.community(0), Some(result.data[0]));
    }

    #[test]
    fn louvain_result_intermediate_falls_back_to_final_assignment() {
        let result = crate::algo::louvain::LouvainResult {
            data: vec![4, 4, 7],
            ran_levels: 1,
            modularities: vec![0.0],
            modularity: 0.0,
            intermediate_communities: None,
            node_count: 3,
            execution_time: std::time::Duration::default(),
        };

        assert_eq!(result.community(2), Some(7));
        assert_eq!(result.intermediate_communities(2), vec![7]);
    }
}
