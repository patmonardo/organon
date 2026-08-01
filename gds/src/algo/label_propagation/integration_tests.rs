//! Label Propagation integration tests

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;

    use crate::config::GraphStoreConfig;
    use crate::procedures::GraphFacade;
    use crate::projection::RelationshipType;
    use crate::task::concurrency::TerminationFlag;
    use crate::task::progress::{TaskProgressTracker, Tasks};
    use crate::types::graph::{MappedNodeId, RelationshipTopology, SimpleIdMap};
    use crate::types::graph_store::{
        Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName,
    };
    use crate::types::schema::{Direction, MutableGraphSchema};

    fn node(value: u64) -> MappedNodeId {
        MappedNodeId::new(value)
    }

    fn store_from_outgoing(outgoing: Vec<Vec<(MappedNodeId, f64)>>) -> DefaultGraphStore {
        let node_count = outgoing.len();

        let mut out_ids: Vec<Vec<MappedNodeId>> = vec![Vec::new(); node_count];
        for (s, targets) in outgoing.iter().enumerate() {
            for &(t, _w) in targets {
                out_ids[s].push(t);
            }
        }

        let mut incoming: Vec<Vec<MappedNodeId>> = vec![Vec::new(); node_count];
        for (source, targets) in out_ids.iter().enumerate() {
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
        relationship_topologies
            .insert(rel_type, RelationshipTopology::new(out_ids, Some(incoming)));

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

    #[test]
    fn label_propagation_converges_on_two_components() {
        // Two disconnected edges: 0-1 and 2-3
        let outgoing = vec![
            vec![(node(1), 1.0)],
            vec![(node(0), 1.0)],
            vec![(node(3), 1.0)],
            vec![(node(2), 1.0)],
        ];
        let store = store_from_outgoing(outgoing);
        let graph = GraphFacade::new(Arc::new(store));

        let result = graph.label_propagation().max_iterations(20).run().unwrap();

        assert!(result.did_converge);
        assert_eq!(result.labels.len(), 4);

        // Each edge should end up with a shared label.
        assert_eq!(result.labels[0], result.labels[1]);
        assert_eq!(result.labels[2], result.labels[3]);
        assert_ne!(result.labels[0], result.labels[2]);
    }

    #[test]
    fn label_propagation_tie_breaks_to_smallest_label() {
        // Star: 0 connected to 1 and 2 (symmetric).
        // With identity init labels, node 0 sees labels {1,2} equal weight -> picks 1.
        let outgoing = vec![
            vec![(node(1), 1.0), (node(2), 1.0)],
            vec![(node(0), 1.0)],
            vec![(node(0), 1.0)],
        ];
        let store = store_from_outgoing(outgoing);
        let graph = GraphFacade::new(Arc::new(store));

        let result = graph
            .label_propagation()
            .concurrency(1)
            .max_iterations(1)
            .run()
            .unwrap();
        assert_eq!(result.labels[0], 1);
    }

    #[test]
    fn label_propagation_respects_seed_property_on_isolated_nodes() {
        let mut store = store_from_outgoing(vec![vec![], vec![], vec![]]);
        store
            .add_node_property_i64("seed".to_string(), vec![42, 42, 99])
            .unwrap();
        let graph = GraphFacade::new(Arc::new(store));

        let result = graph
            .label_propagation()
            .concurrency(1)
            .seed_property("seed")
            .max_iterations(5)
            .run()
            .unwrap();

        assert!(result.did_converge);
        assert_eq!(result.ran_iterations, 1);
        assert_eq!(result.labels, vec![42, 42, 99]);
    }

    #[test]
    fn label_propagation_node_weights_influence_voting() {
        let mut store = store_from_outgoing(vec![
            vec![(node(1), 1.0), (node(2), 1.0)],
            vec![(node(0), 1.0)],
            vec![(node(0), 1.0)],
        ]);
        store
            .add_node_property_i64("seed".to_string(), vec![100, 10, 20])
            .unwrap();
        store
            .add_node_property_f64("weight".to_string(), vec![1.0, 1.0, 10.0])
            .unwrap();
        let graph = GraphFacade::new(Arc::new(store));

        let result = graph
            .label_propagation()
            .concurrency(1)
            .seed_property("seed")
            .node_weight_property("weight")
            .max_iterations(1)
            .run()
            .unwrap();

        assert_eq!(result.labels[0], 20);
    }

    #[test]
    fn label_propagation_isolated_nodes_retain_unique_labels() {
        let store = store_from_outgoing(vec![vec![]; 100]);
        let graph = GraphFacade::new(Arc::new(store));

        let result = graph.label_propagation().max_iterations(10).run().unwrap();

        assert!(result.did_converge);
        assert_eq!(result.ran_iterations, 1);
        assert_eq!(result.labels.len(), 100);
        for (node_id, label) in result.labels.iter().copied().enumerate() {
            assert_eq!(
                label,
                u64::try_from(node_id).expect("fixture node must fit the label domain")
            );
        }
    }

    #[test]
    fn label_propagation_parallel_workers_preserve_component_partition() {
        let store = store_from_outgoing(vec![
            vec![(node(1), 1.0), (node(2), 1.0)],
            vec![(node(0), 1.0), (node(2), 1.0)],
            vec![(node(0), 1.0), (node(1), 1.0)],
            vec![(node(4), 1.0), (node(5), 1.0)],
            vec![(node(3), 1.0), (node(5), 1.0)],
            vec![(node(3), 1.0), (node(4), 1.0)],
        ]);
        let graph = GraphFacade::new(Arc::new(store));

        let result = graph
            .label_propagation()
            .concurrency(4)
            .max_iterations(20)
            .run()
            .unwrap();

        assert!(result.did_converge);
        assert!(result.labels[0..3]
            .iter()
            .all(|label| *label == result.labels[0]));
        assert!(result.labels[3..6]
            .iter()
            .all(|label| *label == result.labels[3]));
        assert_ne!(result.labels[0], result.labels[3]);
    }

    #[test]
    fn label_propagation_rejects_missing_requested_properties() {
        let store = store_from_outgoing(vec![vec![(node(1), 1.0)], vec![(node(0), 1.0)]]);
        let graph = GraphFacade::new(Arc::new(store));

        let weight_error = graph
            .label_propagation()
            .node_weight_property("missing_weight")
            .run()
            .unwrap_err();
        assert!(weight_error.to_string().contains("not found"));

        let seed_error = graph
            .label_propagation()
            .seed_property("missing_seed")
            .run()
            .unwrap_err();
        assert!(seed_error.to_string().contains("not found"));
    }

    #[test]
    fn label_propagation_rejects_negative_seed_values() {
        let mut store = store_from_outgoing(vec![vec![], vec![]]);
        store
            .add_node_property_i64("seed".to_string(), vec![10, -2])
            .unwrap();
        let graph = GraphFacade::new(Arc::new(store));

        let error = graph
            .label_propagation()
            .seed_property("seed")
            .run()
            .unwrap_err();

        assert!(error.to_string().contains("non-negative"));
    }

    #[test]
    #[should_panic(expected = "The execution has been terminated.")]
    fn label_propagation_run_with_context_honors_termination() {
        let store = store_from_outgoing(vec![vec![(node(1), 1.0)], vec![(node(0), 1.0)]]);
        let graph = GraphFacade::new(Arc::new(store));
        let mut progress = TaskProgressTracker::new(Tasks::leaf("label_propagation".to_string()));

        let _ = graph
            .label_propagation()
            .run_with_context(&mut progress, &TerminationFlag::stop_running());
    }
}
