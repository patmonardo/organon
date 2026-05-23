//! Java GDS: pipeline/src/main/java/org/neo4j/gds/ml/pipeline/FeatureStepUtil.java
//!
//! Utility functions for feature step operations in ML pipelines.
//!
//! Provides helpers for:
//! - Computing property dimensions (scalar vs array properties)
//! - Validating computed features for NaN values
//! - Formatting error messages for invalid features

use crate::types::graph::Graph;
use crate::types::properties::node::NodePropertyValues;
use crate::types::ValueType;

/// Compute the dimension of a node property exposed by a graph.
///
/// # Java Source
/// ```java
/// public static int propertyDimension(Graph graph, String nodeProperty) {
///     return propertyDimension(graph.nodeProperties(nodeProperty), nodeProperty);
/// }
/// ```
pub fn property_dimension_for_graph(
    graph: &dyn Graph,
    node_property: &str,
) -> Result<usize, FeatureStepError> {
    let node_properties = graph.node_properties(node_property).ok_or_else(|| {
        FeatureStepError::MissingNodeProperty {
            property: node_property.to_string(),
        }
    })?;

    property_dimension(&*node_properties, node_property)
}

/// Compute the dimension (feature count) of a node property.
///
/// Returns the number of features that a property contributes to a feature vector:
/// - Scalar properties (Long, Double): dimension = 1
/// - Array properties (DoubleArray, FloatArray, LongArray): dimension = array length
///
/// # Java Source
/// ```java
/// public static int propertyDimension(NodePropertyValues nodeProperties, String propertyName) {
///     int dimension = 0;
///     switch (nodeProperties.valueType()) {
///         case LONG:
///         case DOUBLE:
///             dimension = 1;
///             break;
///         case DOUBLE_ARRAY:
///         case FLOAT_ARRAY:
///             dimension = nodeProperties.doubleArrayValue(0).length;
///             break;
///         case LONG_ARRAY:
///             dimension = nodeProperties.longArrayValue(0).length;
///             break;
///         case UNKNOWN:
///             throw new IllegalStateException(formatWithLocale("Unknown ValueType %s", propertyName));
///     }
///     return dimension;
/// }
/// ```
pub fn property_dimension(
    node_properties: &dyn NodePropertyValues,
    property_name: &str,
) -> Result<usize, FeatureStepError> {
    // Use the dimension() method if available (for array types)
    if let Some(dim) = node_properties.dimension() {
        return Ok(dim);
    }

    // Otherwise, determine dimension from value type
    let dimension = match node_properties.value_type() {
        ValueType::Long | ValueType::Double => 1,
        ValueType::DoubleArray | ValueType::FloatArray | ValueType::LongArray => {
            // For array types, get the first element's array length
            // (all nodes should have same-dimensional properties)
            match node_properties.value_type() {
                ValueType::DoubleArray => node_properties
                    .double_array_value(0)
                    .map(|arr| arr.len())
                    .map_err(|e| FeatureStepError::PropertyAccessError {
                        property: property_name.to_string(),
                        message: e.to_string(),
                    })?,
                ValueType::FloatArray => node_properties
                    .float_array_value(0)
                    .map(|arr| arr.len())
                    .map_err(|e| FeatureStepError::PropertyAccessError {
                        property: property_name.to_string(),
                        message: e.to_string(),
                    })?,
                ValueType::LongArray => node_properties
                    .long_array_value(0)
                    .map(|arr| arr.len())
                    .map_err(|e| FeatureStepError::PropertyAccessError {
                        property: property_name.to_string(),
                        message: e.to_string(),
                    })?,
                _ => unreachable!(),
            }
        }
        _ => {
            return Err(FeatureStepError::UnknownValueType {
                property: property_name.to_string(),
            })
        }
    };

    Ok(dimension)
}

/// Validate that computed features do not contain NaN values.
///
/// Checks a slice of the feature vector for NaN values and runs the provided
/// error callback if any are found.
///
/// # Java Source
/// ```java
/// public static void validateComputedFeatures(
///     double[] linkFeatures,
///     int startOffset,
///     int endOffset,
///     Runnable throwError
/// ) {
///     for (int offset = startOffset; offset < endOffset; offset++) {
///         if (Double.isNaN(linkFeatures[offset])) {
///             throwError.run();
///         }
///     }
/// }
/// ```
pub fn validate_computed_features<F>(
    link_features: &[f64],
    start_offset: usize,
    end_offset: usize,
    throw_error: F,
) where
    F: FnOnce(),
{
    for feature in link_features.iter().take(end_offset).skip(start_offset) {
        if feature.is_nan() {
            throw_error();
            return;
        }
    }
}

/// Create an error for NaN values in computed features.
///
/// # Java Source
/// ```java
/// public static void throwNanError(
///     String featureStep,
///     Collection<String> nodeProperties,
///     long source,
///     long target
/// ) {
///     throw new IllegalArgumentException(formatWithLocale(
///         "Encountered NaN when combining the nodeProperties %s for the node pair (%d, %d) when computing the %s feature vector. " +
///         "Either define a default value if its a stored property or check the nodePropertyStep.",
///         StringJoining.join(nodeProperties),
///         source,
///         target,
///         featureStep
///     ));
/// }
/// ```
pub fn throw_nan_error(
    feature_step: &str,
    node_properties: &[String],
    source: u64,
    target: u64,
) -> FeatureStepError {
    FeatureStepError::NanInFeatures {
        feature_step: feature_step.to_string(),
        node_properties: node_properties.to_vec(),
        source,
        target,
    }
}

