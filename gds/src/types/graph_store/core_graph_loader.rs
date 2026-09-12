//! Staged CoreGraphStore loading from Polars frames and persistent snapshots.

use std::collections::{BTreeMap, HashSet};

use crate::collections::backends::polars::{PersistentFrameError, PersistentPolarsFrame};
use crate::collections::dataframe::GDSDataFrame;
use crate::config::GraphStoreConfig;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::OriginalNodeId;
use crate::types::schema::GraphSchema;

use super::{
    Capabilities, CoreGraphStore, CoreGraphStoreError, DatabaseInfo, GraphSnapshotIdentity,
    TypedRelationshipInput,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreGraphLoadColumns {
    pub node_id: String,
    pub relationship_type: String,
    pub source: String,
    pub target: String,
}

impl CoreGraphLoadColumns {
    pub fn new(
        node_id: impl Into<String>,
        relationship_type: impl Into<String>,
        source: impl Into<String>,
        target: impl Into<String>,
    ) -> Self {
        Self {
            node_id: node_id.into(),
            relationship_type: relationship_type.into(),
            source: source.into(),
            target: target.into(),
        }
    }
}

impl Default for CoreGraphLoadColumns {
    fn default() -> Self {
        Self::new("node_id", "relationship_type", "source", "target")
    }
}

#[derive(Debug, Clone)]
pub struct CoreGraphLoadSpec {
    pub columns: CoreGraphLoadColumns,
    pub inverse_indexed_types: HashSet<RelationshipType>,
    pub require_declared_relationship_types: bool,
}

impl CoreGraphLoadSpec {
    pub fn new(columns: CoreGraphLoadColumns) -> Self {
        Self {
            columns,
            inverse_indexed_types: HashSet::new(),
            require_declared_relationship_types: true,
        }
    }

    pub fn with_inverse_indexed_types(
        mut self,
        relationship_types: HashSet<RelationshipType>,
    ) -> Self {
        self.inverse_indexed_types = relationship_types;
        self
    }

    pub fn allow_undeclared_relationship_types(mut self) -> Self {
        self.require_declared_relationship_types = false;
        self
    }
}

impl Default for CoreGraphLoadSpec {
    fn default() -> Self {
        Self::new(CoreGraphLoadColumns::default())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreGraphLoadReport {
    pub snapshot_id: String,
    pub snapshot_version: u64,
    pub node_rows: usize,
    pub relationship_rows: usize,
    pub relationships_by_type: BTreeMap<String, usize>,
    pub inverse_indexed_types: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct LoadedCoreGraphStore {
    store: CoreGraphStore,
    report: CoreGraphLoadReport,
}

impl LoadedCoreGraphStore {
    pub fn store(&self) -> &CoreGraphStore {
        &self.store
    }

    pub fn report(&self) -> &CoreGraphLoadReport {
        &self.report
    }

    pub fn into_parts(self) -> (CoreGraphStore, CoreGraphLoadReport) {
        (self.store, self.report)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CoreGraphLoadError {
    #[error("Polars graph load failed: {0}")]
    Polars(#[from] polars::error::PolarsError),
    #[error("persistent graph frame failed verification: {0}")]
    Persistent(#[from] PersistentFrameError),
    #[error("CoreGraphStore construction failed: {0}")]
    Store(#[from] CoreGraphStoreError),
    #[error("graph load column '{column}' contains null at row {row}")]
    NullValue { column: String, row: usize },
    #[error("relationship type '{0}' is absent from the graph schema")]
    UndeclaredRelationshipType(String),
    #[error(
        "persistent frame version mismatch for {artifact_id}: expected {expected}, found {actual}"
    )]
    SnapshotVersionMismatch {
        artifact_id: String,
        expected: u64,
        actual: u64,
    },
    #[error("invalid graph schema: {0}")]
    Schema(String),
}

#[derive(Debug, Default, Clone, Copy)]
pub struct CoreGraphLoader;

impl CoreGraphLoader {
    #[allow(clippy::too_many_arguments)]
    pub fn load_frames(
        config: GraphStoreConfig,
        identity: GraphSnapshotIdentity,
        database_info: DatabaseInfo,
        schema: GraphSchema,
        capabilities: Capabilities,
        node_frame: &GDSDataFrame,
        relationship_frame: &GDSDataFrame,
        spec: &CoreGraphLoadSpec,
    ) -> Result<LoadedCoreGraphStore, CoreGraphLoadError> {
        schema
            .validate()
            .map_err(|error| CoreGraphLoadError::Schema(error.to_string()))?;
        let node_ids = read_required_i64(node_frame, &spec.columns.node_id)?
            .into_iter()
            .map(OriginalNodeId::new)
            .collect::<Vec<_>>();
        let relationship_types =
            read_required_string(relationship_frame, &spec.columns.relationship_type)?;
        let sources = read_required_i64(relationship_frame, &spec.columns.source)?;
        let targets = read_required_i64(relationship_frame, &spec.columns.target)?;

        let declared_types = schema.relationship_schema().available_types();
        let mut relationships = Vec::with_capacity(relationship_types.len());
        let mut relationships_by_type = BTreeMap::new();
        for ((relationship_type, source), target) in
            relationship_types.into_iter().zip(sources).zip(targets)
        {
            let relationship_type = RelationshipType::of(relationship_type);
            if spec.require_declared_relationship_types
                && !declared_types.contains(&relationship_type)
            {
                return Err(CoreGraphLoadError::UndeclaredRelationshipType(
                    relationship_type.name().to_string(),
                ));
            }
            *relationships_by_type
                .entry(relationship_type.name().to_string())
                .or_insert(0) += 1;
            relationships.push(TypedRelationshipInput {
                relationship_type,
                source: OriginalNodeId::new(source),
                target: OriginalNodeId::new(target),
            });
        }

        let snapshot_id = identity.snapshot_id().to_string();
        let snapshot_version = identity.version();
        let store = CoreGraphStore::try_from_inputs(
            config,
            identity,
            database_info,
            schema,
            capabilities,
            node_ids,
            relationships,
            &spec.inverse_indexed_types,
        )?;
        let mut inverse_indexed_types = spec
            .inverse_indexed_types
            .iter()
            .map(|kind| kind.name().to_string())
            .collect::<Vec<_>>();
        inverse_indexed_types.sort();

        Ok(LoadedCoreGraphStore {
            report: CoreGraphLoadReport {
                snapshot_id,
                snapshot_version,
                node_rows: node_frame.height(),
                relationship_rows: relationship_frame.height(),
                relationships_by_type,
                inverse_indexed_types,
            },
            store,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn load_persistent(
        config: GraphStoreConfig,
        identity: GraphSnapshotIdentity,
        database_info: DatabaseInfo,
        schema: GraphSchema,
        capabilities: Capabilities,
        node_snapshot: &PersistentPolarsFrame,
        relationship_snapshot: &PersistentPolarsFrame,
        spec: &CoreGraphLoadSpec,
    ) -> Result<LoadedCoreGraphStore, CoreGraphLoadError> {
        validate_snapshot_version(node_snapshot, identity.version())?;
        validate_snapshot_version(relationship_snapshot, identity.version())?;
        node_snapshot.verify()?;
        relationship_snapshot.verify()?;
        let node_frame = node_snapshot.collect()?;
        let relationship_frame = relationship_snapshot.collect()?;
        Self::load_frames(
            config,
            identity,
            database_info,
            schema,
            capabilities,
            &node_frame,
            &relationship_frame,
            spec,
        )
    }
}

fn validate_snapshot_version(
    snapshot: &PersistentPolarsFrame,
    expected: u64,
) -> Result<(), CoreGraphLoadError> {
    let manifest = snapshot.manifest();
    if manifest.snapshot_version != expected {
        return Err(CoreGraphLoadError::SnapshotVersionMismatch {
            artifact_id: manifest.artifact_id.clone(),
            expected,
            actual: manifest.snapshot_version,
        });
    }
    Ok(())
}

fn read_required_i64(frame: &GDSDataFrame, column: &str) -> Result<Vec<i64>, CoreGraphLoadError> {
    frame
        .dataframe()
        .column(column)?
        .i64()?
        .into_iter()
        .enumerate()
        .map(|(row, value)| {
            value.ok_or_else(|| CoreGraphLoadError::NullValue {
                column: column.to_string(),
                row,
            })
        })
        .collect()
}

fn read_required_string(
    frame: &GDSDataFrame,
    column: &str,
) -> Result<Vec<String>, CoreGraphLoadError> {
    frame
        .dataframe()
        .column(column)?
        .str()?
        .into_iter()
        .enumerate()
        .map(|(row, value)| {
            value
                .map(str::to_string)
                .ok_or_else(|| CoreGraphLoadError::NullValue {
                    column: column.to_string(),
                    row,
                })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use polars::df;
    use uuid::Uuid;

    use super::*;
    use crate::collections::backends::polars::{PersistentPolarsStore, PolarsFrameBackend};
    use crate::types::graph_store::{DatabaseId, DatabaseLocation, GraphStoreRead};
    use crate::types::schema::{Direction, MutableGraphSchema};

    fn database_info() -> DatabaseInfo {
        DatabaseInfo::new(
            DatabaseId::new("loader-test"),
            DatabaseLocation::remote("localhost", 7687, None, None),
        )
    }

    fn schema() -> GraphSchema {
        let mut schema = MutableGraphSchema::empty();
        schema
            .relationship_schema_mut()
            .add_relationship_type(RelationshipType::of("CITES"), Direction::Directed);
        schema.build()
    }

    fn frames() -> (GDSDataFrame, GDSDataFrame) {
        (
            GDSDataFrame::new(df!("node_id" => [10_i64, 20, 30]).unwrap()),
            GDSDataFrame::new(
                df!(
                    "relationship_type" => ["CITES", "CITES"],
                    "source" => [10_i64, 30],
                    "target" => [20_i64, 20],
                )
                .unwrap(),
            ),
        )
    }

    #[test]
    fn loads_validated_core_graph_from_polars_frames() {
        let (nodes, relationships) = frames();
        let loaded = CoreGraphLoader::load_frames(
            GraphStoreConfig::default(),
            GraphSnapshotIdentity::new("knowledge", "load-4", 4),
            database_info(),
            schema(),
            Capabilities::new(),
            &nodes,
            &relationships,
            &CoreGraphLoadSpec::default()
                .with_inverse_indexed_types(HashSet::from([RelationshipType::of("CITES")])),
        )
        .unwrap();

        assert_eq!(loaded.store().node_count(), 3);
        assert_eq!(loaded.store().relationship_count(), 2);
        assert_eq!(loaded.report().relationships_by_type["CITES"], 2);
        assert_eq!(loaded.report().snapshot_version, 4);
    }

    #[test]
    fn loads_verified_persistent_polars_snapshots() {
        let root = std::env::temp_dir().join(format!("gds-graph-loader-{}", Uuid::new_v4()));
        let persistent = PersistentPolarsStore::new(&root);
        let (nodes, relationships) = frames();
        let nodes = persistent
            .publish("graph:nodes", 5, PolarsFrameBackend::from_dataframe(nodes))
            .unwrap();
        let relationships = persistent
            .publish(
                "graph:relationships",
                5,
                PolarsFrameBackend::from_dataframe(relationships),
            )
            .unwrap();

        let loaded = CoreGraphLoader::load_persistent(
            GraphStoreConfig::default(),
            GraphSnapshotIdentity::new("knowledge", "load-5", 5),
            database_info(),
            schema(),
            Capabilities::new(),
            &nodes,
            &relationships,
            &CoreGraphLoadSpec::default(),
        )
        .unwrap();
        assert_eq!(loaded.store().get_graph().relationship_count(), 2);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn rejects_relationship_type_absent_from_schema() {
        let (nodes, relationships) = frames();
        let error = CoreGraphLoader::load_frames(
            GraphStoreConfig::default(),
            GraphSnapshotIdentity::new("knowledge", "load-bad", 1),
            database_info(),
            GraphSchema::empty(),
            Capabilities::new(),
            &nodes,
            &relationships,
            &CoreGraphLoadSpec::default(),
        )
        .unwrap_err();
        assert!(matches!(
            error,
            CoreGraphLoadError::UndeclaredRelationshipType(ref kind) if kind == "CITES"
        ));
    }
}
