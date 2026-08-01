use crate::types::graph_store::GraphStore;
use crate::types::schema::PropertySchemaTrait;
use crate::types::ValueType;
use std::collections::HashSet;
use thiserror::Error;

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum GraphStoreSchemaError {
    #[error("invalid graph schema: {0}")]
    InvalidSchema(String),

    #[error("{dimension} mismatch: declared {declared:?}, materialized {materialized:?}")]
    SetMismatch {
        dimension: String,
        declared: Vec<String>,
        materialized: Vec<String>,
    },

    #[error(
        "{dimension} property '{property}' has declared type {declared:?} but materialized type {materialized:?}"
    )]
    PropertyTypeMismatch {
        dimension: String,
        property: String,
        declared: ValueType,
        materialized: ValueType,
    },

    #[error(
        "{dimension} property '{property}' has {materialized} values but its owner requires {expected}"
    )]
    PropertyCardinalityMismatch {
        dimension: String,
        property: String,
        expected: usize,
        materialized: usize,
    },

    #[error("cannot inspect {dimension} property '{property}': {message}")]
    PropertyInspection {
        dimension: String,
        property: String,
        message: String,
    },
}

pub type GraphStoreSchemaResult<T> = Result<T, GraphStoreSchemaError>;

/// Verifies that a Store's observable state is exactly described by its manifest.
pub fn validate_graph_store_schema<S>(store: &S) -> GraphStoreSchemaResult<()>
where
    S: GraphStore,
{
    let schema = store.schema();
    schema
        .validate()
        .map_err(|error| GraphStoreSchemaError::InvalidSchema(error.to_string()))?;

    let declared_labels = schema
        .node_schema()
        .available_labels()
        .into_iter()
        .filter(|label| !label.is_all_nodes())
        .map(|label| label.name().to_string())
        .collect::<HashSet<_>>();
    let materialized_labels = store
        .node_labels()
        .into_iter()
        .map(|label| label.name().to_string())
        .collect::<HashSet<_>>();
    compare_sets("node labels", declared_labels, materialized_labels)?;

    compare_sets(
        "node properties",
        schema.node_schema().all_properties(),
        store.node_property_keys(),
    )?;
    for entry in schema.node_schema().entries() {
        if !entry.identifier().is_all_nodes() {
            compare_sets(
                &format!("node properties for label {}", entry.identifier()),
                entry.properties().keys().cloned().collect(),
                store.node_property_keys_for_label(entry.identifier()),
            )?;
        }
        for (key, property) in entry.properties() {
            let values = store.node_property_values(key).map_err(|error| {
                GraphStoreSchemaError::PropertyInspection {
                    dimension: format!("node label {}", entry.identifier()),
                    property: key.clone(),
                    message: error.to_string(),
                }
            })?;
            validate_property_type(
                &format!("node label {}", entry.identifier()),
                key,
                property.value_type(),
                values.value_type(),
            )?;
            validate_cardinality(
                &format!("node label {}", entry.identifier()),
                key,
                store.node_count(),
                values.element_count(),
            )?;
        }
    }

    compare_sets(
        "relationship types",
        schema
            .relationship_schema()
            .available_types()
            .into_iter()
            .map(|rel_type| rel_type.name().to_string())
            .collect(),
        store
            .relationship_types()
            .into_iter()
            .map(|rel_type| rel_type.name().to_string())
            .collect(),
    )?;
    for entry in schema.relationship_schema().entries() {
        let rel_type = entry.identifier();
        compare_sets(
            &format!("relationship properties for type {rel_type}"),
            entry.properties().keys().cloned().collect(),
            store.relationship_property_keys_for_type(rel_type),
        )?;
        for (key, property) in entry.properties() {
            let values = store
                .relationship_property_values(rel_type, key)
                .map_err(|error| GraphStoreSchemaError::PropertyInspection {
                    dimension: format!("relationship type {rel_type}"),
                    property: key.clone(),
                    message: error.to_string(),
                })?;
            validate_property_type(
                &format!("relationship type {rel_type}"),
                key,
                property.value_type(),
                values.value_type(),
            )?;
            validate_cardinality(
                &format!("relationship type {rel_type}"),
                key,
                store.relationship_count_for_type(rel_type),
                values.element_count(),
            )?;
        }
    }

    compare_sets(
        "graph properties",
        schema.graph_properties().keys().cloned().collect(),
        store.graph_property_keys(),
    )?;
    for (key, property) in schema.graph_properties() {
        let values = store.graph_property_values(key).map_err(|error| {
            GraphStoreSchemaError::PropertyInspection {
                dimension: "graph".to_string(),
                property: key.clone(),
                message: error.to_string(),
            }
        })?;
        validate_property_type("graph", key, property.value_type(), values.value_type())?;
    }

    Ok(())
}

fn compare_sets(
    dimension: &str,
    declared: HashSet<String>,
    materialized: HashSet<String>,
) -> GraphStoreSchemaResult<()> {
    if declared == materialized {
        return Ok(());
    }
    let mut declared = declared.into_iter().collect::<Vec<_>>();
    let mut materialized = materialized.into_iter().collect::<Vec<_>>();
    declared.sort();
    materialized.sort();
    Err(GraphStoreSchemaError::SetMismatch {
        dimension: dimension.to_string(),
        declared,
        materialized,
    })
}

fn validate_property_type(
    dimension: &str,
    property: &str,
    declared: ValueType,
    materialized: ValueType,
) -> GraphStoreSchemaResult<()> {
    if declared == materialized {
        return Ok(());
    }
    Err(GraphStoreSchemaError::PropertyTypeMismatch {
        dimension: dimension.to_string(),
        property: property.to_string(),
        declared,
        materialized,
    })
}

fn validate_cardinality(
    dimension: &str,
    property: &str,
    expected: usize,
    materialized: usize,
) -> GraphStoreSchemaResult<()> {
    if expected == materialized {
        return Ok(());
    }
    Err(GraphStoreSchemaError::PropertyCardinalityMismatch {
        dimension: dimension.to_string(),
        property: property.to_string(),
        expected,
        materialized,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GraphStoreConfig;
    use crate::projection::RelationshipType;
    use crate::types::graph::MappedNodeId;
    use crate::types::graph::RelationshipTopology;
    use crate::types::graph::SimpleIdMap;
    use crate::types::graph_store::Capabilities;
    use crate::types::graph_store::DatabaseId;
    use crate::types::graph_store::DatabaseInfo;
    use crate::types::graph_store::DatabaseLocation;
    use crate::types::graph_store::DefaultGraphStore;
    use crate::types::graph_store::GraphName;
    use crate::types::schema::GraphSchema;
    use std::collections::HashMap;

    #[test]
    fn rejects_materialized_relationship_type_absent_from_manifest() {
        let store = DefaultGraphStore::new(
            GraphStoreConfig::default(),
            GraphName::new("invalid"),
            DatabaseInfo::new(
                DatabaseId::new("db"),
                DatabaseLocation::remote("local", 0, None, None),
            ),
            GraphSchema::empty(),
            Capabilities::default(),
            SimpleIdMap::from_original_ids([0, 1]),
            HashMap::from([(
                RelationshipType::of("REL"),
                RelationshipTopology::new(vec![vec![MappedNodeId::new(1)], vec![]], None),
            )]),
        );

        assert!(matches!(
            validate_graph_store_schema(&store),
            Err(GraphStoreSchemaError::SetMismatch { dimension, .. })
                if dimension == "relationship types"
        ));
    }
}
