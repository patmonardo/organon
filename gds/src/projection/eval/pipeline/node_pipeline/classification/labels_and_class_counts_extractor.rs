use crate::collections::{long_multiset::LongMultiSet, HugeIntArray};
use crate::ml::core::subgraph::LocalIdMap;
use crate::types::properties::node::NodePropertyValues;
use crate::types::properties::PropertyValuesError;
use crate::types::ValueType;

#[derive(Debug, PartialEq)]
pub enum LabelsAndClassCountsError {
    MissingValue(u64),
    NonIntegerValue(u64),
    NegativeValue { node_id: u64, value: i64 },
}

impl std::fmt::Display for LabelsAndClassCountsError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingValue(node_id) => {
                write!(
                    formatter,
                    "Node with id {node_id} has no classification target value"
                )
            }
            Self::NonIntegerValue(node_id) => write!(
                formatter,
                "Node with id {node_id} has a non-integer classification target value"
            ),
            Self::NegativeValue { node_id, value } => write!(
                formatter,
                "Node with id {node_id} has negative classification target value {value}"
            ),
        }
    }
}

impl std::error::Error for LabelsAndClassCountsError {}

/// Result of extracting labels and class counts from target node property.
#[derive(Debug, Clone)]
pub struct LabelsAndClassCounts {
    labels: HugeIntArray,
    class_counts: LongMultiSet,
}

impl LabelsAndClassCounts {
    pub fn new(labels: HugeIntArray, class_counts: LongMultiSet) -> Self {
        Self {
            labels,
            class_counts,
        }
    }

    pub fn labels(&self) -> &HugeIntArray {
        &self.labels
    }

    pub fn class_counts(&self) -> &LongMultiSet {
        &self.class_counts
    }
}

/// Utility for extracting labels and class counts from target node properties.
///
/// This is a stateless utility with private constructor (module-level functions in Rust).
pub struct LabelsAndClassCountsExtractor;

impl LabelsAndClassCountsExtractor {
    /// Extract labels and class counts from target node property.
    ///
    /// This creates:
    /// 1. A HugeIntArray of labels (mapped from original class IDs to local consecutive IDs)
    /// 2. A LongMultiSet of class counts (how many nodes per class)
    /// 3. A LocalIdMap for bidirectional mapping between original and local class IDs
    pub fn extract_labels_and_class_counts(
        target_node_property: &dyn NodePropertyValues,
        node_count: u64,
    ) -> Result<LabelsAndClassCounts, LabelsAndClassCountsError> {
        let node_ids = (0..node_count).collect::<Vec<_>>();
        Self::extract_labels_and_class_counts_for_node_ids(target_node_property, &node_ids)
    }

    pub fn extract_labels_and_class_counts_for_node_ids(
        target_node_property: &dyn NodePropertyValues,
        node_ids: &[u64],
    ) -> Result<LabelsAndClassCounts, LabelsAndClassCountsError> {
        let values = Self::extract_values(target_node_property, node_ids)?;
        let mut class_counts = LongMultiSet::new();
        for value in &values {
            class_counts.add(*value);
        }

        // Get unique class IDs and sort them for deterministic mapping
        let class_ids_i64 = class_counts.keys();
        let mut class_ids: Vec<u64> = class_ids_i64.into_iter().map(|id| id as u64).collect();
        class_ids.sort_unstable();

        // Create local ID mapping
        let mut local_id_map = LocalIdMap::of_sorted(&class_ids);

        // Create labels array
        let mut labels = HugeIntArray::new(node_ids.len());

        // Map each node's class ID to local ID
        for (row_id, class_id) in values.into_iter().enumerate() {
            let class_id = class_id as u64;
            let mapped_id = local_id_map.to_mapped(class_id) as i32;
            labels.set(row_id, mapped_id);
        }

        Ok(LabelsAndClassCounts::new(labels, class_counts))
    }

    fn extract_values(
        target_node_property: &dyn NodePropertyValues,
        node_ids: &[u64],
    ) -> Result<Vec<i64>, LabelsAndClassCountsError> {
        if !matches!(
            target_node_property.value_type(),
            ValueType::Byte | ValueType::Short | ValueType::Int | ValueType::Long
        ) {
            return Err(LabelsAndClassCountsError::NonIntegerValue(
                node_ids.first().copied().unwrap_or(0),
            ));
        }

        node_ids
            .iter()
            .map(|node_id| {
                let value =
                    target_node_property
                        .long_value(*node_id)
                        .map_err(|error| match error {
                            PropertyValuesError::UnsupportedType { .. }
                            | PropertyValuesError::UnsupportedOperation(_) => {
                                LabelsAndClassCountsError::NonIntegerValue(*node_id)
                            }
                            PropertyValuesError::InvalidNodeId(_)
                            | PropertyValuesError::ValueNotFound(_) => {
                                LabelsAndClassCountsError::MissingValue(*node_id)
                            }
                        })?;
                if value < 0 {
                    return Err(LabelsAndClassCountsError::NegativeValue {
                        node_id: *node_id,
                        value,
                    });
                }
                Ok(value)
            })
            .collect()
    }

    /// Extract class counts from target node property.
    ///
    /// Returns a multiset (map from class ID to count of nodes with that class).
    pub fn extract_class_counts(
        target_node_property: &dyn NodePropertyValues,
        node_count: u64,
    ) -> Result<LongMultiSet, LabelsAndClassCountsError> {
        let node_ids = (0..node_count).collect::<Vec<_>>();
        Self::extract_class_counts_for_node_ids(target_node_property, &node_ids)
    }

