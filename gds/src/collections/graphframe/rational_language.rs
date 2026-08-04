//! Graph Theory as the first Rational Language client of Dataset.

use crate::collections::dataset::feature::Feature;
use crate::collections::dataset::model::ModelId;
use crate::collections::dataset::model::ModelKind;
use crate::collections::dataset::model::ModelSpec;
use crate::collections::dataset::model::ModelView;
use crate::collections::dataset::plan::CognitionMode;
use crate::collections::dataset::plan::ConceptTriad;
use crate::collections::dataset::plan::Plan;
use crate::collections::graphframe::expr::GraphFrameExpr;
use crate::collections::graphframe::feature_grammar::GraphFeatureGrammarChecked;
use crate::collections::graphframe::feature_grammar::GraphFeatureStratum;
use crate::collections::graphframe::feature_grammar::GraphFeatureValueType;
use crate::collections::graphframe::frame::GraphFrame;
use crate::collections::graphframe::frame::GraphFrameError;

#[derive(Debug, thiserror::Error)]
pub enum GraphRationalLanguageError {
    #[error("graph semantic lowering requires exactly one model declaration")]
    InvalidModelDeclaration,

    #[error("graph semantic lowering requires exactly one grammar declaration")]
    InvalidGrammarDeclaration,

    #[error("graph semantic lowering requires exactly one plan declaration")]
    InvalidPlanDeclaration,

    #[error("declared graph grammar '{declared}' does not match checked grammar '{checked}'")]
    GrammarNameMismatch { declared: String, checked: String },

    #[error(
        "declared graph grammar version '{declared}' does not match checked version '{checked}'"
    )]
    GrammarVersionMismatch { declared: String, checked: String },

    #[error("graph semantic lowering requires exactly one Graph feature rule, found {0}")]
    InvalidGraphFeatureCount(usize),

    #[error("graph feature requires a stable feature id")]
    MissingFeatureId,

    #[error("graph feature plan requires a stable plan id")]
    MissingPlanId,

    #[error("graph feature plan requires a model anchor")]
    MissingModelAnchor,

    #[error("graph feature plan model anchor '{actual}' does not match model '{expected}'")]
    ModelAnchorMismatch { expected: String, actual: String },

    #[error("graph feature plan does not anchor its owning feature '{0}'")]
    MissingFeatureAnchor(String),

    #[error("graph feature plan requires a Rational Model-Feature-Plan principle")]
    InvalidRationalPrinciple,

    #[error("graph feature grammar does not declare Graph.density as a scalar feature")]
    MissingDensityRule,

    #[error(transparent)]
    GraphFrame(#[from] GraphFrameError),
}

#[derive(Debug, Clone)]
pub struct GraphSemanticLowering {
    model: ModelSpec,
    grammar: GraphFeatureGrammarChecked,
    feature: Feature,
}

impl GraphSemanticLowering {
    pub fn model(&self) -> &ModelSpec {
        &self.model
    }

    pub fn grammar(&self) -> &GraphFeatureGrammarChecked {
        &self.grammar
    }

    pub fn feature(&self) -> &Feature {
        &self.feature
    }
}

