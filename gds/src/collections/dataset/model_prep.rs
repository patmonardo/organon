//! Box 1 — Model preparation (essence resolution).
//!
//! This module is the first stage of the Dataset substrate's three-band form
//! execution: it takes a [`ModelSpec`], its accumulated essential constraints,
//! and the [`Feature`]s that predicate marks against it, and produces a
//! [`ModelEssence`] — the *prepared* form of the model with each feature
//! stamped by its [`Modality`] under the model's classificatory commitments.
//!
//! Pipeline position:
//!
//! ```text
//! ┌──────────────────────────────────────────────────────────────┐
//! │ Box 1 — Model preparation (this module)                      │
//! │   FeatStruct unification / contradiction detection           │
//! │   Modality stamping per feature                              │
//! │   Emits: ModelEssence + PreparationReport                    │
//! └─────────────────────────────────┬────────────────────────────┘
//!                                   │
//! ┌─────────────────────────────────▼────────────────────────────┐
//! │ Box 2 — Feature execution                                    │
//! │   Lowers MarkedFeatures to Polars Expr / LazyFrame           │
//! └─────────────────────────────────┬────────────────────────────┘
//!                                   │
//! ┌─────────────────────────────────▼────────────────────────────┐
//! │ Box 3 — Image realization                                    │
//! │   Collects to OntologyDataFrameImage with Provenance         │
//! └──────────────────────────────────────────────────────────────┘
//! ```
//!
//! Design notes:
//! - [`Modality`] lives on the [`MarkedFeature`] *wrapper*, not on
//!   [`crate::collections::dataset::expressions::feature::FeatureExpr`].
//!   The IR describes *what* to compute; modality describes *whether* to
//!   compute it under the model's essential relations.
//! - This pass detects contradictions; it does not resolve them. Resolution
//!   is a policy decision that belongs higher up (eventually in the
//!   `ToolChain` / Agent layer).

use crate::collections::dataset::featstruct::{unify_featstruct, FeatBindings, FeatStruct};
use crate::collections::dataset::feature::Feature;
use crate::collections::dataset::model::ModelSpec;

/// Modality of a feature mark within a model's classificatory commitments.
///
/// Determined by [`prepare_model`] from the unification of the feature's
/// optional [`FeatStruct`] mark against the model's accumulated constraints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Modality {
    /// The model requires this mark to hold; it unifies cleanly and the model
    /// declared the feature required, or the mark is fully entailed by what
    /// has already been accumulated.
    Necessary,
    /// The mark may or may not hold; record what we observe at execution.
    /// This is the default when a feature carries no mark.
    Contingent,
    /// The mark could hold but has not yet been resolved against the model's
    /// constraints. Reserved for future deferred-preparation flows.
    Possible,
    /// The mark contradicts the model's accumulated essential relations.
    /// Box 2 must skip lowering this feature; Box 3 records the contradiction
    /// in the image provenance.
    Impossible,
    /// Preparation could not determine modality (e.g. unification raised a
    /// recoverable error). Treated conservatively as non-executable.
    Unknown,
}

/// Outcome of a single feature's preparation step, recorded for Box 3 to
/// surface in the [`crate::collections::dataset::compile_ir::OntologyDataFrameImage`]
/// provenance and constraint tables.
#[derive(Debug, Clone)]
pub struct PreparationStep {
    /// Optional feature name (carried through from [`Feature::name`]).
    pub feature_name: Option<String>,
    /// The modality assigned to the feature.
    pub modality: Modality,
    /// Human-readable reason for the assignment.
    pub note: String,
}

/// Aggregate report from a model preparation pass.
#[derive(Debug, Clone, Default)]
pub struct PreparationReport {
    pub steps: Vec<PreparationStep>,
}

impl PreparationReport {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, step: PreparationStep) {
        self.steps.push(step);
    }

    /// Number of features stamped with the given modality.
    pub fn count(&self, modality: Modality) -> usize {
        self.steps.iter().filter(|s| s.modality == modality).count()
    }

    pub fn any_impossible(&self) -> bool {
        self.count(Modality::Impossible) > 0
    }
}

/// A [`Feature`] stamped with its mark and modality after essence resolution.
///
/// This is the type Box 2 (Feature execution) consumes when lowering features
/// onto a `LazyFrame`. The lowering may pattern-match on [`Self::modality`]:
///
/// - [`Modality::Impossible`] → skip; record contradiction in provenance.
/// - [`Modality::Necessary`]  → emit as constraint or required column.
/// - [`Modality::Contingent`] → emit as plain column.
/// - [`Modality::Possible`] / [`Modality::Unknown`] → defer or skip per policy.
#[derive(Debug, Clone)]
pub struct MarkedFeature {
    pub feature: Feature,
    pub mark: Option<FeatStruct>,
    pub modality: Modality,
    pub derivation: PreparationStep,
}

