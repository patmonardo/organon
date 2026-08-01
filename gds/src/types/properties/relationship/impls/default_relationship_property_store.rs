use crate::collections::backends::factory::create_double_backend_from_config;
use crate::config::CollectionsConfig;
use crate::types::properties::relationship::relationship_property::RelationshipProperty;
use crate::types::properties::relationship::DefaultRelationshipPropertyValues;
use crate::types::properties::relationship::{
    relationship_property_store::{RelationshipPropertyStore, RelationshipPropertyStoreBuilder},
    relationship_property_values::RelationshipPropertyValues,
};
use crate::types::properties::PropertyStore;
use crate::types::properties::PropertyStoreError;
use crate::types::properties::PropertyStoreResult;
use std::collections::HashMap;
use std::sync::Arc;

/// Default implementation of RelationshipPropertyStore.
#[derive(Debug, Clone)]
pub struct DefaultRelationshipPropertyStore {
    properties: HashMap<String, RelationshipProperty>,
}

/// Builder for DefaultRelationshipPropertyStore.
#[derive(Debug, Clone)]
pub struct DefaultRelationshipPropertyStoreBuilder {
    properties: HashMap<String, RelationshipProperty>,
}

impl PropertyStore for DefaultRelationshipPropertyStore {
    type Property = RelationshipProperty;

    fn get(&self, property_key: &str) -> Option<&Self::Property> {
        self.properties.get(property_key)
    }

    fn columns(&self) -> Box<dyn Iterator<Item = &Self::Property> + '_> {
        Box::new(self.properties.values())
    }

    fn add_column(&mut self, property: Self::Property) -> PropertyStoreResult<()> {
        let key = property.key().to_string();
        if key.trim().is_empty() {
            return Err(PropertyStoreError::InvalidPropertyKey(key));
        }
        if self.properties.contains_key(&key) {
            return Err(PropertyStoreError::PropertyAlreadyExists(key));
        }
        self.properties.insert(key, property);
        Ok(())
    }

    fn replace_column(&mut self, property: Self::Property) -> PropertyStoreResult<Self::Property> {
        let key = property.key().to_string();
        if key.trim().is_empty() {
            return Err(PropertyStoreError::InvalidPropertyKey(key));
        }
        if !self.properties.contains_key(&key) {
            return Err(PropertyStoreError::PropertyNotFound(key));
        }
        Ok(self
            .properties
            .insert(key, property)
            .expect("column existence was checked"))
    }

    fn remove_column(&mut self, property_key: &str) -> PropertyStoreResult<Self::Property> {
        self.properties
            .remove(property_key)
            .ok_or_else(|| PropertyStoreError::PropertyNotFound(property_key.to_string()))
    }
}

/* Domain-specific RelationshipPropertyStore implementation */
impl RelationshipPropertyStore for DefaultRelationshipPropertyStore {
    type Builder = DefaultRelationshipPropertyStoreBuilder;

    fn empty() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }

    fn new(properties: HashMap<String, Self::Property>) -> Self {
        Self { properties }
    }

    fn builder() -> Self::Builder {
        DefaultRelationshipPropertyStoreBuilder::new()
    }

    fn get_all_properties(&self) -> Vec<&Self::Property> {
        self.properties.values().collect()
    }

    fn get_property_values(&self, property_key: &str) -> Option<&dyn RelationshipPropertyValues> {
        self.properties
            .get(property_key)
            .map(|property| property.values())
    }

    fn to_builder(&self) -> Self::Builder {
        DefaultRelationshipPropertyStoreBuilder::from_store(self)
    }
}

/* Builder trait implementation */
impl RelationshipPropertyStoreBuilder for DefaultRelationshipPropertyStoreBuilder {
    type Store = DefaultRelationshipPropertyStore;
    type Property = RelationshipProperty;

    fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }

    fn from_store(store: &Self::Store) -> Self {
        Self {
            properties: store.properties.clone(),
        }
    }

    fn properties(mut self, props: HashMap<String, Self::Property>) -> Self {
        self.properties = props;
        self
    }

    fn put_if_absent(mut self, property: Self::Property) -> Self {
        self.properties
            .entry(property.key().to_string())
            .or_insert(property);
        self
    }

    fn put(mut self, property: Self::Property) -> Self {
        self.properties.insert(property.key().to_string(), property);
        self
    }

    fn remove_property(mut self, key: &str) -> Self {
        self.properties.remove(key);
        self
    }

    fn build(self) -> Self::Store {
        DefaultRelationshipPropertyStore {
            properties: self.properties,
        }
    }
}

/* Inherent convenience methods for the store (ergonomics without trait import) */
impl DefaultRelationshipPropertyStore {
    /// Returns the number of properties in this store.
    pub fn len(&self) -> usize {
        self.properties.len()
    }

    /// Returns whether this store is empty.
    pub fn is_empty(&self) -> bool {
        self.properties.is_empty()
    }

    /// Returns a reference to the property with the given key.
    pub fn get(&self, key: &str) -> Option<&RelationshipProperty> {
        self.properties.get(key)
    }

    /// Returns whether the store contains a property with the given key.
    pub fn contains_key(&self, key: &str) -> bool {
        self.properties.contains_key(key)
    }

    /// Returns a reference to the underlying properties map.
    /// Note: In TypeScript, this is called `relationshipProperties()`.
    pub fn relationship_properties(&self) -> &HashMap<String, RelationshipProperty> {
        &self.properties
    }
}

/* Inherent convenience methods for the builder (do not belong to the trait) */
impl DefaultRelationshipPropertyStoreBuilder {
    /// Convenience method to add a property by supplying property values directly.
    /// Reduces imports for callers who just want to add a property by values.
    pub fn put_property(
        mut self,
        key: impl Into<String>,
        values: impl Into<Arc<dyn RelationshipPropertyValues>>,
    ) -> Self {
        let key_str = key.into();
        let values = values.into();
        use crate::types::PropertyState;

        let prop =
            RelationshipProperty::with_state(key_str.clone(), PropertyState::Persistent, values);
        self.properties.insert(key_str, prop);
        self
    }

    /// Create and put a Double relationship property using CollectionsConfig for backend selection.
    pub fn put_double_with_config(
        mut self,
        config: &CollectionsConfig<f64>,
        key: impl Into<String>,
        values: Vec<f64>,
        default_value: f64,
    ) -> Self {
        let element_count = values.len();

        // Use config to select backend
        let backend = create_double_backend_from_config(config, values);

        // Create property values with selected backend
        let pv: Arc<dyn RelationshipPropertyValues> =
            Arc::new(DefaultRelationshipPropertyValues::with_values(
                (0..backend.len()).filter_map(|i| backend.get(i)).collect(), // Convert backend to vec
                default_value,
                element_count,
            ));

        let key_str = key.into();
        use crate::types::PropertyState;
        let prop = RelationshipProperty::with_state(key_str.clone(), PropertyState::Persistent, pv);
        self.properties.insert(key_str, prop);
        self
    }

    /// Convenience: create and put a Double relationship property from Vec using Vec-backed defaults.
    pub fn put_double_from_vec(
        self,
        key: impl Into<String>,
        values: Vec<f64>,
        default_value: f64,
    ) -> Self {
        // Default to Vec backend
        let default_config = CollectionsConfig::<f64>::default();
        self.put_double_with_config(&default_config, key, values, default_value)
    }
}
