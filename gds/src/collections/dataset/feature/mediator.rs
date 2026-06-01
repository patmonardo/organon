//! Feature-stage mediation and neutral identifiers for Dialectical Learning.
//!
//! Feature is the middle mediation moment: it receives conditioning from Model
//! and transforms it toward Plan. The neutral identifiers in this file are used
//! by the three mediator modules, but the layer-specific binding logic remains
//! in Model, Feature, and Plan rather than in a single shared mediator base.

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

    /// Dialectical ordinal of mediator evolution: Model -> Feature -> Plan.
    pub fn stage(self) -> u8 {
        match self {
            Self::Model => 0,
            Self::Feature => 1,
            Self::Plan => 2,
        }
    }

    /// Immediate next mediator stage in the rational evolution.
    pub fn next(self) -> Option<Self> {
        match self {
            Self::Model => Some(Self::Feature),
            Self::Feature => Some(Self::Plan),
            Self::Plan => None,
        }
    }

    /// Immediate previous mediator stage in the rational evolution.
    pub fn previous(self) -> Option<Self> {
        match self {
            Self::Model => None,
            Self::Feature => Some(Self::Model),
            Self::Plan => Some(Self::Feature),
        }
    }

    /// Canonical evolution path from this stage up to Plan.
    pub fn evolution_path_from(self) -> Vec<Self> {
        let mut out = vec![self];
        let mut cursor = self;
        while let Some(next) = cursor.next() {
            out.push(next);
            cursor = next;
        }
        out
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

/// Binding relation on the rational mediator side.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DialecticalBindingRelation {
    Conditions,
    Realizes,
    Projects,
    Synthesizes,
    Grounds,
    Other,
}

impl DialecticalBindingRelation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Conditions => "conditions",
            Self::Realizes => "realizes",
            Self::Projects => "projects",
            Self::Synthesizes => "synthesizes",
            Self::Grounds => "grounds",
            Self::Other => "other",
        }
    }
}

/// Rational-side binding record between mediator moments.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DialecticalBinding {
    pub from: MediatorId,
    pub to: MediatorId,
    pub relation: DialecticalBindingRelation,
    pub label: Option<String>,
}

impl DialecticalBinding {
    pub fn new(from: MediatorId, to: MediatorId, relation: DialecticalBindingRelation) -> Self {
        Self {
            from,
            to,
            relation,
            label: None,
        }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

/// Frame-level container for rational mediation records.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RationalMediatingFrame {
    bindings: Vec<DialecticalBinding>,
    provenance: Vec<MediatorProvenance>,
}

impl RationalMediatingFrame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_binding(mut self, binding: DialecticalBinding) -> Self {
        self.bindings.push(binding);
        self
    }

    pub fn with_provenance(mut self, provenance: MediatorProvenance) -> Self {
        self.provenance.push(provenance);
        self
    }

    pub fn bindings(&self) -> &[DialecticalBinding] {
        &self.bindings
    }

    pub fn provenance(&self) -> &[MediatorProvenance] {
        &self.provenance
    }

    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty() && self.provenance.is_empty()
    }
}

/// Binding concept levels across the staged rational arc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BindingConceptLevel {
    Model,
    Feature,
    Plan,
}

impl BindingConceptLevel {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Model => "model",
            Self::Feature => "feature",
            Self::Plan => "plan",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FeatureBindingOrientation {
    FromModel,
    TowardPlan,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FeatureMediatingBinding {
    pub binding: DialecticalBinding,
    pub orientation: FeatureBindingOrientation,
}

impl FeatureMediatingBinding {
    pub fn from_model(model_id: MediatorId, feature_id: MediatorId) -> Self {
        Self {
            binding: DialecticalBinding::new(
                model_id,
                feature_id,
                DialecticalBindingRelation::Conditions,
            ),
            orientation: FeatureBindingOrientation::FromModel,
        }
    }

    pub fn toward_plan(feature_id: MediatorId, plan_id: MediatorId) -> Self {
        Self {
            binding: DialecticalBinding::new(
                feature_id,
                plan_id,
                DialecticalBindingRelation::Projects,
            ),
            orientation: FeatureBindingOrientation::TowardPlan,
        }
    }
}

/// Manifest-side record that materializes mediation into artifacts.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ManifestBindingRecord {
    pub artifact_kind: ArtifactKind,
    pub artifact_id: String,
    pub relation: DialecticalBindingRelation,
}

impl ManifestBindingRecord {
    pub fn new(
        artifact_kind: ArtifactKind,
        artifact_id: impl Into<String>,
        relation: DialecticalBindingRelation,
    ) -> Self {
        Self {
            artifact_kind,
            artifact_id: artifact_id.into(),
            relation,
        }
    }
}

/// Manifesting-side frame: where rational bindings appear as persisted artifacts.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ManifestingFrame {
    records: Vec<ManifestBindingRecord>,
}

