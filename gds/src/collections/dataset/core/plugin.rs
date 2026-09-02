//! Dataset plugin contracts.
//!
//! This module defines a language-neutral extension protocol so Dataset can be
//! used as a workflow construction kit for SDSL extensions. Domain languages
//! (Graph, NLP, or user-defined) should implement plugins against these
//! contracts rather than embedding domain ownership in Dataset core.

use std::any::Any;
use std::collections::BTreeMap;
use std::fmt;
use std::sync::Arc;

use crate::collections::dataframe::GDSDataFrame;
use crate::collections::dataset::core::artifact::{DatasetArtifactKind, DatasetArtifactProfile};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DatasetPluginErrorClass {
    DuplicatePlugin,
    PluginNotFound,
    InvalidPayload,
    PluginExecution,
    MissingArtifact,
    ArtifactConflict,
}

impl DatasetPluginErrorClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DuplicatePlugin => "DuplicatePlugin",
            Self::PluginNotFound => "PluginNotFound",
            Self::InvalidPayload => "InvalidPayload",
            Self::PluginExecution => "PluginExecution",
            Self::MissingArtifact => "MissingArtifact",
            Self::ArtifactConflict => "ArtifactConflict",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatasetPluginOperation {
    Validate,
    Compile,
    Execute,
    Materialize,
}

macro_rules! semantic_ref {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }

            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self::new(value)
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self::new(value)
            }
        }
    };
}

semantic_ref!(DatasetModelRef);
semantic_ref!(DatasetFeatureRef);
semantic_ref!(DatasetPlanRef);
semantic_ref!(DatasetCorpusRef);
semantic_ref!(DatasetLanguageModelRef);
semantic_ref!(DatasetLogicRef);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetRationalConcept {
    pub model: DatasetModelRef,
    pub features: Vec<DatasetFeatureRef>,
    pub plan: DatasetPlanRef,
}

