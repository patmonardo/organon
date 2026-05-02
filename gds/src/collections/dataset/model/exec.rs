//! Box 2 — Feature execution (form unfolding).
//!
//! This module is the second stage of the Dataset substrate's three-band form
//! execution. It consumes a [`ModelEssence`] produced by
//! [`crate::collections::dataset::model::prep::prepare_model`] and lowers each
//! [`MarkedFeature`] onto a `LazyFrame`, threading [`Modality`] through the
//! lowering so the resulting plan is *form-aware*, not just column-additive.
//!
//! Pipeline position:
//!
//! ```text
//! ┌──────────────────────────────────────────────────────────────┐
//! │ Box 1 — Model preparation                                    │
//! │   prepare_model(...) -> ModelEssence                         │
//! └─────────────────────────────────┬────────────────────────────┘
//!                                   │
//! ┌─────────────────────────────────▼────────────────────────────┐
//! │ Box 2 — Feature execution (this module)                      │
//! │   execute_essence(essence, lf) -> Execution                  │
//! │     - Necessary  → lower + mark as required column           │
//! │     - Contingent → lower as plain column                     │
//! │     - Impossible → skip; record contradiction                │
//! │     - Possible / Unknown → defer (skip)                      │
//! └─────────────────────────────────┬────────────────────────────┘
//!                                   │
//! ┌─────────────────────────────────▼────────────────────────────┐
//! │ Box 3 — Image realization                                    │
//! │   collect Execution -> OntologyDataFrameImage                │
//! └──────────────────────────────────────────────────────────────┘
//! ```
//!
//! What this module is *not*:
//! - Not a query optimizer (Polars Lazy already is).
//! - Not a runtime collector (Box 3 owns `.collect()`).
//! - Not a contradiction resolver (Box 1 detects, Box 3 reports, policy lives
//!   above the substrate).
//!
//! Lowering policy by modality:
//! - **Necessary** features run [`Feature::apply_to_lazyframe`] *and* their
//!   mark is lowered to a `LazyFrame::filter` predicate so the runtime plan
//!   actually enforces the essence the model committed to. Only scalar
//!   top-level mark entries (`Text` / `Number` / `Bool`) targeting columns
//!   present in the lazy schema are lowered; structural mark entries
//!   (`Struct` / `List` / `Variable` / reentrances) are left untouched and
//!   stay symbolic in the receipt.
//! - **Contingent** features run [`Feature::apply_to_lazyframe`] without any
//!   constraint enforcement.
//! - **Impossible** features are skipped; Box 3 records the contradiction.
//! - **Possible / Unknown** are deferred (skipped).
//!
//! Box 2 still does not call `.collect()`. If a Necessary mark targets a
//! column that does not exist, schema probing fails closed: the constraint
//! is silently *not* lowered (Box 2 stays total), and the missing-column
//! contradiction surfaces at Box 3 collect time. Box 1 remains the place
//! where formally-impossible essence is detected; Box 2 only enforces what
//! the runtime schema actually exposes.

use polars::prelude::{col, lit, LazyFrame};

use crate::collections::dataset::feature::featstruct::{FeatStruct, FeatValue};
use crate::collections::dataset::feature::Feature;
use crate::collections::dataset::model::prep::{
    MarkedFeature, Modality, ModelEssence, PreparationStep,
};

/// One feature's outcome inside an execution pass.
///
/// Every feature in the input [`ModelEssence`] produces an
/// [`ExecutedFeature`], whether or not it was actually lowered onto the
/// `LazyFrame`. This keeps Box 3's image construction deterministic and
/// total (no silent drops).
#[derive(Debug, Clone)]
pub struct ExecutedFeature {
    /// Optional feature name, carried through from [`Feature::name`].
    pub name: Option<String>,
    /// The mark this feature predicates, if any.
    pub mark: Option<FeatStruct>,
    /// Modality stamp from Box 1.
    pub modality: Modality,
    /// Whether Box 2 actually applied the feature to the `LazyFrame`.
    pub applied: bool,
    /// Box 1's preparation step (carried through for Box 3 provenance).
    pub derivation: PreparationStep,
    /// Human-readable note for what Box 2 chose to do.
    pub action: ExecutionAction,
}

