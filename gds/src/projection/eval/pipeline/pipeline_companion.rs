//! Java GDS: pipeline/src/main/java/org/neo4j/gds/ml/pipeline/PipelineCompanion.java
//!
//! Companion utilities for ML pipeline configuration and validation.
//!
//! Provides helpers for:
//! - Preparing pipeline configurations with graph names
//! - Configuring auto-tuning parameters
//! - Validating metric compatibility with training methods

use std::collections::HashMap;

use crate::projection::eval::pipeline::{TrainingMethod, TrainingPipeline, TunableTrainerConfig};

/// Anonymous graph name used when operating on in-memory graphs without a catalog entry.
///
pub const ANONYMOUS_GRAPH: &str = "__ANONYMOUS_GRAPH__";

/// Name of the Out-of-Bag Error metric (specific to Random Forest).
///
/// This metric can only be used with Random Forest model candidates.
pub const OUT_OF_BAG_ERROR: &str = "OUT_OF_BAG_ERROR";

/// Prepare a pipeline configuration by setting the graph name.
///
/// If `graph_name_or_configuration` is a string, it's used as the graph name.
/// Otherwise, the anonymous graph name is used.
///
///
/// # Note
/// In Java GDS, this is used to handle the case where node property steps
/// modify the graph store. In the future, this might operate on a shallow
/// copy instead.
pub fn prepare_pipeline_config(
    graph_name: Option<&str>,
    algo_configuration: &mut HashMap<String, serde_json::Value>,
) {
    let graph_name_value = match graph_name {
        Some(name) => serde_json::Value::String(name.to_string()),
        None => serde_json::Value::String(ANONYMOUS_GRAPH.to_string()),
    };

    algo_configuration.insert("graphName".to_string(), graph_name_value);
}

/// Validate that the main metric is compatible with the pipeline's training methods.
///
/// If OUT_OF_BAG_ERROR is used as the main metric, only Random Forest model
/// candidates are allowed.
///
pub fn validate_main_metric(
    main_metric: &str,
    training_methods: &[String],
) -> Result<(), PipelineCompanionError> {
    if main_metric == OUT_OF_BAG_ERROR {
        let non_rf_methods: Vec<String> = training_methods
            .iter()
            .filter(|method| {
                method.as_str() != TrainingMethod::RandomForestClassification.to_string()
            })
            .cloned()
            .collect();

        if !non_rf_methods.is_empty() {
            return Err(PipelineCompanionError::IncompatibleMetric {
                metric: OUT_OF_BAG_ERROR.to_string(),
                incompatible_methods: non_rf_methods,
            });
        }
    }

    Ok(())
}

/// Validate the main metric against the actual training parameter space.
///
/// This mirrors Java `PipelineCompanion.validateMainMetric`: only non-empty
/// model-candidate entries are considered, and `OUT_OF_BAG_ERROR` is allowed
/// only with `RandomForestClassification` candidates.
pub fn validate_main_metric_for_pipeline<P: TrainingPipeline + ?Sized>(
    pipeline: &P,
    main_metric: &str,
) -> Result<(), PipelineCompanionError> {
    validate_main_metric_for_parameter_space(main_metric, pipeline.training_parameter_space())
}

pub fn validate_main_metric_for_parameter_space(
    main_metric: &str,
    training_parameter_space: &HashMap<TrainingMethod, Vec<Box<dyn TunableTrainerConfig>>>,
) -> Result<(), PipelineCompanionError> {
    if main_metric != OUT_OF_BAG_ERROR {
        return Ok(());
    }

    let mut non_rf_methods: Vec<String> = training_parameter_space
        .iter()
        .filter(|(method, configs)| {
            **method != TrainingMethod::RandomForestClassification && !configs.is_empty()
        })
        .map(|(method, _)| method.to_string())
        .collect();
    non_rf_methods.sort();

    if non_rf_methods.is_empty() {
        return Ok(());
    }

    Err(PipelineCompanionError::IncompatibleMetric {
        metric: OUT_OF_BAG_ERROR.to_string(),
        incompatible_methods: non_rf_methods,
    })
}

/// Errors that can occur in pipeline companion operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipelineCompanionError {
    /// Main metric is incompatible with training methods
    IncompatibleMetric {
        /// The metric that caused the issue
        metric: String,
        /// Training methods that are incompatible
        incompatible_methods: Vec<String>,
    },
}

