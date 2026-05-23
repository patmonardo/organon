use std::sync::Arc;

use crate::applications::algorithms::machinery::NodeProperty;
use crate::collections::backends::vec::VecDoubleArray;
use crate::collections::backends::vec::VecLong;
use crate::procedures::pipelines::NodeClassificationPipelineResult;
use crate::types::properties::node::NodePropertyValues;
use crate::types::properties::node::{
    DefaultDoubleArrayNodePropertyValues, DefaultLongNodePropertyValues,
};

pub fn as_properties(
    result: Option<&NodeClassificationPipelineResult>,
    property_name: &str,
    predicted_probability_property: Option<&str>,
) -> Vec<NodeProperty> {
    let Some(result) = result else {
        return vec![];
    };

    let predicted_classes = result.predicted_classes();
    let node_count = result.root_node_count();

    let mut class_values_vec = vec![0; node_count];
    match result.predicted_node_ids() {
        Some(node_ids) => {
            for (row_id, node_id) in node_ids.iter().enumerate() {
                class_values_vec[*node_id as usize] = predicted_classes.get(row_id);
            }
        }
        None => {
            for idx in 0..predicted_classes.size() {
                class_values_vec[idx] = predicted_classes.get(idx);
            }
        }
    }

    let class_values: Arc<dyn NodePropertyValues> =
        Arc::new(DefaultLongNodePropertyValues::<VecLong>::from_collection(
            VecLong::from(class_values_vec),
            node_count,
        ));

    let mut node_properties = vec![NodeProperty::new(property_name, class_values)];

    if let Some(probabilities) = result.predicted_probabilities() {
        let property_key = predicted_probability_property
            .expect("predictedProbabilityProperty must be set when probabilities are present");

        let mut rows: Vec<Option<Vec<f64>>> = vec![None; node_count];
        match result.predicted_node_ids() {
            Some(node_ids) => {
                for (row_id, node_id) in node_ids.iter().enumerate() {
                    rows[*node_id as usize] = Some(probabilities.get(row_id).clone());
                }
            }
            None => {
                for idx in 0..probabilities.size() {
                    rows[idx] = Some(probabilities.get(idx).clone());
                }
            }
        }

        let probability_values: Arc<dyn NodePropertyValues> = Arc::new(
            DefaultDoubleArrayNodePropertyValues::<VecDoubleArray>::from_collection(
                VecDoubleArray::from(rows),
                node_count,
            ),
        );

        node_properties.push(NodeProperty::new(property_key, probability_values));
    }

    node_properties
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::HugeLongArray;
    use crate::ml::core::subgraph::LocalIdMap;
    use crate::ml::node_classification::NodeClassificationPredictResult;

    #[test]
    fn test_as_properties_expands_filtered_predictions_to_root_node_ids() {
        let mut internal_predictions = HugeLongArray::new(2);
        internal_predictions.set(0, 0);
        internal_predictions.set(1, 1);

        let prediction_result =
            NodeClassificationPredictResult::new(Arc::new(internal_predictions), None);
        let class_id_map = LocalIdMap::of(&[10, 20]);
        let result = NodeClassificationPipelineResult::of_for_node_ids(
            &prediction_result,
            &class_id_map,
            vec![2, 4],
            5,
        );

        let properties = as_properties(Some(&result), "predicted", None);
        let values = &properties[0].values;

        assert_eq!(values.node_count(), 5);
        assert_eq!(values.long_value(2).expect("node 2 value"), 10);
        assert_eq!(values.long_value(4).expect("node 4 value"), 20);
    }
}
