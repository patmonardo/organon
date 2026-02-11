//! Dataset feature schema helpers.
//!
//! This module is intentionally small: it gives the dataset platform an evolving
//! "schema" vocabulary that is selector-driven.
//!
//! The schema surface is also the seed for a dataset symbol table that will
//! later anchor UDT hookups into the DataFrame layer.
//!
//! A selector is a stable *description* of a view over columns/features
//! (by name, dtype, regex, etc.). This is useful even when the underlying
//! feature pipelines are unstructured or streaming-first.

use std::collections::BTreeMap;

use crate::collections::dataframe::Selector;
use crate::collections::dataset::plan::PlanError;
use crate::collections::dataset::Dataset;
use serde_json::Value as JsonValue;

#[derive(Debug, Clone, Default)]
pub struct SymbolTable {
    symbols: BTreeMap<String, SymbolDef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolDef {
    pub kind: String,
    pub detail: Option<JsonValue>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(mut self, name: impl Into<String>, kind: impl Into<String>) -> Self {
        self.symbols.insert(
            name.into(),
            SymbolDef {
                kind: kind.into(),
                detail: None,
            },
        );
        self
    }

    pub fn insert_with(
        mut self,
        name: impl Into<String>,
        kind: impl Into<String>,
        detail: JsonValue,
    ) -> Self {
        self.symbols.insert(
            name.into(),
            SymbolDef {
                kind: kind.into(),
                detail: Some(detail),
            },
        );
        self
    }

    pub fn names(&self) -> Vec<&str> {
        self.symbols.keys().map(|k| k.as_str()).collect()
    }

    pub fn get(&self, name: &str) -> Option<&SymbolDef> {
        self.symbols.get(name)
    }

    pub fn require(&self, name: &str) -> Result<&SymbolDef, PlanError> {
        self.get(name)
            .ok_or_else(|| PlanError::Message(format!("unknown symbol: {name}")))
    }
}

#[derive(Debug, Clone, Default)]
pub struct FeatureSchema {
    views: BTreeMap<String, Selector>,
}

impl FeatureSchema {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a named selector view.
    pub fn view(mut self, name: impl Into<String>, selector: Selector) -> Self {
        self.views.insert(name.into(), selector);
        self
    }

    /// Derive a named view using a closure that can reference existing views.
    ///
    /// This is the lowest-ceremony way to build exploratory schema views.
    pub fn derive_with(
        self,
        name: impl Into<String>,
        f: impl FnOnce(&FeatureSchema) -> Result<Selector, PlanError>,
    ) -> Result<Self, PlanError> {
        let selector = f(&self)?;
        Ok(self.view(name, selector))
    }

    pub fn names(&self) -> Vec<&str> {
        self.views.keys().map(|k| k.as_str()).collect()
    }

    pub fn get(&self, name: &str) -> Option<&Selector> {
        self.views.get(name)
    }

    pub fn require(&self, name: &str) -> Result<&Selector, PlanError> {
        self.get(name)
            .ok_or_else(|| PlanError::Message(format!("unknown schema view: {name}")))
    }

    /// Clone a selector by view name (useful when building custom compositions).
    pub fn selector(&self, name: &str) -> Result<Selector, PlanError> {
        Ok(self.require(name)?.clone())
    }

    /// Derive a new view as the OR (union) of two existing views.
    pub fn derive_or(
        self,
        name: impl Into<String>,
        left: &str,
        right: &str,
    ) -> Result<Self, PlanError> {
        let sel = self.selector(left)?.or(self.selector(right)?);
        Ok(self.view(name, sel))
    }

    /// Derive a new view as the OR (union) of many existing views.
    pub fn derive_or_many<'a>(
        self,
        name: impl Into<String>,
        views: impl IntoIterator<Item = &'a str>,
    ) -> Result<Self, PlanError> {
        let mut iter = views.into_iter();
        let first = iter.next().ok_or_else(|| {
            PlanError::Message("derive_or_many requires at least one view".to_string())
        })?;
        let mut sel = self.selector(first)?;
        for v in iter {
            sel = sel.or(self.selector(v)?);
        }
        Ok(self.view(name, sel))
    }

