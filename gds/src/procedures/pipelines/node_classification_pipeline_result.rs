use std::sync::Arc;

use crate::collections::{HugeLongArray, HugeObjectArray};
use crate::ml::core::subgraph::LocalIdMap;
use crate::ml::node_classification::NodeClassificationPredictResult;

pub struct NodeClassificationPipelineResult {
    predicted_classes: Arc<HugeLongArray>,
    predicted_probabilities: Option<Arc<HugeObjectArray<Vec<f64>>>>,
    predicted_node_ids: Option<Arc<Vec<u64>>>,
    root_node_count: usize,
}

impl NodeClassificationPipelineResult {
    pub fn of(
        node_classification_result: &NodeClassificationPredictResult,
        class_id_map: &LocalIdMap,
    ) -> Self {
        let internal_predictions = node_classification_result.predicted_classes();
        let mut predictions = HugeLongArray::new(internal_predictions.size());

        for index in 0..internal_predictions.size() {
            let internal_class_id = internal_predictions.get(index) as usize;
            let original_class_id = class_id_map.to_original(internal_class_id) as i64;
            predictions.set(index, original_class_id);
        }

        Self::new(
            predictions,
            node_classification_result
                .predicted_probabilities()
                .map(Arc::clone),
            None,
            internal_predictions.size(),
        )
    }

    pub fn of_for_node_ids(
        node_classification_result: &NodeClassificationPredictResult,
        class_id_map: &LocalIdMap,
        predicted_node_ids: Vec<u64>,
        root_node_count: usize,
    ) -> Self {
        let internal_predictions = node_classification_result.predicted_classes();
        let mut predictions = HugeLongArray::new(internal_predictions.size());

        for index in 0..internal_predictions.size() {
            let internal_class_id = internal_predictions.get(index) as usize;
            let original_class_id = class_id_map.to_original(internal_class_id) as i64;
            predictions.set(index, original_class_id);
        }

        Self::new(
            predictions,
            node_classification_result
                .predicted_probabilities()
                .map(Arc::clone),
            Some(predicted_node_ids),
            root_node_count,
        )
    }

    fn new(
        predicted_classes: HugeLongArray,
        predicted_probabilities: Option<Arc<HugeObjectArray<Vec<f64>>>>,
        predicted_node_ids: Option<Vec<u64>>,
        root_node_count: usize,
    ) -> Self {
        Self {
            predicted_classes: Arc::new(predicted_classes),
            predicted_probabilities,
            predicted_node_ids: predicted_node_ids.map(Arc::new),
            root_node_count,
        }
    }

    pub fn predicted_classes(&self) -> &Arc<HugeLongArray> {
        &self.predicted_classes
    }

    pub fn predicted_probabilities(&self) -> Option<&Arc<HugeObjectArray<Vec<f64>>>> {
        self.predicted_probabilities.as_ref()
    }

    pub fn predicted_node_ids(&self) -> Option<&[u64]> {
        self.predicted_node_ids.as_deref().map(Vec::as_slice)
    }

    pub fn predicted_node_count(&self) -> usize {
        self.predicted_node_ids
            .as_deref()
            .map(Vec::len)
            .unwrap_or_else(|| self.predicted_classes.size())
    }

    pub fn root_node_count(&self) -> usize {
        self.root_node_count
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::collections::HugeLongArray;
    use crate::ml::core::subgraph::LocalIdMap;
    use crate::ml::node_classification::NodeClassificationPredictResult;

    use super::NodeClassificationPipelineResult;

    #[test]
    fn test_predicted_node_count_uses_compact_node_ids_when_present() {
        let mut internal_predictions = HugeLongArray::new(2);
        internal_predictions.set(0, 0);
        internal_predictions.set(1, 1);

        let prediction_result =
            NodeClassificationPredictResult::new(Arc::new(internal_predictions), None);
        let class_id_map = LocalIdMap::of(&[10, 20]);
        let result = NodeClassificationPipelineResult::of_for_node_ids(
            &prediction_result,
            &class_id_map,
            vec![1, 3],
            5,
        );

        assert_eq!(result.predicted_node_count(), 2);
        assert_eq!(result.root_node_count(), 5);
    }
}
