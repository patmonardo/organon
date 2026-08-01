use super::property::Property;
use crate::types::ValueType;
use thiserror::Error;

/// Error type for property store operations.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum PropertyStoreError {
    #[error("Property not found: {0}")]
    PropertyNotFound(String),

    #[error("Property already exists: {0}")]
    PropertyAlreadyExists(String),

    #[error("Invalid property key: {0}")]
    InvalidPropertyKey(String),

    #[error("Property '{key}' declares {declared:?} but its values are {materialized:?}")]
    SchemaValueTypeMismatch {
        key: String,
        declared: ValueType,
        materialized: ValueType,
    },
}

pub type PropertyStoreResult<T> = Result<T, PropertyStoreError>;

pub(crate) fn validate_column_value_type(
    key: &str,
    declared: ValueType,
    materialized: ValueType,
) -> PropertyStoreResult<()> {
    if declared == materialized {
        return Ok(());
    }
    Err(PropertyStoreError::SchemaValueTypeMismatch {
        key: key.to_string(),
        declared,
        materialized,
    })
}

/// A framing protocol for schema-bearing property columns.
pub trait PropertyStore: Send + Sync {
    type Property: Property;

    /// Gets a column by its schema key.
    fn get(&self, property_key: &str) -> Option<&Self::Property>;

    /// Iterates over the columns without exposing their storage representation.
    fn columns(&self) -> Box<dyn Iterator<Item = &Self::Property> + '_>;

    /// Adds a new column, rejecting duplicate schema keys.
    fn add_column(&mut self, property: Self::Property) -> PropertyStoreResult<()>;

    /// Replaces an existing column with the same schema key.
    fn replace_column(&mut self, property: Self::Property) -> PropertyStoreResult<Self::Property>;

    /// Removes and returns a column by schema key.
    fn remove_column(&mut self, property_key: &str) -> PropertyStoreResult<Self::Property>;

    /// Checks if the property store is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the number of properties in the store.
    fn len(&self) -> usize {
        self.columns().count()
    }

    /// Returns the set of property keys in this store.
    fn key_set(&self) -> Vec<&str> {
        self.columns()
            .map(|property| property.schema().key())
            .collect()
    }

    /// Checks if the store contains a property with the given key.
    fn contains_key(&self, property_key: &str) -> bool {
        self.get(property_key).is_some()
    }
}
