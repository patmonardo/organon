//! Descriptive mediator vocabulary for Model/Feature realization.
//!
//! These types name the unmanifest mediator layer without changing execution.
//! Corpora, language, and logic can carry these records as provenance when
//! they persist model, feature, and logical-plan artifact capacities.

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FeatureId(pub String);

impl FeatureId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for FeatureId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for FeatureId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MediatorKind {
    Model,
    Feature,
    /// Logical plan mediator (not a raw data execution plan).
    Plan,
}

impl MediatorKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Model => "model",
            Self::Feature => "feature",
            Self::Plan => "plan",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MediatorId {
    kind: MediatorKind,
    value: String,
}

impl MediatorId {
    pub fn new(kind: MediatorKind, value: impl Into<String>) -> Self {
        Self {
            kind,
            value: value.into(),
        }
    }

    pub fn model(value: impl Into<String>) -> Self {
        Self::new(MediatorKind::Model, value)
    }

    pub fn feature(value: impl Into<String>) -> Self {
        Self::new(MediatorKind::Feature, value)
    }

    pub fn plan(value: impl Into<String>) -> Self {
        Self::new(MediatorKind::Plan, value)
    }

    pub fn kind(&self) -> MediatorKind {
        self.kind
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MediatorCapacity(pub usize);

impl MediatorCapacity {
    pub fn new(value: usize) -> Self {
        Self(value)
    }

    pub fn get(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MediatorBindingId(pub String);

impl MediatorBindingId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for MediatorBindingId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for MediatorBindingId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ArtifactKind {
    Corpus,
    Language,
    Logic,
    Other,
}

impl ArtifactKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Corpus => "corpus",
            Self::Language => "language",
            Self::Logic => "logic",
            Self::Other => "other",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MediatorProvenance {
    mediator_id: MediatorId,
    binding_id: Option<MediatorBindingId>,
    artifact_kind: Option<ArtifactKind>,
    layer: String,
    source: String,
    version: String,
    derivation: String,
}

impl MediatorProvenance {
    pub fn new(
        mediator_id: MediatorId,
        layer: impl Into<String>,
        source: impl Into<String>,
        version: impl Into<String>,
        derivation: impl Into<String>,
    ) -> Self {
        Self {
            mediator_id,
            binding_id: None,
            artifact_kind: None,
            layer: layer.into(),
            source: source.into(),
            version: version.into(),
            derivation: derivation.into(),
        }
    }

    pub fn with_binding_id(mut self, binding_id: impl Into<MediatorBindingId>) -> Self {
        self.binding_id = Some(binding_id.into());
        self
    }

    pub fn with_artifact_kind(mut self, artifact_kind: ArtifactKind) -> Self {
        self.artifact_kind = Some(artifact_kind);
        self
    }

    pub fn mediator_id(&self) -> &MediatorId {
        &self.mediator_id
    }

    pub fn binding_id(&self) -> Option<&MediatorBindingId> {
        self.binding_id.as_ref()
    }

    pub fn artifact_kind(&self) -> Option<ArtifactKind> {
        self.artifact_kind
    }

    pub fn layer(&self) -> &str {
        &self.layer
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn derivation(&self) -> &str {
        &self.derivation
    }
}