impl std::fmt::Display for PipelineCompanionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PipelineCompanionError::IncompatibleMetric {
                metric,
                incompatible_methods,
            } => {
                write!(
                    f,
                    "If {} is used as the main metric (the first one), then only RandomForest model candidates are allowed. \
                    Incompatible training methods used are: {}.",
                    metric,
                    incompatible_methods.join(", ")
                )
            }
        }
    }
}

impl std::error::Error for PipelineCompanionError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::eval::pipeline::TunableTrainerConfig;

    #[derive(Clone)]
    struct TestTrainerConfig {
        method: TrainingMethod,
    }

    impl TunableTrainerConfig for TestTrainerConfig {
        fn training_method(&self) -> TrainingMethod {
            self.method
        }

        fn is_concrete(&self) -> bool {
            true
        }

        fn to_map(&self) -> HashMap<String, serde_json::Value> {
            HashMap::new()
        }
    }

    #[test]
    fn test_prepare_pipeline_config_with_name() {
        let mut config = HashMap::new();
        prepare_pipeline_config(Some("myGraph"), &mut config);

        assert_eq!(
            config.get("graphName").and_then(|v| v.as_str()),
            Some("myGraph")
        );
    }

    #[test]
    fn test_prepare_pipeline_config_anonymous() {
        let mut config = HashMap::new();
        prepare_pipeline_config(None, &mut config);

        assert_eq!(
            config.get("graphName").and_then(|v| v.as_str()),
            Some(ANONYMOUS_GRAPH)
        );
    }

    #[test]
    fn test_validate_main_metric_rf_only_with_rf() {
        let methods = vec!["RandomForestClassification".to_string()];
        let result = validate_main_metric(OUT_OF_BAG_ERROR, &methods);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_main_metric_rf_only_with_non_rf() {
        let methods = vec![
            "LogisticRegression".to_string(),
            "RandomForestClassification".to_string(),
        ];
        let result = validate_main_metric(OUT_OF_BAG_ERROR, &methods);
        assert!(result.is_err());

        if let Err(PipelineCompanionError::IncompatibleMetric {
            incompatible_methods,
            ..
        }) = result
        {
            assert_eq!(incompatible_methods, vec!["LogisticRegression"]);
        }
    }

    #[test]
    fn test_validate_main_metric_error_message() {
        let methods = vec![
            "LogisticRegression".to_string(),
            "RandomForestClassification".to_string(),
        ];

        let err = validate_main_metric(OUT_OF_BAG_ERROR, &methods).unwrap_err();
        let msg = err.to_string();

        assert!(msg.starts_with("If OUT_OF_BAG_ERROR is used as the main metric"));
        assert!(msg.contains("Incompatible training methods used are: LogisticRegression."));
    }

    #[test]
    fn test_validate_main_metric_other_metric() {
        let methods = vec!["LogisticRegression".to_string()];
        let result = validate_main_metric("ACCURACY", &methods);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_main_metric_parameter_space_ignores_empty_non_rf_methods() {
        let mut parameter_space: HashMap<TrainingMethod, Vec<Box<dyn TunableTrainerConfig>>> =
            HashMap::new();
        parameter_space.insert(TrainingMethod::LogisticRegression, Vec::new());
        parameter_space.insert(
            TrainingMethod::RandomForestClassification,
            vec![Box::new(TestTrainerConfig {
                method: TrainingMethod::RandomForestClassification,
            })],
        );

        let result = validate_main_metric_for_parameter_space(OUT_OF_BAG_ERROR, &parameter_space);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_main_metric_parameter_space_rejects_non_empty_non_rf_methods() {
        let mut parameter_space: HashMap<TrainingMethod, Vec<Box<dyn TunableTrainerConfig>>> =
            HashMap::new();
        parameter_space.insert(
            TrainingMethod::LogisticRegression,
            vec![Box::new(TestTrainerConfig {
                method: TrainingMethod::LogisticRegression,
            })],
        );
        parameter_space.insert(
            TrainingMethod::RandomForestClassification,
            vec![Box::new(TestTrainerConfig {
                method: TrainingMethod::RandomForestClassification,
            })],
        );

        let err = validate_main_metric_for_parameter_space(OUT_OF_BAG_ERROR, &parameter_space)
            .expect_err("expected incompatible metric");

        assert_eq!(
            err,
            PipelineCompanionError::IncompatibleMetric {
                metric: OUT_OF_BAG_ERROR.to_string(),
                incompatible_methods: vec!["LogisticRegression".to_string()],
            }
        );
    }
}
