use crate::types::properties::property_store::validate_column_value_type;
use crate::types::properties::relationship::RelationshipPropertyValues;
use crate::types::properties::Property;
use crate::types::properties::PropertyStoreResult;
use crate::types::properties::PropertyValues;
use crate::types::schema::Aggregation;
use crate::types::schema::PropertySchema;
use crate::types::schema::PropertySchemaTrait;
use crate::types::schema::RelationshipPropertySchema;
use crate::types::DefaultValue;
use crate::types::PropertyState;
use std::sync::Arc;

/// Concrete relationship property implementation that mirrors the structure of
/// node and graph property defaults. It stores the shared property values
/// alongside the computed schema metadata.
#[derive(Debug, Clone)]
pub struct DefaultRelationshipProperty {
    values: Arc<dyn RelationshipPropertyValues>,
    schema: RelationshipPropertySchema,
}

impl DefaultRelationshipProperty {
    /// Construct a relationship property using the default `PropertyState::Persistent`.
    pub fn of(key: impl Into<String>, values: Arc<dyn RelationshipPropertyValues>) -> Self {
        Self::with_state(key, PropertyState::Persistent, values)
    }

    /// Construct a property with an explicit property state.
    pub fn with_state(
        key: impl Into<String>,
        state: PropertyState,
        values: Arc<dyn RelationshipPropertyValues>,
    ) -> Self {
        let key_str = key.into();
        let value_type = values.value_type();
        let default_value = DefaultValue::of(value_type);
        Self::from_validated_schema(
            RelationshipPropertySchema::new(
                PropertySchema::new(key_str, value_type, default_value, state),
                Aggregation::None,
            ),
            values,
        )
    }

    /// Construct a property with a provided default value in addition to the state.
    pub fn with_default(
        key: impl Into<String>,
        state: PropertyState,
        values: Arc<dyn RelationshipPropertyValues>,
        default_value: DefaultValue,
    ) -> Self {
        let key_str = key.into();
        let value_type = values.value_type();
        Self::from_validated_schema(
            RelationshipPropertySchema::new(
                PropertySchema::new(key_str, value_type, default_value, state),
                Aggregation::None,
            ),
            values,
        )
    }

    pub fn with_aggregation(
        key: impl Into<String>,
        state: PropertyState,
        values: Arc<dyn RelationshipPropertyValues>,
        default_value: DefaultValue,
        aggregation: Aggregation,
    ) -> Self {
        let schema = RelationshipPropertySchema::with_aggregation(
            key,
            values.value_type(),
            default_value,
            state,
            aggregation,
        );
        Self::from_validated_schema(schema, values)
    }

    /// Construct a property from an existing schema after validating its values.
    pub fn try_with_schema(
        schema: RelationshipPropertySchema,
        values: Arc<dyn RelationshipPropertyValues>,
    ) -> PropertyStoreResult<Self> {
        validate_column_value_type(schema.key(), schema.value_type(), values.value_type())?;
        Ok(Self { values, schema })
    }

    pub(crate) fn with_schema(
        schema: RelationshipPropertySchema,
        values: Arc<dyn RelationshipPropertyValues>,
    ) -> Self {
        Self::try_with_schema(schema, values)
            .expect("relationship property schema value type must match its values")
    }

    fn from_validated_schema(
        schema: RelationshipPropertySchema,
        values: Arc<dyn RelationshipPropertyValues>,
    ) -> Self {
        Self { values, schema }
    }

    /// Returns the property values as a shared trait object.
    pub fn values(&self) -> &dyn RelationshipPropertyValues {
        self.values.as_ref()
    }

    /// Returns a cloned `Arc` handle to the underlying property values.
    pub fn values_arc(&self) -> Arc<dyn RelationshipPropertyValues> {
        Arc::clone(&self.values)
    }

    /// Returns the relationship property schema.
    pub fn property_schema(&self) -> &RelationshipPropertySchema {
        &self.schema
    }

    /// Convenience accessor for the property key.
    pub fn key(&self) -> &str {
        self.schema.key()
    }
}