impl MarkedFeature {
    /// Lift this feature's mark (if any) into a
    /// [`crate::collections::dataset::expressions::feature::FeatureExpr::Mark`]
    /// node so downstream IR walkers can see the essence-level commitment as a
    /// first-class symbolic expression rather than reaching into the wrapper.
    ///
    /// Returns `None` if the feature carries no mark.
    pub fn mark_expr(
        &self,
    ) -> Option<crate::collections::dataset::expressions::feature::FeatureExpr> {
        self.mark.as_ref().map(|fs| {
            crate::collections::dataset::expressions::feature::FeatureExpr::Mark(fs.clone())
        })
    }
}

/// The prepared form of a [`Model`](crate::collections::dataset::model::Model):
/// marks resolved, modalities assigned, ready for Box 2 execution.
#[derive(Debug, Clone)]
pub struct ModelEssence {
    pub spec: ModelSpec,
    /// The accumulated [`FeatStruct`] obtained by unifying the model's seed
    /// constraints with each feature's mark in turn (skipping marks that
    /// failed to unify). This is the model's resolved essence.
    pub accumulated: Option<FeatStruct>,
    pub features: Vec<MarkedFeature>,
    pub report: PreparationReport,
}

impl ModelEssence {
    /// Iterate over executable features, i.e. those whose modality is one of
    /// [`Modality::Necessary`] or [`Modality::Contingent`]. Box 2 should
    /// drive its lowering loop from this iterator.
    pub fn executable(&self) -> impl Iterator<Item = &MarkedFeature> {
        self.features
            .iter()
            .filter(|m| matches!(m.modality, Modality::Necessary | Modality::Contingent))
    }

    /// Iterate over features that contradicted the model's essential
    /// relations and were therefore not executable.
    pub fn impossibilities(&self) -> impl Iterator<Item = &MarkedFeature> {
        self.features
            .iter()
            .filter(|m| m.modality == Modality::Impossible)
    }
}

/// Errors raised by [`prepare_model`].
#[derive(Debug, Clone)]
pub enum PreparationError {
    /// The seed constraint and a required feature mark could not be unified
    /// and the caller has opted into strict mode (not yet exposed; reserved).
    StrictContradiction { feature_name: Option<String> },
}

/// Declares whether a feature's mark is required by the model.
///
/// Required marks that unify produce [`Modality::Necessary`]; required marks
/// that fail to unify still produce [`Modality::Impossible`] (we do not abort
/// preparation — the impossibility is reported and Box 3 records it).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkRequirement {
    Required,
    Optional,
}

/// Input to [`prepare_model`]: a feature paired with its (optional) mark and
/// a flag declaring whether the model requires the mark to hold.
#[derive(Debug, Clone)]
pub struct FeatureMark {
    pub feature: Feature,
    pub mark: Option<FeatStruct>,
    pub requirement: MarkRequirement,
}

impl FeatureMark {
    pub fn contingent(feature: Feature) -> Self {
        Self {
            feature,
            mark: None,
            requirement: MarkRequirement::Optional,
        }
    }

    pub fn optional(feature: Feature, mark: FeatStruct) -> Self {
        Self {
            feature,
            mark: Some(mark),
            requirement: MarkRequirement::Optional,
        }
    }

    pub fn required(feature: Feature, mark: FeatStruct) -> Self {
        Self {
            feature,
            mark: Some(mark),
            requirement: MarkRequirement::Required,
        }
    }
}

