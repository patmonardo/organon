//! FastRP storage runtime.
//!
//! This is the **Gross pole**: obtaining graph views and property-backed feature extractors.

use crate::ml::core::features::feature_extraction::{property_extractors, AnyFeatureExtractor};
use crate::projection::eval::algorithm::AlgorithmError;
use crate::task::concurrency::TerminationFlag;
use crate::task::progress::ProgressTracker;
use crate::types::graph::Graph;
use std::sync::Arc;

use super::{FastRPComputationRuntime, FastRPConfig, FastRPResult};

/// FastRP storage runtime.
#[derive(Debug, Default, Clone)]
pub struct FastRPStorageRuntime;

impl FastRPStorageRuntime {
    pub fn new() -> Self {
        Self
    }

    /// Build feature extractors for the requested node properties.
    ///
    /// Returns an error string (caller maps to `AlgorithmError`).
    pub fn feature_extractors(
        &self,
        graph: &dyn Graph,
        feature_properties: &[String],
    ) -> Result<Vec<AnyFeatureExtractor>, String> {
        if feature_properties.is_empty() {
            return Ok(Vec::new());
        }

        let mut extractors = Vec::with_capacity(feature_properties.len());
        for key in feature_properties {
            // Avoid panics from the literal Java translation helper.
            if graph.node_properties(key).is_none() {
                return Err(format!(
                    "Missing node property `{key}`. Consider using a default value in the property projection."
                ));
            }
        }

        // Now safe to use the translated helper.
        extractors.extend(property_extractors(graph, feature_properties));

        Ok(extractors)
    }

    pub fn compute(
        &self,
        graph: Arc<dyn Graph>,
        config: &FastRPConfig,
        progress_tracker: &mut dyn ProgressTracker,
        termination_flag: &TerminationFlag,
    ) -> Result<FastRPResult, AlgorithmError> {
        termination_flag.assert_running();
        let feature_extractors = self
            .feature_extractors(graph.as_ref(), &config.feature_properties)
            .map_err(AlgorithmError::Execution)?;
        FastRPComputationRuntime::run_with_controls(
            graph,
            config,
            feature_extractors,
            progress_tracker,
            termination_flag,
        )
    }
}