    pub fn extract_class_counts_for_node_ids(
        target_node_property: &dyn NodePropertyValues,
        node_ids: &[u64],
    ) -> Result<LongMultiSet, LabelsAndClassCountsError> {
        let mut class_counts = LongMultiSet::new();
        for class_id in Self::extract_values(target_node_property, node_ids)? {
            class_counts.add(class_id);
        }
        Ok(class_counts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::backends::vec::VecDouble;
    use crate::collections::backends::vec::VecLong;
    use crate::types::properties::node::DefaultDoubleNodePropertyValues;
    use crate::types::properties::node::DefaultLongNodePropertyValues;

    #[test]
    fn test_labels_and_class_counts_new() {
        let labels = HugeIntArray::from_vec(vec![0, 1, 0, 2]);
        let mut class_counts = LongMultiSet::new();
        class_counts.add(0);
        class_counts.add(0);
        class_counts.add(1);
        class_counts.add(2);

        let result = LabelsAndClassCounts::new(labels.clone(), class_counts.clone());

        assert_eq!(result.labels().get(0), 0);
        assert_eq!(result.labels().get(1), 1);
        assert_eq!(result.labels().get(2), 0);
        assert_eq!(result.labels().get(3), 2);
        assert_eq!(result.class_counts().count(0), 2);
        assert_eq!(result.class_counts().count(1), 1);
        assert_eq!(result.class_counts().count(2), 1);
    }

    #[test]
    fn test_extract_class_counts() {
        let backend = VecLong::from(vec![0, 1, 0, 2, 0]);
        let target_property = DefaultLongNodePropertyValues::from_collection(backend, 5);
        let node_count = 5;

        let class_counts =
            LabelsAndClassCountsExtractor::extract_class_counts(&target_property, node_count)
                .expect("valid class targets");

        // Should have 3 classes: 0 (3 times), 1 (1 time), 2 (1 time)
        assert_eq!(class_counts.count(0), 3);
        assert_eq!(class_counts.count(1), 1);
        assert_eq!(class_counts.count(2), 1);
        assert_eq!(class_counts.size(), 3);
    }

    #[test]
    fn test_extract_labels_and_class_counts() {
        let backend = VecLong::from(vec![10, 5, 10, 15]);
        let target_property = DefaultLongNodePropertyValues::from_collection(backend, 4);
        let node_count = 4;

        let result = LabelsAndClassCountsExtractor::extract_labels_and_class_counts(
            &target_property,
            node_count,
        )
        .expect("valid class targets");

        // Classes should be mapped: 5->0, 10->1, 15->2
        assert_eq!(result.labels().get(0), 1); // 10 -> 1
        assert_eq!(result.labels().get(1), 0); // 5 -> 0
        assert_eq!(result.labels().get(2), 1); // 10 -> 1
        assert_eq!(result.labels().get(3), 2); // 15 -> 2

        // Class counts: 5 appears 1 time, 10 appears 2 times, 15 appears 1 time
        assert_eq!(result.class_counts().count(5), 1);
        assert_eq!(result.class_counts().count(10), 2);
        assert_eq!(result.class_counts().count(15), 1);
    }

    #[test]
    fn test_extract_labels_and_class_counts_for_node_ids() {
        let backend = VecLong::from(vec![10, 5, 10, 15, 5]);
        let target_property = DefaultLongNodePropertyValues::from_collection(backend, 5);
        let node_ids = vec![1, 3, 4];

        let result = LabelsAndClassCountsExtractor::extract_labels_and_class_counts_for_node_ids(
            &target_property,
            &node_ids,
        )
        .expect("valid class targets");

        // Target rows use the explicit node order: node 1 => 5, node 3 => 15, node 4 => 5.
        assert_eq!(result.labels().size(), 3);
        assert_eq!(result.labels().get(0), 0);
        assert_eq!(result.labels().get(1), 1);
        assert_eq!(result.labels().get(2), 0);
        assert_eq!(result.class_counts().count(5), 2);
        assert_eq!(result.class_counts().count(15), 1);
    }

    #[test]
    fn test_extract_rejects_non_integer_target() {
        let target_property =
            DefaultDoubleNodePropertyValues::from_collection(VecDouble::from(vec![0.0, 1.5]), 2);

        let result =
            LabelsAndClassCountsExtractor::extract_labels_and_class_counts(&target_property, 2);

        assert!(matches!(
            result,
            Err(LabelsAndClassCountsError::NonIntegerValue(0))
        ));
    }

    #[test]
    fn test_extract_rejects_negative_target() {
        let target_property =
            DefaultLongNodePropertyValues::from_collection(VecLong::from(vec![0, -1]), 2);

        let result =
            LabelsAndClassCountsExtractor::extract_labels_and_class_counts(&target_property, 2);

        assert!(matches!(
            result,
            Err(LabelsAndClassCountsError::NegativeValue {
                node_id: 1,
                value: -1,
            })
        ));
    }

    #[test]
    fn test_extract_rejects_missing_target() {
        let target_property =
            DefaultLongNodePropertyValues::from_collection(VecLong::from(vec![0]), 1);

        let result = LabelsAndClassCountsExtractor::extract_labels_and_class_counts_for_node_ids(
            &target_property,
            &[0, 1],
        );

        assert!(matches!(
            result,
            Err(LabelsAndClassCountsError::MissingValue(1))
        ));
    }

    #[test]
    fn test_labels_and_class_counts_accessors() {
        let labels = HugeIntArray::from_vec(vec![0, 1, 2]);
        let mut class_counts = LongMultiSet::new();
        class_counts.add(0);
        class_counts.add(1);
        class_counts.add(2);

        let result = LabelsAndClassCounts::new(labels.clone(), class_counts.clone());

        // Verify accessors return correct references
        assert_eq!(result.labels().get(0), 0);
        assert_eq!(result.class_counts().size(), 3);
        assert_eq!(result.class_counts().count(1), 1);
    }
}
