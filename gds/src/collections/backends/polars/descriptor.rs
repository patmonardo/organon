//! Static identity and capabilities of the Polars backend.

use crate::config::CollectionsBackend;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PolarsBackendCapabilities {
    pub eager_frames: bool,
    pub lazy_plans: bool,
    pub optimizer_control: bool,
    pub plan_explain: bool,
    pub streaming_collect: bool,
}

impl PolarsBackendCapabilities {
    pub const fn native() -> Self {
        Self {
            eager_frames: true,
            lazy_plans: true,
            optimizer_control: true,
            plan_explain: true,
            streaming_collect: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolarsBackendDescriptor {
    name: String,
    capabilities: PolarsBackendCapabilities,
}

impl PolarsBackendDescriptor {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            capabilities: PolarsBackendCapabilities::native(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub const fn backend(&self) -> CollectionsBackend {
        CollectionsBackend::Polars
    }

    pub const fn capabilities(&self) -> PolarsBackendCapabilities {
        self.capabilities
    }
}

impl Default for PolarsBackendDescriptor {
    fn default() -> Self {
        Self::new("polars")
    }
}