/// Prepare a model for Box 2 execution.
///
/// Walks `features` in order. For each feature, unifies its optional mark
/// against the running accumulated essence (seeded by `seed_constraints`) and
/// stamps the feature with the resulting [`Modality`]:
///
/// - **No mark** → [`Modality::Contingent`].
/// - **Mark unifies, was already entailed** (no-op unification) →
///   [`Modality::Necessary`].
/// - **Mark unifies, adds new info** → [`Modality::Necessary`] if
///   `requirement == Required`, else [`Modality::Contingent`].
/// - **Mark fails to unify** → [`Modality::Impossible`].
///
/// Successful unifications update the accumulated essence; failed
/// unifications leave it unchanged so subsequent features still resolve
/// against the consistent core.
pub fn prepare_model(
    spec: ModelSpec,
    seed_constraints: Option<FeatStruct>,
    features: Vec<FeatureMark>,
) -> Result<ModelEssence, PreparationError> {
    let mut accumulated = seed_constraints;
    let mut report = PreparationReport::new();
    let mut marked = Vec::with_capacity(features.len());

    for fm in features {
        let FeatureMark {
            feature,
            mark,
            requirement,
        } = fm;
        let feature_name = feature.name().map(|s| s.to_string());

        let (modality, note, new_accumulated) = match (&accumulated, &mark) {
            // No mark: the feature predicates nothing essence-level.
            (_, None) => (
                Modality::Contingent,
                "no mark — contingent column".to_string(),
                accumulated.clone(),
            ),
            // No accumulated essence yet: adopt the mark verbatim.
            (None, Some(m)) => {
                let modality = match requirement {
                    MarkRequirement::Required => Modality::Necessary,
                    MarkRequirement::Optional => Modality::Contingent,
                };
                (
                    modality,
                    "seeded essence from mark".to_string(),
                    Some(m.clone()),
                )
            }
            // Both present: unify.
            (Some(acc), Some(m)) => {
                let mut bindings = FeatBindings::new();
                match unify_featstruct(acc, m, Some(&mut bindings)) {
                    None => (
                        Modality::Impossible,
                        "mark contradicts accumulated essence".to_string(),
                        accumulated.clone(),
                    ),
                    Some(unified) if &unified == acc => (
                        // Mark adds nothing new — fully entailed by the model.
                        Modality::Necessary,
                        "mark entailed by accumulated essence".to_string(),
                        Some(unified),
                    ),
                    Some(unified) => {
                        let modality = match requirement {
                            MarkRequirement::Required => Modality::Necessary,
                            MarkRequirement::Optional => Modality::Contingent,
                        };
                        (
                            modality,
                            "mark unifies and extends accumulated essence".to_string(),
                            Some(unified),
                        )
                    }
                }
            }
        };

        accumulated = new_accumulated;

        let derivation = PreparationStep {
            feature_name: feature_name.clone(),
            modality,
            note,
        };
        report.push(derivation.clone());

        marked.push(MarkedFeature {
            feature,
            mark,
            modality,
            derivation,
        });
    }

    Ok(ModelEssence {
        spec,
        accumulated,
        features: marked,
        report,
    })
}

// -----------------------------------------------------------------------------
// Model trait → essence (Box 1 ergonomic seam)
// -----------------------------------------------------------------------------

/// Box 1 entry point for any [`crate::collections::dataset::model::Model`].
///
/// Defined as an extension trait so the [`Model`] trait file does not need
/// to import preparation symbols (avoiding any module-cycle suspicion). All
/// `Model` impls receive [`Self::prepare_for`] for free; a model only has
/// to override [`crate::collections::dataset::model::Model::essence_seed`]
/// to contribute a starting essence.
///
/// Typical use:
///
/// ```ignore
/// let essence = my_model.prepare_for(marks)?;        // Box 1
/// let exec    = execute_essence(&essence, lf);       // Box 2
/// let image   = realize_image(&essence, &exec, &opts); // Box 3
/// ```
pub trait ModelPrepExt {
    /// Run Box 1 preparation against the model's
    /// [`crate::collections::dataset::model::Model::essence_seed`] and the
    /// supplied feature marks.
    fn prepare_for(&self, features: Vec<FeatureMark>) -> Result<ModelEssence, PreparationError>;
}

