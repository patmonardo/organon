//! Features: feature-oriented dataset transformations.
//!
//! In the NLTK / WordNet / corpora sense, a "Feature" is a mapping from raw input →
//! feature representation(s) consumed by a downstream model or procedure.
//!
//! In this crate, a `Feature` is primarily backed by a `Plan` (DataOps) so that:
//! - it can run over full datasets (`eval_dataset`) or streaming batches (`to_streaming_transform`)
//! - it can be reported on via the Plan "attention" envelope.
//!
//! Convention (optional): the canonical feature-map artifact is an `item` column.
//! This is *not required* for a `Feature`, but you can enforce it via
//! `Feature::requiring_item(plan)` when you want that contract.

use std::collections::BTreeMap;

use polars::prelude::{Expr, LazyFrame};

use crate::collections::dataframe::Selector;
use crate::collections::dataset::plan::{EvalMode, Plan, PlanAttentionReport, PlanEnv, PlanError};
use crate::collections::dataset::schema::FeatureSchema;
use crate::collections::dataset::streaming::StreamingDataset;
use crate::collections::dataset::Dataset;
use crate::prints::{PrintEnvelope, PrintKind, PrintProvenance};

pub use crate::collections::dataset::expressions::feature::{
    FeatureCondition, FeatureExpr, FeaturePath, FeaturePosition, FeatureRule, FeatureSpec,
    FeatureTemplate, FeatureValue,
};
pub use crate::collections::dataset::namespaces::feature::{
    FeatureExprNameSpace, FeatureNs as FeatureNamespace,
};

#[derive(Debug, Clone)]
pub struct Feature {
    plan: Plan,
}

/// First-class schema view over a named feature.
///
/// This lets the algebra speak in *semantic views* (schema selectors) rather than
/// raw column lists or ad-hoc selectors.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FeatureView {
    feature: String,
    view: String,
}

impl FeatureView {
    pub fn new(feature: impl Into<String>, view: impl Into<String>) -> Self {
        Self {
            feature: feature.into(),
            view: view.into(),
        }
    }

    pub fn feature(&self) -> &str {
        &self.feature
    }

    pub fn view(&self) -> &str {
        &self.view
    }

    pub fn eval(
        &self,
        space: &FeatureSpace,
        schema: &FeatureSchema,
        env: &PlanEnv,
        mode: EvalMode,
    ) -> Result<Dataset, PlanError> {
        space.eval_view(&self.feature, schema, &self.view, env, mode)
    }
}

impl Feature {
    /// Create a `Feature` from a `Plan`.
    ///
    /// This does not enforce any particular output shape. If you want the
    /// canonical `item`-producing contract, use `Feature::requiring_item(plan)`.
    pub fn new(plan: Plan) -> Self {
        Self { plan }
    }

    /// Set the feature's name (delegates to the underlying plan name).
    pub fn named(mut self, name: impl Into<String>) -> Self {
        self.plan = self.plan.named(name);
        self
    }

    /// A borrowed variant of `named`.
    pub fn with_name(&self, name: impl Into<String>) -> Self {
        Self::new(self.plan.clone().named(name))
    }

    /// Create a `Feature` and enforce that the plan ends in an `item` step.
    pub fn requiring_item(plan: Plan) -> Result<Self, PlanError> {
        if !plan.ends_with_item() {
            return Err(PlanError::Message(
                "Feature::requiring_item requires the plan to end with an (item ...) step"
                    .to_string(),
            ));
        }

        Ok(Self { plan })
    }

    pub fn plan(&self) -> &Plan {
        &self.plan
    }

    /// Project into the canonical `item` feature-map artifact.
    ///
    /// This is the bridge from "feature-oriented" pipelines into a stable
    /// tabular representation for well-known DataFrame processing.
    pub fn project_item(mut self, item_expr: Expr) -> Feature {
        self.plan = self.plan.project_item(item_expr);
        self
    }

    /// A borrowed variant of `project_item`.
    pub fn projected_item(&self, item_expr: Expr) -> Feature {
        Feature::new(self.plan.clone().project_item(item_expr))
    }

    /// Compose two feature pipelines by chaining their underlying plans.
    ///
    /// This is the core "feature-chain" idea: raw data can be progressively
    /// featured into richer representations (text → tokens → embeddings, image
    /// → augmentations → embeddings, etc.).
    pub fn chain(self, next: &Feature) -> Feature {
        Feature::new(self.plan.chain(next.plan()))
    }