pub fn lower_graph_semantics(
    expressions: &[GraphFrameExpr],
    grammar: &GraphFeatureGrammarChecked,
) -> Result<GraphSemanticLowering, GraphRationalLanguageError> {
    let models = expressions
        .iter()
        .filter_map(|expression| match expression {
            GraphFrameExpr::Model(model) => Some(model),
            _ => None,
        })
        .collect::<Vec<_>>();
    if models.len() != 1 {
        return Err(GraphRationalLanguageError::InvalidModelDeclaration);
    }

    let grammars = expressions
        .iter()
        .filter_map(|expression| match expression {
            GraphFrameExpr::FeatureGrammar(grammar) => Some(grammar),
            _ => None,
        })
        .collect::<Vec<_>>();
    if grammars.len() != 1 {
        return Err(GraphRationalLanguageError::InvalidGrammarDeclaration);
    }

    let plans = expressions
        .iter()
        .filter_map(|expression| match expression {
            GraphFrameExpr::Plan(plan) => Some(plan),
            _ => None,
        })
        .collect::<Vec<_>>();
    if plans.len() != 1 {
        return Err(GraphRationalLanguageError::InvalidPlanDeclaration);
    }

    let grammar_declaration = grammars[0];
    if grammar_declaration.grammar_name() != grammar.form().name {
        return Err(GraphRationalLanguageError::GrammarNameMismatch {
            declared: grammar_declaration.grammar_name().to_string(),
            checked: grammar.form().name.clone(),
        });
    }
    if let Some(declared_version) = grammar_declaration.grammar_version() {
        if declared_version != grammar.form().version {
            return Err(GraphRationalLanguageError::GrammarVersionMismatch {
                declared: declared_version.to_string(),
                checked: grammar.form().version.clone(),
            });
        }
    }

    let graph_rules = grammar
        .form()
        .feature_rules
        .iter()
        .filter(|rule| rule.address.stratum == GraphFeatureStratum::Graph)
        .collect::<Vec<_>>();
    if graph_rules.len() != 1 {
        return Err(GraphRationalLanguageError::InvalidGraphFeatureCount(
            graph_rules.len(),
        ));
    }

    let model_id = models[0].model_id();
    let plan_id = plans[0].plan_id();
    let feature_id = format!("Graph.{}", graph_rules[0].address.feature_name);
    let model = ModelSpec {
        id: ModelId(model_id.to_string()),
        kind: ModelKind::FeatureModel,
        input: ModelView::Graph,
        output: ModelView::Features,
        description: Some(format!("Graph Theory feature model for {feature_id}")),
    };
    let feature = Feature::new(
        Plan::from_var("graph-store")
            .named(plan_id)
            .with_model_anchor(model_id)
            .with_feature_anchor(&feature_id)
            .with_principle_triad(ConceptTriad::ModelFeaturePlan),
    )
    .with_id(feature_id);

    Ok(GraphSemanticLowering {
        model,
        grammar: grammar.clone(),
        feature,
    })
}

pub fn validate_graph_density_plan(
    model: &ModelSpec,
    feature: &Feature,
    grammar: &GraphFeatureGrammarChecked,
) -> Result<(), GraphRationalLanguageError> {
    let feature_id = feature
        .id()
        .ok_or(GraphRationalLanguageError::MissingFeatureId)?
        .as_str();
    let plan = feature.plan();

    if plan.name().is_none_or(str::is_empty) {
        return Err(GraphRationalLanguageError::MissingPlanId);
    }

    let model_anchor = plan
        .synthesis()
        .model_anchor
        .as_deref()
        .ok_or(GraphRationalLanguageError::MissingModelAnchor)?;
    if model_anchor != model.id.0 {
        return Err(GraphRationalLanguageError::ModelAnchorMismatch {
            expected: model.id.0.clone(),
            actual: model_anchor.to_string(),
        });
    }

    if !plan
        .synthesis()
        .feature_anchors
        .iter()
        .any(|anchor| anchor == feature_id)
    {
        return Err(GraphRationalLanguageError::MissingFeatureAnchor(
            feature_id.to_string(),
        ));
    }

    let has_rational_principle = plan.principle().is_some_and(|principle| {
        principle.triad == ConceptTriad::ModelFeaturePlan
            && principle.mode == CognitionMode::Rational
            && principle.law_of_appearance.is_none()
    });
    if !has_rational_principle {
        return Err(GraphRationalLanguageError::InvalidRationalPrinciple);
    }

    let has_density_rule = grammar.form().feature_rules.iter().any(|rule| {
        rule.address.stratum == GraphFeatureStratum::Graph
            && rule.address.feature_name == "density"
            && rule.value_type == GraphFeatureValueType::Scalar
    });
    if !has_density_rule {
        return Err(GraphRationalLanguageError::MissingDensityRule);
    }

    Ok(())
}

