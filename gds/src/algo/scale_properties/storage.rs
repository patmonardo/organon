//! ScaleProperties storage runtime.
//!
//! Translation source: `org.neo4j.gds.scaleproperties.ScaleProperties`.
//! Scales configured node properties (scalars and numeric arrays) using the
//! core scaler implementations and returns scaled values plus per-property
//! statistics.

use super::spec::{ScalePropertiesConfig, ScalePropertiesResult, ScalePropertiesScaler};
use super::{
    ElementScaler, PropertyFn, PropertyScaler, ScalePropertiesComputationRuntime,
    ScalePropertiesPlan,
};
use crate::algo::algorithms::scaling::{
    CenterScaler, LogScaler, MaxScaler, MeanScaler, MinMaxScaler, NoneScaler, Scaler,
    StdScoreScaler,
};
use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::ProgressTracker;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::graph::IdMap;
use crate::types::prelude::GraphStore;
use crate::types::properties::node::NodePropertyValues;
use crate::types::ValueType;
use std::sync::Arc;

/// Storage runtime: orchestrates scaler preparation and execution.
#[derive(Debug, Default, Clone)]
pub struct ScalePropertiesStorageRuntime;

impl ScalePropertiesStorageRuntime {
    pub fn new() -> Self {
        Self
    }

    /// Scale the configured properties and return scaled values plus stats.
    pub fn compute(
        &self,
        graph_store: &impl GraphStore,
        config: &ScalePropertiesConfig,
        computation: &mut ScalePropertiesComputationRuntime,
    ) -> Result<ScalePropertiesResult, AlgorithmError> {
        let termination_flag = TerminationFlag::running_true();
        let mut progress_tracker = crate::core::utils::progress::NoopProgressTracker;
        self.compute_with_controls(
            graph_store,
            config,
            computation,
            &termination_flag,
            &mut progress_tracker,
        )
    }

