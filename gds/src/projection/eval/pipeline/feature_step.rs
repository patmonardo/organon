//! FeatureStep trait
//!

use std::collections::HashMap;

/// Feature step abstraction.
///
pub trait FeatureStep {
    /// Input node properties required for feature extraction.
    ///
    fn input_node_properties(&self) -> &[String];

    /// Name of this feature step.
    ///
    fn name(&self) -> &str;

    /// Configuration map for this step.
    ///
    fn configuration(&self) -> &HashMap<String, serde_json::Value>;

    /// Convert to map for serialization (ToMapConvertible).
    ///
    fn to_map(&self) -> HashMap<String, serde_json::Value>;
}