/// Errors that can occur during feature step operations.
#[derive(Debug, Clone, PartialEq)]
pub enum FeatureStepError {
    /// A requested node property is not present on the graph
    MissingNodeProperty {
        /// Name of the missing property
        property: String,
    },

    /// Unknown or unsupported value type for a property
    UnknownValueType {
        /// Name of the property with unknown type
        property: String,
    },

    /// Error accessing property values
    PropertyAccessError {
        /// Name of the property that failed to access
        property: String,
        /// Error message from the underlying property system
        message: String,
    },

    /// NaN values encountered in computed features
    NanInFeatures {
        /// Name of the feature step that produced NaN
        feature_step: String,
        /// Node properties that were combined
        node_properties: Vec<String>,
        /// Source node ID
        source: u64,
        /// Target node ID
        target: u64,
    },
}

impl std::fmt::Display for FeatureStepError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FeatureStepError::MissingNodeProperty { property } => {
                write!(f, "Property '{}' not found in graph", property)
            }
            FeatureStepError::UnknownValueType { property } => {
                write!(f, "Unknown ValueType for property: {}", property)
            }
            FeatureStepError::PropertyAccessError { property, message } => {
                write!(f, "Failed to access property '{}': {}", property, message)
            }
            FeatureStepError::NanInFeatures {
                feature_step,
                node_properties,
                source,
                target,
            } => {
                write!(
                    f,
                    "Encountered NaN when combining the nodeProperties [{}] for the node pair ({}, {}) when computing the {} feature vector. \
                    Either define a default value if its a stored property or check the nodePropertyStep.",
                    node_properties.join(", "),
                    source,
                    target,
                    feature_step
                )
            }
        }
    }
}

impl std::error::Error for FeatureStepError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::properties::{PropertyValues, PropertyValuesError, PropertyValuesResult};

    #[derive(Debug)]
    struct TestNodePropertyValues {
        value_type: ValueType,
        dimension: Option<usize>,
    }

    impl PropertyValues for TestNodePropertyValues {
        fn value_type(&self) -> ValueType {
            self.value_type
        }

        fn element_count(&self) -> usize {
            1
        }
    }

    impl NodePropertyValues for TestNodePropertyValues {
        fn double_value(&self, _node_id: u64) -> PropertyValuesResult<f64> {
            Ok(1.0)
        }

        fn long_value(&self, _node_id: u64) -> PropertyValuesResult<i64> {
            Ok(1)
        }

        fn double_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<f64>> {
            Ok(vec![1.0, 2.0, 3.0])
        }

        fn float_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<f32>> {
            Ok(vec![1.0, 2.0])
        }

        fn long_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<i64>> {
            Ok(vec![1, 2, 3, 4])
        }

        fn get_object(&self, _node_id: u64) -> PropertyValuesResult<Box<dyn std::any::Any>> {
            Err(PropertyValuesError::UnsupportedOperation(
                "object access not used in feature util tests".to_string(),
            ))
        }

        fn dimension(&self) -> Option<usize> {
            self.dimension
        }

        fn get_max_long_property_value(&self) -> Option<i64> {
            None
        }

        fn get_max_double_property_value(&self) -> Option<f64> {
            None
        }

        fn has_value(&self, node_id: u64) -> bool {
            node_id == 0
        }
    }

    #[test]
    fn test_property_dimension_scalar() {
        let values = TestNodePropertyValues {
            value_type: ValueType::Double,
            dimension: None,
        };

        assert_eq!(property_dimension(&values, "score").unwrap(), 1);
    }

    #[test]
    fn test_property_dimension_array_uses_first_value_when_no_dimension() {
        let values = TestNodePropertyValues {
            value_type: ValueType::DoubleArray,
            dimension: None,
        };

        assert_eq!(property_dimension(&values, "embedding").unwrap(), 3);
    }

    #[test]
    fn test_property_dimension_prefers_declared_dimension() {
        let values = TestNodePropertyValues {
            value_type: ValueType::DoubleArray,
            dimension: Some(8),
        };

        assert_eq!(property_dimension(&values, "embedding").unwrap(), 8);
    }

    #[test]
    fn test_validate_computed_features_detects_nan() {
        let mut called = false;

        validate_computed_features(&[1.0, f64::NAN, 3.0], 0, 3, || {
            called = true;
        });

        assert!(called);
    }

    #[test]
    fn test_throw_nan_error_message_matches_java_shape() {
        let error = throw_nan_error(
            "hadamard",
            &["embedding".to_string(), "pagerank".to_string()],
            12,
            34,
        );
        let message = error.to_string();

        assert!(message.contains("Encountered NaN"));
        assert!(message.contains("embedding, pagerank"));
        assert!(message.contains("(12, 34)"));
        assert!(message.contains("hadamard"));
    }

    #[test]
    fn test_missing_node_property_message() {
        let error = FeatureStepError::MissingNodeProperty {
            property: "embedding".to_string(),
        };

        assert_eq!(error.to_string(), "Property 'embedding' not found in graph");
    }
}