/// What Box 2 did with a single feature.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionAction {
    /// Lowered onto the `LazyFrame` as a required column (Necessary mark).
    AppliedRequired,
    /// Lowered onto the `LazyFrame` as a plain column (Contingent mark).
    AppliedContingent,
    /// Skipped because the mark contradicted the model's essence.
    SkippedImpossible,
    /// Skipped because preparation could not determine modality, or the
    /// modality is reserved for a future lowering policy.
    SkippedDeferred,
}

impl ExecutionAction {
    pub fn is_applied(&self) -> bool {
        matches!(self, Self::AppliedRequired | Self::AppliedContingent)
    }
}

/// Receipt of one Box 2 execution pass over a [`ModelEssence`].
///
/// Holds the post-lowering `LazyFrame` plus a per-feature [`ExecutedFeature`]
/// trace. Box 3 consumes this to materialize the
/// [`crate::collections::dataset::compile::OntologyDataFrameImage`].
#[derive(Clone)]
pub struct Execution {
    /// The `LazyFrame` after all applicable features have been lowered.
    pub lazyframe: LazyFrame,
    /// One entry per input feature, in input order.
    pub features: Vec<ExecutedFeature>,
}

impl std::fmt::Debug for Execution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Execution")
            .field("lazyframe", &"<LazyFrame>")
            .field("features", &self.features)
            .finish()
    }
}

impl Execution {
    /// Iterate over features that were actually applied to the `LazyFrame`.
    pub fn applied(&self) -> impl Iterator<Item = &ExecutedFeature> {
        self.features.iter().filter(|f| f.applied)
    }

    /// Iterate over features that were skipped because of an
    /// [`Modality::Impossible`] mark.
    pub fn impossibilities(&self) -> impl Iterator<Item = &ExecutedFeature> {
        self.features
            .iter()
            .filter(|f| f.action == ExecutionAction::SkippedImpossible)
    }

    /// Iterate over features that were deferred (Possible / Unknown).
    pub fn deferred(&self) -> impl Iterator<Item = &ExecutedFeature> {
        self.features
            .iter()
            .filter(|f| f.action == ExecutionAction::SkippedDeferred)
    }

    /// Total number of features stamped with each modality, derived from the
    /// receipt rather than the source essence.
    pub fn count(&self, modality: Modality) -> usize {
        self.features
            .iter()
            .filter(|f| f.modality == modality)
            .count()
    }
}

/// Lower a single [`MarkedFeature`] onto `lf` according to its modality.
///
/// Returns the (possibly updated) `LazyFrame` and an [`ExecutedFeature`]
/// receipt. `Necessary` features additionally have their scalar mark
/// entries lowered to `filter` predicates against existing columns; see
/// [`enforce_necessary_mark`].
pub fn execute_marked(lf: LazyFrame, marked: &MarkedFeature) -> (LazyFrame, ExecutedFeature) {
    let name = marked.feature.name().map(|s| s.to_string());
    let mark = marked.mark.clone();
    let modality = marked.modality;
    let derivation = marked.derivation.clone();

    let (next_lf, applied, action) = match modality {
        Modality::Necessary => {
            let lowered = marked.feature.apply_to_lazyframe(lf);
            let constrained = match &marked.mark {
                Some(m) => enforce_necessary_mark(lowered, m),
                None => lowered,
            };
            (constrained, true, ExecutionAction::AppliedRequired)
        }
        Modality::Contingent => {
            let next = marked.feature.apply_to_lazyframe(lf);
            (next, true, ExecutionAction::AppliedContingent)
        }
        Modality::Impossible => (lf, false, ExecutionAction::SkippedImpossible),
        Modality::Possible | Modality::Unknown => (lf, false, ExecutionAction::SkippedDeferred),
    };

    (
        next_lf,
        ExecutedFeature {
            name,
            mark,
            modality,
            applied,
            derivation,
            action,
        },
    )
}

/// Execute every feature in `essence` against `lf` in order.
///
/// Walks the essence's features in their declared order, lowering each via
/// [`execute_marked`]. The accumulated essence on `essence` is *not* applied
/// as a global `filter` here — global essence enforcement is Box 3 / image
/// realization concern. Box 2 only commits to *per-feature* form unfolding.
pub fn execute_essence(essence: &ModelEssence, lf: LazyFrame) -> Execution {
    let mut current = lf;
    let mut features = Vec::with_capacity(essence.features.len());
    for marked in &essence.features {
        let (next, executed) = execute_marked(current, marked);
        current = next;
        features.push(executed);
    }
    Execution {
        lazyframe: current,
        features,
    }
}

