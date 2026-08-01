use super::Aggregation;
use super::SchemaError;
use super::SchemaResult;
use crate::types::DefaultValue;
use crate::types::PropertyState;
use crate::types::ValueType;
use serde::{Deserialize, Serialize};

/// Trait for property schemas (node or relationship properties).
pub trait PropertySchemaTrait: Send + Sync {
    fn key(&self) -> &str;
    fn value_type(&self) -> ValueType;
    fn default_value(&self) -> &DefaultValue;
    fn state(&self) -> PropertyState;
}

/// Schema describing a node property.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PropertySchema {
    key: String,
    value_type: ValueType,
    default_value: DefaultValue,
    state: PropertyState,
}

impl PropertySchema {
    /// Creates a new property schema.
    pub fn new(
        key: impl Into<String>,
        value_type: ValueType,
        default_value: DefaultValue,
        state: PropertyState,
    ) -> Self {
        Self {
            key: key.into(),
            value_type,
            default_value,
            state,
        }
    }

    /// Creates a property schema with a default value inferred from the value type.
    pub fn of(key: impl Into<String>, value_type: ValueType) -> Self {
        Self::new(
            key,
            value_type,
            DefaultValue::of(value_type),
            PropertyState::Persistent,
        )
    }

    /// Creates a property schema with explicit default value and state.
    pub fn with_defaults(
        key: impl Into<String>,
        value_type: ValueType,
        default_value: DefaultValue,
        state: PropertyState,
    ) -> Self {
        Self::new(key, value_type, default_value, state)
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value_type(&self) -> ValueType {
        self.value_type
    }

    pub fn default_value(&self) -> &DefaultValue {
        &self.default_value
    }

    pub fn state(&self) -> PropertyState {
        self.state // PropertyState is Copy, no need to clone
    }

    pub fn ensure_compatible(&self, other: &Self) -> SchemaResult<()> {
        if self.key != other.key {
            return Err(SchemaError::IdentifierMismatch {
                left: self.key.clone(),
                right: other.key.clone(),
            });
        }
        if self.value_type != other.value_type {
            return Err(SchemaError::PropertyTypeConflict {
                key: self.key.clone(),
                left: self.value_type,
                right: other.value_type,
            });
        }
        if self.default_value != other.default_value {
            return Err(SchemaError::PropertyDefaultConflict {
                key: self.key.clone(),
            });
        }
        if self.state != other.state {
            return Err(SchemaError::PropertyStateConflict {
                key: self.key.clone(),
                left: self.state,
                right: other.state,
            });
        }
        Ok(())
    }
}

impl PropertySchemaTrait for PropertySchema {
    fn key(&self) -> &str {
        &self.key
    }

    fn value_type(&self) -> ValueType {
        self.value_type
    }

    fn default_value(&self) -> &DefaultValue {
        &self.default_value
    }

    fn state(&self) -> PropertyState {
        self.state // PropertyState is Copy, no need to clone
    }
}

/// Schema describing a relationship property (extends PropertySchema with aggregation).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationshipPropertySchema {
    base: PropertySchema,
    aggregation: Aggregation,
}

impl RelationshipPropertySchema {
    pub fn new(base: PropertySchema, aggregation: Aggregation) -> Self {
        Self { base, aggregation }
    }

    pub fn base_schema(&self) -> &PropertySchema {
        &self.base
    }

    /// Creates a relationship property schema with default settings.
    pub fn of(key: impl Into<String>, value_type: ValueType) -> Self {
        Self {
            base: PropertySchema::of(key, value_type),
            aggregation: Aggregation::None,
        }
    }

    /// Creates a relationship property schema with explicit parameters.
    pub fn with_aggregation(
        key: impl Into<String>,
        value_type: ValueType,
        default_value: DefaultValue,
        state: PropertyState,
        aggregation: Aggregation,
    ) -> Self {
        Self {
            base: PropertySchema::new(key, value_type, default_value, state),
            aggregation,
        }
    }

