use crate::shell::ShellComponentDescriptor;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FormServiceExecutionState {
    Actual,
    Bindable,
    Planned,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShellDaemonRuntimeProfile {
    pub process_model: String,
    pub supervision_policy: String,
    pub heartbeat_interval_ms: u64,
    pub checkpoint_policy: String,
}

impl ShellDaemonRuntimeProfile {
    pub fn new(
        process_model: impl Into<String>,
        supervision_policy: impl Into<String>,
        heartbeat_interval_ms: u64,
        checkpoint_policy: impl Into<String>,
    ) -> Self {
        Self {
            process_model: process_model.into(),
            supervision_policy: supervision_policy.into(),
            heartbeat_interval_ms,
            checkpoint_policy: checkpoint_policy.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FormServiceMachine {
    pub service_id: String,
    pub purpose: String,
    pub execution_state: FormServiceExecutionState,
    pub runtime_binding: String,
    pub components: Vec<ShellComponentDescriptor>,
    pub daemon_runtime: Option<ShellDaemonRuntimeProfile>,
}

impl FormServiceMachine {
    pub fn new(
        service_id: impl Into<String>,
        purpose: impl Into<String>,
        execution_state: FormServiceExecutionState,
        runtime_binding: impl Into<String>,
        components: Vec<ShellComponentDescriptor>,
    ) -> Self {
        Self {
            service_id: service_id.into(),
            purpose: purpose.into(),
            execution_state,
            runtime_binding: runtime_binding.into(),
            components,
            daemon_runtime: None,
        }
    }

    pub fn with_daemon_runtime(mut self, daemon_runtime: ShellDaemonRuntimeProfile) -> Self {
        self.daemon_runtime = Some(daemon_runtime);
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FormServiceManifest {
    pub activated_services: Vec<String>,
    pub machines: Vec<FormServiceMachine>,
    pub unresolved_patterns: Vec<String>,
}

impl FormServiceManifest {
    pub fn new(machines: Vec<FormServiceMachine>, unresolved_patterns: Vec<String>) -> Self {
        let activated_services = machines
            .iter()
            .map(|machine| machine.service_id.clone())
            .collect();

        Self {
            activated_services,
            machines,
            unresolved_patterns,
        }
    }
}