/// Lower a `Necessary` mark to `LazyFrame::filter` predicates.
///
/// Walks the top-level entries of `mark` (when it is a `FeatStruct::Dict`)
/// and adds a `col(key) == lit(value)` filter for each scalar entry whose
/// key matches a column in `lf`'s lazy schema. Structural entries
/// (`Struct` / `List` / `Variable` / reentrances) and entries targeting
/// unknown columns are left untouched — they remain symbolic on the
/// [`ExecutedFeature::mark`] receipt for Box 3 to record.
///
/// If schema probing fails (e.g. an upstream Polars planning error), this
/// returns `lf` unchanged so Box 2 stays total. The downstream
/// `.collect()` will surface the original error.
pub fn enforce_necessary_mark(lf: LazyFrame, mark: &FeatStruct) -> LazyFrame {
    let dict = match mark {
        FeatStruct::Dict(d) => d,
        // Non-dict marks have no top-level column mapping to enforce.
        FeatStruct::List(_) => return lf,
    };

    let schema = match lf.clone().collect_schema() {
        Ok(s) => s,
        Err(_) => return lf,
    };

    let mut current = lf;
    for (key, value) in dict {
        if !schema.contains(key.as_str()) {
            continue;
        }
        let predicate = match value {
            FeatValue::Text(s) => col(key.as_str()).eq(lit(s.clone())),
            FeatValue::Number(n) => col(key.as_str()).eq(lit(*n)),
            FeatValue::Bool(b) => col(key.as_str()).eq(lit(*b)),
            // Structural / symbolic entries are not lowered to predicates.
            _ => continue,
        };
        current = current.filter(predicate);
    }
    current
}

