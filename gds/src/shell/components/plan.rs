//! Shell-local operational plans.
//!
//! This module defines what `Plan` means inside the current Shell DSL: ordered,
//! inspectable component calls. It does not determine the richer semantic Plan
//! that Dataset may later lower from Corpus, language-model, Logic, or GML
//! mediation.

use crate::shell::ShellAddress;
use serde_json::Value;

use super::builtin_component;
use super::ShellBfsCallBuilder;
use super::ShellComponentCall;
use super::ShellComponentDescriptor;
use super::ShellComponentExecutionKind;
use super::ShellComponentId;
use super::ShellComponentMode;
use super::ShellDijkstraCallBuilder;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellPlanStepRole {
    Node,
    Feature,
    Training,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ShellComponentPlanError {
    #[error("unknown Shell Component `{0}`")]
    UnknownComponent(String),

    #[error("Shell Component `{component}` does not support mode `{mode:?}`")]
    UnsupportedMode {
        component: ShellComponentId,
        mode: ShellComponentMode,
    },
}

/// Generic authoring surface for any registered Shell Component.
#[derive(Debug, Clone)]
pub struct ShellComponentCallBuilder {
    plan: ShellComponentPlan,
    call: ShellComponentCall,
    role: Option<ShellPlanStepRole>,
}

impl ShellComponentCallBuilder {
    pub fn with_input(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.call = self.call.with_input(key, value);
        self
    }

    pub fn finish(self) -> ShellComponentPlan {
        self.plan
            .push_with_role(self.call, self.role.unwrap_or(ShellPlanStepRole::Other))
    }
}

/// Ordered, inspectable Shell Component calls awaiting runtime mediation.
#[derive(Debug, Clone, PartialEq)]
pub struct ShellComponentPlan {
    origin: ShellAddress,
    calls: Vec<ShellComponentCall>,
    step_roles: Vec<ShellPlanStepRole>,
}

impl ShellComponentPlan {
    pub fn new(origin: ShellAddress) -> Self {
        Self {
            origin,
            calls: Vec::new(),
            step_roles: Vec::new(),
        }
    }

    pub fn origin(&self) -> ShellAddress {
        self.origin
    }

    pub fn calls(&self) -> &[ShellComponentCall] {
        &self.calls
    }

    pub fn descriptors(&self) -> Vec<&'static ShellComponentDescriptor> {
        self.calls
            .iter()
            .filter_map(|call| call.descriptor())
            .collect()
    }

    pub fn execution_kinds(&self) -> Vec<ShellComponentExecutionKind> {
        self.calls
            .iter()
            .filter_map(|call| call.execution_kind())
            .collect()
    }

    pub fn has_algorithm_components(&self) -> bool {
        self.execution_kinds()
            .iter()
            .any(|kind| *kind == ShellComponentExecutionKind::Algorithm)
    }

    pub fn has_pipeline_components(&self) -> bool {
        self.execution_kinds()
            .iter()
            .any(|kind| *kind == ShellComponentExecutionKind::Pipeline)
    }

    pub fn has_store_api_components(&self) -> bool {
        self.execution_kinds()
            .iter()
            .any(|kind| *kind == ShellComponentExecutionKind::StoreApi)
    }

    pub fn len(&self) -> usize {
        self.calls.len()
    }

    pub fn is_empty(&self) -> bool {
        self.calls.is_empty()
    }

    pub fn step_roles(&self) -> Vec<&'static str> {
        self.step_roles
            .iter()
            .map(|role| match role {
                ShellPlanStepRole::Node => "node",
                ShellPlanStepRole::Feature => "feature",
                ShellPlanStepRole::Training => "training",
                ShellPlanStepRole::Other => "other",
            })
            .collect()
    }

    pub fn node_steps(&self) -> Vec<&ShellComponentCall> {
        self.calls
            .iter()
            .zip(self.step_roles.iter())
            .filter_map(|(call, role)| (*role == ShellPlanStepRole::Node).then_some(call))
            .collect()
    }

    pub fn feature_steps(&self) -> Vec<&ShellComponentCall> {
        self.calls
            .iter()
            .zip(self.step_roles.iter())
            .filter_map(|(call, role)| (*role == ShellPlanStepRole::Feature).then_some(call))
            .collect()
    }

    pub fn training_steps(&self) -> Vec<&ShellComponentCall> {
        self.calls
            .iter()
            .zip(self.step_roles.iter())
            .filter_map(|(call, role)| (*role == ShellPlanStepRole::Training).then_some(call))
            .collect()
    }

    pub(crate) fn push_with_role(
        mut self,
        call: ShellComponentCall,
        role: ShellPlanStepRole,
    ) -> Self {
        self.calls.push(call);
        self.step_roles.push(role);
        self
    }

    pub fn node_component(
        self,
        name: impl AsRef<str>,
        mode: ShellComponentMode,
    ) -> Result<ShellComponentCallBuilder, ShellComponentPlanError> {
        self.with_role_component(ShellPlanStepRole::Node, name, mode)
    }

    pub fn feature_component(
        self,
        name: impl AsRef<str>,
        mode: ShellComponentMode,
    ) -> Result<ShellComponentCallBuilder, ShellComponentPlanError> {
        self.with_role_component(ShellPlanStepRole::Feature, name, mode)
    }

    pub fn training_component(
        self,
        name: impl AsRef<str>,
        mode: ShellComponentMode,
    ) -> Result<ShellComponentCallBuilder, ShellComponentPlanError> {
        self.with_role_component(ShellPlanStepRole::Training, name, mode)
    }

    fn with_role_component(
        self,
        role: ShellPlanStepRole,
        name: impl AsRef<str>,
        mode: ShellComponentMode,
    ) -> Result<ShellComponentCallBuilder, ShellComponentPlanError> {
        let name = name.as_ref();
        let component = builtin_component(name)
            .ok_or_else(|| ShellComponentPlanError::UnknownComponent(name.to_string()))?;
        let descriptor = component.descriptor();

        if !descriptor.supports(mode) {
            return Err(ShellComponentPlanError::UnsupportedMode {
                component: descriptor.id,
                mode,
            });
        }

        Ok(ShellComponentCallBuilder {
            plan: self,
            call: component.call(mode),
            role: Some(role),
        })
    }

    pub fn with_call(mut self, call: ShellComponentCall) -> Self {
        self = self.push_with_role(call, ShellPlanStepRole::Other);
        self
    }

    pub fn component(
        self,
        name: impl AsRef<str>,
        mode: ShellComponentMode,
    ) -> Result<ShellComponentCallBuilder, ShellComponentPlanError> {
        let name = name.as_ref();
        let component = builtin_component(name)
            .ok_or_else(|| ShellComponentPlanError::UnknownComponent(name.to_string()))?;
        let descriptor = component.descriptor();

        if !descriptor.supports(mode) {
            return Err(ShellComponentPlanError::UnsupportedMode {
                component: descriptor.id,
                mode,
            });
        }

        Ok(ShellComponentCallBuilder {
            plan: self,
            call: component.call(mode),
            role: None,
        })
    }

    pub fn bfs(self, source: u64) -> ShellBfsCallBuilder {
        ShellBfsCallBuilder::new(self, source)
    }

    pub fn dijkstra(self, source: u64) -> ShellDijkstraCallBuilder {
        ShellDijkstraCallBuilder::new(self, source)
    }
}