pub fn observe_graph_density(
    model: &ModelSpec,
    feature: &Feature,
    grammar: &GraphFeatureGrammarChecked,
    frame: &GraphFrame,
) -> Result<Plan, GraphRationalLanguageError> {
    validate_graph_density_plan(model, feature, grammar)?;

    let node_count = frame.node_count()?;
    let relationship_count = frame.relationship_count()?;
    let possible_relationships = node_count.saturating_mul(node_count.saturating_sub(1));
    let density = if possible_relationships == 0 {
        0.0
    } else {
        relationship_count as f64 / possible_relationships as f64
    };
    let feature_id = feature
        .id()
        .expect("validated graph feature id must remain present")
        .as_str();
    let evidence =
        format!("nodes={node_count};relationships={relationship_count};density={density:.6}");

    Ok(feature
        .plan()
        .clone()
        .with_empirical_observation(feature_id, evidence))
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::collections::dataset::feature::Feature;
    use crate::collections::dataset::model::prepare_model;
    use crate::collections::dataset::model::FeatureMark;
    use crate::collections::dataset::model::Modality;
    use crate::collections::dataset::model::ModelId;
    use crate::collections::dataset::model::ModelKind;
    use crate::collections::dataset::model::ModelSpec;
    use crate::collections::dataset::model::ModelView;
    use crate::collections::dataset::plan::CognitionMode;
    use crate::collections::dataset::plan::ConceptTriad;
    use crate::collections::dataset::plan::Plan;
    use crate::collections::graphframe::feature_grammar::validate_graph_feature_grammar;
    use crate::collections::graphframe::feature_grammar::GraphFeatureCardinality;
    use crate::collections::graphframe::feature_grammar::GraphFeatureGrammarChecked;
    use crate::collections::graphframe::feature_grammar::GraphFeatureGrammarForm;
    use crate::collections::graphframe::feature_grammar::GraphFeatureRule;
    use crate::collections::graphframe::feature_grammar::GraphFeatureStratum;
    use crate::collections::graphframe::feature_grammar::GraphFeatureValueType;
    use crate::collections::graphframe::frame::GraphFrame;
    use crate::collections::graphframe::model::GraphFrameModelExt;
    use crate::collections::graphframe::plan::GraphFramePlanExt;
    use crate::types::graph_store::DefaultGraphStore;
    use crate::types::random::RandomGraphConfig;

    use super::lower_graph_semantics;
    use super::observe_graph_density;
    use super::validate_graph_density_plan;
    use super::GraphRationalLanguageError;

    const MODEL_ID: &str = "graph-theory.density-model.v1";
    const FEATURE_ID: &str = "Graph.density";
    const PLAN_ID: &str = "graph-theory.observe-density.v1";

    fn model() -> ModelSpec {
        ModelSpec {
            id: ModelId(MODEL_ID.to_string()),
            kind: ModelKind::FeatureModel,
            input: ModelView::Graph,
            output: ModelView::Features,
            description: Some("Observe directed whole-graph density".to_string()),
        }
    }

    fn density_feature(model_anchor: &str) -> Feature {
        Feature::new(
            Plan::from_var("graph-store")
                .named(PLAN_ID)
                .with_model_anchor(model_anchor)
                .with_feature_anchor(FEATURE_ID)
                .with_principle_triad(ConceptTriad::ModelFeaturePlan),
        )
        .with_id(FEATURE_ID)
    }

    fn density_grammar() -> GraphFeatureGrammarChecked {
        validate_graph_feature_grammar(
            GraphFeatureGrammarForm::new("graph_theory", "v1").with_feature_rule(
                GraphFeatureRule::new(
                    GraphFeatureStratum::Graph,
                    "density",
                    GraphFeatureValueType::Scalar,
                    true,
                    GraphFeatureCardinality::One,
                ),
            ),
        )
        .expect("density grammar should be valid")
    }

    fn deterministic_frame() -> GraphFrame {
        let store = Arc::new(
            DefaultGraphStore::random(&RandomGraphConfig::seeded(21))
                .expect("seeded graph store should build"),
        );
        GraphFrame::from_store(store).expect("graph frame should build")
    }

    #[test]
    fn density_feature_preserves_rational_identity_through_model_preparation() {
        let model = model();
        let feature = density_feature(MODEL_ID);

        validate_graph_density_plan(&model, &feature, &density_grammar())
            .expect("rational graph plan should be valid");
        let essence = prepare_model(
            model.clone(),
            None,
            vec![FeatureMark::contingent(feature.clone())],
        )
        .expect("graph model should prepare");

        assert_eq!(essence.spec.id, model.id);
        assert_eq!(essence.features[0].feature.id(), feature.id());
        assert_eq!(essence.features[0].feature.plan().name(), Some(PLAN_ID));
        assert_eq!(essence.features[0].modality, Modality::Contingent);
        assert_eq!(
            feature.plan().principle().expect("rational principle").mode,
            CognitionMode::Rational
        );
    }

    #[test]
    fn density_observation_transitions_same_plan_from_rational_to_empirical() {
        let model = model();
        let feature = density_feature(MODEL_ID);
        let observed =
            observe_graph_density(&model, &feature, &density_grammar(), &deterministic_frame())
                .expect("density observation should succeed");

        assert_eq!(observed.name(), Some(PLAN_ID));
        assert_eq!(observed.synthesis().model_anchor.as_deref(), Some(MODEL_ID));
        assert_eq!(observed.synthesis().feature_anchors, [FEATURE_ID]);
        let principle = observed.principle().expect("empirical principle");
        assert_eq!(principle.mode, CognitionMode::Empirical);
        assert!(principle.is_empirical_transition());
        let observations = &principle
            .law_of_appearance
            .as_ref()
            .expect("law of appearance")
            .observations;
        assert_eq!(observations[0].feature_anchor, FEATURE_ID);
        assert!(observations[0].evidence.contains("density="));
        assert!(!observations[0].evidence.contains("mock-observation"));
    }

    #[test]
    fn density_plan_rejects_a_model_anchor_mismatch() {
        let error = validate_graph_density_plan(
            &model(),
            &density_feature("graph-theory.other-model"),
            &density_grammar(),
        )
        .expect_err("mismatched model anchor should fail");

        assert!(matches!(
            error,
            GraphRationalLanguageError::ModelAnchorMismatch { .. }
        ));
    }

    #[test]
    fn graphframe_declarations_lower_into_dataset_semantics() {
        let graph_plan = deterministic_frame()
            .gm()
            .model(MODEL_ID)
            .grammar_with_version("graph_theory", "v1")
            .into_plan()
            .gp()
            .id(PLAN_ID)
            .into_plan();
        let lowered = lower_graph_semantics(graph_plan.expressions(), &density_grammar())
            .expect("graph declarations should lower");

        assert_eq!(lowered.model().id.0, MODEL_ID);
        assert_eq!(lowered.model().input, ModelView::Graph);
        assert_eq!(lowered.model().output, ModelView::Features);
        assert_eq!(lowered.grammar().form().name, "graph_theory");
        assert_eq!(
            lowered.feature().id().expect("feature id").as_str(),
            FEATURE_ID
        );
        assert_eq!(lowered.feature().plan().name(), Some(PLAN_ID));
        validate_graph_density_plan(lowered.model(), lowered.feature(), lowered.grammar())
            .expect("lowered Dataset semantics should preserve rational identity");
    }

    #[test]
    fn semantic_lowering_rejects_a_different_checked_grammar() {
        let graph_plan = deterministic_frame()
            .gm()
            .model(MODEL_ID)
            .grammar("citation_graph")
            .into_plan()
            .gp()
            .id(PLAN_ID)
            .into_plan();
        let error = lower_graph_semantics(graph_plan.expressions(), &density_grammar())
            .expect_err("grammar selection must match the checked grammar");

        assert!(matches!(
            error,
            GraphRationalLanguageError::GrammarNameMismatch { .. }
        ));
    }
}