impl ManifestingFrame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_record(mut self, record: ManifestBindingRecord) -> Self {
        self.records.push(record);
        self
    }

    pub fn records(&self) -> &[ManifestBindingRecord] {
        &self.records
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}

/// Third moment: scientific learning as unity of mediating + manifesting framings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScientificLearningFrame {
    level: BindingConceptLevel,
    mediating: RationalMediatingFrame,
    manifesting: ManifestingFrame,
}

impl ScientificLearningFrame {
    pub fn new(level: BindingConceptLevel) -> Self {
        Self {
            level,
            mediating: RationalMediatingFrame::new(),
            manifesting: ManifestingFrame::new(),
        }
    }

    pub fn with_mediating_binding(mut self, binding: DialecticalBinding) -> Self {
        self.mediating = self.mediating.with_binding(binding);
        self
    }

    pub fn with_manifest_record(mut self, record: ManifestBindingRecord) -> Self {
        self.manifesting = self.manifesting.with_record(record);
        self
    }

    pub fn level(&self) -> BindingConceptLevel {
        self.level
    }

    pub fn mediating(&self) -> &RationalMediatingFrame {
        &self.mediating
    }

    pub fn manifesting(&self) -> &ManifestingFrame {
        &self.manifesting
    }

    pub fn is_scientific_learning(&self) -> bool {
        !self.mediating.is_empty() && !self.manifesting.is_empty()
    }
}

/// Feature-stage mediation (middle rational moment).
///
/// Feature mediation receives conditioning from Model and projects toward Plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureMediationMoment {
    pub feature_id: FeatureId,
    pub grammar_anchor: Option<String>,
    pub bindings: Vec<DialecticalBinding>,
    pub mediating_bindings: Vec<FeatureMediatingBinding>,
}

impl FeatureMediationMoment {
    pub fn new(feature_id: impl Into<FeatureId>) -> Self {
        Self {
            feature_id: feature_id.into(),
            grammar_anchor: None,
            bindings: Vec::new(),
            mediating_bindings: Vec::new(),
        }
    }

    pub fn with_grammar_anchor(mut self, grammar_anchor: impl Into<String>) -> Self {
        self.grammar_anchor = Some(grammar_anchor.into());
        self
    }

    pub fn mediator_id(&self) -> MediatorId {
        MediatorId::feature(self.feature_id.0.clone())
    }

    pub fn conditioned_by_model(mut self, model_id: impl Into<String>) -> Self {
        let binding =
            FeatureMediatingBinding::from_model(MediatorId::model(model_id), self.mediator_id());
        self.bindings.push(binding.binding.clone());
        self.mediating_bindings.push(binding);
        self
    }