    pub fn compute_with_controls(
        &self,
        graph_store: &impl GraphStore,
        config: &ScalePropertiesConfig,
        computation: &mut ScalePropertiesComputationRuntime,
        termination_flag: &TerminationFlag,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<ScalePropertiesResult, AlgorithmError> {
        if config.node_properties.is_empty() {
            return Err(AlgorithmError::Execution(
                "node_properties must not be empty".to_string(),
            ));
        }
        if config.concurrency == 0 {
            return Err(AlgorithmError::Execution(
                "concurrency must be greater than zero".to_string(),
            ));
        }

        let node_count = graph_store.node_count();
        let id_map = graph_store.nodes();
        progress_tracker.begin_subtask_unknown();

        let result = self.compute_inner(
            graph_store,
            config,
            computation,
            termination_flag,
            progress_tracker,
            node_count,
            &id_map,
        );

        match &result {
            Ok(_) => progress_tracker.end_subtask(),
            Err(_) => progress_tracker.end_subtask_with_failure(),
        }

        result
    }

    fn compute_inner(
        &self,
        graph_store: &impl GraphStore,
        config: &ScalePropertiesConfig,
        computation: &mut ScalePropertiesComputationRuntime,
        termination_flag: &TerminationFlag,
        progress_tracker: &mut dyn ProgressTracker,
        node_count: usize,
        id_map: &Arc<dyn IdMap>,
    ) -> Result<ScalePropertiesResult, AlgorithmError> {
        if !termination_flag.running() {
            return Err(AlgorithmError::Execution(
                "ScaleProperties terminated".to_string(),
            ));
        }

        let mut property_scalers = Vec::with_capacity(config.node_properties.len());
        for property_name in &config.node_properties {
            if !termination_flag.running() {
                return Err(AlgorithmError::Execution(
                    "ScaleProperties terminated".to_string(),
                ));
            }

            let values = graph_store
                .node_property_values(property_name)
                .map_err(|e| {
                    AlgorithmError::Execution(format!(
                        "Node property `{}` not found in graph store: {}",
                        property_name, e
                    ))
                })?;

            let dimension = values.dimension().ok_or_else(|| {
                AlgorithmError::Execution(format!(
                    "Node property `{}` has unknown dimension",
                    property_name
                ))
            })?;

            if dimension == 0 {
                return Err(AlgorithmError::Execution(format!(
                    "Node property `{}` has zero dimension",
                    property_name
                )));
            }

            let property_scaler = match values.value_type() {
                ValueType::Long | ValueType::Double => build_scalar_scaler(
                    property_name,
                    values.clone(),
                    &config.scaler,
                    node_count,
                    config.concurrency,
                )?,
                ValueType::LongArray | ValueType::FloatArray | ValueType::DoubleArray => {
                    build_array_scaler(
                        property_name,
                        values.clone(),
                        values.value_type(),
                        dimension,
                        &config.scaler,
                        node_count,
                        config.concurrency,
                        &id_map,
                    )?
                }
                other => {
                    return Err(AlgorithmError::Execution(format!(
                        "Scaling node property `{}` of type `{}` is not supported",
                        property_name,
                        other.cypher_name()
                    )))
                }
            };

            let prepared_dimension = property_scaler.dimension();
            progress_tracker.log_progress(node_count.saturating_mul(prepared_dimension));
            property_scalers.push(property_scaler);
        }

        let total_dimension: usize = property_scalers.iter().map(|p| p.dimension()).sum();
        progress_tracker.set_volume(node_count.saturating_mul(total_dimension).saturating_mul(2));

        let plan = ScalePropertiesPlan {
            node_count,
            property_scalers,
        };

        computation.compute_with_controls(plan, termination_flag, |done| {
            progress_tracker.log_progress(done)
        })
    }
}

fn build_scalar_scaler(
    property_name: &str,
    values: Arc<dyn NodePropertyValues>,
    variant: &ScalePropertiesScaler,
    node_count: usize,
    concurrency: usize,
) -> Result<PropertyScaler, AlgorithmError> {
    validate_scalar_values(property_name, values.as_ref(), node_count)?;

    let property_fn: PropertyFn = match values.value_type() {
        ValueType::Long => {
            let pv = values.clone();
            let name = property_name.to_string();
            Arc::new(move |node_id: u64| {
                pv.long_value(node_id).unwrap_or_else(|e| {
                    panic!(
                        "Failed reading long property `{}` for node {}: {}",
                        name, node_id, e
                    )
                }) as f64
            })
        }
        ValueType::Double => {
            let pv = values.clone();
            let name = property_name.to_string();
            Arc::new(move |node_id: u64| {
                pv.double_value(node_id).unwrap_or_else(|e| {
                    panic!(
                        "Failed reading double property `{}` for node {}: {}",
                        name, node_id, e
                    )
                })
            })
        }
        _ => unreachable!(),
    };

    let scaler = create_scaler(variant, node_count, &property_fn, concurrency);
    Ok(PropertyScaler::new(
        property_name.to_string(),
        vec![ElementScaler::new(property_fn, scaler)],
    ))
}

fn build_array_scaler(
    property_name: &str,
    values: Arc<dyn NodePropertyValues>,
    value_type: ValueType,
    dimension: usize,
    variant: &ScalePropertiesScaler,
    node_count: usize,
    concurrency: usize,
    id_map: &Arc<dyn IdMap>,
) -> Result<PropertyScaler, AlgorithmError> {
    validate_array_lengths(
        property_name,
        value_type,
        values.as_ref(),
        node_count,
        dimension,
        id_map,
    )?;

    let mut elements = Vec::with_capacity(dimension);
    for idx in 0..dimension {
        let property_fn = build_array_accessor(values.clone(), value_type, dimension, idx);
        let scaler = create_scaler(variant, node_count, &property_fn, concurrency);
        elements.push(ElementScaler::new(property_fn, scaler));
    }

    Ok(PropertyScaler::new(property_name.to_string(), elements))
}

fn validate_scalar_values(
    property_name: &str,
    values: &dyn NodePropertyValues,
    node_count: usize,
) -> Result<(), AlgorithmError> {
    for node_id in 0..node_count {
        let result = match values.value_type() {
            ValueType::Long => values.long_value(node_id as u64).map(|_| ()),
            ValueType::Double => values.double_value(node_id as u64).map(|_| ()),
            _ => Ok(()),
        };

        if let Err(e) = result {
            return Err(AlgorithmError::Execution(format!(
                "Failed reading property `{}` for node {}: {}",
                property_name, node_id, e
            )));
        }
    }
    Ok(())
}

fn validate_array_lengths(
    property_name: &str,
    value_type: ValueType,
    values: &dyn NodePropertyValues,
    node_count: usize,
    expected_dimension: usize,
    id_map: &Arc<dyn IdMap>,
) -> Result<(), AlgorithmError> {
    for node_id in 0..node_count {
        let length_result: Result<usize, _> = match value_type {
            ValueType::LongArray => values.long_array_value(node_id as u64).map(|arr| arr.len()),
            ValueType::FloatArray => values
                .float_array_value(node_id as u64)
                .map(|arr| arr.len()),
            ValueType::DoubleArray => values
                .double_array_value(node_id as u64)
                .map(|arr| arr.len()),
            _ => Ok(0),
        };

        match length_result {
            Ok(length) => {
                if length > expected_dimension {
                    return Err(invalid_array_error(
                        property_name,
                        expected_dimension,
                        length,
                        node_id as u64,
                        id_map,
                    ));
                }
            }
            Err(e) => {
                return Err(AlgorithmError::Execution(format!(
                    "Failed reading property `{}` for node {}: {}",
                    property_name, node_id, e
                )));
            }
        }
    }
    Ok(())
}

fn invalid_array_error(
    property_name: &str,
    expected_dimension: usize,
    actual_dimension: usize,
    node_id: u64,
    id_map: &Arc<dyn IdMap>,
) -> AlgorithmError {
    let original_id = id_map
        .to_original_node_id(node_id as i64)
        .unwrap_or(node_id as i64);
    AlgorithmError::Execution(format!(
        "For scaling property `{}` expected array of length {} but got length {} for node {}",
        property_name, expected_dimension, actual_dimension, original_id
    ))
}

fn build_array_accessor(
    values: Arc<dyn NodePropertyValues>,
    value_type: ValueType,
    dimension: usize,
    idx: usize,
) -> PropertyFn {
    match value_type {
        ValueType::LongArray => {
            let pv = values.clone();
            Arc::new(move |node_id: u64| match pv.long_array_value(node_id) {
                Ok(array) => pick_array_value(array, dimension, idx, |v| *v as f64),
                Err(_) => f64::NAN,
            })
        }
        ValueType::FloatArray => {
            let pv = values.clone();
            Arc::new(move |node_id: u64| match pv.float_array_value(node_id) {
                Ok(array) => pick_array_value(array, dimension, idx, |v| *v as f64),
                Err(_) => f64::NAN,
            })
        }
        ValueType::DoubleArray => {
            let pv = values.clone();
            Arc::new(move |node_id: u64| match pv.double_array_value(node_id) {
                Ok(array) => pick_array_value(array, dimension, idx, |v| *v),
                Err(_) => f64::NAN,
            })
        }
        _ => Arc::new(|_| f64::NAN),
    }
}

fn pick_array_value<T, F>(array: Vec<T>, dimension: usize, idx: usize, convert: F) -> f64
where
    F: Fn(&T) -> f64,
{
    if idx >= array.len() {
        return f64::NAN;
    }

    if array.len() > dimension {
        return f64::NAN;
    }

    convert(&array[idx])
}

fn create_scaler(
    variant: &ScalePropertiesScaler,
    node_count: usize,
    property_fn: &PropertyFn,
    concurrency: usize,
) -> Box<dyn Scaler> {
    let adapter = |node_id: u64| (property_fn.as_ref())(node_id);

    match variant {
        ScalePropertiesScaler::MinMax => {
            MinMaxScaler::create(node_count as u64, &adapter, concurrency)
        }
        ScalePropertiesScaler::StdScore => {
            StdScoreScaler::create(node_count as u64, &adapter, concurrency)
        }
        ScalePropertiesScaler::Mean => MeanScaler::create(node_count as u64, &adapter, concurrency),
        ScalePropertiesScaler::Max => MaxScaler::create(node_count as u64, &adapter, concurrency),
        ScalePropertiesScaler::Center => {
            CenterScaler::create(node_count as u64, &adapter, concurrency)
        }
        ScalePropertiesScaler::Log => LogScaler::create(0.0),
        ScalePropertiesScaler::None => NoneScaler::create(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GraphStoreConfig;
    use crate::projection::RelationshipType;
    use crate::types::graph::{RelationshipTopology, SimpleIdMap};
    use crate::types::graph_store::{
        Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, GraphName,
    };
    use crate::types::schema::{GraphSchema, MutableGraphSchema, NodeLabel};
    use std::collections::HashMap;

    fn build_store() -> crate::types::prelude::DefaultGraphStore {
        let cfg = GraphStoreConfig::default();
        let graph_name = GraphName::new("g");
        let db_info = DatabaseInfo::new(
            DatabaseId::new("test-db"),
            DatabaseLocation::remote("localhost", 7687, None, None),
        );
        let capabilities = Capabilities::default();

        let mut schema = MutableGraphSchema::empty();
        schema.node_schema_mut().add_label(NodeLabel::all_nodes());
        let schema: GraphSchema = schema.build();

        let id_map = SimpleIdMap::from_original_ids([0, 1, 2]);
        let topologies: HashMap<RelationshipType, RelationshipTopology> = HashMap::new();

        let mut store = crate::types::prelude::DefaultGraphStore::new(
            cfg,
            graph_name,
            db_info,
            schema,
            capabilities,
            id_map,
            topologies,
        );
        store
            .add_node_property_f64("score".to_string(), vec![1.0, 2.0, 3.0])
            .expect("add score property");
        store
    }

    fn config(concurrency: usize) -> ScalePropertiesConfig {
        ScalePropertiesConfig {
            node_properties: vec!["score".to_string()],
            scaler: ScalePropertiesScaler::None,
            concurrency,
        }
    }

    #[test]
    fn controlled_compute_scales_property() {
        let store = build_store();
        let mut computation = ScalePropertiesComputationRuntime::new();
        let storage = ScalePropertiesStorageRuntime::new();
        let mut progress_tracker = crate::core::utils::progress::NoopProgressTracker;
        let termination = TerminationFlag::running_true();

        let result = storage
            .compute_with_controls(
                &store,
                &config(1),
                &mut computation,
                &termination,
                &mut progress_tracker,
            )
            .expect("scale properties");

        assert_eq!(
            result.scaled_properties,
            vec![vec![1.0], vec![2.0], vec![3.0]]
        );
        assert!(result.scaler_statistics.contains_key("score"));
    }

    #[test]
    fn controlled_compute_rejects_zero_concurrency() {
        let store = build_store();
        let mut computation = ScalePropertiesComputationRuntime::new();
        let storage = ScalePropertiesStorageRuntime::new();
        let mut progress_tracker = crate::core::utils::progress::NoopProgressTracker;
        let termination = TerminationFlag::running_true();

        let error = storage
            .compute_with_controls(
                &store,
                &config(0),
                &mut computation,
                &termination,
                &mut progress_tracker,
            )
            .expect_err("zero concurrency should be rejected");

        assert!(format!("{error}").contains("concurrency"));
    }

    #[test]
    fn controlled_compute_honors_termination() {
        let store = build_store();
        let mut computation = ScalePropertiesComputationRuntime::new();
        let storage = ScalePropertiesStorageRuntime::new();
        let mut progress_tracker = crate::core::utils::progress::NoopProgressTracker;
        let termination = TerminationFlag::stop_running();

        let error = storage
            .compute_with_controls(
                &store,
                &config(1),
                &mut computation,
                &termination,
                &mut progress_tracker,
            )
            .expect_err("terminated computation should fail");

        assert!(format!("{error}").contains("terminated"));
    }
}
