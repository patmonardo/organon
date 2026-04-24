//! Dataset DSL namespace builders and registry.
//!
//! This module hosts the concrete dataset namespace builders surfaced from the
//! top-level Dataset DSL shell ([`crate::collections::dataset::namespace`]).
//! It also owns the dataset-side namespace registry, which is intentionally
//! distinct from the DataFrame namespace registry in
//! [`crate::collections::dataframe::namespace`].
//!
//! Layering of the namespace builders (do not duplicate across layers):
//!
//! 1. **Orchestration** — [`dataset::DatasetNs`]: registry, IO, metadata,
//!    projection, reporting. Dataset-level "what/where/how described".
//! 2. **Data-op authoring** — [`dataop::DataOpNs`]: input / encode /
//!    transform / decode / output stages, plus their text-domain variants and
//!    DataFrame lowering helpers. The canonical place to build a
//!    `DatasetDataOpExpr`.
//! 3. **Text-domain alias** — [`text::TextNs`]: a thin alias over the
//!    `text_*` family in `DataOpNs`, kept so toolchain code can spell
//!    text-only stages without prefixing every call.
//! 4. **Feature algebra** — [`feature::FeatureNs`] and
//!    [`feature::FeatureExprNameSpace`]: positions, paths, specs, conditions,
//!    rules, templates, and the symbolic `FeatureExpr` algebra.
//! 5. **Tree algebra** — [`tree::TreeNs`]: nodes, leaves, positions, spans,
//!    and tree transforms used by parse/AST flows.
//!
//! Reserved namespace names recognised by the registry are listed in
//! [`RESERVED_DATASET_NAMESPACES`].

use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::sync::RwLock;

pub mod dataop;
pub mod dataset;
pub mod feature;
pub mod text;
pub mod tree;

/// Errors raised during namespace registration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NameSpaceError {
    Reserved { name: String },
}

#[derive(Default)]
struct NameSpaceRegistry {
    dataset: HashSet<String>,
}

impl NameSpaceRegistry {
    fn dataset_mut(&mut self) -> &mut HashSet<String> {
        &mut self.dataset
    }
}

static DATASET_NAMESPACE_REGISTRY: Lazy<RwLock<NameSpaceRegistry>> =
    Lazy::new(|| RwLock::new(NameSpaceRegistry::default()));

const RESERVED_DATASET_NAMESPACES: &[&str] =
    &["corpus", "text", "image", "audio", "tabular", "tree"];

/// Helper to register the canonical `corpus` namespace.
pub fn register_corpus_namespace() -> Result<(), NameSpaceError> {
    register_dataset_namespace("corpus")
}

/// Register a custom namespace for datasets.
pub fn register_dataset_namespace(name: &str) -> Result<(), NameSpaceError> {
    if RESERVED_DATASET_NAMESPACES
        .iter()
        .any(|reserved| *reserved == name)
    {
        return Err(NameSpaceError::Reserved {
            name: name.to_string(),
        });
    }
    let mut registry = DATASET_NAMESPACE_REGISTRY
        .write()
        .expect("dataset namespace registry poisoned");
    let set = registry.dataset_mut();
    if set.contains(name) {
        // already registered: quietly succeed
        Ok(())
    } else {
        set.insert(name.to_string());
        Ok(())
    }
}

/// Check whether a namespace is registered.
pub fn is_dataset_namespace_registered(name: &str) -> bool {
    let registry = DATASET_NAMESPACE_REGISTRY
        .read()
        .expect("dataset namespace registry poisoned");
    registry.dataset.contains(name)
}
