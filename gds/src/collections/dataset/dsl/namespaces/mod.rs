//! Dataset DSL namespace builders and registry.
//!
//! `core` carries the registry and reserved-name rules; `mode` carries the
//! Component-facing namespace surfaces.

pub mod core;
pub mod dataop;
pub mod dataset;
pub mod expr;
pub mod feature;
pub mod mode;
pub mod model;
pub mod plan;
pub mod text;
pub mod treens;

pub use core::{
    is_dataset_namespace_registered, register_corpus_namespace, register_dataset_namespace,
    NameSpaceError,
};
pub use mode::{DatasetNs, ExprNs, FeatureExprNs, FeatureNs, ModelNs, PlanNs, TextNs, TreeNs};