impl DatasetRationalConcept {
    pub fn new(
        model: impl Into<DatasetModelRef>,
        features: Vec<DatasetFeatureRef>,
        plan: impl Into<DatasetPlanRef>,
    ) -> Self {
        Self {
            model: model.into(),
            features,
            plan: plan.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetEmpiricalConcept {
    pub corpus: DatasetCorpusRef,
    pub language_model: DatasetLanguageModelRef,
    pub logic: DatasetLogicRef,
}

impl DatasetEmpiricalConcept {
    pub fn new(
        corpus: impl Into<DatasetCorpusRef>,
        language_model: impl Into<DatasetLanguageModelRef>,
        logic: impl Into<DatasetLogicRef>,
    ) -> Self {
        Self {
            corpus: corpus.into(),
            language_model: language_model.into(),
            logic: logic.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetPluginCapabilities {
    pub operations: Vec<DatasetPluginOperation>,
    pub consumes: Vec<DatasetArtifactKind>,
    pub produces: Vec<DatasetArtifactKind>,
    pub polars_lazy: bool,
}

impl DatasetPluginCapabilities {
    pub fn new(operations: Vec<DatasetPluginOperation>) -> Self {
        Self {
            operations,
            consumes: Vec::new(),
            produces: Vec::new(),
            polars_lazy: false,
        }
    }

    pub fn with_consumes(mut self, kinds: Vec<DatasetArtifactKind>) -> Self {
        self.consumes = kinds;
        self
    }

    pub fn with_produces(mut self, kinds: Vec<DatasetArtifactKind>) -> Self {
        self.produces = kinds;
        self
    }

    pub fn with_polars_lazy(mut self, enabled: bool) -> Self {
        self.polars_lazy = enabled;
        self
    }

    pub fn supports(&self, operation: DatasetPluginOperation) -> bool {
        self.operations.contains(&operation)
    }
}

/// The two mediated moments of a Concept admitted as a Dataset plugin.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetConcept {
    pub rational: DatasetRationalConcept,
    pub empirical: DatasetEmpiricalConcept,
}

impl DatasetConcept {
    pub fn new(rational: DatasetRationalConcept, empirical: DatasetEmpiricalConcept) -> Self {
        Self {
            rational,
            empirical,
        }
    }

    pub fn is_determinate(&self) -> bool {
        !self.rational.model.is_empty()
            && !self.rational.features.is_empty()
            && !self.rational.plan.is_empty()
            && !self.empirical.corpus.is_empty()
            && !self.empirical.language_model.is_empty()
            && !self.empirical.logic.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetPluginError {
    pub class: DatasetPluginErrorClass,
    pub message: String,
}

impl DatasetPluginError {
    pub fn new(class: DatasetPluginErrorClass, message: impl Into<String>) -> Self {
        Self {
            class,
            message: message.into(),
        }
    }
}

impl fmt::Display for DatasetPluginError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.class.as_str(), self.message)
    }
}

impl std::error::Error for DatasetPluginError {}

#[derive(Clone)]
pub struct DatasetPluginPayload {
    inner: Arc<dyn Any + Send + Sync>,
}

impl DatasetPluginPayload {
    pub fn typed<T>(value: T) -> Self
    where
        T: Any + Send + Sync,
    {
        Self {
            inner: Arc::new(value),
        }
    }

    pub fn downcast_ref<T>(&self) -> Option<&T>
    where
        T: Any,
    {
        self.inner.as_ref().downcast_ref::<T>()
    }
}

#[derive(Clone)]
pub struct DatasetPluginValidationRequest {
    pub language_id: String,
    pub workflow_id: String,
    pub payload: DatasetPluginPayload,
}

impl DatasetPluginValidationRequest {
    pub fn new(
        language_id: impl Into<String>,
        workflow_id: impl Into<String>,
        payload: DatasetPluginPayload,
    ) -> Self {
        Self {
            language_id: language_id.into(),
            workflow_id: workflow_id.into(),
            payload,
        }
    }
}

#[derive(Clone)]
pub struct DatasetPluginRequest {
    pub plugin_id: String,
    pub operation: DatasetPluginOperation,
    pub concept: DatasetConcept,
    pub input_artifact_ids: Vec<String>,
    pub output_artifact_id: Option<String>,
    pub validation: DatasetPluginValidationRequest,
}

impl DatasetPluginRequest {
    pub fn new(
        plugin_id: impl Into<String>,
        operation: DatasetPluginOperation,
        concept: DatasetConcept,
        validation: DatasetPluginValidationRequest,
    ) -> Self {
        Self {
            plugin_id: plugin_id.into(),
            operation,
            concept,
            input_artifact_ids: Vec::new(),
            output_artifact_id: None,
            validation,
        }
    }

    pub fn with_input_artifact(mut self, artifact_id: impl Into<String>) -> Self {
        self.input_artifact_ids.push(artifact_id.into());
        self
    }

    pub fn with_output_artifact(mut self, artifact_id: impl Into<String>) -> Self {
        self.output_artifact_id = Some(artifact_id.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct DatasetPluginArtifact {
    pub artifact_id: String,
    pub table: GDSDataFrame,
    pub profile: DatasetArtifactProfile,
}

impl DatasetPluginArtifact {
    pub fn new(
        artifact_id: impl Into<String>,
        table: GDSDataFrame,
        profile: DatasetArtifactProfile,
    ) -> Self {
        Self {
            artifact_id: artifact_id.into(),
            table,
            profile,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DatasetPluginDiagnosticLevel {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetPluginDiagnostic {
    pub level: DatasetPluginDiagnosticLevel,
    pub code: String,
    pub message: String,
}

impl DatasetPluginDiagnostic {
    pub fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            level: DatasetPluginDiagnosticLevel::Error,
            code: code.into(),
            message: message.into(),
        }
    }

    pub fn warning(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            level: DatasetPluginDiagnosticLevel::Warning,
            code: code.into(),
            message: message.into(),
        }
    }

    pub fn info(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            level: DatasetPluginDiagnosticLevel::Info,
            code: code.into(),
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetPluginValidationReport {
    pub plugin_id: String,
    pub passed: bool,
    pub summary: String,
    pub diagnostics: Vec<DatasetPluginDiagnostic>,
    pub facts: BTreeMap<String, String>,
}

impl DatasetPluginValidationReport {
    pub fn success(plugin_id: impl Into<String>, summary: impl Into<String>) -> Self {
        Self {
            plugin_id: plugin_id.into(),
            passed: true,
            summary: summary.into(),
            diagnostics: Vec::new(),
            facts: BTreeMap::new(),
        }
    }

    pub fn failure(plugin_id: impl Into<String>, summary: impl Into<String>) -> Self {
        Self {
            plugin_id: plugin_id.into(),
            passed: false,
            summary: summary.into(),
            diagnostics: Vec::new(),
            facts: BTreeMap::new(),
        }
    }

    pub fn with_diagnostic(mut self, diagnostic: DatasetPluginDiagnostic) -> Self {
        self.diagnostics.push(diagnostic);
        self
    }

    pub fn with_fact(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.facts.insert(key.into(), value.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct DatasetPluginResponse {
    pub operation: DatasetPluginOperation,
    pub report: DatasetPluginValidationReport,
    pub artifacts: Vec<DatasetPluginArtifact>,
    pub provenance: BTreeMap<String, String>,
}

impl DatasetPluginResponse {
    pub fn new(operation: DatasetPluginOperation, report: DatasetPluginValidationReport) -> Self {
        Self {
            operation,
            report,
            artifacts: Vec::new(),
            provenance: BTreeMap::new(),
        }
    }

    pub fn with_artifact(mut self, artifact: DatasetPluginArtifact) -> Self {
        self.artifacts.push(artifact);
        self
    }

    pub fn with_provenance(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.provenance.insert(key.into(), value.into());
        self
    }
}

pub trait DatasetLanguagePlugin: Send + Sync {
    fn plugin_id(&self) -> &'static str;

    fn language_id(&self) -> &'static str;

    fn capabilities(&self) -> DatasetPluginCapabilities {
        DatasetPluginCapabilities::new(vec![DatasetPluginOperation::Validate])
    }

    fn validate(
        &self,
        request: &DatasetPluginValidationRequest,
    ) -> Result<DatasetPluginValidationReport, DatasetPluginError>;

    fn execute(
        &self,
        request: &DatasetPluginRequest,
    ) -> Result<DatasetPluginResponse, DatasetPluginError> {
        if !self.capabilities().supports(request.operation) {
            return Err(DatasetPluginError::new(
                DatasetPluginErrorClass::PluginExecution,
                format!(
                    "plugin {} does not support operation {:?}",
                    self.plugin_id(),
                    request.operation
                ),
            ));
        }
        let report = self.validate(&request.validation)?;
        Ok(DatasetPluginResponse::new(request.operation, report)
            .with_provenance("plugin_id", self.plugin_id())
            .with_provenance("workflow_id", request.validation.workflow_id.clone()))
    }
}

#[derive(Clone, Default)]
pub struct DatasetPluginRegistry {
    plugins: BTreeMap<String, Arc<dyn DatasetLanguagePlugin>>,
}

impl DatasetPluginRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.plugins.len()
    }

    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }

    pub fn register(
        &mut self,
        plugin: Arc<dyn DatasetLanguagePlugin>,
    ) -> Result<(), DatasetPluginError> {
        let plugin_id = plugin.plugin_id().to_string();
        if self.plugins.contains_key(&plugin_id) {
            return Err(DatasetPluginError::new(
                DatasetPluginErrorClass::DuplicatePlugin,
                format!("plugin {} is already registered", plugin_id),
            ));
        }
        self.plugins.insert(plugin_id, plugin);
        Ok(())
    }

    pub fn plugin(&self, plugin_id: &str) -> Option<Arc<dyn DatasetLanguagePlugin>> {
        self.plugins.get(plugin_id).cloned()
    }

    pub fn validate(
        &self,
        plugin_id: &str,
        request: &DatasetPluginValidationRequest,
    ) -> Result<DatasetPluginValidationReport, DatasetPluginError> {
        let plugin = self.plugins.get(plugin_id).ok_or_else(|| {
            DatasetPluginError::new(
                DatasetPluginErrorClass::PluginNotFound,
                format!("plugin {} is not registered", plugin_id),
            )
        })?;
        plugin.validate(request)
    }

    pub fn execute(
        &self,
        request: &DatasetPluginRequest,
    ) -> Result<DatasetPluginResponse, DatasetPluginError> {
        let plugin = self.plugins.get(&request.plugin_id).ok_or_else(|| {
            DatasetPluginError::new(
                DatasetPluginErrorClass::PluginNotFound,
                format!("plugin {} is not registered", request.plugin_id),
            )
        })?;
        if !plugin.capabilities().supports(request.operation) {
            return Err(DatasetPluginError::new(
                DatasetPluginErrorClass::PluginExecution,
                format!(
                    "plugin {} does not support operation {:?}",
                    plugin.plugin_id(),
                    request.operation
                ),
            ));
        }
        plugin.execute(request)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::DatasetLanguagePlugin;
    use super::DatasetPluginDiagnostic;
    use super::DatasetPluginPayload;
    use super::DatasetPluginRegistry;
    use super::DatasetPluginValidationReport;
    use super::DatasetPluginValidationRequest;

    struct EchoPlugin;

    impl DatasetLanguagePlugin for EchoPlugin {
        fn plugin_id(&self) -> &'static str {
            "dataset.echo"
        }

        fn language_id(&self) -> &'static str {
            "echo"
        }

        fn validate(
            &self,
            request: &DatasetPluginValidationRequest,
        ) -> Result<DatasetPluginValidationReport, super::DatasetPluginError> {
            let message = request
                .payload
                .downcast_ref::<String>()
                .cloned()
                .unwrap_or_else(|| "unknown".to_string());
            Ok(
                DatasetPluginValidationReport::success(self.plugin_id(), "echo validation")
                    .with_diagnostic(DatasetPluginDiagnostic::info(
                        "echo.payload",
                        message.clone(),
                    ))
                    .with_fact("echo", message),
            )
        }
    }

    #[test]
    fn registry_registers_and_dispatches_plugin() {
        let mut registry = DatasetPluginRegistry::new();
        registry
            .register(Arc::new(EchoPlugin))
            .expect("register should succeed");

        let request = DatasetPluginValidationRequest::new(
            "echo",
            "workflow.alpha",
            DatasetPluginPayload::typed("hello".to_string()),
        );
        let report = registry
            .validate("dataset.echo", &request)
            .expect("validation should succeed");

        assert!(report.passed);
        assert_eq!(report.facts.get("echo"), Some(&"hello".to_string()));
        assert_eq!(report.diagnostics.len(), 1);
    }
}
