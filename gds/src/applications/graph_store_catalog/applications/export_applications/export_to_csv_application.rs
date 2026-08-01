use crate::applications::graph_store_catalog::results::ExportResult;
use crate::projection::RelationshipType;
use crate::types::graph::IdMap as _;
use crate::types::graph::MappedNodeId;
use crate::types::graph_store::DefaultGraphStore;
use crate::types::graph_store::GraphStore as _;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

/// ExportToCsvApplication
///
/// Java parity: exports the catalog graph to CSV.
/// Rust pass-1: exports a CSV representation from the in-memory `DefaultGraphStore`.
#[derive(Clone, Debug, Default)]
pub struct ExportToCsvApplication;

impl ExportToCsvApplication {
    pub fn compute(
        &self,
        graph_store: &DefaultGraphStore,
        export_path: &str,
    ) -> Result<ExportResult, String> {
        let export_root = PathBuf::from(export_path);
        if export_root.as_os_str().is_empty() {
            return Err("export_path must not be empty".to_string());
        }

        fs::create_dir_all(&export_root).map_err(|e| {
            format!(
                "Failed to create export directory '{}': {e}",
                export_root.display()
            )
        })?;

        let nodes_exported = self.write_nodes_csv(graph_store, &export_root)?;
        let relationships_exported = self.write_relationships_csv(graph_store, &export_root)?;

        Ok(ExportResult::new(
            nodes_exported,
            relationships_exported,
            Some(export_root.to_string_lossy().to_string()),
        ))
    }

    fn write_nodes_csv(
        &self,
        graph_store: &DefaultGraphStore,
        export_root: &Path,
    ) -> Result<u64, String> {
        let graph = graph_store.graph();
        let file_path = export_root.join("nodes.csv");
        let mut file = fs::File::create(&file_path)
            .map_err(|e| format!("Failed to create '{}': {e}", file_path.display()))?;

        writeln!(file, "id,labels")
            .map_err(|e| format!("Failed to write '{}': {e}", file_path.display()))?;

        let node_count = graph.node_count();
        for mapped_index in 0..node_count {
            let mapped_node_id = MappedNodeId::try_from(mapped_index)
                .map_err(|_| "Graph node count exceeds mapped ID space".to_string())?;
            let original_id = graph
                .to_original_node_id(mapped_node_id)
                .ok_or_else(|| format!("No original node ID for mapped node {mapped_node_id}"))?;
            let labels = graph
                .node_labels(mapped_node_id)
                .into_iter()
                .map(|l| l.name().replace('"', "\""))
                .collect::<Vec<_>>()
                .join(";");

            // CSV: quote labels to keep delimiter-stable.
            writeln!(file, "{},\"{}\"", original_id.get(), labels)
                .map_err(|e| format!("Failed to write '{}': {e}", file_path.display()))?;
        }

        u64::try_from(node_count).map_err(|_| "Graph node count exceeds export range".to_string())
    }

    fn write_relationships_csv(
        &self,
        graph_store: &DefaultGraphStore,
        export_root: &Path,
    ) -> Result<u64, String> {
        let mut total: u64 = 0;

        let rel_types = graph_store.relationship_types();
        for rel_type in rel_types.iter() {
            total += self.write_relationships_for_type(graph_store, rel_type, export_root)?;
        }

        Ok(total)
    }

    fn write_relationships_for_type(
        &self,
        graph_store: &DefaultGraphStore,
        rel_type: &RelationshipType,
        export_root: &Path,
    ) -> Result<u64, String> {
        let mut types = std::collections::HashSet::new();
        types.insert(rel_type.clone());
        let filtered = graph_store.get_graph_with_types(&types).map_err(|e| {
            format!(
                "Failed to construct graph view for relationship type '{}': {e}",
                rel_type.name()
            )
        })?;

        let safe_name = rel_type.name().replace('/', "_");
        let file_path = export_root.join(format!("relationships_{}.csv", safe_name));
        let mut file = fs::File::create(&file_path)
            .map_err(|e| format!("Failed to create '{}': {e}", file_path.display()))?;

        writeln!(file, "source,target")
            .map_err(|e| format!("Failed to write '{}': {e}", file_path.display()))?;

        let mut count: u64 = 0;
        let node_count = filtered.node_count();
        for mapped_index in 0..node_count {
            let mapped_source = MappedNodeId::try_from(mapped_index)
                .map_err(|_| "Graph node count exceeds mapped ID space".to_string())?;
            let source_original = filtered
                .to_original_node_id(mapped_source)
                .ok_or_else(|| format!("No original node ID for mapped node {mapped_source}"))?;

            for cursor in
                filtered.stream_relationships(mapped_source, filtered.default_property_value())
            {
                let mapped_target = cursor.target_id();
                let target_original =
                    filtered.to_original_node_id(mapped_target).ok_or_else(|| {
                        format!("No original node ID for mapped node {mapped_target}")
                    })?;
                writeln!(file, "{},{}", source_original.get(), target_original.get())
                    .map_err(|e| format!("Failed to write '{}': {e}", file_path.display()))?;
                count += 1;
            }
        }

        Ok(count)
    }
}
