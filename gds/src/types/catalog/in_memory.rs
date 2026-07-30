use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use super::{CatalogError, Dropped, GraphCatalog, GraphMemoryUsage, ListEntry};
use crate::types::graph::{degrees::Degrees, id_map::IdMap};
use crate::types::graph_store::{DefaultGraphStore, GraphStore};

#[derive(Default)]
pub struct InMemoryGraphCatalog {
    entries: RwLock<HashMap<String, Arc<DefaultGraphStore>>>,
}

impl InMemoryGraphCatalog {
    pub fn new() -> Self {
        Self {
            entries: RwLock::new(HashMap::new()),
        }
    }
}

impl GraphCatalog for InMemoryGraphCatalog {
    fn set(&self, name: &str, store: Arc<DefaultGraphStore>) {
        let mut map = self.entries.write().expect("catalog poisoned");
        map.insert(name.to_string(), store);
    }

    fn get(&self, name: &str) -> Option<Arc<DefaultGraphStore>> {
        let map = self.entries.read().ok()?;
        map.get(name).cloned()
    }

    fn with_store_mut(
        &self,
        name: &str,
        mutator: &mut dyn FnMut(&mut DefaultGraphStore),
    ) -> Result<(), CatalogError> {
        let mut map = self.entries.write().expect("catalog poisoned");
        let current = map
            .get(name)
            .ok_or_else(|| CatalogError::NotFound(name.to_string()))?;
        let mut successor = current.as_ref().clone();
        mutator(&mut successor);
        map.insert(name.to_string(), Arc::new(successor));
        Ok(())
    }

    fn drop(&self, names: &[&str], fail_if_missing: bool) -> Result<Vec<Dropped>, CatalogError> {
        let mut map = self.entries.write().expect("catalog poisoned");
        let mut dropped = Vec::with_capacity(names.len());
        for n in names {
            match map.remove(*n) {
                Some(store) => dropped.push(Dropped {
                    name: n.to_string(),
                    node_count: GraphStore::node_count(store.as_ref()) as u64,
                    relationship_count: GraphStore::relationship_count(store.as_ref()) as u64,
                }),
                None if fail_if_missing => return Err(CatalogError::NotFound((*n).to_string())),
                None => {}
            }
        }
        Ok(dropped)
    }

    fn list(&self, filter: Option<&str>, include_degree_dist: bool) -> Vec<ListEntry> {
        let map = self.entries.read().expect("catalog poisoned");
        let iter = map.iter().filter(|(name, _)| match filter {
            Some(f) => name.as_str() == f,
            None => true,
        });
        iter.map(|(name, store)| ListEntry {
            name: name.clone(),
            node_count: GraphStore::node_count(store.as_ref()) as u64,
            relationship_count: GraphStore::relationship_count(store.as_ref()) as u64,
            degree_distribution: if include_degree_dist {
                Some(simple_degree_histogram(store))
            } else {
                None
            },
        })
        .collect()
    }

    fn size_of(&self, name: &str) -> Result<GraphMemoryUsage, CatalogError> {
        let map = self.entries.read().expect("catalog poisoned");
        let store = map
            .get(name)
            .ok_or_else(|| CatalogError::NotFound(name.to_string()))?;
        // Placeholder memory accounting; can be replaced with real tracker later
        Ok(GraphMemoryUsage {
            bytes: 0,
            nodes: GraphStore::node_count(store.as_ref()) as u64,
            relationships: GraphStore::relationship_count(store.as_ref()) as u64,
        })
    }
}

fn simple_degree_histogram(store: &DefaultGraphStore) -> HashMap<u32, u64> {
    let mut hist = HashMap::new();
    let graph = store.graph();
    let n = IdMap::node_count(graph.as_ref());
    for node_id in 0..n {
        let deg = Degrees::degree(graph.as_ref(), node_id as i64) as u32;
        *hist.entry(deg).or_insert(0) += 1;
    }
    hist
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GraphStoreConfig;
    use crate::types::graph::id_map::SimpleIdMap;
    use crate::types::graph_store::{
        Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, GraphName,
    };
    use crate::types::schema::GraphSchema;
    use std::collections::HashMap;

    fn test_store() -> DefaultGraphStore {
        let mut store = DefaultGraphStore::new(
            GraphStoreConfig::default(),
            GraphName::new("test"),
            DatabaseInfo::new(
                DatabaseId::new("test"),
                DatabaseLocation::remote("localhost", 7687, None, None),
            ),
            GraphSchema::empty(),
            Capabilities::default(),
            SimpleIdMap::from_original_ids([0, 1, 2]),
            HashMap::new(),
        );
        store
            .add_node_property_i64("input".to_string(), vec![1, 2, 3])
            .expect("add input property");
        store
    }

    #[test]
    fn mutation_replaces_snapshot_without_invalidating_live_readers() {
        let catalog = InMemoryGraphCatalog::new();
        catalog.set("test", Arc::new(test_store()));

        let old_snapshot = catalog.get("test").expect("old snapshot");
        let old_nodes = old_snapshot.nodes();
        let old_input = old_snapshot
            .node_property_values("input")
            .expect("old input property");

        catalog
            .with_store_mut("test", &mut |store| {
                store
                    .add_node_property_i64("first".to_string(), vec![4, 5, 6])
                    .expect("add first property");
            })
            .expect("replace catalog snapshot");

        let first_snapshot = catalog.get("test").expect("first snapshot");
        assert!(!old_snapshot.has_node_property("first"));
        assert!(first_snapshot.has_node_property("first"));
        assert!(Arc::ptr_eq(&old_nodes, &first_snapshot.nodes()));
        assert!(Arc::ptr_eq(
            &old_input,
            &first_snapshot
                .node_property_values("input")
                .expect("shared input property")
        ));

        catalog
            .with_store_mut("test", &mut |store| {
                store
                    .add_node_property_i64("second".to_string(), vec![7, 8, 9])
                    .expect("add second property");
            })
            .expect("replace catalog snapshot again");

        let latest = catalog.get("test").expect("latest snapshot");
        assert!(latest.has_node_property("first"));
        assert!(latest.has_node_property("second"));
    }
}
