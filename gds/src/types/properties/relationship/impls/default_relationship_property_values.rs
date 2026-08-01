//! Default Relationship Property Values: Universal Collections-Backed Implementations
//!
//! This module generates relationship property value adapters using the ValueType table
//! and the universal adapter system. All adapters are generic over Collections
//! backends (Vec, Huge, Arrow), enabling runtime backend selection.

// Import the macros from the crate root
use crate::generate_all_relationship_adapters;
use crate::types::ValueType;

// Generate all relationship property adapters from the ValueType table
// This expands to adapters for: Byte, Short, Int, Long, BigInt, Float, Double, Boolean, Char, String
generate_all_relationship_adapters!();

// Deferred: Relationship array adapters require additional trait implementations
// (DoubleArrayRelationshipPropertyValues, FloatArrayRelationshipPropertyValues, etc.)
// and will be enabled once array accessor methods are fully designed.
// use crate::generate_all_relationship_array_adapters;
// generate_all_relationship_array_adapters!();

// Note: The generated types are generic over Collections backend C:
// - DefaultLongRelationshipPropertyValues<C>
// - DefaultDoubleRelationshipPropertyValues<C>
// - DefaultFloatRelationshipPropertyValues<C>
// - DefaultIntRelationshipPropertyValues<C>
// - DefaultShortRelationshipPropertyValues<C>
// - DefaultByteRelationshipPropertyValues<C>
// - DefaultBooleanRelationshipPropertyValues<C>

// For backwards compatibility, create a type alias for the most common case (Double with Vec)
use crate::collections::backends::vec::VecDouble;
pub type DefaultRelationshipPropertyValues = DefaultDoubleRelationshipPropertyValues<VecDouble>;

// Provide backwards-compatible constructors
impl DefaultRelationshipPropertyValues {
    pub fn with_values(values: Vec<f64>, default_value: f64, element_count: usize) -> Self {
        let backend = VecDouble::from(values);
        let universal = crate::collections::adapter::UniversalPropertyValues::new(
            backend,
            ValueType::Double,
            default_value,
        );
        Self::new(universal, element_count)
    }

    pub fn with_default(values: Vec<f64>, element_count: usize) -> Self {
        Self::with_values(values, 0.0, element_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::properties::relationship::RelationshipPropertyValues;
    use crate::types::properties::PropertyValues;
    use crate::types::ValueType;

    #[test]
    fn default_relationship_property_values_behavior() {
        let values = DefaultRelationshipPropertyValues::with_values(vec![1.0, 2.5, 3.7], 0.0, 3);

        assert_eq!(values.value_type(), ValueType::Double);
        assert_eq!(values.element_count(), 3); // Use element_count from PropertyValues trait
        assert_eq!(values.double_value(1).unwrap(), 2.5);
        assert_eq!(values.default_value(), 0.0);
        assert!(values.has_value(0));
        assert!(!values.has_value(10));
    }

    #[test]
    fn preserves_custom_default_value() {
        let values = DefaultRelationshipPropertyValues::with_values(vec![1.0], 7.5, 1);
        assert_eq!(values.default_value(), 7.5);

        let values = DefaultRelationshipPropertyValues::with_values(vec![1.0], f64::NAN, 1);
        assert!(values.default_value().is_nan());
    }

    #[test]
    fn long_access_rejects_lossy_conversion() {
        let values = DefaultRelationshipPropertyValues::with_values(vec![2.0, 2.5], 0.0, 2);

        assert_eq!(values.long_value(0).unwrap(), 2);
        assert!(values.long_value(1).is_err());
    }
}
