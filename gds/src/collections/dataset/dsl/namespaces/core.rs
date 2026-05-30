//! Core Dataset DSL namespace registry.
//!
//! This layer owns the registry and reserved-name rules. The Component-facing
//! namespace surfaces live in `mode`.

use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::sync::RwLock;

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