    /// A borrowed variant of `chain`.
    pub fn chained(&self, next: &Feature) -> Feature {
        Feature::new(self.plan.clone().chain(next.plan()))
    }

    pub fn into_plan(self) -> Plan {
        self.plan
    }

    pub fn name(&self) -> Option<&str> {
        self.plan.name()
    }

    /// Evaluate the feature mapping over a dataset binding environment.
    pub fn eval_dataset(&self, env: &PlanEnv, mode: EvalMode) -> Result<Dataset, PlanError> {
        self.plan.eval_dataset(env, mode)
    }

    /// Evaluate the feature pipeline and then select a view using a selector.
    pub fn eval_select(
        &self,
        env: &PlanEnv,
        mode: EvalMode,
        selector: &Selector,
    ) -> Result<Dataset, PlanError> {
        let ds = self.eval_dataset(env, mode)?;
        Ok(ds.select_selector(selector)?)
    }

    pub fn attention_report(&self, env: Option<&PlanEnv>, mode: EvalMode) -> PlanAttentionReport {
        self.plan.attention_report(env, mode)
    }

    pub fn to_print_envelope(
        &self,
        env: Option<&PlanEnv>,
        mode: EvalMode,
        run_id: Option<String>,
    ) -> PrintEnvelope {
        self.plan.to_print_envelope(env, mode, run_id)
    }

    /// Apply the feature mapping to a streaming batch `LazyFrame`.
    pub fn apply_to_lazyframe(&self, lf: LazyFrame) -> LazyFrame {
        self.plan.apply_to_lazyframe(lf)
    }

    /// Convert this feature mapping into a reusable streaming transform.
    pub fn to_streaming_transform(&self) -> Box<dyn Fn(LazyFrame) -> LazyFrame + Send + Sync> {
        self.plan.to_streaming_transform()
    }

    /// Create a streaming feature processor over a dataset.
    pub fn stream(&self, dataset: Dataset, batch_size: usize) -> StreamingDataset {
        StreamingDataset::new(dataset, batch_size).with_feature(self)
    }
}

/// Feature Algebra: a named space of features that can be selected and combined.
///
/// This is intentionally conservative: it is a *declarative* container.
/// Execution remains feature-by-feature via `Feature` methods.
#[derive(Debug, Clone, Default)]
pub struct FeatureSpace {
    features: BTreeMap<String, Feature>,
}

