//! DataFrame API registry helpers (py-polars inspired).

use std::collections::HashSet;
use std::sync::RwLock;

use once_cell::sync::Lazy;

/// Target type for namespace registration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NameSpaceTarget {
    Expr,
    DataFrame,
    LazyFrame,
    Series,
}

/// Errors raised during namespace registration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NameSpaceError {
    Reserved {
        name: String,
        target: NameSpaceTarget,
    },
}

#[derive(Default)]
struct NameSpaceRegistry {
    expr: HashSet<String>,
    dataframe: HashSet<String>,
    lazyframe: HashSet<String>,
    series: HashSet<String>,
}

impl NameSpaceRegistry {
    fn set_mut(&mut self, target: NameSpaceTarget) -> &mut HashSet<String> {
        match target {
            NameSpaceTarget::Expr => &mut self.expr,
            NameSpaceTarget::DataFrame => &mut self.dataframe,
            NameSpaceTarget::LazyFrame => &mut self.lazyframe,
            NameSpaceTarget::Series => &mut self.series,
        }
    }
}

static NAMESPACE_REGISTRY: Lazy<RwLock<NameSpaceRegistry>> =
    Lazy::new(|| RwLock::new(NameSpaceRegistry::default()));
const RESERVED_EXPR_NAMESPACES: &[&str] = &[
    "arr", "bin", "cat", "dt", "ext", "list", "meta", "name", "str", "struct", "record",
];
const RESERVED_SERIES_NAMESPACES: &[&str] = &[
    "arr", "bin", "cat", "dt", "ext", "list", "str", "struct", "record",
];

fn reserved_for(target: NameSpaceTarget) -> &'static [&'static str] {
    match target {
        NameSpaceTarget::Expr => RESERVED_EXPR_NAMESPACES,
        NameSpaceTarget::Series => RESERVED_SERIES_NAMESPACES,
        NameSpaceTarget::DataFrame | NameSpaceTarget::LazyFrame => &[],
    }
}

fn register_namespace(target: NameSpaceTarget, name: &str) -> Result<(), NameSpaceError> {
    if reserved_for(target)
        .iter()
        .any(|reserved| *reserved == name)
    {
        return Err(NameSpaceError::Reserved {
            name: name.to_string(),
            target,
        });
    }

    let mut registry = NAMESPACE_REGISTRY
        .write()
        .expect("namespace registry lock poisoned");
    let set = registry.set_mut(target);
    if set.contains(name) {
        eprintln!(
            "Overriding existing custom namespace {name:?} on {target:?}",
            name = name,
            target = target
        );
    } else {
        set.insert(name.to_string());
    }

    Ok(())
}

/// Register a custom namespace for Expr pipelines.
pub fn register_expr_namespace(name: &str) -> Result<(), NameSpaceError> {
    register_namespace(NameSpaceTarget::Expr, name)
}

/// Register a custom namespace for DataFrames.
pub fn register_dataframe_namespace(name: &str) -> Result<(), NameSpaceError> {
    register_namespace(NameSpaceTarget::DataFrame, name)
}

/// Register a custom namespace for LazyFrames.
pub fn register_lazyframe_namespace(name: &str) -> Result<(), NameSpaceError> {
    register_namespace(NameSpaceTarget::LazyFrame, name)
}

/// Register a custom namespace for Series.
pub fn register_series_namespace(name: &str) -> Result<(), NameSpaceError> {
    register_namespace(NameSpaceTarget::Series, name)
}
