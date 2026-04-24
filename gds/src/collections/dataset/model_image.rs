//! Box 3 — Image realization.
//!
//! This module is the third stage of the Dataset substrate's three-band form
//! execution. It consumes a [`ModelEssence`] from Box 1 and an [`Execution`]
//! from Box 2 and materializes them as an [`OntologyDataFrameImage`]: the
//! dataset rendered as the *image* of the model's classificatory work, with
//! per-feature provenance and per-mark constraint rows.
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
//! │ Box 2 — Feature execution                                    │
//! │   execute_essence(essence, lf) -> Execution                  │
//! └─────────────────────────────────┬────────────────────────────┘
//!                                   │
//! ┌─────────────────────────────────▼────────────────────────────┐
//! │ Box 3 — Image realization (this module)                      │
//! │   realize_image(essence, exec, opts) -> OntologyDataFrameImage│
//! │     - one Model row per essence                              │
//! │     - one Feature row per ExecutedFeature                    │
//! │     - one Constraint row per essence-mark band               │
//! │     - one Provenance row per ExecutedFeature                 │
//! └──────────────────────────────────────────────────────────────┘
//! ```
//!
//! Box 3 is **deterministic and total**: every input feature produces exactly
//! one Feature row and one Provenance row. Box 2's
//! [`crate::collections::dataset::model_exec::ExecutionAction`] flows through
//! into the row `kind` so consumers can tell which features were applied,
//! skipped as impossible, or deferred.
//!
//! Box 3 deliberately does *not* call `.collect()` on the `LazyFrame`. The
//! image is the *epistemic* projection of the substrate — what was committed
//! to and why — independent of when the tabular runtime materializes the
//! data. Callers who want a materialized DataFrame can collect
//! [`Execution::lazyframe`] separately.

use crate::collections::dataset::compile_ir::{
    OntologyDataFrameImage, OntologyDataFrameImageTables, OntologyImageConstraintRow,
    OntologyImageFeatureRow, OntologyImageModelRow, OntologyImageProvenanceRow,
    OntologyRuntimeMode,
};
use crate::collections::dataset::featstruct::{format_featstruct, FeatStruct};
use crate::collections::dataset::model_exec::{ExecutedFeature, Execution, ExecutionAction};
use crate::collections::dataset::model_prep::{Modality, ModelEssence};

/// Options controlling image realization.
#[derive(Debug, Clone)]
pub struct ImageOptions {
    /// Logical engine name recorded on the image and provenance rows.
    pub engine: String,
    /// Substrate label recorded on provenance rows.
    pub substrate: String,
    /// Source identifier recorded on provenance rows.
    pub source: String,
    /// Runtime mode tag stamped on provenance rows.
    pub runtime_mode: OntologyRuntimeMode,
    /// Optional fixed timestamp for provenance rows. Useful for deterministic
    /// tests; production callers typically leave this `None` and patch it in
    /// at the boundary.
    pub generated_at_unix_ms: Option<u64>,
}

impl Default for ImageOptions {
    fn default() -> Self {
        Self {
            engine: "polars".to_string(),
            substrate: "dataframe/dataset".to_string(),
            source: "gdsl/sdsl".to_string(),
            runtime_mode: OntologyRuntimeMode::TranscendentalLogic,
            generated_at_unix_ms: None,
        }
    }
}

/// Realize the [`OntologyDataFrameImage`] from a Box 1 essence and Box 2
/// execution receipt.
///
/// The image is a deterministic projection of the substrate's commitments:
///
/// - **Model row** — one row, derived from `essence.spec`. Ontology id is
///   the model id.
/// - **Feature rows** — one per [`ExecutedFeature`], in execution order. The
///   row `kind` encodes the [`ExecutionAction`] (`applied-required`,
///   `applied-contingent`, `skipped-impossible`, `skipped-deferred`).
/// - **Constraint rows** — derived from `essence.accumulated` (the model's
///   resolved essence) plus one row per `Impossible` feature recording the
///   contradicting mark.
/// - **Provenance rows** — one per [`ExecutedFeature`], carrying the Box 1
///   derivation note alongside Box 2's action tag.
pub fn realize_image(
    essence: &ModelEssence,
    execution: &Execution,
    opts: &ImageOptions,
) -> OntologyDataFrameImage {
    let model_id = essence.spec.id.0.clone();
    let ontology_ids = vec![model_id.clone()];

    let models = vec![OntologyImageModelRow {
        model_id: model_id.clone(),
        label: model_id.clone(),
        kind: model_kind_label(&essence.spec.kind),
        ontology_ids: ontology_ids.clone(),
    }];

    let features = execution
        .features
        .iter()
        .enumerate()
        .map(|(idx, ef)| feature_row(idx, ef, &model_id, &ontology_ids))
        .collect::<Vec<_>>();

    let mut constraints = Vec::new();
    if let Some(acc) = &essence.accumulated {
        constraints.push(constraint_row(&model_id, "essence", "featstruct", acc));
    }
    for (idx, ef) in execution
        .features
        .iter()
        .enumerate()
        .filter(|(_, ef)| ef.action == ExecutionAction::SkippedImpossible)
    {
        if let Some(mark) = &ef.mark {
            constraints.push(constraint_row(
                &model_id,
                &format!("contradiction-{idx}"),
                "featstruct-contradiction",
                mark,
            ));
        }
    }

    let provenance = execution
        .features
        .iter()
        .map(|ef| provenance_row(&model_id, ef, opts))
        .collect::<Vec<_>>();

    OntologyDataFrameImage {
        image_id: format!("ontology-image:{}", model_id),
        engine: opts.engine.clone(),
        tables: OntologyDataFrameImageTables {
            models,
            features,
            constraints,
            queries: Vec::new(),
            provenance,
        },
    }
}

