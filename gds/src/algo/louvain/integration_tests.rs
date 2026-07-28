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
    use crate::task::concurrency::TerminationFlag;
    use crate::task::progress::{TaskProgressTracker, Tasks};
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
    fn louvain_preserves_seed_property_on_isolated_nodes() {
        let mut store = store_from_outgoing(vec![vec![], vec![], vec![]]);
        store
            .add_node_property_i64("seed".to_string(), vec![10, 10, 20])
            .unwrap();
        let graph = GraphFacade::new(Arc::new(store));

        let result = graph.louvain().seed_property("seed").run().unwrap();

        assert_eq!(result.data, vec![10, 10, 20]);
    }

    #[test]
    fn louvain_rejects_missing_seed_property() {
        let store = store_from_outgoing(vec![vec![], vec![]]);
        let graph = GraphFacade::new(Arc::new(store));

        let error = graph.louvain().seed_property("missing").run().unwrap_err();

        assert!(error.to_string().contains("does not exist"));
    }

    #[test]
    fn louvain_rejects_negative_seed_values() {
        let mut store = store_from_outgoing(vec![vec![], vec![]]);
        store
            .add_node_property_i64("seed".to_string(), vec![10, -2])
            .unwrap();
        let graph = GraphFacade::new(Arc::new(store));

        let error = graph.louvain().seed_property("seed").run().unwrap_err();

        assert!(error.to_string().contains("non-negative"));
    }

    #[test]
    #[should_panic(expected = "The execution has been terminated.")]
    fn louvain_run_with_context_honors_termination() {
        let store = store_from_outgoing(vec![vec![1], vec![0]]);
        let graph = GraphFacade::new(Arc::new(store));
        let mut progress = TaskProgressTracker::new(Tasks::leaf("louvain".to_string()));

        let _ = graph
            .louvain()
            .run_with_context(&mut progress, &TerminationFlag::stop_running());
    }

    #[test]
    fn louvain_config_accepts_camel_case_seed_property() {
        let config: crate::algo::louvain::LouvainConfig =
            serde_json::from_value(serde_json::json!({ "seedProperty": "seed" })).unwrap();

        assert_eq!(config.seed_property.as_deref(), Some("seed"));
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

    #[test]
    fn louvain_k1_scheduler_is_equivalent_across_concurrency() {
        let outgoing = vec![
            vec![1, 2],
            vec![0, 2],
            vec![0, 1, 3],
            vec![2, 4, 5],
            vec![3, 5],
            vec![3, 4],
        ];
        let run = |concurrency| {
            let graph = GraphFacade::new(Arc::new(store_from_outgoing(outgoing.clone())));
            graph.louvain().concurrency(concurrency).run().unwrap()
        };

        let sequential = run(1);
        let parallel = run(4);

        assert_eq!(
            normalized_partition(&sequential.data),
            normalized_partition(&parallel.data)
        );
        assert!((sequential.modularity - parallel.modularity).abs() < 1e-12);
        assert_eq!(sequential.ran_levels, parallel.ran_levels);
    }

    #[test]
    fn seeded_louvain_preserves_external_labels_across_concurrency() {
        let outgoing = vec![vec![1], vec![0, 2], vec![1, 3], vec![2]];
        let run = |concurrency| {
            let mut store = store_from_outgoing(outgoing.clone());
            store
                .add_node_property_i64("seed".to_string(), vec![10, 10, 20, 20])
                .unwrap();
            GraphFacade::new(Arc::new(store))
                .louvain()
                .concurrency(concurrency)
                .seed_property("seed")
                .run()
                .unwrap()
        };

        let sequential = run(1);
        let parallel = run(4);

        assert_eq!(sequential.data, parallel.data);
        assert!(sequential
            .data
            .iter()
            .all(|community| [10, 20].contains(community)));
        assert!((sequential.modularity - parallel.modularity).abs() < 1e-12);
    }

    fn normalized_partition(communities: &[u64]) -> Vec<usize> {
        let mut dense = HashMap::new();
        communities
            .iter()
            .map(|community| {
                let next = dense.len();
                *dense.entry(*community).or_insert(next)
            })
            .collect()
    }
}