impl<M: crate::collections::dataset::model::Model + ?Sized> ModelPrepExt for M {
    fn prepare_for(&self, features: Vec<FeatureMark>) -> Result<ModelEssence, PreparationError> {
        prepare_model(self.spec().clone(), self.essence_seed(), features)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::dataset::featstruct::{FeatDict, FeatStruct, FeatValue};
    use crate::collections::dataset::model::{ModelId, ModelKind, ModelView};
    use crate::collections::dataset::plan::{Plan, Source};

    fn spec() -> ModelSpec {
        ModelSpec {
            id: ModelId("test".to_string()),
            kind: ModelKind::FeatureModel,
            input: ModelView::Tokens,
            output: ModelView::Features,
            description: None,
        }
    }

    fn feature(name: &str) -> Feature {
        Feature::new(Plan::new(Source::Var("x".to_string()))).named(name)
    }

    fn dict(pairs: &[(&str, FeatValue)]) -> FeatStruct {
        let mut d = FeatDict::new();
        for (k, v) in pairs {
            d.insert((*k).to_string(), v.clone());
        }
        FeatStruct::Dict(d)
    }

    #[test]
    fn no_mark_is_contingent() {
        let essence =
            prepare_model(spec(), None, vec![FeatureMark::contingent(feature("f1"))]).unwrap();
        assert_eq!(essence.features.len(), 1);
        assert_eq!(essence.features[0].modality, Modality::Contingent);
        assert!(essence.accumulated.is_none());
    }

    #[test]
    fn required_mark_with_no_seed_is_necessary() {
        let m = dict(&[("pos", FeatValue::text("noun"))]);
        let essence = prepare_model(
            spec(),
            None,
            vec![FeatureMark::required(feature("f1"), m.clone())],
        )
        .unwrap();
        assert_eq!(essence.features[0].modality, Modality::Necessary);
        assert_eq!(essence.accumulated.as_ref(), Some(&m));
    }

    #[test]
    fn contradictory_mark_is_impossible() {
        let seed = dict(&[("pos", FeatValue::text("noun"))]);
        let bad = dict(&[("pos", FeatValue::text("verb"))]);
        let essence = prepare_model(
            spec(),
            Some(seed.clone()),
            vec![FeatureMark::required(feature("clash"), bad)],
        )
        .unwrap();
        assert_eq!(essence.features[0].modality, Modality::Impossible);
        // Accumulated essence is preserved across contradictions.
        assert_eq!(essence.accumulated.as_ref(), Some(&seed));
        assert!(essence.report.any_impossible());
    }

    #[test]
    fn entailed_mark_is_necessary() {
        let seed = dict(&[
            ("pos", FeatValue::text("noun")),
            ("num", FeatValue::text("sg")),
        ]);
        let entailed = dict(&[("pos", FeatValue::text("noun"))]);
        let essence = prepare_model(
            spec(),
            Some(seed.clone()),
            vec![FeatureMark::optional(feature("f"), entailed)],
        )
        .unwrap();
        assert_eq!(essence.features[0].modality, Modality::Necessary);
        assert_eq!(essence.accumulated.as_ref(), Some(&seed));
    }

    #[test]
    fn executable_skips_impossibilities() {
        let seed = dict(&[("pos", FeatValue::text("noun"))]);
        let bad = dict(&[("pos", FeatValue::text("verb"))]);
        let ok = dict(&[("num", FeatValue::text("sg"))]);
        let essence = prepare_model(
            spec(),
            Some(seed),
            vec![
                FeatureMark::required(feature("a"), bad),
                FeatureMark::optional(feature("b"), ok),
                FeatureMark::contingent(feature("c")),
            ],
        )
        .unwrap();
        let names: Vec<_> = essence
            .executable()
            .map(|m| m.feature.name().unwrap().to_string())
            .collect();
        assert_eq!(names, vec!["b".to_string(), "c".to_string()]);
        assert_eq!(essence.impossibilities().count(), 1);
    }

    #[test]
    fn model_prep_ext_routes_essence_seed() {
        use crate::collections::dataset::model::{Model, ModelSpec};

        struct SeededModel {
            spec: ModelSpec,
            seed: FeatStruct,
        }
        impl Model for SeededModel {
            fn spec(&self) -> &ModelSpec {
                &self.spec
            }
            fn apply(
                &self,
                _dataset: &crate::collections::dataset::Dataset,
                _ctx: &crate::collections::dataset::model::ModelContext,
            ) -> crate::collections::dataset::model::ModelResult {
                Default::default()
            }
            fn essence_seed(&self) -> Option<FeatStruct> {
                Some(self.seed.clone())
            }
        }

        let seed = dict(&[("pos", FeatValue::text("noun"))]);
        let model = SeededModel {
            spec: spec(),
            seed: seed.clone(),
        };
        let entailed = dict(&[("pos", FeatValue::text("noun"))]);
        let essence = model
            .prepare_for(vec![FeatureMark::required(feature("a"), entailed)])
            .unwrap();
        assert_eq!(essence.features.len(), 1);
        assert_eq!(essence.features[0].modality, Modality::Necessary);
        assert_eq!(essence.accumulated.as_ref(), Some(&seed));
    }

    #[test]
    fn marked_feature_lifts_mark_into_feature_expr() {
        use crate::collections::dataset::expressions::feature::FeatureExpr;

        let m = dict(&[("pos", FeatValue::text("noun"))]);
        let essence = prepare_model(
            spec(),
            None,
            vec![FeatureMark::required(feature("a"), m.clone())],
        )
        .unwrap();

        let lifted = essence.features[0].mark_expr().unwrap();
        match &lifted {
            FeatureExpr::Mark(fs) => assert_eq!(fs, &m),
            other => panic!("expected FeatureExpr::Mark, got {other:?}"),
        }

        // A markless feature returns None.
        let bare =
            prepare_model(spec(), None, vec![FeatureMark::contingent(feature("b"))]).unwrap();
        assert!(bare.features[0].mark_expr().is_none());
    }
}
