//! ExecutableNodePropertyStep trait
//!

use crate::types::graph_store::DefaultGraphStore;
use dyn_clone::DynClone;
use std::collections::HashMap;
use std::error::Error as StdError;

/// Executable node property step.
///
pub trait ExecutableNodePropertyStep: DynClone + Send + Sync {
    /// Execute the algorithm and mutate graph store with computed property.
    ///
    fn execute(
        &self,
        graph_store: &mut DefaultGraphStore,
        node_labels: &[String],
        relationship_types: &[String],
        concurrency: usize,
    ) -> Result<(), Box<dyn StdError + Send + Sync>>;

    /// Configuration for this step.
    ///
    fn config(&self) -> &HashMap<String, serde_json::Value>;

    /// Context node labels (additional labels beyond train/test).
    ///
    fn context_node_labels(&self) -> &[String] {
        &[]
    }

    /// Context relationship types (additional types beyond train/test).
    ///
    fn context_relationship_types(&self) -> &[String] {
        &[]
    }

    /// Procedure name (e.g., "gds.pageRank.mutate").
    ///
    fn proc_name(&self) -> &str;

    /// Root task name for progress tracking.
    ///
    fn root_task_name(&self) -> &str {
        self.proc_name()
    }

    /// The property name that will be mutated.
    ///
    fn mutate_node_property(&self) -> &str;

    /// Convert to map for serialization (ToMapConvertible).
    ///
    fn to_map(&self) -> HashMap<String, serde_json::Value> {
        let mut map = HashMap::new();
        map.insert("procName".to_string(), serde_json::json!(self.proc_name()));
        map.insert(
            "mutateProperty".to_string(),
            serde_json::json!(self.mutate_node_property()),
        );
        map.insert("config".to_string(), serde_json::json!(self.config()));
        map
    }
}

dyn_clone::clone_trait_object!(ExecutableNodePropertyStep);
