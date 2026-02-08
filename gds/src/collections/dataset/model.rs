//! Models: intentionally minimal scaffolding for pairing models with features.
//!
//! We keep this light on purpose:
//! - A `Feature` is our canonical feature pipeline (often backed by a `Plan`).
//! - A `Model` can be many things (NNs, linear models, symbolic rules, etc).
//! - The creative composition point is to *pair* a model with a feature mapping.

use crate::collections::dataset::feature::Feature;

#[derive(Debug, Clone)]
pub struct ModelWithFeatures<M> {
    pub features: Feature,
    pub model: M,
}

impl<M> ModelWithFeatures<M> {
    pub fn new(features: Feature, model: M) -> Self {
        Self { features, model }
    }

    pub fn map_model<N>(self, f: impl FnOnce(M) -> N) -> ModelWithFeatures<N> {
        ModelWithFeatures {
            features: self.features,
            model: f(self.model),
        }
    }

    pub fn map_features(self, f: impl FnOnce(Feature) -> Feature) -> Self {
        Self {
            features: f(self.features),
            model: self.model,
        }
    }
}