impl FeatureSpace {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.features.len()
    }

    pub fn is_empty(&self) -> bool {
        self.features.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &Feature)> {
        self.features
            .iter()
            .map(|(name, feature)| (name.as_str(), feature))
    }

    pub fn insert(mut self, name: impl Into<String>, feature: Feature) -> Self {
        self.features.insert(name.into(), feature);
        self
    }

    /// Insert a feature derived from an existing feature.
    pub fn insert_derived(
        mut self,
        name: impl Into<String>,
        base: &str,
        f: impl FnOnce(Feature) -> Feature,
    ) -> Result<Self, PlanError> {
        let base_feature = self.require(base)?.clone();
        self.features.insert(name.into(), f(base_feature));
        Ok(self)
    }

    /// Left-biased union: existing names are kept.
    pub fn union(mut self, other: FeatureSpace) -> Self {
        for (name, feature) in other.features {
            self.features.entry(name).or_insert(feature);
        }
        self
    }

    /// Insert a template-derived feature using a builder.
    pub fn insert_template_with(
        mut self,
        name: impl Into<String>,
        template: &FeatureTemplate,
        f: impl FnOnce(&FeatureTemplate) -> Feature,
    ) -> Self {
        self.features.insert(name.into(), f(template));
        self
    }

    /// Right-biased merge: incoming names overwrite existing ones.
    pub fn merge(mut self, other: FeatureSpace) -> Self {
        for (name, feature) in other.features {
            self.features.insert(name, feature);
        }
        self
    }

    /// Remove a feature by name (no-op if missing).
    pub fn remove(mut self, name: &str) -> Self {
        self.features.remove(name);
        self
    }

    /// Keep only the named features. Returns an error on unknown names.
    pub fn keep<'a>(&self, names: impl IntoIterator<Item = &'a str>) -> Result<Self, PlanError> {
        let mut out = FeatureSpace::new();
        for name in names {
            let feature = self.require(name)?.clone();
            out.features.insert(name.to_string(), feature);
        }
        Ok(out)
    }

    /// Drop the named features (unknown names are ignored).
    pub fn drop<'a>(mut self, names: impl IntoIterator<Item = &'a str>) -> Self {
        for name in names {
            self.features.remove(name);
        }
        self
    }

    /// Rename a feature, erroring if the source is missing or the target exists.
    pub fn rename(mut self, from: &str, to: &str) -> Result<Self, PlanError> {
        if self.features.contains_key(to) {
            return Err(PlanError::Message(format!("feature already exists: {to}")));
        }
        let feature = self
            .features
            .remove(from)
            .ok_or_else(|| PlanError::Message(format!("unknown feature: {from}")))?;
        self.features.insert(to.to_string(), feature);
        Ok(self)
    }

    /// Return a new FeatureSpace that contains names present in both spaces.
    pub fn intersect(&self, other: &FeatureSpace) -> Self {
        let mut out = FeatureSpace::new();
        for (name, feature) in &self.features {
            if other.features.contains_key(name) {
                out.features.insert(name.clone(), feature.clone());
            }
        }
        out
    }

    /// Map features by name, producing a new FeatureSpace.
    pub fn map_features(&self, f: impl Fn(&str, &Feature) -> Feature) -> Self {
        let mut out = FeatureSpace::new();
        for (name, feature) in &self.features {
            out.features.insert(name.clone(), f(name, feature));
        }
        out
    }

    pub fn series(&self) -> FeatureSeriesNameSpace {
        FeatureSeriesNameSpace::new(self.clone())
    }

    pub fn into_series(self) -> FeatureSeries {
        FeatureSeries::new(self)
    }
    pub fn names(&self) -> Vec<&str> {
        self.features.keys().map(|k| k.as_str()).collect()
    }

    pub fn get(&self, name: &str) -> Option<&Feature> {
        self.features.get(name)
    }

    pub fn require(&self, name: &str) -> Result<&Feature, PlanError> {
        self.get(name)
            .ok_or_else(|| PlanError::Message(format!("unknown feature: {name}")))
    }

    /// Create a first-class schema view reference.
    pub fn view_ref(&self, feature: &str, view: &str) -> FeatureView {
        FeatureView::new(feature, view)
    }

    /// Compose two named features (left then right) into a new Feature.
    pub fn compose(&self, left: &str, right: &str) -> Result<Feature, PlanError> {
        let a = self.require(left)?;
        let b = self.require(right)?;
        Ok(a.chained(b))
    }

    /// Build a feature pipeline by chaining features in order.
    pub fn pipeline<'a>(
        &self,
        names: impl IntoIterator<Item = &'a str>,
    ) -> Result<Feature, PlanError> {
        let mut iter = names.into_iter();
        let first = iter.next().ok_or_else(|| {
            PlanError::Message("pipeline requires at least one feature name".to_string())
        })?;
        let mut out = self.require(first)?.clone();
        for name in iter {
            let next = self.require(name)?;
            out = out.chain(next);
        }
        Ok(out)
    }

    /// Project a named feature into the canonical `item` artifact.
    pub fn project_item(&self, base: &str, item_expr: Expr) -> Result<Feature, PlanError> {
        let feature = self.require(base)?;
        Ok(feature.projected_item(item_expr))
    }

    /// Insert a projected feature (canonical `item`) under a new name.
    pub fn insert_projected_item(
        self,
        name: impl Into<String>,
        base: &str,
        item_expr: Expr,
    ) -> Result<Self, PlanError> {
        self.insert_derived(name, base, |f| f.project_item(item_expr))
    }

    pub fn attention_report(&self, env: Option<&PlanEnv>, mode: EvalMode) -> serde_json::Value {
        let mut out = serde_json::Map::new();
        for (name, feature) in &self.features {
            out.insert(
                name.clone(),
                serde_json::to_value(feature.attention_report(env, mode)).unwrap_or_else(
                    |e| serde_json::json!({"error": format!("failed to serialize report: {e}")}),
                ),
            );
        }
        serde_json::Value::Object(out)
    }

    pub fn to_print_envelope(
        &self,
        env: Option<&PlanEnv>,
        mode: EvalMode,
        run_id: Option<String>,
    ) -> PrintEnvelope {
        PrintEnvelope::new(
            PrintKind::Ml,
            PrintProvenance {
                source: "gds::collections::dataset::feature_space".to_string(),
                run_id,
                kernel_version: None,
            },
            serde_json::json!({
                "mode": format!("{mode:?}"),
                "features": self.names(),
                "reports": self.attention_report(env, mode),
            }),
        )
    }

    pub fn stream(
        &self,
        feature_name: &str,
        dataset: Dataset,
        batch_size: usize,
    ) -> Result<StreamingDataset, PlanError> {
        let feature = self.require(feature_name)?;
        Ok(feature.stream(dataset, batch_size))
    }

    /// Evaluate a named feature and then select a view using a selector.
    pub fn eval_select(
        &self,
        feature_name: &str,
        env: &PlanEnv,
        mode: EvalMode,
        selector: &Selector,
    ) -> Result<Dataset, PlanError> {
        let feature = self.require(feature_name)?;
        feature.eval_select(env, mode, selector)
    }

    /// Evaluate a named feature and then select a named schema view.
    pub fn eval_view(
        &self,
        feature_name: &str,
        schema: &FeatureSchema,
        view_name: &str,
        env: &PlanEnv,
        mode: EvalMode,
    ) -> Result<Dataset, PlanError> {
        let selector = schema.require(view_name)?;
        self.eval_select(feature_name, env, mode, selector)
    }
}

