use std::sync::Arc;

use crate::collections::HugeDoubleArray;

pub struct NodeRegressionPipelineResult {
    predicted_values: Arc<HugeDoubleArray>,
    predicted_node_ids: Option<Arc<Vec<u64>>>,
    root_node_count: usize,
}

impl NodeRegressionPipelineResult {
    pub fn of(predicted_values: HugeDoubleArray) -> Self {
        let root_node_count = predicted_values.size();
        Self::new(predicted_values, None, root_node_count)
    }

    pub fn of_for_node_ids(
        predicted_values: HugeDoubleArray,
        predicted_node_ids: Vec<u64>,
        root_node_count: usize,
    ) -> Self {
        Self::new(predicted_values, Some(predicted_node_ids), root_node_count)
    }

    fn new(
        predicted_values: HugeDoubleArray,
        predicted_node_ids: Option<Vec<u64>>,
        root_node_count: usize,
    ) -> Self {
        Self {
            predicted_values: Arc::new(predicted_values),
            predicted_node_ids: predicted_node_ids.map(Arc::new),
            root_node_count,
        }
    }

    pub fn predicted_values(&self) -> &Arc<HugeDoubleArray> {
        &self.predicted_values
    }

    pub fn predicted_node_ids(&self) -> Option<&[u64]> {
        self.predicted_node_ids.as_deref().map(Vec::as_slice)
    }

    pub fn predicted_node_count(&self) -> usize {
        self.predicted_node_ids
            .as_deref()
            .map(Vec::len)
            .unwrap_or_else(|| self.predicted_values.size())
    }

    pub fn root_node_count(&self) -> usize {
        self.root_node_count
    }
}

#[cfg(test)]
mod tests {
    use crate::collections::HugeDoubleArray;

    use super::NodeRegressionPipelineResult;

    #[test]
    fn test_predicted_node_count_uses_compact_node_ids_when_present() {
        let predictions = HugeDoubleArray::from_vec(vec![1.0, 2.0]);
        let result = NodeRegressionPipelineResult::of_for_node_ids(predictions, vec![1, 3], 5);

        assert_eq!(result.predicted_node_count(), 2);
        assert_eq!(result.root_node_count(), 5);
    }
}