/// Execute a standalone [`Feature`] without modality (treated as Contingent).
///
/// Convenience for callers that have a `Feature` but no `ModelEssence` yet.
/// Equivalent to [`Feature::apply_to_lazyframe`] but returns a one-element
/// [`Execution`] receipt so the caller has uniform downstream handling.
pub fn execute_feature(lf: LazyFrame, feature: &Feature) -> Execution {
    let next = feature.apply_to_lazyframe(lf);
    let executed = ExecutedFeature {
        name: feature.name().map(|s| s.to_string()),
        mark: None,
        modality: Modality::Contingent,
        applied: true,
        derivation: PreparationStep {
            feature_name: feature.name().map(|s| s.to_string()),
            modality: Modality::Contingent,
            note: "executed without preparation — contingent by default".to_string(),
        },
        action: ExecutionAction::AppliedContingent,
    };
    Execution {
        lazyframe: next,
        features: vec![executed],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::dataset::feature::featstruct::{FeatDict, FeatStruct, FeatValue};
    use crate::collections::dataset::model::prep::{prepare_model, FeatureMark};
    use crate::collections::dataset::model::{ModelId, ModelKind, ModelSpec, ModelView};
    use crate::collections::dataset::plan::{Plan, Source};
    use polars::prelude::{df, IntoLazy};

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

    fn lf() -> LazyFrame {
        df!("a" => &[1i64, 2, 3]).unwrap().lazy()
    }

    #[test]
    fn execute_essence_runs_executable_and_skips_impossibles() {
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

        let exec = execute_essence(&essence, lf());

        // Three receipts, one per input feature.
        assert_eq!(exec.features.len(), 3);

        // a is Impossible -> skipped.
        assert_eq!(exec.features[0].action, ExecutionAction::SkippedImpossible);
        assert!(!exec.features[0].applied);

        // b is Contingent -> applied as contingent.
        assert_eq!(exec.features[1].action, ExecutionAction::AppliedContingent);
        assert!(exec.features[1].applied);

        // c is Contingent -> applied as contingent.
        assert_eq!(exec.features[2].action, ExecutionAction::AppliedContingent);
        assert!(exec.features[2].applied);

        assert_eq!(exec.applied().count(), 2);
        assert_eq!(exec.impossibilities().count(), 1);
        assert_eq!(exec.deferred().count(), 0);
    }

    #[test]
    fn necessary_features_tagged_as_required() {
        let m = dict(&[("pos", FeatValue::text("noun"))]);
        let essence =
            prepare_model(spec(), None, vec![FeatureMark::required(feature("a"), m)]).unwrap();
        let exec = execute_essence(&essence, lf());
        assert_eq!(exec.features[0].action, ExecutionAction::AppliedRequired);
        assert_eq!(exec.features[0].modality, Modality::Necessary);
    }

    #[test]
    fn execute_feature_yields_contingent_singleton() {
        let exec = execute_feature(lf(), &feature("solo"));
        assert_eq!(exec.features.len(), 1);
        assert_eq!(exec.features[0].action, ExecutionAction::AppliedContingent);
        assert!(exec.features[0].applied);
    }

    #[test]
    fn empty_essence_returns_unchanged_lazyframe_and_empty_receipt() {
        let essence = prepare_model(spec(), None, vec![]).unwrap();
        let exec = execute_essence(&essence, lf());
        assert!(exec.features.is_empty());
        assert_eq!(exec.applied().count(), 0);
    }

    #[test]
    fn necessary_text_mark_filters_rows_on_matching_column() {
        let frame = df!(
            "pos" => &["noun", "verb", "noun"],
            "n"   => &[1i64, 2, 3],
        )
        .unwrap()
        .lazy();

        let m = dict(&[("pos", FeatValue::text("noun"))]);
        let essence =
            prepare_model(spec(), None, vec![FeatureMark::required(feature("a"), m)]).unwrap();
        let exec = execute_essence(&essence, frame);

        let collected = exec.lazyframe.collect().unwrap();
        assert_eq!(collected.height(), 2);
        let pos = collected.column("pos").unwrap().str().unwrap();
        assert!(pos.into_no_null_iter().all(|v| v == "noun"));
    }

    #[test]
    fn necessary_int_and_bool_marks_chain_filters() {
        let frame = df!(
            "k"   => &[1i64, 1, 2, 1],
            "ok"  => &[true, false, true, true],
        )
        .unwrap()
        .lazy();

        let m = dict(&[("k", FeatValue::Number(1)), ("ok", FeatValue::Bool(true))]);
        let essence =
            prepare_model(spec(), None, vec![FeatureMark::required(feature("a"), m)]).unwrap();
        let exec = execute_essence(&essence, frame);

        let collected = exec.lazyframe.collect().unwrap();
        assert_eq!(collected.height(), 2);
    }

    #[test]
    fn necessary_mark_with_unknown_column_does_not_filter() {
        let frame = df!("a" => &[1i64, 2, 3]).unwrap().lazy();

        // mark targets `pos`, which does not exist in the frame schema.
        let m = dict(&[("pos", FeatValue::text("noun"))]);
        let essence =
            prepare_model(spec(), None, vec![FeatureMark::required(feature("x"), m)]).unwrap();
        let exec = execute_essence(&essence, frame);

        // Unknown column => filter not lowered, frame still collects all rows.
        let collected = exec.lazyframe.collect().unwrap();
        assert_eq!(collected.height(), 3);
        // Receipt still records Necessary + AppliedRequired.
        assert_eq!(exec.features[0].modality, Modality::Necessary);
        assert_eq!(exec.features[0].action, ExecutionAction::AppliedRequired);
    }

    #[test]
    fn necessary_mark_with_structural_value_is_left_symbolic() {
        let frame = df!("pos" => &["noun", "verb"]).unwrap().lazy();

        // Struct-valued mark entry has no scalar lowering; should not filter.
        let nested = dict(&[("inner", FeatValue::text("x"))]);
        let mut outer = FeatDict::new();
        outer.insert("agreement".to_string(), FeatValue::Struct(nested));
        let m = FeatStruct::Dict(outer);

        let essence =
            prepare_model(spec(), None, vec![FeatureMark::required(feature("a"), m)]).unwrap();
        let exec = execute_essence(&essence, frame);

        let collected = exec.lazyframe.collect().unwrap();
        assert_eq!(collected.height(), 2);
    }

    #[test]
    fn contingent_mark_does_not_filter() {
        let frame = df!(
            "pos" => &["noun", "verb", "noun"],
        )
        .unwrap()
        .lazy();

        let m = dict(&[("pos", FeatValue::text("noun"))]);
        // optional => Contingent (not Necessary), so no enforcement.
        let essence =
            prepare_model(spec(), None, vec![FeatureMark::optional(feature("a"), m)]).unwrap();
        let exec = execute_essence(&essence, frame);

        let collected = exec.lazyframe.collect().unwrap();
        assert_eq!(collected.height(), 3);
        assert_eq!(exec.features[0].action, ExecutionAction::AppliedContingent);
    }
}
