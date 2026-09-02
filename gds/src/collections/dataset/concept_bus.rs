//! Typed protocol carried between the cold Dataset ORB and hot Oculus.

use crate::collections::dataset::core::{DatasetPluginRequest, DatasetPluginResponse};

/// Identity shared by one Concept mediation and its receipt.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConceptCorrelationId(String);

impl ConceptCorrelationId {
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

impl From<&str> for ConceptCorrelationId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for ConceptCorrelationId {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

/// Ordinal identity for a revision of the same mediated Concept.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConceptRevision(u64);

impl ConceptRevision {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}

/// A determinate Concept submitted to Oculus over the personal Concept Bus.
#[derive(Clone)]
pub struct ConceptEnvelope {
    correlation_id: ConceptCorrelationId,
    revision: ConceptRevision,
    request: DatasetPluginRequest,
}

impl ConceptEnvelope {
    pub fn new(
        correlation_id: impl Into<ConceptCorrelationId>,
        request: DatasetPluginRequest,
    ) -> Self {
        Self {
            correlation_id: correlation_id.into(),
            revision: ConceptRevision::default(),
            request,
        }
    }

    /// Build an envelope whose identity follows the request workflow.
    pub fn from_request(request: DatasetPluginRequest) -> Self {
        let correlation_id = request.validation.workflow_id.clone();
        Self::new(correlation_id, request)
    }

    pub fn with_revision(mut self, revision: impl Into<ConceptRevision>) -> Self {
        self.revision = revision.into();
        self
    }

    pub fn correlation_id(&self) -> &ConceptCorrelationId {
        &self.correlation_id
    }

    pub const fn revision(&self) -> ConceptRevision {
        self.revision
    }

    pub fn request(&self) -> &DatasetPluginRequest {
        &self.request
    }

    pub fn into_request(self) -> DatasetPluginRequest {
        self.request
    }
}

impl From<u64> for ConceptRevision {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

/// Oculus judgment returned after the ORB has completed cold dispatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConceptJudgment {
    Admitted,
    Refused,
}

impl ConceptJudgment {
    pub const fn is_admitted(self) -> bool {
        matches!(self, Self::Admitted)
    }
}

/// Receipt returned over the Concept Bus with identity and execution evidence.
#[derive(Debug, Clone)]
pub struct ConceptReceipt {
    correlation_id: ConceptCorrelationId,
    revision: ConceptRevision,
    judgment: ConceptJudgment,
    response: DatasetPluginResponse,
    produced_artifact_ids: Vec<String>,
    mediation_trace: Vec<String>,
}

impl ConceptReceipt {
    pub(crate) fn new(
        envelope: &ConceptEnvelope,
        response: DatasetPluginResponse,
        mediation_trace: Vec<String>,
    ) -> Self {
        let judgment = if response.report.passed {
            ConceptJudgment::Admitted
        } else {
            ConceptJudgment::Refused
        };
        let produced_artifact_ids = response
            .artifacts
            .iter()
            .map(|artifact| artifact.artifact_id.clone())
            .collect();

        Self {
            correlation_id: envelope.correlation_id.clone(),
            revision: envelope.revision,
            judgment,
            response,
            produced_artifact_ids,
            mediation_trace,
        }
    }

    pub fn correlation_id(&self) -> &ConceptCorrelationId {
        &self.correlation_id
    }

    pub const fn revision(&self) -> ConceptRevision {
        self.revision
    }

    pub const fn judgment(&self) -> ConceptJudgment {
        self.judgment
    }

    pub fn response(&self) -> &DatasetPluginResponse {
        &self.response
    }

    pub fn produced_artifact_ids(&self) -> &[String] {
        &self.produced_artifact_ids
    }

    pub fn mediation_trace(&self) -> &[String] {
        &self.mediation_trace
    }

    pub fn into_response(self) -> DatasetPluginResponse {
        self.response
    }
}