/// Convenience: prepare → execute → realize, in one call. Useful for tests
/// and for callers who do not need to inspect the intermediate
/// [`Execution`] receipt.
///
/// Returns both the image and the execution receipt so the `LazyFrame` can
/// still be reached if desired.
pub fn realize_from_essence(
    essence: &ModelEssence,
    lf: polars::prelude::LazyFrame,
    opts: &ImageOptions,
) -> (OntologyDataFrameImage, Execution) {
    let exec = crate::collections::dataset::model_exec::execute_essence(essence, lf);
    let image = realize_image(essence, &exec, opts);
    (image, exec)
}

// -----------------------------------------------------------------------------
// Internal row builders
// -----------------------------------------------------------------------------

fn model_kind_label(kind: &crate::collections::dataset::model::ModelKind) -> String {
    use crate::collections::dataset::model::ModelKind;
    match kind {
        ModelKind::Tagger => "tagger".to_string(),
        ModelKind::Parser => "parser".to_string(),
        ModelKind::Segmenter => "segmenter".to_string(),
        ModelKind::LanguageModel => "language-model".to_string(),
        ModelKind::Semantic => "semantic".to_string(),
        ModelKind::FeatureModel => "feature-model".to_string(),
        ModelKind::Composite => "composite".to_string(),
        ModelKind::Custom(name) => format!("custom:{name}"),
    }
}

fn feature_row(
    index: usize,
    ef: &ExecutedFeature,
    model_id: &str,
    ontology_ids: &[String],
) -> OntologyImageFeatureRow {
    let label = ef
        .name
        .clone()
        .unwrap_or_else(|| format!("feature-{index}"));
    let kind = match ef.action {
        ExecutionAction::AppliedRequired => "applied-required",
        ExecutionAction::AppliedContingent => "applied-contingent",
        ExecutionAction::SkippedImpossible => "skipped-impossible",
        ExecutionAction::SkippedDeferred => "skipped-deferred",
    };
    OntologyImageFeatureRow {
        feature_id: format!("feature:{model_id}:{index}:{}", sanitize(&label)),
        model_id: Some(model_id.to_string()),
        label,
        kind: kind.to_string(),
        ontology_ids: ontology_ids.to_vec(),
    }
}

fn constraint_row(
    model_id: &str,
    suffix: &str,
    language: &str,
    fs: &FeatStruct,
) -> OntologyImageConstraintRow {
    OntologyImageConstraintRow {
        ontology_id: model_id.to_string(),
        constraint_id: format!("constraint:{model_id}:{suffix}"),
        language: language.to_string(),
        text: format_featstruct(fs),
    }
}

fn provenance_row(
    model_id: &str,
    ef: &ExecutedFeature,
    opts: &ImageOptions,
) -> OntologyImageProvenanceRow {
    let modality_tag = match ef.modality {
        Modality::Necessary => "necessary",
        Modality::Contingent => "contingent",
        Modality::Possible => "possible",
        Modality::Impossible => "impossible",
        Modality::Unknown => "unknown",
    };
    let action_tag = match ef.action {
        ExecutionAction::AppliedRequired => "applied-required",
        ExecutionAction::AppliedContingent => "applied-contingent",
        ExecutionAction::SkippedImpossible => "skipped-impossible",
        ExecutionAction::SkippedDeferred => "skipped-deferred",
    };
    let feature_tag = ef.name.as_deref().unwrap_or("<anon>");
    OntologyImageProvenanceRow {
        source: opts.source.clone(),
        specification_id: format!(
            "{model_id}::{feature_tag}::{modality_tag}::{action_tag}::{note}",
            note = sanitize(&ef.derivation.note),
        ),
        runtime_mode: opts.runtime_mode,
        substrate: opts.substrate.clone(),
        generated_at_unix_ms: opts.generated_at_unix_ms.unwrap_or(0),
    }
}

