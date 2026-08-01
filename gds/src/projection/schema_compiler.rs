use crate::projection::AbstractProjections;
use crate::projection::Aggregation as ProjectionAggregation;
use crate::projection::NodeProjection;
use crate::projection::NodeProjections;
use crate::projection::Orientation;
use crate::projection::PropertyMapping;
use crate::projection::RelationshipProjection;
use crate::projection::RelationshipProjections;
use crate::types::schema::Aggregation as SchemaAggregation;
use crate::types::schema::Direction;
use crate::types::schema::GraphSchema;
use crate::types::schema::MutableNodeSchema;
use crate::types::schema::MutableRelationshipSchema;
use crate::types::schema::NodeSchemaEntry;
use crate::types::schema::PropertySchema;
use crate::types::schema::PropertySchemaTrait;
use crate::types::schema::RelationshipPropertySchema;
use crate::types::schema::RelationshipSchemaEntry;
use crate::types::PropertyState;
use thiserror::Error;

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum ProjectionSchemaError {
    #[error("node projection key '{key}' does not match embedded label '{embedded}'")]
    NodeProjectionKeyMismatch { key: String, embedded: String },

    #[error("relationship projection key '{key}' does not match embedded type '{embedded}'")]
    RelationshipProjectionKeyMismatch { key: String, embedded: String },

    #[error("node projection refers to source label '{0}' that is not declared")]
    NodeLabelNotFound(String),

    #[error("relationship projection refers to source type '{0}' that is not declared")]
    RelationshipTypeNotFound(String),

    #[error("node property '{property}' is not declared for source label '{owner}'")]
    NodePropertyNotFound { owner: String, property: String },

    #[error("relationship property '{property}' is not declared for source type '{owner}'")]
    RelationshipPropertyNotFound { owner: String, property: String },

    #[error("a wildcard {dimension} projection cannot be combined with explicit projections")]
    WildcardMixedWithExplicit { dimension: &'static str },
}

pub type ProjectionSchemaResult<T> = Result<T, ProjectionSchemaError>;

/// Compiles projection requests against source facts into a materialized Store manifest.
pub struct ProjectionSchemaCompiler;

impl ProjectionSchemaCompiler {
    pub fn compile(
        source: &GraphSchema,
        node_projections: &NodeProjections,
        relationship_projections: &RelationshipProjections,
    ) -> ProjectionSchemaResult<GraphSchema> {
        let node_schema = compile_nodes(source, node_projections)?;
        let relationship_schema = compile_relationships(source, relationship_projections)?;

        Ok(GraphSchema::new(
            node_schema.build(),
            relationship_schema.build(),
            source.graph_properties().clone(),
        ))
    }
}

fn compile_nodes(
    source: &GraphSchema,
    projections: &NodeProjections,
) -> ProjectionSchemaResult<MutableNodeSchema> {
    validate_node_projection_keys(projections)?;
    let mut output = MutableNodeSchema::empty();

    if let Some(wildcard) = wildcard_node_projection(projections)? {
        for source_entry in source.node_schema().entries() {
            compile_node_entry(&mut output, source_entry, wildcard)?;
        }
        return Ok(output);
    }

    for (label, projection) in projections.projections() {
        let source_entry = source
            .node_schema()
            .get(label)
            .ok_or_else(|| ProjectionSchemaError::NodeLabelNotFound(label.name().to_string()))?;
        compile_node_entry(&mut output, source_entry, projection)?;
    }
    Ok(output)
}

fn compile_node_entry(
    output: &mut MutableNodeSchema,
    source: &NodeSchemaEntry,
    projection: &NodeProjection,
) -> ProjectionSchemaResult<()> {
    let entry = output.get_or_create_label(source.identifier().clone());
    if projection.project_all() && projection.properties().is_empty() {
        for property in source.properties().values() {
            entry.add_property_schema(property.clone());
        }
        return Ok(());
    }

    for mapping in projection.properties() {
        let property = source
            .properties()
            .get(mapping.neo_property_key())
            .ok_or_else(|| ProjectionSchemaError::NodePropertyNotFound {
                owner: source.identifier().name().to_string(),
                property: mapping.neo_property_key().to_string(),
            })?;
        entry.add_property_schema(mapped_node_property(mapping, property));
    }
    Ok(())
}

fn mapped_node_property(mapping: &PropertyMapping, source: &PropertySchema) -> PropertySchema {
    PropertySchema::with_defaults(
        mapping.property_key(),
        source.value_type(),
        resolved_default(mapping, source),
        PropertyState::Persistent,
    )
}

fn compile_relationships(
    source: &GraphSchema,
    projections: &RelationshipProjections,
) -> ProjectionSchemaResult<MutableRelationshipSchema> {
    validate_relationship_projection_keys(projections)?;
    let mut output = MutableRelationshipSchema::empty();

    if let Some(wildcard) = wildcard_relationship_projection(projections)? {
        for source_entry in source.relationship_schema().entries() {
            compile_relationship_entry(&mut output, source_entry, wildcard)?;
        }
        return Ok(output);
    }

    for (rel_type, projection) in projections.projections() {
        let source_entry = source.relationship_schema().get(rel_type).ok_or_else(|| {
            ProjectionSchemaError::RelationshipTypeNotFound(rel_type.name().to_string())
        })?;
        compile_relationship_entry(&mut output, source_entry, projection)?;
    }
    Ok(output)
}

fn compile_relationship_entry(
    output: &mut MutableRelationshipSchema,
    source: &RelationshipSchemaEntry,
    projection: &RelationshipProjection,
) -> ProjectionSchemaResult<()> {
    let entry = output.get_or_create_type(
        source.identifier().clone(),
        direction_for(projection.orientation()),
    );
    if projection.project_all() && projection.properties().is_empty() {
        for property in source.properties().values() {
            entry.add_property_schema(mapped_relationship_property(
                property.key(),
                property,
                projection.aggregation(),
            ));
        }
        return Ok(());
    }

    for mapping in projection.properties() {
        let property = source
            .properties()
            .get(mapping.neo_property_key())
            .ok_or_else(|| ProjectionSchemaError::RelationshipPropertyNotFound {
                owner: source.identifier().name().to_string(),
                property: mapping.neo_property_key().to_string(),
            })?;
        let aggregation = mapping.aggregation().resolve(projection.aggregation());
        entry.add_property_schema(RelationshipPropertySchema::with_aggregation(
            mapping.property_key(),
            property.value_type(),
            resolved_default(mapping, property),
            PropertyState::Persistent,
            resolved_aggregation(aggregation, property.aggregation()),
        ));
    }
    Ok(())
}

fn mapped_relationship_property(
    key: &str,
    source: &RelationshipPropertySchema,
    aggregation: ProjectionAggregation,
) -> RelationshipPropertySchema {
    RelationshipPropertySchema::with_aggregation(
        key,
        source.value_type(),
        source.default_value().clone(),
        PropertyState::Persistent,
        resolved_aggregation(aggregation, source.aggregation()),
    )
}

fn resolved_default<T>(mapping: &PropertyMapping, source: &T) -> crate::types::DefaultValue
where
    T: PropertySchemaTrait,
{
    if mapping.default_value().is_null_value() {
        source.default_value().clone()
    } else {
        mapping.default_value().clone()
    }
}

fn resolved_aggregation(
    requested: ProjectionAggregation,
    source: SchemaAggregation,
) -> SchemaAggregation {
    match requested {
        ProjectionAggregation::Default => SchemaAggregation::resolve(source),
        ProjectionAggregation::None => SchemaAggregation::None,
        ProjectionAggregation::Min => SchemaAggregation::Min,
        ProjectionAggregation::Max => SchemaAggregation::Max,
        ProjectionAggregation::Sum => SchemaAggregation::Sum,
        ProjectionAggregation::Count => SchemaAggregation::Count,
        ProjectionAggregation::Single => SchemaAggregation::Single,
    }
}

fn direction_for(orientation: Orientation) -> Direction {
    match orientation {
        Orientation::Natural | Orientation::Reverse => Direction::Directed,
        Orientation::Undirected => Direction::Undirected,
    }
}

fn validate_node_projection_keys(projections: &NodeProjections) -> ProjectionSchemaResult<()> {
    for (key, projection) in projections.projections() {
        if key != projection.label() {
            return Err(ProjectionSchemaError::NodeProjectionKeyMismatch {
                key: key.name().to_string(),
                embedded: projection.label().name().to_string(),
            });
        }
    }
    Ok(())
}

fn validate_relationship_projection_keys(
    projections: &RelationshipProjections,
) -> ProjectionSchemaResult<()> {
    for (key, projection) in projections.projections() {
        if key != projection.rel_type() {
            return Err(ProjectionSchemaError::RelationshipProjectionKeyMismatch {
                key: key.name().to_string(),
                embedded: projection.rel_type().name().to_string(),
            });
        }
    }
    Ok(())
}

fn wildcard_node_projection(
    projections: &NodeProjections,
) -> ProjectionSchemaResult<Option<&NodeProjection>> {
    let wildcard = projections
        .projections()
        .values()
        .find(|projection| projection.project_all());
    if wildcard.is_some() && projections.size() != 1 {
        return Err(ProjectionSchemaError::WildcardMixedWithExplicit { dimension: "node" });
    }
    Ok(wildcard.map(AsRef::as_ref))
}

fn wildcard_relationship_projection(
    projections: &RelationshipProjections,
) -> ProjectionSchemaResult<Option<&RelationshipProjection>> {
    let wildcard = projections
        .projections()
        .values()
        .find(|projection| projection.project_all());
    if wildcard.is_some() && projections.size() != 1 {
        return Err(ProjectionSchemaError::WildcardMixedWithExplicit {
            dimension: "relationship",
        });
    }
    Ok(wildcard.map(AsRef::as_ref))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::PropertyMapping;
    use crate::projection::PropertyMappings;
    use crate::types::schema::MutableGraphSchema;
    use crate::types::schema::NodeLabel;
    use crate::types::schema::RelationshipType;
    use crate::types::DefaultValue;
    use crate::types::ValueType;
    use std::sync::Arc;

    fn source_schema() -> GraphSchema {
        let person = NodeLabel::of("Person");
        let knows = RelationshipType::of("KNOWS");
        let mut schema = MutableGraphSchema::empty();
        schema
            .node_schema_mut()
            .add_property(person.clone(), "name", ValueType::String)
            .add_property(person, "age", ValueType::Long);
        schema
            .relationship_schema_mut()
            .get_or_create_type(knows, Direction::Directed)
            .add_property_schema(RelationshipPropertySchema::with_aggregation(
                "weight",
                ValueType::Double,
                DefaultValue::double(0.0),
                PropertyState::Persistent,
                SchemaAggregation::Sum,
            ));
        schema.put_graph_property(
            "node_count",
            PropertySchema::of("node_count", ValueType::Long),
        );
        schema.build()
    }

    #[test]
    fn compiles_aliases_direction_and_aggregation() {
        let person = NodeLabel::of("Person");
        let knows = RelationshipType::of("KNOWS");
        let node_mapping = PropertyMapping::with_source("years", "age").unwrap();
        let relationship_mapping = PropertyMapping::with_source("strength", "weight").unwrap();
        let nodes = NodeProjections::builder()
            .add(
                person.clone(),
                Arc::new(NodeProjection::new(
                    person.clone(),
                    PropertyMappings::of(vec![node_mapping]),
                )),
            )
            .build();
        let relationships = RelationshipProjections::builder()
            .add(
                knows.clone(),
                Arc::new(RelationshipProjection::new(
                    knows.clone(),
                    Orientation::Undirected,
                    ProjectionAggregation::Max,
                    false,
                    PropertyMappings::of(vec![relationship_mapping]),
                )),
            )
            .build();

        let compiled =
            ProjectionSchemaCompiler::compile(&source_schema(), &nodes, &relationships).unwrap();

        let person_schema = compiled.node_schema().get(&person).unwrap();
        assert_eq!(
            person_schema.properties()["years"].value_type(),
            ValueType::Long
        );
        let knows_schema = compiled.relationship_schema().get(&knows).unwrap();
        assert_eq!(knows_schema.direction(), Direction::Undirected);
        assert_eq!(
            knows_schema.properties()["strength"].aggregation(),
            SchemaAggregation::Max
        );
        assert!(compiled.graph_properties().contains_key("node_count"));
        assert!(compiled.validate().is_ok());
    }

    #[test]
    fn wildcard_projection_expands_source_owners_and_properties() {
        let nodes = NodeProjections::builder()
            .add(NodeLabel::of("*"), Arc::new(NodeProjection::all()))
            .build();
        let relationships = RelationshipProjections::builder()
            .add(
                RelationshipType::of("*"),
                Arc::new(RelationshipProjection::all_undirected()),
            )
            .build();

        let compiled =
            ProjectionSchemaCompiler::compile(&source_schema(), &nodes, &relationships).unwrap();

        assert_eq!(compiled.node_schema().all_properties().len(), 2);
        assert_eq!(compiled.direction(), Some(Direction::Undirected));
        assert_eq!(compiled.relationship_schema().all_properties().len(), 1);
    }

    #[test]
    fn rejects_collection_key_that_disagrees_with_projection() {
        let nodes = NodeProjections::builder()
            .add(
                NodeLabel::of("Wrong"),
                Arc::new(NodeProjection::of(NodeLabel::of("Person"))),
            )
            .build();

        let error = ProjectionSchemaCompiler::compile(
            &source_schema(),
            &nodes,
            &RelationshipProjections::empty(),
        )
        .unwrap_err();

        assert!(matches!(
            error,
            ProjectionSchemaError::NodeProjectionKeyMismatch { .. }
        ));
    }
}
