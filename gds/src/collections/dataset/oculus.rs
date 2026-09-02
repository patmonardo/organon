//! Oculus: the Dataset-level opaque PureForm Eye of Eval.
//!
//! Dataset Core owns background dispatch. Oculus alone admits the reciprocal
//! Rational/Empirical Concept to evaluation and returns its mediated result.

use crate::collections::dataframe::GDSDataFrame;
use crate::collections::dataset::core::{
    DatasetLanguagePlugin, DatasetOrb, DatasetPluginError, DatasetPluginErrorClass,
    DatasetPluginRequest, DatasetPluginResponse,
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct OculusEvaluation {
    response: DatasetPluginResponse,
    mediation_trace: Vec<String>,
}

#[derive(Debug)]
pub struct OculusDataFrameMediation {
    dataset: DatasetOrb,
    output: GDSDataFrame,
    evaluation: OculusEvaluation,
}

impl OculusDataFrameMediation {
    pub fn dataset(&self) -> &DatasetOrb {
        &self.dataset
    }

    pub fn output(&self) -> &GDSDataFrame {
        &self.output
    }

    pub fn evaluation(&self) -> &OculusEvaluation {
        &self.evaluation
    }

    pub fn into_parts(self) -> (DatasetOrb, GDSDataFrame, OculusEvaluation) {
        (self.dataset, self.output, self.evaluation)
    }
}

impl OculusEvaluation {
    pub fn response(&self) -> &DatasetPluginResponse {
        &self.response
    }

    pub fn mediation_trace(&self) -> &[String] {
        &self.mediation_trace
    }

    pub fn into_response(self) -> DatasetPluginResponse {
        self.response
    }
}

/// The only public entrance to Ocular evaluation.
#[derive(Debug, Default, Clone, Copy)]
pub struct OculusHandle {
    kernel: OculusKernel,
}

impl OculusHandle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn evaluate(
        &self,
        dataset: &mut DatasetOrb,
        request: &DatasetPluginRequest,
    ) -> Result<OculusEvaluation, DatasetPluginError> {
        self.kernel.evaluate(dataset, request)
    }

    pub fn register_plugin(
        &self,
        dataset: &mut DatasetOrb,
        plugin: Arc<dyn DatasetLanguagePlugin>,
    ) -> Result<(), DatasetPluginError> {
        self.kernel.register_plugin(dataset, plugin)
    }

    /// Execute the public `DataFrame -> DatasetOrb -> DataFrame` algebra.
    pub fn mediate_dataframe(
        &self,
        dataset_name: impl Into<String>,
        frame: GDSDataFrame,
        plugin: Arc<dyn DatasetLanguagePlugin>,
        request: &DatasetPluginRequest,
    ) -> Result<OculusDataFrameMediation, DatasetPluginError> {
        let mut dataset = DatasetOrb::named(dataset_name, frame);
        self.register_plugin(&mut dataset, plugin)?;
        let evaluation = self.evaluate(&mut dataset, request)?;
        let output = if let Some(artifact_id) = &request.output_artifact_id {
            dataset
                .artifact(artifact_id)
                .ok_or_else(|| {
                    DatasetPluginError::new(
                        DatasetPluginErrorClass::MissingArtifact,
                        format!("requested output artifact {artifact_id} was not produced"),
                    )
                })?
                .table()
                .clone()
        } else {
            dataset.table().clone()
        };

        Ok(OculusDataFrameMediation {
            dataset,
            output,
            evaluation,
        })
    }
}

/// Private evaluator authority. Plugins and Dataset Core cannot invoke its
/// internal laws directly; they meet it only through `OculusHandle`.
#[derive(Debug, Default, Clone, Copy)]
struct OculusKernel;

impl OculusKernel {
    fn register_plugin(
        &self,
        dataset: &mut DatasetOrb,
        plugin: Arc<dyn DatasetLanguagePlugin>,
    ) -> Result<(), DatasetPluginError> {
        if plugin.language_id().is_empty() || plugin.capabilities().operations.is_empty() {
            return Err(DatasetPluginError::new(
                DatasetPluginErrorClass::InvalidPayload,
                "Oculus requires a named language and at least one plugin operation",
            ));
        }
        dataset.register_plugin(plugin)
    }

    fn evaluate(
        &self,
        dataset: &mut DatasetOrb,
        request: &DatasetPluginRequest,
    ) -> Result<OculusEvaluation, DatasetPluginError> {
        if !request.concept.is_determinate() {
            return Err(DatasetPluginError::new(
                DatasetPluginErrorClass::InvalidPayload,
                "Oculus requires determinate <Model,Feature,Plan> and <Corpus,LM,Logic> moments",
            ));
        }

        let response = dataset.dispatch(request)?;
        let mediation_trace = vec![
            format!(
                "rational={}:{}:{}",
                request.concept.rational.model.as_str(),
                request
                    .concept
                    .rational
                    .features
                    .iter()
                    .map(|feature| feature.as_str())
                    .collect::<Vec<_>>()
                    .join("|"),
                request.concept.rational.plan.as_str()
            ),
            format!(
                "empirical={}:{}:{}",
                request.concept.empirical.corpus.as_str(),
                request.concept.empirical.language_model.as_str(),
                request.concept.empirical.logic.as_str()
            ),
            format!(
                "judgment={}",
                if response.report.passed {
                    "admitted"
                } else {
                    "refused"
                }
            ),
        ];

        Ok(OculusEvaluation {
            response,
            mediation_trace,
        })
    }
}