#[derive(Debug, Clone)]
pub struct FeatureSeries {
    space: FeatureSpace,
}

impl FeatureSeries {
    pub fn new(space: FeatureSpace) -> Self {
        Self { space }
    }

    pub fn space(&self) -> &FeatureSpace {
        &self.space
    }

    pub fn into_space(self) -> FeatureSpace {
        self.space
    }

    pub fn feature(&self) -> FeatureSeriesNameSpace {
        FeatureSeriesNameSpace::new(self.space.clone())
    }
}

#[derive(Debug, Clone)]
pub struct FeatureSeriesNameSpace {
    space: FeatureSpace,
}

impl FeatureSeriesNameSpace {
    pub fn new(space: FeatureSpace) -> Self {
        Self { space }
    }

    pub fn expr(&self) -> crate::collections::dataset::namespaces::feature::FeatureExprNameSpace {
        crate::collections::dataset::namespaces::feature::FeatureExprNameSpace::new()
    }

    pub fn union(&self, other: FeatureSpace) -> FeatureSeries {
        FeatureSeries::new(self.space.clone().union(other))
    }

    pub fn merge(&self, other: FeatureSpace) -> FeatureSeries {
        FeatureSeries::new(self.space.clone().merge(other))
    }

    pub fn keep<'a>(
        &self,
        names: impl IntoIterator<Item = &'a str>,
    ) -> Result<FeatureSeries, PlanError> {
        Ok(FeatureSeries::new(self.space.keep(names)?))
    }

    pub fn drop<'a>(&self, names: impl IntoIterator<Item = &'a str>) -> FeatureSeries {
        FeatureSeries::new(self.space.clone().drop(names))
    }

    pub fn rename(&self, from: &str, to: &str) -> Result<FeatureSeries, PlanError> {
        Ok(FeatureSeries::new(self.space.clone().rename(from, to)?))
    }

    pub fn intersect(&self, other: &FeatureSpace) -> FeatureSeries {
        FeatureSeries::new(self.space.intersect(other))
    }

    pub fn map_features(&self, f: impl Fn(&str, &Feature) -> Feature) -> FeatureSeries {
        FeatureSeries::new(self.space.map_features(f))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::dataset::plan::Source;

    fn feature_named(name: &str) -> Feature {
        Feature::new(Plan::new(Source::Var("ds".to_string()))).named(name)
    }

    #[test]
    fn feature_space_merge_overwrites() {
        let left = FeatureSpace::new()
            .insert("a", feature_named("left"))
            .insert("b", feature_named("left-b"));
        let right = FeatureSpace::new().insert("a", feature_named("right"));

        let merged = left.merge(right);
        let a = merged.get("a").unwrap();
        assert_eq!(a.name(), Some("right"));
        assert_eq!(merged.len(), 2);
    }

    #[test]
    fn feature_space_rename_changes_key() {
        let space = FeatureSpace::new().insert("a", feature_named("alpha"));
        let renamed = space.rename("a", "b").unwrap();

        assert!(renamed.get("a").is_none());
        assert!(renamed.get("b").is_some());
    }

    #[test]
    fn feature_space_keep_requires_names() {
        let space = FeatureSpace::new()
            .insert("a", feature_named("alpha"))
            .insert("b", feature_named("beta"));
        let kept = space.keep(["b"]).unwrap();

        assert_eq!(kept.len(), 1);
        assert!(kept.get("b").is_some());
    }
}