    /// Derive a new view as the AND (intersection) of two existing views.
    pub fn derive_and(
        self,
        name: impl Into<String>,
        left: &str,
        right: &str,
    ) -> Result<Self, PlanError> {
        let sel = self.selector(left)?.and(self.selector(right)?);
        Ok(self.view(name, sel))
    }

    /// Derive a new view as the AND (intersection) of many existing views.
    pub fn derive_and_many<'a>(
        self,
        name: impl Into<String>,
        views: impl IntoIterator<Item = &'a str>,
    ) -> Result<Self, PlanError> {
        let mut iter = views.into_iter();
        let first = iter.next().ok_or_else(|| {
            PlanError::Message("derive_and_many requires at least one view".to_string())
        })?;
        let mut sel = self.selector(first)?;
        for v in iter {
            sel = sel.and(self.selector(v)?);
        }
        Ok(self.view(name, sel))
    }

    /// Derive a new view as NOT(base).
    pub fn derive_not(self, name: impl Into<String>, base: &str) -> Result<Self, PlanError> {
        let sel = self.selector(base)?.not();
        Ok(self.view(name, sel))
    }

    /// Derive a new view as base EXCLUDE remove.
    pub fn derive_exclude(
        self,
        name: impl Into<String>,
        base: &str,
        remove: &str,
    ) -> Result<Self, PlanError> {
        let sel = self.selector(base)?.exclude(self.selector(remove)?);
        Ok(self.view(name, sel))
    }

    /// Derive a new view as base EXCLUDE (remove_1 OR remove_2 OR ...).
    pub fn derive_exclude_many<'a>(
        self,
        name: impl Into<String>,
        base: &str,
        removes: impl IntoIterator<Item = &'a str>,
    ) -> Result<Self, PlanError> {
        let mut iter = removes.into_iter();
        let first = iter.next().ok_or_else(|| {
            PlanError::Message("derive_exclude_many requires at least one remove view".to_string())
        })?;
        let mut remove_sel = self.selector(first)?;
        for v in iter {
            remove_sel = remove_sel.or(self.selector(v)?);
        }
        let sel = self.selector(base)?.exclude(remove_sel);
        Ok(self.view(name, sel))
    }

    /// Apply a named schema view to a concrete Dataset.
    pub fn select(&self, ds: &Dataset, view: &str) -> Result<Dataset, PlanError> {
        let selector = self.require(view)?;
        Ok(ds.select_selector(selector)?)
    }
}

#[derive(Debug, Clone, Default)]
pub struct ModelSchema {
    inputs: BTreeMap<String, Selector>,
    outputs: BTreeMap<String, Selector>,
}

impl ModelSchema {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn input_view(mut self, name: impl Into<String>, selector: Selector) -> Self {
        self.inputs.insert(name.into(), selector);
        self
    }

    pub fn output_view(mut self, name: impl Into<String>, selector: Selector) -> Self {
        self.outputs.insert(name.into(), selector);
        self
    }

    pub fn input_names(&self) -> Vec<&str> {
        self.inputs.keys().map(|k| k.as_str()).collect()
    }

    pub fn output_names(&self) -> Vec<&str> {
        self.outputs.keys().map(|k| k.as_str()).collect()
    }

    pub fn input(&self, name: &str) -> Option<&Selector> {
        self.inputs.get(name)
    }

    pub fn output(&self, name: &str) -> Option<&Selector> {
        self.outputs.get(name)
    }

    pub fn require_input(&self, name: &str) -> Result<&Selector, PlanError> {
        self.input(name)
            .ok_or_else(|| PlanError::Message(format!("unknown model input view: {name}")))
    }

    pub fn require_output(&self, name: &str) -> Result<&Selector, PlanError> {
        self.output(name)
            .ok_or_else(|| PlanError::Message(format!("unknown model output view: {name}")))
    }
}