impl Property for DefaultRelationshipProperty {
    fn values(&self) -> Arc<dyn PropertyValues> {
        Arc::clone(&self.values) as Arc<dyn PropertyValues>
    }

    fn schema(&self) -> &PropertySchema {
        self.schema.base_schema()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::properties::relationship::DefaultRelationshipPropertyValues;
    use crate::types::properties::PropertyValues;
    use crate::types::ValueType;

    #[test]
    fn default_relationship_property_creation() {
        let values: Arc<dyn RelationshipPropertyValues> = Arc::new(
            DefaultRelationshipPropertyValues::with_values(vec![1.0, 2.5, 3.7], 0.0, 3),
        );
        let property = DefaultRelationshipProperty::of("weight", values.clone());

        assert_eq!(property.key(), "weight");
        assert_eq!(
            property.property_schema().state(),
            PropertyState::Persistent
        );
        assert_eq!(property.property_schema().value_type(), ValueType::Double);
        assert!(Arc::ptr_eq(&property.values_arc(), &values));
    }

    #[test]
    fn relationship_property_with_state() {
        let values: Arc<dyn RelationshipPropertyValues> = Arc::new(
            DefaultRelationshipPropertyValues::with_values(vec![10.0, 20.0], 0.0, 2),
        );
        let property =
            DefaultRelationshipProperty::with_state("distance", PropertyState::Transient, values);

        assert_eq!(property.key(), "distance");
        assert_eq!(property.property_schema().state(), PropertyState::Transient);
    }

    #[test]
    fn relationship_property_with_explicit_default() {
        let values: Arc<dyn RelationshipPropertyValues> = Arc::new(
            DefaultRelationshipPropertyValues::with_values(vec![5.5, 6.6], 1.0, 2),
        );
        let default_value = DefaultValue::double(1.0); // ← CLEAN API!
        let property = DefaultRelationshipProperty::with_default(
            "score",
            PropertyState::Persistent,
            values,
            default_value.clone(),
        );

        assert_eq!(property.key(), "score");
        assert_eq!(property.property_schema().default_value(), &default_value);
    }

    #[test]
    fn relationship_property_preserves_aggregation() {
        let values: Arc<dyn RelationshipPropertyValues> = Arc::new(
            DefaultRelationshipPropertyValues::with_values(vec![5.5, 6.6], 1.0, 2),
        );
        let property = DefaultRelationshipProperty::with_aggregation(
            "score",
            PropertyState::Persistent,
            values,
            DefaultValue::double(1.0),
            Aggregation::Max,
        );

        assert_eq!(property.property_schema().aggregation(), Aggregation::Max);
    }

    #[test]
    fn relationship_property_values_access() {
        let values: Arc<dyn RelationshipPropertyValues> = Arc::new(
            DefaultRelationshipPropertyValues::with_values(vec![1.0, 2.0, 3.0], 0.0, 3),
        );
        let property = DefaultRelationshipProperty::of("weight", values);

        // Test trait object access
        let values_ref = property.values();
        assert_eq!(values_ref.element_count(), 3);

        // Test Arc access
        let values_arc = property.values_arc();
        assert_eq!(values_arc.element_count(), 3);
    }

    #[test]
    fn relationship_property_schema_defaults() {
        let values: Arc<dyn RelationshipPropertyValues> = Arc::new(
            DefaultRelationshipPropertyValues::with_values(vec![1.0, 2.0], 0.0, 2),
        );
        let property = DefaultRelationshipProperty::of("test", values);

        // Should use system default for Double type
        let schema = property.property_schema();
        assert!(schema.default_value().double_value().unwrap().is_nan());
    }

    #[test]
    fn explicit_schema_rejects_mismatched_values() {
        let values: Arc<dyn RelationshipPropertyValues> = Arc::new(
            DefaultRelationshipPropertyValues::with_values(vec![1.0], 0.0, 1),
        );
        let schema = RelationshipPropertySchema::of("count", ValueType::Long);

        assert!(matches!(
            DefaultRelationshipProperty::try_with_schema(schema, values),
            Err(crate::types::properties::PropertyStoreError::SchemaValueTypeMismatch { .. })
        ));
    }
}
