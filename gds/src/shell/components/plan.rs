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
use super::ShellComponentId;
use super::ShellComponentMode;
use super::ShellDijkstraCallBuilder;

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
}

impl ShellComponentCallBuilder {
    pub fn with_input(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.call = self.call.with_input(key, value);
        self
    }

    pub fn finish(self) -> ShellComponentPlan {
        self.plan.push(self.call)
    }
}

/// Ordered, inspectable Shell Component calls awaiting runtime mediation.
#[derive(Debug, Clone, PartialEq)]
pub struct ShellComponentPlan {
    origin: ShellAddress,
    calls: Vec<ShellComponentCall>,
}

impl ShellComponentPlan {
    pub fn new(origin: ShellAddress) -> Self {
        Self {
            origin,
            calls: Vec::new(),
        }
    }

    pub fn origin(&self) -> ShellAddress {
        self.origin
    }

    pub fn calls(&self) -> &[ShellComponentCall] {
        &self.calls
    }

    pub fn len(&self) -> usize {
        self.calls.len()
    }

    pub fn is_empty(&self) -> bool {
        self.calls.is_empty()
    }

    pub(crate) fn push(mut self, call: ShellComponentCall) -> Self {
        self.calls.push(call);
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
        })
    }

    pub fn bfs(self, source: u64) -> ShellBfsCallBuilder {
        ShellBfsCallBuilder::new(self, source)
    }

    pub fn dijkstra(self, source: u64) -> ShellDijkstraCallBuilder {
        ShellDijkstraCallBuilder::new(self, source)
    }
}
