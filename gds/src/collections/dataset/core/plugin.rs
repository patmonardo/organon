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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DatasetPluginErrorClass {
    DuplicatePlugin,
    PluginNotFound,
    InvalidPayload,
    PluginExecution,
}

impl DatasetPluginErrorClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DuplicatePlugin => "DuplicatePlugin",
            Self::PluginNotFound => "PluginNotFound",
            Self::InvalidPayload => "InvalidPayload",
            Self::PluginExecution => "PluginExecution",
        }
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

pub trait DatasetLanguagePlugin: Send + Sync {
    fn plugin_id(&self) -> &'static str;

    fn language_id(&self) -> &'static str;

    fn validate(
        &self,
        request: &DatasetPluginValidationRequest,
    ) -> Result<DatasetPluginValidationReport, DatasetPluginError>;
}

#[derive(Default)]
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