    pub fn aggregation(&self) -> Aggregation {
        self.aggregation
    }

    pub fn ensure_compatible(&self, other: &Self) -> SchemaResult<()> {
        self.base.ensure_compatible(&other.base)?;
        if self.aggregation != other.aggregation {
            return Err(SchemaError::PropertyAggregationConflict {
                key: self.key().to_string(),
                left: self.aggregation,
                right: other.aggregation,
            });
        }
        Ok(())
    }

    /// Returns a normalized version where DEFAULT aggregation is resolved.
    pub fn normalize(&self) -> Self {
        if self.aggregation == Aggregation::Default {
            Self {
                base: self.base.clone(),
                aggregation: Aggregation::resolve(self.aggregation),
            }
        } else {
            self.clone()
        }
    }
}

impl PropertySchemaTrait for RelationshipPropertySchema {
    fn key(&self) -> &str {
        self.base.key()
    }

    fn value_type(&self) -> ValueType {
        self.base.value_type()
    }

    fn default_value(&self) -> &DefaultValue {
        self.base.default_value()
    }

    fn state(&self) -> PropertyState {
        self.base.state()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_property_schema_creation() {
        let schema = PropertySchema::of("name", ValueType::String);
        assert_eq!(schema.key(), "name");
        assert_eq!(schema.value_type(), ValueType::String);
        assert_eq!(schema.default_value(), &DefaultValue::of(ValueType::String));
    }

    #[test]
    fn test_relationship_property_schema() {
        let schema = RelationshipPropertySchema::of("weight", ValueType::Double);
        assert_eq!(schema.key(), "weight");
        assert_eq!(schema.value_type(), ValueType::Double);
        assert_eq!(schema.aggregation(), Aggregation::None);
    }

    #[test]
    fn test_aggregation_resolve() {
        assert_eq!(
            Aggregation::resolve(Aggregation::Default),
            Aggregation::None
        );
        assert_eq!(Aggregation::resolve(Aggregation::Sum), Aggregation::Sum);
    }

    #[test]
    fn test_normalize() {
        let schema = RelationshipPropertySchema::with_aggregation(
            "weight",
            ValueType::Double,
            DefaultValue::double(0.0), // Use the clean API
            PropertyState::Persistent,
            Aggregation::Default,
        );

        let normalized = schema.normalize();
        assert_eq!(normalized.aggregation(), Aggregation::None);
    }

    #[test]
    fn compatibility_includes_default_and_state() {
        let persistent = PropertySchema::with_defaults(
            "score",
            ValueType::Double,
            DefaultValue::double(0.0),
            PropertyState::Persistent,
        );
        let different_default = PropertySchema::with_defaults(
            "score",
            ValueType::Double,
            DefaultValue::double(1.0),
            PropertyState::Persistent,
        );
        let transient = PropertySchema::with_defaults(
            "score",
            ValueType::Double,
            DefaultValue::double(0.0),
            PropertyState::Transient,
        );

        assert!(matches!(
            persistent.ensure_compatible(&different_default),
            Err(SchemaError::PropertyDefaultConflict { .. })
        ));
        assert!(matches!(
            persistent.ensure_compatible(&transient),
            Err(SchemaError::PropertyStateConflict { .. })
        ));
    }

    #[test]
    fn relationship_compatibility_includes_aggregation() {
        let sum = RelationshipPropertySchema::with_aggregation(
            "weight",
            ValueType::Double,
            DefaultValue::double(0.0),
            PropertyState::Persistent,
            Aggregation::Sum,
        );
        let max = RelationshipPropertySchema::with_aggregation(
            "weight",
            ValueType::Double,
            DefaultValue::double(0.0),
            PropertyState::Persistent,
            Aggregation::Max,
        );

        assert!(matches!(
            sum.ensure_compatible(&max),
            Err(SchemaError::PropertyAggregationConflict { .. })
        ));
    }
}
