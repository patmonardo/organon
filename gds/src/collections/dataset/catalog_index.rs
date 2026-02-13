//! Catalog/index utility for compiled dataset IR.

use std::collections::{BTreeMap, BTreeSet};

use crate::collections::dataset::compile_ir::{DatasetCompilation, DatasetNode, DatasetNodeKind};

/// Derived index over compilation nodes to support multi-pass exploration.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DatasetCatalogIndex {
    by_kind: BTreeMap<DatasetNodeKind, BTreeSet<String>>,
    reverse_deps: BTreeMap<String, BTreeSet<String>>,
}

impl DatasetCatalogIndex {
    pub fn from_compilation(compilation: &DatasetCompilation) -> Self {
        let mut index = Self::default();

        for (node_id, node) in &compilation.nodes {
            index
                .by_kind
                .entry(node.kind)
                .or_default()
                .insert(node_id.clone());

            for dep in &node.depends_on {
                index
                    .reverse_deps
                    .entry(dep.clone())
                    .or_default()
                    .insert(node_id.clone());
            }
        }

        index
    }

    pub fn nodes_of_kind<'a>(
        &'a self,
        compilation: &'a DatasetCompilation,
        kind: DatasetNodeKind,
    ) -> Vec<&'a DatasetNode> {
        self.by_kind
            .get(&kind)
            .into_iter()
            .flat_map(|ids| ids.iter())
            .filter_map(|node_id| compilation.nodes.get(node_id))
            .collect()
    }

    pub fn dependents_of<'a>(
        &'a self,
        compilation: &'a DatasetCompilation,
        node_id: &str,
    ) -> Vec<&'a DatasetNode> {
        self.reverse_deps
            .get(node_id)
            .into_iter()
            .flat_map(|ids| ids.iter())
            .filter_map(|id| compilation.nodes.get(id))
            .collect()
    }

    pub fn all_kinds(&self) -> Vec<DatasetNodeKind> {
        self.by_kind.keys().copied().collect()
    }
}