fn sanitize(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.') {
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push('_');
        }
    }
    if out.is_empty() {
        "unnamed".to_string()
    } else {
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::dataset::featstruct::{FeatDict, FeatStruct, FeatValue};
    use crate::collections::dataset::feature::Feature;
    use crate::collections::dataset::model::{ModelId, ModelKind, ModelSpec, ModelView};
    use crate::collections::dataset::model_exec::execute_essence;
    use crate::collections::dataset::model_prep::{prepare_model, FeatureMark};
    use crate::collections::dataset::plan::{Plan, Source};
    use polars::prelude::{df, IntoLazy};

    fn spec() -> ModelSpec {
        ModelSpec {
            id: ModelId("m".to_string()),
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

    fn opts() -> ImageOptions {
        ImageOptions {
            generated_at_unix_ms: Some(1234),
            ..Default::default()
        }
    }

    #[test]
    fn image_has_one_model_row_keyed_to_essence_id() {
        let essence = prepare_model(spec(), None, vec![]).unwrap();
        let lf = df!("a" => &[1i64]).unwrap().lazy();
        let exec = execute_essence(&essence, lf);
        let image = realize_image(&essence, &exec, &opts());
        assert_eq!(image.tables.models.len(), 1);
        assert_eq!(image.tables.models[0].model_id, "m");
        assert_eq!(image.tables.models[0].kind, "feature-model");
        assert_eq!(image.image_id, "ontology-image:m");
    }

    #[test]
    fn feature_row_kind_encodes_execution_action() {
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
        let lf = df!("a" => &[1i64]).unwrap().lazy();
        let exec = execute_essence(&essence, lf);
        let image = realize_image(&essence, &exec, &opts());

        // Feature rows: one per ExecutedFeature, deterministic order.
        assert_eq!(image.tables.features.len(), 3);
        assert_eq!(image.tables.features[0].kind, "skipped-impossible");
        assert_eq!(image.tables.features[1].kind, "applied-contingent");
        assert_eq!(image.tables.features[2].kind, "applied-contingent");

        // All feature rows are tied to the model.
        for row in &image.tables.features {
            assert_eq!(row.model_id.as_deref(), Some("m"));
            assert_eq!(row.ontology_ids, vec!["m".to_string()]);
        }

        // Provenance: one row per executed feature.
        assert_eq!(image.tables.provenance.len(), 3);
        assert!(image.tables.provenance[0]
            .specification_id
            .contains("impossible"));
        assert!(image.tables.provenance[1]
            .specification_id
            .contains("contingent"));
    }

    #[test]
    fn constraint_rows_carry_essence_and_contradictions() {
        let seed = dict(&[("pos", FeatValue::text("noun"))]);
        let bad = dict(&[("pos", FeatValue::text("verb"))]);

        let essence = prepare_model(
            spec(),
            Some(seed),
            vec![FeatureMark::required(feature("clash"), bad)],
        )
        .unwrap();
        let lf = df!("a" => &[1i64]).unwrap().lazy();
        let exec = execute_essence(&essence, lf);
        let image = realize_image(&essence, &exec, &opts());

        // One row for accumulated essence + one for the contradiction.
        assert_eq!(image.tables.constraints.len(), 2);
        assert_eq!(
            image.tables.constraints[0].constraint_id,
            "constraint:m:essence"
        );
        assert_eq!(image.tables.constraints[0].language, "featstruct");
        assert_eq!(
            image.tables.constraints[1].constraint_id,
            "constraint:m:contradiction-0"
        );
        assert_eq!(
            image.tables.constraints[1].language,
            "featstruct-contradiction"
        );
    }

    #[test]
    fn empty_essence_produces_minimal_image() {
        let essence = prepare_model(spec(), None, vec![]).unwrap();
        let lf = df!("a" => &[1i64]).unwrap().lazy();
        let exec = execute_essence(&essence, lf);
        let image = realize_image(&essence, &exec, &opts());

        assert_eq!(image.tables.models.len(), 1);
        assert!(image.tables.features.is_empty());
        assert!(image.tables.constraints.is_empty());
        assert!(image.tables.provenance.is_empty());
        assert_eq!(image.engine, "polars");
    }

    #[test]
    fn realize_from_essence_threads_through() {
        let m = dict(&[("pos", FeatValue::text("noun"))]);
        let essence =
            prepare_model(spec(), None, vec![FeatureMark::required(feature("a"), m)]).unwrap();
        let lf = df!("a" => &[1i64]).unwrap().lazy();
        let (image, exec) = realize_from_essence(&essence, lf, &opts());
        assert_eq!(image.tables.features.len(), 1);
        assert_eq!(image.tables.features[0].kind, "applied-required");
        assert_eq!(exec.applied().count(), 1);
    }
}