    pub fn project_to_plan(mut self, plan_id: impl Into<String>) -> Self {
        let binding =
            FeatureMediatingBinding::toward_plan(self.mediator_id(), MediatorId::plan(plan_id));
        self.bindings.push(binding.binding.clone());
        self.mediating_bindings.push(binding);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::dataset::model::{ModelBindingNature, ModelMediationMoment};
    use crate::collections::dataset::plan::{PlanBindingAxis, PlanMediationMoment};

    #[test]
    fn mediator_kind_exposes_dialectical_evolution_path() {
        let path = MediatorKind::Model.evolution_path_from();
        assert_eq!(
            path,
            vec![
                MediatorKind::Model,
                MediatorKind::Feature,
                MediatorKind::Plan
            ]
        );
        assert_eq!(MediatorKind::Feature.previous(), Some(MediatorKind::Model));
        assert_eq!(MediatorKind::Plan.next(), None);
    }

    #[test]
    fn rational_mediating_frame_collects_bindings_and_provenance() {
        let binding = DialecticalBinding::new(
            MediatorId::model("model:1"),
            MediatorId::feature("feature:1"),
            DialecticalBindingRelation::Realizes,
        )
        .with_label("box1");

        let provenance = MediatorProvenance::new(
            MediatorId::plan("plan:1"),
            "rational",
            "unit-test",
            "v1",
            "fixture",
        )
        .with_artifact_kind(ArtifactKind::Logic);

        let frame = RationalMediatingFrame::new()
            .with_binding(binding)
            .with_provenance(provenance);

        assert_eq!(frame.bindings().len(), 1);
        assert_eq!(frame.provenance().len(), 1);
        assert!(!frame.is_empty());
    }

    #[test]
    fn feature_mediation_is_middle_moment() {
        let mediation = FeatureMediationMoment::new("feature:agreement")
            .with_grammar_anchor("grammar:featstruct")
            .conditioned_by_model("model:syntax")
            .project_to_plan("plan:inference");

        assert_eq!(mediation.bindings.len(), 2);
        assert_eq!(
            mediation.bindings[0].relation,
            DialecticalBindingRelation::Conditions
        );
        assert_eq!(
            mediation.bindings[1].relation,
            DialecticalBindingRelation::Projects
        );
        assert_eq!(
            mediation.mediating_bindings[0].orientation,
            FeatureBindingOrientation::FromModel
        );
        assert_eq!(
            mediation.mediating_bindings[1].orientation,
            FeatureBindingOrientation::TowardPlan
        );
    }

    #[test]
    fn scientific_learning_requires_both_sides() {
        let learning = ScientificLearningFrame::new(BindingConceptLevel::Plan)
            .with_mediating_binding(DialecticalBinding::new(
                MediatorId::feature("feature:agreement"),
                MediatorId::plan("plan:inference"),
                DialecticalBindingRelation::Synthesizes,
            ))
            .with_manifest_record(ManifestBindingRecord::new(
                ArtifactKind::Logic,
                "artifact:logic:inference-trace",
                DialecticalBindingRelation::Grounds,
            ));

        assert!(learning.is_scientific_learning());
        assert_eq!(learning.level(), BindingConceptLevel::Plan);
        assert_eq!(learning.mediating().bindings().len(), 1);
        assert_eq!(learning.manifesting().records().len(), 1);
    }

    #[test]
    fn dialectical_learning_uses_three_distinct_mediator_modules() {
        let model = ModelMediationMoment::new("model:syntax")
            .with_schema_anchor("schema:agreement")
            .condition_feature("feature:agreement");
        let feature = FeatureMediationMoment::new("feature:agreement")
            .with_grammar_anchor("grammar:featstruct")
            .conditioned_by_model("model:syntax")
            .project_to_plan("plan:agreement-logic");
        let plan = PlanMediationMoment::new("plan:agreement-logic")
            .with_principle_anchor("principle:model-feature-plan")
            .synthesize_feature("feature:agreement")
            .manifest_logic("logic:rule:agreement", DialecticalBindingRelation::Grounds);

        assert_eq!(
            model.feature_bindings[0].nature,
            ModelBindingNature::ConditionsFeature
        );
        assert_eq!(
            feature.mediating_bindings[0].orientation,
            FeatureBindingOrientation::FromModel
        );
        assert_eq!(
            feature.mediating_bindings[1].orientation,
            FeatureBindingOrientation::TowardPlan
        );
        assert_eq!(
            plan.feature_bindings[0].axis,
            PlanBindingAxis::VerticalFeature
        );
        assert_eq!(
            plan.logic_bindings[0].axis,
            PlanBindingAxis::HorizontalLogic
        );
        assert_eq!(
            plan.logic_bindings[0].record.artifact_kind,
            ArtifactKind::Logic
        );
    }
}
