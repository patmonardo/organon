use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConceptTriad {
    ModelFeaturePlan,
    EntityPropertyRelation,
    ShapeContextMorph,
}

impl ConceptTriad {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ModelFeaturePlan => "model-feature-plan",
            Self::EntityPropertyRelation => "entity-property-relation",
            Self::ShapeContextMorph => "shape-context-morph",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CognitionMode {
    Rational,
    Empirical,
}

impl CognitionMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Rational => "rational",
            Self::Empirical => "empirical",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceObservation {
    pub feature_anchor: String,
    pub evidence: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LawOfAppearance {
    pub observations: Vec<AppearanceObservation>,
}

impl LawOfAppearance {
    pub fn add_observation(
        mut self,
        feature_anchor: impl Into<String>,
        evidence: impl Into<String>,
    ) -> Self {
        self.observations.push(AppearanceObservation {
            feature_anchor: feature_anchor.into(),
            evidence: evidence.into(),
        });
        self
    }

    pub fn is_empty(&self) -> bool {
        self.observations.is_empty()
    }

    pub fn mock_from_feature_anchors<I>(feature_anchors: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<String>,
    {
        feature_anchors
            .into_iter()
            .fold(Self::default(), |law, feature| {
                let feature = feature.into();
                law.add_observation(feature.clone(), format!("mock-observation:{feature}"))
            })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanPrinciple {
    pub triad: ConceptTriad,
    pub mode: CognitionMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub law_of_appearance: Option<LawOfAppearance>,
}

impl PlanPrinciple {
    pub fn rational(triad: ConceptTriad) -> Self {
        Self {
            triad,
            mode: CognitionMode::Rational,
            law_of_appearance: None,
        }
    }

    pub fn with_law_of_appearance(mut self, law_of_appearance: LawOfAppearance) -> Self {
        self.mode = CognitionMode::Empirical;
        self.law_of_appearance = if law_of_appearance.is_empty() {
            None
        } else {
            Some(law_of_appearance)
        };
        self
    }

    pub fn is_empirical_transition(&self) -> bool {
        self.mode == CognitionMode::Empirical
            && self
                .law_of_appearance
                .as_ref()
                .is_some_and(|law| !law.is_empty())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanPrincipleReport {
    pub triad: String,
    pub mode: String,
    pub empirical_transition: bool,
    pub observation_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observations: Option<Vec<AppearanceObservation>>,
}

impl From<&PlanPrinciple> for PlanPrincipleReport {
    fn from(value: &PlanPrinciple) -> Self {
        let observations = value
            .law_of_appearance
            .as_ref()
            .and_then(|law| (!law.observations.is_empty()).then(|| law.observations.clone()));
        let observation_count = observations.as_ref().map_or(0, |rows| rows.len());
        Self {
            triad: value.triad.as_str().to_string(),
            mode: value.mode.as_str().to_string(),
            empirical_transition: value.is_empirical_transition(),
            observation_count,
            observations,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlanSynthesis {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_anchor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_anchor: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub feature_anchors: Vec<String>,
}

impl PlanSynthesis {
    pub fn is_empty(&self) -> bool {
        self.model_anchor.is_none()
            && self.image_anchor.is_none()
            && self.feature_anchors.is_empty()
    }

    pub fn is_unified(&self) -> bool {
        self.image_anchor.is_some() && !self.feature_anchors.is_empty()
    }

    pub fn with_model_anchor(mut self, model_anchor: impl Into<String>) -> Self {
        self.model_anchor = Some(model_anchor.into());
        self
    }

    pub fn with_image_anchor(mut self, image_anchor: impl Into<String>) -> Self {
        self.image_anchor = Some(image_anchor.into());
        self
    }

    pub fn with_feature_anchor(mut self, feature_anchor: impl Into<String>) -> Self {
        let feature_anchor = feature_anchor.into();
        if !self
            .feature_anchors
            .iter()
            .any(|existing| existing == &feature_anchor)
        {
            self.feature_anchors.push(feature_anchor);
        }
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanSynthesisReport {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_anchor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_anchor: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub feature_anchors: Vec<String>,
    pub unified: bool,
}

impl From<&PlanSynthesis> for PlanSynthesisReport {
    fn from(value: &PlanSynthesis) -> Self {
        Self {
            model_anchor: value.model_anchor.clone(),
            image_anchor: value.image_anchor.clone(),
            feature_anchors: value.feature_anchors.clone(),
            unified: value.is_unified(),
        }
    }
}
